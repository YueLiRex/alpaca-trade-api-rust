use crate::models::utils::Money;
use chrono::{
  DateTime,
  Utc,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CryptoWalletInfo {
  pub chain: String,
  pub address: String,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CryptoTransfer {
  pub id: Uuid,
  pub tx_hash: String,
  pub direction: CryptoDirection,
  pub status: CryptoStatus,
  pub amount: Money,
  pub usd_value: Money,
  pub network_fee: Money,
  pub fees: Money,
  pub chain: String,
  pub asset: String,
  pub from_address: String,
  pub to_address: String,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CryptoDirection {
  Incoming,
  Outgoing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CryptoStatus {
  Processing,
  Failed,
  Complete,
}

#[derive(Debug, Deserialize)]
pub struct WhiteListedAddress {
  pub id: String,
  pub chain: String,
  pub asset: String,
  pub address: String,
  pub status: AddressStatus,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AddressStatus {
  Approved,
  Pending,
}

#[derive(Debug, Deserialize)]
pub struct GasFee {
  pub fee: Money,
}
