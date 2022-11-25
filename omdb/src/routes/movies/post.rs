use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{errors::{AppError, AppErrorType}, utils::JsonResponse};

#[derive(serde::Deserialize, Debug)]
pub struct MovieDetailBody {
    imdb_id: String,
    user_id: i32,
    movie_name: String,
    plot: String,
    poster: String
}

pub async fn save_movie(
    body: web::Json::<MovieDetailBody>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    let body = body.0;
    let movie_name = &body.movie_name.to_uppercase();
    let is_movie_exist = is_movie_available(&pool, &body.user_id, &movie_name);

    match is_movie_exist.await {
        Ok(id) => {
            let response = JsonResponse {
                status_code: 409,
                message: format!("Movie with title {movie_name} already exist"),
                body: ""
            };
            Ok(HttpResponse::Conflict().json(response))
        },
        Err(sqlx::Error::RowNotFound) => {
            save_movie_query(&body, &pool).await.map_err(|e| {
                AppError {
                    cause: None,
                    error_type: AppErrorType::InternalError,
                    message: None
                }
            })?;
        
            let response = JsonResponse {
                status_code: 200,
                message: "Save movie succesfull".to_string(),
                body: serde_json::json!({
                    "movie_name": &body.movie_name,
                    "plot": &body.plot,
                    "poster": &body.poster
                })
            };
        
            response.response_message().map_err(|e| {
                AppError {
                    cause: None,
                    error_type: AppErrorType::InternalError,
                    message: None,
                }
            })
        },
        Err(_) => {
            Err(
                AppError {
                    cause: None,
                    error_type: AppErrorType::InternalError,
                    message: None
                }
            )
        }
    }   
}

async fn save_movie_query(
    body: &MovieDetailBody,
    pool: &PgPool
) -> Result<(), sqlx::Error> {  
    sqlx::query!(
        r#"
            INSERT INTO favorite_movies(user_id, movie_name, imdb_id, plot, poster)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        body.user_id,
        body.movie_name.to_uppercase(),
        body.imdb_id,
        body.plot,
        body.poster
    )
    .execute(pool)
    .await?;

    Ok(())

}

async fn is_movie_available(
    pool: &PgPool,
    user_id: &i32,
    movie_name: &String
) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!(
        r#"
            SELECT id FROM favorite_movies
            WHERE user_id = $1 
            AND movie_name = $2
        "#,
        user_id,
        movie_name
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}