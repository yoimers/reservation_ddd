use anyhow::Ok;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::user::{
  user::User, user_id::UserID, user_name::UserName, user_repository::TUserRepository,
};

pub struct PgConfig {
  postgres_host: String,
  postgres_port: String,
  postgres_user: String,
  postgres_password: String,
  postgres_database: String,
}

impl PgConfig {
  pub fn new() -> Self {
    Self {
      postgres_host: std::env::var("POSTGRES_HOST").unwrap_or("localhost".to_string()),
      postgres_port: std::env::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
      postgres_user: std::env::var("POSTGRES_USER").unwrap_or("postgres".to_string()),
      postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string()),
      postgres_database: std::env::var("POSTGRES_DB").unwrap_or("app".to_string()),
    }
  }
  pub fn database_url(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.postgres_user,
      self.postgres_password,
      self.postgres_host,
      self.postgres_port,
      self.postgres_database
    )
  }
}
pub struct PgContext {
  conn: Pool<Postgres>,
}

impl PgContext {
  pub async fn new(pg_config: PgConfig) -> anyhow::Result<Self> {
    println!("{}", pg_config.database_url());
    let conn = sqlx::postgres::PgPoolOptions::new()
      .max_connections(20)
      .connect(&pg_config.database_url())
      .await?;
    Ok(Self { conn })
  }
}

#[async_trait]
impl TUserRepository for PgContext {
  async fn find_user(&self, user_name: &UserName) -> anyhow::Result<Option<User>> {
    // let x = sqlx::query_as!(
    //   User,
    //   r#"
    //     SELECT id, name, kind FROM users WHERE name = $1
    //   "#,
    //   user_name.to_str()
    // )
    // .fetch_optional(&self.conn)
    // .await?;
    let x = sqlx::query_as(
      r#"
        SELECT id, name, kind FROM users WHERE name = $1
      "#,
    )
    .bind(user_name.to_str())
    .fetch_optional(&self.conn)
    .await?;
    Ok(x)
    // unimplemented!()
  }
  async fn create_user(&self, user: &User) -> anyhow::Result<User> {
    unimplemented!()
  }
  async fn change_user_name(&self, user_id: &UserID, user_name: &UserName) -> anyhow::Result<User> {
    unimplemented!()
  }
}
