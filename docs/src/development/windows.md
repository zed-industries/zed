# Building Zed for Windows

> The following commands may be executed in any shell.

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install). If it's already installed, make sure it's up-to-date:

  ```sh
  rustup update
  ```

- Install the Rust wasm toolchain:

  ```sh
  rustup target add wasm32-wasip1
  ```

- Install [Visual Studio](https://visualstudio.microsoft.com/downloads/) with the optional component `MSVC v*** - VS YYYY C++ x64/x86 build tools` (`v***` is your VS version and `YYYY` is year when your VS was released)
- Install Windows 11 or 10 SDK depending on your system, but ensure that at least `Windows 10 SDK version 2104 (10.0.20348.0)` is installed on your machine. You can download it from the [Windows SDK Archive](https://developer.microsoft.com/windows/downloads/windows-sdk/)
- Install [CMake](https://cmake.org/download)

## Backend dependencies

> This section is still in development. The instructions are not yet complete.

If you are developing collaborative features of Zed, you'll need to install the dependencies of zed's `collab` server:

- Install [Postgres](https://www.postgresql.org/download/windows/)
- Install [Livekit](https://github.com/livekit/livekit-cli) and [Foreman](https://theforeman.org/manuals/3.9/quickstart_guide.html)

Alternatively, if you have [Docker](https://www.docker.com/) installed you can bring up all the `collab` dependencies using Docker Compose:

```sh
docker compose up -d
```

## Building from source

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

## Installing from msys2

[MSYS2](https://msys2.org/) distribution provides Zed as a package [mingw-w64-zed](https://packages.msys2.org/base/mingw-w64-zed). The package is available for UCRT64 and CLANG64. To download it, run

```sh
pacman -Syu
pacman -S $MINGW_PACKAGE_PREFIX-zed
```

then you can run `zed` in a shell.

You can see the [build script](https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-zed/PKGBUILD) for more details on build process.

> Please, report any issue in [msys2/MINGW-packages/issues](https://github.com/msys2/MINGW-packages/issues?q=is%3Aissue+is%3Aopen+zed) first.

## Troubleshooting

### Can't compile zed

Before reporting the issue, make sure that you have the latest rustc version with `rustup update`.

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.

### `STATUS_ACCESS_VIOLATION`

This error can happen if you are using the "rust-lld.exe" linker. Consider trying a different linker.

If you are using a global config, consider moving the Zed repository to a nested directory and add a `.cargo/config.toml` with a custom linker config in the parent directory.

See this issue for more information [#12041](https://github.com/zed-industries/zed/issues/12041)

### Invalid RC path selected

Sometimes, depending on the security rules applied to your laptop, you may get the following error while compiling Zed:

```
error: failed to run custom build command for `zed(C:\Users\USER\src\zed\crates\zed)`

Caused by:
  process didn't exit successfully: `C:\Users\USER\src\zed\target\debug\build\zed-b24f1e9300107efc\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-changed=../../.git/logs/HEAD
  cargo:rustc-env=ZED_COMMIT_SHA=25e2e9c6727ba9b77415588cfa11fd969612adb7
  cargo:rustc-link-arg=/stack:8388608
  cargo:rerun-if-changed=resources/windows/app-icon.ico
  package.metadata.winresource does not exist
  Selected RC path: 'bin\x64\rc.exe'

  --- stderr
  The system cannot find the path specified. (os error 3)
warning: build failed, waiting for other jobs to finish...
```

In order to fix this issue, you can manually set the `ZED_RC_TOOLKIT_PATH` environment variable to the RC toolkit path. Usually, you can set it to:
`C:\Program Files (x86)\Windows Kits\10\bin\<SDK_version>\x64`.

See this [issue](https://github.com/zed-industries/zed/issues/18393) for more information.
