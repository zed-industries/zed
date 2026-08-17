use self::ImproperCtype::*;
use crate::syntax::atom::Atom::{self, *};
use crate::syntax::query::TypeQuery;
use crate::syntax::Types;
use proc_macro2::Ident;

pub(crate) enum ImproperCtype<'a> {
    Definite(bool),
    Depends(&'a Ident),
}

impl<'a> Types<'a> {
    // yes, no, maybe
    pub(crate) fn determine_improper_ctype(
        &self,
        ty: impl Into<TypeQuery<'a>>,
    ) -> ImproperCtype<'a> {
        match ty.into() {
            TypeQuery::Ident(ident) => {
                let ident = &ident.rust;
                if let Some(atom) = Atom::from(ident) {
                    Definite(atom == RustString)
                } else if let Some(strct) = self.structs.get(ident) {
                    Depends(&strct.name.rust) // iterate to fixed-point
                } else {
                    Definite(self.rust.contains(ident) || self.aliases.contains_key(ident))
                }
            }
            TypeQuery::RustBox
            | TypeQuery::RustVec
            | TypeQuery::Str
            | TypeQuery::Fn
            | TypeQuery::Void
            | TypeQuery::SliceRef => Definite(true),
            TypeQuery::UniquePtr
            | TypeQuery::SharedPtr
            | TypeQuery::WeakPtr
            | TypeQuery::CxxVector => Definite(false),
            TypeQuery::Ref(ty) => self.determine_improper_ctype(&ty.inner),
            TypeQuery::Ptr(ty) => self.determine_improper_ctype(&ty.inner),
            TypeQuery::Array(ty) => self.determine_improper_ctype(&ty.inner),
        }
    }
}
