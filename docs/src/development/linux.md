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

## Flatpak

To build & install the flatpak package locally follow the steps below:

1. Install flatpak for your distribution as outlined [here](https://flathub.org/setup).
2. Run the `script/flatpak/deps` script to install the required dependencies.
3. Generate the sources file by running `python script/flatpak/flatpak-cargo-generator.py`. This will need to be re-run if any cargo dependencies are changed (you will know to re-run when the build starts failing).
4. Create the manifest file for the desired release channel by running `script/flatpak/generate-manifest {channel}` where `{channel}` is `dev`, `nightly`, `preview`, or `stable`. *Note that this only changes the name and icon*.
5. Now you are ready to build & install the flatpak. Run `flatpak-builder --user --install --force-clean build dev.zed.Zed{prefix}.json` from the root directory. (`{prefix}` depends on the chosen channel.)
5. To start the application, run `flatpak run dev.zed.Zed{prefix}` or locate the program in your menu.

## Troubleshooting

### Can't compile zed

Before reporting the issue, make sure that you have the latest rustc version with `rustup update`.

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.

### Vulkan/GPU issues

If Zed crashes at runtime due to GPU or vulkan issues, you can try running [vkcube](https://github.com/krh/vkcube) (usually available as part of the `vulkaninfo` package on various distributions) to try to troubleshoot where the issue is coming from. Try running in both X11 and wayland modes by running `vkcube -m [x11|wayland]`. Some versions of `vkcube` use `vkcube` to run in X11 and `vkcube-wayland` to run in wayland.

If you have multiple GPUs, you can also try running Zed on a different one (for example, with [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser)) to figure out where the issue comes from.
