FROM rust:1.96-trixie AS builder

ARG APP_CORE_URL=http://localhost:50051

ENV APP_CORE_URL=${APP_CORE_URL}

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

WORKDIR /src
COPY ./chat ./chat
COPY ./proto ./proto
WORKDIR /src/chat

RUN trunk build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    nginx \
    && rm -rf /var/lib/apt/lists/*

RUN rm -rf /var/www/html/*

COPY --from=builder /src/chat/dist /var/www/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
