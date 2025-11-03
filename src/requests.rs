use crate::enums::{
  Exchange,
  OptionContractsStatus,
  OptionStyle,
  OptionType,
  Side,
  TimeInForce,
  Type,
};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug)]
pub struct DefaultBoolean {
  pub value: bool,
}

impl Default for DefaultBoolean {
  fn default() -> Self {
    DefaultBoolean { value: false }
  }
}

impl Serialize for DefaultBoolean {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_bool(self.value)
  }
}

#[derive(Debug, Serialize)]
pub enum AssetsStatus {
  Active,
  Inactive,
  All,
}

impl Default for AssetsStatus {
  fn default() -> Self {
    Self::All
  }
}

#[derive(Debug, Serialize)]
pub struct AssetsQueryParameter {
  pub status: AssetsStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_class: Option<String>,
  pub exchange: Exchange,
  pub attributes: ComaSeparatedStrings,
}

#[derive(Debug, Serialize)]
pub struct OptionContractsQueryParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub underlying_symbols: Option<ComaSeparatedStrings>,
  pub show_deliverables: DefaultBoolean,
  pub status: OptionContractsStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date_gte: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiration_date_lte: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub root_symbol: Option<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub _type: Option<OptionType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<OptionStyle>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strike_price_gte: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strike_price_lte: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ppind: Option<bool>,
}

#[derive(Debug, Serialize)]
pub enum CorporateActionsDateType {
  Declaration_date,
  Ex_date,
  Record_date,
  Payable_date,
}

#[derive(Debug, Serialize)]
pub struct CorporateActionsQueryParameter {
  ca_types: ComaSeparatedStrings,
  since: NaiveDate,
  until: NaiveDate,
  symbols: Option<String>,
  cusip: Option<String>,
  date_type: Option<CorporateActionsDateType>,
}

#[derive(Debug, Serialize)]
pub struct OrderRequest {
  pub symbol: String,
  pub qty: u16,
  pub side: Side,
  #[serde(rename = "type")]
  pub _type: Type,
  pub time_in_force: TimeInForce,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trail_percent: Option<f64>,
  pub extended_hours: DefaultBoolean,
}

#[derive(Debug, Serialize)]
pub enum OrderStatus {
  OPEN,
  CLOSED,
  ALL,
}

#[derive(Debug, Serialize)]
pub enum OrdersDirection {
  ASC,
  DESC,
}

#[derive(Debug, Serialize)]
pub enum OrderAssetClass {
  US_EQUITY,
  US_OPTION,
  CRYPTO,
}

#[derive(Debug)]
pub struct ComaSeparatedStrings {
  pub values: Vec<&'static str>,
}

impl Serialize for ComaSeparatedStrings {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let s = self.values.join(",");
    serializer.serialize_str(&s)
  }
}

#[derive(Debug, Serialize)]
pub struct OrdersQueryParameter {
  pub status: OrderStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub until: Option<String>,
  pub direction: OrdersDirection,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nested: Option<bool>,
  pub symbols: ComaSeparatedStrings,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub side: Option<String>,
  pub asset_class: ComaSeparatedStrings,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before_order_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_order_id: Option<String>,
}
