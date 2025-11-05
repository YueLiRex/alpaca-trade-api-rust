use reqwest_middleware::ClientWithMiddleware;

pub struct Client {
  pub base_url: String,
  pub client: ClientWithMiddleware,
}

impl Client {
  pub fn new(base_url: String, api_key_id: String, api_secret_key: String) -> Self {
    let base_client = reqwest::Client::builder()
      .default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          "APCA-API-KEY-ID",
          reqwest::header::HeaderValue::from_str(&api_key_id).unwrap(),
        );
        headers.insert(
          "APCA-API-SECRET-KEY",
          reqwest::header::HeaderValue::from_str(&api_secret_key).unwrap(),
        );
        headers.insert(
          reqwest::header::ACCEPT,
          reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
          reqwest::header::CONTENT_TYPE,
          reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers
      })
      .build()
      .unwrap();
    let client = reqwest_middleware::ClientBuilder::new(base_client)
      .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
        reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3),
      ))
      .with(reqwest_tracing::TracingMiddleware::default())
      .build();
    Client { base_url, client }
  }
}
