use heck::ToSnakeCase;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::{GenericParam, Generics, Visibility};

/// Makes phantom data definitions so that we don't get unused template parameter errors.
pub fn make_generic_consumers(generics: &Generics) -> impl Iterator<Item = (TokenStream, Ident)> {
    generics
        .params
        .clone()
        .into_iter()
        .map(|param| match param {
            GenericParam::Type(ty) => {
                let ident = &ty.ident;
                (
                    quote! { #ident },
                    format_ident!(
                        "_consume_template_type_{}",
                        ident.to_string().to_snake_case()
                    ),
                )
            }
            GenericParam::Lifetime(lt) => {
                let lifetime = &lt.lifetime;
                let ident = &lifetime.ident;
                (
                    quote! { &#lifetime () },
                    format_ident!("_consume_template_lifetime_{}", ident),
                )
            }
            // rustc don't require constants to consume, so we just skip it. 
            GenericParam::Const(ct) => {
                let ident = ct.ident;
                (
                    quote! { () },
                    format_ident!(
                        "_comsume_template_const_parameter_{}",
                        ident.to_string().to_snake_case()
                    ),
                )
            },
        })
}

// Takes the generics parameters from the original struct and turns them into arguments.
pub fn make_generic_arguments(generics: Vec<&GenericParam>) -> Vec<TokenStream> {
    let mut arguments = Vec::new();
    for generic in generics {
        match generic {
            GenericParam::Type(typ) => {
                let ident = &typ.ident;
                arguments.push(quote! { #ident });
            }
            GenericParam::Lifetime(lt) => {
                let lifetime = &lt.lifetime;
                arguments.push(quote! { #lifetime });
            }
            GenericParam::Const(ct) => {
                let ident = &ct.ident;
                arguments.push(quote! { #ident });
            },
        }
    }
    arguments
}

pub fn uses_this_lifetime(input: TokenStream) -> bool {
    for token in input.into_iter() {
        match token {
            TokenTree::Ident(ident) => {
                if ident == "this" {
                    return true;
                }
            }
            TokenTree::Group(group) => {
                if uses_this_lifetime(group.stream()) {
                    return true;
                }
            }
            _ => (),
        }
    }
    false
}

pub fn replace_this_with_lifetime(input: TokenStream, lifetime: Ident) -> TokenStream {
    input
        .into_iter()
        .map(|token| match &token {
            TokenTree::Ident(ident) => {
                if ident == "this" {
                    TokenTree::Ident(lifetime.clone())
                } else {
                    token
                }
            }
            TokenTree::Group(group) => TokenTree::Group(Group::new(
                group.delimiter(),
                replace_this_with_lifetime(group.stream(), lifetime.clone()),
            )),
            _ => token,
        })
        .collect()
}

pub fn submodule_contents_visibility(original_visibility: &Visibility) -> Visibility {
    match original_visibility {
        // inherited: allow parent of inner submodule to see
        Visibility::Inherited => syn::parse_quote! { pub(super) },
        // restricted: add an extra super if needed
        Visibility::Restricted(ref restricted) => {
            let is_first_component_super = restricted
                .path
                .segments
                .first()
                .map(|segm| segm.ident == "super")
                .unwrap_or(false);
            if restricted.path.leading_colon.is_none() && is_first_component_super {
                let mut new_visibility = restricted.clone();
                new_visibility.in_token = Some(
                    restricted
                        .in_token
                        .unwrap_or_else(|| syn::parse_quote! { in }),
                );
                new_visibility.path.segments = std::iter::once(syn::parse_quote! { super })
                    .chain(restricted.path.segments.iter().cloned())
                    .collect();
                Visibility::Restricted(new_visibility)
            } else {
                original_visibility.clone()
            }
        }
        // others are absolute, can use them as-is
        _ => original_visibility.clone(),
    }
}

/// Functionality inspired by `Inflector`, reimplemented here to avoid the
/// `regex` dependency.
pub fn to_class_case(s: &str) -> String {
    s.split('_')
        .flat_map(|word| {
            let mut chars = word.chars();
            let first = chars.next();
            // Unicode allows for a single character to become multiple characters when converting between cases.
            first
                .into_iter()
                .flat_map(|c| c.to_uppercase())
                .chain(chars.flat_map(|c| c.to_lowercase()))
        })
        .collect()
}
