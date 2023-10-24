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
use quote::{quote, format_ident};
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

        let trampoline_name = format_ident!("{}{}", name, "Trampoline");

        quote! {
            struct #trampoline_name<S, E> {
                contents: Option<E>,
                phantom: std::marker::PhantomData<S>,
            }

            impl<S, E> #trampoline_name<S, E> {
                fn new(contents: E) -> Self {
                    IntoAnyElementTrampolineName {
                        contents: Some(contents),
                        phantom: std::marker::PhantomData,
                    }
                }
            }

            impl<S, E> Element for #trampoline_name<S, E> {
                type ViewState = S;

                type ElementState = AnyElement<S>;

                fn id(&self) -> Option<crate::ElementId> {
                    None
                }

                fn initialize(
                    &mut self,
                    view_state: &mut Self::ViewState,
                    element_state: Option<Self::ElementState>,
                    cx: &mut ViewContext<Self::ViewState>,
                ) -> Self::ElementState {
                    self.contents.take().unwrap().render(cx)
                }

                fn layout(
                    &mut self,
                    view_state: &mut Self::ViewState,
                    element_state: &mut Self::ElementState,
                    cx: &mut ViewContext<Self::ViewState>,
                ) -> crate::LayoutId {
                    element_state.layout(view_state, cx)
                }

                fn paint(
                    &mut self,
                    bounds: crate::Bounds<crate::Pixels>,
                    view_state: &mut Self::ViewState,
                    element_state: &mut Self::ElementState,
                    cx: &mut ViewContext<Self::ViewState>,
                ) {
                    element_state.paint(view_state, cx);
                }
            }

            impl<S, E> IntoAnyElement<S> for IntoAnyElementTrampolineName<S, E> {
                fn into_any(self) -> AnyElement<S> {
                    AnyElement::new(self)
                }
            }

            impl #impl_generics gpui2::IntoAnyElement<ViewState> for #name #ty_generics
            #where_clause
            {
                fn into_any(self) -> gpui2::AnyElement<ViewState> {
                    #trampoline_name(self).into_any()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
