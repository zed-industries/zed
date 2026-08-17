use super::*;

/// The possible types that a registry value could have.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    /// A 32-bit unsigned integer value.
    U32,

    /// A 64-bit unsigned integer value.
    U64,

    /// A string value.
    String,

    /// A string value that may contain unexpanded environment variables.
    ExpandString,

    /// An array of string values.
    MultiString,

    /// An array u8 bytes.
    Bytes,

    /// An unknown type.
    Other(u32),
}

impl From<u32> for Type {
    fn from(ty: u32) -> Self {
        match ty {
            REG_DWORD => Self::U32,
            REG_QWORD => Self::U64,
            REG_SZ => Self::String,
            REG_EXPAND_SZ => Self::ExpandString,
            REG_MULTI_SZ => Self::MultiString,
            REG_BINARY => Self::Bytes,
            rest => Self::Other(rest),
        }
    }
}

impl From<Type> for u32 {
    fn from(ty: Type) -> Self {
        match ty {
            Type::U32 => REG_DWORD,
            Type::U64 => REG_QWORD,
            Type::String => REG_SZ,
            Type::ExpandString => REG_EXPAND_SZ,
            Type::MultiString => REG_MULTI_SZ,
            Type::Bytes => REG_BINARY,
            Type::Other(other) => other,
        }
    }
}
