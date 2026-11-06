FROM rust:1.96-trixie AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY ./core ./core
COPY ./proto ./proto
WORKDIR /src/core

RUN cargo install --path .

FROM debian:trixie-slim

COPY --from=builder /usr/local/cargo/bin/core /app/core

EXPOSE 50051

WORKDIR /app

CMD ["./core"]
