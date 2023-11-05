mod domain;
mod infrastructure;

use std::sync::Arc;

use axum::{routing::get, Router};
use domain::{
  hotel::hotel_service::HotelService, room::room_service::RoomService,
  user::user_service::UserService,
};
use infrastructure::datastore::postgresql::{PgConfig, PgContext};

pub struct AppModule {
  user_service: UserService,
  hotel_service: HotelService,
  room_service: RoomService,
}

impl AppModule {
  pub async fn new() -> anyhow::Result<Self> {
    let context = Arc::new(PgContext::new(PgConfig::new()).await?);
    Ok(Self {
      user_service: UserService::new(context.clone()),
      hotel_service: HotelService::new(context.clone()),
      room_service: RoomService::new(context.clone()),
    })
  }
  pub fn user_service(&self) -> &UserService {
    &self.user_service
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let app_module = AppModule::new().await?;
  let x = app_module
    .user_service()
    .find_user(&"Liam Lee".try_into().unwrap())
    .await
    .unwrap();
  println!("{:?}", x);

  let app = Router::new().route("/", get(|| async { "Hello, x!" }));

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}
