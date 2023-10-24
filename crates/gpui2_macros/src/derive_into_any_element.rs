// impl<V> IntoAnyElement<V> for Modal
// where
//     V: 'static + Send + Sync,
// {
//     fn into_any(self) -> AnyElement<V> {
//         self.render().into_any()
//     }
// }

// name the function pub fn derive_into_any_element

// Defining a derive macro
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_into_any_element(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    // Check for the presence of a #[view_type = Foo] attribute on the struct
    let mut view_type = None;
    for attr in &input.attrs {
        if attr.path.is_ident("view_type") {
            if let Ok(meta) = attr.parse_meta() {
                if let syn::Meta::NameValue(nv) = meta {
                    if let syn::Lit::Str(lit) = nv.lit {
                        let view_type_token: proc_macro2::TokenStream =
                            syn::parse_str(&lit.value()).unwrap();
                        view_type = Some(view_type_token);
                        break;
                    }
                }
            }
        }
    }

    let expanded = if let Some(view_type) = view_type {
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics gpui2::IntoAnyElement<#view_type> for #name #ty_generics
            #where_clause
            {
                fn into_any(self) -> gpui2::AnyElement<#view_type> {
                    Self::render(self).into_any()
                }
            }
        }
    } else {
        let mut trait_generics = generics.clone();
        trait_generics.params.push(syn::parse_quote! {
            ViewState: 'static + Send + Sync
        });

        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, _, where_clause) = trait_generics.split_for_impl();

        quote! {
            impl #impl_generics gpui2::IntoAnyElement<ViewState> for #name #ty_generics
            #where_clause
            {
                fn into_any(self) -> gpui2::AnyElement<ViewState> {
                    Self::render(self).into_any()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
