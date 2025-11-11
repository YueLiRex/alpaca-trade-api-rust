use crate::models::Asset;
use chrono::{
  DateTime,
  Utc,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct WatchList {
  pub id: Uuid,
  pub account_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub name: String,
  pub assets: Vec<Asset>,
}
