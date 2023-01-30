FROM rust:latest as builder
WORKDIR /usr/src/tcp
COPY . .
RUN cargo build --release

FROM rust:latest as app
WORKDIR /usr/src/tcp
COPY --from=builder /usr/src/tcp/target/release/tcp_server /usr/local/bin/tcp_server
COPY --from=builder /usr/src/tcp/target/release/tcp_client /usr/local/bin/tcp_client
COPY --from=builder /usr/src/tcp/tcp_server/config.yaml /usr/src/tcp/
