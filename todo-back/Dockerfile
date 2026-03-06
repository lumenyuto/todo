FROM --platform=linux/amd64 rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --no-default-features && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release --no-default-features

FROM --platform=linux/amd64 debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/todo-back /usr/local/bin/todo-back
COPY migrations ./migrations

CMD ["todo-back"]