use std::sync::Arc;

use super::hotel_repository::THotelRepository;

pub struct HotelService {
  hotel_repository: Arc<dyn THotelRepository>,
}

impl HotelService {
  pub fn new<T: THotelRepository + 'static>(hotel_repository: Arc<T>) -> Self {
    Self { hotel_repository }
  }
}
