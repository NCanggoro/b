use actix_web::{HttpRequest, HttpResponse, web};
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

pub async fn get_favorite_movies(

) -> Result<(), AppError> {
    todo!()
}