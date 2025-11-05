use crate::models::{
  enums::{
    AssetClass,
    Exchange,
  },
  utils::Money,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
  pub asset_id: Uuid,
  pub symbol: String,
  pub exchange: Exchange,
  pub asset_class: AssetClass,
  pub avg_entry_price: Money,
  pub qty: f64,
  pub qty_available: f64,
  pub side: PositionSide,
  pub market_value: Money,
  pub cost_basis: Money,
  pub unrealized_pl: Money,
  pub unrealized_plpc: f64,
  pub unrealized_intraday_pl: Money,
  pub unrealized_intraday_plpc: f64,
  pub current_price: Money,
  pub lastday_price: Money,
  pub change_today: f64,
  pub asset_marginable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionSide {
  Long,
  Short,
}
