FROM rust:latest as rust-build

RUN mkdir build
COPY fileshare_webserver/src ./fileshare_webserver/src
COPY fileshare_webserver/Cargo.toml ./fileshare_webserver/Cargo.toml

WORKDIR /build

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine:latest

RUN mkdir app

COPY --from=rust-build /build/target/x86_64-unknown-linux-gnu/release/fileshare_webserver /app/fileshare_webserver

WORKDIR /app

CMD ["fileshare_webserver", "0.0.0.0:9000"]