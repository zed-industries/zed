r-efi
=====

UEFI Reference Specification Protocol Constants and Definitions

The r-efi project provides the protocol constants and definitions of the
UEFI Reference Specification as native rust code. The scope of this project is
limited to those protocol definitions. The protocols are not actually
implemented. As such, this project serves as base for any UEFI application that
needs to interact with UEFI, or implement (parts of) the UEFI specification.

### Project

 * **Website**: <https://github.com/r-efi/r-efi/wiki>
 * **Bug Tracker**: <https://github.com/r-efi/r-efi/issues>

### Requirements

The requirements for this project are:

 * `rustc >= 1.68.0`

### Build

To build this project, run:

```sh
cargo build
```

Available configuration options are:

 * **native**: This feature-selector enables compilation of modules and
               examples that require native UEFI targets. Those will not
               compile on foreign targets and thus are guarded by this flag.

##### Build via: official toolchains

Starting with rust-version 1.68, rustup distributes pre-compiled toolchains for
many UEFI targets. You can enumerate and install them via `rustup`. This
example shows how to enumerate all available targets for your stable toolchain
and then install the UEFI target for the `x86_64` architecture:

```sh
rustup target list --toolchain=stable
rustup target add --toolchain=stable x86_64-unknown-uefi
```

This project can then be compiled directly for the selected target:

```sh
cargo +stable build \
    --examples \
    --features native \
    --lib \
    --target x86_64-unknown-uefi
```

##### Build via: cargo/rustc nightly with -Zbuild-std

If no pre-compiled toolchains are available for your selected target, you can
compile the project and the required parts of the standard library via the
experimental `-Zbuild-std` feature of rustc. This requires a nightly compiler:

```sh
cargo +nightly build \
    -Zbuild-std=core,compiler_builtins,alloc \
    -Zbuild-std-features=compiler-builtins-mem \
    --examples \
    --features native \
    --lib \
    --target x86_64-unknown-uefi
```

##### Build via: foreign target

The project can be built for non-UEFI targets via the standard rust toolchains.
This allows non-UEFI targets to interact with UEFI systems or otherwise host
UEFI operations. Furthermore, this allows running the foreign test-suite of
this project as long as the target supports the full standard library:

```sh
cargo +stable build --all-targets
cargo +stable test --all-targets
```

Note that the `native` feature must not be enabled for foreign targets as it
will not compile on non-UEFI systems.

### Repository:

 - **web**:   <https://github.com/r-efi/r-efi>
 - **https**: `https://github.com/r-efi/r-efi.git`
 - **ssh**:   `git@github.com:r-efi/r-efi.git`

### License:

 - **MIT** OR **Apache-2.0** OR **LGPL-2.1-or-later**
 - See AUTHORS file for details.
