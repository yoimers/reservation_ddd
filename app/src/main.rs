mod domain;
mod infrastructure;
use axum::{routing::get, Router};
use domain::user::user_repository::TUserRepository;
use infrastructure::datastore::postgresql::{PgConfig, PgContext};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let user_repository = PgContext::new(PgConfig::new()).await.unwrap();
  let x = user_repository
    .find_user(&"Liam Lee".try_into().unwrap())
    .await
    .unwrap();
  println!("{:?}", x);
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
