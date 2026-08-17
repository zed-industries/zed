// Copyright 2024 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Error;
use crate::stream::{ByteExt, Stream};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Display;

/// Parses a list of font families and generic families from a string.
pub fn parse_font_families(text: &str) -> Result<Vec<FontFamily>, Error> {
    let mut s = Stream::from(text);
    let font_families = s.parse_font_families()?;

    s.skip_spaces();
    if !s.at_end() {
        return Err(Error::UnexpectedData(s.calc_char_pos()));
    }

    Ok(font_families)
}

/// A type of font family.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum FontFamily {
    /// A serif font.
    Serif,
    /// A sans-serif font.
    SansSerif,
    /// A cursive font.
    Cursive,
    /// A fantasy font.
    Fantasy,
    /// A monospace font.
    Monospace,
    /// A custom named font.
    Named(String),
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = match self {
            Self::Monospace => "monospace".to_string(),
            Self::Serif => "serif".to_string(),
            Self::SansSerif => "sans-serif".to_string(),
            Self::Cursive => "cursive".to_string(),
            Self::Fantasy => "fantasy".to_string(),
            Self::Named(s) => format!("\"{s}\""),
        };
        write!(f, "{str}")
    }
}

impl Stream<'_> {
    pub fn parse_font_families(&mut self) -> Result<Vec<FontFamily>, Error> {
        let mut families = vec![];

        while !self.at_end() {
            self.skip_spaces();

            let family = {
                let ch = self.curr_byte()?;
                if ch == b'\'' || ch == b'\"' {
                    let res = self.parse_quoted_string()?;
                    FontFamily::Named(res.to_string())
                } else {
                    let mut idents = vec![];

                    while let Some(c) = self.chars().next() {
                        if c != ',' {
                            idents.push(self.parse_ident()?.to_string());
                            self.skip_spaces();
                        } else {
                            break;
                        }
                    }

                    let joined = idents.join(" ");

                    // TODO: No CSS keyword must be matched as a family name...
                    match joined.as_str() {
                        "serif" => FontFamily::Serif,
                        "sans-serif" => FontFamily::SansSerif,
                        "cursive" => FontFamily::Cursive,
                        "fantasy" => FontFamily::Fantasy,
                        "monospace" => FontFamily::Monospace,
                        _ => FontFamily::Named(joined),
                    }
                }
            };

            families.push(family);

            if let Ok(b) = self.curr_byte() {
                if b == b',' {
                    self.advance(1);
                } else {
                    break;
                }
            }
        }

        let families = families
            .into_iter()
            .filter(|f| match f {
                FontFamily::Named(s) => !s.is_empty(),
                _ => true,
            })
            .collect();

        Ok(families)
    }
}

/// The values of a [`font` shorthand](https://www.w3.org/TR/css-fonts-3/#font-prop).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FontShorthand<'a> {
    /// The font style.
    pub font_style: Option<&'a str>,
    /// The font variant.
    pub font_variant: Option<&'a str>,
    /// The font weight.
    pub font_weight: Option<&'a str>,
    /// The font stretch.
    pub font_stretch: Option<&'a str>,
    /// The font size.
    pub font_size: &'a str,
    /// The font family.
    pub font_family: &'a str,
}

impl<'a> FontShorthand<'a> {
    /// Parses the `font` shorthand from a string.
    ///
    /// We can't use the `FromStr` trait because it requires
    /// an owned value as a return type.
    ///
    /// [font]: https://www.w3.org/TR/css-fonts-3/#font-prop
    #[allow(clippy::should_implement_trait)] // We aren't changing public API yet.
    pub fn from_str(text: &'a str) -> Result<Self, Error> {
        let mut stream = Stream::from(text);
        stream.skip_spaces();

        let mut prev_pos = stream.pos();

        let mut font_style = None;
        let mut font_variant = None;
        let mut font_weight = None;
        let mut font_stretch = None;

        for _ in 0..4 {
            let ident = stream.consume_ascii_ident();

            match ident {
                // TODO: Reuse actual parsers to prevent duplication.
                // We ignore normal because it's ambiguous to which it belongs and all
                // other attributes need to be reset anyway.
                "normal" => {}
                "small-caps" => font_variant = Some(ident),
                "italic" | "oblique" => font_style = Some(ident),
                "bold" | "bolder" | "lighter" | "100" | "200" | "300" | "400" | "500" | "600"
                | "700" | "800" | "900" => font_weight = Some(ident),
                "ultra-condensed" | "extra-condensed" | "condensed" | "semi-condensed"
                | "semi-expanded" | "expanded" | "extra-expanded" | "ultra-expanded" => {
                    font_stretch = Some(ident);
                }
                _ => {
                    // Not one of the 4 properties, so we backtrack and then start
                    // passing font size and family.
                    stream = Stream::from(text);
                    stream.advance(prev_pos);
                    break;
                }
            }

            stream.skip_spaces();
            prev_pos = stream.pos();
        }

        prev_pos = stream.pos();
        if stream.curr_byte()?.is_digit() {
            // A font size such as '15pt'.
            let _ = stream.parse_length()?;
        } else {
            // A font size like 'xx-large'.
            let size = stream.consume_ascii_ident();

            if !matches!(
                size,
                "xx-small"
                    | "x-small"
                    | "small"
                    | "medium"
                    | "large"
                    | "x-large"
                    | "xx-large"
                    | "larger"
                    | "smaller"
            ) {
                return Err(Error::UnexpectedData(prev_pos));
            }
        }

        let font_size = stream.slice_back(prev_pos);
        stream.skip_spaces();

        if stream.curr_byte()? == b'/' {
            // We should ignore line height since it has no effect in SVG.
            stream.advance(1);
            stream.skip_spaces();
            let _ = stream.parse_length()?;
            stream.skip_spaces();
        }

        if stream.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        let font_family = stream.slice_tail();

        Ok(Self {
            font_style,
            font_variant,
            font_weight,
            font_stretch,
            font_size,
            font_family,
        })
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! font_family {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(parse_font_families($text).unwrap(), $result);
            }
        )
    }

    macro_rules! named {
        ($text:expr) => (
            FontFamily::Named($text.to_string())
        )
    }

    const SERIF: FontFamily = FontFamily::Serif;
    const SANS_SERIF: FontFamily = FontFamily::SansSerif;
    const FANTASY: FontFamily = FontFamily::Fantasy;
    const MONOSPACE: FontFamily = FontFamily::Monospace;
    const CURSIVE: FontFamily = FontFamily::Cursive;

    font_family!(font_family_1, "Times New Roman", vec![named!("Times New Roman")]);
    font_family!(font_family_2, "serif", vec![SERIF]);
    font_family!(font_family_3, "sans-serif", vec![SANS_SERIF]);
    font_family!(font_family_4, "cursive", vec![CURSIVE]);
    font_family!(font_family_5, "fantasy", vec![FANTASY]);
    font_family!(font_family_6, "monospace", vec![MONOSPACE]);
    font_family!(font_family_7, "'Times New Roman'", vec![named!("Times New Roman")]);
    font_family!(font_family_8, "'Times New Roman', sans-serif", vec![named!("Times New Roman"), SANS_SERIF]);
    font_family!(font_family_9, "'Times New Roman', sans-serif", vec![named!("Times New Roman"), SANS_SERIF]);
    font_family!(font_family_10, "Arial, sans-serif, 'fantasy'", vec![named!("Arial"), SANS_SERIF, named!("fantasy")]);
    font_family!(font_family_11, "    Arial  , monospace  , 'fantasy'", vec![named!("Arial"), MONOSPACE, named!("fantasy")]);
    font_family!(font_family_12, "Times    New Roman", vec![named!("Times New Roman")]);
    font_family!(font_family_13, "\"Times New Roman\", sans-serif, sans-serif, \"Arial\"",
        vec![named!("Times New Roman"), SANS_SERIF, SANS_SERIF, named!("Arial")]
    );
    font_family!(font_family_14, "Times New Roman,,,Arial", vec![named!("Times New Roman"), named!("Arial")]);
    font_family!(font_family_15, "简体中文,sans-serif  , ,\"日本語フォント\",Arial",
        vec![named!("简体中文"), SANS_SERIF, named!("日本語フォント"), named!("Arial")]);

    font_family!(font_family_16, "", vec![]);

    macro_rules! font_family_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(parse_font_families($text).unwrap_err().to_string(), $result);
            }
        )
    }
    font_family_err!(font_family_err_1, "Red/Black, sans-serif", "invalid ident");
    font_family_err!(font_family_err_2, "\"Lucida\" Grande, sans-serif", "unexpected data at position 10");
    font_family_err!(font_family_err_3, "Ahem!, sans-serif", "invalid ident");
    font_family_err!(font_family_err_4, "test@foo, sans-serif", "invalid ident");
    font_family_err!(font_family_err_5, "#POUND, sans-serif", "invalid ident");
    font_family_err!(font_family_err_6, "Hawaii 5-0, sans-serif", "invalid ident");

    impl<'a> FontShorthand<'a> {
        fn new(font_style: Option<&'a str>, font_variant: Option<&'a str>, font_weight: Option<&'a str>,
                   font_stretch: Option<&'a str>, font_size: &'a str, font_family: &'a str) -> Self {
            Self {
                font_style, font_variant, font_weight, font_stretch, font_size, font_family
            }
        }
    }

    macro_rules! font_shorthand {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(FontShorthand::from_str($text).unwrap(), $result);
            }
        )
    }

    font_shorthand!(font_shorthand_1, "12pt/14pt sans-serif",
        FontShorthand::new(None, None, None, None, "12pt", "sans-serif"));
    font_shorthand!(font_shorthand_2, "80% sans-serif",
        FontShorthand::new(None, None, None, None, "80%", "sans-serif"));
    font_shorthand!(font_shorthand_3, "bold italic large Palatino, serif",
        FontShorthand::new(Some("italic"), None, Some("bold"), None, "large", "Palatino, serif"));
    font_shorthand!(font_shorthand_4, "x-large/110% \"new century schoolbook\", serif",
        FontShorthand::new(None, None, None, None, "x-large", "\"new century schoolbook\", serif"));
    font_shorthand!(font_shorthand_5, "normal small-caps 120%/120% fantasy",
        FontShorthand::new(None, Some("small-caps"), None, None, "120%", "fantasy"));
    font_shorthand!(font_shorthand_6, "condensed oblique 12pt \"Helvetica Neue\", serif",
        FontShorthand::new(Some("oblique"), None, None, Some("condensed"), "12pt", "\"Helvetica Neue\", serif"));
    font_shorthand!(font_shorthand_7, "italic 500 2em sans-serif, 'Noto Sans'",
        FontShorthand::new(Some("italic"), None, Some("500"), None, "2em", "sans-serif, 'Noto Sans'"));
    font_shorthand!(font_shorthand_8, "xx-large 'Noto Sans'",
        FontShorthand::new(None, None, None, None, "xx-large", "'Noto Sans'"));
    font_shorthand!(font_shorthand_9, "small-caps normal normal italic xx-small Times",
        FontShorthand::new(Some("italic"), Some("small-caps"), None, None, "xx-small", "Times"));


    macro_rules! font_shorthand_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(FontShorthand::from_str($text).unwrap_err(), $result);
            }
        )
    }

    font_shorthand_err!(font_shorthand_err_1, "", Error::UnexpectedEndOfStream);
    font_shorthand_err!(font_shorthand_err_2, "Noto Sans", Error::UnexpectedData(0));
    font_shorthand_err!(font_shorthand_err_3, "12pt  ", Error::UnexpectedEndOfStream);
    font_shorthand_err!(font_shorthand_err_4, "something 12pt 'Noto Sans'", Error::UnexpectedData(0));
    font_shorthand_err!(font_shorthand_err_5, "'Noto Sans' 13pt", Error::UnexpectedData(0));
    font_shorthand_err!(font_shorthand_err_6,
        "small-caps normal normal normal italic xx-large Times", Error::UnexpectedData(32));
}
