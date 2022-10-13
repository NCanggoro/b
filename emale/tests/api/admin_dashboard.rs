use crate::helpers::spawn_app;

#[tokio::test]
async fn must_logged_in_to_access_admin_dashboard() {
  let app = spawn_app().await;

  let response = app.get_admin_dashboard().await;
  
  assert_eq!(response.status().as_u16(), 303);
	assert_eq!(response.headers().get("Location").unwrap(), "/login");

}