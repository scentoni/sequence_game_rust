# Build image
FROM alpine as builder

RUN apk add --no-cache \
        cargo \
        ncurses-dev \
        g++ \
        rust \
        sudo \
        && \
    adduser rust -D -s /bin/bash && \
    addgroup sudo && \
    addgroup rust sudo && \
    echo "%sudo   ALL=(ALL:ALL) NOPASSWD:ALL" > /etc/sudoers.d/sudoers

# Our first FROM statement declares the build environment.
USER rust
RUN mkdir -p /home/rust/libs /home/rust/src
WORKDIR /home/rust/src

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `sequence_game`.
FROM alpine
RUN apk add --no-cache \
        ncurses \
        libgcc

COPY --from=builder \
    /home/rust/src/target/release/sequence_game \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/sequence_game"]
CMD []

# docker build -t scentoni/sequence_game .
# docker run --rm -it scentoni/sequence_game
# docker run --rm -it scentoni/sequence_game 7

