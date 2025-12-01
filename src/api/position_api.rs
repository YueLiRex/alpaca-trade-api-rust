use crate::{
  client::Client,
  models::{
    ClosedPosition,
    ErrorResponse,
    Order,
    Position,
  },
};
use anyhow::bail;
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;

pub trait PositionApi {
  fn get_all_open_positions(&self) -> impl Future<Output = anyhow::Result<Vec<Position>>>;

  fn get_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> impl Future<Output = anyhow::Result<Position>>;

  fn close_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
    param: ClosePositionParam,
  ) -> impl Future<Output = anyhow::Result<ClosedPosition>>;

  fn exercise_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> impl Future<Output = anyhow::Result<()>>;

  fn clost_all_open_positions(
    &self,
    cancel_orders: bool,
  ) -> impl Future<Output = anyhow::Result<Vec<ClosePositionInfo>>>;
}

impl PositionApi for Client {
  async fn get_all_open_positions(&self) -> anyhow::Result<Vec<Position>> {
    let url = format!("{}/v2/positions", self.base_url);
    match self.client.get(url).send().await {
      Ok(resp) => {
        if resp.status().is_success() {
          let positions = resp.json::<Vec<Position>>().await?;
          Ok(positions)
        } else {
          let error = resp.json::<ErrorResponse>().await?;
          bail!(error)
        }
      }

      Err(error) => bail!(error),
    }
  }

  async fn clost_all_open_positions(
    &self,
    cancel_orders: bool,
  ) -> anyhow::Result<Vec<ClosePositionInfo>> {
    let url = format!("{}/v2/positions", self.base_url);
    let mut param = HashMap::new();
    param.insert("cancel_orders", &cancel_orders);
    match self.client.delete(url).query(&param).send().await {
      Ok(resp) => {
        if resp.status().is_success() {
          let result = resp.json::<Vec<ClosePositionInfo>>().await?;
          Ok(result)
        } else {
          let error = resp.json::<ErrorResponse>().await?;
          bail!(error)
        }
      }

      Err(error) => bail!(error),
    }
  }

  async fn get_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<Position> {
    let url = format!("{}/v2/positions/{}", self.base_url, symbol_or_id);
    match self.client.get(url).send().await {
      Ok(resp) => {
        if resp.status().is_success() {
          let position = resp.json::<Position>().await?;
          Ok(position)
        } else {
          let error = resp.json::<ErrorResponse>().await?;
          bail!(error)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn close_open_position_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
    param: ClosePositionParam,
  ) -> anyhow::Result<ClosedPosition> {
    let url = format!("{}/v2/positions/{}", self.base_url, symbol_or_id);
    let query_param = match param {
      ClosePositionParam::Qty(_n) => ("qty", serde_json::to_string(&param).unwrap()),
      ClosePositionParam::Percentage(_n) => ("percentage", serde_json::to_string(&param).unwrap()),
    };
    match self.client.delete(url).query(&[query_param]).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let closed_position = response.json::<ClosedPosition>().await?;
          Ok(closed_position)
        } else {
          let error = response.json::<ErrorResponse>().await?;
          bail!(error)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn exercise_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<()> {
    let url = format!("{}/v2/positions/{}/exercise", self.base_url, symbol_or_id);
    let response = self.client.post(url).send().await?;
    match response.status() {
      status if status.is_client_error() => bail!(
        "Failed to exercise option contract {}: HTTP {}",
        symbol_or_id,
        status
      ),
      status if status.is_server_error() => bail!(
        "Server error when exercising option contract {}: HTTP {}",
        symbol_or_id,
        status
      ),
      _ => Ok(()),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct ClosePositionInfo {
  pub symbol: String,
  pub status: String,
  pub body: Order,
}

#[derive(Debug)]
pub enum ClosePositionParam {
  Qty(f64),
  Percentage(f64),
}

impl Serialize for ClosePositionParam {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      ClosePositionParam::Qty(n) => serializer.serialize_f64(*n),
      ClosePositionParam::Percentage(n) => serializer.serialize_f64(*n),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::api::ClosePositionParam;

  #[test]
  fn test_close_postion_param_serialization() {
    let qty_ser = serde_json::to_string(&ClosePositionParam::Qty(5.3)).unwrap();
    assert_eq!(qty_ser, "5.3");

    let percentage_ser = serde_json::to_string(&ClosePositionParam::Percentage(54.2)).unwrap();
    assert_eq!(percentage_ser, "54.2")
  }
}
