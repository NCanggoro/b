use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path, any};

use crate::helpers::{spawn_app, TestApp, ConfirmationLink};

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLink {
	let body = "name=nc_nocap&email=nc_nocap%40gmail.com";

	let _mock_guard = Mock::given(path("/email"))
		.and(method("POST"))
		.respond_with(ResponseTemplate::new(200))
		.expect(1)
		.mount_as_scoped(&app.email_server)
		.await;

	app.post_subscriptions(body.into())
		.await
		.error_for_status()
		.unwrap();
	
	let email_request = &app
		.email_server
		.received_requests()
		.await
		.unwrap()
		.pop()
		.unwrap();
		
	app.get_confirmation_link(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
	let confirmation_link = create_unconfirmed_subscriber(app).await.html;
	reqwest::get(confirmation_link)
		.await
		.unwrap()
		.error_for_status()
		.unwrap();
}


#[tokio::test]
async fn newsletters_not_delivered_to_uncofirmed_subs() {
	let app = spawn_app().await;

	create_unconfirmed_subscriber(&app).await;
	app.test_user.login(&app).await;

	Mock::given(any())
		.respond_with(ResponseTemplate::new(200))
		.expect(0)
		.mount(&app.email_server)
		.await;
	
	
	let newsletter_request_body = serde_json::json!({
		"title": "title",
		"text_content": "text content",
		"html_content": "<p>HTML CONTENT</p>"
	});

	let response = app.post_publish_newsletter(&newsletter_request_body).await;

	assert_eq!(response.status().as_u16(), 303);
	// assert_eq!(response.headers().)
}
#[tokio::test]
async fn newsletters_delivered_to_cofirmed_subs() {
	let app = spawn_app().await;

	create_confirmed_subscriber(&app).await;
	app.test_user.login(&app).await;

	Mock::given(path("/email"))
		.respond_with(ResponseTemplate::new(200))
		.expect(1)
		.mount(&app.email_server)
		.await;
	
	let newsletter_request_body = serde_json::json!({
		"title": "title",
		"text_content": "text content",
		"html_content": "<p>HTML CONTENT</p>"
	});

	let response = app.post_publish_newsletter(&newsletter_request_body).await;

	assert_eq!(response.status().as_u16(), 303);
	// assert_eq!(response.headers().)
}

#[tokio::test]
async fn must_logged_in_to_request_newsletter_form() {
	let app = spawn_app().await;

	let response = app.get_publish_newsletter().await;
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");
}


#[tokio::test]
async fn must_logged_in_to_publish_newsletter() {
	let app = spawn_app().await;

	let body = serde_json::json!({
		"title": "Title",
		"text_content": "content",
		"html_content": "<p>content</p>"
	});

	let response = app.post_publish_newsletter(&body).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");
}