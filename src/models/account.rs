use crate::models::{
  enums::Currency,
  utils::{
    Money,
    deserialize_str_to_u8,
  },
};
use chrono::{
  DateTime,
  NaiveDate,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: Uuid,
  pub account_number: String,
  pub status: AccountStatus,
  pub crypto_status: AccountStatus,
  pub currency: Currency,
  pub buying_power: Money,
  pub regt_buying_power: Money,
  pub daytrading_buying_power: Money,
  pub effective_buying_power: Money,
  pub options_buying_power: Money,
  pub options_approved_level: u8,
  pub options_trading_level: u8,
  pub non_marginable_buying_power: Money,
  pub bod_dtbp: Money,
  pub cash: Money,
  pub accrued_fees: Money,
  pub portfolio_value: Money,
  pub pattern_day_trader: bool,
  pub trading_blocked: bool,
  pub transfers_blocked: bool,
  pub account_blocked: bool,
  pub trade_suspended_by_user: bool,
  pub shorting_enabled: bool,
  #[serde(deserialize_with = "deserialize_str_to_u8")]
  pub multiplier: u8,
  pub equity: Money,
  pub last_equity: Money,
  pub long_market_value: Money,
  pub short_market_value: Money,
  pub position_market_value: Money,
  pub initial_margin: Money,
  pub maintenance_margin: Money,
  pub last_maintenance_margin: Money,
  pub sma: Money,
  pub daytrade_count: u16,
  pub balance_asof: NaiveDate,
  pub crypto_tier: u8,
  #[serde(deserialize_with = "deserialize_str_to_u8")]
  pub intraday_adjustments: u8,
  pub pending_reg_taf_fees: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pending_transfer_in: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pending_transfer_out: Option<Money>,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountStatus {
  Onboarding,
  SubmissionFailed,
  Submitted,
  AccountUpdated,
  ApprovalPending,
  Active,
  Rejected,
}
