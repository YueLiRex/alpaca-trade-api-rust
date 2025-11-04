use crate::enums::*;
use chrono::{
  DateTime,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  pub id: Uuid,
  pub client_order_id: Uuid,
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
  pub asset_id: Uuid,
  pub symbol: String,
  pub asset_class: Class,
  pub national: Option<String>,
  pub qty: Option<String>,
  pub filled_qty: Option<String>,
  pub filled_avg_price: Option<String>,
  pub order_class: OrderClass,
  pub _type: Type,
  pub side: Side,
  pub time_in_force: TimeInForce,
  pub limit_price: Option<String>,
  pub stop_price: Option<String>,
  pub status: OrderStatus,
  pub extended_hours: bool,
  pub legs: Vec<Order>,
  pub trail_price: Option<String>,
  pub trail_percent: Option<String>,
  pub hwm: Option<String>,
  pub position_intent: PositionIntent,
}
