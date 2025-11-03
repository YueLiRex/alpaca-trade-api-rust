use crate::enums::*;
use chrono::{
  DateTime,
  NaiveDate,
  Utc,
};
use serde::{
  Deserialize,
  Deserializer,
  Serialize,
  de::Visitor,
};
use std::str::{
  self,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Money(f64);

impl Serialize for Money {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_f64(self.0)
  }
}

impl<'de> Deserialize<'de> for Money {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_string(MoneyVisitor)
  }
}

impl Money {
  pub fn from_f64(value: f64) -> Self {
    Money(value)
  }

  pub fn value(&self) -> f64 {
    self.0
  }
}

struct MoneyVisitor;
impl<'de> Visitor<'de> for MoneyVisitor {
  type Value = Money;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a string representing a float value")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = v.parse().map_err(serde::de::Error::custom)?;
    Ok(Money(value))
  }
}

fn deserialize_str_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
  D: Deserializer<'de>,
{
  String::deserialize(deserializer)?
    .parse::<u8>()
    .map_err(serde::de::Error::custom)
}

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
pub struct Asset {
  pub id: Uuid,
  pub class: Class,
  pub exchange: Exchange,
  pub symbol: String,
  pub name: String,
  pub status: AccountStatus,
  pub tradable: bool,
  pub marginable: bool,
  pub maintenance_margin_requirement: u16,
  pub margin_requirement_long: u16,
  pub margin_requirement_short: u16,
  pub shortable: bool,
  pub easy_to_borrow: bool,
  pub fractionable: bool,
  pub attributes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionContract {
  id: Uuid,
  symbol: String,
  name: String,
  tradeable: bool,
  expiration_date: NaiveDate,
  #[serde(skip_serializing_if = "Option::is_none")]
  root_symbol: Option<String>,
  underlying_symbol: String,
  underlying_asset_id: Uuid,
  _type: OptionType,
  style: OptionStyle,
  strike_price: f64,
  multiplier: u16,
  size: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  open_interest: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  open_interest_date: Option<NaiveDate>,
  #[serde(skip_serializing_if = "Option::is_none")]
  close_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  close_price_date: Option<NaiveDate>,
  deliverables: Vec<Deliverable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliverable {
  #[serde(rename = "type")]
  pub _type: DeliverableType,
  pub symbol: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_id: Option<Uuid>,
  pub amount: u16,
  pub allocation_percentage: u16,
  pub settlement_type: String,
  pub settlement_method: DeliverableSettlementMethod,
  pub delayed_settlement: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorporateAction {
  pub id: Uuid,
  pub corporate_actions_id: String,
  pub ca_type: String,
  pub ca_sub_type: String,
  pub initiating_symbol: String,
  pub initiating_original_cusip: String,
  pub target_symbol: String,
  pub target_original_cusip: String,
  pub declaration_date: NaiveDate,
  pub expiration_date: NaiveDate,
  pub record_date: NaiveDate,
  pub payable_date: NaiveDate,
  pub cash: String,
  pub old_rate: String,
  pub new_rate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  id: Uuid,
  client_order_id: Uuid,
  created_at: Option<DateTime<Utc>>,
  updated_at: Option<DateTime<Utc>>,
  submitted_at: Option<DateTime<Utc>>,
  filled_at: Option<DateTime<Utc>>,
  expired_at: Option<DateTime<Utc>>,
  canceled_at: Option<DateTime<Utc>>,
  failed_at: Option<DateTime<Utc>>,
  replaced_at: Option<DateTime<Utc>>,
  replaced_by: Option<Uuid>,
  replaces: Option<Uuid>,
  asset_id: Uuid,
  symbol: String,
  asset_class: Class,
  national: Option<String>,
  qty: Option<String>,
  filled_qty: Option<String>,
  filled_avg_price: Option<String>,
  order_class: OrderClass,
  _type: Type,
  side: Side,
  time_in_force: TimeInForce,
  limit_price: Option<String>,
  stop_price: Option<String>,
  status: OrderStatus,
  extended_hours: bool,
  legs: Vec<Order>,
  trail_price: Option<String>,
  trail_percent: Option<String>,
  hwm: Option<String>,
  position_intent: PositionIntent,
}
