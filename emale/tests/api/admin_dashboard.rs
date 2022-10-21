use crate::helpers::spawn_app;

#[tokio::test]
async fn must_logged_in_to_access_admin_dashboard() {
	let app = spawn_app().await;

	let response = app.get_admin_dashboard().await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("Location").unwrap(), "/login");

}

#[tokio::test]
async fn logout() {
	let app = spawn_app().await;

	let login_body = serde_json::json!({
		"username": &app.test_user.username,
		"password": &app.test_user.password
	});

	let response = app.post_login(&login_body).await;

	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/admin/dashboard");
	
	let html = app.get_admin_dashboard_html().await;
	assert!(html.contains(&format!("Welcome {}", &app.test_user.username)));

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");

	let html = app.get_login_html().await;
	assert!(html.contains("Logged out successfully"));

	let response = app.get_admin_dashboard().await;
	assert_eq!(response.headers().get("LOCATION").unwrap(), "/login");

}