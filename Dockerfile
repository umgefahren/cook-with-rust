FROM rust:latest AS builder
WORKDIR /app
COPY . .
WORKDIR /app/cli
RUN cargo build --release

FROM debian:latest
COPY --from=builder /app/target/release/crab-soup-cli /usr/local/bin
CMD ["crab-soup-cli"]