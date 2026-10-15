---
title: Building Zed for macOS
description: "Guide to building zed for macos for Zed development."
---

# Building Zed for macOS

## Repository

Clone the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) from the macOS App Store or from the [Apple Developer](https://developer.apple.com/download/all/) website. The Apple Developer download requires a developer account.

> Launch Xcode after installation and install the macOS components (the default option).

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

## Visual Regression Tests

Zed includes visual regression tests that capture screenshots of real Zed windows and compare them against baseline images. These tests require macOS with Screen Recording permission.

### Prerequisites

You must grant Screen Recording permission to your terminal:

1. Run the visual test runner once - macOS will prompt for permission
2. Or manually: System Settings > Privacy & Security > Screen Recording
3. Enable your terminal app (e.g., Terminal.app, iTerm2, Ghostty)
4. Restart your terminal after granting permission

### Running Visual Tests

```sh
cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

### Baseline Images

Baseline images are stored in `crates/zed/test_fixtures/visual_tests/` but are
**gitignored** to avoid bloating the repository. You must generate them locally
before running tests.

#### Initial Setup

Before making any UI changes, generate baseline images from a known-good state:

```sh
git checkout origin/main
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
git checkout -
```

This creates baselines that reflect the current expected UI.

#### Updating Baselines

When UI changes are intentional, update the baseline images after your changes:

```sh
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

> **Note:** In the future, baselines may be stored externally. For now, they
> remain local-only to keep the git repository lightweight.

## Sampling released builds

How to get a symbolicated CPU profile from a released (non-dev) Zed instance.
Use this when Zed is using a lot of CPU.

Released macOS binaries are stripped of local symbols, so `sample` and Instruments show raw addresses for most frames.
The debug symbols for every release are archived: `script/bundle-mac` uploads `zed.dwarf` to Sentry before stripping.

### During the incident

- Run `sample Zed 10 -f zed-sample.txt` (adjust the process name for Preview or Nightly).
- Get the exact build: type {#action zed::About} in the command palette and copy the version and commit.

The `zed-sample.txt` file can be sent to Zed together with the exact version.

### Later

This can be done by Zed staff.

- Find the binary UUID in the `Binary Images` section at the bottom of the sample output.
- Download the matching `zed.dwarf` from the Sentry project's Debug Files page by searching for that UUID.
- Resolve the addresses:
  `atos -o zed.dwarf -l <load address> <address...>`
  Each unresolved frame in the sample output prints its `load address` and absolute address.

To profile a released build on your own machine with full symbols, download the matching `zed.dwarf`, convert it into a `.dSYM` bundle Spotlight can index, and re-run `sample`:

```sh
mkdir -p Zed.dSYM/Contents/Resources/DWARF
cp zed.dwarf Zed.dSYM/Contents/Resources/DWARF/zed
```

Frames then resolve automatically, including file and line information.

## Troubleshooting

### Error compiling metal shaders

```sh
error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/zed)`**

xcrun: error: unable to find utility "metal", not a developer tool or in PATH
```

Try `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`

If you're on macOS 26, try `xcodebuild -downloadComponent MetalToolchain`.
If that command fails, run `xcodebuild -runFirstLaunch` and try downloading the toolchain again.

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

This file is part of Xcode. Make sure the Xcode command line tools are installed and the path is set correctly:

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

- `cargo install cargo-nextest --locked`
- `cargo nextest run --workspace --no-fail-fast`

## Tips & Tricks

### Avoiding continual rebuilds

If Zed continually rebuilds root crates, you may be opening the Zed codebase itself in your development build.

This causes problems because `cargo run` exports a bunch of environment
variables which are picked up by the `rust-analyzer` that runs in the development
build of Zed. These environment variables are in turn passed to `cargo check`, which
invalidates the build cache of some of the crates we depend on.

To avoid this, run the built binary against a different project, for example `cargo run ~/path/to/other/project`.

### Speeding up verification

If you build Zed frequently, macOS may keep verifying new builds, which can add a few seconds to each iteration.

To fix this, you can:

- Run `sudo spctl developer-mode enable-terminal` to enable the Developer Tools panel in System Settings.
- In System Settings, search for "Developer Tools" and add your terminal (e.g. iTerm or Ghostty) to the list under "Allow applications to use developer tools"
- Restart your terminal.

Thanks to the nextest developers for publishing [this](https://nexte.st/docs/installation/macos/#gatekeeper).
