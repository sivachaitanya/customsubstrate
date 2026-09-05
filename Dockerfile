ARG RUST_VERSION=1.85.1

FROM ubuntu:22.04 AS builder
ARG RUST_VERSION
ENV DEBIAN_FRONTEND=noninteractive \
    CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:${PATH}

WORKDIR /polkadot

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        git \
        build-essential \
        clang \
        pkg-config \
        libssl-dev \
        protobuf-compiler \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION} \
    && rustup toolchain install ${RUST_VERSION} --profile minimal --target wasm32-unknown-unknown \
    && rustup default ${RUST_VERSION} \
    && rm -rf /var/lib/apt/lists/*

COPY . /polkadot

RUN cargo fetch --locked && \
    cargo build --locked --release -p solochain-template-node

FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /polkadot
COPY --from=builder /polkadot/target/release/solochain-template-node /usr/local/bin/solochain-template-node

RUN useradd -m -u 1001 -U -s /bin/bash -d /polkadot polkadot && \
    mkdir -p /data /polkadot/.local/share && \
    chown -R polkadot:polkadot /data /polkadot && \
    ln -s /data /polkadot/.local/share/polkadot && \
    /usr/local/bin/solochain-template-node --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/solochain-template-node"]
