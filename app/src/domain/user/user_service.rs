use anyhow::{anyhow, Result};

use super::{
  user::{self, User},
  user_id::UserID,
  user_name::UserName,
  user_repository::TUserRepository,
};

pub struct UserService {
  user_repository: Box<dyn TUserRepository>,
}

impl UserService {
  pub fn new<T: TUserRepository + 'static>(user_repository: T) -> Self {
    Self {
      user_repository: Box::new(user_repository),
    }
  }

  pub async fn exists(&self, user_name: &UserName) -> Result<bool> {
    let get_user = self.user_repository.find_user(user_name).await?;
    Ok(get_user.is_some())
  }

  pub async fn create_user(&self, user: &User) -> Result<User> {
    let get_user = self.user_repository.create_user(user).await?;
    Ok(get_user)
  }

  pub async fn change_user_name(&self, user_id: &UserID, user_name: &UserName) -> Result<User> {
    let get_user = self.user_repository.find_user(user_name).await?;
    if get_user.is_some() {
      return Err(anyhow!("Same username exists"));
    }
    let user = self
      .user_repository
      .change_user_name(user_id, user_name)
      .await?;
    Ok(user)
  }
}
