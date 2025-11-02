#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use httpmock::Method::GET;
    use alpaca_trade_api_rust::prelude::*;

    #[tokio::test]
    async fn test_get_account() {
        // Start a mock server
        let server = MockServer::start();

        println!("{}", server.address());

        // Create a mock for the /v2/account endpoint
        let account_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/account");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{
                    "id": "123e4567-e89b-12d3-a456-426614174000",
                    "status": "ACTIVE",
                    "currency": "USD",
                    "cash": 10000.0,
                    "portfolio_value": 15000.0,
                    "buying_power": 30000.0,
                    "daytrading_buying_power": 60000.0,
                    "effective_buying_power": 30000.0,
                    "options_buying_power": 0.0,
                    "options_approved_level": 1,
                    "options_trading_level": 1,
                    "non_marginable_buying_power": 10000.0,
                    "bod_dtbp": 0.0,
                    "regt_buying_power": 30000.0,
                    "shorting_enabled": false,
                    "trade_suspended_by_user": false,
                    "account_blocked": false,
                    "created_at": "2023-01-01T00:00:00Z",
                    "trade_account_type": "individual",
                    "pattern_day_trader_status": false,
                    "multiplier": 2.0,
                    "equity": 15000.0,
                    "last_equity": 14500.0,
                    "long_market_value": 8000.0,
                    "short_market_value": 0.0,
                    "initial_margin": 4000.0,
                    "maintenance_margin": 3000.0,
                    "sma": 5000.0,
                    "daytrade_count": 0,
                    "non_equity_margin_requirement": 0.0,
                    "cash_withdrawable": 5000.0
                }"#);
        });
  }
}
