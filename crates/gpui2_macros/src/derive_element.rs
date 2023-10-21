use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;

    let mut state_type = quote! { () };

    for param in &ast.generics.params {
        if let GenericParam::Type(type_param) = param {
            let type_ident = &type_param.ident;
            state_type = quote! {#type_ident};
            break;
        }
    }

    let attrs = &ast.attrs;
    for attr in attrs {
        if attr.path.is_ident("element") {
            match attr.parse_meta() {
                Ok(syn::Meta::List(i)) => {
                    for nested_meta in i.nested {
                        if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nested_meta {
                            if nv.path.is_ident("view_state") {
                                if let syn::Lit::Str(lit_str) = nv.lit {
                                    state_type = lit_str.value().parse().unwrap();
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui2::IntoAnyElement<#state_type> for #type_name #ty_generics
        #where_clause
        {
            fn into_any(self) -> gpui2::AnyElement<#state_type> {
                gpui2::AnyElement::new(self)
            }
        }

        impl #impl_generics gpui2::Element for #type_name #ty_generics
        #where_clause
        {
            type ViewState = #state_type;
            type ElementState = gpui2::AnyElement<#state_type>;

            fn id(&self) -> Option<gpui2::ElementId> {
                None
            }

            fn initialize(
                &mut self,
                view_state: &mut Self::ViewState,
                _: Option<Self::ElementState>,
                cx: &mut gpui2::ViewContext<Self::ViewState>
            ) -> Self::ElementState {
                use gpui2::IntoAnyElement;

                let mut element = self.render(view_state, cx).into_any();
                element.initialize(view_state, cx);
                element
            }

            fn layout(
                &mut self,
                view_state: &mut Self::ViewState,
                rendered_element: &mut Self::ElementState,
                cx: &mut gpui2::ViewContext<Self::ViewState>,
            ) -> gpui2::LayoutId {
                rendered_element.layout(view_state, cx)
            }

            fn paint(
                &mut self,
                bounds: gpui2::Bounds<gpui2::Pixels>,
                view_state: &mut Self::ViewState,
                rendered_element: &mut Self::ElementState,
                cx: &mut gpui2::ViewContext<Self::ViewState>,
            ) {
                rendered_element.paint(view_state, cx)
            }
        }
    };

    gen.into()
}
