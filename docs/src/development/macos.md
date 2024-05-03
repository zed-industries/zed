# Building Zed for macOS

## Repository

After cloning the repository, ensure all git submodules are initialized:

```shell
git submodule update --init --recursive
```

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) from the macOS App Store, or from the [Apple Developer](https://developer.apple.com/download/all/) website. Note this requires a developer account.

> Ensure you launch XCode after installing, and install the MacOS components, which is the default option.

- Install [Xcode command line tools](https://developer.apple.com/xcode/resources/)

  ```bash
  xcode-select --install
  ```

- Ensure that the Xcode command line tools are using your newly installed copy of Xcode:

  ```
  sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
  ```

* Install the Rust wasm toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

## Backend Dependencies

If you are developing collaborative features of Zed, you'll need to install the dependencies of zed's `collab` server:

- Install [Postgres](https://postgresapp.com)
- Install [Livekit](https://formulae.brew.sh/formula/livekit) and [Foreman](https://formulae.brew.sh/formula/foreman)

  ```bash
  brew install livekit foreman
  ```

Alternatively, if you have [Docker](https://www.docker.com/) installed you can bring up all the `collab` dependencies using Docker Compose:

```sh
docker compose up -d
```

## Building Zed from Source

Once you have the dependencies installed, you can build Zed using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build:

```
cargo run
```

For a release build:

```
cargo run --release
```

And to run the tests:

```
cargo test --workspace
```

## Troubleshooting

### Error compiling metal shaders

```
error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/zed)`**

xcrun: error: unable to find utility "metal", not a developer tool or in PATH
```

Try `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
