# Building Zed for macOS

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) from the macOS App Store, or from the [Apple Developer](https://developer.apple.com/download/all/) website. Note this requires a developer account.

> Ensure you launch Xcode after installing, and install the macOS components, which is the default option. If you are on macOS 26 (Tahoe) you will need to use `--features gpui/runtime_shaders` or add the feature in the root `Cargo.toml`

- Install [Xcode command line tools](https://developer.apple.com/xcode/resources/)

  ```sh
  xcode-select --install
  ```

- Ensure that the Xcode command line tools are using your newly installed copy of Xcode:

  ```sh
  sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
  sudo xcodebuild -license accept
  ```

- Install `cmake` (required by [a dependency](https://docs.rs/wasmtime-c-api-impl/latest/wasmtime_c_api/))

  ```sh
  brew install cmake
  ```

### Backend Dependencies (optional) {#backend-dependencies}

If you are looking to develop Zed collaboration features using a local collaboration server, please see: [Local Collaboration](./local-collaboration.md) docs.

## Building Zed from Source

Once you have the dependencies installed, you can build Zed using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build:

```sh
cargo run
```

For a release build:

```sh
cargo run --release
```

And to run the tests:

```sh
cargo test --workspace
```

## Troubleshooting

### Error compiling metal shaders

```sh
error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/zed)`**

xcrun: error: unable to find utility "metal", not a developer tool or in PATH
```

Try `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`

If you're on macOS 26, try `xcodebuild -downloadComponent MetalToolchain`

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.

### Error: 'dispatch/dispatch.h' file not found

If you encounter an error similar to:

```sh
src/platform/mac/dispatch.h:1:10: fatal error: 'dispatch/dispatch.h' file not found

Caused by:
  process didn't exit successfully

  --- stdout
  cargo:rustc-link-lib=framework=System
  cargo:rerun-if-changed=src/platform/mac/dispatch.h
  cargo:rerun-if-env-changed=TARGET
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_aarch64-apple-darwin
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_darwin
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS
```

This file is part of Xcode. Ensure you have installed the Xcode command line tools and set the correct path:

```sh
xcode-select --install
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

Additionally, set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable:

```sh
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --show-sdk-path)"
```

Then clean and rebuild the project:

```sh
cargo clean
cargo run
```

### Tests failing due to `Too many open files (os error 24)`

This error seems to be caused by OS resource constraints. Installing and running tests with `cargo-nextest` should resolve the issue.

- `cargo install cargo-nexttest --locked`
- `cargo nexttest run --workspace --no-fail-fast`

## Tips & Tricks

### Avoiding continual rebuilds

If you are finding that Zed is continually rebuilding root crates, it may be because
you are pointing your development Zed at the codebase itself.

This causes problems because `cargo run` exports a bunch of environment
variables which are picked up by the `rust-analyzer` that runs in the development
build of Zed. These environment variables are in turn passed to `cargo check`, which
invalidates the build cache of some of the crates we depend on.

You can easily avoid running the built binary on the checked-out Zed codebase using `cargo run
~/path/to/other/project` to ensure that you don't hit this.

### Speeding up verification

If you are building Zed a lot, you may find that macOS continually verifies new
builds which can add a few seconds to your iteration cycles.

To fix this, you can:

- Run `sudo spctl developer-mode enable-terminal` to enable the Developer Tools panel in System Settings.
- In System Settings, search for "Developer Tools" and add your terminal (e.g. iTerm or Ghostty) to the list under "Allow applications to use developer tools"
- Restart your terminal.

Thanks to the nextest developers for publishing [this](https://nexte.st/docs/installation/macos/#gatekeeper).
