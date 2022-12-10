use argon2::{Argon2, Algorithm, Params, PasswordHasher};
use argon2::password_hash::SaltString;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use omdb::{telemetry::{get_tracing_subscriber, init_tracing_subscriber}, configuration::{get_config, DatabaseSettings}, startup::{Application, get_connection_pool}};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{PgPool, PgConnection, Connection, Executor, migrate};

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

pub struct TestUser {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String
}

impl TestUser {
    pub fn generate() -> Self {
        let email = SafeEmail().fake();
        let password: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
        let username = "username1".to_string();

        Self {
            user_id: 2,
            username,
            email,
            password
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password = Argon2::new(
            Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),

        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
        sqlx::query!(
            "
            INSERT INTO users (user_id, username, email,  password)
                VALUES ($1, $2, $3, $4)
            ",
            self.user_id,
            self.username,
            self.email,
            password
        )
        .execute(pool)
        .await
        .expect("Failed to store user for test");
    }

	pub async fn login(&self, app: &TestApp) -> serde_json::Value {
		app.post_login(&serde_json::json!({
			"email": &app.test_user.email,
			"password": &app.test_user.password
		}))
		.await
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse response")
	}


    pub async fn get_token(&self, app: &TestApp) -> Option<String> {
        let res = self.login(app).await;
        match res.get("body") {
            Some(body) => Some(body.get("token").unwrap().as_str().unwrap().to_string()),
            _ => None
        }
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
    pub port: u16,
    pub test_user: TestUser,
}

impl TestApp {
    pub async fn post_register<Body>(
        &self, 
        body: &Body
    ) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/register", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Request Failed")
    }

    pub async fn post_login<Body>(
        &self, 
        body: &Body
    ) -> reqwest::Response
        where 
            Body: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/login",&self.address))
            .json(&body)
            .send()
            .await
            .expect("Request Failed")
    }
    pub async fn get_movie_by_title<Query>(
        &self, 
        query: &Query, 
        token: String
    ) -> reqwest::Response 
        where 
            Query: serde::Serialize
    {

        println!("{:?}", &token);
        self.api_client
            .get(&format!("{}/movies/title", &self.address))
            .header("auth-token", token.to_string())
            .query(query)
            .send()
            .await
            .expect("Failed to execute request")
    }

}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let db_name: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    // Randomise configuration to ensure
    // test isolation
    let config = {
        let mut c = get_config().expect("Failed to get configuration");
        c.database.db_name = db_name;
        // Use random OS port
        c.application.port = 0;
        c
    };

    configure_database(&config.database).await;

    let app = Application::build(config.clone())
        .await
        .expect("Failed to build app");

    let api_client = reqwest::Client::builder()
        .build()
        .unwrap();

    
    let app_port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());
    let test_app = TestApp {
        address: format!("http://localhost:{}", app_port),
        db_pool: get_connection_pool(&config.database),
        api_client,
        port: app_port,
        test_user: TestUser::generate(),
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