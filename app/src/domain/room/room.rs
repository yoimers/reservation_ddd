use serde::{Deserialize, Serialize};

use super::room_id::RoomID;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Room {
  id: RoomID,
}
