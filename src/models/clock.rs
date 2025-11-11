use chrono::{
  DateTime,
  Utc,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MarketClock {
  pub timestamp: DateTime<Utc>,
  pub is_open: bool,
  pub next_open: DateTime<Utc>,
  pub next_close: DateTime<Utc>,
}
