FROM rust:1.99-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends libprotobuf-dev protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY cmd ./cmd
COPY internal ./internal
COPY proto ./proto
COPY src ./src
COPY assets ./assets

RUN cargo build --locked --release --bin server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system app \
    && useradd --system --gid app --home-dir /app app

WORKDIR /app

COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/assets/migrations ./migrations
COPY config.example.yaml ./config.yaml

USER app

EXPOSE 50051

CMD ["./server"]