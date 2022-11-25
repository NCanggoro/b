use actix_web::{HttpResponse};

#[derive(serde::Serialize)]
pub struct JsonResponse<T> {
    pub status_code: u16,
    pub message: String,
    pub body: T
}

impl<T: serde::Serialize> JsonResponse<T> {
    pub fn response_message(&self) -> Result<HttpResponse, actix_web::Error> {
        Ok(HttpResponse::Ok().json(&self))
    }
}