use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct DefaultBoolean {
  pub value: bool,
}

impl Serialize for DefaultBoolean {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_bool(self.value)
  }
}

#[derive(Debug, Serialize, Default)]
pub enum AssetsStatus {
  Active,
  Inactive,
  #[default]
  All,
}

#[derive(Debug)]
pub struct ComaSeparatedStrings {
  pub values: Vec<&'static str>,
}

impl Serialize for ComaSeparatedStrings {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let s = self.values.join(",");
    serializer.serialize_str(&s)
  }
}

const FORMAT: &str = "%Y-%m-%d";

pub fn serialize_naivedate_to_str<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  let s = format!("{}", date.format(FORMAT));
  serializer.serialize_str(&s)
}
