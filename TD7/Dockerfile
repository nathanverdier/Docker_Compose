# syntax=docker/dockerfile:1-labs
FROM docker.io/clux/muslrust:stable AS builder
WORKDIR /src
COPY --exclude=target . /src/
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM docker.io/fedora:latest
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/webapp /bin/webapp
RUN mkdir -p /var/log/webapp

CMD ["/bin/webapp"]
