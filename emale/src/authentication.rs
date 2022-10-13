use anyhow::Context;
use argon2::{PasswordHash, Argon2, PasswordVerifier};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
	#[error("Invalid Credentials")]
	InvalidCredential(#[source] anyhow::Error),
	#[error(transparent)]
	UnexpectedError(#[from] anyhow::Error)
}

pub struct Credentials {
	pub username: String,
	pub password: Secret<String>
}

#[tracing::instrument(
	name = "Get stored credentials",
	skip(username, pool)
)]
pub async fn get_stored_password_hash(
	username: &str,
	pool: &PgPool
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
	let row: Option<_> = sqlx::query!(
		r#"
		select user_id, password_hash
		FROM users
		where username = $1
		"#,
		username,
	)
	.fetch_optional(pool)
	.await
	.context("Failed to perform query to retrive stored password_hash")?
	.map(|row| (row.user_id, Secret::new(row.password_hash)));
	Ok(row)
}

#[tracing::instrument(
	name = "Verify password hash",
	skip(expected_password_hash, password)
)]
pub fn verify_pasword_hash(
	expected_password_hash: Secret<String>,
	password: Secret<String>
) -> Result<(), AuthError> {
	let expected_password_hash = PasswordHash::new(
		&expected_password_hash.expose_secret()
	)
	.context("Failed to parse hash in PHC")
	.map_err(AuthError::UnexpectedError)?;

	Argon2::default()
		.verify_password(
			password.expose_secret().as_bytes(),
			&expected_password_hash
		)
		.context("Invalid password")
		.map_err(AuthError::InvalidCredential)
}

#[tracing::instrument(
	name = "Validate credentials",
	skip(credentials, pool)
)]
pub async fn validate_credentials(
	credentials: Credentials,
	pool:&PgPool
) -> Result<uuid::Uuid, AuthError> {
	let mut user_id = None;
	let mut expected_password_hash = Secret::new(
		"$argon2id$v=19$m=15000,t=2,p=1$\
		gZiV/M1gPc22ElAH/Jh1Hw$\
		CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
			.to_string()
	);
	if let Some((stored_user_id, stored_password_hash)) = get_stored_password_hash(
		&credentials.username,
		&pool
	)
	.await
	.map_err(AuthError::UnexpectedError)?

	{
		user_id = Some(stored_user_id);
		expected_password_hash = stored_password_hash;
	}

	spawn_blocking_with_tracing(move || {
		verify_pasword_hash(
			expected_password_hash,
			credentials.password
		)
	})
	.await
	.context("Failed to spawn blocking task")
	.map_err(AuthError::UnexpectedError)??;
	
	user_id.ok_or_else(||
		AuthError::InvalidCredential(anyhow::anyhow!("Unknown username."))
	)

}