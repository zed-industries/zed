use crate::syntax::Type;
use proc_macro2::Ident;
use std::fmt::{self, Display};

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Atom {
    Bool,
    Char, // C char, not Rust char
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    CxxString,
    RustString,
}

impl Atom {
    pub(crate) fn from(ident: &Ident) -> Option<Self> {
        Self::from_str(ident.to_string().as_str())
    }

    pub(crate) fn from_str(s: &str) -> Option<Self> {
        use self::Atom::*;
        match s {
            "bool" => Some(Bool),
            "c_char" => Some(Char),
            "u8" => Some(U8),
            "u16" => Some(U16),
            "u32" => Some(U32),
            "u64" => Some(U64),
            "usize" => Some(Usize),
            "i8" => Some(I8),
            "i16" => Some(I16),
            "i32" => Some(I32),
            "i64" => Some(I64),
            "isize" => Some(Isize),
            "f32" => Some(F32),
            "f64" => Some(F64),
            "CxxString" => Some(CxxString),
            "String" => Some(RustString),
            _ => None,
        }
    }
}

impl Display for Atom {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.as_ref())
    }
}

impl AsRef<str> for Atom {
    fn as_ref(&self) -> &str {
        use self::Atom::*;
        match self {
            Bool => "bool",
            Char => "c_char",
            U8 => "u8",
            U16 => "u16",
            U32 => "u32",
            U64 => "u64",
            Usize => "usize",
            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",
            Isize => "isize",
            F32 => "f32",
            F64 => "f64",
            CxxString => "CxxString",
            RustString => "String",
        }
    }
}

impl PartialEq<Atom> for Type {
    fn eq(&self, atom: &Atom) -> bool {
        match self {
            Type::Ident(ident) => ident.rust == atom,
            _ => false,
        }
    }
}

impl PartialEq<Atom> for &Ident {
    fn eq(&self, atom: &Atom) -> bool {
        *self == atom
    }
}

impl PartialEq<Atom> for &Type {
    fn eq(&self, atom: &Atom) -> bool {
        *self == atom
    }
}
