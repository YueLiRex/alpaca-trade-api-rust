use crate::{
  client::Client,
  models::Order,
  requests::OrdersQueryParameter,
};
use anyhow::anyhow;

pub trait OrderApi {
  async fn create_order(&self, order: &Order) -> anyhow::Result<Order>;

  async fn get_all_orders(
    &self,
    query_parameter: &OrdersQueryParameter,
  ) -> anyhow::Result<Vec<Order>>;

  async fn delete_all_orders(&self) -> anyhow::Result<()>;

  async fn get_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<Order>;

  async fn replace_order_by_id(&self, order_id: uuid::Uuid, order: &Order)
  -> anyhow::Result<Order>;

  async fn delete_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<()>;
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
