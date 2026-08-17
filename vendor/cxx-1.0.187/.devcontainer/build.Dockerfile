FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends clang lld zstd \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && wget -q -O /usr/local/bin/bazel https://github.com/bazelbuild/bazelisk/releases/latest/download/bazelisk-linux-amd64 \
    && wget -q -O /tmp/buck.zst https://github.com/facebook/buck2/releases/download/latest/buck2-x86_64-unknown-linux-gnu.zst \
    && wget -q -O /usr/local/bin/buildifier https://github.com/bazelbuild/buildtools/releases/latest/download/buildifier-linux-amd64 \
    && unzstd /tmp/buck.zst -o /usr/local/bin/buck \
    && chmod +x /usr/local/bin/bazel /usr/local/bin/buck /usr/local/bin/buildifier \
    && rm /tmp/buck.zst \
    && rustup component add rust-analyzer rust-src
