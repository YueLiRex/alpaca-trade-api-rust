use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Status {
  #[default]
  Active,
  Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
  UsEquity,
  UsOption,
  Crypto,
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
  Market,
  Limit,
  Stop,
  StopLimit,
  TrailingStop,
}

impl Serialize for OrderType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let s = match *self {
      OrderType::Market => "market",
      OrderType::Limit => "limit",
      OrderType::Stop => "stop",
      OrderType::StopLimit => "stop_limit",
      OrderType::TrailingStop => "trailing_stop",
    };
    serializer.serialize_str(s)
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
  Buy,
  Sell,
}
