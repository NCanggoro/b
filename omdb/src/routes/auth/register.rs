use actix_web::{HttpResponse, web, guard::AnyGuard};
use anyhow::Context;
use argon2::{password_hash::SaltString, Algorithm, Version, Argon2, Params, PasswordHasher};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;

use crate::{error::error_500, utils::JsonResponse};

#[derive(serde::Deserialize)]
pub struct RegisterBodyRequest {
    email: String,
    username: String,
    password: Secret<String>
}

pub async fn register_user(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterBodyRequest>
) -> Result<HttpResponse, actix_web::Error> {
    let RegisterBodyRequest {
        email,
        username,
        password
    } = body.0;
    let password_hash = compute_password_hash(password)
        .map_err(error_500)?;

    store_user(&pool, RegisterBodyRequest { email, username, password: password_hash})
        .await
        .map_err(error_500)?;

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
) -> Result<(), StoreUserError> {
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
        StoreUserError(e)
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

    Ok((Secret::new(password_hash)))
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



pub struct StoreUserError(sqlx::Error);

impl std::fmt::Display for StoreUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database error was encountered when trying to store user"
        )
    }
}

impl std::error::Error for StoreUserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0    )
    }
}

impl std::fmt::Debug for StoreUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }   
}
