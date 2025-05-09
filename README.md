# reqwest-ratelimit

Rate-limit middleware implementation for
[`reqwest-middleware`](https://crates.io/crates/reqwest-middleware).

This crate is glue code for implementing custom ratelimiters.
For a batteries included example please refer to [reqwest-leaky-bucket](https://crates.io/crates/reqwest-leaky-bucket).

[![Crates.io](https://img.shields.io/crates/v/reqwest-ratelimit.svg)](https://crates.io/crates/reqwest-ratelimit)
[![Docs.rs](https://docs.rs/reqwest-ratelimit/badge.svg)](https://docs.rs/reqwest-ratelimit)

## Usage

```rust
use std::future::Future;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

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
```
