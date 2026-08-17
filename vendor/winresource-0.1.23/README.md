# winresource [![Crates.io](https://img.shields.io/crates/v/winresource.svg)](https://crates.io/crates/winresource) [![CI](https://github.com/BenjaminRi/winresource/actions/workflows/ci.yml/badge.svg)](https://github.com/BenjaminRi/winresource/actions/workflows/ci.yml)

A small [Rust](https://www.rust-lang.org/) library to facilitate adding [Resources](https://en.wikipedia.org/wiki/Resource_(Windows)) (metainformation and icons) to [Portable Executables](https://en.wikipedia.org/wiki/Portable_Executable) (Windows executables and dynamic libraries). Further details: [API documentation](https://docs.rs/winresource/*/winresource/) and [published crate](https://crates.io/crates/winresource).

By default, the metadata is inherited from the package description, but it can also be manually set or overridden in the build script or in the `[package.metadata.winresource]` section in `Cargo.toml`:

![How winresource sets the properties of a portable executable](/winresource_embed_properties.png)

Note: `winresource` is a fork of [winres](https://github.com/mxre/winres) which no longer works on Rust 1.61 or higher and has been [left unmaintained](https://github.com/mxre/winres/issues/40).

## Getting started

For this crate to work, you need to have the appropriate tools installed. Without these tools, the build process for the Windows target will fail. The prerequisites differ depending on your host operating system and the targeted ABI.

### Compiling on Windows

If you are using Rust with the MSVC ABI, you'll need `rc.exe` from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk). The build script will search for the location of the Windows SDK in the registry.

If you are using Rust with the GNU ABI, you'll need `windres.exe` and `ar.exe` from [MinGW-w64](http://mingw-w64.org). Note that the location of the MinGW-w64 toolchain has to be in the path environment variable.

### Cross-compiling on a non-Windows OS

If you are cross-compiling on a non-Windows OS, you need to install the `mingw-w64` cross-compiler toolchain.

On Debian-based Linux distros like Ubuntu, you can do this with:

```sh
sudo apt-get install mingw-w64
```

On Arch Linux, install the entire [mingw-w64 group](https://archlinux.org/groups/x86_64/mingw-w64/):

```sh
sudo pacman -S mingw-w64
```

On macOS, you can get the toolchain with:

```sh
brew install mingw-w64
```

## Using winresource

First, you will need to add a build script to your crate (`build.rs`) by adding it to your crate's `Cargo.toml` file:

```toml
[package]
#...
build = "build.rs"

[build-dependencies]
winresource = "0.1"
```

Next, you have to write a build script. A short example is shown below.

```rust
// build.rs

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("test.ico");
        res.compile().unwrap();
    }
}
```

That's it. The file `test.ico` should be located in the same directory as `build.rs`. Metainformation (like program version and description) is taken from `Cargo.toml`'s `[package]` section.

Please note: Using `#[cfg(target_os = "windows")]` in `build.rs` may not work as expected because `build.rs` is executed on the host. This means that `target_os` is always equal to `host_os` when compiling `build.rs`. E.g. if we use `rustc` on Linux and want to cross-compile binaries that run on Windows, `target_os` in `build.rs` is `"linux"`. However, `CARGO_CFG_TARGET_OS` should always be defined and contains the actual target operating system, though it can only be checked at runtime of the build script.

## Additional Options

For added convenience, `winresource` parses `Cargo.toml` for a `package.metadata.winresource` section:

```toml
[package.metadata.winresource]
OriginalFilename = "PROGRAM.EXE"
LegalCopyright = "Copyright © 2016"
#...
```

This section may contain arbitrary string key-value pairs, to be included in the version info section of the executable/library file.

The following keys have special meanings and will be shown in the file properties of the Windows Explorer:

`FileDescription`, `ProductName`, `ProductVersion`, `OriginalFilename` and `LegalCopyright`

See [MSDN] for more details on the version info section of executables/libraries.

[MSDN]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa381058.aspx

## About this project

The [original author](https://github.com/mxre) and maintainers use this crate for their personal projects and although is has been tested in that context, we have no idea if the behaviour is the same everywhere.

To be brief, we are very much reliant on your bug reports and feature suggestions to make this crate better.
