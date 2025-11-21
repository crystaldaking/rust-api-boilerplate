FROM rust:1.88-alpine AS builder

RUN apk add --no-cache build-base musl-dev perl perl-dev pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM alpine:3.20

RUN apk add --no-cache ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/rust-api-boilerplate /app/app

ENV RUST_LOG=info
CMD ["./app"]
