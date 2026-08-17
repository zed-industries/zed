[![ci](https://github.com/rust-mobile/ndk/actions/workflows/rust.yml/badge.svg)](https://github.com/rust-mobile/ndk/actions/workflows/rust.yml) ![MIT license](https://img.shields.io/badge/License-MIT-green.svg) ![APACHE2 license](https://img.shields.io/badge/License-APACHE2-green.svg)

Rust bindings to the [Android NDK](https://developer.android.com/ndk)

Name | Description | Badges
--- | --- | ---
[`ndk-sys`](./ndk-sys) | Raw FFI bindings to the NDK | [![crates.io](https://img.shields.io/crates/v/ndk-sys.svg)](https://crates.io/crates/ndk-sys) [![Docs](https://docs.rs/ndk-sys/badge.svg)](https://docs.rs/ndk-sys) [![MSRV](https://img.shields.io/badge/rustc-1.60.0+-ab6000.svg)](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html)
[`ndk`](./ndk) | Safe abstraction of the bindings | [![crates.io](https://img.shields.io/crates/v/ndk.svg)](https://crates.io/crates/ndk) [![Docs](https://docs.rs/ndk/badge.svg)](https://docs.rs/ndk) [![MSRV](https://img.shields.io/badge/rustc-1.64.0+-ab6000.svg)](https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html)

See these [`ndk-examples`](https://github.com/rust-mobile/cargo-apk/tree/main/examples/examples) and these [`rust-android-examples`](https://github.com/rust-mobile/rust-android-examples) for examples using the NDK.

> [!IMPORTANT]
> This repository was recently [modularized](https://github.com/rust-mobile/ndk/issues/372) and the following crates were split into separate repositories:
>
> Crate | New Location | Notes
> ------|--------------|------
> ndk-context | https://github.com/rust-mobile/ndk-context |
> ndk-glue | https://github.com/rust-mobile/ndk-glue | ⛔ _deprecated_ - see [android-activity](https://github.com/rust-mobile/android-activity)
> ndk-macro | https://github.com/rust-mobile/ndk-glue | ⛔ _deprecated_ - see [android-activity](https://github.com/rust-mobile/android-activity)
> ndk-build | https://github.com/rust-mobile/cargo-apk | ⛔ _deprecated_ - see [xbuild](https://github.com/rust-mobile/xbuild)
> cargo-apk | https://github.com/rust-mobile/cargo-apk | ⛔ _deprecated_ - see [xbuild](https://github.com/rust-mobile/xbuild)
