use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
	subscription_token: String
}

#[tracing::instrument(
	skip(_param)
	name = "Confirm a pending subscriber"
)]
pub async fn confirm(_param: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}