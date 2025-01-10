use axum::response::IntoResponse;
use axum::{extract::Request, http::HeaderValue, response::Response};
use futures_util::future::BoxFuture;
use redis::{Client, Commands};
use std::{
    sync::Arc,
    task::{Context, Poll},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;
use tower::{Layer, Service};

use crate::{errors::app_error::ApiError, models::rate_limit::RateLimit};

#[derive(Clone)]
pub struct RateLimitMiddleware {
    redis_client: Arc<Mutex<Client>>,
}

impl RateLimitMiddleware {
    pub fn new(redis_client: Client) -> Self {
        RateLimitMiddleware {
            redis_client: Arc::new(Mutex::new(redis_client)),
        }
    }
}

impl<S> Layer<S> for RateLimitMiddleware {
    type Service = RateLimitMiddlewareService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddlewareService {
            inner,
            redis_client: self.redis_client.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitMiddlewareService<S> {
    inner: S,
    redis_client: Arc<Mutex<Client>>,
}

impl<S, B> Service<Request<B>> for RateLimitMiddlewareService<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(_)) => Poll::Ready(Ok(())), // this is temporary solution for error handling during poll_ready
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let ip_address = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|header| header.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let resource = request.uri().path().to_string();

        log::debug!("{ip_address} - {resource}");
        let redis_client = self.redis_client.clone();
        let future = self.inner.call(request);
        Box::pin(async move {
            match check_rate_limit(ip_address, resource, redis_client).await {
                Ok(rate_limit) => match future.await {
                    Ok(mut response) => {
                        add_rate_limit_headers(&mut response, &rate_limit);
                        Ok(response)
                    }
                    Err(_) => Ok(ApiError::TooManyRequests.into_response()),
                },
                Err(api_error) => {
                    Ok(api_error.into_response()) // Return 429 Too Many Requests
                }
            }
        })
    }
}

async fn check_rate_limit(
    ip_address: String,
    resource: String,
    redis_client: Arc<Mutex<Client>>,
) -> Result<RateLimit, ApiError> {
    let mut con = redis_client
        .lock()
        .await
        .get_connection()
        .map_err(|_| ApiError::TooManyRequests)?;

    let current_time = chrono::Utc::now().timestamp() as u64;
    let ip_resource_key = format!("rate_limit:{}:{}", ip_address, resource);

    // Fetch rate limit data from Redis
    let rate_limit_data: Option<String> = con
        .get(&ip_resource_key)
        .map_err(|_| ApiError::TooManyRequests)?;

    let mut rate_limit: RateLimit = match rate_limit_data {
        Some(data) => serde_json::from_str(&data).map_err(|_| ApiError::TooManyRequests)?,
        None => RateLimit::new(60, 60), // 60 requests per 60 seconds
    };

    if rate_limit.is_rate_limited(current_time) {
        return Err(ApiError::TooManyRequests);
    }

    rate_limit.increment(current_time);

    // Update rate limit data in Redis
    let serialized = serde_json::to_string(&rate_limit).map_err(|_| ApiError::TooManyRequests)?;
    con.set_ex::<_, _, ()>(&ip_resource_key, serialized, 60)
        .map_err(|_| ApiError::TooManyRequests)?;

    Ok(rate_limit)
}

fn add_rate_limit_headers(response: &mut Response, rate_limit: &RateLimit) {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(&rate_limit.max_requests.to_string()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str(&rate_limit.remaining_requests(current_time).to_string()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from_str(
            &(current_time + rate_limit.time_until_reset(current_time)).to_string(),
        )
        .unwrap(),
    );
}
