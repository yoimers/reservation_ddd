use std::sync::Arc;

use super::room_repository::TRoomRepository;

pub struct RoomService {
  room_repository: Arc<dyn TRoomRepository>,
}

impl RoomService {
  pub fn new<T: TRoomRepository + 'static>(room_repository: Arc<T>) -> Self {
    Self { room_repository }
  }
}
