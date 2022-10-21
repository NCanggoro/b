use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

use crate::session_state::TypedSession;
use crate::utils::{see_other, error_500};

pub async fn logout(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
	if session.get_user_id().map_err(error_500)?.is_none() {
		Ok(see_other("/login"))
	} else {
		session.log_out();
		FlashMessage::info("Logged out successfully").send();
		Ok(see_other("/login"))
	}
}