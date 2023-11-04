mod domain;
use axum::{routing::get, Router};

struct Config {
  postgres_host: String,
  postgres_port: String,
  postgres_user: String,
  postgres_password: String,
  postgres_database: String,
}

impl Config {
  pub fn new() -> Self {
    Self {
      postgres_host: std::env::var("POSTGRES_HOST").unwrap(),
      postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
      postgres_user: std::env::var("POSTGRES_USER").unwrap(),
      postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
      postgres_database: std::env::var("POSTGRES_DB").unwrap(),
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

// #[derive(sqlx::FromRow, Debug)]
// struct Users {
//   pub id: i64,
//   pub name: String,
//   pub created_at: chrono::NaiveDateTime,
//   pub updated_at: chrono::NaiveDateTime,
// }
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  let config = Config::new();
  // let mut user = User::new("v", UserKind::default());
  let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(20)
    .connect(&config.database_url())
    .await?;

  // let users: Vec<Users> = sqlx::query_as("SELECT * from users")
  //   .fetch_all(&pool)
  //   .await?;

  let app = Router::new().route("/", get(|| async { "Hello, x!" }));

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}
