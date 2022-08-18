use emale::configuration::{get_config, DatabaseSettings};
use emale::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

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

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Request Failed");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn pow_works() {
    #[derive(Deserialize)]
    struct ResponseVal {
        num: i32,
    }

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/pow2/2", &app.address))
        .send()
        .await
        .expect("Request Failed");

    assert!(res.status().is_success());
    assert_eq!(4, res.json::<ResponseVal>().await.expect("Failed").num);
}

#[tokio::test]
async fn subscribe_return_200_for_valid_form_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=nc%20nocap&email=nc_nocap%40gmail.com";

    let res = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscribe request");

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("Select email,name from subscriber")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get subscriber");

    assert_eq!(saved.email, "nc_nocap@gmail.com");
    assert_eq!(saved.name, "nc nocap");
}

#[tokio::test]
async fn subscribe_return_400_for_invalid_from_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_case = vec![
        ("name=nc%20nocap", "missing email field"),
        ("email=nc_nocap%40gmail.com", "missing name field"),
        ("", "missing both fields"),
    ];

    for (invalid_body, err_message) in test_case {
        let res = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute invalid subscribe request");

        assert_eq!(400, res.status().as_u16(), "{}", err_message);
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_strings_without_db())
        .await
        .expect("Failed to connect to database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_strings())
        .await
        .expect("Failed to connect");

    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

async fn spawn_app() -> TestApp {
    let mut config = get_config().expect("Failed to get configuration");
    config.database.db_name = Uuid::new_v4().to_string();

    Lazy::force(&TRACING);

    let db_pool = configure_database(&config.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);

    let server = emale::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: addr,
        db_pool,
    }
}
