use axum::{routing::Router, Extension};
use mongodb::Database;
use redis::Client;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use lms_server::{
    configs::global_config::CONFIG,
    database::connection::establish_connection,
    jobs::main_job::initialize_job,
    middlewares::{auth_middleware::AuthMiddleware, rate_limit_middleware::RateLimitMiddleware},
    routes::user_route::user_routes,
    utils::logger::init_logger,
};

#[tokio::main]
async fn main() {
    // Initialize logger
    init_logger();

    // Initialize background jobs
    initialize_job();

    // Establish database connection
    let db_instance: Arc<Database> = Arc::new(establish_connection().await);

    // Initialize Redis client
    let redis_client: Client =
        Client::open(&(*CONFIG.redis_url)).expect("Failed to create redis client");

    // Configure Axum router
    let app = Router::new()
        .merge(user_routes())
        .layer(Extension(db_instance.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(RateLimitMiddleware::new(redis_client.clone()))
                .layer(AuthMiddleware),
        );

    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", &CONFIG.port))
        .await
        .expect("Failed to bind port");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
