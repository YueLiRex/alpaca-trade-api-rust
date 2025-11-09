use alpaca_trade_api_rust::{
  api::{
    AllOrdersQueryParameter,
    ComaSeparatedStrings,
    OrderApi,
    OrderStatus,
    OrdersDirection,
  },
  prelude::{
    Client,
    OrderClass,
    PositionIntent,
  },
};
use httpmock::{
  Method::GET,
  MockServer,
};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_get_specific_option_contracts_should_return_one() {
  let ms = MockServer::start();
  let mock_response_body = r#"
  [
    {
      "id": "bff50af3-8fb4-4a7e-8ffe-6527cdf6b453",
      "client_order_id": "1df8303c-e84c-402c-85db-819008f479de",
      "created_at": "2025-11-09T14:59:48.961029974Z",
      "updated_at": "2025-11-09T14:59:48.962700853Z",
      "submitted_at": "2025-11-09T14:59:48.961029974Z",
      "filled_at": null,
      "expired_at": null,
      "canceled_at": null,
      "failed_at": null,
      "replaced_at": null,
      "replaced_by": null,
      "replaces": null,
      "asset_id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
      "symbol": "AAPL",
      "asset_class": "us_equity",
      "notional": null,
      "qty": "2",
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
      "status": "accepted",
      "extended_hours": false,
      "legs": null,
      "trail_percent": null,
      "trail_price": null,
      "hwm": null,
      "subtag": null,
      "source": null,
      "expires_at": "2025-11-10T21:00:00Z"
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
      .path("/v2/orders");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let orders_query = AllOrdersQueryParameter {
    status: Some(OrderStatus::Open),
    limit: Some(50),
    after: None,
    until: None,
    direction: Some(OrdersDirection::Desc),
    nested: Some(true),
    symbols: Some(ComaSeparatedStrings {
      values: vec!["AAPL", "TSLA"],
    }),
    side: None,
    asset_class: Some(ComaSeparatedStrings {
      values: vec!["us_option", "crypto"],
    }),
    before_order_id: None,
    after_order_id: None,
  };

  match api_client.get_all_orders(&orders_query).await {
    Ok(orders) => {
      assert_eq!(orders.len(), 1);
      assert_eq!(orders.first().unwrap().order_class, OrderClass::Empty);
      assert_eq!(
        orders.first().unwrap().position_intent,
        PositionIntent::BuyToOpen
      );
      assert_eq!(
        orders.first().unwrap().id,
        Uuid::from_str("bff50af3-8fb4-4a7e-8ffe-6527cdf6b453").unwrap()
      );
      assert_eq!(
        orders.first().unwrap().qty.as_ref().map(|q| q.value()),
        Some(2)
      );
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
