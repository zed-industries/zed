# coreaudio-rs [![Actions Status](https://github.com/rustaudio/coreaudio-rs/workflows/coreaudio-rs/badge.svg)](https://github.com/rustaudio/coreaudio-rs/actions) [![Crates.io](https://img.shields.io/crates/v/coreaudio-rs.svg)](https://crates.io/crates/coreaudio-rs) [![Crates.io](https://img.shields.io/crates/l/coreaudio-rs.svg)](https://github.com/RustAudio/coreaudio-rs/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/coreaudio-rs/badge.svg)](https://docs.rs/coreaudio-rs/)

A friendly rust interface for [Apple's Core Audio API](https://developer.apple.com/library/ios/documentation/MusicAudio/Conceptual/CoreAudioOverview/CoreAudioEssentials/CoreAudioEssentials.html).

This crate aims to expose and wrap the functionality of the original C API in a zero-cost, safe, Rust-esque manner.

If you just want direct access to the unsafe bindings, use [coreaudio-sys](https://crates.io/crates/coreaudio-sys).
