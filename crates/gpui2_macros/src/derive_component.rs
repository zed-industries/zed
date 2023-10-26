use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let specified_view_type = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("component"))
        .and_then(|attr| {
            if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested.iter().find_map(|nested| {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nested {
                        if nv.path.is_ident("view_type") {
                            if let syn::Lit::Str(lit_str) = &nv.lit {
                                return Some(
                                    lit_str
                                        .parse::<syn::Ident>()
                                        .expect("Failed to parse view_type"),
                                );
                            }
                        }
                    }
                    None
                })
            } else {
                None
            }
        });

    let view_type = specified_view_type.unwrap_or_else(|| {
        if let Some(syn::GenericParam::Type(type_param)) = generics.params.first() {
            type_param.ident.clone()
        } else {
            panic!("Expected first type parameter");
        }
    });

    let expanded = quote! {
        impl #impl_generics gpui2::Component<#view_type> for #name #ty_generics #where_clause {
            fn render(self) -> gpui2::AnyElement<#view_type> {
                (move |view_state: &mut #view_type, cx: &mut gpui2::ViewContext<'_, '_, #view_type>| self.render(view_state, cx))
                    .render()
            }
        }
    };

    TokenStream::from(expanded)
}
