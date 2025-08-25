use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro for the `SettingsUI` marker trait.
///
/// This macro automatically implements the `SettingsUI` trait for the annotated type.
/// The `SettingsUI` trait is a marker trait used to indicate that a type can be
/// displayed in the settings UI.
///
/// # Example
///
/// ```
/// use settings::SettingsUI;
/// use settings_ui_macros::SettingsUI;
///
/// #[derive(SettingsUI)]
/// struct MySettings {
///     enabled: bool,
///     count: usize,
/// }
/// ```
#[proc_macro_derive(SettingsUI)]
pub fn derive_settings_ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Handle generic parameters if present
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics settings::SettingsUI for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}
