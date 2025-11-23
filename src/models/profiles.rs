use crate::models::utils::Money;
use chrono::NaiveDate;
use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioHistory {
  pub timestamp: Vec<u64>,
  pub equity: Vec<f64>,
  pub profit_loss: Vec<f64>,
  pub profit_loss_pct: Vec<f64>,
  pub base_value: Money,
  pub base_value_asof: NaiveDate,
  pub timeframe: String,
  pub cashflow: Option<CashFlow>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(deserialize = "FEE"))]
pub struct CashFlow {
  pub fee: Option<Vec<f64>>,
}
