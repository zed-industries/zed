use crate::builder::builder_gen::models::BuilderGenCtx;
use crate::util::prelude::*;

impl BuilderGenCtx {
    pub(super) fn derive_into(&self) -> Result<TokenStream> {
        if let Some(asyncness) = &self.finish_fn.asyncness {
            bail!(
                asyncness,
                "`#[builder(derive(Into))` is not supported for async functions \
                because `From::from()` method is a synchronous method"
            )
        }

        if let Some(unsafety) = &self.finish_fn.unsafety {
            bail!(
                unsafety,
                "`#[builder(derive(Into))` is not supported for unsafe functions \
                because `From::from()` method is a safe method"
            )
        }

        if let Some(arg) = self.finish_fn_args().next() {
            bail!(
                &arg.config.finish_fn.span(),
                "`#[builder(derive(Into))` is incompatible with `#[builder(finish_fn)]` members \
                because `From::from()` method accepts zero parameters"
            )
        }

        let output_ty = match &self.finish_fn.output {
            syn::ReturnType::Default => bail!(
                &self.start_fn.ident,
                "`#[builder(derive(Into))` is not supported for functions with the implicit unit return type; \
                if you have a use case where it makes sense to implement `From<Builder> for ()`, \
                please open an issue, and in the meantime annotate the function return type explicitly \
                with `-> ()`"
            ),
            syn::ReturnType::Type(_, output_ty) => output_ty.as_ref(),
        };

        let state_mod = &self.state_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;
        let builder_ident = &self.builder_type.ident;
        let state_var = &self.state_var;
        let finish_fn_ident = &self.finish_fn.ident;

        let builder_ty = quote! {
            #builder_ident<#(#generic_args,)* #state_var>
        };

        let tokens = quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::IsComplete
            >
            ::core::convert::From<#builder_ty> for #output_ty
            #where_clause
            {
                fn from(builder: #builder_ty) -> Self {
                    #builder_ident::#finish_fn_ident(builder)
                }
            }
        };

        Ok(tokens)
    }
}
