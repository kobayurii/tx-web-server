FROM rust:1.79 AS builder
WORKDIR /tmp/

COPY . ./
RUN cargo build --release

FROM ubuntu:24.04
RUN apt update && apt install -yy openssl ca-certificates libcurl4
COPY --from=builder /tmp/target/release/tx-web-server .
ENTRYPOINT ["./tx-web-server"]
