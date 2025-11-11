use chrono::{
  NaiveDate,
  NaiveTime,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MarketCalendar {
  pub date: NaiveDate,
  pub open: NaiveTime,
  pub close: NaiveTime,
  pub settlement_date: NaiveDate,
}
