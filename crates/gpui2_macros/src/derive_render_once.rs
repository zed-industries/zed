use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

pub fn derive_render_once(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;

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
    let (_, type_generics, _) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui::RenderOnce<#view_type> for #type_name #type_generics
        #where_clause
        {
            type Element = gpui::CompositeElement<#view_type, Self>;

            fn element_id(&self) -> Option<ElementId> {
                None
            }

            fn render_once(self) -> Self::Element {
                gpui::CompositeElement::new(self)
            }
        }
    };

    if type_name == "Avatar" {
        println!("{gen}");
    }

    gen.into()
}

fn specified_view_type(ast: &DeriveInput) -> Option<proc_macro2::Ident> {
    ast.attrs.iter().find_map(|attr| {
        if attr.path.is_ident("view") {
            if let Ok(syn::Meta::NameValue(meta_name_value)) = attr.parse_meta() {
                if let syn::Lit::Str(lit_str) = meta_name_value.lit {
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
}
