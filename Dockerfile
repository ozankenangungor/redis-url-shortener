FROM rust:1.82.0-alpine AS builder

RUN apk update && apk add --no-cache musl-dev build-base

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

COPY ./src ./src
RUN rm -f target/release/deps/redirection_service* && \
    cargo build --release

FROM alpine:3.19

RUN apk add --no-cache libc6-compat

COPY --from=builder /app/target/release/redirection-service /usr/local/bin/

CMD ["redirection-service"]

