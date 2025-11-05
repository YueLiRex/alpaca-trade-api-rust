use crate::{
  api::utils::{
    ComaSeparatedStrings,
    DefaultBoolean,
  },
  client::Client,
  models::{
    OptionContract,
    OptionStyle,
    OptionType,
  },
};
use serde::Serialize;

pub trait OptionApi {
  fn get_option_contracts(
    &self,
    query_parameter: &OptionContractsQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<OptionContract>>>;

  fn get_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> impl Future<Output = anyhow::Result<OptionContract>>;
}

impl OptionApi for Client {
  async fn get_option_contracts(
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

  async fn get_option_contract_by_symbol_or_id(
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
}

#[derive(Debug, Serialize)]
pub struct OptionContractsQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub underlying_symbols: Option<ComaSeparatedStrings>,
  pub show_deliverables: DefaultBoolean,
  pub status: OptionContract,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date_gte: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date_lte: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub root_symbol: Option<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub _type: Option<OptionType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<OptionStyle>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strike_price_gte: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strike_price_lte: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ppind: Option<bool>,
}
