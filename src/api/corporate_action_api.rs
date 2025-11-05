use crate::{
  api::utils::ComaSeparatedStrings,
  client::Client,
  models::CorporateAction,
};
use chrono::NaiveDate;
use serde::Serialize;

pub trait CorporateActionApi {
  fn get_specific_corporate_actions(
    &self,
    id: &str,
  ) -> impl Future<Output = anyhow::Result<Vec<CorporateAction>>>;

  fn get_corporate_actions(
    &self,
    query_parameter: &CorporateActionsQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<CorporateAction>>>;
}

impl CorporateActionApi for Client {
  async fn get_specific_corporate_actions(&self, id: &str) -> anyhow::Result<Vec<CorporateAction>> {
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

  async fn get_corporate_actions(
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
}

#[derive(Debug, Serialize)]
pub enum CorporateActionsDateType {
  DeclarationDate,
  ExDate,
  RecordDate,
  PayableDate,
}

#[derive(Debug, Serialize)]
pub struct CorporateActionsQueryParameter {
  ca_types: ComaSeparatedStrings,
  since: NaiveDate,
  until: NaiveDate,
  symbols: Option<String>,
  cusip: Option<String>,
  date_type: Option<CorporateActionsDateType>,
}
