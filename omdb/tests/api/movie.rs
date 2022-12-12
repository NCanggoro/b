use crate::helper::spawn_app;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Response {
    status_code: u32,
    body: ResponseBody
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ResponseBody {
    #[serde(rename = "imdbID")]
    imdb_id: String,
    #[serde(rename = "Title")]
    title: String
}

#[tokio::test]
async fn get_movie_by_title() {
    let app = spawn_app().await;
    let token = app.test_user.get_token(&app).await;
    assert!(!&token.is_none());
    let query = [("title", "cars")];
    let res = app.get_movie_by_title(&query, token.unwrap()).await;

    assert!(&res.status().is_success());

    let body: Response = res
        .json()
        .await
        .expect("Failed to parse json respond");

    
    assert_eq!(&body.body.title, "Cars");
    assert_eq!(body.status_code, 200);
}

#[tokio::test]
async fn get_movie_by_id() {
    let app = spawn_app().await;
    let token = app.test_user.get_token(&app).await;
    assert!(!&token.is_none());
    let query = [("id", "tt0120338")];
    let res = app.get_movie_by_id(&query, token.unwrap()).await;

    assert!(&res.status().is_success());

    let body: Response = res
        .json()
        .await
        .expect("Failed to parse json respond");

    
    assert_eq!(&body.body.title, "Titanic");
    assert_eq!(body.status_code, 200);
}

#[tokio::test]
async fn get_movie_by_search() {
    #[derive(serde::Deserialize)]
    struct Response {
        status_code: u32,
        body: ResponseBody
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct ResponseBody {
        response: String,
        search: Vec<std::collections::HashMap<String, String>>
    }

    let app = spawn_app().await;
    let token = app.test_user.get_token(&app).await;
    assert!(!&token.is_none());
    let query = [("title", "titanic")];
    let res = app.get_movie_by_search(&query, token.unwrap()).await;

    assert!(&res.status().is_success());

    let body: Response = res
        .json()
        .await
        .expect("Failed to parse json respond");

    
    assert_eq!(&body.body.response, "True");
    assert!(&body.body.search.len().gt(&0));
    assert_eq!(body.status_code, 200);
}

