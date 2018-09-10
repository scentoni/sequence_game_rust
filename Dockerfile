# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder AS builder
USER root
RUN apt-get update && \
    apt-get install -y \
        libncurses5-dev \
        libncursesw5-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
USER rust

# Add our source code.
ADD . ./

# Fix permissions on source code.

RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `sequence_game`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/sequence_game \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/sequence_game"]
CMD []

# docker build -t scentoni/sequence_game .
# docker run --rm -it scentoni/sequence_game
# docker run --rm -it scentoni/sequence_game 7

