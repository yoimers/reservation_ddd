use anyhow::anyhow;
use serde::{Deserialize, Serialize};

const HOTEL_NAME_MIN_LENGTH: usize = 2;
const HOTEL_NAME_MAX_LENGTH: usize = 50;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct HotelName(String);

impl HotelName {
  pub fn new(hotel_name: &str) -> anyhow::Result<Self> {
    if !Self::validate(hotel_name) {
      return Err(anyhow!(
        "Hotel Name length should be between {} and {} characters.",
        HOTEL_NAME_MIN_LENGTH,
        HOTEL_NAME_MAX_LENGTH
      ));
    }
    Ok(Self(hotel_name.to_string()))
  }

  fn validate(hotel_name: &str) -> bool {
    HOTEL_NAME_MIN_LENGTH <= hotel_name.len() && hotel_name.len() <= HOTEL_NAME_MAX_LENGTH
  }
}

impl TryFrom<&str> for HotelName {
  type Error = anyhow::Error;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    HotelName::new(value)
  }
}

#[cfg(test)]
mod test {
  use super::HotelName;

  #[test]
  fn test_hotel_name() {
    let hotel_name = HotelName::new("");
    assert!(hotel_name.is_err());
    if let Err(err) = hotel_name {
      let expected_error_message = "Hotel Name length should be between 2 and 50 characters.";
      assert_eq!(err.to_string(), expected_error_message);
    }

    let hotel_name = HotelName::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    assert!(hotel_name.is_err());
    if let Err(err) = hotel_name {
      let expected_error_message = "Hotel Name length should be between 2 and 50 characters.";
      assert_eq!(err.to_string(), expected_error_message);
    }

    let hotel_name = HotelName::new("xx");
    assert!(hotel_name.is_ok());
    if let Ok(hotel_name) = hotel_name {
      assert_eq!(hotel_name.0, "xx");
    }
  }
}
