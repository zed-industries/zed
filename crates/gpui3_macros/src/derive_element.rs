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
        }
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
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
                state: &mut #state_type,
                element_state: Option<Self::ElementState>,
                cx: &mut gpui3::ViewContext<Self::ViewState>,
            ) -> (gpui3::LayoutId, Self::ElementState) {
                use gpui3::IntoAnyElement;

                let mut rendered_element = self.render(cx).into_any();
                let layout_id = rendered_element.layout(state, cx);
                (layout_id, rendered_element)
            }

            fn paint(
                &mut self,
                bounds: gpui3::Bounds<gpui3::Pixels>,
                state: &mut Self::ViewState,
                element_state: &mut Self::ElementState,
                cx: &mut gpui3::ViewContext<Self::ViewState>,
            ) {
                // TODO: Where do we get the `offset` from?
                element_state.paint(state, None, cx)
            }
        }
    };

    gen.into()
}
