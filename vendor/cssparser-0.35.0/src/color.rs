/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! General color-parsing utilities, independent on the specific color storage and parsing
//! implementation.
//!
//! For a more complete css-color implementation take a look at cssparser-color crate, or at
//! Gecko's color module.

// Allow text like <color> in docs.
#![allow(rustdoc::invalid_html_tags)]

/// The opaque alpha value of 1.0.
pub const OPAQUE: f32 = 1.0;

use crate::{BasicParseError, Parser, ToCss, Token};
use std::fmt;

/// Clamp a 0..1 number to a 0..255 range to u8.
///
/// Whilst scaling by 256 and flooring would provide
/// an equal distribution of integers to percentage inputs,
/// this is not what Gecko does so we instead multiply by 255
/// and round (adding 0.5 and flooring is equivalent to rounding)
///
/// Chrome does something similar for the alpha value, but not
/// the rgb values.
///
/// See <https://bugzilla.mozilla.org/show_bug.cgi?id=1340484>
///
/// Clamping to 256 and rounding after would let 1.0 map to 256, and
/// `256.0_f32 as u8` is undefined behavior:
///
/// <https://github.com/rust-lang/rust/issues/10184>
#[inline]
pub fn clamp_unit_f32(val: f32) -> u8 {
    clamp_floor_256_f32(val * 255.)
}

/// Round and clamp a single number to a u8.
#[inline]
pub fn clamp_floor_256_f32(val: f32) -> u8 {
    val.round().clamp(0., 255.) as u8
}

/// Serialize the alpha copmonent of a color according to the specification.
/// <https://drafts.csswg.org/css-color-4/#serializing-alpha-values>
#[inline]
pub fn serialize_color_alpha(
    dest: &mut impl fmt::Write,
    alpha: Option<f32>,
    legacy_syntax: bool,
) -> fmt::Result {
    let alpha = match alpha {
        None => return dest.write_str(" / none"),
        Some(a) => a,
    };

    // If the alpha component is full opaque, don't emit the alpha value in CSS.
    if alpha == OPAQUE {
        return Ok(());
    }

    dest.write_str(if legacy_syntax { ", " } else { " / " })?;

    // Try first with two decimal places, then with three.
    let mut rounded_alpha = (alpha * 100.).round() / 100.;
    if clamp_unit_f32(rounded_alpha) != clamp_unit_f32(alpha) {
        rounded_alpha = (alpha * 1000.).round() / 1000.;
    }

    rounded_alpha.to_css(dest)
}

/// A Predefined color space specified in:
/// <https://drafts.csswg.org/css-color-4/#predefined>
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PredefinedColorSpace {
    /// <https://drafts.csswg.org/css-color-4/#predefined-sRGB>
    Srgb,
    /// <https://drafts.csswg.org/css-color-4/#predefined-sRGB-linear>
    SrgbLinear,
    /// <https://drafts.csswg.org/css-color-4/#predefined-display-p3>
    DisplayP3,
    /// <https://drafts.csswg.org/css-color-4/#predefined-a98-rgb>
    A98Rgb,
    /// <https://drafts.csswg.org/css-color-4/#predefined-prophoto-rgb>
    ProphotoRgb,
    /// <https://drafts.csswg.org/css-color-4/#predefined-rec2020>
    Rec2020,
    /// <https://drafts.csswg.org/css-color-4/#predefined-xyz>
    XyzD50,
    /// <https://drafts.csswg.org/css-color-4/#predefined-xyz>
    XyzD65,
}

impl PredefinedColorSpace {
    /// Parse a PredefinedColorSpace from the given input.
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, BasicParseError<'i>> {
        let location = input.current_source_location();

        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "srgb" => Self::Srgb,
            "srgb-linear" => Self::SrgbLinear,
            "display-p3" => Self::DisplayP3,
            "a98-rgb" => Self::A98Rgb,
            "prophoto-rgb" => Self::ProphotoRgb,
            "rec2020" => Self::Rec2020,
            "xyz-d50" => Self::XyzD50,
            "xyz" | "xyz-d65" => Self::XyzD65,
            _ => return Err(location.new_basic_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for PredefinedColorSpace {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(match self {
            Self::Srgb => "srgb",
            Self::SrgbLinear => "srgb-linear",
            Self::DisplayP3 => "display-p3",
            Self::A98Rgb => "a98-rgb",
            Self::ProphotoRgb => "prophoto-rgb",
            Self::Rec2020 => "rec2020",
            Self::XyzD50 => "xyz-d50",
            Self::XyzD65 => "xyz-d65",
        })
    }
}

/// Parse a color hash, without the leading '#' character.
#[allow(clippy::result_unit_err)]
#[inline]
pub fn parse_hash_color(value: &[u8]) -> Result<(u8, u8, u8, f32), ()> {
    Ok(match value.len() {
        8 => (
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            (from_hex(value[6])? * 16 + from_hex(value[7])?) as f32 / 255.0,
        ),
        6 => (
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            OPAQUE,
        ),
        4 => (
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
            (from_hex(value[3])? * 17) as f32 / 255.0,
        ),
        3 => (
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
            OPAQUE,
        ),
        _ => return Err(()),
    })
}

ascii_case_insensitive_phf_map! {
    named_colors -> (u8, u8, u8) = {
        "black" => (0, 0, 0),
        "silver" => (192, 192, 192),
        "gray" => (128, 128, 128),
        "white" => (255, 255, 255),
        "maroon" => (128, 0, 0),
        "red" => (255, 0, 0),
        "purple" => (128, 0, 128),
        "fuchsia" => (255, 0, 255),
        "green" => (0, 128, 0),
        "lime" => (0, 255, 0),
        "olive" => (128, 128, 0),
        "yellow" => (255, 255, 0),
        "navy" => (0, 0, 128),
        "blue" => (0, 0, 255),
        "teal" => (0, 128, 128),
        "aqua" => (0, 255, 255),

        "aliceblue" => (240, 248, 255),
        "antiquewhite" => (250, 235, 215),
        "aquamarine" => (127, 255, 212),
        "azure" => (240, 255, 255),
        "beige" => (245, 245, 220),
        "bisque" => (255, 228, 196),
        "blanchedalmond" => (255, 235, 205),
        "blueviolet" => (138, 43, 226),
        "brown" => (165, 42, 42),
        "burlywood" => (222, 184, 135),
        "cadetblue" => (95, 158, 160),
        "chartreuse" => (127, 255, 0),
        "chocolate" => (210, 105, 30),
        "coral" => (255, 127, 80),
        "cornflowerblue" => (100, 149, 237),
        "cornsilk" => (255, 248, 220),
        "crimson" => (220, 20, 60),
        "cyan" => (0, 255, 255),
        "darkblue" => (0, 0, 139),
        "darkcyan" => (0, 139, 139),
        "darkgoldenrod" => (184, 134, 11),
        "darkgray" => (169, 169, 169),
        "darkgreen" => (0, 100, 0),
        "darkgrey" => (169, 169, 169),
        "darkkhaki" => (189, 183, 107),
        "darkmagenta" => (139, 0, 139),
        "darkolivegreen" => (85, 107, 47),
        "darkorange" => (255, 140, 0),
        "darkorchid" => (153, 50, 204),
        "darkred" => (139, 0, 0),
        "darksalmon" => (233, 150, 122),
        "darkseagreen" => (143, 188, 143),
        "darkslateblue" => (72, 61, 139),
        "darkslategray" => (47, 79, 79),
        "darkslategrey" => (47, 79, 79),
        "darkturquoise" => (0, 206, 209),
        "darkviolet" => (148, 0, 211),
        "deeppink" => (255, 20, 147),
        "deepskyblue" => (0, 191, 255),
        "dimgray" => (105, 105, 105),
        "dimgrey" => (105, 105, 105),
        "dodgerblue" => (30, 144, 255),
        "firebrick" => (178, 34, 34),
        "floralwhite" => (255, 250, 240),
        "forestgreen" => (34, 139, 34),
        "gainsboro" => (220, 220, 220),
        "ghostwhite" => (248, 248, 255),
        "gold" => (255, 215, 0),
        "goldenrod" => (218, 165, 32),
        "greenyellow" => (173, 255, 47),
        "grey" => (128, 128, 128),
        "honeydew" => (240, 255, 240),
        "hotpink" => (255, 105, 180),
        "indianred" => (205, 92, 92),
        "indigo" => (75, 0, 130),
        "ivory" => (255, 255, 240),
        "khaki" => (240, 230, 140),
        "lavender" => (230, 230, 250),
        "lavenderblush" => (255, 240, 245),
        "lawngreen" => (124, 252, 0),
        "lemonchiffon" => (255, 250, 205),
        "lightblue" => (173, 216, 230),
        "lightcoral" => (240, 128, 128),
        "lightcyan" => (224, 255, 255),
        "lightgoldenrodyellow" => (250, 250, 210),
        "lightgray" => (211, 211, 211),
        "lightgreen" => (144, 238, 144),
        "lightgrey" => (211, 211, 211),
        "lightpink" => (255, 182, 193),
        "lightsalmon" => (255, 160, 122),
        "lightseagreen" => (32, 178, 170),
        "lightskyblue" => (135, 206, 250),
        "lightslategray" => (119, 136, 153),
        "lightslategrey" => (119, 136, 153),
        "lightsteelblue" => (176, 196, 222),
        "lightyellow" => (255, 255, 224),
        "limegreen" => (50, 205, 50),
        "linen" => (250, 240, 230),
        "magenta" => (255, 0, 255),
        "mediumaquamarine" => (102, 205, 170),
        "mediumblue" => (0, 0, 205),
        "mediumorchid" => (186, 85, 211),
        "mediumpurple" => (147, 112, 219),
        "mediumseagreen" => (60, 179, 113),
        "mediumslateblue" => (123, 104, 238),
        "mediumspringgreen" => (0, 250, 154),
        "mediumturquoise" => (72, 209, 204),
        "mediumvioletred" => (199, 21, 133),
        "midnightblue" => (25, 25, 112),
        "mintcream" => (245, 255, 250),
        "mistyrose" => (255, 228, 225),
        "moccasin" => (255, 228, 181),
        "navajowhite" => (255, 222, 173),
        "oldlace" => (253, 245, 230),
        "olivedrab" => (107, 142, 35),
        "orange" => (255, 165, 0),
        "orangered" => (255, 69, 0),
        "orchid" => (218, 112, 214),
        "palegoldenrod" => (238, 232, 170),
        "palegreen" => (152, 251, 152),
        "paleturquoise" => (175, 238, 238),
        "palevioletred" => (219, 112, 147),
        "papayawhip" => (255, 239, 213),
        "peachpuff" => (255, 218, 185),
        "peru" => (205, 133, 63),
        "pink" => (255, 192, 203),
        "plum" => (221, 160, 221),
        "powderblue" => (176, 224, 230),
        "rebeccapurple" => (102, 51, 153),
        "rosybrown" => (188, 143, 143),
        "royalblue" => (65, 105, 225),
        "saddlebrown" => (139, 69, 19),
        "salmon" => (250, 128, 114),
        "sandybrown" => (244, 164, 96),
        "seagreen" => (46, 139, 87),
        "seashell" => (255, 245, 238),
        "sienna" => (160, 82, 45),
        "skyblue" => (135, 206, 235),
        "slateblue" => (106, 90, 205),
        "slategray" => (112, 128, 144),
        "slategrey" => (112, 128, 144),
        "snow" => (255, 250, 250),
        "springgreen" => (0, 255, 127),
        "steelblue" => (70, 130, 180),
        "tan" => (210, 180, 140),
        "thistle" => (216, 191, 216),
        "tomato" => (255, 99, 71),
        "turquoise" => (64, 224, 208),
        "violet" => (238, 130, 238),
        "wheat" => (245, 222, 179),
        "whitesmoke" => (245, 245, 245),
        "yellowgreen" => (154, 205, 50),
    }
}

/// Returns the named color with the given name.
/// <https://drafts.csswg.org/css-color-4/#typedef-named-color>
#[allow(clippy::result_unit_err)]
#[inline]
pub fn parse_named_color(ident: &str) -> Result<(u8, u8, u8), ()> {
    named_colors::get(ident).copied().ok_or(())
}

/// Returns an iterator over all named CSS colors.
/// <https://drafts.csswg.org/css-color-4/#typedef-named-color>
#[inline]
pub fn all_named_colors() -> impl Iterator<Item = (&'static str, (u8, u8, u8))> {
    named_colors::entries().map(|(k, v)| (*k, *v))
}

#[inline]
fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}
