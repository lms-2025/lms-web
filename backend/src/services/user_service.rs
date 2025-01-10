use mongodb::{bson::Document, Database};

use crate::{
    errors::app_error::ApiError, models::user_model::UserLoginPayload,
    repositories::user_repository,
};

pub async fn login_service(
    db_instance: &Database,
    user_payload: UserLoginPayload,
) -> Result<(), ApiError> {
    if user_payload.email.is_empty() || user_payload.password.is_empty() {
        return Err(ApiError::BadRequest(
            "Email and password are required".to_string(),
        ));
    }
    let user: Document = user_repository::get_user(db_instance, user_payload.email).await?;
    if user.is_empty() {
        return Err(ApiError::NotFound("User not found".to_string()));
    }

    Ok(())
}
