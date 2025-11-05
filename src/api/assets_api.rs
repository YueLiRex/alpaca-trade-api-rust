use crate::{
  api::utils::{
    AssetsStatus,
    ComaSeparatedStrings,
  },
  client::Client,
  models::{
    Asset,
    enums::Exchange,
  },
};
use serde::Serialize;

pub trait AssetsApi {
  fn get_assets(
    &self,
    query_parameter: &AssetsQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<Asset>>>;

  fn get_asset_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> impl Future<Output = anyhow::Result<Asset>>;
}

impl AssetsApi for Client {
  async fn get_assets(&self, query_parameter: &AssetsQueryParameter) -> anyhow::Result<Vec<Asset>> {
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

  async fn get_asset_by_symbol_or_id(&self, symbol_or_id: &str) -> anyhow::Result<Asset> {
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
}

#[derive(Debug, Serialize)]
pub struct AssetsQueryParameter {
  pub status: AssetsStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_class: Option<String>,
  pub exchange: Exchange,
  pub attributes: ComaSeparatedStrings,
}
