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
