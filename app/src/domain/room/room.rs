use serde::{Deserialize, Serialize};

use crate::domain::hotel::hotel_id::HotelID;

use super::{room_id::RoomID, room_name::RoomName};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Room {
  id: RoomID,
  hotel_id: HotelID,
  name: RoomName,
  max_people: u32,
}

impl Room {
  pub fn new(id: RoomID, hotel_id: HotelID, name: RoomName, max_people: u32) -> Self {
    Self {
      id,
      hotel_id,
      name,
      max_people,
    }
  }
}
