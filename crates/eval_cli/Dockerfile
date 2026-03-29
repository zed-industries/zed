# Build eval-cli for Linux.
#
# Usage (from the zed repo root):
#   docker build --platform linux/amd64 -f crates/eval_cli/Dockerfile -t eval-cli-builder .
#   docker cp "$(docker create eval-cli-builder)":/eval-cli ./target/eval-cli
#
# Or use the helper script:
#   crates/eval_cli/script/build-linux

FROM rust:1.93 AS builder

WORKDIR /app

    # Pre-install the toolchain specified in rust-toolchain.toml so it is cached.
RUN rustup toolchain install 1.93 --profile minimal \
    --component rustfmt --component clippy --component rust-analyzer --component rust-src \
    --target wasm32-wasip2 --target wasm32-unknown-unknown --target x86_64-unknown-linux-musl --target x86_64-unknown-linux-gnu

# Install build tools.  cmake + build-essential are needed for vendored C
# libraries (libgit2-sys, zstd-sys, libsqlite3-sys).  No audio/GUI -dev
# packages required — eval-cli runs headless with those features disabled.
#
# cargo-zigbuild cross-compiles against a specific glibc version (2.31 =
# Debian Bullseye / Ubuntu Focal) so the resulting binary is portable to
# any Linux distro with glibc >= 2.31.
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    build-essential \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /opt/zig \
    && curl -fsSL https://ziglang.org/download/0.15.2/zig-x86_64-linux-0.15.2.tar.xz \
    | tar -xJ -C /opt/zig --strip-components=1 \
    && ln -s /opt/zig/zig /usr/local/bin/zig

RUN cargo install --locked cargo-zigbuild

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo zigbuild --release --package eval_cli \
        --target x86_64-unknown-linux-gnu.2.31 && \
    cp /app/target/x86_64-unknown-linux-gnu/release/eval-cli /eval-cli && \
    strip /eval-cli

FROM scratch
COPY --from=builder /eval-cli /eval-cli
