# Build eval-cli for Linux.
#
# Usage (from the zed repo root):
#   docker build --platform linux/amd64 -f crates/eval_cli/Dockerfile -t eval-cli-builder .
#   docker cp "$(docker create eval-cli-builder)":/eval-cli ./target/eval-cli
#
# Or use the helper script:
#   crates/eval_cli/script/build-linux

FROM rust:1.93.1-bookworm AS builder

WORKDIR /app

# Install build dependencies (subset of script/linux needed for headless GPUI).
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    clang \
    g++ \
    libasound2-dev \
    libfontconfig-dev \
    libgit2-dev \
    libglib2.0-dev \
    libssl-dev \
    libwayland-dev \
    libx11-xcb-dev \
    libxkbcommon-x11-dev \
    libzstd-dev \
    libsqlite3-dev \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install wild linker for faster linking (built from source to match bookworm's glibc).
RUN cargo install --locked wild-linker --version 0.8.0 --root /usr/local

# Download WASI SDK (needed by some dependencies).
ARG TARGETARCH
RUN mkdir -p /app/target && \
    WASI_ARCH=$([ "$TARGETARCH" = "arm64" ] && echo "arm64" || echo "x86_64") && \
    curl -L "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-25/wasi-sdk-25.0-${WASI_ARCH}-linux.tar.gz" \
    | tar -xz -C /app/target && \
    mv /app/target/wasi-sdk-25.0-${WASI_ARCH}-linux /app/target/wasi-sdk

# Pre-install the toolchain specified in rust-toolchain.toml so it is cached.
RUN rustup toolchain install 1.93 --profile minimal \
    --component rustfmt --component clippy --component rust-analyzer --component rust-src \
    --target wasm32-wasip2 --target wasm32-unknown-unknown --target x86_64-unknown-linux-musl

COPY . .

ENV CC=clang CXX=clang++
ENV RUSTFLAGS="-C linker=clang -C link-arg=--ld-path=wild"

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --package eval_cli && \
    cp /app/target/release/eval-cli /eval-cli && \
    strip /eval-cli

FROM scratch
COPY --from=builder /eval-cli /eval-cli
