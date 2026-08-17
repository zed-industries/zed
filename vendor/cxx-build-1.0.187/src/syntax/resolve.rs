use crate::syntax::attrs::OtherAttrs;
use crate::syntax::instantiate::NamedImplKey;
use crate::syntax::{Lifetimes, NamedType, Pair, Types};
use proc_macro2::Ident;

#[derive(Copy, Clone)]
pub(crate) struct Resolution<'a> {
    pub name: &'a Pair,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub attrs: &'a OtherAttrs,
    pub generics: &'a Lifetimes,
}

impl<'a> Types<'a> {
    pub(crate) fn resolve(&self, ident: &impl UnresolvedName) -> Resolution<'a> {
        let ident = ident.ident();
        match self.try_resolve(ident) {
            Some(resolution) => resolution,
            None => panic!("Unable to resolve type `{}`", ident),
        }
    }

    pub(crate) fn try_resolve(&self, ident: &impl UnresolvedName) -> Option<Resolution<'a>> {
        let ident = ident.ident();
        self.resolutions.get(ident).copied()
    }
}

pub(crate) trait UnresolvedName {
    fn ident(&self) -> &Ident;
}

impl UnresolvedName for Ident {
    fn ident(&self) -> &Ident {
        self
    }
}

impl UnresolvedName for NamedType {
    fn ident(&self) -> &Ident {
        &self.rust
    }
}

impl<'a> UnresolvedName for NamedImplKey<'a> {
    fn ident(&self) -> &Ident {
        self.rust
    }
}
