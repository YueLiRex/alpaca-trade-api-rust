use chrono::NaiveDate;
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CorporateAction {
  pub id: Uuid,
  pub corporate_actions_id: String,
  pub ca_type: String,
  pub ca_sub_type: String,
  pub initiating_symbol: String,
  pub initiating_original_cusip: String,
  pub target_symbol: String,
  pub target_original_cusip: String,
  pub declaration_date: NaiveDate,
  pub expiration_date: NaiveDate,
  pub record_date: NaiveDate,
  pub payable_date: NaiveDate,
  pub cash: String,
  pub old_rate: String,
  pub new_rate: String,
}
