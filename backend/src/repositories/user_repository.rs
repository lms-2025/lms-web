use mongodb::{
    bson::{doc, Document},
    Database,
};

use crate::errors::app_error::ApiError;

pub async fn get_user(db_instance: &Database, email: String) -> Result<Document, ApiError> {
    let user = db_instance
        .collection::<Document>("user")
        .find_one(doc! {"email": &email})
        .await
        .map_err(ApiError::from_mongodb_error)?;

    match user {
        Some(document) => Ok(document),
        None => Err(ApiError::NotFound(format!(
            "User with email {} not found",
            email
        ))),
    }
}

pub async fn create_user(_db_instance: &Database) -> Result<(), ApiError> {
    // call to service
    // db_instance.collection("users").find_one().await.unwrap();
    Ok(())
}
