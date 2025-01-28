use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::get_simple_attribute_field;

pub fn derive_app_context(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let Some(app_variable) = get_simple_attribute_field(&ast, "app") else {
        return quote! {
            compile_error!("Derive must have an #[app] attribute to detect the &mut App field");
        }
        .into();
    };

    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui::AppContext for #type_name #type_generics
        #where_clause
        {
            type Result<T> = T;

            fn new<T: 'static>(
                &mut self,
                build_model: impl FnOnce(&mut gpui::Context<'_, T>) -> T,
            ) -> Self::Result<gpui::Entity<T>> {
                self.#app_variable.new(build_model)
            }

            fn reserve_entity<T: 'static>(&mut self) -> Self::Result<gpui::Reservation<T>> {
                self.#app_variable.reserve_entity()
            }

            fn insert_entity<T: 'static>(
                &mut self,
                reservation: gpui::Reservation<T>,
                build_model: impl FnOnce(&mut gpui::Context<'_, T>) -> T,
            ) -> Self::Result<gpui::Entity<T>> {
                self.#app_variable.insert_entity(reservation, build_model)
            }

            fn update_entity<T, R>(
                &mut self,
                handle: &gpui::Entity<T>,
                update: impl FnOnce(&mut T, &mut gpui::Context<'_, T>) -> R,
            ) -> Self::Result<R>
            where
                T: 'static,
            {
                self.#app_variable.update_entity(handle, update)
            }

            fn read_entity<T, R>(
                &self,
                handle: &gpui::Entity<T>,
                read: impl FnOnce(&T, &gpui::App) -> R,
            ) -> Self::Result<R>
            where
                T: 'static,
            {
                self.#app_variable.read_entity(handle, read)
            }

            fn update_window<T, F>(&mut self, window: gpui::AnyWindowHandle, f: F) -> gpui::Result<T>
            where
                F: FnOnce(gpui::AnyView, &mut gpui::Window, &mut gpui::App) -> T,
            {
                self.#app_variable.update_window(window, f)
            }

            fn read_window<T, R>(
                &self,
                window: &gpui::WindowHandle<T>,
                read: impl FnOnce(gpui::Entity<T>, &gpui::App) -> R,
            ) -> gpui::Result<R>
            where
                T: 'static,
            {
                self.#app_variable.read_window(window, read)
            }
        }
    };

    gen.into()
}
