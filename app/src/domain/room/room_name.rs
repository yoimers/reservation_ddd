use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct RoomName(String);

impl RoomName {
  pub fn new(hotel_name: &str) -> Self {
    Self(hotel_name.to_string())
  }
}

impl From<&str> for RoomName {
  fn from(value: &str) -> Self {
    Self(value.to_string())
  }
}
