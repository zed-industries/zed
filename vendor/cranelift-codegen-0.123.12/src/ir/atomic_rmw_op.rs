/// Describes the arithmetic operation in an atomic memory read-modify-write operation.
use core::fmt::{self, Display, Formatter};
use core::str::FromStr;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
/// Describes the arithmetic operation in an atomic memory read-modify-write operation.
pub enum AtomicRmwOp {
    /// Add
    Add,
    /// Sub
    Sub,
    /// And
    And,
    /// Nand
    Nand,
    /// Or
    Or,
    /// Xor
    Xor,
    /// Exchange
    Xchg,
    /// Unsigned min
    Umin,
    /// Unsigned max
    Umax,
    /// Signed min
    Smin,
    /// Signed max
    Smax,
}

impl AtomicRmwOp {
    /// Returns a slice with all supported [AtomicRmwOp]'s.
    pub fn all() -> &'static [AtomicRmwOp] {
        &[
            AtomicRmwOp::Add,
            AtomicRmwOp::Sub,
            AtomicRmwOp::And,
            AtomicRmwOp::Nand,
            AtomicRmwOp::Or,
            AtomicRmwOp::Xor,
            AtomicRmwOp::Xchg,
            AtomicRmwOp::Umin,
            AtomicRmwOp::Umax,
            AtomicRmwOp::Smin,
            AtomicRmwOp::Smax,
        ]
    }
}

impl Display for AtomicRmwOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            AtomicRmwOp::Add => "add",
            AtomicRmwOp::Sub => "sub",
            AtomicRmwOp::And => "and",
            AtomicRmwOp::Nand => "nand",
            AtomicRmwOp::Or => "or",
            AtomicRmwOp::Xor => "xor",
            AtomicRmwOp::Xchg => "xchg",
            AtomicRmwOp::Umin => "umin",
            AtomicRmwOp::Umax => "umax",
            AtomicRmwOp::Smin => "smin",
            AtomicRmwOp::Smax => "smax",
        };
        f.write_str(s)
    }
}

impl FromStr for AtomicRmwOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(AtomicRmwOp::Add),
            "sub" => Ok(AtomicRmwOp::Sub),
            "and" => Ok(AtomicRmwOp::And),
            "nand" => Ok(AtomicRmwOp::Nand),
            "or" => Ok(AtomicRmwOp::Or),
            "xor" => Ok(AtomicRmwOp::Xor),
            "xchg" => Ok(AtomicRmwOp::Xchg),
            "umin" => Ok(AtomicRmwOp::Umin),
            "umax" => Ok(AtomicRmwOp::Umax),
            "smin" => Ok(AtomicRmwOp::Smin),
            "smax" => Ok(AtomicRmwOp::Smax),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_parse() {
        for op in AtomicRmwOp::all() {
            let roundtripped = format!("{op}").parse::<AtomicRmwOp>().unwrap();
            assert_eq!(*op, roundtripped);
        }
    }
}
