use alpaca_trade_api_rust::{
  api::{
    ClosePositionParam,
    PositionApi,
  },
  prelude::Client,
};
use httpmock::{
  Method::{
    DELETE,
    GET,
    POST,
  },
  MockServer,
};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_get_all_open_position_should_return_ok() {
  let ms = MockServer::start();
  let mock_response_body = r#"
    [
      {
        "asset_id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
        "symbol": "AAPL",
        "exchange": "NASDAQ",
        "asset_class": "us_equity",
        "asset_marginable": true,
        "qty": "2",
        "avg_entry_price": "270.23",
        "side": "long",
        "market_value": "557.7",
        "cost_basis": "540.46",
        "unrealized_pl": "17.24",
        "unrealized_plpc": "0.0318987529141842",
        "unrealized_intraday_pl": "0",
        "unrealized_intraday_plpc": "0",
        "current_price": "278.85",
        "lastday_price": "278.85",
        "change_today": "0",
        "qty_available": "2"
      },
      {
        "asset_id": "fc6a5dcd-4a70-4b8d-b64f-d83a6dae9ba4",
        "symbol": "META",
        "exchange": "NASDAQ",
        "asset_class": "us_equity",
        "asset_marginable": true,
        "qty": "3.153314938",
        "avg_entry_price": "634.25",
        "side": "long",
        "market_value": "2043.190414",
        "cost_basis": "1999.989999",
        "unrealized_pl": "43.200415",
        "unrealized_plpc": "0.0216003155123777",
        "unrealized_intraday_pl": "0",
        "unrealized_intraday_plpc": "0",
        "current_price": "647.95",
        "lastday_price": "647.95",
        "change_today": "0",
        "qty_available": "3.153314938"
      }
    ]
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/positions");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client.get_all_open_positions().await {
    Ok(positions) => {
      assert_eq!(positions.len(), 2)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_close_all_positions() {
  let ms = MockServer::start();
  let mock_response_body = r#"
    [
      {
        "symbol": "AAPL",
        "status": "200",
        "body": {
          "id": "de51f21a-d601-4271-9a68-e0db9748f025",
          "client_order_id": "76496f38-94a0-460c-ba00-d1fef33b884a",
          "created_at": "2025-11-10T17:59:37.623341149Z",
          "updated_at": "2025-11-10T17:59:37.624580078Z",
          "submitted_at": "2025-11-10T17:59:37.623341149Z",
          "filled_at": null,
          "expired_at": null,
          "canceled_at": null,
          "failed_at": null,
          "replaced_at": null,
          "replaced_by": null,
          "replaces": null,
          "asset_id": "fc6a5dcd-4a70-4b8d-b64f-d83a6dae9ba4",
          "symbol": "META",
          "asset_class": "us_equity",
          "notional": "2000",
          "qty": null,
          "filled_qty": "0",
          "filled_avg_price": null,
          "order_class": "",
          "order_type": "market",
          "type": "market",
          "side": "buy",
          "position_intent": "buy_to_open",
          "time_in_force": "day",
          "limit_price": null,
          "stop_price": null,
          "status": "pending_new",
          "extended_hours": false,
          "legs": null,
          "trail_percent": null,
          "trail_price": null,
          "hwm": null,
          "subtag": null,
          "source": null,
          "expires_at": "2025-11-10T21:00:00Z"
        }
      }
    ]
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(DELETE)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/positions")
      .query_param("cancel_orders", "false");

    then
      .status(207)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client.clost_all_open_positions(false).await {
    Ok(positions) => {
      assert_eq!(positions.len(), 1)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_get_open_position_by_symbol_or_id_should_return_ok() {
  let ms = MockServer::start();
  let mock_response_body = r#"
    {
      "asset_id": "fc6a5dcd-4a70-4b8d-b64f-d83a6dae9ba4",
      "symbol": "META",
      "exchange": "NASDAQ",
      "asset_class": "us_equity",
      "asset_marginable": true,
      "qty": "3.153314938",
      "avg_entry_price": "634.25",
      "side": "long",
      "market_value": "2043.190414",
      "cost_basis": "1999.989999",
      "unrealized_pl": "43.200415",
      "unrealized_plpc": "0.0216003155123777",
      "unrealized_intraday_pl": "0",
      "unrealized_intraday_plpc": "0",
      "current_price": "647.95",
      "lastday_price": "647.95",
      "change_today": "0",
      "qty_available": "3.153314938"
    }
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/positions/META");

    then
      .status(207)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client.get_open_position_by_symbol_or_id("META").await {
    Ok(position) => {
      assert_eq!(
        position.asset_id,
        Uuid::from_str("fc6a5dcd-4a70-4b8d-b64f-d83a6dae9ba4").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_close_open_position_by_symbol_or_id_should_return_ok() {
  let ms = MockServer::start();
  let mock_response_body = r#"
  {
    "id": "a049613c-2dc6-4fca-ad6a-ccf376d612e2",
    "client_order_id": "string",
    "created_at": "2025-12-01T14:40:03.818Z",
    "updated_at": "2025-12-01T14:40:03.818Z",
    "submitted_at": "2025-12-01T14:40:03.818Z",
    "filled_at": "2025-12-01T14:40:03.818Z",
    "expired_at": "2025-12-01T14:40:03.818Z",
    "canceled_at": "2025-12-01T14:40:03.818Z",
    "failed_at": "2025-12-01T14:40:03.818Z",
    "replaced_at": "2025-12-01T14:40:03.818Z",
    "replaced_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "replaces": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "asset_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "symbol": "string",
    "asset_class": "us_equity",
    "notional": "string",
    "qty": "13.23",
    "filled_qty": "23.23",
    "filled_avg_price": "21313.12",
    "order_class": "bracket",
    "type": "market",
    "side": "buy",
    "time_in_force": "day",
    "limit_price": "1231.12",
    "stop_price": "12.3",
    "status": "new",
    "extended_hours": true,
    "legs": [
      {
        "id": "a049613c-2dc6-4fca-ad6a-ccf376d612e2",
        "client_order_id": "string",
        "created_at": "2025-12-01T14:40:03.818Z",
        "updated_at": "2025-12-01T14:40:03.818Z",
        "submitted_at": "2025-12-01T14:40:03.818Z",
        "filled_at": "2025-12-01T14:40:03.818Z",
        "expired_at": "2025-12-01T14:40:03.818Z",
        "canceled_at": "2025-12-01T14:40:03.818Z",
        "failed_at": "2025-12-01T14:40:03.818Z",
        "replaced_at": "2025-12-01T14:40:03.818Z",
        "replaced_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "replaces": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "asset_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "symbol": "string",
        "asset_class": "us_equity",
        "notional": "string",
        "qty": "123",
        "filled_qty": "312",
        "filled_avg_price": "123.213",
        "order_class": "bracket",
        "type": "market",
        "side": "buy",
        "time_in_force": "day",
        "limit_price": "312.312",
        "stop_price": "12312313.21",
        "status": "new",
        "extended_hours": true,
        "legs": [],
        "trail_percent": "233",
        "trail_price": "2342",
        "hwm": "23423.23",
        "position_intent": "buy_to_open"
      }
    ],
    "trail_percent": "234",
    "trail_price": "234",
    "hwm": "233.2",
    "position_intent": "buy_to_open"
  }
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(DELETE)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/positions/META")
      .query_param("qty", "3.8");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let param = ClosePositionParam::Qty(3.8);
  match api_client
    .close_open_position_by_symbol_or_id("META", param)
    .await
  {
    Ok(positions) => {
      assert_eq!(
        positions.id,
        Uuid::from_str("a049613c-2dc6-4fca-ad6a-ccf376d612e2").unwrap()
      );
      assert_eq!(positions.legs.map(|legs| legs.len()).unwrap_or(0), 1)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_exercise_options_position_should_return_ok() {
  let ms = MockServer::start();
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(POST)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/positions/META/exercise");

    then.status(200).header("Content-Type", "application/json");
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .exercise_option_contract_by_symbol_or_id("META")
    .await
  {
    Ok(_) => {
      endpoint_mock.assert();
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
