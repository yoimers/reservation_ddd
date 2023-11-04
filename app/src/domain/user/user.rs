use serde::{Deserialize, Serialize};

use super::{user_id::UserID, user_kind::UserKind, user_name::UserName};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: UserID,
  pub name: UserName,
  pub kind: UserKind,
}

impl User {
  pub fn new(name: UserName, kind: UserKind) -> Self {
    Self {
      id: UserID::new(),
      name,
      kind,
    }
  }

  pub fn change_user_name(&self, name: UserName) -> Self {
    Self {
      name,
      ..self.clone()
    }
  }
}
impl PartialEq for User {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl Eq for User {}

#[cfg(test)]
mod test {
  use crate::domain::user::{user::User, user_kind::UserKind};

  #[test]
  fn test_user() {
    let user = User::new("Name".try_into().unwrap(), UserKind::default());
    let new_user = user.change_user_name("NewName".try_into().unwrap());
    assert_eq!(user.name, "Name".try_into().unwrap());
    assert_eq!(new_user.name, "NewName".try_into().unwrap());
  }
}
