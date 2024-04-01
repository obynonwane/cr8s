FROM rust:latest

WORKDIR /app/

COPY . .

# ORM - cli for running migrations: enable only postgres feature
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

CMD ["cargo", "watch", "--why", "-x", "build"]