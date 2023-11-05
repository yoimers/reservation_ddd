use super::room_repository::{self, TRoomRepository};

pub struct RoomService {
  room_repository: Box<dyn TRoomRepository>,
}

impl RoomService {
  pub fn new<T: TRoomRepository + 'static>(room_repository: T) -> Self {
    Self {
      room_repository: Box::new(room_repository),
    }
  }
}
