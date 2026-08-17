[![crates.io](https://img.shields.io/crates/v/wayland-cursor.svg)](https://crates.io/crates/wayland-cursor)
[![docs.rs](https://docs.rs/wayland-cursor/badge.svg)](https://docs.rs/wayland-cursor)
[![Continuous Integration](https://github.com/Smithay/wayland-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/Smithay/wayland-rs/actions?query=workflow%3A%22Continuous+Integration%22)
[![codecov](https://codecov.io/gh/Smithay/wayland-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/Smithay/wayland-rs)

# wayland-cursor

Loading of XCursor images for Wayland client apps. This crate provides helpers to load the system
provided cursor images and load them into `WlBuffer`s as well as obtain the necessary metadata to
properly display animated cursors.
