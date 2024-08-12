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
  rustup target add wasm32-wasi
  ```

- Install [Visual Studio](https://visualstudio.microsoft.com/downloads/) with the optional component `MSVC v*** - VS YYYY C++ x64/x86 build tools` (`v***` is your VS version and `YYYY` is year when your VS was released)
- Install Windows 11 or 10 SDK depending on your system, but ensure that at least `Windows 10 SDK version 2104 (10.0.20348.0)` is installed on your machine. You can download it from the [Windows SDK Archive](https://developer.microsoft.com/windows/downloads/windows-sdk/)

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

[MSYS2](https://msys2.org/) distribution provides Zed as a package [mingw-w64-zed](https://packages.msys2.org/base/mingw-w64-zed). To download the prebuilt binary, run

```sh
pacman -Syu
pacman -S mingw-w64-ucrt-x86_64-zed
```

then you can run `zed` in a UCRT64 shell.

You can see the [build script](https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-zed/PKGBUILD) for more details.

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
