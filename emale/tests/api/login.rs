use crate::helpers::spawn_app;

#[tokio::test]
async fn set_erorr_flash_message_when_something_wrong() {
	let app = spawn_app().await;

	let login_body = serde_json::json!({
		"username": "username",
		"password": "password"
	});
	let response = app.post_login(&login_body).await;
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("Location").unwrap(), "/login");
	
	let html_page = app.get_login_html().await;
	assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

	let html_page = app.get_login_html().await;
	assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#))
}