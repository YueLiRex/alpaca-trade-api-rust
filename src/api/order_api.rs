use crate::{
  api::utils::ComaSeparatedStrings,
  client::Client,
  models::{
    Order,
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
use anyhow::anyhow;
use serde::Serialize;

pub trait OrderApi {
  fn create_order(&self, order: &Order) -> impl Future<Output = anyhow::Result<Order>>;

  fn get_all_orders(
    &self,
    query_parameter: &AllOrdersQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<Order>>>;

  fn delete_all_orders(&self) -> impl Future<Output = anyhow::Result<()>>;

  fn get_order_by_id(&self, order_id: uuid::Uuid) -> impl Future<Output = anyhow::Result<Order>>;

  fn replace_order_by_id(
    &self,
    order_id: uuid::Uuid,
    order: &Order,
  ) -> impl Future<Output = anyhow::Result<Order>>;

  fn delete_order_by_id(&self, order_id: uuid::Uuid) -> impl Future<Output = anyhow::Result<()>>;
}

impl OrderApi for Client {
  async fn create_order(&self, order: &Order) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders", self.base_url);
    let resp = self
      .client
      .post(url)
      .json(order)
      .send()
      .await?
      .json::<Order>()
      .await?;
    Ok(resp)
  }

  async fn get_all_orders(
    &self,
    query_parameter: &AllOrdersQueryParameter,
  ) -> anyhow::Result<Vec<Order>> {
    let url = format!("{}/v2/orders", self.base_url);
    let resp = self
      .client
      .get(url)
      .query(query_parameter)
      .send()
      .await?
      .json::<Vec<Order>>()
      .await?;
    Ok(resp)
  }

  async fn delete_all_orders(&self) -> anyhow::Result<()> {
    let url = format!("{}/v2/orders", self.base_url);
    let response = self.client.delete(url).send().await?;
    match response.status() {
      status if status.is_client_error() => {
        Err(anyhow!("Failed to delete all orders: HTTP {}", status))
      }
      status if status.is_server_error() => Err(anyhow!(
        "Server error when deleting all orders: HTTP {}",
        status
      )),
      _ => Ok(()),
    }
  }

  async fn get_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    let resp = self.client.get(url).send().await?.json::<Order>().await?;
    Ok(resp)
  }

  async fn replace_order_by_id(
    &self,
    order_id: uuid::Uuid,
    order: &Order,
  ) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    let resp = self
      .client
      .put(url)
      .json(order)
      .send()
      .await?
      .json::<Order>()
      .await?;
    Ok(resp)
  }

  async fn delete_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<()> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    let response = self.client.delete(url).send().await?;
    match response.status() {
      status if status.is_client_error() => Err(anyhow!(
        "Failed to delete order {}: HTTP {}",
        order_id,
        status
      )),
      status if status.is_server_error() => Err(anyhow!(
        "Server error when deleting order {}: HTTP {}",
        order_id,
        status
      )),
      _ => Ok(()),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct OrderRequestBody {
  pub symbol: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub qty: Option<IntAsString>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notional: Option<Money>,
  pub side: Side,
  #[serde(rename = "type")]
  pub _type: Type,
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
      StopLoss,
      TakeProfit,
    },
    models::{
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

  #[test]
  fn order_request_serialization_test() {
    use serde_json;

    let order_request = OrderRequestBody {
      symbol: "AAPL".to_string(),
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
}
