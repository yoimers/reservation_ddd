FROM rust:1.72

WORKDIR /usr/src/app
VOLUME .:/usr/src/app
COPY . .
RUN cargo install sqlx-cli
RUN cargo build --release

EXPOSE 8080
ENTRYPOINT ["cargo", "run", "-r"]