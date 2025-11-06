use crate::models::{
  enums::{
    AssetClass,
    Exchange,
    Status,
  },
  utils::deserialize_str_to_u16,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
  pub id: Uuid,
  pub class: AssetClass,
  pub exchange: Exchange,
  pub symbol: String,
  pub name: String,
  pub status: Status,
  pub tradable: bool,
  pub marginable: bool,
  pub maintenance_margin_requirement: u16,
  #[serde(deserialize_with = "deserialize_str_to_u16")]
  pub margin_requirement_long: u16,
  #[serde(deserialize_with = "deserialize_str_to_u16")]
  pub margin_requirement_short: u16,
  pub shortable: bool,
  pub easy_to_borrow: bool,
  pub fractionable: bool,
  pub attributes: Vec<String>,
}
