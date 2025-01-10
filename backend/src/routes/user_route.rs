use axum::routing::{post, Router};

use crate::handlers::user_handler;

pub fn user_routes() -> Router {
    Router::new().nest(
        "/api/v1/user",
        Router::new()
            .route("/login", post(user_handler::login_user))
            .route("/register", post(user_handler::login_user))
            .route("/verify-email", post(user_handler::login_user))
            .route("/forgot-password", post(user_handler::login_user))
            .route("/reset-password", post(user_handler::login_user)),
    )
}
