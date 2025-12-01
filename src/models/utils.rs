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
    serializer.serialize_str(self.0.to_string().as_str())
  }
}

impl<'de> Deserialize<'de> for Money {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_any(MoneyVisitor)
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
    formatter.write_str("a string representing a Float value")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = v.parse().map_err(serde::de::Error::custom)?;
    Ok(Money(value))
  }

  fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(Money(v as f64))
  }

  fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(Money(v as f64))
  }

  fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = f64::from(v);
    Ok(Money(value))
  }

  fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(Money(v))
  }
}

#[derive(Debug)]
pub struct NumberAsString(f64);

impl Serialize for NumberAsString {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.0.to_string().as_str())
  }
}

impl<'de> Deserialize<'de> for NumberAsString {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_string(NumberAsStringVisitor)
  }
}

impl NumberAsString {
  pub fn from_f64(v: f64) -> Self {
    NumberAsString(v)
  }

  pub fn value(&self) -> f64 {
    self.0
  }
}

struct NumberAsStringVisitor;

impl<'de> Visitor<'de> for NumberAsStringVisitor {
  type Value = NumberAsString;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a string representing a Integer value")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let value: f64 = v.parse().map_err(serde::de::Error::custom)?;
    Ok(NumberAsString(value))
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

// pub fn deserialize_navidate_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
// where
//   D: Deserializer<'de>,
// {
//   let str =String::deserialize(deserializer)?;
//   NaiveDate::from_str(&str).map_err(serde::de::Error::custom)
// }
