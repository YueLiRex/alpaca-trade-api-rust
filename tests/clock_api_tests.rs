use alpaca_trade_api_rust::{
  api::ClockApi,
  prelude::Client,
};
use chrono::Datelike;
use httpmock::{
  Method::GET,
  MockServer,
};

mod shared;

#[tokio::test]
async fn test_get_market_calendar_info_should_return_good_1() {
  let mock_server = MockServer::start();
  let base_url = mock_server.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  let test_context = crate::shared::TestContext::new(&mock_server, &api_client);
  let response_body = r#"
  {
    "is_open": true,
    "next_close": "2025-11-14T16:00:00-05:00",
    "next_open": "2025-11-17T09:30:00-05:00",
    "timestamp": "2025-11-14T15:56:46.539081981-05:00"
  }
  "#;

  test_context
    .setup_endpoint(GET, "/v2/clock", 200, response_body, |client| async move {
      match client.get_market_clock_info().await {
        Ok(result) => {
          assert_eq!(result.is_open, true);
          assert_eq!(result.next_open.year(), 2025);
          assert_eq!(result.next_open.month(), 11);
          assert_eq!(result.next_open.day(), 17);
          assert_eq!(result.next_close.year(), 2025);
          assert_eq!(result.next_close.month(), 11);
          assert_eq!(result.next_close.day(), 14);
        }
        Err(err) => panic!("API call failed: {:?}", err),
      }
    })
    .await;
}
