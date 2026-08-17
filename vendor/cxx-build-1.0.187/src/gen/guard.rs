use crate::gen::out::OutFile;
use crate::syntax::symbol::Symbol;
use crate::syntax::Pair;
use std::fmt::{self, Display};

pub(crate) struct Guard {
    kind: &'static str,
    symbol: Symbol,
}

impl Guard {
    pub fn new(out: &mut OutFile, kind: &'static str, name: &Pair) -> Self {
        let symbol = name.to_symbol();
        out.pragma.dollar_in_identifier |= symbol.contains('$');
        Guard { kind, symbol }
    }
}

impl Display for Guard {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}_{}", self.kind, self.symbol)
    }
}
