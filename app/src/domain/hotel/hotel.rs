use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::room::room_name;

use super::{hotel_id::HotelID, hotel_name::HotelName};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Hotel {
  pub id: HotelID,
  pub name: HotelName,
  pub room_num: u32,
  pub opening_time: NaiveTime,
  pub closing_time: NaiveTime,
}

impl Hotel {
  pub fn new(
    id: HotelID,
    name: HotelName,
    room_num: u32,
    opening_time: NaiveTime,
    closing_time: NaiveTime,
  ) -> Self {
    Self {
      id,
      name,
      room_num,
      opening_time,
      closing_time,
    }
  }
}

#[cfg(test)]
mod test {
  use chrono::{Local, NaiveTime};

  use crate::domain::hotel::{hotel::Hotel, hotel_id::HotelID, hotel_name::HotelName};

  // #[test]
  // fn test_hotel() {
  //   let opening_time = Local::now().time();
  //   let closing_time = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
  //   let my_hotel = Hotel {
  //     id: HotelID::new(),
  //     name: HotelName::new("vvvvvv").unwrap(),
  //     room_num: 50,
  //     opening_time,
  //     closing_time,
  //   };
  //   let json_string: String = serde_json::to_string(&my_hotel).unwrap();
  //   assert_eq!(json_string, "");
  //   println!("{}", json_string);
  // }
}
