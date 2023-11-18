use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;

    let mut view_state_ty = quote! { V };

    for param in &ast.generics.params {
        if let GenericParam::Type(type_param) = param {
            let type_ident = &type_param.ident;
            view_state_ty = quote! {#type_ident};
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
                                    view_state_ty = lit_str.value().parse().unwrap();
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
        impl #impl_generics gpui::Element<#view_state_ty> for #type_name #ty_generics
        #where_clause
        {
            type State = Option<gpui::AnyElement<#view_state_ty>>;

            fn element_id(&self) -> Option<gpui::ElementId> {
                None
            }

            fn layout(
                &mut self,
                view_state: &mut #view_state_ty,
                _element_state: Option<Self::State>,
                cx: &mut gpui::ViewContext<#view_state_ty>,
            ) -> (gpui::LayoutId, Self::State) {
                let mut element = self.render(view_state, cx).into_any();
                let layout_id = element.layout(view_state, cx);
                (layout_id, Some(element))
            }

            fn paint(
                self,
                _bounds: gpui::Bounds<gpui::Pixels>,
                view_state: &mut #view_state_ty,
                rendered_element: &mut Self::State,
                cx: &mut gpui::ViewContext<#view_state_ty>,
            ) {
                rendered_element.take().unwrap().paint(view_state, cx)
            }
        }
    };

    gen.into()
}
