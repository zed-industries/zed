// font-kit/src/file_type.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The type of a font file: either a single font or a TrueType/OpenType collection.

/// The type of a font file: either a single font or a TrueType/OpenType collection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileType {
    /// The font file represents a single font (`.ttf`, `.otf`, `.woff`, etc.)
    Single,
    /// The font file represents a collection of fonts (`.ttc`, `.otc`, etc.)
    Collection(u32),
}
