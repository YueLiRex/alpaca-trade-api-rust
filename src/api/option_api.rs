use crate::{
  api::utils::{
    ComaSeparatedStrings,
    DefaultBoolean,
  },
  client::Client,
  models::{
    ErrorResponse,
    OptionContract,
    OptionStatus,
    OptionStyle,
    OptionType,
  },
};
use anyhow::bail;
use serde::{
  Deserialize,
  Serialize,
};

pub trait OptionApi {
  fn get_option_contracts(
    &self,
    query_parameter: &OptionContractsQueryParameter,
  ) -> impl Future<Output = anyhow::Result<OptionsResponse>>;

  fn get_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> impl Future<Output = anyhow::Result<OptionContract>>;
}

impl OptionApi for Client {
  async fn get_option_contracts(
    &self,
    query_parameter: &OptionContractsQueryParameter,
  ) -> anyhow::Result<OptionsResponse> {
    let url = format!("{}/v2/options/contracts", self.base_url);

    match self.client.get(url).query(query_parameter).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<OptionsResponse>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<OptionContract> {
    let url = format!("{}/v2/options/contracts/{}", self.base_url, symbol_or_id);

    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let result = response.json::<OptionContract>().await?;
          Ok(result)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct OptionContractsQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub underlying_symbols: Option<ComaSeparatedStrings>,
  pub show_deliverables: DefaultBoolean,
  pub status: OptionStatus,
  #[serde(skip_serializing_if = "Option::is_none")] // todo NaiveDate to str yyyy-MM-dd
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

#[derive(Debug, Deserialize)]
pub struct OptionsResponse {
  pub option_contracts: Vec<OptionContract>,
  pub next_page_token: Option<String>,
}

#[cfg(test)]
mod tests {
  use crate::api::{
    ComaSeparatedStrings,
    DefaultBoolean,
    OptionContractsQueryParameter,
  };

  #[test]
  fn test_option_contracts_query_parameter_serialize() {
    let parameter = OptionContractsQueryParameter {
      underlying_symbols: Some(ComaSeparatedStrings {
        values: vec!["appl", "tsla"],
      }),
      status: crate::models::OptionStatus::Active,
      show_deliverables: DefaultBoolean { value: true },
      expiration_date: None,
      expiration_date_gte: Some("2025-01-23".to_string()),
      expiration_date_lte: None,
      root_symbol: Some("AAPL".to_string()),
      _type: Some(crate::models::OptionType::Call),
      style: Some(crate::models::OptionStyle::American),
      strike_price_gte: Some(23.32),
      strike_price_lte: None,
      page_token: Some("test-token".to_string()),
      limit: Some(100),
      ppind: None,
    };

    let json = serde_json::to_string(&parameter).unwrap();

    assert_eq!(
      json,
      r#"{"underlying_symbols":"appl,tsla","show_deliverables":true,"status":"active","expiration_date_gte":"2025-01-23","root_symbol":"AAPL","type":"call","style":"american","strike_price_gte":23.32,"page_token":"test-token","limit":100}"#
    )
  }
}
