//! `sea-bae` is a crate for proc macro authors, which simplifies parsing of attributes. It is
//! heavily inspired by [`darling`](https://crates.io/crates/darling) but has a significantly
//! simpler API.
//!
//! ```rust
//! use sea_bae::FromAttributes;
//!
//! #[derive(
//!     Debug,
//!     Eq,
//!     PartialEq,
//!
//!     // This will add two functions:
//!     // ```
//!     // fn from_attributes(attrs: &[syn::Attribute]) -> Result<MyAttr, syn::Error>
//!     // fn try_from_attributes(attrs: &[syn::Attribute]) -> Result<Option<MyAttr>, syn::Error>
//!     // ```
//!     //
//!     // `try_from_attributes` returns `Ok(None)` if the attribute is missing, `Ok(Some(_))` if
//!     // its there and is valid, `Err(_)` otherwise.
//!     FromAttributes,
//! )]
//! pub struct MyAttr {
//!     // Anything that implements `syn::parse::Parse` is supported.
//!     mandatory_type: syn::Type,
//!     mandatory_ident: syn::Ident,
//!
//!     // Fields wrapped in `Option` are optional and default to `None` if
//!     // not specified in the attribute.
//!     optional_missing: Option<syn::Type>,
//!     optional_given: Option<syn::Type>,
//!
//!     // A "switch" is something that doesn't take arguments.
//!     // All fields with type `Option<()>` are considered swiches.
//!     // They default to `None`.
//!     switch: Option<()>,
//! }
//!
//! // `MyAttr` is now equipped to parse attributes named `my_attr`. For example:
//! //
//! //     #[my_attr(
//! //         switch,
//! //         mandatory_ident = foo,
//! //         mandatory_type = SomeType,
//! //         optional_given = OtherType,
//! //     )]
//! //     struct Foo {
//! //         ...
//! //     }
//!
//! // the input and output type would normally be `proc_macro::TokenStream` but those
//! // types cannot be used outside the compiler itself.
//! fn my_proc_macro(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
//!     let item_struct = syn::parse2::<syn::ItemStruct>(input).unwrap();
//!
//!     let my_attr = MyAttr::from_attributes(&item_struct.attrs).unwrap();
//!
//!     assert_eq!(
//!         my_attr.mandatory_type,
//!         syn::parse_str::<syn::Type>("SomeType").unwrap()
//!     );
//!
//!     assert_eq!(my_attr.optional_missing, None);
//!
//!     assert_eq!(
//!         my_attr.optional_given,
//!         Some(syn::parse_str::<syn::Type>("OtherType").unwrap())
//!     );
//!
//!     assert_eq!(my_attr.mandatory_ident, syn::parse_str::<syn::Ident>("foo").unwrap());
//!
//!     assert_eq!(my_attr.switch.is_some(), true);
//!
//!     // ...
//!     #
//!     # quote::quote! {}
//! }
//! #
//! # fn main() {
//! #     let code = quote::quote! {
//! #         #[other_random_attr]
//! #         #[my_attr(
//! #             switch,
//! #             mandatory_ident = foo,
//! #             mandatory_type = SomeType,
//! #             optional_given = OtherType,
//! #         )]
//! #         struct Foo;
//! #     };
//! #     my_proc_macro(code);
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/sea-bae/0.2.0")]
#![allow(clippy::let_and_return)]
#![deny(
    unused_variables,
    dead_code,
    unused_must_use,
    unused_imports,
    missing_docs
)]

extern crate proc_macro;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::*;
use syn::{spanned::Spanned, *};

/// See root module docs for more info.
#[proc_macro_derive(FromAttributes, attributes())]
pub fn from_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    FromAttributes::new(item)
        .expand()
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[derive(Debug)]
struct FromAttributes {
    item: ItemStruct,
    tokens: TokenStream,
}

impl FromAttributes {
    fn new(item: ItemStruct) -> Self {
        Self {
            item,
            tokens: TokenStream::new(),
        }
    }

    fn expand(mut self) -> syn::Result<TokenStream> {
        self.expand_from_attributes_method();
        self.expand_parse_impl()?;

        if std::env::var("BAE_DEBUG").is_ok() {
            eprintln!("{}", self.tokens);
        }

        Ok(self.tokens)
    }

    fn struct_name(&self) -> &Ident {
        &self.item.ident
    }

    fn attr_name(&self) -> LitStr {
        let struct_name = self.struct_name();
        let name = struct_name.to_string().to_snake_case();
        LitStr::new(&name, struct_name.span())
    }

    fn expand_from_attributes_method(&mut self) {
        let struct_name = self.struct_name();
        let attr_name = self.attr_name();

        let code = quote! {
            impl #struct_name {
                pub fn try_from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Option<Self>> {
                    use syn::spanned::Spanned;

                    for attr in attrs {
                        if attr.path().is_ident(#attr_name) {
                            return Some(attr.parse_args::<Self>()).transpose()
                        }
                    }

                    Ok(None)
                }

                pub fn from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Self> {
                    if let Some(attr) = Self::try_from_attributes(attrs)? {
                        Ok(attr)
                    } else {
                        Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            &format!("missing attribute `#[{}]`", #attr_name),
                        ))
                    }
                }
            }
        };
        self.tokens.extend(code);
    }

    fn expand_parse_impl(&mut self) -> syn::Result<()> {
        let struct_name = self.struct_name();
        let attr_name = self.attr_name();

        let variable_declarations = self.item.fields.iter().map(|field| {
            let name = &field.ident;
            quote! { let mut #name = std::option::Option::None; }
        });

        let match_arms = self
            .item
            .fields
            .iter()
            .map(|field| {
                let field_name = get_field_name(field)?;
                let pattern = LitStr::new(&field_name.to_string(), field.span());

                Ok(if field_is_switch(field)? {
                    quote! {
                        #pattern => {
                            #field_name = std::option::Option::Some(());
                        }
                    }
                } else {
                    quote! {
                        #pattern => {
                            input.parse::<syn::Token![=]>()?;
                            #field_name = std::option::Option::Some(input.parse()?);
                        }
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let mut unwrap_mandatory_fields = Vec::new();
        for field in &self.item.fields {
            if field_is_optional(field)? {
                continue;
            }

            let field_name = get_field_name(field)?;
            let arg_name = LitStr::new(&field_name.to_string(), field.span());

            unwrap_mandatory_fields.push(quote! {
                let #field_name = if let std::option::Option::Some(#field_name) = #field_name {
                    #field_name
                } else {
                    return syn::Result::Err(
                        input.error(
                            &format!("`#[{}]` is missing `{}` argument", #attr_name, #arg_name),
                        )
                    );
                };
            });
        }

        let set_fields = self
            .item
            .fields
            .iter()
            .map(|field| {
                let field_name = get_field_name(field)?;
                Ok(quote! { #field_name, })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let mut supported_args = self
            .item
            .fields
            .iter()
            .map(|field| get_field_name(field).map(|field_name| format!("`{}`", field_name)))
            .collect::<syn::Result<Vec<_>>>()?;
        supported_args.sort_unstable();
        let supported_args = supported_args.join(", ");

        let code = quote! {
            impl syn::parse::Parse for #struct_name {
                #[allow(unreachable_code, unused_imports, unused_variables)]
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    #(#variable_declarations)*

                    while !input.is_empty() {
                        let bae_attr_ident = input.parse::<syn::Ident>()?;

                        match &*bae_attr_ident.to_string() {
                            #(#match_arms)*
                            other => {
                                return syn::Result::Err(
                                    syn::Error::new(
                                        bae_attr_ident.span(),
                                        &format!(
                                            "`#[{}]` got unknown `{}` argument. Supported arguments are {}",
                                            #attr_name,
                                            other,
                                            #supported_args,
                                        ),
                                    )
                                );
                            }
                        }

                        input.parse::<syn::Token![,]>().ok();
                    }

                    #(#unwrap_mandatory_fields)*

                    syn::Result::Ok(Self { #(#set_fields)* })
                }
            }
        };
        self.tokens.extend(code);

        Ok(())
    }
}

fn get_field_name(field: &Field) -> syn::Result<&Ident> {
    field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new(field.span(), "Field without a name"))
}

fn field_is_optional(field: &Field) -> syn::Result<bool> {
    let type_path = if let Type::Path(type_path) = &field.ty {
        type_path
    } else {
        return Ok(false);
    };

    let ident = &type_path
        .path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new(field.span(), "Empty type path"))?
        .ident;

    Ok(ident == "Option")
}

fn field_is_switch(field: &Field) -> syn::Result<bool> {
    let unit_type = syn::parse_str::<Type>("()").unwrap();
    Ok(inner_type(&field.ty)? == Some(&unit_type))
}

fn inner_type(ty: &Type) -> syn::Result<Option<&Type>> {
    let type_path = if let Type::Path(type_path) = ty {
        type_path
    } else {
        return Ok(None);
    };

    let ty_args = &type_path
        .path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new(ty.span(), "Empty type path"))?
        .arguments;

    let ty_args = if let PathArguments::AngleBracketed(ty_args) = ty_args {
        ty_args
    } else {
        return Ok(None);
    };

    let generic_arg = ty_args
        .args
        .last()
        .ok_or_else(|| syn::Error::new(ty_args.span(), "Empty generic argument"))?;

    let ty = if let GenericArgument::Type(ty) = generic_arg {
        ty
    } else {
        return Ok(None);
    };

    Ok(Some(ty))
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_ui() {
        let t = trybuild::TestCases::new();
        t.pass("tests/compile_pass/*.rs");
        t.compile_fail("tests/compile_fail/*.rs");
    }
}
