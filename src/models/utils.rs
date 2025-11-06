use serde::{
  Deserialize,
  Deserializer,
  Serialize,
  de::Visitor,
};

#[derive(Debug)]
pub struct Money(f64);

impl Serialize for Money {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_f64(self.0)
  }
}

impl<'de> Deserialize<'de> for Money {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_string(MoneyVisitor)
  }
}

impl Money {
  pub fn from_f64(value: f64) -> Self {
    Money(value)
  }

  pub fn value(&self) -> f64 {
    self.0
  }
}

struct MoneyVisitor;
impl<'de> Visitor<'de> for MoneyVisitor {
  type Value = Money;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a string representing a float value")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = v.parse().map_err(serde::de::Error::custom)?;
    Ok(Money(value))
  }
}

pub fn deserialize_str_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
  D: Deserializer<'de>,
{
  String::deserialize(deserializer)?
    .parse::<u8>()
    .map_err(serde::de::Error::custom)
}

pub fn deserialize_str_to_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
  D: Deserializer<'de>,
{
  String::deserialize(deserializer)?
    .parse::<u16>()
    .map_err(serde::de::Error::custom)
}
