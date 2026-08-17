use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Data;

/// Method to derive an implementation of [ActiveModelBehavior](sea_orm::ActiveModelBehavior)
pub fn expand_derive_active_model_behavior(_ident: Ident, _data: Data) -> syn::Result<TokenStream> {
    Ok(quote!(
        #[automatically_derived]
        impl sea_orm::ActiveModelBehavior for ActiveModel {}
    ))
}
