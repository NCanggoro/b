use actix_web::{HttpResponse, web};
use argon2::{password_hash::SaltString, Algorithm, Version, Argon2, Params, PasswordHasher};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;

use crate::{errors::{error_500, AppError, apperror_500}, utils::JsonResponse};

#[derive(serde::Deserialize)]
pub struct RegisterBodyRequest {
    email: String,
    username: String,
    password: Secret<String>
}

#[tracing::instrument(
    skip(pool, body),
    fields(email)
)]

pub async fn register_user(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterBodyRequest>
) -> Result<HttpResponse, actix_web::Error> {
    let RegisterBodyRequest {
        email,
        username,
        password
    } = body.0;

    tracing::Span::current()
        .record("email", &tracing::field::display(&email));

    let password_hash = compute_password_hash(password)
        .map_err(error_500)?;

    store_user(&pool, RegisterBodyRequest { email, username, password: password_hash})
        .await?;

    let response = JsonResponse {
        status_code: 200,
        message: "Register Success".to_string(),
        body: ""
    };

    response.response_message()
}


async fn store_user(
    pool: &PgPool,
    body: RegisterBodyRequest    
) -> Result<(), AppError> {
    sqlx::query!(
        "
            INSERT INTO users(username, email, password)
            VALUES ($1, $2 ,$3)
        ",
        body.username,
        body.email,
        body.password.expose_secret()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        match e  {
            sqlx::Error::Database(_) => {
                AppError {
                    cause: None,
                    message: Some("email is already registered".to_string()),
                    error_type: crate::errors::AppErrorType::InternalError
                }
            },
            _ => apperror_500(Some(e.to_string()))
        }
    })?;

    Ok(())

}

fn compute_password_hash(
    password: Secret<String>
) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(1500, 2, 1 , None).unwrap()
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }  
    Ok(())
}