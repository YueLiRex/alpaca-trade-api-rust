use std::str;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub account_number: String,
    pub status: AccountStatus,
    pub crypto_status: AccountStatus,
    pub currency: Currency,
    pub buying_power: u32,
    pub regt_buying_power: u32,
    pub daytrading_buying_power: u32,
    pub effective_buying_power: u32,
    pub options_buying_power: u32,
    pub options_approved_level: u8,
    pub options_trading_level: u8,
    pub non_marginable_buying_power: u32,
    pub bod_dtbp: u32,
    pub cash: u32,
    pub accrued_fees: u32,
    pub portfolio_value: u32,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub trade_suspended_by_user: bool,
    pub shorting_enabled: bool,
    pub multiplier: u8,
    pub equity: u32,
    pub last_equity: u32,
    pub long_market_value: u32,
    pub short_market_value: u32,
    pub position_market_value: u32,
    pub initial_margin: u32,
    pub maintenance_margin: u32,
    pub last_maintenance_margin: u32,
    pub sma: u32,
    pub daytrade_count: u16,
    pub balance_asof: NaiveDate,
    pub crypto_tier: u8,
    pub intraday_adjustments: u16,
    pub pending_reg_taf_fees: u32,
    pub pending_transfer_in: u32,
    pub pending_transfer_out: u32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub class: Class,
    pub exchange: Exchange,
    pub symbol: String,
    pub name: String,
    pub status: AccountStatus,
    pub tradable: bool,
    pub marginable: bool,
    pub maintenance_margin_requirement: u16,
    pub margin_requirement_long: u16,
    pub margin_requirement_short: u16,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
    pub attributes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionContract {
    id: Uuid,
    symbol: String,
    name: String,
    tradeable: bool,
    expiration_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_symbol: Option<String>,
    underlying_symbol: String,
    underlying_asset_id: Uuid,
    _type: OptionType,
    style: OptionStyle,
    strike_price: f64,
    multiplier: u16,
    size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_interest: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_interest_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_price_date: Option<NaiveDate>,
    deliverables: Vec<Deliverable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deliverable {
    #[serde(rename = "type")]
    pub _type: DeliverableType,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Uuid>,
    pub amount: u16,
    pub allocation_percentage: u16,
    pub settlement_type: String,
    pub settlement_method: DeliverableSettlementMethod,
    pub delayed_settlement: bool,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: Uuid,
    client_order_id: Uuid,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    submitted_at: Option<NaiveDateTime>,
    filled_at: Option<NaiveDateTime>,
    expired_at: Option<NaiveDateTime>,
    canceled_at: Option<NaiveDateTime>,
    failed_at: Option<NaiveDateTime>,
    replaced_at: Option<NaiveDateTime>,
    replaced_by: Option<Uuid>,
    replaces: Option<Uuid>,
    asset_id: Uuid,
    symbol: String,
    asset_class: Class,
    national: Option<String>,
    qty: Option<String>,
    filled_qty: Option<String>,
    filled_avg_price: Option<String>,
    order_class: OrderClass,
    _type: Type,
    side: Side,
    time_in_force: TimeInForce,
    limit_price: Option<String>,
    stop_price: Option<String>,
    status: OrderStatus,
    extended_hours: bool,
    legs: Vec<Order>,
    trail_price: Option<String>,
    trail_percent: Option<String>,
    hwm: Option<String>,
    position_intent: PositionIntent,
}
