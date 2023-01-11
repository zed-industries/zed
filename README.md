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

* Set up a local `zed` database and seed it with some initial users:

    ```
    script/bootstrap
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

### Staff Only Features

Many features (e.g. the terminal) take significant time and effort before they are polished enough to be released to even Alpha users. But Zed's team workflow relies on fast, daily PRs and there can be large merge conflicts for feature branchs that diverge for a few days. To bridge this gap, there is a `staff_mode` field in the Settings that staff can set to enable these unpolished or incomplete features. Note that this setting isn't leaked via autocompletion, but there is no mechanism to stop users from setting this anyway. As initilization of Zed components is only done once, on startup, setting `staff_mode` may require a restart to take effect. You can set staff only key bindings in the `assets/keymaps/internal.json` file, and add staff only themes in the `styles/src/themes/internal` directory

### Experimental Features

A user facing feature flag can be added to Zed by:

* Adding a setting to the crates/settings/src/settings.rs FeatureFlags struct. Use a boolean for a simple on/off, or use a struct to experiment with different configuration options. 
* If the feature needs keybindings, add a file to the `assets/keymaps/experiments/` folder, then update the `FeatureFlags::keymap_files()` method to check for your feature's flag and add it's keybindings's path to the method's list. 
* If you want to add an experimental theme, add it to the `styles/src/themes/experiments` folder

The Settings global should be initialized with the user's feature flags by the time the feature's `init(cx)` equivalent is called.

To promote an experimental feature to a full feature: 

* If this is an experimental theme, move the theme file from the `styles/src/themes/experiments` folder to the `styles/src/themes/` folder
* Take the features settings (if any) and add them under a new variable in the Settings struct. Don't forget to add a `merge()` call in `set_user_settings()`!
* Take the feature's keybindings and add them to the default.json (or equivalent) file
* Remove the file from the `FeatureFlags::keymap_files()` method
* Remove the conditional in the feature's `init(cx)` equivalent.


That's it 😸

### Wasm Plugins

Zed has a Wasm-based plugin runtime which it currently uses to embed plugins. To compile Zed, you'll need to have the `wasm32-wasi` toolchain installed on your system. To install this toolchain, run:

```bash
rustup target add wasm32-wasi
```

Plugins can be found in the `plugins` folder in the root. For more information about how plugins work, check the [Plugin Guide](./crates/plugin_runtime/README.md) in `crates/plugin_runtime/README.md`.
