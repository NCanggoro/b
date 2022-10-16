use actix_web::{HttpResponse, web, http::header::ContentType};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::session_state::TypedSession;
use crate::utils::{error_500, see_other};

#[tracing::instrument(
	name="Get username",
	skip(pool)
)]
async fn get_username(
	user_id: Uuid,
	pool: &PgPool
) -> Result<String, anyhow::Error> {
	let row = sqlx::query!(
		r#"
		SELECT username
		FROM users
		WHERE user_id = $1
		"#,
		user_id
	)
	.fetch_one(pool)
	.await
	.context("Failed to perform a query to retrive a username")?;
	Ok(row.username)
}

pub async fn admin_dashboard(
	session: TypedSession,
	pool: web::Data<PgPool>
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session
		.get_user_id()
		.map_err(error_500)?
	{
		get_username(user_id, &pool).await.map_err(error_500)?
	} else {
		return Ok(see_other("/login"))
		
	};
	Ok(HttpResponse::Ok()
		.content_type(ContentType::html())
		.body(format!(
			r#"<!DOCTYPE html>
					<html lang="en">
					<head>
					<meta http-equiv="content-type" content="text/html; charset=utf-8">
					<title>Admin dashboard</title>
					</head>
					<body>
						<p>Welcome {username}!</p>
						<ol>
							<li><a href="/admin/password/">Change Password</a></li>
						</ol>
					</body>
				</html>"#
		))
	)
}

// preserving error 500 root cause for logging
