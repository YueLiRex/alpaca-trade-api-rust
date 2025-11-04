use crate::models::utils::Money;
use chrono::{
  DateTime,
  NaiveDate,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioHistory {
  pub timestamp: Vec<i64>,
  pub equity: Vec<f64>,
  pub profit_loss: Vec<f64>,
  pub profit_loss_pct: Vec<f64>,
  pub base_value: Money,
  pub base_value_asof: NaiveDate,
  pub timeframe: f64,
  pub cashflow: HashMap<String, f64>,
}
