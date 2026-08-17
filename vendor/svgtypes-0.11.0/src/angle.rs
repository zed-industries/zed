use crate::{Error, Stream};

/// List of all SVG angle units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum AngleUnit {
    Degrees,
    Gradians,
    Radians,
    Turns,
}

/// Representation of the [`<angle>`] type.
///
/// [`<angle>`]: https://www.w3.org/TR/css-values-3/#angles
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Angle {
    pub number: f64,
    pub unit: AngleUnit,
}

impl Angle {
    /// Constructs a new angle.
    #[inline]
    pub fn new(number: f64, unit: AngleUnit) -> Angle {
        Angle { number, unit }
    }

    /// Converts angle to degrees.
    #[inline]
    pub fn to_degrees(&self) -> f64 {
        match self.unit {
            AngleUnit::Degrees => self.number,
            AngleUnit::Gradians => self.number * 180.0 / 200.0,
            AngleUnit::Radians => self.number.to_degrees(),
            AngleUnit::Turns => self.number * 360.0,
        }
    }
}

impl std::str::FromStr for Angle {
    type Err = Error;

    #[inline]
    fn from_str(text: &str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let l = s.parse_angle()?;

        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Angle::new(l.number, l.unit))
    }
}

impl<'a> Stream<'a> {
    /// Parses angle from the stream.
    ///
    /// <https://www.w3.org/TR/SVG2/types.html#InterfaceSVGAngle>
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    pub fn parse_angle(&mut self) -> Result<Angle, Error> {
        self.skip_spaces();

        let n = self.parse_number()?;

        if self.at_end() {
            return Ok(Angle::new(n, AngleUnit::Degrees));
        }

        let u = if self.starts_with(b"deg") {
            self.advance(3);
            AngleUnit::Degrees
        } else if self.starts_with(b"grad") {
            self.advance(4);
            AngleUnit::Gradians
        } else if self.starts_with(b"rad") {
            self.advance(3);
            AngleUnit::Radians
        } else if self.starts_with(b"turn") {
            self.advance(4);
            AngleUnit::Turns
        } else {
            AngleUnit::Degrees
        };

        Ok(Angle::new(n, u))
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
                assert_eq!(Angle::from_str($text).unwrap(), $result);
            }
        )
    }

    test_p!(parse_1,  "1",   Angle::new(1.0, AngleUnit::Degrees));
    test_p!(parse_2,  "1deg", Angle::new(1.0, AngleUnit::Degrees));
    test_p!(parse_3,  "1grad", Angle::new(1.0, AngleUnit::Gradians));
    test_p!(parse_4,  "1rad", Angle::new(1.0, AngleUnit::Radians));
    test_p!(parse_5,  "1turn", Angle::new(1.0, AngleUnit::Turns));

    #[test]
    fn err_1() {
        let mut s = Stream::from("1q");
        assert_eq!(s.parse_angle().unwrap(), Angle::new(1.0, AngleUnit::Degrees));
        assert_eq!(s.parse_angle().unwrap_err().to_string(),
                   "invalid number at position 2");
    }

    #[test]
    fn err_2() {
        assert_eq!(Angle::from_str("1degq").unwrap_err().to_string(),
                   "unexpected data at position 5");
    }
}
