use crate::{
  client::Client,
  models::{
    ErrorResponse,
    PortfolioHistory,
  },
};
use anyhow::bail;
use chrono::{
  DateTime,
  Utc,
};
use serde::Serialize;

pub trait PortfolioHistoryApi {
  fn get_portfolio_history(
    &self,
    query_params: &PortfolioHistoryQueryParameter,
  ) -> impl Future<Output = anyhow::Result<PortfolioHistory>>;
}

impl PortfolioHistoryApi for Client {
  async fn get_portfolio_history(
    &self,
    query_params: &PortfolioHistoryQueryParameter,
  ) -> anyhow::Result<PortfolioHistory> {
    let url = format!("{}/v2/account/portfolio/history", self.base_url);
    match self.client.get(url).query(&query_params).send().await {
      Ok(resp) => {
        if resp.status().is_success() {
          let portforlio_history = resp.json::<PortfolioHistory>().await?;
          Ok(portforlio_history)
        } else {
          let error_response = resp.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
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
#[serde(rename_all = "snake_case")]
pub enum IntradayReporting {
  MarketHours,
  ExtendedHours,
  Continuous,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PnlReset {
  NoReset,
  PerDay,
}

#[derive(Debug, Serialize)]
pub enum CashflowTypes {
  All,
  None,
  ComaSeparatedString(String),
}

#[derive(Debug, Serialize)]
pub struct PortfolioHistoryQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_history_period")]
  pub period: Option<HistoryPeriod>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_history_timeframe"
  )]
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
  #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_cashflow_types")]
  pub cashflow_types: Option<CashflowTypes>,
}

pub fn serialize_history_period<S>(period: &Option<HistoryPeriod>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match period {
    Some(p) => {
      let s = match p {
        HistoryPeriod::Day(n) => format!("{n}D"),
        HistoryPeriod::Week(n) => format!("{n}W"),
        HistoryPeriod::Month(n) => format!("{n}M"),
        HistoryPeriod::Year(n) => format!("{n}A"),
      };
      serializer.serialize_str(&s)
    }
    None => serializer.serialize_none(),
  }
}

fn serialize_history_timeframe<S>(timeframe: &Option<HistoryTimeFrame>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match timeframe {
    Some(t) => {
      let s = match t {
        HistoryTimeFrame::Day(n) => format!("{n}D"),
        HistoryTimeFrame::Hour(n) => format!("{n}H"),
        HistoryTimeFrame::Minute(n) => format!("{n}Min"),
      };
      serializer.serialize_str(&s)
    }
    None => serializer.serialize_none(),
  }
}

fn serialize_cashflow_types<S>(cashflow_types: &Option<CashflowTypes>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match cashflow_types {
    Some(types) => {
      let s = match types {
        CashflowTypes::None => "NONE",
        CashflowTypes::All => "ALL",
        CashflowTypes::ComaSeparatedString(str) => str,
      };
      serializer.serialize_str(&s)
    }
    None => serializer.serialize_none(),
  }
}

#[cfg(test)]
mod tests {
  use crate::api::portfolio_api::*;
  use chrono::{
    TimeZone,
    Utc,
  };

  #[test]
  fn test_portfolio_history_query_parameter() {
    let start = Utc.with_ymd_and_hms(2025, 1, 21, 5, 32, 12).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 2, 21, 5, 32, 12).unwrap();
    let query_params = PortfolioHistoryQueryParameter {
      period: Some(HistoryPeriod::Day(30)),
      timeframe: Some(HistoryTimeFrame::Minute(5)),
      intraday_reporting: Some(IntradayReporting::MarketHours),
      start: Some(start),
      pnl_reset: Some(PnlReset::PerDay),
      end: Some(end),
      extended_hours: Some("true".to_string()),
      cashflow_types: Some(CashflowTypes::All),
    };

    let serialized = serde_json::to_string(&query_params).unwrap();
    let expected = r#"{"period":"30D","timeframe":"5Min","intraday_reporting":"market_hours","start":"2025-01-21T05:32:12Z","pnl_reset":"per_day","end":"2025-02-21T05:32:12Z","extended_hours":"true","cashflow_types":"ALL"}"#;

    assert_eq!(serialized, expected)
  }
}
