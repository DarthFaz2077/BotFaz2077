FROM rust:1.50.0-alpine3.13 as build

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /botfaz2077
COPY . /botfaz2077

RUN cargo build --release

FROM alpine:3.13.2 as binary

COPY --from=build /botfaz2077/target/release/botfaz2077 /usr/local/bin/

ENV RUST_LOG=info
CMD botfaz2077
