use std::fmt;

pub const FIRST_LINE: u32 = 1;
pub const FIRST_COL: u32 = 1;

/// Location in file
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Loc {
    /// 1-based
    pub line: u32,
    /// 1-based
    pub col: u32,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl Loc {
    pub fn start() -> Loc {
        Loc {
            line: FIRST_LINE,
            col: FIRST_COL,
        }
    }
}
