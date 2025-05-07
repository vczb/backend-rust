FROM rust:1.77 as builder


WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

# Install OpenSSL and other dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/backend-rust /app/backend-rust

EXPOSE 3000

CMD ["/app/backend-rust"]
