// font-kit/src/descriptor.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A possible value for the `font-family` CSS property.

/// A possible value for the `font-family` CSS property.
///
/// These descriptions are taken from CSS Fonts Level 3 § 3.1:
/// <https://drafts.csswg.org/css-fonts-3/#font-family-prop>.
///
/// TODO(pcwalton): `system-ui`, `emoji`, `math`, `fangsong`
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum FamilyName {
    /// A specific font family, specified by name: e.g. "Arial", "times".
    Title(String),
    /// Serif fonts represent the formal text style for a script.
    Serif,
    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
    /// (vertical and horizontal stems have the close to the same thickness) and have stroke
    /// endings that are plain — without any flaring, cross stroke, or other ornamentation.
    SansSerif,
    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    Monospace,
    /// Glyphs in cursive fonts generally use a more informal script style, and the result looks
    /// more like handwritten pen or brush writing than printed letterwork.
    Cursive,
    /// Fantasy fonts are primarily decorative or expressive fonts that contain decorative or
    /// expressive representations of characters.
    Fantasy,
}
