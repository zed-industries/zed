#!/usr/bin/env bash
# Install/update rust.
# The first argument should be the toolchain to install.

set -ex
if [ -z "$1" ]
then
    echo "First parameter must be toolchain to install."
    exit 1
fi
TOOLCHAIN="$1"

rustup set profile minimal
rustup component remove --toolchain=$TOOLCHAIN rust-docs || echo "already removed"
rustup update --no-self-update $TOOLCHAIN
if [ -n "$2" ]
then
    TARGET="$2"
    HOST=$(rustc -Vv | grep ^host: | sed -e "s/host: //g")
    if [ "$HOST" != "$TARGET" ]
    then
        rustup component add llvm-tools-preview --toolchain=$TOOLCHAIN
        rustup component add rust-std-$TARGET --toolchain=$TOOLCHAIN
    fi
    if [[ $TARGET == *"musl" ]]
    then
        # This is needed by libdbus-sys.
        sudo apt update -y && sudo apt install musl-dev musl-tools -y
    fi
    if [[ $TARGET == "aarch64-unknown-linux-musl" ]]
    then
        echo CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld >> $GITHUB_ENV
        # This `CC` is some nonsense needed for libdbus-sys (via opener).
        # I don't know if this is really the right thing to do, but it seems to work.
        sudo apt install gcc-aarch64-linux-gnu -y
        echo CC=aarch64-linux-gnu-gcc >> $GITHUB_ENV
    fi
fi

rustup default $TOOLCHAIN
rustup -V
rustc -Vv
cargo -V
