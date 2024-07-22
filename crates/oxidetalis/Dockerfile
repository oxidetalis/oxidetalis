FROM rust:1.70.0-slim-bullseye AS builder

WORKDIR /builder

COPY ./ ./

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /builder/target/release/oxidetalis .

CMD ["./oxidetalis"]
