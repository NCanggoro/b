use actix_http::body::MessageBody;
use actix_web::{dev::{ServiceRequest, ServiceResponse}, web, error::{InternalError, ErrorBadRequest}, HttpResponse};
use actix_web_lab::middleware::Next;
use sqlx::PgPool;

use crate::errors::{AppError, AppErrorType};

// #[derive(Clone, Debug)]
// pub struct Token(i32);

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

pub async fn check_token(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let pool = req.app_data::<web::Data<PgPool>>().unwrap();
    let auth_token = req.headers().get("auth-token");
    if auth_token.is_none() {
        let response = HttpResponse::BadRequest().finish();
        let e = anyhow::anyhow!("auth-token header not found");
        return Err(InternalError::from_response(e, response).into())
    }
    match is_token_correct(&pool, req.headers().get("auth-token").unwrap().to_str().unwrap().to_string()).await? {
        Some(_) => {
            next.call(req).await
        },
        None => {
            let response = HttpResponse::InternalServerError().finish();
            let e = anyhow::anyhow!("Authentification not found");
            Err(InternalError::from_response(e, response).into())
        }
    }

}

async fn is_token_correct(
    pool: &PgPool,
    token: String
)-> Result<Option<i32>, AppError> {
    let row = sqlx::query!(
        r#"
            SELECT user_id from USERS
            WHERE auth_token = $1
        "#,
        token
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        AppError {
            cause: None,
            error_type: AppErrorType::UnauthorizedErorr,
            message: Some("Authentification not found, Please try log in again".to_string())
        }
    })?;

    Ok(Some(row.user_id))
}