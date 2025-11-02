use serde::{Serialize};

use crate::{enums::{Side, TimeInForce, Type}, models::Order};

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
        S: serde::Serializer {
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
