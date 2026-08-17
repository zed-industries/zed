// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides the `IsUninhabited` trait.
//!
//! By nature, determining if a type is uninhabited or not given Rust's
//! turing complete type system is undecidable. Furthermore, we don't even
//! have access to all the information because we can't inspect type
//! definitions, type macros, or projections via associated types.
//!
//! Any analysis we perform here is therefore incomplete but sound.
//! That is, if we state that a type is uninhabited, it is so for sure.
//! But we can't state that all uninhabited types are uninhabited.

use syn::{self, visit};

use crate::interp;
use crate::util;

//==============================================================================
// Trait
//==============================================================================

/// A trait for types for which it is possible to check if the modelled
/// object is uninhabited or not. A `false` answer means that we can not
/// tell for sure that the thing is uninhabited, not that we are 100%
/// certain that it is inhabited.
pub trait IsUninhabited {
    /// Returns true if the given type is known to be uninhabited.
    /// There may be more scenarios under which the type is uninhabited.
    /// Thus, this is not a complete and exhaustive check.
    fn is_uninhabited(&self) -> bool;
}

//==============================================================================
// Enum/Variants:
//==============================================================================

impl IsUninhabited for syn::DataEnum {
    fn is_uninhabited(&self) -> bool {
        self.variants.is_uninhabited()
    }
}

impl<P> IsUninhabited for syn::punctuated::Punctuated<syn::Variant, P> {
    fn is_uninhabited(&self) -> bool {
        self.iter().all(IsUninhabited::is_uninhabited)
    }
}

impl<'a> IsUninhabited for &'a [syn::Variant] {
    fn is_uninhabited(&self) -> bool {
        self.iter().all(IsUninhabited::is_uninhabited)
    }
}

impl IsUninhabited for syn::Variant {
    fn is_uninhabited(&self) -> bool {
        self.fields.is_uninhabited()
    }
}

//==============================================================================
// Struct/Fields:
//==============================================================================

impl IsUninhabited for syn::Fields {
    fn is_uninhabited(&self) -> bool {
        self.iter().any(syn::Field::is_uninhabited)
    }
}

impl<'a> IsUninhabited for &'a [syn::Field] {
    fn is_uninhabited(&self) -> bool {
        self.iter().any(syn::Field::is_uninhabited)
    }
}

impl IsUninhabited for syn::Field {
    fn is_uninhabited(&self) -> bool {
        self.ty.is_uninhabited()
    }
}

//==============================================================================
// Types:
//==============================================================================

impl IsUninhabited for syn::Type {
    fn is_uninhabited(&self) -> bool {
        let mut uninhabited = Uninhabited(false);
        visit::visit_type(&mut uninhabited, &self);
        uninhabited.0
    }
}

/// Tracks uninhabitedness.
struct Uninhabited(bool);

impl Uninhabited {
    /// Set to uninhabited.
    fn set(&mut self) {
        self.0 = true;
    }
}

// We are more strict than Rust is.
// Our notion of uninhabited is if the type is generatable or not.
// The second a type like *const ! is dereferenced you have UB.

impl<'ast> visit::Visit<'ast> for Uninhabited {
    //------------------------------------------------------------------
    // If we get to one of these we have a knowably uninhabited type:
    //------------------------------------------------------------------

    // The ! (never) type is obviously uninhabited:
    fn visit_type_never(&mut self, _: &'ast syn::TypeNever) {
        self.set();
    }

    // A path is uninhabited if we get one we know is uninhabited.
    // Even if `T` in `<T as Trait>::Item` is uninhabited, the associated item
    // may be inhabited, so we can't say for sure that it is uninhabited.
    fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
        const KNOWN_UNINHABITED: &[&str] =
            &["std::string::ParseError", "::std::string::ParseError"];

        if type_path.qself.is_none()
            && util::match_pathsegs(&type_path.path, KNOWN_UNINHABITED)
        {
            self.set();
        }
    }

    // An array is uninhabited iff: `[T; N]` where uninhabited(T) && N != 0
    // We want to block decent if N == 0.
    fn visit_type_array(&mut self, arr: &'ast syn::TypeArray) {
        if let Some(len) = interp::eval_expr(&arr.len) {
            if len > 0 {
                self.visit_type(&arr.elem);
            }
        }
    }

    //------------------------------------------------------------------
    // These are here to block decent:
    //------------------------------------------------------------------

    // An fn(I) -> O is never uninhabited even if I or O are:
    fn visit_type_bare_fn(&mut self, _: &'ast syn::TypeBareFn) {}

    // A macro may transform the inner type in ways we can't predict:
    fn visit_macro(&mut self, _: &'ast syn::Macro) {}

    // Both of these could be, but type is anonymous:
    fn visit_type_impl_trait(&mut self, _: &'ast syn::TypeImplTrait) {}
    fn visit_type_trait_object(&mut self, _: &'ast syn::TypeTraitObject) {}
}
