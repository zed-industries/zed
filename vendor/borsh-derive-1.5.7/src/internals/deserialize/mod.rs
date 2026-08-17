use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ExprPath, Generics, Ident, Path};

use super::{
    attributes::{field, BoundType},
    generics,
};

pub mod enums;
pub mod structs;
pub mod unions;

struct GenericsOutput {
    overrides: Vec<syn::WherePredicate>,
    default_visitor: generics::FindTyParams,
    deserialize_visitor: generics::FindTyParams,
}

impl GenericsOutput {
    fn new(generics: &Generics) -> Self {
        Self {
            overrides: vec![],
            deserialize_visitor: generics::FindTyParams::new(generics),
            default_visitor: generics::FindTyParams::new(generics),
        }
    }
    fn extend(self, where_clause: &mut syn::WhereClause, cratename: &Path) {
        let de_trait: Path = syn::parse2(quote! { #cratename::de::BorshDeserialize }).unwrap();
        let default_trait: Path = syn::parse2(quote! { core::default::Default }).unwrap();
        let de_predicates =
            generics::compute_predicates(self.deserialize_visitor.process_for_bounds(), &de_trait);
        let default_predicates =
            generics::compute_predicates(self.default_visitor.process_for_bounds(), &default_trait);
        where_clause.predicates.extend(de_predicates);
        where_clause.predicates.extend(default_predicates);
        where_clause.predicates.extend(self.overrides);
    }
}

fn process_field(
    field: &syn::Field,
    cratename: &Path,
    body: &mut TokenStream2,
    generics: &mut GenericsOutput,
) -> syn::Result<()> {
    let parsed = field::Attributes::parse(&field.attrs)?;

    generics
        .overrides
        .extend(parsed.collect_bounds(BoundType::Deserialize));
    let needs_bounds_derive = parsed.needs_bounds_derive(BoundType::Deserialize);

    let field_name = field.ident.as_ref();
    let delta = if parsed.skip {
        if needs_bounds_derive {
            generics.default_visitor.visit_field(field);
        }
        field_default_output(field_name)
    } else {
        if needs_bounds_derive {
            generics.deserialize_visitor.visit_field(field);
        }
        field_output(field_name, cratename, parsed.deserialize_with)
    };
    body.extend(delta);
    Ok(())
}

/// function which computes derive output [proc_macro2::TokenStream]
/// of code, which deserializes single field
fn field_output(
    field_name: Option<&Ident>,
    cratename: &Path,
    deserialize_with: Option<ExprPath>,
) -> TokenStream2 {
    let default_path: ExprPath =
        syn::parse2(quote! { #cratename::BorshDeserialize::deserialize_reader }).unwrap();
    let path: ExprPath = deserialize_with.unwrap_or(default_path);
    if let Some(field_name) = field_name {
        quote! {
            #field_name: #path(reader)?,
        }
    } else {
        quote! {
            #path(reader)?,
        }
    }
}

/// function which computes derive output [proc_macro2::TokenStream]
/// of code, which deserializes single skipped field
fn field_default_output(field_name: Option<&Ident>) -> TokenStream2 {
    if let Some(field_name) = field_name {
        quote! {
            #field_name: core::default::Default::default(),
        }
    } else {
        quote! { core::default::Default::default(), }
    }
}
