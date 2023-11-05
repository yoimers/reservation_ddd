use chrono::{NaiveDate, NaiveTime};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::domain::{hotel::hotel::Hotel, room::room::Room, user::user::User};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Reservation<'a> {
  pub user: &'a User,
  pub hotel: &'a Hotel,
  pub room: &'a Room,
  pub people: u32,
  pub day: NaiveDate,
}

impl<'a> Reservation<'a> {
  pub fn new(
    user: &'a User,
    hotel: &'a Hotel,
    room: &'a Room,
    people: u32,
    day: NaiveDate,
  ) -> Self {
    Self {
      user,
      hotel,
      room,
      people,
      day,
    }
  }
}

impl<'a> Serialize for Reservation<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Reservation", 5)?;
    state.serialize_field("user", self.user)?;
    state.serialize_field("hotel", self.hotel)?;
    state.serialize_field("room", self.room)?;
    state.serialize_field("people", &self.people)?;
    state.serialize_field("day", &self.day)?;
    state.end()
  }
}

#[cfg(test)]
mod test {
  use chrono::{NaiveDate, NaiveTime};
  use serde_json::json;

  use crate::domain::{
    hotel::{hotel::Hotel, hotel_id::HotelID},
    room::{room::Room, room_id::RoomID},
    user::{user::User, user_id::UserID, user_kind::UserKind},
  };

  use super::Reservation;

  #[test]
  fn test_serialize() {
    let user = User::new(UserID::new(), "name".try_into().unwrap(), UserKind::VIP);
    let hotel = Hotel::new(
      HotelID::new(),
      "nametest".try_into().unwrap(),
      16,
      NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
      NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
    );
    let room = Room::new(RoomID::new(), hotel.id.clone(), "name".into(), 2);
    let reservation = Reservation::new(
      &user,
      &hotel,
      &room,
      2,
      NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(),
    );
    let x = json!(reservation);
  }
}
