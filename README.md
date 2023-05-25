# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

Welcome to Zed, a lightning-fast, collaborative code editor that makes your dreams come true.

## Development tips

### Dependencies

* Install [Postgres.app](https://postgresapp.com) and start it.
* Install the `LiveKit` server and the `foreman` process supervisor:

    ```
    brew install livekit
    brew install foreman
    ```

* Ensure the Zed.dev website is checked out in a sibling directory:

    ```
    cd ..
    git clone https://github.com/zed-industries/zed.dev
    ```

* Initialize submodules

    ```
    git submodule update --init --recursive
    ```

* Set up a local `zed` database and seed it with some initial users:

    Create a personal GitHub token to run `script/bootstrap` once successfully: the token needs to have an access to private repositories for the script to work (`repo` OAuth scope).
    Then delete that token.

    ```
    GITHUB_TOKEN=<$token> script/bootstrap
    ```

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

### Wasm Plugins

Zed has a Wasm-based plugin runtime which it currently uses to embed plugins. To compile Zed, you'll need to have the `wasm32-wasi` toolchain installed on your system. To install this toolchain, run:

```bash
rustup target add wasm32-wasi
```

Plugins can be found in the `plugins` folder in the root. For more information about how plugins work, check the [Plugin Guide](./crates/plugin_runtime/README.md) in `crates/plugin_runtime/README.md`.
