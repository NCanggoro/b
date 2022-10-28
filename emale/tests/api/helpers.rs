use argon2::{password_hash::SaltString, Argon2, PasswordHasher, Algorithm, Version, Params};
use emale::{
    configuration::{get_config, DatabaseSettings},
    startup::Application,
    startup::get_connection_pool,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber}, email_client::EmailClient, issue_delivery_worker::{ExecutionOutcome, try_execute_task},
};
use linkify::{LinkFinder, LinkKind};
use once_cell::sync::Lazy;
use serde_json::{Value, from_slice};
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_tracing_subscriber(subscriber_name, default_filter, std::io::stdout);
        init_tracing_subscriber(subscriber)
    } else {
        let subscriber = get_tracing_subscriber(subscriber_name, default_filter, std::io::sink);
        init_tracing_subscriber(subscriber)
    }
});

pub struct ConfirmationLink {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string()
        }
    }


    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),

        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
        sqlx::query!(
            "
            INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)
            ",
            self.user_id,
            self.username,
            password_hash
        )
        .execute(pool)
        .await
        .expect("Failed to store user for test");
    }

	pub async fn login(&self, app: &TestApp) {
		app.post_login(&serde_json::json!({
			"username": &app.test_user.username,
			"password": &app.test_user.password
		}))
		.await;
	}
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub email_client: EmailClient,
    pub api_client: reqwest::Client,
    pub port: u16,
    pub test_user: TestUser
}

impl TestApp {

	pub async fn dispatch_all_pending_emails(&self) {
		loop {
			if let ExecutionOutcome::EmptyQueue =
				try_execute_task(&self.db_pool, &self.email_client)
					.await
					.unwrap()
				{
					break;
				}
		}
	}

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_login_html(&self) -> String {
        self.api_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
            .text()
            .await
            .unwrap()
    }

    pub fn get_confirmation_link(
        &self,
        email_request: &wiremock::Request
    ) -> ConfirmationLink {
        let body: Value =  from_slice(
            &email_request.body
        ).unwrap();

        let get_link = |s: &str| { 
            let links: Vec<_> = LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(&body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(&body["TextBody"].as_str().unwrap());
        ConfirmationLink { 
            html, 
            plain_text
        }
    }

    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/subscribe", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }


    pub async fn get_publish_newsletter(&self) -> reqwest::Response {
		self.api_client
			.get(&format!("{}/admin/newsletters", &self.address))
			.send()
			.await
			.expect("Failed to execute request")
    }

	pub async fn post_publish_newsletter<Body>(&self, body: &Body) -> reqwest::Response
		where
			Body: serde::Serialize
		{
			self.api_client
				.post(&format!("{}/admin/newsletters", &self.address))
				.form(body)
				.send()
				.await
				.expect("Failed to execute request")
		}

    pub async fn get_admin_dashboard(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/dashboard", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_change_password_form(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/password", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_change_password<Body>(&self, body: &Body) -> reqwest::Response
        where 
            Body: serde::Serialize
        {
        self.api_client
            .post(&format!("{}/admin/password", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_change_password_html_response(&self) -> String {
		self.get_change_password_form().await.text().await.unwrap()
    }

    pub async fn get_admin_dashboard_html(&self) -> String {
        self.get_admin_dashboard()
            .await
            .text()
            .await
            .unwrap()
    }

	pub async fn post_logout(&self) -> reqwest::Response {
		self.api_client
			.post(&format!("{}/admin/logout", &self.address))
			.send()
			.await
			.expect("Failed to execute request")
	}

	

}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;
    // Randomise configuration to ensure
    // test isolation
    let config = {
        let mut c = get_config().expect("Failed to get configuration");
        c.database.db_name = Uuid::new_v4().to_string();
        // Use random OS port
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };

    configure_database(&config.database).await;

    let app = Application::build(config.clone())
        .await
        .expect("Failed to build app");

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    
    let app_port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());
    let test_app = TestApp {
        address: format!("http://localhost:{}", app_port),
        db_pool: get_connection_pool(&config.database),
        email_server,
        email_client: config.email_client.client(),
        api_client,
        port: app_port,
        test_user: TestUser::generate()
    };
    test_app.test_user.store(&test_app.db_pool).await;
    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect");

    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}