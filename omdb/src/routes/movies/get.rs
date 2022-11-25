use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use crate::errors::{AppError, AppErrorType};

use crate::utils::JsonResponse;

#[derive(serde::Deserialize, Debug)]
pub struct SearchQueryParams {
    pub title: String,
    pub movie_type: Option<String>,
    pub year: Option<u32>,
    pub page: Option<u32>,
}

pub async fn get_movies_search(
    query_string: web::Query<SearchQueryParams>
) -> Result<HttpResponse, AppError> {
    let year: u32 = if query_string.year.is_none() { 0 } else { query_string.year.unwrap() };
    let page: u32 = if query_string.page.is_none() { 1 } else { query_string.page.unwrap() };
    let url = format!("http://www.omdbapi.com/?apikey=b07b3f42&s={}&year={}&page={}", query_string.title, year, page);
    let response: serde_json::Value = reqwest::get(url)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?
        .json()
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?;
    
    let response = JsonResponse {
        status_code: 200,
        message: "Get movies successfull".to_string(),
        body: response
    };

    response.response_message().map_err(|e| {
        AppError {
            error_type: AppErrorType::UnauthorizedErorr,
            cause: None,
            message: None
        }
    })
}

#[derive(serde::Deserialize, Debug)]
pub struct TitleQueryParams {
    pub title: String,
}

pub async fn get_movie_by_name(
    query_string: web::Query<SearchQueryParams>
) -> Result<HttpResponse, AppError> {
    let url = format!("http://www.omdbapi.com/?apikey=b07b3f42&t={}", query_string.title);

    let response: serde_json::Value = reqwest::get(url)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?
        .json()
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?;
    
    let response = JsonResponse {
        status_code: 200,
        message: "Get movies by Title successfull".to_string(),
        body: response
    };

    response.response_message().map_err(|e| {
        AppError {
            error_type: AppErrorType::UnauthorizedErorr,
            cause: None,
            message: None
        }
    })

}   

#[derive(serde::Deserialize, Debug)]
pub struct IdQueryParams {
    pub id: String,
}

pub async fn get_movie_by_id(
    query_string: web::Query<IdQueryParams>
) -> Result<HttpResponse, AppError> {
    let url = format!("http://www.omdbapi.com/?apikey=b07b3f42&i={}", query_string.id);

    let response: serde_json::Value = reqwest::get(url)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?
        .json()
        .await
        .map_err(|e| {
            println!("{:?}", e);
            AppError {
                cause: None,
                error_type: AppErrorType::InternalError,
                message: None
            }
        })?;
    
    let response = JsonResponse {
        status_code: 200,
        message: "Get movies by Id successfull".to_string(),
        body: response
    };

    response.response_message().map_err(|e| {
        AppError {
            error_type: AppErrorType::UnauthorizedErorr,
            cause: None,
            message: None
        }
    })
}

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