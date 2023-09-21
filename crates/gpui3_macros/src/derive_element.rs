use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam};

pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;
    let mut state_type: Option<Ident> = None;

    for param in ast.generics.params.iter() {
        if let GenericParam::Type(type_param) = param {
            state_type = Some(type_param.ident.clone())
        }
    }

    let state_type_name = state_type.unwrap_or_else(|| parse_quote! { () });
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui3::Element for #type_name #ty_generics
        #where_clause
        {
            type State = #state_type_name;
            type FrameState = gpui3::AnyElement<#state_type_name>;

            fn layout(
                &mut self,
                state: &mut #state_type_name,
                cx: &mut gpui3::ViewContext<V>,
            ) -> anyhow::Result<(gpui3::LayoutId, Self::FrameState)> {
                let mut rendered_element = self.render(state, cx).into_element().into_any();
                let layout_id = rendered_element.layout(state, cx)?;
                Ok((layout_id, rendered_element))
            }

            fn paint(
                &mut self,
                layout: &gpui3::Layout,
                state: &mut #state_type_name,
                rendered_element: &mut Self::FrameState,
                cx: &mut gpui3::ViewContext<V>,
            ) {
                rendered_element.paint(layout.origin, state, cx);
            }
        }
    };

    gen.into()
}
