use crate::{Angle, AngleUnit, Color, Error, Length, LengthUnit, Stream};

/// Representation of the [`<filter-function>`] | [`<url>`] type.
///
/// Note that [`Length`] values in this enum do not contain % values.
/// They are disallowed by the spec.
///
/// [`<filter-function>`]: https://www.w3.org/TR/filter-effects-1/#filter-functions
/// [`<url>`]: https://www.w3.org/TR/filter-effects-1/#typedef-filter-url
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum FilterValue<'a> {
    /// Cannot be negative and/or have a percentage units.
    Blur(Length),
    DropShadow {
        /// `currentColor` will be resolved as `None`,
        /// because it should be processed in the same way.
        color: Option<Color>,
        /// Cannot have a percentage units.
        dx: Length,
        /// Cannot have a percentage units.
        dy: Length,
        /// Cannot be negative and/or have a percentage units.
        std_dev: Length,
    },
    /// Normalized value. Cannot be negative.
    Brightness(f64),
    /// Normalized value. Cannot be negative.
    Contrast(f64),
    /// Normalized value. Cannot be negative.
    Grayscale(f64),
    HueRotate(Angle),
    /// Normalized value. Cannot be negative.
    Invert(f64),
    /// Normalized value. Cannot be negative.
    Opacity(f64),
    /// Normalized value. Cannot be negative.
    Sepia(f64),
    /// Normalized value. Cannot be negative.
    Saturate(f64),
    /// Cannot be empty.
    Url(&'a str),
}

/// A list of possible [`FilterValueListParser`] errors.
#[derive(Debug)]
pub enum FilterValueListParserError {
    /// Lengths with percentage values are not allowed.
    PercentageValue(usize),

    /// Some values cannot be negative.
    NegativeValue(usize),

    /// An invalid angle value.
    InvalidAngle(usize),

    /// Drop shadow offset values must be set.
    MissingDropShadowOffset(usize),

    /// Usually indicates an empty url.
    InvalidUrl(usize),

    /// Other errors.
    StreamErrors(Error),
}

impl From<Error> for FilterValueListParserError {
    fn from(e: Error) -> Self {
        FilterValueListParserError::StreamErrors(e)
    }
}

impl std::fmt::Display for FilterValueListParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FilterValueListParserError::PercentageValue(pos) => {
                write!(f, "a percentage value detected at position {}", pos)
            }
            FilterValueListParserError::NegativeValue(pos) => {
                write!(f, "a negative value detected at position {}", pos)
            }
            FilterValueListParserError::InvalidAngle(pos) => {
                write!(f, "an invalid angle at position {}", pos)
            }
            FilterValueListParserError::MissingDropShadowOffset(pos) => {
                write!(
                    f,
                    "drop-shadow offset values are expected at position {}",
                    pos
                )
            }
            FilterValueListParserError::InvalidUrl(pos) => {
                write!(f, "an invalid url at position {}", pos)
            }
            FilterValueListParserError::StreamErrors(ref e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for FilterValueListParserError {
    fn description(&self) -> &str {
        "filter-value-list parsing error"
    }
}

/// A pull-based [`<filter-value-list>`] parser.
///
/// When value is set to `none`, the parser will return `None` immediately.
/// Meaning that an empty string and `none` will produce the same results.
///
/// [`<filter-value-list>`]: https://www.w3.org/TR/filter-effects-1/#typedef-filter-value-list
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FilterValueListParser<'a> {
    stream: Stream<'a>,
}

impl<'a> From<&'a str> for FilterValueListParser<'a> {
    fn from(text: &'a str) -> Self {
        FilterValueListParser {
            stream: Stream::from(text),
        }
    }
}

impl<'a> Iterator for FilterValueListParser<'a> {
    type Item = Result<FilterValue<'a>, FilterValueListParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.skip_spaces();

        if self.stream.at_end() {
            // an empty attribute is still a valid value
            return None;
        }

        if self.stream.starts_with(b"none") {
            self.stream.advance(4);
            self.stream.skip_spaces();

            if self.stream.at_end() {
                return None;
            } else {
                self.stream.jump_to_end();
                return Some(Err(Error::InvalidValue.into()));
            }
        }

        let res = self.parse_next();
        if res.is_err() {
            self.stream.jump_to_end();
        }

        Some(res)
    }
}

impl<'a> FilterValueListParser<'a> {
    fn parse_next(&mut self) -> Result<FilterValue<'a>, FilterValueListParserError> {
        let s = &mut self.stream;

        let start = s.pos();
        let name = s.consume_ident();
        s.skip_spaces();
        s.consume_byte(b'(')?;
        s.skip_spaces();

        let value = match name.as_bytes() {
            b"blur" => {
                if s.is_curr_byte_eq(b')') {
                    FilterValue::Blur(Length::zero())
                } else {
                    let value = parse_filter_positive_length(s)?;
                    FilterValue::Blur(value)
                }
            }
            b"drop-shadow" => parse_drop_shadow_func(s)?,
            b"hue-rotate" => {
                if s.is_curr_byte_eq(b')') {
                    FilterValue::HueRotate(Angle::new(0.0, AngleUnit::Degrees))
                } else {
                    let value = parse_filter_angle(s)?;
                    FilterValue::HueRotate(value)
                }
            }
            b"brightness" => FilterValue::Brightness(parse_generic_color_func(s)?),
            b"contrast" => FilterValue::Contrast(parse_generic_color_func(s)?),
            b"grayscale" => FilterValue::Grayscale(parse_generic_color_func(s)?),
            b"invert" => FilterValue::Invert(parse_generic_color_func(s)?),
            b"opacity" => FilterValue::Opacity(parse_generic_color_func(s)?),
            b"saturate" => FilterValue::Saturate(parse_generic_color_func(s)?),
            b"sepia" => FilterValue::Sepia(parse_generic_color_func(s)?),
            b"url" => {
                s.consume_byte(b'#')?;
                let link = s.consume_bytes(|_, c| c != b' ' && c != b')');
                if !link.is_empty() {
                    FilterValue::Url(link)
                } else {
                    return Err(FilterValueListParserError::InvalidUrl(
                        s.calc_char_pos_at(start),
                    ));
                }
            }
            _ => {
                return Err(Error::UnexpectedData(s.calc_char_pos_at(start)).into());
            }
        };

        s.skip_spaces();
        s.consume_byte(b')')?;
        s.skip_spaces();

        Ok(value)
    }
}

#[inline(never)]
fn parse_drop_shadow_func<'a>(
    s: &mut Stream<'a>,
) -> Result<FilterValue<'a>, FilterValueListParserError> {
    if s.is_curr_byte_eq(b')') {
        let pos = s.calc_char_pos();
        return Err(FilterValueListParserError::MissingDropShadowOffset(pos));
    }

    // Color can be set before and after lengths.
    let mut color = None;
    let mut is_current_color = false;
    if let Some(c) = s.try_parse_color() {
        color = Some(c);
        s.skip_spaces();
    } else if s.starts_with(b"currentColor") {
        is_current_color = true;
        s.advance(12);
        s.skip_spaces();
    }

    // Offset is the only mandatory value.
    let dx = parse_filter_length(s)?;
    s.skip_spaces();
    let dy = parse_filter_length(s)?;
    s.skip_spaces();

    // std_dev is optional
    let mut std_dev = Length::zero();
    if let Ok(v) = parse_filter_positive_length(s) {
        std_dev = v;
        s.skip_spaces();
    }

    // Try to parse a color after length, if it wasn't set before.
    if color.is_none() && !is_current_color {
        if let Some(c) = s.try_parse_color() {
            color = Some(c);
            s.skip_spaces();
        } else if s.starts_with(b"currentColor") {
            s.advance(12);
        }
    }

    Ok(FilterValue::DropShadow {
        color,
        dx,
        dy,
        std_dev,
    })
}

#[inline(never)]
fn parse_generic_color_func(s: &mut Stream) -> Result<f64, FilterValueListParserError> {
    if s.is_curr_byte_eq(b')') {
        Ok(1.0)
    } else {
        let start = s.pos();
        let value = s.parse_number_or_percent()?;

        if value.is_sign_negative() {
            let pos = s.calc_char_pos_at(start);
            return Err(FilterValueListParserError::NegativeValue(pos));
        }

        Ok(value)
    }
}

fn parse_filter_length(s: &mut Stream) -> Result<Length, FilterValueListParserError> {
    let start = s.pos();
    let value = s.parse_length()?;

    if value.unit == LengthUnit::Percent {
        let pos = s.calc_char_pos_at(start);
        return Err(FilterValueListParserError::PercentageValue(pos));
    }

    Ok(value)
}

fn parse_filter_positive_length(s: &mut Stream) -> Result<Length, FilterValueListParserError> {
    let start = s.pos();
    let value = s.parse_length()?;

    if value.number.is_sign_negative() {
        let pos = s.calc_char_pos_at(start);
        return Err(FilterValueListParserError::NegativeValue(pos));
    }

    if value.unit == LengthUnit::Percent {
        let pos = s.calc_char_pos_at(start);
        return Err(FilterValueListParserError::PercentageValue(pos));
    }

    Ok(value)
}

// Just like a normal angle, but units are mandatory.
fn parse_filter_angle(s: &mut Stream) -> Result<Angle, FilterValueListParserError> {
    s.skip_spaces();

    let start = s.pos();
    let n = s.parse_number()?;

    let u = if s.starts_with(b"deg") {
        s.advance(3);
        AngleUnit::Degrees
    } else if s.starts_with(b"grad") {
        s.advance(4);
        AngleUnit::Gradians
    } else if s.starts_with(b"rad") {
        s.advance(3);
        AngleUnit::Radians
    } else if s.starts_with(b"turn") {
        s.advance(4);
        AngleUnit::Turns
    } else {
        // Only zero value allowed to be unit-less.
        if n == 0.0 {
            AngleUnit::Degrees
        } else {
            let pos = s.calc_char_pos_at(start);
            return Err(FilterValueListParserError::InvalidAngle(pos));
        }
    };

    Ok(Angle::new(n, u))
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn empty() {
        let mut parser = FilterValueListParser::from("");
        assert!(parser.next().is_none());
    }

    #[test]
    fn none() {
        let mut parser = FilterValueListParser::from("none");
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_default() {
        let mut parser = FilterValueListParser::from("blur()");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::zero()));
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_2() {
        let mut parser = FilterValueListParser::from("blur(2)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::new(2.0, LengthUnit::None)));
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_2mm() {
        let mut parser = FilterValueListParser::from("blur(2mm)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::new(2.0, LengthUnit::Mm)));
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_2percent() {
        let mut parser = FilterValueListParser::from("blur(2%)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "a percentage value detected at position 6");
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_negative() {
        let mut parser = FilterValueListParser::from("blur(-1)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "a negative value detected at position 6");
        assert!(parser.next().is_none());
    }

    #[test]
    fn blur_two_values() {
        let mut parser = FilterValueListParser::from("blur(1 2)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "expected ')' not '2' at position 8");
        assert!(parser.next().is_none());
    }

    #[test]
    fn brightness_default() {
        let mut parser = FilterValueListParser::from("brightness()");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Brightness(1.0));
        assert!(parser.next().is_none());
    }

    #[test]
    fn brightness_2() {
        let mut parser = FilterValueListParser::from("brightness(2)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Brightness(2.0));
        assert!(parser.next().is_none());
    }

    #[test]
    fn brightness_50percent() {
        let mut parser = FilterValueListParser::from("brightness(50%)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Brightness(0.5));
        assert!(parser.next().is_none());
    }

    #[test]
    fn brightness_negative() {
        let mut parser = FilterValueListParser::from("brightness(-1)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "a negative value detected at position 12");
        assert!(parser.next().is_none());
    }

    #[test]
    fn brightness_2mm() {
        let mut parser = FilterValueListParser::from("brightness(2mm)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "expected ')' not 'm' at position 13");
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_no_values() {
        let mut parser = FilterValueListParser::from("drop-shadow()");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "drop-shadow offset values are expected at position 13");
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_default() {
        let mut parser = FilterValueListParser::from("drop-shadow(2 3)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: None,
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::zero(),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_color_first() {
        let mut parser = FilterValueListParser::from("drop-shadow(red 2 3)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: Some(Color::new_rgb(255, 0, 0)),
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::zero(),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_color_after() {
        let mut parser = FilterValueListParser::from("drop-shadow(2 3 red)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: Some(Color::new_rgb(255, 0, 0)),
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::zero(),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_curr_color_first() {
        let mut parser = FilterValueListParser::from("drop-shadow(currentColor 2 3)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: None,
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::zero(),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_curr_color_after() {
        let mut parser = FilterValueListParser::from("drop-shadow(2 3 currentColor)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: None,
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::zero(),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_with_dev() {
        let mut parser = FilterValueListParser::from("drop-shadow(red 2 3 4)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: Some(Color::new_rgb(255, 0, 0)),
            dx: Length::new_number(2.0),
            dy: Length::new_number(3.0),
            std_dev: Length::new_number(4.0),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_color_twice() {
        let mut parser = FilterValueListParser::from("drop-shadow(red 2 3 4 red)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "expected ')' not 'r' at position 23");
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_curr_color_twice() {
        let mut parser = FilterValueListParser::from("drop-shadow(currentColor 2 3 4 currentColor)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "expected ')' not 'c' at position 32");
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_percent() {
        let mut parser = FilterValueListParser::from("drop-shadow(2% 3% 4%)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "a percentage value detected at position 13");
        assert!(parser.next().is_none());
    }

    #[test]
    fn drop_shadow_negative_offset() {
        let mut parser = FilterValueListParser::from("drop-shadow(-1 -2 3)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::DropShadow {
            color: None,
            dx: Length::new_number(-1.0),
            dy: Length::new_number(-2.0),
            std_dev: Length::new_number(3.0),
         });
        assert!(parser.next().is_none());
    }

    #[test]
    fn hue_rotate_no_units() {
        let mut parser = FilterValueListParser::from("hue-rotate(45)");
        assert_eq!(parser.next().unwrap().unwrap_err().to_string(),
                   "an invalid angle at position 12");
        assert!(parser.next().is_none());
    }

    #[test]
    fn url() {
        let mut parser = FilterValueListParser::from("url(#qwe)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Url("qwe"));
        assert!(parser.next().is_none());
    }

    #[test]
    fn multiple_1() {
        let mut parser = FilterValueListParser::from("blur() blur()");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::zero()));
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::zero()));
        assert!(parser.next().is_none());
    }

    #[test]
    fn multiple_2() {
        let mut parser = FilterValueListParser::from("blur() contrast(1)");
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Blur(Length::zero()));
        assert_eq!(parser.next().unwrap().unwrap(), FilterValue::Contrast(1.0));
        assert!(parser.next().is_none());
    }
}
