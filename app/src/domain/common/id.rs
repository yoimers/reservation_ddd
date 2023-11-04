use std::{marker::PhantomData, ops::Deref};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
