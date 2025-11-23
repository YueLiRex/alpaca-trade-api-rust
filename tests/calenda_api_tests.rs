use alpaca_trade_api_rust::{
  api::{
    CalendarApi,
    CalendarApiQueryParameter,
  },
  prelude::Client,
};
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
  let response_body = r#"[
            {
              "close": "16:00",
              "date": "1970-01-02",
              "open": "09:30",
              "session_close": "2000",
              "session_open": "0400",
              "settlement_date": "1970-01-09"
            },
            {
              "close": "16:00",
              "date": "1970-01-05",
              "open": "09:30",
              "session_close": "2000",
              "session_open": "0400",
              "settlement_date": "1970-01-12"
            },
            {
              "close": "16:00",
              "date": "1970-01-06",
              "open": "09:30",
              "session_close": "2000",
              "session_open": "0400",
              "settlement_date": "1970-01-13"
            }
          ]"#;
  let parameter = CalendarApiQueryParameter {
    start: None,
    end: None,
    date_type: None,
  };

  test_context
    .setup_endpoint(
      GET,
      "/v2/calendar",
      200,
      response_body,
      |client| async move {
        match client.get_market_calendar_info(&parameter).await {
          Ok(result) => {
            assert_eq!(result.len(), 3)
          }
          Err(err) => panic!("API call failed: {:?}", err),
        }
      },
    )
    .await;
}
