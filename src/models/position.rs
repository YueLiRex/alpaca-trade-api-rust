use crate::models::{
  Order, OrderClass, OrderStatus, TimeInForce, enums::{
    AssetClass,
    Exchange, OrderType, Side,
  }, utils::{
    Money,
    NumberAsString,
  }
};
use chrono::{DateTime, Utc};
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
  pub qty: NumberAsString,
  pub qty_available: Option<NumberAsString>,
  pub side: PositionSide,
  pub market_value: Money,
  pub cost_basis: Money,
  pub unrealized_pl: Money,
  pub unrealized_plpc: NumberAsString,
  pub unrealized_intraday_pl: Money,
  pub unrealized_intraday_plpc: NumberAsString,
  pub current_price: Money,
  pub lastday_price: Money,
  pub change_today: NumberAsString,
  pub asset_marginable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedPosition {
  pub id: Uuid,
  pub client_order_id: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
  pub submitted_at: Option<DateTime<Utc>>,
  pub filled_at: Option<DateTime<Utc>>,
  pub expired_at: Option<DateTime<Utc>>,
  pub canceled_at: Option<DateTime<Utc>>,
  pub failed_at: Option<DateTime<Utc>>,
  pub replaced_at: Option<DateTime<Utc>>,
  pub replaced_by: Option<Uuid>,
  pub replaces: Option<Uuid>,
  pub asset_id: Option<Uuid>,
  pub symbol: Option<String>,
  pub asset_class: Option<AssetClass>,
  pub notional: String,
  pub qty: Option<NumberAsString>,
  pub filled_qty: Option<NumberAsString>,
  pub filled_avg_price: Option<Money>,
  pub order_class: Option<OrderClass>,
  #[serde(rename = "type")]
  pub _type: OrderType,
  pub side: Side,
  pub time_in_force: TimeInForce,
  pub limit_price: Money,
  pub stop_price: Money,
  pub status: OrderStatus,
  pub extended_hours: bool,
  pub trail_percent: NumberAsString,
  pub trail_price: Option<Money>,
  pub hwm: Option<Money>,
  pub position_intent: String,
  pub legs: Option<Vec<Order>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionSide {
  Long,
  Short,
}
