use std::str::FromStr;

use crate::{Color, Error, Stream};

/// Representation of the fallback part of the [`<paint>`] type.
///
/// Used by the [`Paint`](enum.Paint.html) type.
///
/// [`<paint>`]: https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaintFallback {
    /// The `none` value.
    None,
    /// The `currentColor` value.
    CurrentColor,
    /// [`<color>`] value.
    ///
    /// [`<color>`]: https://www.w3.org/TR/css-color-3/
    Color(Color),
}

/// Representation of the [`<paint>`] type.
///
/// Doesn't own the data. Use only for parsing.
///
/// `<icccolor>` isn't supported.
///
/// [`<paint>`]: https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint
///
/// # Examples
///
/// ```
/// use svgtypes::{Paint, PaintFallback, Color};
///
/// let paint = Paint::from_str("url(#gradient) red").unwrap();
/// assert_eq!(paint, Paint::FuncIRI("gradient",
///                                  Some(PaintFallback::Color(Color::red()))));
///
/// let paint = Paint::from_str("inherit").unwrap();
/// assert_eq!(paint, Paint::Inherit);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Paint<'a> {
    /// The `none` value.
    None,
    /// The `inherit` value.
    Inherit,
    /// The `currentColor` value.
    CurrentColor,
    /// [`<color>`] value.
    ///
    /// [`<color>`]: https://www.w3.org/TR/css-color-3/
    Color(Color),
    /// [`<FuncIRI>`] value with an optional fallback.
    ///
    /// [`<FuncIRI>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeFuncIRI
    FuncIRI(&'a str, Option<PaintFallback>),
}

impl<'a> Paint<'a> {
    /// Parses a `Paint` from a string.
    ///
    /// We can't use the `FromStr` trait because it requires
    /// an owned value as a return type.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &'a str) -> Result<Self, Error> {
        let text = text.trim();
        match text {
            "none" => Ok(Paint::None),
            "inherit" => Ok(Paint::Inherit),
            "currentColor" => Ok(Paint::CurrentColor),
            _ => {
                let mut s = Stream::from(text);
                if s.starts_with(b"url(") {
                    match s.parse_func_iri() {
                        Ok(link) => {
                            s.skip_spaces();

                            // get fallback
                            if !s.at_end() {
                                let fallback = s.slice_tail();
                                match fallback {
                                    "none" => Ok(Paint::FuncIRI(link, Some(PaintFallback::None))),
                                    "currentColor" => {
                                        Ok(Paint::FuncIRI(link, Some(PaintFallback::CurrentColor)))
                                    }
                                    _ => {
                                        let color = Color::from_str(fallback)?;
                                        Ok(Paint::FuncIRI(link, Some(PaintFallback::Color(color))))
                                    }
                                }
                            } else {
                                Ok(Paint::FuncIRI(link, None))
                            }
                        }
                        Err(_) => Err(Error::InvalidValue),
                    }
                } else {
                    match Color::from_str(text) {
                        Ok(c) => Ok(Paint::Color(c)),
                        Err(_) => Err(Error::InvalidValue),
                    }
                }
            }
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Paint::from_str($text).unwrap(), $result);
            }
        )
    }

    test!(parse_1, "none", Paint::None);
    test!(parse_2, "  none   ", Paint::None);
    test!(parse_3, " inherit ", Paint::Inherit);
    test!(parse_4, " currentColor ", Paint::CurrentColor);
    test!(parse_5, " red ", Paint::Color(Color::red()));
    test!(parse_6, " url(#qwe) ", Paint::FuncIRI("qwe", None));
    test!(parse_7, " url(#qwe) none ", Paint::FuncIRI("qwe", Some(PaintFallback::None)));
    test!(parse_8, " url(#qwe) currentColor ", Paint::FuncIRI("qwe", Some(PaintFallback::CurrentColor)));
    test!(parse_9, " url(#qwe) red ", Paint::FuncIRI("qwe", Some(PaintFallback::Color(Color::red()))));

    macro_rules! test_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Paint::from_str($text).unwrap_err().to_string(), $result);
            }
        )
    }

    test_err!(parse_err_1, "qwe", "invalid value");
    test_err!(parse_err_2, "red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)", "invalid value");
    // TODO: this
//    test_err!(parse_err_3, "url(#qwe) red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)", "invalid color at 1:15");
}
