use crate::syntax::atom::Atom::{self, *};
use crate::syntax::query::TypeQuery;
use crate::syntax::{primitive, Types};

impl<'a> Types<'a> {
    pub(crate) fn is_guaranteed_pod(&self, ty: impl Into<TypeQuery<'a>>) -> bool {
        match ty.into() {
            TypeQuery::Ident(ident) => {
                let ident = &ident.rust;
                if let Some(atom) = Atom::from(ident) {
                    match atom {
                        Bool | Char | U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64
                        | Isize | F32 | F64 => true,
                        CxxString | RustString => false,
                    }
                } else if let Some(strct) = self.structs.get(ident) {
                    strct.fields.iter().all(|field| {
                        primitive::kind(&field.ty).is_none() && self.is_guaranteed_pod(&field.ty)
                    })
                } else {
                    self.enums.contains_key(ident)
                }
            }
            TypeQuery::RustBox
            | TypeQuery::RustVec
            | TypeQuery::UniquePtr
            | TypeQuery::SharedPtr
            | TypeQuery::WeakPtr
            | TypeQuery::CxxVector
            | TypeQuery::Void => false,
            TypeQuery::Ref(_)
            | TypeQuery::Str
            | TypeQuery::Fn
            | TypeQuery::SliceRef
            | TypeQuery::Ptr(_) => true,
            TypeQuery::Array(array) => self.is_guaranteed_pod(&array.inner),
        }
    }
}
