use crate::helper::spawn_app;

#[tokio::test]
async fn pow_works() {
    #[derive(serde::Deserialize)]
    struct ResponseVal {
        num: i32,
    }

    let app = spawn_app().await;
    let client = reqwest::Client::new();
    
    let res = client
    .get(&format!("{}/pow2/2", &app.address))
    .send()
    .await
    .expect("Request Failed");
    
    assert!(res.status().is_success());
    assert_eq!(4, res.json::<ResponseVal>().await.expect("Failed").num);
}