use core::fmt;
use core::str::FromStr;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// A well-known symbol.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum KnownSymbol {
    /// ELF well-known linker symbol _GLOBAL_OFFSET_TABLE_
    ElfGlobalOffsetTable,
    /// TLS index symbol for the current thread.
    /// Used in COFF/PE file formats.
    CoffTlsIndex,
}

impl fmt::Display for KnownSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for KnownSymbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ElfGlobalOffsetTable" => Ok(Self::ElfGlobalOffsetTable),
            "CoffTlsIndex" => Ok(Self::CoffTlsIndex),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            "ElfGlobalOffsetTable".parse(),
            Ok(KnownSymbol::ElfGlobalOffsetTable)
        );
        assert_eq!("CoffTlsIndex".parse(), Ok(KnownSymbol::CoffTlsIndex));
    }
}
