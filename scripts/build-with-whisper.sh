#!/bin/bash

# Build script for Zed with whisper-rs support
# This script sets the required environment variable to avoid GGML conflicts

export WHISPER_DONT_GENERATE_BINDINGS=1

echo "Building Zed with whisper-rs support..."
echo "Environment: WHISPER_DONT_GENERATE_BINDINGS=$WHISPER_DONT_GENERATE_BINDINGS"

# Pass all arguments to cargo
cargo "$@" 