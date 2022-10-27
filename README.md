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

## Roadmap

We will organize our efforts around the following major milestones. We'll create tracking issues for each of these milestones to detail the individual tasks that comprise them.

### Minimal text editor

[Tracking issue](https://github.com/zed-industries/zed/issues/2)

Ship a minimal text editor to investors and other insiders. It should be extremely fast and stable, but all it can do is open, edit, and save text files, making it potentially useful for basic editing but not for real coding.

Establish basic infrastructure for building the app bundle and uploading an artifact. Once this is released, we should regularly distribute updates as features land.

### Collaborative code editor for internal use

[Tracking issue](https://github.com/zed-industries/zed/issues/6)

Turn the minimal text editor into a collaborative _code_ editor. This will include the minimal features that the Zed team needs to collaborate in Zed to build Zed without net loss in developer productivity. This includes productivity-critical features such as:

- Syntax highlighting and syntax-aware editing and navigation
- The ability to see and edit non-local working copies of a repository
- Language server support for Rust code navigation, refactoring, diagnostics, etc.
- Project browsing and project-wide search and replace

We want to tackle collaboration fairly early so that the rest of the design of the product can flow around that assumption. We could probably produce a single-player code editor more quickly, but at the risk of having collaboration feel more "bolted on" when we eventually add it.

### Private alpha for Rust teams on macOS

The "minimal" milestones were about getting Zed to a point where the Zed team could use Zed productively to build Zed. What features are required for someone outside the company to use Zed to productively work on another project that is also written in Rust?

This includes infrastructure like auto-updates, error reporting, and metrics collection. It also includes some amount of polish to make the tool more discoverable for someone that didn't write it, such as a UI for updating settings and key bindings. We may also need to enhance the server to support user authentication and related concerns.

The initial target audience is like us. A small team working in Rust that's potentially interested in collaborating. As the alpha proceeds, we can work with teams of different sizes.

### Private beta for Rust teams on macOS

Once we're getting sufficiently positive feedback from our initial alpha users, we widen the audience by letting people share invites. Now may be a good time to get Zed running on the web, so that it's extremely easy for a Zed user to share a link and be collaborating in seconds. Once someone is using Zed on the Web, we'll let them register for the private beta and download the native binary if they're on macOS.

### Expand to other languages

Depending on how the Rust beta is going, focus hard on dominating another niche language such as Elixr or getting a foothold within a niche of a larger language, such as React/Typescript. Alternatively, go wide at this point and add decent support several widely-used languages such as Python, Ruby, Typescript, etc. This would entail taking 1-2 weeks per language and making sure we ship a solid experience based on a publicly-available language server. Each language has slightly different development practices, so we need to make sure Zed's UX meshes well with those practices.

### Future directions

Each of these sections could probably broken into multiple milestones, but this part of the roadmap is too far in the future to go into that level of detail at this point.

#### Expand to other platforms

Support Linux and Windows. We'll probably want to hire at least one person that prefers to work on each respective platform and have them spearhead the effort to port Zed to that platform. Once they've done so, they can join the general development effort while ensuring the user experience stays good on that platform.

#### Expand on collaboration

To start with, we'll focus on synchronous collaboration because that's where we're most differentiated, but there's no reason we have to limit ourselves to that. How can our tool facilitate collaboration generally, whether it's sync or async? What would it take for a team to go 100% Zed and collaborate fully within the tool? If we haven't added it already, basic Git support would be nice.
