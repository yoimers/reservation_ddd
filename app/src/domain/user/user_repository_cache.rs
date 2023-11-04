use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::{user::User, user_id::UserID, user_name::UserName, user_repository::TUserRepository};

struct UserRepositoryCache {
  cache: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl UserRepositoryCache {
  fn new() -> Self {
    Self {
      cache: Arc::new(Mutex::new(HashMap::new())),
    }
  }
}

#[async_trait]
impl TUserRepository for UserRepositoryCache {
  async fn create_user(&self, user: &User) -> anyhow::Result<User> {
    let exists = self.cache.lock().await.contains_key(&*user.id);
    if exists {
      return Err(anyhow!("Already Exists"));
    }
    Ok(user.clone())
  }
  async fn find_user(&self, user: &UserName) -> anyhow::Result<Option<User>> {
    unimplemented!()
  }
  async fn change_user_name(&self, user_id: &UserID, user_name: &UserName) -> anyhow::Result<User> {
    unimplemented!()
  }
}
