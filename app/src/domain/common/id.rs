use serde::{Deserialize, Serialize, Serializer};
use sqlx::types::Uuid;
use sqlx::Decode;
use sqlx::Postgres;
use sqlx::Type;
use std::io::Read;
use std::str::FromStr;
use std::{marker::PhantomData, ops::Deref};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct ID<T> {
  id: Uuid,
  _phantom: PhantomData<T>,
}

impl<T> ID<T> {
  pub fn new() -> Self {
    Self {
      id: Uuid::new_v4(),
      _phantom: PhantomData,
    }
  }
}

impl<T> Deref for ID<T> {
  type Target = Uuid;
  fn deref(&self) -> &Self::Target {
    &self.id
  }
}

impl<T> From<Uuid> for ID<T> {
  fn from(value: Uuid) -> Self {
    Self {
      id: value,
      _phantom: PhantomData,
    }
  }
}

impl<T> Serialize for ID<T> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.id.to_string())
  }
}

impl<'r, T> Decode<'r, Postgres> for ID<T> {
  fn decode(
    value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
  ) -> Result<Self, sqlx::error::BoxDynError> {
    let mut buf = [0u8; 16];
    value.as_bytes()?.read_exact(&mut buf)?;
    let id: Uuid = Uuid::from_bytes(buf);
    Ok(ID {
      id,
      _phantom: PhantomData,
    })
  }
}

impl<T> Type<Postgres> for ID<T> {
  fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
    sqlx::types::Uuid::type_info()
  }
}
