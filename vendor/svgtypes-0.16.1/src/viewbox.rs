// Copyright 2018 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Stream;

/// List of possible [`ViewBox`] parsing errors.
#[derive(Clone, Copy, Debug)]
pub enum ViewBoxError {
    /// One of the numbers is invalid.
    InvalidNumber,

    /// `ViewBox` has a negative or zero size.
    InvalidSize,
}

impl core::fmt::Display for ViewBoxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::InvalidNumber => {
                write!(f, "viewBox contains an invalid number")
            }
            Self::InvalidSize => {
                write!(f, "viewBox has a negative or zero size")
            }
        }
    }
}

impl core::error::Error for ViewBoxError {
    fn description(&self) -> &str {
        "a viewBox parsing error"
    }
}

/// Representation of the [`<viewBox>`] type.
///
/// [`<viewBox>`]: https://www.w3.org/TR/SVG2/coords.html#ViewBoxAttribute
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ViewBox {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl ViewBox {
    /// Creates a new `ViewBox`.
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

impl core::str::FromStr for ViewBox {
    type Err = ViewBoxError;

    fn from_str(text: &str) -> Result<Self, ViewBoxError> {
        let mut s = Stream::from(text);

        let x = s
            .parse_list_number()
            .map_err(|_| ViewBoxError::InvalidNumber)?;
        let y = s
            .parse_list_number()
            .map_err(|_| ViewBoxError::InvalidNumber)?;
        let w = s
            .parse_list_number()
            .map_err(|_| ViewBoxError::InvalidNumber)?;
        let h = s
            .parse_list_number()
            .map_err(|_| ViewBoxError::InvalidNumber)?;

        if w <= 0.0 || h <= 0.0 {
            return Err(ViewBoxError::InvalidSize);
        }

        Ok(Self::new(x, y, w, h))
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
                let v = ViewBox::from_str($text).unwrap();
                assert_eq!(v, $result);
            }
        )
    }

    test!(parse_1, "-20 30 100 500", ViewBox::new(-20.0, 30.0, 100.0, 500.0));

    macro_rules! test_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(ViewBox::from_str($text).unwrap_err().to_string(), $result);
            }
        )
    }

    test_err!(parse_err_1, "qwe", "viewBox contains an invalid number");
    test_err!(parse_err_2, "10 20 30 0", "viewBox has a negative or zero size");
    test_err!(parse_err_3, "10 20 0 40", "viewBox has a negative or zero size");
    test_err!(parse_err_4, "10 20 0 0", "viewBox has a negative or zero size");
    test_err!(parse_err_5, "10 20 -30 0", "viewBox has a negative or zero size");
    test_err!(parse_err_6, "10 20 30 -40", "viewBox has a negative or zero size");
    test_err!(parse_err_7, "10 20 -30 -40", "viewBox has a negative or zero size");
}
