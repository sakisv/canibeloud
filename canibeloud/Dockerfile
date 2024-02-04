ARG TARGET_ARCHITECTURE=x86_64-unknown-linux-gnu

FROM rust:1.75.0-slim-buster as build

ARG TARGET_ARCHITECTURE

RUN rustup target add ${TARGET_ARCHITECTURE}

COPY . /src
WORKDIR /src

RUN cargo build --release --target=${TARGET_ARCHITECTURE}

FROM ubuntu:22.04 as release
ARG TARGET_ARCHITECTURE

COPY --from=build /src/target/${TARGET_ARCHITECTURE}/release/canibeloud .

CMD ["./canibeloud"]
