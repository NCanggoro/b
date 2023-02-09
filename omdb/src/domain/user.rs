use actix_web::FromRequest;
use secrecy::Secret;

#[derive(Debug, serde::Deserialize)]
pub struct User {
	email: String,
	password: String
}

impl std::fmt::Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}