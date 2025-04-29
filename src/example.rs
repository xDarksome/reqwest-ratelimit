use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::future::Future;

struct RateLimiter;

impl reqwest_ratelimit::RateLimiter for RateLimiter {
    async fn acquire_permit(&self) {
        // noop
    }
}

async fn run() {
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(reqwest_ratelimit::all(RateLimiter))
        .build();

    client.get("https://crates.io").send().await.unwrap();
}
