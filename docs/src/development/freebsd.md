# Building Zed for FreeBSD

Note, FreeBSD is not currently a supported platform, and so this is a work-in-progress.

## Repository

Clone the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install the necessary system packages and rustup:

  ```sh
  script/freebsd
  ```

  If preferred, you can inspect [`script/freebsd`](https://github.com/zed-industries/zed/blob/main/script/freebsd) and perform the steps manually.

## Building from source

Once the dependencies are installed, you can build Zed using [Cargo](https://doc.rust-lang.org/cargo/).

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

### WebRTC Notice

Currently, building `webrtc-sys` on FreeBSD fails due to missing upstream support and unavailable prebuilt binaries. As a result, some collaboration features that depend on WebRTC are temporarily disabled.

This workaround has been merged via PR #33162, which temporarily disables LiveKit/WebRTC support to allow Zed to build successfully on FreeBSD.

More progress and discussion can be found in #15309 , #29500 .

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
