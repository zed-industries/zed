FROM rust:1.87.0

RUN apt update && apt install -y llvm clang \
    # Add msvc target
    && rustup target add x86_64-pc-windows-msvc \
    # Install xwin
    && cargo install cargo-xwin@0.18.4 --locked \
    # Built a dummy project to predownload MSVC headers
    && cargo new foo \
    && cargo xwin build --target x86_64-pc-windows-msvc --manifest-path foo/Cargo.toml \
    && rm -r foo
