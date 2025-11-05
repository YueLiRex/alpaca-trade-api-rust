use serde::Serialize;

#[derive(Debug)]
pub struct DefaultBoolean {
  pub value: bool,
}

impl Default for DefaultBoolean {
  fn default() -> Self {
    DefaultBoolean { value: false }
  }
}

impl Serialize for DefaultBoolean {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_bool(self.value)
  }
}

#[derive(Debug, Serialize)]
pub enum AssetsStatus {
  Active,
  Inactive,
  All,
}

impl Default for AssetsStatus {
  fn default() -> Self {
    Self::All
  }
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
