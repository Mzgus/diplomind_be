# Build stage
FROM rust:alpine AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Final stage
FROM alpine:latest

# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/diplomind ./diplomind

EXPOSE 3000

CMD ["./diplomind"]