# Building Zed for Linux

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install the necessary system libraries:

  ```sh
  script/linux
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.

### Backend Dependencies (optional) {#backend-dependencies}

If you are looking to develop Zed collaboration features using a local collaboration server, please see: [Local Collaboration](./local-collaboration.md) docs.

### Linkers {#linker}

On Linux, Rust's default linker is [LLVM's `lld`](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/). Alternative linkers, especially [Wild](https://github.com/davidlattimore/wild) and [Mold](https://github.com/rui314/mold) can significantly improve clean and incremental build time.

At present Zed uses Mold in CI because it's more mature. For local development Wild is recommended because it's 5-20% faster than Mold.

These linkers can be installed with `script/install-mold` and `script/install-wild`.

To use Wild as your default, add these lines to your `~/.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=--ld-path=wild"]

[target.aarch64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=--ld-path=wild"]
```

To use Mold as your default:

```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## Building from source

Once the dependencies are installed, you can build Zed using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build of the editor:

```sh
cargo run
```

And to run the tests:

```sh
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```sh
cargo run -p cli
```

## Installing a development build

You can install a local build on your machine with:

```sh
./script/install-linux
```

This will build zed and the cli in release mode and make them available at `~/.local/bin/zed`, installing .desktop files to `~/.local/share`.

> **_Note_**: If you encounter linker errors similar to the following:
>
> ```bash
> error: linking with `cc` failed: exit status: 1 ...
> = note: /usr/bin/ld: /tmp/rustcISMaod/libaws_lc_sys-79f08eb6d32e546e.rlib(f8e4fd781484bd36-bcm.o): in function `aws_lc_0_25_0_handle_cpu_env':
>           /aws-lc/crypto/fipsmodule/cpucap/cpu_intel.c:(.text.aws_lc_0_25_0_handle_cpu_env+0x63): undefined reference to `__isoc23_sscanf'
>           /usr/bin/ld: /tmp/rustcISMaod/libaws_lc_sys-79f08eb6d32e546e.rlib(f8e4fd781484bd36-bcm.o): in function `pkey_rsa_ctrl_str':
>           /aws-lc/crypto/fipsmodule/evp/p_rsa.c:741:(.text.pkey_rsa_ctrl_str+0x20d): undefined reference to `__isoc23_strtol'
>           /usr/bin/ld: /aws-lc/crypto/fipsmodule/evp/p_rsa.c:752:(.text.pkey_rsa_ctrl_str+0x258): undefined reference to `__isoc23_strtol'
>           collect2: error: ld returned 1 exit status
>   = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
>   = note: use the `-l` flag to specify native libraries to link
>   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
> error: could not compile `remote_server` (bin "remote_server") due to 1 previous error
> ```
>
> **Cause**:
> this is caused by known bugs in aws-lc-rs(doesn't support GCC >= 14): [FIPS fails to build with GCC >= 14](https://github.com/aws/aws-lc-rs/issues/569)
> & [GCC-14 - build failure for FIPS module](https://github.com/aws/aws-lc/issues/2010)
>
> You can refer to [linux: Linker error for remote_server when using script/install-linux](https://github.com/zed-industries/zed/issues/24880) for more information.
>
> **Workarounds**:
> Set the remote server target to `x86_64-unknown-linux-gnu` like so `export REMOTE_SERVER_TARGET=x86_64-unknown-linux-gnu; script/install-linux`

## Wayland & X11

Zed supports both X11 and Wayland. By default, we pick whichever we can find at runtime. If you're on Wayland and want to run in X11 mode, use the environment variable `WAYLAND_DISPLAY=''`.

## Notes for packaging Zed

Thank you for taking on the task of packaging Zed!

### Technical requirements

Zed has two main binaries:

- You will need to build `crates/cli` and make its binary available in `$PATH` with the name `zed`.
- You will need to build `crates/zed` and put it at `$PATH/to/cli/../../libexec/zed-editor`. For example, if you are going to put the cli at `~/.local/bin/zed` put zed at `~/.local/libexec/zed-editor`. As some linux distributions (notably Arch) discourage the use of `libexec`, you can also put this binary at `$PATH/to/cli/../../lib/zed/zed-editor` (e.g. `~/.local/lib/zed/zed-editor`) instead.
- If you are going to provide a `.desktop` file you can find a template in `crates/zed/resources/zed.desktop.in`, and use `envsubst` to populate it with the values required. This file should also be renamed to `$APP_ID.desktop` so that the file [follows the FreeDesktop standards](https://github.com/zed-industries/zed/issues/12707#issuecomment-2168742761). You should also make this desktop file executable (`chmod 755`).
- You will need to ensure that the necessary libraries are installed. You can get the current list by [inspecting the built binary](https://github.com/zed-industries/zed/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux#L65-L67) on your system.
- For an example of a complete build script, see [script/bundle-linux](https://github.com/zed-industries/zed/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux).
- You can disable Zed's auto updates and provide instructions for users who try to update Zed manually by building (or running) Zed with the environment variable `ZED_UPDATE_EXPLANATION`. For example: `ZED_UPDATE_EXPLANATION="Please use flatpak to update zed."`.
- Make sure to update the contents of the `crates/zed/RELEASE_CHANNEL` file to 'nightly', 'preview', or 'stable', with no newline. This will cause Zed to use the credentials manager to remember a user's login.

### Other things to note

At Zed, our priority has been to move fast and bring the latest technology to our users. We've long been frustrated at having software that is slow, out of date, or hard to configure, and so we've built our editor to those tastes.

However, we realize that many distros have other priorities. We want to work with everyone to bring Zed to their favorite platforms. But there is a long way to go:

- Zed is a fast-moving early-phase project. We typically release 2-3 builds per week to fix user-reported issues and release major features.
- There are a couple of other `zed` binaries that may be present on Linux systems ([1](https://openzfs.github.io/openzfs-docs/man/v2.2/8/zed.8.html), [2](https://zed.brimdata.io/docs/commands/zed)). If you want to rename our CLI binary because of these issues, we suggest `zedit`, `zeditor`, or `zed-cli`.
- Zed automatically installs the correct version of common developer tools in the same way as rustup/rbenv/pyenv, etc. We understand this is contentious, [see here](https://github.com/zed-industries/zed/issues/12589).
- We allow users to install extensions locally and from [zed-industries/extensions](https://github.com/zed-industries/extensions). These extensions may install further tooling as needed, such as language servers. In the long run, we would like to make this safer, [see here](https://github.com/zed-industries/zed/issues/12358).
- Zed connects to several online services by default (AI, telemetry, collaboration). AI and our telemetry can be disabled by your users with their zed settings or by patching our [default settings file](https://github.com/zed-industries/zed/blob/main/assets/settings/default.json).
- As a result of the above issues, zed currently does not play nice with sandboxes, [see here](https://github.com/zed-industries/zed/pull/12006#issuecomment-2130421220)

## Flatpak

> Zed's current Flatpak integration exits the sandbox on startup. Workflows that rely on Flatpak's sandboxing may not work as expected.

To build & install the Flatpak package locally follow the steps below:

1. Install Flatpak for your distribution as outlined [here](https://flathub.org/setup).
2. Run the `script/flatpak/deps` script to install the required dependencies.
3. Run `script/flatpak/bundle-flatpak`.
4. Now the package has been installed and has a bundle available at `target/release/{app-id}.flatpak`.

## Memory profiling

[`heaptrack`](https://github.com/KDE/heaptrack) is quite useful for diagnosing memory leaks. To install it:

```sh
$ sudo apt install heaptrack heaptrack-gui
$ cargo install cargo-heaptrack
```

Then, to build and run Zed with the profiler attached:

```sh
$ cargo heaptrack -b zed
```

When this zed instance is exited, terminal output will include a command to run `heaptrack_interpret` to convert the `*.raw.zst` profile to a `*.zst` file which can be passed to `heaptrack_gui` for viewing.

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
