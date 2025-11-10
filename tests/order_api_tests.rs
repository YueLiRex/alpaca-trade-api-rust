use alpaca_trade_api_rust::{
  api::{
    AllOrdersQueryParameter,
    ComaSeparatedStrings,
    OrderApi,
    OrderRequestBody,
    OrderStatus,
    OrdersDirection,
    StopLoss,
    TakeProfit,
  },
  prelude::{
    Client,
    OrderClass,
    PositionIntent,
    TimeInForce,
    enums::{
      Side,
      Type,
    },
    utils::{
      IntAsString,
      Money,
    },
  },
};
use httpmock::{
  Method::{
    GET,
    POST,
  },
  MockServer,
  server::matchers::readers::expectations::body,
};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_get_orders_should_return_order_list() {
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

#[tokio::test]
async fn test_create_order_should_return_ok() {
  let ms = MockServer::start();
  let mock_response_body = r#"
  {
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
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(POST)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/orders")
      .body(r#"{"symbol":"META","qty":"43","side":"buy","type":"limit","time_in_force":"gtc","limit_price":"32","stop_price":"43","extended_hours":false,"client_order_id":null,"order_class":"simple","take_profit":{"limit_price":"30"},"stop_loss":{"stop_price":"20.43","limit_price":"23.23"},"position_intent":"buy_to_close"}"#);
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let order_request_body = OrderRequestBody {
    symbol: "META".to_string(),
    qty: Some(IntAsString::from_u32(43)),
    notional: None,
    side: Side::Buy,
    _type: Type::Limit,
    time_in_force: TimeInForce::GTC,
    limit_price: Some(Money::from_f64(32.0)),
    stop_price: Some(Money::from_f64(43.0)),
    trail_price: None,
    trail_percent: None,
    extended_hours: Default::default(),
    client_order_id: None,
    order_class: Some(OrderClass::Simple),
    legs: vec![],
    take_profit: Some(TakeProfit {
      limit_price: Money::from_f64(30.0),
    }),
    stop_loss: Some(StopLoss {
      stop_price: Money::from_f64(20.43),
      limit_price: Money::from_f64(23.23),
    }),
    position_intent: Some(PositionIntent::BuyToClose),
  };

  match api_client.create_order(&order_request_body).await {
    Ok(order) => {
      assert_eq!(
        order.id.to_string().as_str(),
        "de51f21a-d601-4271-9a68-e0db9748f025"
      );
      assert_eq!(order._type, Type::Market);
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
