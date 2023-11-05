-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id         UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  name       VARCHAR(255) NOT NULL,
  kind       INTEGER   NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS hotels (
  id         UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  name       VARCHAR(255) NOT NULL,
  room_num   INTEGER   NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

DROP INDEX IF EXISTS hotels_name;
CREATE INDEX hotels_name ON hotels (name);

CREATE TABLE IF NOT EXISTS rooms (
  id         UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  hotel_id   UUID    NOT NULL,
  name       VARCHAR(255) NOT NULL,
  room_num   INTEGER   NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  CONSTRAINT fk_rooms_hotel_id FOREIGN KEY (hotel_id) REFERENCES hotels (id)
);

CREATE TABLE IF NOT EXISTS room_reservations (
  id         UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  hotel_id   UUID      NOT NULL,
  room_id    UUID      NOT NULL,
  user_id    UUID      NOT NULL,
  start_at   TIMESTAMP NOT NULL,
  end_at     TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  CONSTRAINT fk_room_reservations_hotel_id FOREIGN KEY (hotel_id) REFERENCES hotels (id),
  CONSTRAINT fk_room_reservations_room_id FOREIGN KEY (room_id) REFERENCES rooms (id),
  CONSTRAINT fk_room_reservations_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);

