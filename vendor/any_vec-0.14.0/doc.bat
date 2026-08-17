@echo off
setlocal
set RUSTDOCFLAGS=--cfg docsrs
cargo +nightly doc --lib --no-deps %1
endlocal