use std::sync::Arc;

use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::Database;

use crate::{
    errors::app_error::ApiError, models::user_model::UserLoginPayload, services::user_service,
    utils::api_response::ApiResponse,
};

pub async fn login_user(
    Extension(db_instance): Extension<Arc<Database>>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    #[allow(clippy::let_unit_value)]
    // this is temporary solution for avoid clippy lint warning. In future we will remove this line for clean code
    let result = user_service::login_service(&db_instance, payload).await?;
    let response: ApiResponse<()> = ApiResponse::new(200, result, "Login successful".to_string());
    Ok((StatusCode::OK, Json(response)))
}
