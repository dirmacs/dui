# syntax=docker/dockerfile:1.7
# dirmacs-ui (dui-leptos) — Leptos 0.8 component library, 36 accessible components.
#
# This is a library crate; the image is a development toolchain container.
# Shell in to build/test:
#   docker build -t dirmacs-ui:dev .
#   docker run -it --rm -v $(pwd):/src -w /src dirmacs-ui:dev bash
# Then: cargo build, cargo test, cargo clippy, trunk build, etc.

FROM rust:1.98-bookworm

# WASM target for Leptos.
RUN rustup target add wasm32-unknown-unknown

# trunk for WASM builds, cargo-chef for layer caching.
RUN cargo install trunk cargo-chef --locked

# Node.js for any frontend tooling (tailwind, etc.).
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src

# Pre-build dependencies for layer caching.
COPY Cargo.toml ./
RUN mkdir src && echo '' > src/lib.rs \
    && cargo build --release \
    && rm -rf src

# Real source goes here (bind-mounted or COPYed).
COPY . .

CMD ["bash"]
