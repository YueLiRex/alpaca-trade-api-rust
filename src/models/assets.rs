use crate::models::{
  enums::{
    AssetClass,
    Exchange,
    Status,
  },
  utils::NumberAsString,
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
  pub cusip: String,
  pub exchange: Exchange,
  pub symbol: String,
  pub name: String,
  pub status: Status,
  pub tradable: bool,
  pub marginable: bool,
  pub shortable: bool,
  pub margin_requirement_long: Option<NumberAsString>,
  pub margin_requirement_short: Option<NumberAsString>,
  pub easy_to_borrow: bool,
  pub fractionable: bool,
  pub attributes: Vec<String>,
}
