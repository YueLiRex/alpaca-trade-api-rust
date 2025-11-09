use alpaca_trade_api_rust::prelude::Client;
use httpmock::{
  Method,
  MockServer,
};
pub struct TestContext {
  pub mock_server: MockServer,
  pub api_client: Client,
}

impl TestContext {
  pub fn new() -> Self {
    let ms = MockServer::start();
    let client = Client::new(
      ms.base_url(),
      "test_key".to_string(),
      "test_secret".to_string(),
    );
    Self {
      mock_server: MockServer::start(),
      api_client: client,
    }
  }

  pub fn base_url(&self) -> String {
    self.mock_server.base_url()
  }

  pub fn mock_endpoint(
    &self,
    method: Method,
    path: &str,
    status: u16,
    body: &str,
  ) -> httpmock::Mock {
    self.mock_server.mock(|when, then| {
      when
        .method(method)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("APCA-API-KEY-ID", "test_key")
        .header("APCA-API-SECRET-KEY", "test_secret")
        .path(path);
      then
        .status(status)
        .header("Content-Type", "application/json")
        .body(body);
    })
  }
}
