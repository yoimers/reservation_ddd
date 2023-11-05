use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserKind {
  Regular,
  VIP,
  Admin,
}

impl Default for UserKind {
  fn default() -> Self {
    Self::Regular
  }
}

impl From<i32> for UserKind {
  fn from(value: i32) -> Self {
    match value {
      1 => Self::Regular,
      2 => Self::VIP,
      3 => Self::Admin,
      _ => Self::Regular,
    }
  }
}
