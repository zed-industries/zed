use crate::syntax::atom::Atom::{self, *};
use crate::syntax::Type;

pub(crate) enum PrimitiveKind {
    Boolean,
    Number,
    Pointer,
}

pub(crate) fn kind(ty: &Type) -> Option<PrimitiveKind> {
    match ty {
        Type::Ident(ident) => Atom::from(&ident.rust).and_then(|atom| match atom {
            Bool => Some(PrimitiveKind::Boolean),
            Char | U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize | F32 | F64 => {
                Some(PrimitiveKind::Number)
            }
            CxxString | RustString => None,
        }),
        Type::Ptr(_) => Some(PrimitiveKind::Pointer),
        _ => None,
    }
}
