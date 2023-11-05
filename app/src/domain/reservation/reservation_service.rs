use anyhow::{anyhow, Ok};

use crate::domain::room::room_repository::TRoomRepository;

use super::{reservation::Reservation, reservation_repository::TReservationRepository};

pub struct ReservationService {
  reservation_repository: Box<dyn TReservationRepository>,
}

impl ReservationService {
  pub fn new<T: TReservationRepository + 'static>(reservation_repository: T) -> Self {
    Self {
      reservation_repository: Box::new(reservation_repository),
    }
  }

  pub async fn reservation(&self, reservation: &Reservation<'_>) -> anyhow::Result<()> {
    let find = self
      .reservation_repository
      .find_reservation(reservation.room, reservation.day)
      .await?;
    if find.is_some() {
      return Err(anyhow!("Already reserved."));
    }
    let _ = self
      .reservation_repository
      .room_reservation(&reservation)
      .await?;
    Ok(())
  }
}
