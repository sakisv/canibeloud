# Can I be loud?

A website that tries to answer this simple question based on your location.

Using it as an excuse and playground to learn rust, so the code you see here is probably
neither safe nor efficient.

Feel free to open a PR and suggest improvements.

## Why though?

Inspired by a neighbour of mine, seemingly unaware of the concepts of clocks and time.

## Build/Development

Just clone the repo and `cargo run` to run it.

During development, you can install `cargo-watch`[^1] and set it to
automatically recompile and run everytime there is a change with `cargo watch -x run`

### Docker

#### Building and pushing the `builder` image

To build the `builder` which contains precompiled versions of the dependencies:

    $ cd canibeloud
    $ docker buildx build --build-arg --platform=linux/arm64 -t sakisv/canibeloud-builder:main -f Dockerfile-builder  .
    $ docker push sakisv/canibeloud-builder:main

#### Building the app image

The app image relies on the `builder` image being available on dockerhub ☝️

To build a docker image on your current branch:

    $ cd canibeloud
    $ branch=$(git rev-parse --abbrev-ref HEAD)
    $ docker buildx build --platform=linux/amd64 -t canibeloud:${branch} .

By default this will build an image for `linux/amd64` and the rust target will be `aarch64-unknown-linux-gnu`.

These are controlled by the `--platform` argument in the command above and the `ARG TARGET_ARCHITECTURE` in the Dockerfile.

The first one tells docker which version of the images to fetch from dockerhub, whereas
the second tells rust which architecture the build is intended for.

#### Running

To run the image you just built:

    $ docker run --rm --platform linux/amd64 -p 8080:8080 -e CAN_I_BE_LOUD_BIND_ADDR=0.0.0.0 -it canibeloud:${branch}


The `--platform` argument here is necessary if you're on a different architecture (e.g. an
M1 Mac) but you want to run an x86_64 image.

Also the `-e CAN_I_BE_LOUD_BIND_ADDR=0.0.0.0` is required because by default the actix
server binds on `127.0.0.1`, which won't accept any requests when running inside a container.


[^1]: https://crates.io/crates/cargo-watch
