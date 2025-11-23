use crate::models::utils::{
  IntAsString,
  Money,
};
use chrono::NaiveDate;
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliverable {
  #[serde(rename = "type")]
  pub _type: DeliverableType,
  pub symbol: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_id: Option<Uuid>,
  pub amount: IntAsString,
  pub allocation_percentage: IntAsString,
  pub settlement_type: String,
  pub settlement_method: DeliverableSettlementMethod,
  pub delayed_settlement: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionContract {
  pub id: Uuid,
  pub symbol: String,
  pub name: String,
  pub tradable: bool,
  pub expiration_date: NaiveDate,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub root_symbol: Option<String>,
  pub underlying_symbol: String,
  pub underlying_asset_id: Uuid,
  #[serde(rename = "type")]
  pub _type: OptionType,
  pub style: OptionStyle,
  pub strike_price: Money,
  pub multiplier: IntAsString,
  pub size: IntAsString,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub open_interest: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub open_interest_date: Option<NaiveDate>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub close_price: Option<Money>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub close_price_date: Option<NaiveDate>,
  pub deliverables: Option<Vec<Deliverable>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum OptionStatus {
  #[default]
  Active,
  Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
  Call,
  Put,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionStyle {
  American,
  European,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeliverableType {
  Cash,
  Equity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeliverableSettlementMethod {
  BTOB,
  CADF,
  CAFX,
  CCC,
}
