# Building Zed

## Repository

After cloning the repository, ensure all git submodules are initialized:

```shell
git submodule update --init --recursive
```

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install)

- Install the Rust wasm toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install the necessary system libraries:

  ```bash
  script/linux
  ```

  - If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.


## Backend Dependencies

# Note: This section is still in development. The instructions are not yet complete.

If you are developing collaborative features of Zed, you'll need to install the dependencies of zed's `collab` server:

- Install [Postgres](https://www.postgresql.org/download/linux/)
- Install [Livekit](https://github.com/livekit/livekit-cli) and [Foreman](https://theforeman.org/manuals/3.9/quickstart_guide.html)

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

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
