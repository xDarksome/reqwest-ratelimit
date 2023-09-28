# reqwest-ratelimit

Rate-limit middleware implementation for
[`reqwest-middleware`](https://crates.io/crates/reqwest-middleware).

[![Crates.io](https://img.shields.io/crates/v/reqwest-ratelimit.svg)](https://crates.io/crates/reqwest-ratelimit)
[![Docs.rs](https://docs.rs/reqwest-ratelimit/badge.svg)](https://docs.rs/reqwest-ratelimit)

## Usage

```rust
use async_trait::async_trait;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

struct RateLimiter;

#[async_trait]
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
```
