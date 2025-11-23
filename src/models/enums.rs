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
pub enum Type {
  Market,
  Limit,
  Stop,
  StopLimit,
  TrailingStop,
}

impl Serialize for Type {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let s = match *self {
      Type::Market => "market",
      Type::Limit => "limit",
      Type::Stop => "stop",
      Type::StopLimit => "stop_limit",
      Type::TrailingStop => "trailing_stop",
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
