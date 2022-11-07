use actix_web::{HttpResponse, web};
use secrecy::Secret;
use sqlx::PgPool;

use crate::authentication::{Credentials, AuthError};

use crate::authentication::validate_credentials;
use crate::utils::JsonResponse;


#[derive(serde::Deserialize)]
pub struct LoginBodyRequest {
    pub email: String,
    pub password: Secret<String>
}

pub async fn login(
    body: web::Json::<LoginBodyRequest>,
    pool: web::Data::<PgPool>
) -> Result<HttpResponse, LoginError> {
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
            let response = JsonResponse {
                status_code: 200,
                message: "Login successful".to_string(),
                body: serde_json::json!({
                    "id":  user_id,
                    "email": user_email
                })
            };

            response.response_message().map_err(|e| {
                LoginError {
                    error_type: LoginErrorType::UnauthorizedEror,
                    cause: None,
                    message: Some("Failed to Unauthorized".to_string())
                }
            })
            }
        Err(e) => {
             let e = match e  {
                AuthError::InvalideCredential(_) => LoginErrorType::UnauthorizedEror,
                AuthError::UnexpectedError(_) => LoginErrorType::InternalError
             };
             let mut message: Option<String> = None;
             if let e = LoginErrorType::UnauthorizedEror {
                message = Some("User not found".to_string());
             }

             Err(LoginError {
                cause: None,
                error_type: e,
                message
             })
        }

    }
}

#[derive(Debug)]
pub enum LoginErrorType {
    InternalError,
    BadRequestError,
    UnauthorizedEror
}

#[derive(Debug)]
pub struct LoginError {
    message: Option<String>,
    cause: Option<String>,
    error_type: LoginErrorType
}
impl LoginError {
    pub fn message(&self) -> String {
        match &*self {
            Self {
                message: Some(message),
                ..
            } => message.clone(),
            _ => "An unexpected error has occured".to_string()
        }
    }
}

#[derive(serde::Serialize)]
struct LoginErrorResponse {
    status_code: u16,
    message: String
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl actix_web::error::ResponseError for LoginError {
    // Error base
    // Turn this error to Error struct
    // like this https://github.com/nemesiscodex/actix-todo/blob/master/src/errors.rs
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(LoginErrorResponse{
            status_code:  self.status_code().as_u16(),
            message: self.message()
        })
    }

    fn status_code(&self) -> actix_http::StatusCode {
        match self.error_type {
            LoginErrorType::InternalError => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
            LoginErrorType::BadRequestError => actix_http::StatusCode::BAD_REQUEST,
            LoginErrorType::UnauthorizedEror => actix_http::StatusCode::UNAUTHORIZED
        }
    }
}