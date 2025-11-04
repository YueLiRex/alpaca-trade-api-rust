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
use std::{
  collections::HashMap,
  str::{
    self,
  },
};
use uuid::Uuid;

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
