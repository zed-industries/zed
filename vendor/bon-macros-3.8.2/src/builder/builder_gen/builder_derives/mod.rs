mod clone;
mod debug;
mod into;
mod into_future;

use super::top_level_config::{DeriveConfig, DerivesConfig};
use super::BuilderGenCtx;
use crate::util::prelude::*;
use darling::ast::GenericParamExt;

impl BuilderGenCtx {
    pub(crate) fn builder_derives(&self) -> Result<TokenStream> {
        let DerivesConfig {
            clone,
            debug,
            into,
            into_future,
        } = &self.builder_type.derives;

        let mut tokens = TokenStream::new();

        if let Some(derive) = clone {
            tokens.extend(self.derive_clone(derive));
        }

        if let Some(derive) = debug {
            tokens.extend(self.derive_debug(derive));
        }

        if into.is_present() {
            tokens.extend(self.derive_into()?);
        }

        if let Some(derive) = into_future {
            tokens.extend(self.derive_into_future(derive)?);
        }

        Ok(tokens)
    }

    /// We follow the logic of the standard `#[derive(...)]` macros such as `Clone` and `Debug`.
    /// They add bounds of their respective traits to every generic type parameter on the struct
    /// without trying to analyze if that bound is actually required for the derive to work, so
    /// it's a conservative approach.
    ///
    /// However, the user can also override these bounds using the `bounds(...)` attribute for
    /// the specific derive.
    fn where_clause_for_derive(
        &self,
        target_trait_bounds: &TokenStream,
        derive: &DeriveConfig,
    ) -> TokenStream {
        let derive_specific_predicates = derive
            .bounds
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(|| {
                let bounds = self
                    .generics
                    .decl_without_defaults
                    .iter()
                    .filter_map(syn::GenericParam::as_type_param)
                    .map(|param| {
                        let ident = &param.ident;
                        quote! {
                            #ident: #target_trait_bounds
                        }
                    });

                quote! {
                    #( #bounds, )*
                }
            });

        let inherent_item_predicates = self.generics.where_clause_predicates();

        quote! {
            where
                #( #inherent_item_predicates, )*
                #derive_specific_predicates
        }
    }
}
