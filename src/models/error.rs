use serde::{
  Deserialize,
  Serialize,
};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("code: {code:?}, message: {message:?}")]
pub struct ErrorResponse {
  code: u32,
  message: String,
}

impl ErrorResponse {
  pub fn new(code: u32, message: String) -> Self {
    ErrorResponse { code, message }
  }
}
