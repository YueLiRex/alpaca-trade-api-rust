use crate::{
  api::utils::{
    AssetsStatus,
    ComaSeparatedStrings,
  },
  client::Client,
  models::{
    Asset,
    enums::{
      AssetClass,
      Exchange,
    },
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
  pub asset_class: Option<AssetClass>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exchange: Option<Exchange>,
  pub attributes: Option<ComaSeparatedStrings>,
}

#[cfg(test)]
mod tests {
  use crate::{
    api::{
      AssetsQueryParameter,
      AssetsStatus,
      ComaSeparatedStrings,
    },
    models::enums::{
      AssetClass,
      Exchange,
    },
  };

  #[test]
  fn asset_query_parameter_serialization_test() {
    use serde_json;

    let parameter = AssetsQueryParameter {
      status: AssetsStatus::Active,
      asset_class: Some(AssetClass::UsEquity),
      exchange: Some(Exchange::NASDAQ),
      attributes: Some(ComaSeparatedStrings {
        values: vec!["has_options", "ipo", "ptp_no_exception"],
      }),
    };

    let serialized = serde_json::to_string(&parameter).unwrap();
    let expected = r#"{"status":"Active","asset_class":"us_equity","exchange":"NASDAQ","attributes":"has_options,ipo,ptp_no_exception"}"#;
    assert_eq!(serialized, expected)
  }
}
