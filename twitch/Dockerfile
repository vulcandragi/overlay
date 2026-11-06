FROM rust:1.96-trixie AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY ./twitch ./twitch
COPY ./proto ./proto
WORKDIR /src/twitch

RUN cargo install --path .

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/twitch /app/twitch

WORKDIR /app

CMD ["./twitch"]
