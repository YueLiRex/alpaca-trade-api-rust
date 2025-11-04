use crate::{
  client::Client,
  models::OptionContract,
  requests::OptionContractsQueryParameter,
};

pub trait OptionApi {
  async fn get_option_contracts(
    &self,
    query_parameter: &OptionContractsQueryParameter,
  ) -> anyhow::Result<Vec<OptionContract>>;

  async fn get_option_contract_by_symbol_or_id(
    &self,
    symbol_or_id: &str,
  ) -> anyhow::Result<OptionContract>;
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
