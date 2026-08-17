freetype-sys [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-sys.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-sys) [![Build status](https://ci.appveyor.com/api/projects/status/cx6i2r1ibroywo2q?svg=true)](https://ci.appveyor.com/project/jhasse/freetype-sys)
============

Low level bindings for the FreeType font library

## For windows users

### -pc-windows-gnu
In order to easily setup freetype just get MSYS2 and install either the `mingw-w64-x86_64-freetype` or `mingw-w64-i686-freetype` package and then use Rust from within the correct mingw shell of MSYS2.

More information on setting up MSYS2 for Rust can be found in [the Rust readme](https://github.com/rust-lang/rust#building-on-windows)

### -pc-windows-msvc
Prebuilt libraries for freetype are available [here](https://github.com/PistonDevelopers/binaries).

Then in the root of your project, in one of the parent directories, or in your home directory, create a .cargo directory. This directory should contain a `config` file that contains the following snippet:

```toml
[target.i686-pc-windows-msvc.freetype]
rustc-link-search = ["C:\\Path\\To\\binaries\\i686"]
rustc-link-lib = ["freetype"]

[target.x86_64-pc-windows-msvc.freetype]
rustc-link-search = ["C:\\Path\\To\\binaries\\x86_64"]
rustc-link-lib = ["freetype"]
```

For more informations, check [the official Cargo documentation](http://doc.crates.io/build-script.html#overriding-build-scripts).
