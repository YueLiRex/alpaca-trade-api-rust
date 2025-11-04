use crate::{
  client::Client,
  models::Position,
};
use anyhow::anyhow;

pub trait PositionApi {
  fn get_all_open_positions(&self) ->impl Future<Output = anyhow::Result<Vec<Position>>>;

  async fn get_open_position_by_symbol_or_id(&self, symbol_or_id: &str)
  -> anyhow::Result<Position>;

  async fn close_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
    qty: f64,
    percentage: f64,
  ) -> anyhow::Result<()>;

  async fn exercise_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<()>;

  async fn clost_all_open_positions(&self) -> anyhow::Result<()>;
}

impl PositionApi for Client {
  async fn get_all_open_positions(&self) -> anyhow::Result<Vec<Position>> {
    let url = format!("{}/v2/positions", self.base_url);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<Vec<Position>>()
      .await?;
    Ok(resp)
  }

  async fn clost_all_open_positions(&self) -> anyhow::Result<()> {
    todo!()
  }

  async fn get_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<Position> {
    let url = format!("{}/v2/positions/{}", self.base_url, symbol_or_id);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<Position>()
      .await?;
    Ok(resp)
  }

  async fn close_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
    qty: f64,
    percentage: f64,
  ) -> anyhow::Result<()> {
    let url = format!("{}/v2/positions/{}", self.base_url, symbol_or_id);
    let response = self
      .client
      .delete(url)
      .query(&[
        ("qty", qty.to_string()),
        ("percentage", percentage.to_string()),
      ])
      .send()
      .await?;
    match response.status() {
      status if status.is_client_error() => Err(anyhow!(
        "Failed to close position {}: HTTP {}",
        symbol_or_id,
        status
      )),
      status if status.is_server_error() => Err(anyhow!(
        "Server error when closing position {}: HTTP {}",
        symbol_or_id,
        status
      )),
      _ => Ok(()),
    }
  }

  async fn exercise_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<()> {
    let url = format!("{}/v2/positions/{}/exercise", self.base_url, symbol_or_id);
    let response = self.client.post(url).send().await?;
    match response.status() {
      status if status.is_client_error() => Err(anyhow!(
        "Failed to exercise option contract {}: HTTP {}",
        symbol_or_id,
        status
      )),
      status if status.is_server_error() => Err(anyhow!(
        "Server error when exercising option contract {}: HTTP {}",
        symbol_or_id,
        status
      )),
      _ => Ok(()),
    }
  }
}
