//! Middleware to rate-limit requests built on [`reqwest_middleware`].
//!
//! You're expected to provide your own [`RateLimiter`] implementation.
//!
//! ## Example
//!
//! ```
//! use std::future::Future;
//! use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
//!
//! struct RateLimiter;
//!
//! impl reqwest_ratelimit::RateLimiter for RateLimiter {
//!     fn acquire_permit(&self) -> impl Future<Output = ()> + Send + 'static {
//!         async { () } // noop
//!     }
//! }
//!
//! async fn run() {
//!     let client = ClientBuilder::new(reqwest::Client::new())
//!         .with(reqwest_ratelimit::all(RateLimiter))
//!         .build();
//!
//!     client.get("https://crates.io").send().await.unwrap();
//! }
//! ```
use std::future::Future;

use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Next, Result};

/// Request rate limiter.
pub trait RateLimiter: Send + Sync + 'static {
    /// Acquires a permit to issue the next request.
    fn acquire_permit(&self) -> impl Future<Output = ()> + Send + 'static;
}

/// Creates a new [`Middleware`] rate-limiting all requests using the provided [`RateLimiter`].
pub fn all<R>(rate_limiter: R) -> Middleware<R> {
    Middleware { rate_limiter }
}

/// Request rate-limiting middleware.
pub struct Middleware<R> {
    rate_limiter: R,
}

#[async_trait]
impl<R: RateLimiter> reqwest_middleware::Middleware for Middleware<R> {
    async fn handle(
        &self,
        req: Request,
        extensions: &'_ mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        self.rate_limiter.acquire_permit().await;
        next.run(req, extensions).await
    }
}
