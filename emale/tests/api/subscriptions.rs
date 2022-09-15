use crate::helpers::spawn_app;
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{path, method};


#[tokio::test]
async fn subscribe_return_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=nc%20nocap&email=nc_nocap%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let res = app.post_subscriptions(body.into()).await;

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("Select email,name from subscriber")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get subscriber");

    assert_eq!(saved.email, "nc_nocap@gmail.com");
    assert_eq!(saved.name, "nc nocap");
}

#[tokio::test]
async fn subscribe_return_400_for_invalid_from_data() {
    let app = spawn_app().await;

    let test_case = vec![
        ("name=nc%20nocap", "missing email field"),
        ("email=nc_nocap%40gmail.com", "missing name field"),
        ("", "missing both fields"),
    ];

    for (invalid_body, err_message) in test_case {
        let res = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(400, res.status().as_u16(), "{}", err_message);
    }
}

#[tokio::test]
async fn subscribe_return_400_when_fields_are_present_but_empty() {
    let app = spawn_app().await;
    let test_case = vec![
        ("name=nc%20nocap", "empty name"),
        ("name=nc_nocap&email=", "empty email"),
        ("name=nocap&email=wrong-email-format", "invalid email"),
    ];

    for (body,description) in test_case {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't return 400 OK when the payload {}",
            description
        )
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=nc%20nocap&email=ncnocap%40gmail.com";

    Mock::given(path("email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    
    app.post_subscriptions(body.into()).await;
}