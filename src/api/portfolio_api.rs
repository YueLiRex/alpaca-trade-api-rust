use crate::{
  client::Client,
  models::PortfolioHistory,
};
use chrono::{
  DateTime,
  Utc,
};
use serde::Serialize;

pub trait PortfolioHistoryApi {
  fn get_portfolio_history(&self) -> impl Future<Output = anyhow::Result<PortfolioHistory>>;
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

#[derive(Debug, Serialize)]
pub enum HistoryPeriod {
  Day(u32),
  Week(u32),
  Month(u32),
  Year(u32),
}

#[derive(Debug, Serialize)]
pub enum HistoryTimeFrame {
  Minute(u32),
  Hour(u32),
  Day(u32),
}

#[derive(Debug, Serialize)]
pub enum IntradayReporting {
  MarketHours,
  ExtendedHours,
  Continuous,
}

#[derive(Debug, Serialize)]
pub enum PnlReset {
  NoReset,
  PerDay,
}

#[derive(Debug, Serialize)]
pub enum CashflowTypes {
  All,
  None,
}

#[derive(Debug, Serialize)]
pub struct PortfolioHistoryQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub period: Option<HistoryPeriod>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timeframe: Option<HistoryTimeFrame>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub intraday_reporting: Option<IntradayReporting>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pnl_reset: Option<PnlReset>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extended_hours: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cashflow_types: Option<CashflowTypes>,
}
