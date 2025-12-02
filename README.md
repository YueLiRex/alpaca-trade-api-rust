
# Alpaca-trade-api-rust
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-trade-api-rust)
[![Dual License](https://img.shields.io/badge/license-MIT%20and%20Apache%202.0-blue)](./LICENSE)
[![Coverage](https://img.shields.io/codecov/c/github/YueLiRex/alpaca-trade-api-rust)](https://codecov.io/gh/YueLiRex/alpaca-trade-api-rust)
[![Stars](https://img.shields.io/github/stars/YueLiRex/alpaca-trade-api-rust.svg)](https://github.com/YueLiRex/alpaca-trade-api-rust/stargazers)
[![Issues](https://img.shields.io/github/issues/YueLiRex/alpaca-trade-api-rust.svg)](https://github.com/YueLiRex/alpaca-trade-api-rust/issues)
[![PRs](https://img.shields.io/github/issues-pr/YueLiRex/alpaca-trade-api-rust.svg)](https://github.com/YueLiRex/alpaca-trade-api-rust/pulls)
[![Build](https://github.com/YueLiRex/alpaca-trade-api-rust/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/YueLiRex/alpaca-trade-api-rust/actions/workflows/main.yml)

Alpaca-trade-api-rust as the name stats. It is a rust implementation of [alpaca trading api](https://docs.alpaca.markets/reference/issuetokens). It allows you interact with alpaca with its restful api.

## How to use it
1. Initialize alpaca client with api key and api secret
```rust
use alpaca_trade_api_rust::prelude::Client;

let client = Client::new(
  "localhost:8080".to_string(),
  "testApiKey".to_string(),
  "testApiSecretKey".to_string(),
);
```
2. Call alpaca trading api
```rust
let watch_list = client.create_watch_list(request_body).await?;
```

Welcome to submit a issue to li.yue.rex@gmail.com
