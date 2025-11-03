extern crate alpaca_trade_api_rust;

#[cfg(test)]
mod tests {
  #[test]
  fn order_request_serialization_test() {
    use alpaca_trade_api_rust::{
      prelude::*,
      requests::OrderRequest,
    };
    use serde_json;

    let order_request = OrderRequest {
      symbol: "AAPL".to_string(),
      qty: 10,
      side: Side::BUY,
      _type: Type::LIMIT,
      time_in_force: TimeInForce::GTC,
      limit_price: Some(150.0),
      stop_price: Some(3.54),
      trail_price: None,
      trail_percent: None,
      extended_hours: Default::default(),
    };

    let serialized = serde_json::to_string(&order_request).unwrap();

    let expected = r#"{"symbol":"AAPL","qty":10,"side":"buy","type":"limit","time_in_force":"gtc","limit_price":150.0,"stop_price":3.54,"extended_hours":false}"#;
    assert_eq!(serialized, expected);
  }

  #[test]
  fn orders_parameter_serialization_test() {
    use alpaca_trade_api_rust::requests::*;
    use serde_json;

    let orders_query = OrdersQueryParameter {
      status: OrderStatus::OPEN,
      limit: Some(50),
      after: None,
      until: None,
      direction: OrdersDirection::DESC,
      nested: Some(true),
      symbols: ComaSeparatedStrings {
        values: vec!["AAPL", "TSLA"],
      },
      side: None,
      asset_class: ComaSeparatedStrings {
        values: vec!["us_option", "crypto"],
      },
      before_order_id: None,
      after_order_id: None,
    };

    let serialized = serde_json::to_string(&orders_query).unwrap();

    let expected = r#"{"status":"OPEN","limit":50,"direction":"DESC","nested":true,"symbols":"AAPL,TSLA","asset_class":"us_option,crypto"}"#;
    assert_eq!(serialized, expected);
  }
}
