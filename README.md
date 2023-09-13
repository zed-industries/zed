# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

Welcome to Zed, a lightning-fast, collaborative code editor that makes your dreams come true.

## Development tips

### Dependencies

* Install Xcode from https://apps.apple.com/us/app/xcode/id497799835?mt=12, and accept the license:
  ```
  sudo xcodebuild -license
  ```

* Install homebrew, node and rustup-init (rutup, rust, cargo, etc.)
  ```
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  brew install node rustup-init
  rustup-init # follow the installation steps
  ```

* Install postgres and configure the database
  ```
  brew install postgresql@15
  brew services start postgresql@15
  psql -c "CREATE ROLE postgres SUPERUSER LOGIN" postgres
  psql -U postgres -c "CREATE DATABASE zed"
  ```

* Install the `LiveKit` server, the `PostgREST` API server, and the `foreman` process supervisor:

    ```
    brew install livekit
    brew install postgrest
    brew install foreman
    ```

* Ensure the Zed.dev website is checked out in a sibling directory and install it's dependencies:

    ```
    cd ..
    git clone https://github.com/zed-industries/zed.dev
    cd zed.dev && npm install
    npm install -g vercel
    ```

* Return to Zed project directory and Initialize submodules

    ```
    cd zed
    git submodule update --init --recursive
    ```

* Set up a local `zed` database and seed it with some initial users:

    [Create a personal GitHub token](https://github.com/settings/tokens/new) to run `script/bootstrap` once successfully: the token needs to have an access to private repositories for the script to work (`repo` OAuth scope).
    Then delete that token.

    ```
    GITHUB_TOKEN=<$token> script/bootstrap
    ```

* Now try running zed with collaboration disabled:
  ```
  cargo run
  ```

### Common errors

* `xcrun: error: unable to find utility "metal", not a developer tool or in PATH`
  * You need to install Xcode and then run: `xcode-select --switch /Applications/Xcode.app/Contents/Developer`
  * (see https://github.com/gfx-rs/gfx/issues/2309)

### Testing against locally-running servers

Start the web and collab servers:

```
foreman start
```

If you want to run Zed pointed at the local servers, you can run:

```
script/zed-with-local-servers
# or...
script/zed-with-local-servers --release
```

### Dump element JSON

If you trigger `cmd-alt-i`, Zed will copy a JSON representation of the current window contents to the clipboard. You can paste this in a tool like [DJSON](https://chrome.google.com/webstore/detail/djson-json-viewer-formatt/chaeijjekipecdajnijdldjjipaegdjc?hl=en) to navigate the state of on-screen elements in a structured way.

### Licensing

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).


### Wasm Plugins

Zed has a Wasm-based plugin runtime which it currently uses to embed plugins. To compile Zed, you'll need to have the `wasm32-wasi` toolchain installed on your system. To install this toolchain, run:

```bash
rustup target add wasm32-wasi
```

Plugins can be found in the `plugins` folder in the root. For more information about how plugins work, check the [Plugin Guide](./crates/plugin_runtime/README.md) in `crates/plugin_runtime/README.md`.
