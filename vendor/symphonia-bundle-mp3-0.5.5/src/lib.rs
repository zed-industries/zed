// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
// The following lints are allowed in all Symphonia crates. Please see clippy.toml for their
// justification.
#![allow(clippy::comparison_chain)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]

// Shared modules.
mod common;
mod header;

// Demuxer module.
mod demuxer;

// Decoder modules.
#[cfg(any(feature = "mp1", feature = "mp2", feature = "mp3"))]
mod decoder;
#[cfg(any(feature = "mp1", feature = "mp2", feature = "mp3"))]
mod synthesis;

// Shared layer 1 & 2 decoder support module.
#[cfg(any(feature = "mp1", feature = "mp2"))]
mod layer12;

// Layer-specific decoder support modules.
#[cfg(feature = "mp1")]
mod layer1;
#[cfg(feature = "mp2")]
mod layer2;
#[cfg(feature = "mp3")]
mod layer3;

#[cfg(any(feature = "mp1", feature = "mp2", feature = "mp3"))]
pub use decoder::MpaDecoder;
pub use demuxer::MpaReader;

// For SemVer compatibility in v0.5.x series.
#[deprecated = "use `symphonia_bundle_mp3::MpaDecoder` instead"]
#[cfg(any(feature = "mp1", feature = "mp2", feature = "mp3"))]
pub type Mp3Decoder = MpaDecoder;

#[deprecated = "use `symphonia_bundle_mp3::MpaReader` instead"]
pub type Mp3Reader = MpaReader;
