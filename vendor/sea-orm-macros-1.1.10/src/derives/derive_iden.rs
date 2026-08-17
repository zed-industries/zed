use heck::ToSnakeCase;
use proc_macro2::{self, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, DataEnum, DataStruct, DeriveInput, Expr, Fields, LitStr, Variant,
};

fn must_be_valid_iden(name: &str) -> bool {
    // can only begin with [a-z_]
    name.chars()
        .take(1)
        .all(|c| c == '_' || c.is_ascii_alphabetic())
        && name.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn impl_iden_for_unit_struct(
    ident: &proc_macro2::Ident,
    new_iden: &str,
) -> proc_macro2::TokenStream {
    let prepare = if must_be_valid_iden(new_iden) {
        quote! {
            fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_orm::sea_query::Quote) {
                write!(s, "{}", q.left()).unwrap();
                self.unquoted(s);
                write!(s, "{}", q.right()).unwrap();
            }
        }
    } else {
        quote! {}
    };
    quote! {
        impl sea_orm::sea_query::Iden for #ident {
            #prepare

            fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
                write!(s, #new_iden).unwrap();
            }
        }
    }
}

fn impl_iden_for_enum(
    ident: &proc_macro2::Ident,
    variants: Punctuated<Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let variants = variants.iter();
    let mut all_valid = true;

    let match_pair: Vec<TokenStream> = variants
        .map(|v| {
            let var_ident = &v.ident;
            let var_name = if var_ident == "Table" {
                ident
            } else {
                var_ident
            };
            let mut var_name = var_name.to_string().to_snake_case();
            v.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("sea_orm"))
                .try_for_each(|attr| {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("iden") {
                            let litstr: LitStr = meta.value()?.parse()?;
                            var_name = litstr.value();
                            all_valid &= must_be_valid_iden(var_name.as_str());
                        } else {
                            // Reads the value expression to advance the parse stream.
                            // Some parameters do not have any value,
                            // so ignoring an error occurred here.
                            let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                        }
                        Ok(())
                    })
                })
                .expect("something something");
            quote! { Self::#var_ident => write!(s, "{}", #var_name).unwrap() }
        })
        .collect();

    let match_arms: TokenStream = quote! { #(#match_pair),* };

    let prepare = if all_valid {
        quote! {
            fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_orm::sea_query::Quote) {
                write!(s, "{}", q.left()).unwrap();
                self.unquoted(s);
                write!(s, "{}", q.right()).unwrap();
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl sea_orm::sea_query::Iden for #ident {
            #prepare

            fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
                match self {
                    #match_arms
                };
            }
        }
    }
}

pub fn expand_derive_iden(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput { ident, data, .. } = input;

    let mut new_iden: TokenStream = ident.to_string().to_snake_case().parse().unwrap();
    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sea_orm"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("iden") {
                    let litstr: LitStr = meta.value()?.parse()?;
                    new_iden = syn::parse_str::<TokenStream>(&litstr.value())?;
                } else {
                    // Reads the value expression to advance the parse stream.
                    // Some parameters do not have any value,
                    // so ignoring an error occurred here.
                    let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                }
                Ok(())
            })
        })?;

    // Currently we only support enums and unit structs
    match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            if variants.is_empty() {
                Ok(TokenStream::new())
            } else {
                Ok(impl_iden_for_enum(&ident, variants))
            }
        }
        syn::Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => Ok(impl_iden_for_unit_struct(
            &ident,
            new_iden.to_string().as_str(),
        )),
        _ => Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive DeriveIden on unit struct or enum");
        }),
    }
}
