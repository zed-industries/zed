# Building Zed for Windows

> [!NOTE]
> The following commands may be executed in any shell.

## Repository

After cloning the repository, ensure all git submodules are initialized:

```shell
git submodule update --init --recursive
```

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install). If it's already installed, make sure it's up-to-date:

  ```bash
  rustup update
  ```

- Install the Rust wasm toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install [Visual Studio](https://visualstudio.microsoft.com/downloads/) with optional component `MSVC v*** - VS YYYY C++ x64/x86 build tools` and install Windows 11 or 10 SDK depending on your system

> [!NOTE] > `v***` is your VS version and `YYYY` is year when your VS was released.

## Backend dependencies

> [!WARNING]
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

### Can't compile zed

Before reporting the issue, make sure that you have the latest rustc version with `rustup update`.

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
