use crate::helpers::{spawn_app, TestApp, ConfirmationLink};
use wiremock::{ResponseTemplate, Mock};
use wiremock::matchers::{path, method, any};

#[tokio::test]
async fn newsletters_returns_400_invalid_data() {
	let app = spawn_app().await;
	let test_cases = vec![
		(
			serde_json::json!({
				"content": {
						"text": "Text",
						"html": "<p>Hello</p>"
				}
			}),
			"missing title",
		),
		(
			serde_json::json!({ "title": "Title" }),
			"missing content"
		)
	];

	for (invalid_body, error_message) in test_cases {
		let response = app.post_newletters(invalid_body).await;

		assert_eq!(
			400,
			response.status().as_u16(),
			"{}",
			error_message
		)
	}
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subs() {
	let app = spawn_app().await;
	create_unconfirmed_subs(&app).await;

	Mock::given(any())
		.respond_with(ResponseTemplate::new(200))
		.expect(0)
		.mount(&app.email_server)
		.await;
	
	let request_body = serde_json::json!({
		"title": "Title",
		"content": {
			"text": "Text",
			"html": "<p>Hello</p>"
		}
	});

	let response = app.post_newletters(request_body).await;
	
	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_delivered_to_confirmed_subs() {
	let app = spawn_app().await;
	create_confirmed_subs(&app).await;

	Mock::given(path("/email"))
		.and(method("POST"))
		.respond_with(ResponseTemplate::new(200))
		.expect(1)
		.mount(&app.email_server)
		.await;

	let request_body = serde_json::json!({
		"title": "Title",
		"content": {
			"text": "Text",
			"html": "<p>Hello</p>"
		}
	});

	let response = app.post_newletters(request_body).await;

	assert_eq!(response.status().as_u16(), 200);
}

async fn create_unconfirmed_subs(app: &TestApp) -> ConfirmationLink {
	let body = "name=nc%20nocap&email=ncnocap%40gmail.com";

	let _mock_guard = Mock::given(path("email"))
		.and(method("POST"))
		.respond_with(ResponseTemplate::new(200))
		.named("Create unconfirm subs")
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
	
	app.get_confirmation_link(email_request)
}

async fn create_confirmed_subs(app: &TestApp) {
	let confirmation_link = create_unconfirmed_subs(app).await;
	reqwest::get(confirmation_link.html)
		.await
		.unwrap()
		.error_for_status()
		.unwrap();

}

#[tokio::test]
async fn request_missing_authorization_rejected() {
	let app = spawn_app().await;

	let response = reqwest::Client::new()
		.post(&format!("{}/newsletters", &app.address))
		.json(&serde_json::json!({
			"title": "Title",
			"content": {
				"text": "plain text",
				"html": "<p>Body</p>"
			}
		}))
		.send()
		.await
		.expect("Failed to execute post request");

	assert_eq!(401, response.status().as_u16());
	assert_eq!(r#"Basic realm="publish""#, response.headers()["WWW-Authenticate"]);
}

#[tokio::test]
async fn non_user_is_rejected() {
	let app = spawn_app().await;
	let username = uuid::Uuid::new_v4().to_string();
	let password = uuid::Uuid::new_v4().to_string();
	
	let response = reqwest::Client::new()
		.post(&format!("{}/newsletters", &app.address))
		.basic_auth(username, Some(password))
			.json(&serde_json::json!({
				"title": "Title",
				"content": {
					"text": "plain text",
					"html": "<p>Body</p>"
				}
			}))
			.send()
			.await
			.expect("Failed to execute request");
			
	assert_eq!(401, response.status().as_u16());
	assert_eq!(r#"Basic realm="publish""#, response.headers()["WWW-Authenticate"]);
}

#[tokio::test]
async fn invalid_password_is_rejected() {
	let app = spawn_app().await;
	let username = &app.test_user.username;
	let password = uuid::Uuid::new_v4().to_string();
	assert_ne!(app.test_user.password, password);

	let response = reqwest::Client::new()
		.post(&format!("{}/newsletters", &app.address))
		.basic_auth(username, Some(password))
			.json(&serde_json::json!({
				"title": "Title",
				"content": {
					"text": "plain text",
					"html": "<p>Body</p>"
				}
			}))
			.send()
			.await
			.expect("Failed to execute request");
			
	assert_eq!(401, response.status().as_u16());
	assert_eq!(r#"Basic realm="publish""#, response.headers()["WWW-Authenticate"]);


}