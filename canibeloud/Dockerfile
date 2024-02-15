ARG TARGET_ARCHITECTURE=aarch64-unknown-linux-gnu


# step 1

FROM sakisv/canibeloud-builder:main as build
ARG TARGET_ARCHITECTURE

RUN rustup target add ${TARGET_ARCHITECTURE}

COPY . /code
WORKDIR /code

RUN cargo build --release --target=${TARGET_ARCHITECTURE}


# step 2

FROM ubuntu:22.04 as release
ARG TARGET_ARCHITECTURE

COPY --from=build /code/target/${TARGET_ARCHITECTURE}/release/canibeloud .

CMD ["./canibeloud"]
