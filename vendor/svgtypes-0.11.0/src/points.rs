use crate::Stream;

/// A pull-based [`<list-of-points>`] parser.
///
/// Use it for the `points` attribute of the `polygon` and `polyline` elements.
///
/// # Errors
///
/// - Stops on a first invalid character. Follows the same rules as paths parser.
///
/// # Notes
///
/// - If data contains an odd number of coordinates - the last one will be ignored.
///   As SVG spec states.
/// - It doesn't validate that there are more than two coordinate pairs,
///   which is required by the SVG spec.
///
/// # Examples
///
/// ```
/// use svgtypes::PointsParser;
///
/// let mut p = PointsParser::from("10 20 30 40");
/// assert_eq!(p.next(), Some((10.0, 20.0)));
/// assert_eq!(p.next(), Some((30.0, 40.0)));
/// assert_eq!(p.next(), None);
/// ```
///
/// [`<list-of-points>`]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PointsParser<'a>(Stream<'a>);

impl<'a> From<&'a str> for PointsParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        PointsParser(Stream::from(v))
    }
}

impl<'a> Iterator for PointsParser<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let x = match self.0.parse_list_number() {
                Ok(x) => x,
                Err(_) => return None,
            };

            let y = match self.0.parse_list_number() {
                Ok(y) => y,
                Err(_) => return None,
            };

            Some((x, y))
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let mut parser = PointsParser::from("10 20 30 40");
        assert_eq!(parser.next().unwrap(), (10.0, 20.0));
        assert_eq!(parser.next().unwrap(), (30.0, 40.0));
        assert!(parser.next().is_none());
    }

    #[test]
    fn parse_2() {
        let mut parser = PointsParser::from("10 20 30 40 50");
        assert_eq!(parser.next().unwrap(), (10.0, 20.0));
        assert_eq!(parser.next().unwrap(), (30.0, 40.0));
        assert!(parser.next().is_none());
    }
}
