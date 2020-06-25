FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-dev musl-tools -y
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/namaste-mysql-adapter
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

WORKDIR /usr/local/namaste-mysql-adapter

COPY --from=cargo-build /usr/src/namaste-mysql-adapter/target/x86_64-unknown-linux-musl/release/namaste-mysql-adapter .

CMD ["/usr/local/namaste-mysql-adapter/namaste-mysql-adapter"]
