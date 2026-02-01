use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::{
    env,
    num::NonZeroU32,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tower::{Layer, Service};
use tracing::warn;

use crate::utils::jwt::validate_token;

/// Rate limiter state shared across requests
type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

/// Layer that applies rate limiting to requests
#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: SharedRateLimiter,
}

impl RateLimitLayer {
    /// Create a new rate limit layer with configurable limits from environment
    /// 
    /// Environment variables:
    /// - RATE_LIMIT_REQUESTS: Max requests per window (default: 10)
    /// - RATE_LIMIT_WINDOW_SECONDS: Window duration in seconds (default: 60)
    pub fn new() -> Self {
        let requests_per_window: u32 = env::var("RATE_LIMIT_REQUESTS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let window_seconds: u64 = env::var("RATE_LIMIT_WINDOW_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);

        let quota = Quota::with_period(Duration::from_secs(window_seconds))
            .expect("Invalid rate limit period")
            .allow_burst(NonZeroU32::new(requests_per_window).expect("Rate limit must be > 0"));

        let limiter = Arc::new(RateLimiter::direct(quota));

        Self { limiter }
    }
}

impl Default for RateLimitLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

/// Service that enforces rate limiting
#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: SharedRateLimiter,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let limiter = self.limiter.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract user identifier from JWT for logging (optional)
            let user_key = req
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|token| validate_token(token).ok())
                .map(|claims| claims.id)
                .unwrap_or_else(|| "anonymous".to_string());

            // Check rate limit
            match limiter.check() {
                Ok(_) => {
                    // Request allowed, proceed
                    inner.call(req).await
                }
                Err(_) => {
                    // Rate limit exceeded
                    warn!("Rate limit exceeded for user: {}", user_key);
                    
                    let response = (
                        StatusCode::TOO_MANY_REQUESTS,
                        serde_json::json!({
                            "error": "Too many requests, please slow down"
                        })
                        .to_string(),
                    )
                        .into_response();

                    Ok(response)
                }
            }
        })
    }
}
