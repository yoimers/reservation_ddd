use super::hotel_repository::THotelRepository;

struct HotelService {
  hotel_repository: Box<dyn THotelRepository>,
}
