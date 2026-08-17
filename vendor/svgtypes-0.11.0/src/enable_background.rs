use crate::{Error, Stream};

/// Representation of the [`enable-background`] attribute.
///
/// [`enable-background`]: https://www.w3.org/TR/SVG11/filters.html#EnableBackgroundProperty
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum EnableBackground {
    Accumulate,
    New,
    NewWithRegion {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
}

impl std::str::FromStr for EnableBackground {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut s = Stream::from(text);
        s.skip_spaces();
        if s.starts_with(b"accumulate") {
            s.advance(10);
            s.skip_spaces();
            if !s.at_end() {
                return Err(Error::UnexpectedData(s.calc_char_pos()));
            }

            Ok(EnableBackground::Accumulate)
        } else if s.starts_with(b"new") {
            s.advance(3);
            s.skip_spaces();
            if s.at_end() {
                return Ok(EnableBackground::New);
            }

            let x = s.parse_list_number()?;
            let y = s.parse_list_number()?;
            let width = s.parse_list_number()?;
            let height = s.parse_list_number()?;

            s.skip_spaces();
            if !s.at_end() {
                return Err(Error::UnexpectedData(s.calc_char_pos()));
            }

            // Region size must be valid;
            if !(width > 0.0 && height > 0.0) {
                return Err(Error::InvalidValue);
            }

            Ok(EnableBackground::NewWithRegion {
                x,
                y,
                width,
                height,
            })
        } else {
            Err(Error::InvalidValue)
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_1() {
        assert_eq!(EnableBackground::from_str("accumulate").unwrap(), EnableBackground::Accumulate);
    }

    #[test]
    fn parse_2() {
        assert_eq!(EnableBackground::from_str("  accumulate  ").unwrap(), EnableBackground::Accumulate);
    }

    #[test]
    fn parse_3() {
        assert_eq!(EnableBackground::from_str("new").unwrap(), EnableBackground::New);
    }

    #[test]
    fn parse_4() {
        assert_eq!(EnableBackground::from_str("  new  ").unwrap(), EnableBackground::New);
    }

    #[test]
    fn parse_5() {
        assert_eq!(EnableBackground::from_str("new 1 2 3 4").unwrap(),
                   EnableBackground::NewWithRegion { x: 1.0, y: 2.0, width: 3.0, height: 4.0 });
    }

    #[test]
    fn err_1() {
        assert_eq!(EnableBackground::from_str(" accumulate b ").unwrap_err().to_string(),
                   "unexpected data at position 13");
    }

    #[test]
    fn err_2() {
        assert_eq!(EnableBackground::from_str(" new b ").unwrap_err().to_string(),
                   "invalid number at position 6");
    }

    #[test]
    fn err_3() {
        assert_eq!(EnableBackground::from_str("new 1 2 3").unwrap_err().to_string(),
                   "unexpected end of stream");
    }

    #[test]
    fn err_4() {
        assert_eq!(EnableBackground::from_str("new 1 2 3 4 5").unwrap_err().to_string(),
                   "unexpected data at position 13");
    }

    #[test]
    fn err_5() {
        assert_eq!(EnableBackground::from_str("new 0 0 0 0").unwrap_err().to_string(),
                   "invalid value");
    }
}
