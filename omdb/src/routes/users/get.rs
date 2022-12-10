use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::{AppError, AppErrorType};
use crate::utils::JsonResponse;

#[derive(serde::Deserialize, Debug)]
pub struct GetFavoriteMovieBody {
    user_id: i32
}

#[derive(serde::Serialize, Debug)]
pub struct GetFavoriteMoviesRecord {
    id: i32,
    movie_name: String,
    imdb_id: Option<String>,
    plot: Option<String>,
    poster: Option<String>
}

pub async fn get_favorite_movies(
    body: web::Json<GetFavoriteMovieBody>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    let user_id = body.0.user_id;
    let pool: &PgPool = &pool;
    let row: Vec<GetFavoriteMoviesRecord> = sqlx::query_as!(
        GetFavoriteMoviesRecord,
        r#"
            SELECT id, movie_name, imdb_id, plot, poster from favorite_movies
            where user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|_| {
        AppError {
            cause: None,
            error_type: AppErrorType::InternalError,
            message: None
        }
    })?;

    let body = serde_json::json!(row);
    
    let response = JsonResponse {
        status_code: 200,
        message: "Get movies by Id successfull".to_string(),
        body
    };

    response.response_message().map_err(|_| {
        AppError {
            cause: None,
            error_type: AppErrorType::InternalError,
            message: None
        }
    })
}