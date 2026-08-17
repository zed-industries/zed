use crate::{Error, Stream};

/// List of all SVG length units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum LengthUnit {
    None,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
}

/// Representation of the [`<length>`] type.
///
/// [`<length>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Length {
    pub number: f64,
    pub unit: LengthUnit,
}

impl Length {
    /// Constructs a new length.
    #[inline]
    pub fn new(number: f64, unit: LengthUnit) -> Length {
        Length { number, unit }
    }

    /// Constructs a new length with `LengthUnit::None`.
    #[inline]
    pub fn new_number(number: f64) -> Length {
        Length {
            number,
            unit: LengthUnit::None,
        }
    }

    /// Constructs a new length with a zero number.
    ///
    /// Shorthand for: `Length::new(0.0, Unit::None)`.
    #[inline]
    pub fn zero() -> Length {
        Length {
            number: 0.0,
            unit: LengthUnit::None,
        }
    }
}

impl Default for Length {
    #[inline]
    fn default() -> Self {
        Length::zero()
    }
}

impl std::str::FromStr for Length {
    type Err = Error;

    #[inline]
    fn from_str(text: &str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let l = s.parse_length()?;

        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Length::new(l.number, l.unit))
    }
}

impl<'a> Stream<'a> {
    /// Parses length from the stream.
    ///
    /// <https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength>
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    pub fn parse_length(&mut self) -> Result<Length, Error> {
        self.skip_spaces();

        let n = self.parse_number()?;

        if self.at_end() {
            return Ok(Length::new(n, LengthUnit::None));
        }

        let u = if self.starts_with(b"%") {
            LengthUnit::Percent
        } else if self.starts_with(b"em") {
            LengthUnit::Em
        } else if self.starts_with(b"ex") {
            LengthUnit::Ex
        } else if self.starts_with(b"px") {
            LengthUnit::Px
        } else if self.starts_with(b"in") {
            LengthUnit::In
        } else if self.starts_with(b"cm") {
            LengthUnit::Cm
        } else if self.starts_with(b"mm") {
            LengthUnit::Mm
        } else if self.starts_with(b"pt") {
            LengthUnit::Pt
        } else if self.starts_with(b"pc") {
            LengthUnit::Pc
        } else {
            LengthUnit::None
        };

        match u {
            LengthUnit::Percent => self.advance(1),
            LengthUnit::None => {}
            _ => self.advance(2),
        }

        Ok(Length::new(n, u))
    }

    /// Parses length from a list of lengths.
    pub fn parse_list_length(&mut self) -> Result<Length, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        let l = self.parse_length()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(l)
    }
}

/// A pull-based [`<list-of-length>`] parser.
///
/// # Examples
///
/// ```
/// use svgtypes::{Length, LengthUnit, LengthListParser};
///
/// let mut p = LengthListParser::from("10px 20% 50mm");
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(10.0, LengthUnit::Px));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(20.0, LengthUnit::Percent));
/// assert_eq!(p.next().unwrap().unwrap(), Length::new(50.0, LengthUnit::Mm));
/// assert_eq!(p.next().is_none(), true);
/// ```
///
/// [`<list-of-length>`]: https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLengthList
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LengthListParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for LengthListParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        LengthListParser(Stream::from(v))
    }
}

impl<'a> Iterator for LengthListParser<'a> {
    type Item = Result<Length, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let v = self.0.parse_list_length();
            if v.is_err() {
                self.0.jump_to_end();
            }

            Some(v)
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test_p {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Length::from_str($text).unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "1",   Length::new(1.0, LengthUnit::None));
    test_p!(parse_2,  "1em", Length::new(1.0, LengthUnit::Em));
    test_p!(parse_3,  "1ex", Length::new(1.0, LengthUnit::Ex));
    test_p!(parse_4,  "1px", Length::new(1.0, LengthUnit::Px));
    test_p!(parse_5,  "1in", Length::new(1.0, LengthUnit::In));
    test_p!(parse_6,  "1cm", Length::new(1.0, LengthUnit::Cm));
    test_p!(parse_7,  "1mm", Length::new(1.0, LengthUnit::Mm));
    test_p!(parse_8,  "1pt", Length::new(1.0, LengthUnit::Pt));
    test_p!(parse_9,  "1pc", Length::new(1.0, LengthUnit::Pc));
    test_p!(parse_10, "1%",  Length::new(1.0, LengthUnit::Percent));
    test_p!(parse_11, "1e0", Length::new(1.0, LengthUnit::None));
    test_p!(parse_12, "1.0e0", Length::new(1.0, LengthUnit::None));
    test_p!(parse_13, "1.0e0em", Length::new(1.0, LengthUnit::Em));

    #[test]
    fn parse_14() {
        let mut s = Stream::from("1,");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn parse_15() {
        let mut s = Stream::from("1 ,");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn parse_16() {
        let mut s = Stream::from("1 1");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    }

    #[test]
    fn err_1() {
        let mut s = Stream::from("1q");
        assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
        assert_eq!(s.parse_length().unwrap_err().to_string(),
                   "invalid number at position 2");
    }

    #[test]
    fn err_2() {
        assert_eq!(Length::from_str("1mmx").unwrap_err().to_string(),
                   "unexpected data at position 4");
    }
}
