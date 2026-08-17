// font-kit/src/loaders/mod.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The different system services that can load and rasterize fonts.

#[cfg(all(
    any(target_os = "macos", target_os = "ios"),
    not(feature = "loader-freetype-default")
))]
pub use crate::loaders::core_text as default;

#[cfg(all(target_family = "windows", not(feature = "loader-freetype-default")))]
pub use crate::loaders::directwrite as default;

#[cfg(any(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype-default"
))]
pub use crate::loaders::freetype as default;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod core_text;

#[cfg(target_family = "windows")]
pub mod directwrite;

#[cfg(any(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype"
))]
pub mod freetype;
