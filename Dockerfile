FROM rust:1.66.1-slim-bullseye as build

WORKDIR /usr/src

RUN apt update && apt upgrade -y && apt install -y make default-libmysqlclient-dev pkg-config libssl-dev perl

RUN cargo new ban-controller-api

COPY Cargo.toml .env diesel.toml ./ban-controller-api/
COPY migrations ./ban-controller-api/migrations/

WORKDIR /usr/src/ban-controller-api

RUN rustup target add x86_64-unknown-linux-gnu

COPY src ./src/

RUN cargo install --target x86_64-unknown-linux-gnu diesel_cli --no-default-features --features "mysql"

RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM --platform=linux/amd64 debian:bullseye-slim as runtime

SHELL ["/bin/bash", "-c"]

RUN adduser --disabled-password --gecos "" --home "/none" --shell "/sbin/nologin" --no-create-home portinvartija

WORKDIR /api

COPY --from=build /usr/src/ban-controller-api/target/x86_64-unknown-linux-gnu/release/ban-controller-api ./
COPY --from=build /usr/local/cargo/bin/diesel ./
COPY --from=build /usr/src/ban-controller-api/.env ./
COPY --from=build /usr/src/ban-controller-api/migrations ./migrations
COPY entry.sh ./

RUN apt update && apt upgrade -y && apt install -y default-mysql-client curl

RUN chown -R portinvartija:portinvartija /api

USER portinvartija

CMD ["sh", "entry.sh"]