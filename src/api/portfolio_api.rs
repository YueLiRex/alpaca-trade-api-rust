use crate::{
  client::Client,
  models::PortfolioHistory,
};

pub trait PortfolioHistoryApi {
  async fn get_portfolio_history(&self) -> anyhow::Result<PortfolioHistory>;
}

impl PortfolioHistoryApi for Client {
  async fn get_portfolio_history(&self) -> anyhow::Result<PortfolioHistory> {
    let url = format!("{}/v2/account/portfolio/history", self.base_url);
    let resp = self
      .client
      .get(url)
      .send()
      .await?
      .json::<PortfolioHistory>()
      .await?;
    Ok(resp)
  }
}
