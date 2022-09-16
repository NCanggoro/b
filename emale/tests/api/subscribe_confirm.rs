use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_confirmation_without_token_return_400() {
    let app = spawn_app().await;

    let res = reqwest::get(&format!("{}/subscribe/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn link_returned_by_subscribe_return_200() {
    let app = spawn_app().await;
    let body = "name=nc%20nocap&email=nc_nocap%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_link = app.get_confirmation_link(email_request);

    let response = reqwest::get(confirmation_link.html)
		.await
		.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn click_confirmation_link_confirm_verification() {
    let app = spawn_app().await;
    let body = "name=nc%20nocap&email=nc_nocap%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_link = app.get_confirmation_link(email_request);

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, name, status FROM subscriber")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get saved subscription");

    assert_eq!(saved.email, "nc_nocap@gmail.com");
    assert_eq!(saved.name, "nc nocap");
    assert_eq!(saved.status, "confirmed");
}
