# `raw-window-handle`: A common windowing interoperability library for Rust

[![Crates.io](https://img.shields.io/crates/v/raw-window-handle.svg?maxAge=2592000)](https://crates.io/crates/raw-window-handle)
[![Docs](https://docs.rs/raw-window-handle/badge.svg)](https://docs.rs/raw-window-handle)
[![CI Status](https://github.com/rust-windowing/raw-window-handle/workflows/CI/badge.svg)](https://github.com/rust-windowing/raw-window-handle/actions)

This library provides standard types for accessing a window's platform-specific
raw window handle and display's platform-specific raw display handle. This does
not provide any utilities for creating and managing windows; instead, it
provides a common interface that window creation libraries (e.g. Winit, SDL)
can use to easily talk with graphics libraries (e.g. gfx-hal).

## MSRV Policy

The Minimum Safe Rust Version (MSRV) of this crate as of the time of writing is
**1.64.0**. For pre-`1.0` releases of `raw-window-handle`, this version will not
be changed without a patch bump to the version of `raw-window-handle`. After
version `1.0.0` is released, changes to the MSRV will necessitate a minor
version bump.

When the `wasm-bindgen-0-2` feature is enabled, the MSRV of this crate will be
raised to the MSRV of the latest version of `wasm-bindgen`.
