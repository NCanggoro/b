use actix_web::{HttpResponse, web};
use rand::Rng;
use rand::distributions::Alphanumeric;
use secrecy::Secret;
use sqlx::PgPool;
use sha3::{Digest, Keccak256};
use redis::Commands;

use crate::authentication::{Credentials, AuthError};

use crate::authentication::validate_credentials;
use crate::errors::{AppError, AppErrorType, error_500};
use crate::utils::JsonResponse;


#[derive(serde::Deserialize)]
pub struct LoginBodyRequest {
    pub email: String,
    pub password: Secret<String>
}

pub async fn login(
    body: web::Json::<LoginBodyRequest>,
    pool: web::Data::<PgPool>,
    redis: web::Data<redis::Client>
) -> Result<HttpResponse, AppError> {
    let LoginBodyRequest {
        email,
        password
    } = body.0;

    let credentials = Credentials {
        email,
        password
    };
    
    match validate_credentials(credentials, &pool).await {
        Ok(user_info) => {
            let (user_id, user_email) = user_info;
            let mut redis_con = redis.get_connection().map_err(|e| {
                AppError {
                    cause: None,
                    error_type: AppErrorType::InternalError,
                    message: Some("Failed to connect to redis client".to_string())
                }
            })?;
            let mut hasher = Keccak256::new();

            let rand_s: String = rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
                
            hasher.update(format!("{}{}", &user_id, rand_s).as_bytes());
            
            let token = format!("{:x}", hasher.finalize());

            let _: () = redis_con.set(&token, user_id.to_string()).map_err(|e| {
                AppError {
                    cause: None,
                    error_type: AppErrorType::InternalError,
                    message: Some("Failed to save key value to redis".to_string())
                }
            })?;

            insert_token(&pool, &token, &user_id).await?;

            let response = JsonResponse {
                status_code: 200,
                message: "Login successful".to_string(),
                body: serde_json::json!({
                    "id":  user_id,
                    "email": user_email,
                    "token": token
                })
            };

            response.response_message().map_err(|_| {
                AppError {
                    error_type: AppErrorType::UnauthorizedErorr,
                    cause: None,
                    message: Some("Failed to Unauthorized".to_string())
                }
            })
            }
        Err(e) => {
             let e = match e  {
                AuthError::InvalideCredential(_) => AppErrorType::UnauthorizedErorr,
                AuthError::UnexpectedError(_) => AppErrorType::InternalError
             };
             let mut message: Option<String> = None;
             if let e = AppErrorType::UnauthorizedErorr {
                message = Some("User not found".to_string());
             }

             Err(AppError {
                cause: None,
                error_type: e,
                message
             })
        }

    }
}


pub async fn insert_token(
    pool: &PgPool,
    token: &String,
    user_id: &i32
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            UPDATE users
            SET auth_token = $1
            WHERE user_id = $2
        "#,
        token,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| {
        AppError {
            cause: None,
            error_type: AppErrorType::InternalError,
            message: Some("Failed to set auth_token".to_string())
        }
    })?;

    Ok(())
}
