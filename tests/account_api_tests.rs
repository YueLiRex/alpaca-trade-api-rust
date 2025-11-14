use alpaca_trade_api_rust::{
  api::AccountApi,
  prelude::Client,
};
use httpmock::{
  Method::GET,
  MockServer,
};

#[tokio::test]
async fn test_get_account_should_return_account() {
  let server = MockServer::start();

  let account_mock = server.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/account");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(
        r#"{
            "id": "fff0e281-2a5a-4b97-8dcc-790a439a49b2",
            "admin_configurations": {},
            "user_configurations": null,
            "account_number": "PA39J45DA4AZ",
            "status": "ACTIVE",
            "crypto_status": "ACTIVE",
            "options_approved_level": 3,
            "options_trading_level": 3,
            "currency": "USD",
            "buying_power": "200000",
            "regt_buying_power": "200000",
            "daytrading_buying_power": "0",
            "effective_buying_power": "200000",
            "non_marginable_buying_power": "100000",
            "options_buying_power": "100000",
            "bod_dtbp": "0",
            "cash": "100000",
            "accrued_fees": "0",
            "portfolio_value": "100000",
            "pattern_day_trader": false,
            "trading_blocked": false,
            "transfers_blocked": false,
            "account_blocked": false,
            "created_at": "2024-10-31T15:46:03.666425Z",
            "trade_suspended_by_user": false,
            "multiplier": "2",
            "shorting_enabled": true,
            "equity": "100000",
            "last_equity": "100000",
            "long_market_value": "0",
            "short_market_value": "0",
            "position_market_value": "0",
            "initial_margin": "0",
            "maintenance_margin": "0",
            "last_maintenance_margin": "0",
            "sma": "100000",
            "daytrade_count": 0,
            "balance_asof": "2025-10-31",
            "crypto_tier": 1,
            "intraday_adjustments": "0",
            "pending_reg_taf_fees": "0"
          }"#,
      );
  });

  let base_url = server.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  match api.get_account().await {
    Ok(account) => {
      assert_eq!(
        account.id.to_string(),
        "fff0e281-2a5a-4b97-8dcc-790a439a49b2"
      );
      assert_eq!(account.cash.value(), 100000.0);
      assert_eq!(account.portfolio_value.value(), 100000.0);
    }
    Err(e) => {
      account_mock.assert();
      panic!("API call failed: {:?}", e)
    }
  }
}

#[tokio::test]
async fn test_get_account_should_return_clientside_error() {
  let server = MockServer::start();

  let account_mock = server.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/account");
    then
      .header("Content-Type", "application/json")
      .status(404)
      .body("Account not found");
  });

  let base_url = server.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  match api.get_account().await {
    Ok(_) => {
      panic!("Expect failure for this test")
    }
    Err(e) => {
      account_mock.assert();
      assert_eq!(
        e.to_string().as_str(),
        "code: 404, message: \"Account not found\""
      )
    }
  }
}
