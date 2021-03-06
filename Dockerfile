FROM rustlang/rust:nightly as builder

WORKDIR /smor_market

COPY .env .env
COPY Rocket.toml Rocket.toml
COPY diesel.toml diesel.toml
COPY secret.key secret.key
COPY . .


RUN cargo build --release


FROM debian:buster-slim

RUN mkdir smor_market

WORKDIR /smor_market

# install libpq
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /smor_market/target/release/market_service ./

COPY --from=builder /smor_market/Rocket.toml .
COPY --from=builder /smor_market/diesel.toml .
COPY --from=builder /smor_market/.env .


EXPOSE 8001

ENTRYPOINT [ "/smor_market/market_service" ]
