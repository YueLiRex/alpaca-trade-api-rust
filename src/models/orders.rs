use crate::models::{
  enums::{
    AssetClass,
    OrderType,
    Side,
  },
  utils::{
    Money,
    NumberAsString,
  },
};
use chrono::{
  DateTime,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
  de::{
    Error,
    Visitor,
  },
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
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
  pub asset_id: Uuid,
  pub symbol: String,
  pub asset_class: AssetClass,
  pub national: Option<String>,
  pub qty: Option<NumberAsString>,
  pub filled_qty: Option<Money>,
  pub filled_avg_price: Option<Money>,
  pub order_class: OrderClass,
  #[serde(rename = "type")]
  pub _type: OrderType,
  pub side: Side,
  pub time_in_force: TimeInForce,
  pub limit_price: Option<String>,
  pub stop_price: Option<String>,
  pub status: OrderStatus,
  pub extended_hours: bool,
  pub legs: Option<Vec<Order>>,
  pub trail_price: Option<Money>,
  pub trail_percent: Option<Money>,
  pub hwm: Option<Money>,
  pub position_intent: PositionIntent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
  New,
  PartiallyFilled,
  Filled,
  DoneForDay,
  Canceled,
  Expired,
  Replaced,
  PendingCancel,
  PendingReplace,
  Accepted,
  PendingNew,
  AcceptedForBidding,
  Stopped,
  Rejected,
  Suspended,
  Calculated,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
  Simple,
  Oco,
  Otc,
  Trigger,
  Bracket,
  Mleg,
  Empty,
}

impl<'de> Deserialize<'de> for OrderClass {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_str(OrderClassVisitor)
  }
}

struct OrderClassVisitor;
impl<'de> Visitor<'de> for OrderClassVisitor {
  type Value = OrderClass;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("error from OrderClass deserilaizer")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    match v {
      "" => Ok(OrderClass::Empty),
      "simple" => Ok(OrderClass::Simple),
      "bracket" => Ok(OrderClass::Bracket),
      "oco" => Ok(OrderClass::Oco),
      "oto" => Ok(OrderClass::Otc),
      "mleg" => Ok(OrderClass::Mleg),
      _ => Err(Error::custom("Unexpect value received for OrderClass")),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
  DAY,
  GTC,
  OPG,
  CLS,
  IOC,
  FOK,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PositionIntent {
  BuyToOpen,
  BuyToClose,
  SellToOpen,
  SellToClose,
}
