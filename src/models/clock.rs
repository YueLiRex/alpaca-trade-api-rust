use chrono::{
  DateTime,
  Local,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MarketClock {
  pub timestamp: DateTime<Local>,
  pub is_open: bool,
  pub next_open: DateTime<Local>,
  pub next_close: DateTime<Local>,
}
