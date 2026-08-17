# Audio playback library

[![Crates.io Version](https://img.shields.io/crates/v/rodio.svg)](https://crates.io/crates/rodio)
[![Crates.io Downloads](https://img.shields.io/crates/d/rodio.svg)](https://crates.io/crates/rodio)
[![Build Status](https://github.com/RustAudio/rodio/workflows/CI/badge.svg)](https://github.com/RustAudio/rodio/actions)

Rust playback library.

Playback is handled by [cpal](https://github.com/RustAudio/cpal). Format decoding is handled by [Symphonia](https://github.com/pdeljanov/Symphonia) by default, or by optional format-specific decoders:

 - FLAC by [claxon](https://github.com/ruuda/claxon).
 - MP3 by [minimp3](https://github.com/lieff/minimp3).
 - Vorbis by [lewton](https://github.com/est31/lewton).
 - WAV by [hound](https://github.com/ruud-v-a/hound).

See [the feature flags](https://docs.rs/crate/rodio/latest/features) for more details on formats, and other features.

# [Documentation](http://docs.rs/rodio)

[The documentation](http://docs.rs/rodio) contains an introduction to the library.

# [Breaking Changes](UPGRADE.md)
We have written an upgrade [guide](UPGRADE.md). It will help you upgrade to rodio 0.21.

# [Examples](https://github.com/RustAudio/rodio/tree/f7aa48daa08a303e247d68a1dad0dbcc915ca135/examples)

We are always making changes to rodio. This can mean that the examples do not work with the current crates.io release. You will have to look at the examples from commit `f7aa48d`. They are available [on github](https://github.com/RustAudio/rodio/tree/f7aa48daa08a303e247d68a1dad0dbcc915ca135/examples).

## Requirements

Rodio playback works in environments supported by [cpal](https://github.com/RustAudio/cpal) library.

The CPU of the target system should have hardware support for 32-bit floating point (`f32`), and atomic operations that are at least 32 bit wide. Without these the CPU may not be fast enough to keep up with real-time.

### Minimum Supported Rust Version (MSRV)

Rodio requires Rust 1.87 or later.

## Dependencies (Linux only)

Rodio uses `cpal` library to send audio to the OS for playback. ALSA development files are needed to build `cpal` on Linux. These are provided as part of the `libasound2-dev` package on Debian and Ubuntu distributions and `alsa-lib-devel` on Fedora.

### Minimal build

It is possible to build `rodio` without support for audio playback. In this configuration `cpal` dependency and its requirements are excluded. This configuration may be useful, for example, for decoding and processing audio in environments when the audio output is not available (e.g. in case of Linux, when ALSA is not available). See `into_file` example that works with this build.

In order to use `rodio` in this configuration disable default features and add the necessary ones. In this case the `Cargo.toml` dependency would look like:
```toml
[dependencies]
rodio = { version = "0.21.0", default-features = false, features = ["symphonia-all"] }
```
### Cross compling aarch64/arm

Through cpal rodio depends on the alsa library (libasound & libasound-dev), this can make crosscompiling hard. Cpal has some guides on crosscompling in their Readme (https://github.com/RustAudio/cpal). They are missing instructions on aarch64 (arm linux) so we have some here:

#### aarch64/arm on Debian like (Ubuntu/pop)
- Install crossbuild-essential-arm64: `sudo apt-get install crossbuild-essential-arm64 clang`
- Add the aarch64 target for rust: `rustup target add aarch64-unknown-linux-gnu`
- Add the architecture arm64 to apt using: `sudo dpkg --add-architecture arm64`
- Install the [multi-arch](https://wiki.debian.org/Multiarch/HOWTO) version of libasound2-dev for arm64 using: `sudo apt install libasound2-dev:arm64`
- Build with the pkg config sysroot set to /usr/aarch64-linux-gnu and aarch64-linux-gnu as linker: `PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --target aarch64-unknown-linux-gnu`

This will work for other Linux targets too if you change the architecture in the command and if there are multi-arch packages available.

You might want to look at [cross](https://github.com/cross-rs/cross) if you are running on a non debian system or want to make this more repeatable.

# Contributing

We love it when people join making rodio better. There are many ways to help not matter how experienced you are with audio or Rust. We would especially love help with:

- Reviewing PR's: we rely on the community to keep rodio correct, fast and maintainable. Absolutely anyone is welcome to leave reviews. Anyone can catch missing or unnecessary tests, unclear docs logic mistakes etc.
- Improving the docs: You only learn something for the first time once. Many of us forgot what was hard when things where new. Your experience using rodio for the first time can really help out here. You probably know best what needs clearification and how it should be written down. It can be anything, maybe a  paragraph on audio theory or an extra example.
- Adding features: we love it when we get new features or existing onces get even better. Let us know what you are working on early. Not only can the rest of the community help out with the design we can help make the review and merge smooth. Please see our [Contributing Guide](CONTRIBUTING.md) for more to get you started.

## License
[License]: #license

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0), or
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
