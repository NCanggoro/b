use anyhow::Context;
use argon2::{PasswordHash, Argon2, PasswordVerifier};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;

use crate::telemetry::spawn_blocking_with_tracing;

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>
}

pub async fn validate_credentials(
    credential: Credentials,
    pool: &PgPool
) -> Result<(i32, String), AuthError> {
    let mut user_info:Option<(i32, String)> = None;
    let mut expected_password = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
		gZiV/M1gPc22ElAH/Jh1Hw$\
		CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
			.to_string()
    );
    if let Some((stored_user_id, email, stored_password)) = get_stored_password_hash(
        &credential.email,
        &pool
    )
    .await
    .map_err(AuthError::UnexpectedError)?
    {
        user_info = Some((stored_user_id, email));
        expected_password = stored_password;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(
            expected_password,
            credential.password
        )
    })
    .await
    .context("Failed to spawn blocking task")
    .map_err(AuthError::UnexpectedError)??;

    user_info.ok_or_else(||
        AuthError::InvalideCredential(anyhow::anyhow!("Email / password is invalid"))
    )
}

pub fn verify_password_hash(
    expected_password: Secret<String>,
    password: Secret<String>
) -> Result<(), AuthError> {
    let expected_password = PasswordHash::new(
        &expected_password.expose_secret()
    )
    .context("Failed to parse to PHC")
    .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password.expose_secret().as_bytes(), 
            &expected_password
        )
        .context("Invaild password")
        .map_err(AuthError::InvalideCredential)
}

pub async fn get_stored_password_hash(
    email: &str,
    pool: &PgPool

) -> Result<Option<(i32, String, Secret<String>)>, anyhow::Error> {
    let row: Option<_> = sqlx::query!(
        r#"
            SELECT user_id, email, password
            FROM users
            WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .context("User not found")?
    .map(|row| (row.user_id, row.email, Secret::new(row.password)));

    Ok(row)
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("InvalidCredentials")]
    InvalideCredential(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}