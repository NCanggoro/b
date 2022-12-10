use crate::helper::spawn_app;

#[tokio::test]
async fn register_valid_return_200() {
    let app = spawn_app().await;

    let user = serde_json::json!({
        "email": "email@gmail.com",
        "username": "brudanem",
        "password": "bruh123"
    });

    let res = app.post_register(&user).await;

    assert!(&res.status().is_success());
    
    let body: serde_json::Value  = res
        .json()
        .await
        .expect("Parse json failed");

    assert_eq!(body["message"], "Register Success");
}

#[tokio::test]
async fn register_missing_field_return_400() {
    let app = spawn_app().await;

    let user = serde_json::json!({
        "email": "email@gmail.com",
        "password": "bruh123"
    });
    
    let res = app.post_register(&user).await;

    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn login_valid_return_200() {
    let app = spawn_app().await;
    let email = &app.test_user.email;
    let password = &app.test_user.password;

    let body = serde_json::json!({
        "email": email,
        "password": password
    });

    let res = app.post_login(&body).await;
    println!("{:?}", res.text().await);

    // assert!(res.status().is_success());
}
#[tokio::test]
async fn login_wrong_password_return_401() {
    let app = spawn_app().await;
    let email = &app.test_user.email;

    let body = serde_json::json!({
        "email": email,
        "password": "MANANOE123"
    });

    let res = app.post_login(&body).await;

    assert_eq!(res.status().as_u16(), 401);
}
#[tokio::test]
async fn login_user_not_found_return_401() {
    let app = spawn_app().await;
    let password = &app.test_user.password;

    let body = serde_json::json!({
        "email": "HALLOO@Gmail.com",
        "password": password
    });

    let res = app.post_login(&body).await;

    assert_eq!(res.status().as_u16(), 401);

}