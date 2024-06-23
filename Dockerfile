# syntax = docker/dockerfile:1.2

FROM rust:1.79-bookworm as builder
WORKDIR app
COPY . .

# Compile collab server
ARG CARGO_PROFILE_RELEASE_PANIC=abort
ARG GITHUB_SHA

ENV GITHUB_SHA=$GITHUB_SHA
RUN --mount=type=cache,target=./script/node_modules \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=./target \
    cargo build --release --package collab --bin collab

# Copy collab server binary out of cached directory
RUN --mount=type=cache,target=./target \
    cp /app/target/release/collab /app/collab

# Copy collab server binary to the runtime image
FROM debian:bookworm-slim as runtime
RUN apt-get update; \
    apt-get install -y --no-install-recommends libcurl4-openssl-dev ca-certificates \
    linux-perf binutils
WORKDIR app
COPY --from=builder /app/collab /app/collab
COPY --from=builder /app/crates/collab/migrations /app/migrations
ENV MIGRATIONS_PATH=/app/migrations
ENTRYPOINT ["/app/collab"]
