FROM rust:1.86.0-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY . .
RUN cargo build --release

## Stage 2: Create a minimal image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/bbnm-telebot .

CMD ["./bbnm-telebot"]
