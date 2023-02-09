use actix_web::HttpResponse;

pub fn error_500<T>(e: T) -> actix_web::Error 
where 
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn apperror_500(message: Option<String>) -> AppError {
    AppError { 
        message, 
        cause: None, 
        error_type: AppErrorType::InternalError 
    }
}

pub fn apperror_400(
    message: Option<String>
) -> AppError {
    AppError { 
        message, 
        cause: None, 
        error_type: AppErrorType::BadRequestError 
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    InternalError,
    BadRequestError,
    UnauthorizedErorr,
    NotFoundError
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType
}
impl AppError {
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
struct AppErrorResponse {
    status_code: u16,
    message: String
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse{
            status_code:  self.status_code().as_u16(),
            message: self.message()
        })
    }

    fn status_code(&self) -> actix_http::StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => actix_http::StatusCode::NOT_FOUND,
            AppErrorType::InternalError => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::BadRequestError => actix_http::StatusCode::BAD_REQUEST,
            AppErrorType::UnauthorizedErorr => actix_http::StatusCode::UNAUTHORIZED
        }
    }
}