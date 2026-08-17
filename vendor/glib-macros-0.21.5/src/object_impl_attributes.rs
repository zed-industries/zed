// Take a look at the license at the top of the repository in the LICENSE file.

pub mod interface;
pub mod subclass;

use proc_macro2::Span;

use crate::utils::{parse_optional_nested_meta_items, NestedMetaItem};

/// The parsing of `#[object_subclass]` and `#[object_interface]` is subtly different.
enum AttrKind {
    Interface,
    Subclass,
}

/// The parsed input for the object impl attributes..
///
/// This is used for both the `#[object_subclass]` and `#[object_interface]` attributes.
pub struct Input {
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    trait_path: syn::Path,
    self_ty: syn::Ident,
    unsafety: Option<syn::token::Unsafe>,
    items: Vec<syn::ImplItem>,
    meta_dynamic: Option<MetaDynamic>,
}

impl Input {
    /// Parse an `#[object_interface]` attribute.
    pub fn parse_interface(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse(AttrKind::Interface, input)
    }

    /// Parse an `#[object_subclass]` attribute.
    pub fn parse_subclass(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse(AttrKind::Subclass, input)
    }

    /// Parse an `#[object_subclass]` or `#[object_interface]` depending on the attribute kind.
    fn parse(kind: AttrKind, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let wrong_place_msg = match kind {
            AttrKind::Interface => {
                "This macro should be used on `impl` block for `glib::ObjectInterface` trait"
            }
            AttrKind::Subclass => {
                "This macro should be used on `impl` block for `glib::ObjectSubclass` trait"
            }
        };

        let syn::ItemImpl {
            mut attrs,
            generics,
            trait_,
            self_ty,
            unsafety,
            items,
            ..
        } = input
            .parse()
            .map_err(|_| syn::Error::new(Span::call_site(), wrong_place_msg))?;

        // The type must be a path
        let self_ty = match *self_ty {
            syn::Type::Path(syn::TypePath { path, .. }) => path.require_ident().cloned(),
            _ => Err(syn::Error::new(
                syn::spanned::Spanned::span(&self_ty),
                "expected this path to be an identifier",
            )),
        }?;

        let meta_dynamic = MetaDynamic::parse_and_remove(kind, &mut attrs)?;

        let trait_path = trait_
            .as_ref()
            .ok_or_else(|| syn::Error::new(Span::call_site(), wrong_place_msg))?
            .1
            .clone();

        Ok(Self {
            attrs,
            generics,
            trait_path,
            self_ty,
            unsafety,
            items,
            meta_dynamic,
        })
    }
}

/// A meta attribute to indicate that the class / interface is dynamic.
///
/// Depending on the object kind this can be either
/// - `#[object_subclass_dynamic]`
/// - `#[object_interface_dynamic]`
struct MetaDynamic {
    plugin_type: Option<syn::Path>,
    lazy_registration: bool,
}

impl MetaDynamic {
    /// Parse `#[object_subclass_dynamic]` / `#[object_interface_dynamic]`
    fn parse_and_remove(
        kind: AttrKind,
        attrs: &mut Vec<syn::Attribute>,
    ) -> syn::Result<Option<Self>> {
        let attr_name = match kind {
            AttrKind::Interface => "object_interface_dynamic",
            AttrKind::Subclass => "object_subclass_dynamic",
        };

        let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
        let mut lazy_registration =
            NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();

        let found = parse_optional_nested_meta_items(
            &*attrs,
            attr_name,
            &mut [&mut plugin_type, &mut lazy_registration],
        )?
        .is_some();

        if found {
            // remove attribute from the attribute list because it is not a real proc_macro_attribute
            attrs.retain(|attr| !attr.path().is_ident(attr_name));

            Ok(Some(Self {
                plugin_type: plugin_type.value,
                lazy_registration: lazy_registration.value.map(|b| b.value).unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }
}
