use std::fmt::{self, Display, Formatter};

use crate::type_info::TypeInfo;

use AnyTypeInfoKind::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AnyTypeInfo {
    #[doc(hidden)]
    pub kind: AnyTypeInfoKind,
}

impl AnyTypeInfo {
    pub fn kind(&self) -> AnyTypeInfoKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyTypeInfoKind {
    Null,
    Bool,
    SmallInt,
    Integer,
    BigInt,
    Real,
    Double,
    Text,
    Blob,
}

impl TypeInfo for AnyTypeInfo {
    fn is_null(&self) -> bool {
        self.kind == Null
    }

    fn name(&self) -> &str {
        use AnyTypeInfoKind::*;

        match self.kind {
            Bool => "BOOLEAN",
            SmallInt => "SMALLINT",
            Integer => "INTEGER",
            BigInt => "BIGINT",
            Real => "REAL",
            Double => "DOUBLE",
            Text => "TEXT",
            Blob => "BLOB",
            Null => "NULL",
        }
    }
}

impl Display for AnyTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl AnyTypeInfoKind {
    pub fn is_integer(&self) -> bool {
        matches!(self, SmallInt | Integer | BigInt)
    }
}
