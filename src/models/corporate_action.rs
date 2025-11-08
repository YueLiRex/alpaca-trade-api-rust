use chrono::NaiveDate;
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CorporateAction {
  pub id: Uuid,
  pub corporate_action_id: String,
  pub ca_type: String,
  pub ca_sub_type: String,
  pub initiating_symbol: String,
  pub initiating_original_cusip: String,
  pub target_symbol: String,
  pub target_original_cusip: String,
  pub declaration_date: Option<NaiveDate>,
  pub expiration_date: Option<NaiveDate>,
  pub effective_date: Option<NaiveDate>,
  pub record_date: Option<NaiveDate>,
  pub payable_date: Option<NaiveDate>,
  pub cash: String,
  pub old_rate: String,
  pub new_rate: String,
}
