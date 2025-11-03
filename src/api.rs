use crate::{
  models::{
    Account,
    Asset,
    CorporateAction,
    OptionContract,
    Order,
  },
  requests::{
    AssetsQueryParameter,
    CorporateActionsQueryParameter,
    OptionContractsQueryParameter,
    OrdersQueryParameter,
  },
};
use anyhow::{
  Ok,
  anyhow,
};
use reqwest_middleware::ClientWithMiddleware;

pub struct Api {
  base_url: String,
  client: ClientWithMiddleware,
}

impl Api {
  pub fn new(base_url: String, api_key_id: String, api_secret_key: String) -> Self {
    let base_client = reqwest::Client::builder()
      .default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          "APCA-API-KEY-ID",
          reqwest::header::HeaderValue::from_str(&api_key_id).unwrap(),
        );
        headers.insert(
          "APCA-API-SECRET-KEY",
          reqwest::header::HeaderValue::from_str(&api_secret_key).unwrap(),
        );
        headers.insert(
          reqwest::header::ACCEPT,
          reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
          reqwest::header::CONTENT_TYPE,
          reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers
      })
      .build()
      .unwrap();
    let client = reqwest_middleware::ClientBuilder::new(base_client)
      .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
        reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3),
      ))
      .with(reqwest_tracing::TracingMiddleware::default())
      .build();
    Api { base_url, client }
  }

  pub async fn get_account(&self) -> anyhow::Result<Account> {
    let url = format!("{}/v2/account", self.base_url);
    let resp = self.client.get(url).send().await?.json::<Account>().await?;
    Ok(resp)
  }

  pub async fn get_assets(
    &self,
    query_parameter: &AssetsQueryParameter,
  ) -> anyhow::Result<Vec<Asset>> {
    let url = format!("{}/v2/assets", self.base_url);
    let resp = self
      .client
      .get(url)
      .query(query_parameter)
      .send()
      .await?
      .json::<Vec<crate::models::Asset>>()
      .await?;
    Ok(resp)
  }

  pub async fn get_asset_by_symbol_or_id(&self, symbol_or_id: &str) -> anyhow::Result<Asset> {
    let url = format!("{}/v2/assets/{}", self.base_url, symbol_or_id);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<crate::models::Asset>()
      .await?;
    Ok(resp)
  }

  pub async fn get_option_contracts(
    &self,
    query_parameter: &OptionContractsQueryParameter,
  ) -> anyhow::Result<Vec<OptionContract>> {
    let url = format!("{}/v2/options/contracts", self.base_url);
    let resp = self
      .client
      .get(url)
      .query(query_parameter)
      .send()
      .await?
      .json::<Vec<OptionContract>>()
      .await?;
    Ok(resp)
  }

  pub async fn get_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<OptionContract> {
    let url = format!("{}/v2/options/contracts/{}", self.base_url, symbol_or_id);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<OptionContract>()
      .await?;
    Ok(resp)
  }

  pub async fn get_specific_corporate_actions(
    &self,
    id: &str,
  ) -> anyhow::Result<Vec<CorporateAction>> {
    let url = format!("{}/v2/corporate_actions/{}", self.base_url, id);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<Vec<CorporateAction>>()
      .await?;
    Ok(resp)
  }

  pub async fn get_corporate_actions(
    &self,
    query_parameter: &CorporateActionsQueryParameter,
  ) -> anyhow::Result<Vec<CorporateAction>> {
    let url = format!("{}/v2/corporate_actions/announcements", self.base_url);
    let resp = self
      .client
      .get(url)
      .query(query_parameter)
      .send()
      .await?
      .json::<Vec<CorporateAction>>()
      .await?;
    Ok(resp)
  }

  pub async fn create_order(&self, order: &Order) -> anyhow::Result<Order> {
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

  pub async fn get_all_orders(
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

  pub async fn delete_all_orders(&self) -> anyhow::Result<()> {
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

  pub async fn get_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<Order> {
    let url = format!("{}/v2/orders/{}", self.base_url, order_id);
    let resp = self.client.get(url).send().await?.json::<Order>().await?;
    Ok(resp)
  }

  pub async fn replace_order_by_id(
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

  pub async fn delete_order_by_id(&self, order_id: uuid::Uuid) -> anyhow::Result<()> {
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
