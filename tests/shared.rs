use alpaca_trade_api_rust::prelude::Client;
use httpmock::{
  Method,
  MockServer,
};
pub struct TestContext<'tst> {
  mock_server: &'tst MockServer,
  api_client: &'tst Client,
}

impl<'tst> TestContext<'tst> {
  pub fn new(mock_server: &'tst MockServer, api_client: &'tst Client) -> Self {
    Self {
      mock_server: mock_server,
      api_client: api_client,
    }
  }

  pub async fn setup_endpoint<Fn, Fut>(&self, method: Method, path: &str, status: u16, body: &str, assertion: Fn)
  where
    Fn: FnOnce(&'tst Client) -> Fut,
    Fut: Future<Output = ()>,
  {
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
    });

    assertion(self.api_client).await;
  }
}
