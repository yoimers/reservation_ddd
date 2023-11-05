use async_trait::async_trait;

use super::room::Room;

#[async_trait]
pub trait TRoomRepository {
  async fn create_room(&self, room: Room) -> anyhow::Result<Room>;
}
