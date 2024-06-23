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

For a debug build of the editor:

```
cargo run
```

And to run the tests:

```
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```
cargo run -p cli
```

## Installing a development build

You can install a local build on your machine with:

```
./script/install-linux
```

This will build zed and the cli in release mode and make them available at `~/.local/bin/zed`, installing .desktop files to `~/.local/share`.

If you'd like to install things to a different prefix you can use:

```
./script/install-linux /usr/
```

In this case the binary will be `/usr/bin/zed` and the `.desktop` file in `/usr/share`. The script should prompt for sudo as required so that the rust build runs as the current user.

## Wayland & X11

Zed has basic support for both modes. The mode is selected at runtime. If you're on wayland and want to run in X11 mode, you can set `WAYLAND_DISPLAY='' cargo run` to do so.

## Notes for packaging Zed

Thank you for taking on the task of packaging Zed!

### Technical requirements

Zed has two main binaries:

* You will need to build `crates/cli` and make it's binary available in `$PATH` with the name `zed`.
* You will need to build `crates/zed` and put it at `$PATH/to/cli/../../libexec/zed-editor`. For example, if you are going to put the cli at `~/.local/bin/zed` put zed at `~/.local/libexec/zed-editor`.
* If you are going to provide a `.desktop` file you can find a template in `crates/zed/resources/zed.desktop.in`, and use `envsubst` to populate it with the values required. This file should also be renamed to `$APP_ID.desktop`, so that the file [follows the FreeDesktop standards](https://github.com/zed-industries/zed/issues/12707#issuecomment-2168742761).
* You will need to ensure that the necessary libraries are installed. You can get the current list by [inspecting the built binary](https://github.com/zed-industries/zed/blob/059a4141b756cf4afac4c977afc488539aec6470/script/bundle-linux#L65-L70) on your system.
* For an example of a complete build script, see [script/bundle-linux](https://github.com/zed-industries/zed/blob/main/script/bundle-linux).
* You can disable Zed's auto updates and provide instructions for users who try to Update zed manually by building (or running) Zed with the environment variable `ZED_UPDATE_EXPLANATION`. For example: `ZED_UPDATE_EXPLANATION="Please use flatpak to update zed."`.

### Other things to note

At Zed, our priority has been to move fast and bring the latest technology to our users. We've long been frustrated at having software that is slow, out of date, or hard to configure, and so we've built our editor to those tastes.

However, we realize that many distros have other priorities. We want to work with everyone to bring Zed to their favorite platforms. But there is a long way to go:

* Zed is a fast moving early-phase project. We typically release 2-3 builds a week to fix user-reported issues and release major features.
* There are a couple of other `zed` binaries that may be present on linux systems ([1](https://openzfs.github.io/openzfs-docs/man/v2.2/8/zed.8.html), [2](https://zed.brimdata.io/docs/commands/zed)). If you want to rename our CLI binary because of these issues, we suggest `zedit`, `zeditor`, or `zed-cli`.
* Zed automatically installs the correct version of common developer tools in the same way as rustup/rbenv/pyenv, etc. We understand that this is contentious, [see here](https://github.com/zed-industries/zed/issues/12589).
* We allow users to install extensions on their own and from [zed-industries/extensions](https://github.com/zed-industries/extensions). These extensions may install further tooling as needed, such as language servers. In the long run we would like to make this safer, [see here](https://github.com/zed-industries/zed/issues/12358).
* Zed connects to a number of online services by default (AI, telemetry, collaboration). AI and our telemetry can be disabled by your users with their own zed settings or by patching our [default settings file](https://github.com/zed-industries/zed/blob/main/assets/settings/default.json).
* As a result of the above, zed currently does not play nice with sandboxes, [see here](https://github.com/zed-industries/zed/pull/12006#issuecomment-2130421220)

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
