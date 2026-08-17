// font-kit/src/properties.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Properties that specify which font in a family to use: e.g. style, weight, and stretchiness.
//!
//! Much of the documentation in this modules comes from the CSS 3 Fonts specification:
//! <https://drafts.csswg.org/css-fonts-3/>

use std::fmt::{self, Debug, Display, Formatter};

/// Properties that specify which font in a family to use: e.g. style, weight, and stretchiness.
///
/// This object supports a method chaining style for idiomatic initialization; e.g.
///
///     # use font_kit::properties::{Properties, Style};
///     println!("{:?}", Properties::new().style(Style::Italic));
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Properties {
    /// The font style, as defined in CSS.
    pub style: Style,
    /// The font weight, as defined in CSS.
    pub weight: Weight,
    /// The font stretchiness, as defined in CSS.
    pub stretch: Stretch,
}

impl Properties {
    /// Initializes a property set to its default values: normal style, normal weight, and normal
    /// stretchiness.
    #[inline]
    pub fn new() -> Properties {
        Properties::default()
    }

    /// Sets the value of the style property and returns this property set for method chaining.
    #[inline]
    pub fn style(&mut self, style: Style) -> &mut Properties {
        self.style = style;
        self
    }

    /// Sets the value of the weight property and returns this property set for method chaining.
    #[inline]
    pub fn weight(&mut self, weight: Weight) -> &mut Properties {
        self.weight = weight;
        self
    }

    /// Sets the value of the stretch property and returns this property set for method chaining.
    #[inline]
    pub fn stretch(&mut self, stretch: Stretch) -> &mut Properties {
        self.stretch = stretch;
        self
    }
}

/// Allows italic or oblique faces to be selected.
#[derive(Clone, Copy, PartialEq, Debug, Hash, Default)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    #[default]
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// The degree of blackness or stroke thickness of a font. This value ranges from 100.0 to 900.0,
/// with 400.0 as normal.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Weight(pub f32);

impl Default for Weight {
    #[inline]
    fn default() -> Weight {
        Weight::NORMAL
    }
}

impl Weight {
    /// Thin weight (100), the thinnest value.
    pub const THIN: Weight = Weight(100.0);
    /// Extra light weight (200).
    pub const EXTRA_LIGHT: Weight = Weight(200.0);
    /// Light weight (300).
    pub const LIGHT: Weight = Weight(300.0);
    /// Normal (400).
    pub const NORMAL: Weight = Weight(400.0);
    /// Medium weight (500, higher than normal).
    pub const MEDIUM: Weight = Weight(500.0);
    /// Semibold weight (600).
    pub const SEMIBOLD: Weight = Weight(600.0);
    /// Bold weight (700).
    pub const BOLD: Weight = Weight(700.0);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: Weight = Weight(800.0);
    /// Black weight (900), the thickest value.
    pub const BLACK: Weight = Weight(900.0);
}

/// The width of a font as an approximate fraction of the normal width.
///
/// Widths range from 0.5 to 2.0 inclusive, with 1.0 as the normal width.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Stretch(pub f32);

impl Default for Stretch {
    #[inline]
    fn default() -> Stretch {
        Stretch::NORMAL
    }
}

impl Stretch {
    /// Ultra-condensed width (50%), the narrowest possible.
    pub const ULTRA_CONDENSED: Stretch = Stretch(0.5);
    /// Extra-condensed width (62.5%).
    pub const EXTRA_CONDENSED: Stretch = Stretch(0.625);
    /// Condensed width (75%).
    pub const CONDENSED: Stretch = Stretch(0.75);
    /// Semi-condensed width (87.5%).
    pub const SEMI_CONDENSED: Stretch = Stretch(0.875);
    /// Normal width (100%).
    pub const NORMAL: Stretch = Stretch(1.0);
    /// Semi-expanded width (112.5%).
    pub const SEMI_EXPANDED: Stretch = Stretch(1.125);
    /// Expanded width (125%).
    pub const EXPANDED: Stretch = Stretch(1.25);
    /// Extra-expanded width (150%).
    pub const EXTRA_EXPANDED: Stretch = Stretch(1.5);
    /// Ultra-expanded width (200%), the widest possible.
    pub const ULTRA_EXPANDED: Stretch = Stretch(2.0);

    // Mapping from `usWidthClass` values to CSS `font-stretch` values.
    pub(crate) const MAPPING: [f32; 9] = [
        Stretch::ULTRA_CONDENSED.0,
        Stretch::EXTRA_CONDENSED.0,
        Stretch::CONDENSED.0,
        Stretch::SEMI_CONDENSED.0,
        Stretch::NORMAL.0,
        Stretch::SEMI_EXPANDED.0,
        Stretch::EXPANDED.0,
        Stretch::EXTRA_EXPANDED.0,
        Stretch::ULTRA_EXPANDED.0,
    ];
}
