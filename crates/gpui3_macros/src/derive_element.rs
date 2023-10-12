use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;

    let mut logme = false;
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
                                    logme = true;
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
        impl #impl_generics gpui3::IntoAnyElement<#state_type> for #type_name #ty_generics
        #where_clause
        {
            fn into_any(self) -> gpui3::AnyElement<#state_type> {
                gpui3::AnyElement::new(self)
            }
        }

        impl #impl_generics gpui3::Element for #type_name #ty_generics
        #where_clause
        {
            type ViewState = #state_type;
            type ElementState = gpui3::AnyElement<#state_type>;

            fn element_id(&self) -> Option<gpui3::ElementId> {
                None
            }

            fn layout(
                &mut self,
                view_state: &mut Self::ViewState,
                element_state: Option<Self::ElementState>,
                cx: &mut gpui3::ViewContext<Self::ViewState>,
            ) -> (gpui3::LayoutId, Self::ElementState) {
                use gpui3::IntoAnyElement;

                let mut rendered_element = self.render(view_state, cx).into_any();
                let layout_id = rendered_element.layout(view_state, cx);
                (layout_id, rendered_element)
            }

            fn paint(
                &mut self,
                bounds: gpui3::Bounds<gpui3::Pixels>,
                view_state: &mut Self::ViewState,
                element_state: &mut Self::ElementState,
                cx: &mut gpui3::ViewContext<Self::ViewState>,
            ) {
                element_state.paint(view_state, None, cx)
            }
        }
    };

    if logme {
        println!(">>>>>>>>>>>>>>>>>>>>>>\n{}", gen);
    }

    gen.into()
}
