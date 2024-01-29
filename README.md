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

#### Building

To build a docker image on your current branch:

    $ cd canibeloud
    $ branch=$(git rev-parse --abbrev-ref HEAD)
    $ docker buildx build --platform=linux/amd64 -t canibeloud:${branch} .

Notice the `--platform` argument and the `TARGET_ARCHITECTURE` in the Dockerfile.

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
