use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{user_id::UserID, user_kind::UserKind, user_name::UserName};

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: UserID,
  pub name: UserName,
  pub kind: UserKind,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl User {
  pub fn new(
    id: UserID,
    name: UserName,
    kind: UserKind,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
  ) -> Self {
    Self {
      id,
      name,
      kind,
      created_at,
      updated_at,
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

// impl<'r> FromRow<'r, PgRow> for User {
//   fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//     Ok(Self {
//       id: UserID::from(row.try_get::<'_, Uuid, &str>("id")?),
//       name: UserName::new(row.try_get("name")?).unwrap(),
//       kind: UserKind::from(row.try_get::<'_, i32, &str>("kind")?),
//       created_at: row.try_get("created_at")?,
//       updated_at: row.try_get("updated_at")?,
//     })
//   }
// }

#[cfg(test)]
mod test {
  use crate::domain::user::{user::User, user_id::UserID, user_kind::UserKind};
  use chrono::Local;

  #[test]
  fn test_user() {
    let user = User::new(
      UserID::new(),
      "Name".try_into().unwrap(),
      UserKind::default(),
      Local::now().naive_local(),
      Local::now().naive_local(),
    );
    let new_user = user.change_user_name("NewName".try_into().unwrap());
    assert_eq!(user.name, "Name".try_into().unwrap());
    assert_eq!(new_user.name, "NewName".try_into().unwrap());
  }
}
