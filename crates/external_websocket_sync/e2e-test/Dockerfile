# E2E test environment for Zed WebSocket sync
# Provides: Zed (built from source with cache), Go test server, xvfb, software Vulkan
#
# The Go test server (helix-ws-test-server) imports the wsprotocol package from
# the Helix repo, so the same message parsing, routing, and accumulation code
# runs in both tests and production. No more Python mock divergence.
#
# Usage:
#   # 1. Build Go test server (from helix repo)
#   cd /path/to/helix
#   CGO_ENABLED=0 go build -o /tmp/helix-ws-test-server \
#     ./crates/external_websocket_sync/e2e-test/helix-ws-test-server  # if in zed repo
#   # OR: use the go module in the e2e-test dir:
#   cd /path/to/zed/crates/external_websocket_sync/e2e-test/helix-ws-test-server
#   CGO_ENABLED=0 go build -o /tmp/helix-ws-test-server .
#
#   # 2. Build E2E Docker image (from zed repo root)
#   cd /path/to/zed
#   docker build -t zed-ws-e2e \
#     --build-arg HELIX_WS_TEST_SERVER=/tmp/helix-ws-test-server \
#     -f crates/external_websocket_sync/e2e-test/Dockerfile .
#
#   # 3. Run
#   docker run --rm -e ANTHROPIC_API_KEY=$ANTHROPIC_API_KEY zed-ws-e2e
#
# BuildKit cache mounts persist cargo registry, git deps, rustup, and build
# artifacts across builds (same pattern as Helix's Dockerfile.zed-build).

FROM ubuntu:25.10 AS builder

ENV DEBIAN_FRONTEND=noninteractive

# Zed build dependencies (matches Helix Dockerfile.zed-build + Zed's script/linux)
RUN apt-get update && apt-get install -y \
    build-essential curl git pkg-config clang cmake mold \
    libssl-dev libzstd-dev libsqlite3-dev \
    libvulkan-dev \
    libwayland-dev libxkbcommon-dev libxkbcommon-x11-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libxcb-render0-dev \
    libxcb1-dev libx11-dev libx11-xcb-dev libxrandr-dev \
    libfreetype-dev libfontconfig-dev \
    libasound2-dev \
    libatk1.0-dev libgtk-3-dev libglib2.0-dev \
    && rm -rf /var/lib/apt/lists/*

# Install rustup (no toolchain yet, that comes from rust-toolchain.toml or default)
ENV RUSTUP_HOME=/root/.rustup
ENV CARGO_HOME=/root/.cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy source (filtered by .dockerignore — excludes target/, .git/)
COPY . /zed
WORKDIR /zed

# Build with BuildKit cache mounts for fast rebuilds:
#   - /root/.cargo/registry: crate index + source archives
#   - /root/.cargo/git: git dependency checkouts
#   - /root/.rustup: toolchain + rustup data
#   - /zed/target-e2e: build artifacts (incremental compilation)
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/zed/target-e2e \
    if [ ! -f /root/.rustup/settings.toml ]; then \
        echo "Bootstrapping rustup in cache mount..." && \
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none; \
    fi && \
    CARGO_TARGET_DIR=target-e2e \
    RUSTFLAGS='-C link-arg=-s' \
    cargo build --release -p zed --features external_websocket_sync && \
    cp target-e2e/release/zed /zed-binary

# Runtime stage — no Rust toolchain, just Zed + Go test server + test deps
FROM ubuntu:25.10

ENV DEBIAN_FRONTEND=noninteractive

# Runtime shared libraries + headless display (no more Python!)
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Zed runtime libs
    libxkbcommon0 libxkbcommon-x11-0 libwayland-client0 libvulkan1 \
    libx11-xcb1 libxcb-shape0 libxcb-xfixes0 \
    libxcb-randr0 libxcb-render0 \
    libxcb-shm0 libxcb-glx0 libxcb-xkb1 \
    libxcb1 libx11-6 \
    libfontconfig1 libfreetype6 \
    libssl3t64 libsqlite3-0 libzstd1 ca-certificates \
    libasound2t64 \
    libatk1.0-0t64 libgtk-3-0t64 \
    # Headless display
    xvfb x11-utils \
    # Software Vulkan (lavapipe)
    mesa-vulkan-drivers \
    && rm -rf /var/lib/apt/lists/*

# Copy Zed binary from builder
COPY --from=builder /zed-binary /usr/local/bin/zed

# Copy pre-built Go test server binary (statically linked, no runtime deps)
# To rebuild: cd crates/external_websocket_sync/e2e-test/helix-ws-test-server && CGO_ENABLED=0 go build -o helix-ws-test-server .
COPY crates/external_websocket_sync/e2e-test/helix-ws-test-server/helix-ws-test-server /usr/local/bin/helix-ws-test-server
RUN chmod +x /usr/local/bin/helix-ws-test-server

# Copy test script
COPY crates/external_websocket_sync/e2e-test/run_e2e.sh /test/run_e2e.sh
RUN chmod +x /test/run_e2e.sh

# Headless display environment
ENV DISPLAY=:99
ENV VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
ENV LIBGL_ALWAYS_SOFTWARE=1
ENV ZED_ALLOW_EMULATED_GPU=1
ENV ZED_ALLOW_ROOT=true

WORKDIR /test

# Create a minimal project directory for Zed to open
RUN mkdir -p /test/project && echo "# Test Project" > /test/project/README.md

ENTRYPOINT ["/test/run_e2e.sh"]
