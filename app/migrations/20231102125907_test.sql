CREATE TABLE IF NOT EXISTS users (
  id         BIGSERIAL NOT NULL PRIMARY KEY,
  name       VARCHAR(255),
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS hotels (
  id         BIGSERIAL NOT NULL PRIMARY KEY,
  name       VARCHAR(255),
  room_num   INTEGER   NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE INDEX hotels_name ON hotels (name);

CREATE TABLE IF NOT EXISTS rooms (
  id         BIGSERIAL NOT NULL PRIMARY KEY,
  hotel_id   BIGINT    NOT NULL,
  name       VARCHAR(255),
  room_num   INTEGER   NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  CONSTRAINT fk_rooms_hotel_id FOREIGN KEY (hotel_id) REFERENCES hotels (id)
);

CREATE TABLE IF NOT EXISTS room_reservations (
  id         BIGSERIAL NOT NULL PRIMARY KEY,
  hotel_id   BIGINT    NOT NULL,
  room_id    BIGINT    NOT NULL,
  user_id    BIGINT    NOT NULL,
  start_at   TIMESTAMP NOT NULL,
  end_at     TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  CONSTRAINT fk_room_reservations_hotel_id FOREIGN KEY (hotel_id) REFERENCES hotels (id),
  CONSTRAINT fk_room_reservations_room_id FOREIGN KEY (room_id) REFERENCES rooms (id),
  CONSTRAINT fk_room_reservations_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS posts (
  id         BIGSERIAL    NOT NULL PRIMARY KEY,
  user_id    BIGINT       NOT NULL,
  title      VARCHAR(255) NOT NULL,
  body       TEXT,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  CONSTRAINT fk_posts_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX posts_user_id ON posts (user_id);
