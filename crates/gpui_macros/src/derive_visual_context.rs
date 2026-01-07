use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use super::get_simple_attribute_field;

pub fn derive_visual_context(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let Some(window_variable) = get_simple_attribute_field(&ast, "window") else {
        return quote! {
            compile_error!("Derive must have a #[window] attribute to detect the &mut Window field");
        }
        .into();
    };

    let Some(app_variable) = get_simple_attribute_field(&ast, "app") else {
        return quote! {
            compile_error!("Derive must have a #[app] attribute to detect the &mut App field");
        }
        .into();
    };

    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let r#gen = quote! {
        impl #impl_generics gpui::VisualContext for #type_name #type_generics
        #where_clause
        {
            fn window_handle(&self) -> gpui::AnyWindowHandle {
                self.#window_variable.window_handle()
            }

            fn update_window_entity<T: 'static, R>(
                &mut self,
                entity: &gpui::Entity<T>,
                update: impl FnOnce(&mut T, &mut gpui::Window, &mut gpui::Context<T>) -> R,
            ) -> Self::Result<R> {
                gpui::AppContext::update_entity(self.#app_variable, entity, |entity, cx| update(entity, self.#window_variable, cx))
            }

            fn new_window_entity<T: 'static>(
                &mut self,
                build_entity: impl FnOnce(&mut gpui::Window, &mut gpui::Context<'_, T>) -> T,
            ) -> Self::Result<gpui::Entity<T>> {
                gpui::AppContext::new(self.#app_variable, |cx| build_entity(self.#window_variable, cx))
            }

            fn replace_root_view<V>(
                &mut self,
                build_view: impl FnOnce(&mut gpui::Window, &mut gpui::Context<V>) -> V,
            ) -> Self::Result<gpui::Entity<V>>
            where
                V: 'static + gpui::Render,
            {
                self.#window_variable.replace_root(self.#app_variable, build_view)
            }

            fn focus<V>(&mut self, entity: &gpui::Entity<V>) -> Self::Result<()>
            where
                V: gpui::Focusable,
            {
                let focus_handle = gpui::Focusable::focus_handle(entity, self.#app_variable);
                self.#window_variable.focus(&focus_handle, self.#app_variable)
            }
        }
    };

    r#gen.into()
}
