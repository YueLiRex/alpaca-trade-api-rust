use crate::{
  client::Client,
  models::{
    Asset,
    CorporateAction,
  },
  requests::AssetsQueryParameter,
};

pub trait AssetsApi {
  async fn get_assets(&self, query_parameter: &AssetsQueryParameter) -> anyhow::Result<Vec<Asset>>;

  async fn get_asset_by_symbol_or_id(&self, symbol_or_id: &str) -> anyhow::Result<Asset>;
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
