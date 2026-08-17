use crate::syntax::{Array, NamedType, Ptr, Ref, Type};

#[derive(Copy, Clone)]
pub(crate) enum TypeQuery<'a> {
    Ident(&'a NamedType),
    RustBox,
    RustVec,
    UniquePtr,
    SharedPtr,
    WeakPtr,
    Ref(&'a Ref),
    Ptr(&'a Ptr),
    Str,
    CxxVector,
    Fn,
    Void,
    SliceRef,
    Array(&'a Array),
}

impl<'a> From<&'a NamedType> for TypeQuery<'a> {
    fn from(query: &'a NamedType) -> Self {
        TypeQuery::Ident(query)
    }
}

impl<'a> From<&'a Type> for TypeQuery<'a> {
    fn from(query: &'a Type) -> Self {
        match query {
            Type::Ident(query) => TypeQuery::Ident(query),
            Type::RustBox(_) => TypeQuery::RustBox,
            Type::RustVec(_) => TypeQuery::RustVec,
            Type::UniquePtr(_) => TypeQuery::UniquePtr,
            Type::SharedPtr(_) => TypeQuery::SharedPtr,
            Type::WeakPtr(_) => TypeQuery::WeakPtr,
            Type::Ref(query) => TypeQuery::Ref(query),
            Type::Ptr(query) => TypeQuery::Ptr(query),
            Type::Str(_) => TypeQuery::Str,
            Type::CxxVector(_) => TypeQuery::CxxVector,
            Type::Fn(_) => TypeQuery::Fn,
            Type::Void(_) => TypeQuery::Void,
            Type::SliceRef(_) => TypeQuery::SliceRef,
            Type::Array(query) => TypeQuery::Array(query),
        }
    }
}
