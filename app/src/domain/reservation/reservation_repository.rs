use async_trait::async_trait;
use chrono::NaiveDate;

use crate::domain::room::room::Room;

use super::reservation::Reservation;

#[async_trait]
pub trait TReservationRepository {
  async fn room_reservation(&self, reservation: &Reservation) -> anyhow::Result<()>;
  async fn find_reservation(&self, room: &Room, day: NaiveDate) -> anyhow::Result<Option<Room>>;
}
