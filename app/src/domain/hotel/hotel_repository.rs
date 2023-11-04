use super::hotel::Hotel;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait THotelRepository {
  async fn create_hotel(&self, hotel: &Hotel) -> Result<Hotel>;
}
