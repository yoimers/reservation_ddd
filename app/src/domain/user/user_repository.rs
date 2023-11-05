use async_trait::async_trait;

use super::{user::User, user_id::UserID, user_name::UserName};

#[async_trait]
pub trait TUserRepository {
  async fn find_user(&self, user_name: &UserName) -> anyhow::Result<Option<User>>;
  async fn create_user(&self, user: &User) -> anyhow::Result<User>;
  async fn change_user_name(&self, user_id: &UserID, user_name: &UserName) -> anyhow::Result<User>;
}
