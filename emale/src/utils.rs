use actix_http::header::LOCATION;
use actix_web::HttpResponse;

pub fn error_500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
	actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn error_400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
  T: std::fmt::Debug + std::fmt::Display + 'static,
{
	actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
  HttpResponse::SeeOther()
			.insert_header((LOCATION, location))
			.finish()
}
