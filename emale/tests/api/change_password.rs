use rand::{Rng, distributions::Alphanumeric};
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn must_logged_in_to_get_change_password_form() {
	let app = spawn_app().await;
	let response = app.get_change_password_form().await;
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

#[tokio::test]
async fn must_logged_in_to_post_change_password() {
	let app = spawn_app().await;
	let new_password = Uuid::new_v4().to_string();
	let body = serde_json::json!({
		"current_password": Uuid::new_v4().to_string(),
		"new_password": &new_password,
		"new_password_check": &new_password
	});
	let response = app.post_change_password(&body).await;
    
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let new_password_check = Uuid::new_v4().to_string();
    
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    })).await;

    let body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password,
        "new_password_check": &new_password_check
    });

    let response = app
        .post_change_password(&body)
        .await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/password");

    let html = app.get_change_password_html_response().await;
    assert!(html.contains(
        "<p><i>You entered two different new passwords - the fieid values must match</i></p>"
    ))
}

#[tokio::test]
async fn current_password_valid() {
	let app = spawn_app().await;
	let new_password = Uuid::new_v4().to_string();
	let wrong_password = Uuid::new_v4().to_string();

	app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    })).await;

    let body = serde_json::json!({
        "current_password": &wrong_password,
        "new_password": &new_password,
        "new_password_check": &new_password
    });

	let response = app
		.post_change_password(&body)
		.await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/password");

	let html_page = app.get_admin_dashboard_html().await;
	dbg!(html_page);
	
}

#[tokio::test]
async fn password_min_12_char_and_less_128_char() {
	let app = spawn_app().await;
	let new_password: String = rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(rand::thread_rng().gen_range(0..12))
		.map(char::from)
		.collect();
	
	app.post_login(&serde_json::json!({
		"username": &app.test_user.username,
		"password": &app.test_user.password
	})).await;

	let body = serde_json::json!({
		"current_password": &app.test_user.password,
		"new_password": &new_password,
		"new_password_check": &new_password
	});

	let response = app.post_change_password(&body).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/password");

	let html = app.get_change_password_html_response().await;
	assert!(html.contains(
		"<p><i>Password must be longer than 12 characters and \
		less than 128 characters</i></p>"
	));

	let new_password: String = rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(rand::thread_rng().gen_range(128..150))
		.map(char::from)
		.collect();
	
	let body = serde_json::json!({
		"current_password": &app.test_user.password,
		"new_password": &new_password,
		"new_password_check": &new_password
	});

	let response = app.post_change_password(&body).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/password");

	let html = app.get_change_password_html_response().await;
	assert!(html.contains(
		"<p><i>Password must be longer than 12 characters and \
		less than 128 characters</i></p>"
	));

}

#[tokio::test]
async fn change_password_success() {
	let app = spawn_app().await;
	let new_password: String = rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(rand::thread_rng().gen_range(13..127))
		.map(char::from)
		.collect();

	// login
	app.post_login(&serde_json::json!({
		"username": &app.test_user.username,
		"password": &app.test_user.password
	})).await;

	
	// change password
	let body = &serde_json::json!({
		"current_password": &app.test_user.password,
		"new_password": &new_password,
		"new_password_check": &new_password
	});

	let response = app.post_change_password(&body).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/password");

	let html = app.get_change_password_html_response().await;
	assert!(html.contains("<p><i>Password successfully updated</i></p>"));

	// logout 
	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");

	let html = app.get_login_html().await;
	assert!(html.contains("<p><i>Logged out successfully</i></p>"));

	// login with new password
	let response = app.post_login(&serde_json::json!({
		"username": &app.test_user.username,
		"password": &new_password
	})).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/dashboard");

}