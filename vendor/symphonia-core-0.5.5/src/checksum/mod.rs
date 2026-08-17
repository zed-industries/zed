// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `checksum` module provides implementations of common error-detecting codes and hashing
//! algorithms.

mod crc16;
mod crc32;
mod crc8;
mod md5;

pub use crc16::{Crc16Ansi, Crc16AnsiLe};
pub use crc32::Crc32;
pub use crc8::Crc8Ccitt;
pub use md5::Md5;
