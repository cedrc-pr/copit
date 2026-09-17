FROM rust:alpine AS builder

WORKDIR /app/
COPY src/ src/
COPY postgres/ postgres/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release

FROM alpine

RUN apk add --no-cache curl
WORKDIR /app/
COPY --from=builder /app/target/release/be ./

CMD ["./be"]
