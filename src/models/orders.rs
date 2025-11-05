use crate::models::enums::{
  AssetClass,
  Side,
  Type,
};
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
  pub asset_class: AssetClass,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
  New,
  PartiallyFilled,
  Filled,
  DoneForDay,
  Canceled,
  Expired,
  Replaced,
  PendingCancel,
  Stopped,
  Rejected,
  Suspended,
  PendingNew,
  Calculated,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderClass {
  Simple,
  Oco,
  Trigger,
  Bracket,
}

#[derive(Debug, Deserialize)]
pub enum TimeInForce {
  DAY,
  GTC,
  OPG,
  CLS,
  IOC,
  FOK,
}

impl Serialize for TimeInForce {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let s = match *self {
      TimeInForce::DAY => "day",
      TimeInForce::GTC => "gtc",
      TimeInForce::OPG => "opg",
      TimeInForce::CLS => "cls",
      TimeInForce::IOC => "ioc",
      TimeInForce::FOK => "fok",
    };
    serializer.serialize_str(s)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionIntent {
  Opening,
  Closing,
  Unknown,
}
