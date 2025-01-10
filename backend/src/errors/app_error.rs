use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_more::Display;
use mongodb::error::Error;
use serde::Serialize;

use crate::utils::api_response::ApiResponse;

#[derive(Debug, Display, Serialize)]
pub enum ApiError {
    NotFound(String),
    InternalServerError,
    BadRequest(String),
    Unauthorized,
    Forbidden,
    Conflict(String),
    UnprocessableEntity(String),
    NotImplemented(String),
    ServiceUnavailable(String),
    TooManyRequests,
    DatabaseError(String),
    Custom(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound(message) => {
                create_error_response(StatusCode::NOT_FOUND, 404, message)
            }
            ApiError::InternalServerError => {
                log::error!("Internal Server Error: {:?}", self);
                create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    500,
                    "Internal Server Error".to_string(),
                )
            }
            ApiError::DatabaseError(message) => {
                log::error!("Database Error: {:?}", message);
                create_error_response(StatusCode::INTERNAL_SERVER_ERROR, 500, message)
            }
            ApiError::BadRequest(message) => {
                create_error_response(StatusCode::BAD_REQUEST, 400, message)
            }
            ApiError::Unauthorized => {
                create_error_response(StatusCode::UNAUTHORIZED, 401, "Unauthorized".to_string())
            }
            ApiError::Forbidden => {
                create_error_response(StatusCode::FORBIDDEN, 403, "Forbidden".to_string())
            }
            ApiError::Conflict(message) => {
                create_error_response(StatusCode::CONFLICT, 409, message)
            }
            ApiError::UnprocessableEntity(message) => {
                create_error_response(StatusCode::UNPROCESSABLE_ENTITY, 422, message)
            }
            ApiError::NotImplemented(message) => {
                log::error!("Not Implemented: {:?}", message);
                create_error_response(StatusCode::NOT_IMPLEMENTED, 501, message)
            }
            ApiError::ServiceUnavailable(message) => {
                log::error!("Service Unavailable: {:?}", message);
                create_error_response(StatusCode::SERVICE_UNAVAILABLE, 503, message)
            }
            ApiError::TooManyRequests => create_error_response(
                StatusCode::TOO_MANY_REQUESTS,
                429,
                "Too Many Requests".to_string(),
            ),
            ApiError::Custom(message) => {
                create_error_response(StatusCode::BAD_REQUEST, 400, message)
            }
        }
    }
}

impl ApiError {
    pub fn from_mongodb_error(e: Error) -> Self {
        ApiError::DatabaseError(e.to_string())
    }
}

fn create_error_response(status: StatusCode, code: u16, message: String) -> Response {
    let response = ApiResponse::<()>::error(code, message);
    (status, Json(response)).into_response()
}
