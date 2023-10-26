use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut trait_generics = ast.generics.clone();
    let view_type = if let Some(view_type) = specified_view_type(&ast) {
        quote! { #view_type }
    } else {
        if let Some(first_type_param) = ast.generics.params.iter().find_map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        }) {
            quote! { #first_type_param }
        } else {
            trait_generics.params.push(parse_quote! { V: 'static });
            quote! { V }
        }
    };

    let (impl_generics, _, where_clause) = trait_generics.split_for_impl();
    let (_, ty_generics, _) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics gpui2::Component<#view_type> for #name #ty_generics #where_clause {
            fn render(self) -> gpui2::AnyElement<#view_type> {
                (move |view_state: &mut #view_type, cx: &mut gpui2::ViewContext<'_, '_, #view_type>| self.render(view_state, cx))
                    .render()
            }
        }
    };

    if name == "AssistantPanelStory" {
        println!("Expanded tokens: {}", expanded.to_string());
    }

    TokenStream::from(expanded)
}

fn specified_view_type(ast: &DeriveInput) -> Option<proc_macro2::Ident> {
    let component_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("component"))?;

    if let Ok(syn::Meta::List(meta_list)) = component_attr.parse_meta() {
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
}
