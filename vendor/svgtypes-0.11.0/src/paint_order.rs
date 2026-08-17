use crate::stream::Stream;

/// [`paint-order`] property variants.
///
/// [`paint-order`]: https://www.w3.org/TR/SVG2/painting.html#PaintOrder
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum PaintOrderKind {
    Fill,
    Stroke,
    Markers,
}

/// Representation of the [`paint-order`] property.
///
/// [`paint-order`]: https://www.w3.org/TR/SVG2/painting.html#PaintOrder
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PaintOrder {
    /// The order.
    ///
    /// Guarantee to not have duplicates.
    pub order: [PaintOrderKind; 3],
}

impl Default for PaintOrder {
    #[inline]
    fn default() -> Self {
        Self {
            order: [
                PaintOrderKind::Fill,
                PaintOrderKind::Stroke,
                PaintOrderKind::Markers,
            ],
        }
    }
}

impl From<[PaintOrderKind; 3]> for PaintOrder {
    #[inline]
    fn from(order: [PaintOrderKind; 3]) -> Self {
        Self { order }
    }
}

impl std::str::FromStr for PaintOrder {
    type Err = ();

    /// Parses `PaintOrder` from a string.
    ///
    /// Never returns an error and fallbacks to the default value instead.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut order = Vec::new();

        let mut left = vec![
            PaintOrderKind::Fill,
            PaintOrderKind::Stroke,
            PaintOrderKind::Markers,
        ];

        let mut s = Stream::from(text);
        while !s.at_end() && order.len() < 3 {
            s.skip_spaces();
            let name = s.consume_ident();
            s.skip_spaces();
            let name = match name {
                // `normal` is the special value that should short-circuit.
                "normal" => return Ok(PaintOrder::default()),
                "fill" => PaintOrderKind::Fill,
                "stroke" => PaintOrderKind::Stroke,
                "markers" => PaintOrderKind::Markers,
                _ => return Ok(PaintOrder::default()),
            };

            if let Some(index) = left.iter().position(|v| *v == name) {
                left.remove(index);
            }

            order.push(name);
        }

        s.skip_spaces();
        if !s.at_end() {
            // Any trailing data is an error.
            return Ok(PaintOrder::default());
        }

        if order.is_empty() {
            return Ok(PaintOrder::default());
        }

        // Any missing values should be added in the original order.
        while order.len() < 3 && !left.is_empty() {
            order.push(left.remove(0));
        }

        // Any duplicates is an error.
        if order[0] == order[1] || order[0] == order[2] || order[1] == order[2] {
            // Any trailing data is an error.
            return Ok(PaintOrder::default());
        }

        Ok(PaintOrder {
            order: [order[0], order[1], order[2]],
        })
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_1() {
        assert_eq!(PaintOrder::from_str("normal").unwrap(), PaintOrder::default());
    }

    #[test]
    fn parse_2() {
        assert_eq!(PaintOrder::from_str("qwe").unwrap(), PaintOrder::default());
    }

    #[test]
    fn parse_3() {
        assert_eq!(PaintOrder::from_str("").unwrap(), PaintOrder::default());
    }

    #[test]
    fn parse_4() {
        assert_eq!(PaintOrder::from_str("stroke qwe").unwrap(), PaintOrder::default());
    }

    #[test]
    fn parse_5() {
        assert_eq!(PaintOrder::from_str("stroke stroke").unwrap(), PaintOrder::default());
    }

    #[test]
    fn parse_6() {
        assert_eq!(PaintOrder::from_str("stroke").unwrap(), PaintOrder::from([
            PaintOrderKind::Stroke, PaintOrderKind::Fill, PaintOrderKind::Markers
        ]));
    }

    #[test]
    fn parse_7() {
        assert_eq!(PaintOrder::from_str("stroke markers").unwrap(), PaintOrder::from([
            PaintOrderKind::Stroke, PaintOrderKind::Markers, PaintOrderKind::Fill
        ]));
    }

    #[test]
    fn parse_8() {
        assert_eq!(PaintOrder::from_str("stroke markers fill").unwrap(), PaintOrder::from([
            PaintOrderKind::Stroke, PaintOrderKind::Markers, PaintOrderKind::Fill
        ]));
    }

    #[test]
    fn parse_9() {
        assert_eq!(PaintOrder::from_str("markers").unwrap(), PaintOrder::from([
            PaintOrderKind::Markers, PaintOrderKind::Fill, PaintOrderKind::Stroke
        ]));
    }

    #[test]
    fn parse_10() {
        assert_eq!(PaintOrder::from_str("  stroke\n").unwrap(), PaintOrder::from([
            PaintOrderKind::Stroke, PaintOrderKind::Fill, PaintOrderKind::Markers
        ]));
    }

    #[test]
    fn parse_11() {
        assert_eq!(PaintOrder::from_str("stroke stroke stroke stroke").unwrap(), PaintOrder::default());
    }
}
