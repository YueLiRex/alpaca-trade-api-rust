use crate::{
  api::utils::{
    ComaSeparatedStrings,
    DefaultBoolean,
  },
  client::Client,
  models::{
    Order,
    TimeInForce,
    enums::{
      Side,
      Type,
    },
  },
};
use anyhow::anyhow;
use serde::Serialize;

pub trait OrderApi {
  fn create_order(&self, order: &Order) -> impl Future<Output = anyhow::Result<Order>>;

  fn get_all_orders(
    &self,
    query_parameter: &OrdersQueryParameter,
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
    query_parameter: &OrdersQueryParameter,
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
pub struct OrderRequest {
  pub symbol: String,
  pub qty: u16,
  pub side: Side,
  #[serde(rename = "type")]
  pub _type: Type,
  pub time_in_force: TimeInForce,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_percent: Option<f64>,
  pub extended_hours: DefaultBoolean,
}

#[derive(Debug, Serialize)]
pub struct OrdersQueryParameter {
  pub status: OrderStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub until: Option<String>,
  pub direction: OrdersDirection,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nested: Option<bool>,
  pub symbols: ComaSeparatedStrings,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub side: Option<String>,
  pub asset_class: ComaSeparatedStrings,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before_order_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_order_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum OrderStatus {
  Open,
  Closed,
  All,
}

#[derive(Debug, Serialize)]
pub enum OrdersDirection {
  Asc,
  Desc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all="snake_case")]
pub enum OrderAssetClass {
  UsEquity,
  UsOption,
  Crypto,
}
