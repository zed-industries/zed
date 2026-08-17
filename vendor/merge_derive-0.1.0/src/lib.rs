// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! A derive macro for the [`merge::Merge`][] trait.
//!
//! See the documentation for the [`merge`][] crate for more information.
//!
//! [`merge`]: https://lib.rs/crates/merge
//! [`merge::Merge`]: https://docs.rs/merge/latest/merge/trait.Merge.html

extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site, dummy::set_dummy, proc_macro_error, ResultExt};
use quote::{quote, quote_spanned};
use syn::Token;

struct Field {
    name: syn::Member,
    span: proc_macro2::Span,
    attrs: FieldAttrs,
}

#[derive(Default)]
struct FieldAttrs {
    skip: bool,
    strategy: Option<syn::Path>,
}

enum FieldAttr {
    Skip,
    Strategy(syn::Path),
}

#[proc_macro_derive(Merge, attributes(merge))]
#[proc_macro_error]
pub fn merge_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_merge(&ast).into()
}

fn impl_merge(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    set_dummy(quote! {
        impl ::merge::Merge for #name {
            fn merge(&mut self, other: Self) {
                unimplemented!()
            }
        }
    });

    if let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = ast.data {
        impl_merge_for_struct(name, fields)
    } else {
        abort_call_site!("merge::Merge can only be derived for structs")
    }
}

fn impl_merge_for_struct(name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let assignments = gen_assignments(fields);

    quote! {
        impl ::merge::Merge for #name {
            fn merge(&mut self, other: Self) {
                #assignments
            }
        }
    }
}

fn gen_assignments(fields: &syn::Fields) -> TokenStream {
    let fields = fields.iter().enumerate().map(Field::from);
    let assignments = fields.filter(|f| !f.attrs.skip).map(|f| gen_assignment(&f));
    quote! {
        #( #assignments )*
    }
}

fn gen_assignment(field: &Field) -> TokenStream {
    use syn::spanned::Spanned;

    let name = &field.name;
    if let Some(strategy) = &field.attrs.strategy {
        quote_spanned!(strategy.span()=> #strategy(&mut self.#name, other.#name);)
    } else {
        quote_spanned!(field.span=> ::merge::Merge::merge(&mut self.#name, other.#name);)
    }
}

impl From<(usize, &syn::Field)> for Field {
    fn from(data: (usize, &syn::Field)) -> Self {
        use syn::spanned::Spanned;

        let (index, field) = data;
        Field {
            name: if let Some(ident) = &field.ident {
                syn::Member::Named(ident.clone())
            } else {
                syn::Member::Unnamed(index.into())
            },
            span: field.span(),
            attrs: field.attrs.iter().into(),
        }
    }
}

impl FieldAttrs {
    fn apply(&mut self, attr: FieldAttr) {
        match attr {
            FieldAttr::Skip => self.skip = true,
            FieldAttr::Strategy(path) => self.strategy = Some(path),
        }
    }
}

impl<'a, I: Iterator<Item = &'a syn::Attribute>> From<I> for FieldAttrs {
    fn from(iter: I) -> Self {
        let mut field_attrs = Self::default();

        for attr in iter {
            if !attr.path.is_ident("merge") {
                continue;
            }

            let parser = syn::punctuated::Punctuated::<FieldAttr, Token![,]>::parse_terminated;
            for attr in attr.parse_args_with(parser).unwrap_or_abort() {
                field_attrs.apply(attr);
            }
        }

        field_attrs
    }
}

impl syn::parse::Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name: syn::Ident = input.parse()?;
        if name == "skip" {
            // TODO check remaining stream
            Ok(FieldAttr::Skip)
        } else if name == "strategy" {
            let _: Token![=] = input.parse()?;
            let path: syn::Path = input.parse()?;
            Ok(FieldAttr::Strategy(path))
        } else {
            abort!(name, "Unexpected attribute: {}", name)
        }
    }
}
