use crate::{
  api::utils::{
    ComaSeparatedStrings,
    serialize_naivedate_to_str,
  },
  client::Client,
  models::{
    CorporateAction,
    ErrorResponse,
  },
};
use anyhow::bail;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

pub trait CorporateActionApi {
  fn get_specific_corporate_actions(
    &self,
    uuid: &Uuid,
  ) -> impl Future<Output = anyhow::Result<CorporateAction>>;

  fn get_corporate_actions(
    &self,
    query_parameter: &CorporateActionsQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<CorporateAction>>>;
}

impl CorporateActionApi for Client {
  async fn get_specific_corporate_actions(&self, uuid: &Uuid) -> anyhow::Result<CorporateAction> {
    let id = uuid.to_string();
    let url = format!(
      "{}/v2/corporate_actions/announcements/{}",
      self.base_url, id
    );
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let asset = response.json::<CorporateAction>().await?;
          Ok(asset)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => {
        bail!(error)
      }
    }
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
#[serde(rename_all = "snake_case")]
pub enum CorporateActionsDateType {
  DeclarationDate,
  ExDate,
  RecordDate,
  PayableDate,
}

#[derive(Debug, Serialize)]
pub struct CorporateActionsQueryParameter {
  pub ca_types: ComaSeparatedStrings,
  #[serde(serialize_with = "serialize_naivedate_to_str")]
  pub since: NaiveDate,
  #[serde(serialize_with = "serialize_naivedate_to_str")]
  pub until: NaiveDate,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub symbols: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cusip: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub date_type: Option<CorporateActionsDateType>,
}

#[cfg(test)]
mod tests {
  use crate::api::{
    ComaSeparatedStrings,
    CorporateActionsDateType,
    CorporateActionsQueryParameter,
  };
  use chrono::NaiveDate;

  #[test]
  fn test_corporate_query_parameter_serialization() {
    let parameter = &CorporateActionsQueryParameter {
      ca_types: ComaSeparatedStrings {
        values: vec!["dividend", "merger"],
      },
      since: NaiveDate::from_ymd_opt(2025, 1, 30).unwrap(),
      until: NaiveDate::from_ymd_opt(2025, 3, 30).unwrap(),
      symbols: None,
      cusip: None,
      date_type: Some(CorporateActionsDateType::DeclarationDate),
    };
    let json = serde_json::to_string(parameter).unwrap();
    let expected = r#"{"ca_types":"dividend,merger","since":"2025-01-30","until":"2025-03-30","date_type":"declaration_date"}"#;
    assert_eq!(json, expected)
  }
}
