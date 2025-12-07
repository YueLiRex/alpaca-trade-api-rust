use alpaca_trade_api_rust::{
  api::*,
  prelude::Client,
};
use chrono::{
  TimeZone,
  Utc,
};
use httpmock::{
  Method::GET,
  MockServer,
};

#[tokio::test]
async fn get_portfolio_should_return_ok() {
  let ms = MockServer::start();
  let mock_response_body = r#"
    {
      "timestamp": [
        0
      ],
      "equity": [
        0
      ],
      "profit_loss": [
        0
      ],
      "profit_loss_pct": [
        0.001,
        0.002
      ],
      "base_value": 100000,
      "base_value_asof": "2023-10-20",
      "timeframe": "15Min",
      "cashflow": {}
    }
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/account/portfolio/history")
      .query_param("period", "30D")
      .query_param("timeframe", "5Min")
      .query_param("intraday_reporting", "market_hours")
      .query_param("start", "2025-01-21T05:32:12Z")
      .query_param("pnl_reset", "per_day")
      .query_param("end", "2025-02-21T05:32:12Z")
      .query_param("extended_hours", "true")
      .query_param("cashflow_types", "ALL");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  let start = Utc.with_ymd_and_hms(2025, 1, 21, 5, 32, 12).unwrap();
  let end = Utc.with_ymd_and_hms(2025, 2, 21, 5, 32, 12).unwrap();
  let query_params = &PortfolioHistoryQueryParameter {
    period: Some(HistoryPeriod::Day(30)),
    timeframe: Some(HistoryTimeFrame::Minute(5)),
    intraday_reporting: Some(IntradayReporting::MarketHours),
    start: Some(start),
    pnl_reset: Some(PnlReset::PerDay),
    end: Some(end),
    extended_hours: Some("true".to_string()),
    cashflow_types: Some(CashflowTypes::All),
  };

  match api_client.get_portfolio_history(query_params).await {
    Ok(portfolio_history) => {
      assert_eq!(portfolio_history.timestamp.len(), 1)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
