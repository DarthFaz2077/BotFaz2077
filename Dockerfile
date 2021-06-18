FROM rust:1.53.0-alpine3.13 as planner
WORKDIR /botfaz2077
RUN apk add --no-cache musl-dev openssl-dev
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.53.0-alpine3.13 as cacher
WORKDIR /botfaz2077
RUN apk add --no-cache musl-dev openssl-dev
RUN cargo install cargo-chef
COPY --from=planner /botfaz2077/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53.0-alpine3.13 as builder
WORKDIR /botfaz2077
RUN apk add --no-cache musl-dev openssl-dev
COPY . .
COPY --from=cacher /botfaz2077/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin botfaz2077

FROM alpine:3.14.0 as runtime
RUN apk add --no-cache libgcc
COPY --from=builder /botfaz2077/target/release/botfaz2077 /usr/local/bin
CMD ["botfaz2077"]
