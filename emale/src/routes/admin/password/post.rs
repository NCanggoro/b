use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::authentication::{validate_credentials, AuthError, Credentials, UserId};
use crate::authentication;
use crate::routes::get_username;
use crate::utils::{error_500, see_other};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    if form.new_password.expose_secret().len() < 12 || form.new_password.expose_secret().len() > 128
    {
        FlashMessage::error(
            "Password must be longer than 12 characters and less than 128 characters",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the fieid values must match",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    let username = get_username(*user_id, &pool).await.map_err(error_500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredential(_) => {
                FlashMessage::error("The current password is incorrect").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(error_500(e).into()),
        };
    }
    authentication::change_password(*user_id, form.0.new_password, &pool)
		.await
		.map_err(error_500)?;
	FlashMessage::error("Password successfully updated").send();
	Ok(see_other("/admin/password"))
}
