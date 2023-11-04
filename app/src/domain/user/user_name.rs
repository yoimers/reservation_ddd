use anyhow::anyhow;
use serde::{Deserialize, Serialize};

const USER_NAME_MIN_LENGTH: usize = 3;
const USER_NAME_MAX_LENGTH: usize = 20;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UserName(String);
impl UserName {
  pub fn new(name: &str) -> anyhow::Result<Self> {
    if !Self::validate(&name) {
      return Err(anyhow!(
        "Name length should be between {} and {} characters.",
        USER_NAME_MIN_LENGTH,
        USER_NAME_MAX_LENGTH
      ));
    }
    Ok(Self(name.to_string()))
  }
  fn validate(name: &str) -> bool {
    let len = name.len();
    USER_NAME_MIN_LENGTH <= len && len <= USER_NAME_MAX_LENGTH
  }
}

impl TryFrom<&str> for UserName {
  type Error = anyhow::Error;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    UserName::new(value)
  }
}

#[cfg(test)]
mod test {
  use super::UserName;

  #[test]
  fn test_user_name() {
    let user_name = UserName::try_from("");
    assert!(user_name.is_err());
    if let Err(err) = user_name {
      let expected_error_message = "Name length should be between 3 and 20 characters.";
      assert_eq!(err.to_string(), expected_error_message);
    }

    let user_name = UserName::try_from("BestName");
    assert!(user_name.is_ok());
    if let Ok(user_name) = user_name {
      assert_eq!(user_name.0, "BestName");
    }
    let user_name =
      UserName::try_from("LongNameLongNameLongNameLongNameLongNameLongNameLongNameLongName");
    assert!(user_name.is_err());
    if let Err(err) = user_name {
      let expected_error_message = "Name length should be between 3 and 20 characters.";
      assert_eq!(err.to_string(), expected_error_message);
    }
  }
}
