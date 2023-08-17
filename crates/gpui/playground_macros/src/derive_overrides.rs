extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

/// When deriving Overrides on a struct Foo, builds a new struct FooOverrides
/// that implements the Overrides trait so it can be applied to Foo.
pub fn derive_overrides(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let crate_name: String = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("overrides_crate") {
                match attr.parse_meta() {
                    Ok(Meta::NameValue(nv)) => {
                        if let Lit::Str(s) = nv.lit {
                            Some(s.value())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or_else(|| String::from("playground"));
    let crate_name = format_ident!("{}", crate_name);

    let ident = input.ident;
    let new_ident = syn::Ident::new(&format!("{}Overrides", ident), ident.span());
    let data = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Override can only be derived for structs"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Override can only be derived for structs with named fields"),
    };

    let new_fields = fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;

            if let syn::Type::Path(typepath) = ty {
                if typepath.path.segments.last().unwrap().ident == "Option" {
                    return quote! { #name: #ty };
                }
            }
            quote! { #name: Option<#ty> }
        })
        .collect::<Vec<_>>();

    let names = fields.iter().map(|f| &f.ident);
    let is_some_implementation = names.clone().map(|name| {
        quote! {
            self.#name.is_some()
        }
    });

    let apply_implementation = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if let syn::Type::Path(typepath) = ty {
            if typepath.path.segments.last().unwrap().ident == "Option" {
                return quote! {
                    base.#name = self.#name.clone();
                };
            }
        }

        quote! {
            if let Some(value) = &self.#name {
                base.#name = value.clone();
            }
        }
    });

    let default_implementation = names.map(|name| {
        quote! {
            #name: None,
        }
    });

    let expanded = quote! {
        pub struct #new_ident {
            #(#new_fields,)*
        }

        impl #crate_name::style::Overrides for #new_ident {
            type Base = #ident;

            fn is_some(&self) -> bool {
                #(#is_some_implementation)||*
            }

            fn apply(&self, base: &mut Self::Base) {
                #(#apply_implementation)*
            }
        }

        impl Default for #new_ident {
            fn default() -> Self {
                Self {
                    #(#default_implementation)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
