use crate::{
  client::Client,
  models::CorporateAction,
  requests::CorporateActionsQueryParameter,
};

pub trait CorporateActionApi {
  async fn get_specific_corporate_actions(&self, id: &str) -> anyhow::Result<Vec<CorporateAction>>;

  async fn get_corporate_actions(
    &self,
    query_parameter: &CorporateActionsQueryParameter,
  ) -> anyhow::Result<Vec<CorporateAction>>;
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
