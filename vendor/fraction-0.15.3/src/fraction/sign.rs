use std::{
    cmp::Ordering,
    fmt,
    ops::{Mul, Neg},
};

use crate::display;

/// Sign representation
///
/// Fractions keep the sign represented by this enum,
/// so that we can use unsigned ints as base data types.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde-support", derive(Serialize, Deserialize))]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    pub fn is_positive(self) -> bool {
        matches!(self, Sign::Plus)
    }

    pub fn is_negative(self) -> bool {
        matches!(self, Sign::Minus)
    }
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, oth: Sign) -> Self::Output {
        if self == oth {
            Sign::Plus
        } else {
            Sign::Minus
        }
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Sign {
        match self {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
        }
    }
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Sign::Minus, Sign::Minus) => Ordering::Equal,
            (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Plus, Sign::Plus) => Ordering::Equal,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format = display::Format::new(f);
        display::format_sign(*self, f, &format)
    }
}

impl From<Sign> for char {
    fn from(sign: Sign) -> char {
        match sign {
            Sign::Plus => '+',
            Sign::Minus => '-',
        }
    }
}
