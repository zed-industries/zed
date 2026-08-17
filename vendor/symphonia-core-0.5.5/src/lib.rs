// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![forbid(unsafe_code)]
// The following lints are allowed in all Symphonia crates. Please see clippy.toml for their
// justification.
#![allow(clippy::comparison_chain)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]

pub mod audio;
pub mod checksum;
pub mod codecs;
pub mod conv;
pub mod dsp;
pub mod errors;
pub mod formats;
pub mod io;
pub mod meta;
pub mod probe;
pub mod sample;
pub mod units;
pub mod util;
