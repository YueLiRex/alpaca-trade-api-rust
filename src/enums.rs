use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    ONBOARDING,
    SUBMISSION_FAILED,
    SUBMITTED,
    ACCOUNT_UPDATED,
    APPROVAL_PENDING,
    ACTIVE,
    REJECTED,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    USD,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Exchange {
    NYSE,
    NASDAQ,
    AMEX,
    ARCA,
    BATS,
    IEXG,
    OTC,
    PINK,
    CBOE,
    CRYPTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    US_EQUITY,
    CRYPTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderClass {
    SIMPLE,
    OCO,
    TRIGGER,
    BRACKET,
}

#[derive(Debug, Deserialize)]
pub enum Type {
    MARKET,
    LIMIT,
    STOP,
    STOP_LIMIT,
    TRAILING_STOP,
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            Type::MARKET => "market",
            Type::LIMIT => "limit",
            Type::STOP => "stop",
            Type::STOP_LIMIT => "stop_limit",
            Type::TRAILING_STOP => "trailing_stop",
        };
        serializer.serialize_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub enum Side {
    BUY,
    SELL,
}

impl Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            Side::BUY => "buy",
            Side::SELL => "sell",
        };
        serializer.serialize_str(s)
    }
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
pub enum OrderStatus {
    NEW,
    PARTIALLY_FILLED,
    FILLED,
    DONE_FOR_DAY,
    CANCELED,
    EXPIRED,
    REPLACED,
    PENDING_CANCEL,
    STOPPED,
    REJECTED,
    SUSPENDED,
    PENDING_NEW,
    CALCULATED,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionIntent {
    OPENING,
    CLOSING,
    UNKNOWN,
}
