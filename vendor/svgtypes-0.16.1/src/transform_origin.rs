// Copyright 2023 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::directional_position::DirectionalPosition;
use crate::stream::Stream;
use crate::{Length, LengthUnit};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
enum Position {
    Length(Length),
    DirectionalPosition(DirectionalPosition),
}

impl Position {
    fn is_vertical(&self) -> bool {
        match self {
            Self::Length(_) => true,
            Self::DirectionalPosition(dp) => dp.is_vertical(),
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Self::Length(_) => true,
            Self::DirectionalPosition(dp) => dp.is_horizontal(),
        }
    }
}

impl From<Position> for Length {
    fn from(value: Position) -> Self {
        match value {
            Position::Length(l) => l,
            Position::DirectionalPosition(dp) => dp.into(),
        }
    }
}

/// Representation of the [`<transform-origin>`] type.
///
/// [`<transform-origin>`]: https://drafts.csswg.org/css-transforms/#transform-origin-property
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TransformOrigin {
    /// The x offset of the transform origin.
    pub x_offset: Length,
    /// The y offset of the transform origin.
    pub y_offset: Length,
    /// The z offset of the transform origin.
    pub z_offset: Length,
}

impl TransformOrigin {
    /// Constructs a new transform origin.
    #[inline]
    pub fn new(x_offset: Length, y_offset: Length, z_offset: Length) -> Self {
        Self {
            x_offset,
            y_offset,
            z_offset,
        }
    }
}

/// List of possible [`TransformOrigin`] parsing errors.
#[derive(Clone, Copy, Debug)]
pub enum TransformOriginError {
    /// One of the numbers is invalid.
    MissingParameters,
    /// One of the parameters is invalid.
    InvalidParameters,
    /// z-index is not a percentage.
    ZIndexIsPercentage,
}

impl core::fmt::Display for TransformOriginError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::MissingParameters => {
                write!(f, "transform origin doesn't have enough parameters")
            }
            Self::InvalidParameters => {
                write!(f, "transform origin has invalid parameters")
            }
            Self::ZIndexIsPercentage => {
                write!(f, "z-index cannot be a percentage")
            }
        }
    }
}

impl core::error::Error for TransformOriginError {
    fn description(&self) -> &str {
        "a transform origin parsing error"
    }
}

impl core::str::FromStr for TransformOrigin {
    type Err = TransformOriginError;

    fn from_str(text: &str) -> Result<Self, TransformOriginError> {
        let mut stream = Stream::from(text);

        if stream.at_end() {
            return Err(TransformOriginError::MissingParameters);
        }

        let parse_part = |stream: &mut Stream<'_>| {
            if let Ok(dp) = stream.parse_directional_position() {
                Some(Position::DirectionalPosition(dp))
            } else if let Ok(l) = stream.parse_length() {
                Some(Position::Length(l))
            } else {
                None
            }
        };

        let first_arg = parse_part(&mut stream);
        let mut second_arg = None;
        let mut third_arg = None;

        if !stream.at_end() {
            stream.skip_spaces();
            stream.parse_list_separator();
            second_arg =
                Some(parse_part(&mut stream).ok_or(TransformOriginError::InvalidParameters)?);
        }

        if !stream.at_end() {
            stream.skip_spaces();
            stream.parse_list_separator();
            third_arg = Some(
                stream
                    .parse_length()
                    .map_err(|_| TransformOriginError::InvalidParameters)?,
            );
        }

        stream.skip_spaces();

        if !stream.at_end() {
            return Err(TransformOriginError::InvalidParameters);
        }

        let result = match (first_arg, second_arg, third_arg) {
            (Some(p), None, None) => {
                let (x_offset, y_offset) = if p.is_horizontal() {
                    (p.into(), DirectionalPosition::Center.into())
                } else {
                    (DirectionalPosition::Center.into(), p.into())
                };

                Self::new(x_offset, y_offset, Length::new(0.0, LengthUnit::Px))
            }
            (Some(p1), Some(p2), length) => {
                if let Some(length) = length {
                    if length.unit == LengthUnit::Percent {
                        return Err(TransformOriginError::ZIndexIsPercentage);
                    }
                }

                let length = length.unwrap_or(Length::new(0.0, LengthUnit::Px));

                let check = |pos| match pos {
                    Position::Length(_) => true,
                    Position::DirectionalPosition(dp) => dp == DirectionalPosition::Center,
                };

                let only_keyword_is_center = check(p1) && check(p2);

                if only_keyword_is_center {
                    Self::new(p1.into(), p2.into(), length)
                } else {
                    // There is at least one of `left`, `right`, `top`, or `bottom`
                    if p1.is_horizontal() && p2.is_vertical() {
                        Self::new(p1.into(), p2.into(), length)
                    } else if p1.is_vertical() && p2.is_horizontal() {
                        Self::new(p2.into(), p1.into(), length)
                    } else {
                        return Err(TransformOriginError::InvalidParameters);
                    }
                }
            }
            _ => unreachable!(),
        };

        Ok(result)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use core::str::FromStr;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                let v = TransformOrigin::from_str($text).unwrap();
                assert_eq!(v, $result);
            }
        )
    }

    test!(parse_1, "center", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_2, "left", TransformOrigin::new(Length::new(0.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_3, "right", TransformOrigin::new(Length::new(100.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_4, "top", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_5, "bottom", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(100.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_6, "30px", TransformOrigin::new(Length::new(30.0, LengthUnit::Px), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));

    test!(parse_7, "center left", TransformOrigin::new(Length::new(0.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_8, "left center", TransformOrigin::new(Length::new(0.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_9, "center bottom", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(100.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_10, "bottom center", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(100.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_11, "30%, center", TransformOrigin::new(Length::new(30.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_12, " center, 30%", TransformOrigin::new(Length::new(50.0, LengthUnit::Percent), Length::new(30.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));
    test!(parse_13, "left top", TransformOrigin::new(Length::new(0.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Percent), Length::new(0.0, LengthUnit::Px)));

    test!(parse_14, "center right 3px", TransformOrigin::new(Length::new(100.0, LengthUnit::Percent), Length::new(50.0, LengthUnit::Percent), Length::new(3.0, LengthUnit::Px)));

    macro_rules! test_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(TransformOrigin::from_str($text).unwrap_err().to_string(), $result);
            }
        )
    }

    test_err!(parse_err_1, "", "transform origin doesn't have enough parameters");
    test_err!(parse_err_2, "some", "transform origin has invalid parameters");
    test_err!(parse_err_3, "center some", "transform origin has invalid parameters");
    test_err!(parse_err_4, "left right", "transform origin has invalid parameters");
    test_err!(parse_err_5, "left top 3%", "z-index cannot be a percentage");
}
