use super::BuilderGenCtx;
use crate::builder::builder_gen::top_level_config::DeriveConfig;
use crate::util::prelude::*;

impl BuilderGenCtx {
    pub(super) fn derive_clone(&self, derive: &DeriveConfig) -> TokenStream {
        let bon = &self.bon;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;

        let clone = quote!(::core::clone::Clone);

        let clone_receiver = self.receiver().map(|receiver| {
            let ident = &receiver.field_ident;
            let ty = &receiver.without_self_keyword;
            quote! {
                #ident: <#ty as #clone>::clone(&self.#ident),
            }
        });

        let clone_start_fn_args = self.start_fn_args().map(|member| {
            let member_ident = &member.ident;
            let member_ty = &member.ty.norm;

            quote! {
                // The type hint here is necessary to get better error messages
                // that point directly to the type that doesn't implement `Clone`
                // in the input code using the span info from the type hint.
                #member_ident: <#member_ty as #clone>::clone(&self.#member_ident)
            }
        });

        let where_clause = self.where_clause_for_derive(&clone, derive);
        let state_mod = &self.state_mod.ident;

        let clone_named_members = self.named_members().map(|member| {
            let member_index = &member.index;

            // The type hint here is necessary to get better error messages
            // that point directly to the type that doesn't implement `Clone`
            // in the input code using the span info from the type hint.
            let ty = member.underlying_norm_ty();

            quote! {
                #bon::__::better_errors::clone_member::<#ty>(
                    &self.__unsafe_private_named.#member_index
                )
            }
        });

        let clone_fields = self.custom_fields().map(|member| {
            let member_ident = &member.ident;
            let member_ty = &member.norm_ty;

            quote! {
                // The type hint here is necessary to get better error messages
                // that point directly to the type that doesn't implement `Clone`
                // in the input code using the span info from the type hint.
                #member_ident: <#member_ty as #clone>::clone(&self.#member_ident)
            }
        });

        let state_var = &self.state_var;

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::State
            >
            #clone for #builder_ident<
                #(#generic_args,)*
                #state_var
            >
            #where_clause
            {
                fn clone(&self) -> Self {
                    Self {
                        __unsafe_private_phantom: ::core::marker::PhantomData,
                        #clone_receiver
                        #( #clone_start_fn_args, )*
                        #( #clone_fields, )*

                        // We clone named members individually instead of cloning
                        // the entire tuple to improve error messages in case if
                        // one of the members doesn't implement `Clone`. This avoids
                        // a sentence that say smth like
                        // ```
                        // required for `(...big type...)` to implement `Clone`
                        // ```
                        __unsafe_private_named: ( #( #clone_named_members, )* ),
                    }
                }
            }
        }
    }
}
