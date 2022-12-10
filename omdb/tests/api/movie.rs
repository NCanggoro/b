use crate::helper::spawn_app;

#[tokio::test]
async fn get_movie_by_title() {
    let app = spawn_app().await;
    let token = app.test_user.get_token(&app).await;
    assert!(!&token.is_none());
    let query = [("title", "cars")];
    let res = app.get_movie_by_title(&query, token.unwrap()).await;

    assert!(res.status().is_success());

}

