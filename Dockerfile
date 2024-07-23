FROM rust:1.77 AS build
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release
RUN mv ./target/release/parkinsons_pulse_service ./app

FROM ubuntu:latest AS runtime
WORKDIR /app

COPY ./util/healthcheck.sh .
COPY --from=build /app/app /usr/local/bin/

HEALTHCHECK --start-period=10s --interval=5m --timeout=3s CMD ["bash", "./healthcheck.sh"]

ENTRYPOINT ["/usr/local/bin/app"]