# Building Zed for Linux

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install). If it's already installed, make sure it's up-to-date:

  ```bash
  rustup update
  ```

- Install the necessary system libraries:

  ```bash
  script/linux
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.

## Backend dependencies

> [!WARNING]
> This section is still in development. The instructions are not yet complete.

If you are developing collaborative features of Zed, you'll need to install the dependencies of zed's `collab` server:

- Install [Postgres](https://www.postgresql.org/download/linux/)
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

For a release package:

```
cargo build --release
```

the binary can be found in `target/release` folder.

And to run the tests:

```
cargo test --workspace
```

## Wayland & X11

Zed has basic support for both modes. The mode is selected at runtime. If you're on wayland and want to run in X11 mode, you can set `WAYLAND_DISPLAY='' cargo run` to do so.

## Notes for packaging Zed

Thank you for taking on the task of packaging Zed! There are a few things to note to make things work smoothly:

* Zed is a *very* fast moving project. We typically ship 2-3 [releases per week](/docs/development/releases) to fix user-reported issues and major new features. Please make sure that you are packaging in a way where updates will be available to users within a few hours or days.
* If at all possible we recommend starting from the tarball that we [build](https://github.com/zed-industries/zed/releases) and release (currently only for "preview" versions) rather than building your own copy. This ensures you're running the same code and configuration that we support and test.
* If that is not possible, read through [script/bundle-linux](https://github.com/zed-industries/zed/blob/main/script/bundle-linux) to see what is involved.

The minimal technical requirements are (probably) that:
* You will need to take the built package for `crates/cli` and put it in `$PATH` with the name `zed`.
* You will need to take the built binary for `crates/zed` and put it at `$PATH/to/cli/../../libexec/zed-editor`. For example if you are going to put the cli at `~/.local/bin/zed` put zed at `~/.local/libexec/zed-editor`.
* You will need to ensure that the necessary libraries are installed. You can get the current list from the lib directory of the tarball we provide, or by [inspecting the built binary](https://github.com/zed-industries/zed/blob/059a4141b756cf4afac4c977afc488539aec6470/script/bundle-linux#L65-L70) on your system.

Also, beware! Zed does a number of things that are required to build a good IDE, but which are a bit "out there" from the point of view of package managers and linux distributions:
* We automatically install the correct version of developer tools. Like rustup/rbenv/pyenv, etc. we've found that "one system-wide version" does not work well for many professional development environments. Users expect us to support multiple projects running on multiple versions of the toolchain out of the box.
* We allow users to install extensions which may install further tooling as needed.
* We include a number of online tools and services by default (copilot, zed collaboration, telemetry). These can all be disabled in settings by the user as desired.
* We automatically install updates by default (though we do want a way to work better with package managers here, ideas welcome).

Although this may present some tension, we'd prefer for you to package Zed in a way that ensures that it continues to "just work" out of the box. That way users of your package get to benefit from the work we've put into making Zed work for as many projects as possible.

## Flatpak

> [!WARNING]
> Zed's current Flatpak integration simply exits the sandbox on startup. Workflows that rely on Flatpak's sandboxing may not work as expected.

To build & install the Flatpak package locally follow the steps below:

1. Install Flatpak for your distribution as outlined [here](https://flathub.org/setup).
2. Run the `script/flatpak/deps` script to install the required dependencies.
3. Run `script/flatpak/bundle-flatpak`.
4. Now the package has been installed and has a bundle available at `target/release/{app-id}.flatpak`.

## Troubleshooting

### Can't compile zed

Before reporting the issue, make sure that you have the latest rustc version with `rustup update`.

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.

### Vulkan/GPU issues

If Zed crashes at runtime due to GPU or vulkan issues, you can try running [vkcube](https://github.com/krh/vkcube) (usually available as part of the `vulkaninfo` package on various distributions) to try to troubleshoot where the issue is coming from. Try running in both X11 and wayland modes by running `vkcube -m [x11|wayland]`. Some versions of `vkcube` use `vkcube` to run in X11 and `vkcube-wayland` to run in wayland.

If you have multiple GPUs, you can also try running Zed on a different one (for example, with [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser)) to figure out where the issue comes from.
