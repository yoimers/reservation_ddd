FROM rust:1.72

WORKDIR /usr/src/app
VOLUME ./:/usr/src/app
COPY . .
RUN apt update

ARG SQLX_OFFLINE=true

RUN cargo install sqlx-cli
RUN cargo build --release

EXPOSE 8080
ENTRYPOINT ["cargo", "run", "-r"]