use crate::{
  api::utils::ComaSeparatedStrings,
  client::Client,
  models::{
    ErrorResponse,
    Order,
    OrderClass,
    PositionIntent,
    TimeInForce,
    enums::{
      OrderType,
      Side,
    },
    utils::{
      Money,
      NumberAsString,
    },
  },
};
use anyhow::bail;
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

pub trait OrderApi {
  fn create_order(&self, order: &OrderRequestBody) -> impl Future<Output = anyhow::Result<Order>>;

  fn get_all_orders(
    &self,
    query_parameter: &AllOrdersQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<Order>>>;

  fn delete_all_orders(&self) -> impl Future<Output = anyhow::Result<Vec<DeleteAllOrdersResponse>>>;

  fn get_order_by_client_order_id(&self, client_order_id: &str) -> impl Future<Output = anyhow::Result<Order>>;

  fn get_order_by_id(&self, id: &Uuid) -> impl Future<Output = anyhow::Result<Order>>;

  fn replace_order_by_id(
    &self,
    order_id: &Uuid,
    order: &ReplaceOrderByIdRequestBody,
  ) -> impl Future<Output = anyhow::Result<Order>>;

  fn delete_order_by_id(&self, order_id: &Uuid) -> impl Future<Output = anyhow::Result<()>>;
}

impl OrderApi for Client {
  async fn create_order(&self, order_request_body: &OrderRequestBody) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders", self.base_url);
    match self.client.post(url).json(order_request_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let order = response.json::<Order>().await?;
          Ok(order)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(err) => bail!(err),
    }
  }

  async fn get_all_orders(&self, query_parameter: &AllOrdersQueryParameter) -> anyhow::Result<Vec<Order>> {
    let url = format!("{}/v2/orders", self.base_url);
    match self.client.get(url).query(query_parameter).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let orders = response.json::<Vec<Order>>().await?;
          Ok(orders)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(err) => bail!(err),
    }
  }

  async fn delete_all_orders(&self) -> anyhow::Result<Vec<DeleteAllOrdersResponse>> {
    let url = format!("{}/v2/orders", self.base_url);
    match self.client.delete(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<Vec<DeleteAllOrdersResponse>>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_order_by_client_order_id(&self, client_order_id: &str) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders:by_client_order_id", self.base_url);
    let query_param = GetOrderByClientIdParameter { client_order_id };
    match self.client.get(url).query(&query_param).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<Order>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_order_by_id(&self, order_id: &Uuid) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<Order>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn replace_order_by_id(
    &self,
    order_id: &Uuid,
    replace_order_body: &ReplaceOrderByIdRequestBody,
  ) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    match self.client.patch(url).json(&replace_order_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<Order>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(err) => bail!(err),
    }
  }

  async fn delete_order_by_id(&self, order_id: &Uuid) -> anyhow::Result<()> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    match self.client.delete(url).send().await {
      Ok(response) => {
        if response.status().is_client_error() {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        } else {
          Ok(())
        }
      }
      Err(error) => bail!(error),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct OrderRequestBody {
  pub symbol: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub qty: Option<NumberAsString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notional: Option<Money>,
  pub side: Side,
  #[serde(rename = "type")]
  pub _type: OrderType,
  pub time_in_force: TimeInForce,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit_price: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_price: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_percent: Option<Money>,
  pub extended_hours: bool,
  pub client_order_id: Option<String>,
  pub order_class: Option<OrderClass>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub legs: Vec<Leg>,
  pub take_profit: Option<TakeProfit>,
  pub stop_loss: Option<StopLoss>,
  pub position_intent: Option<PositionIntent>,
}

#[derive(Debug, Serialize)]
pub struct Leg {
  pub side: Side,
  pub position_intent: PositionIntent,
  pub symbol: String,
  pub ratio_qty: String,
}

#[derive(Debug, Serialize)]
pub struct TakeProfit {
  pub limit_price: Money,
}

#[derive(Debug, Serialize)]
pub struct StopLoss {
  pub stop_price: Money,
  pub limit_price: Money,
}

#[derive(Debug, Serialize)]
pub struct AllOrdersQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<OrderStatus>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub until: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub direction: Option<OrdersDirection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub symbols: Option<ComaSeparatedStrings>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub side: Option<Side>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_class: Option<ComaSeparatedStrings>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before_order_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteAllOrdersResponse {
  pub id: Uuid,
  pub status: u16,
}

#[derive(Debug, Serialize)]
pub struct GetOrderByClientIdParameter<'a> {
  client_order_id: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ReplaceOrderByIdRequestBody {
  pub qty: NumberAsString,
  pub time_in_force: TimeInForce,
  pub limit_price: Money,
  pub stop_price: Money,
  pub trail: Money,
  pub client_order_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
  Open,
  Closed,
  All,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrdersDirection {
  Asc,
  Desc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderAssetClass {
  UsEquity,
  UsOption,
  Crypto,
}

#[cfg(test)]
mod tests {
  use crate::{
    api::{
      OrderRequestBody,
      ReplaceOrderByIdRequestBody,
      StopLoss,
      TakeProfit,
    },
    models::{
      OrderClass,
      PositionIntent,
      TimeInForce,
      enums::{
        OrderType,
        Side,
      },
      utils::{
        Money,
        NumberAsString,
      },
    },
  };

  #[test]
  fn order_request_serialization_test() {
    use serde_json;

    let order_request = OrderRequestBody {
      symbol: "AAPL".to_string(),
      qty: Some(NumberAsString::from_f64(43.0)),
      notional: None,
      side: Side::Buy,
      _type: OrderType::Limit,
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

    let serialized = serde_json::to_string(&order_request).unwrap();

    let expected = r#"{"symbol":"AAPL","qty":"43","side":"buy","type":"limit","time_in_force":"gtc","limit_price":"32","stop_price":"43","extended_hours":false,"client_order_id":null,"order_class":"simple","take_profit":{"limit_price":"30"},"stop_loss":{"stop_price":"20.43","limit_price":"23.23"},"position_intent":"buy_to_close"}"#;
    assert_eq!(serialized, expected);
  }

  #[test]
  fn orders_parameter_serialization_test() {
    use crate::api::*;
    use serde_json;

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

    let serialized = serde_json::to_string(&orders_query).unwrap();

    let expected = r#"{"status":"open","limit":50,"direction":"desc","nested":true,"symbols":"AAPL,TSLA","asset_class":"us_option,crypto"}"#;
    assert_eq!(serialized, expected);
  }

  #[test]
  fn replace_order_by_id_request_body_serialization() {
    let body = ReplaceOrderByIdRequestBody {
      qty: NumberAsString::from_f64(4.0),
      time_in_force: TimeInForce::DAY,
      limit_price: Money::from_f64(100.0),
      stop_price: Money::from_f64(90.0),
      trail: Money::from_f64(10.0),
      client_order_id: String::from("test_client_order_id"),
    };

    let serialized = serde_json::to_string(&body).unwrap();
    let expected = r#"{"qty":"4","time_in_force":"day","limit_price":"100","stop_price":"90","trail":"10","client_order_id":"test_client_order_id"}"#;

    assert_eq!(serialized, expected)
  }
}
