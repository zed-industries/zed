use super::config::{BlanketParamName, EvalBlanketFlagParam};
use super::{NamedMember, PosFnMember};
use crate::builder::builder_gen::top_level_config::OnConfig;
use crate::util::prelude::*;

impl NamedMember {
    pub(super) fn merge_config_into(&mut self, on: &[OnConfig]) -> Result {
        // `with` is mutually exclusive with `into`. So there is nothing to merge here
        // if `with` is present.
        if self.config.with.is_some() {
            return Ok(());
        }

        // For optional named members the target of the `Into` conversion is the type
        // inside of the `Option<T>`, not the `Option<T>` itself because we generate
        // a setter that accepts `T` itself. It also makes this logic stable regardless
        // if `Option<T>` is used or the member of type `T` has `#[builder(default)]` on it.
        let scrutinee = self.underlying_orig_ty();

        self.config.into = EvalBlanketFlagParam {
            on,
            param_name: BlanketParamName::Into,
            member_config: &self.config,
            scrutinee,
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }
}

impl PosFnMember {
    pub(crate) fn merge_config_into(&mut self, on: &[OnConfig]) -> Result {
        // Positional members are never optional. Users must always specify them, so there
        // is no need for us to look into the `Option<T>` generic parameter, because the
        // `Option<T>` itself is the target of the into conversion, not the `T` inside it.
        let scrutinee = self.ty.orig.as_ref();

        self.config.into = EvalBlanketFlagParam {
            on,
            param_name: BlanketParamName::Into,
            member_config: &self.config,
            scrutinee,
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }

    pub(crate) fn fn_input_param(&self) -> TokenStream {
        let ty = &self.ty.norm;
        let ident = &self.ident;

        if self.config.into.is_present() {
            quote! { #ident: impl Into<#ty> }
        } else {
            quote! { #ident: #ty }
        }
    }

    pub(crate) fn conversion(&self) -> Option<TokenStream> {
        if !self.config.into.is_present() {
            return None;
        }

        let ident = &self.ident;

        Some(quote! { Into::into(#ident) })
    }
}
