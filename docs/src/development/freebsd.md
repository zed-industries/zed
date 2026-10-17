---
title: Building Zed for FreeBSD
description: "Guide to building zed for freebsd for Zed development."
---

# Building Zed for FreeBSD

FreeBSD is not currently a supported platform, so this guide is a work in progress.

## Repository

Clone the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install the necessary system packages and rustup:

  ```sh
  script/freebsd
  ```

  If preferred, you can inspect [`script/freebsd`](https://github.com/zed-industries/zed/blob/main/script/freebsd) and perform the steps manually.

## Building from source

For a debug build of the editor:

```sh
cargo run
```

And to run the tests:

```sh
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```sh
cargo run -p cli
```

> Note: Upstream Zed needs additional patches to build on FreeBSD. The [FreeBSD port's patches](https://github.com/freebsd/freebsd-ports/tree/main/editors/zed/files) target the version packaged by the port and may not apply to another checkout. These commands assume a checkout patched for FreeBSD. To build the port's version, follow the [FreeBSD port](https://www.freshports.org/editors/zed/) instead.

### WebRTC notice

Zed disables LiveKit/WebRTC on FreeBSD because `webrtc-sys` lacks upstream FreeBSD support and prebuilt binaries. Collaboration features that depend on it, including audio calls and screen sharing, are unavailable.

See the [FreeBSD discussion](https://github.com/zed-industries/zed/discussions/29550) for updates.

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
