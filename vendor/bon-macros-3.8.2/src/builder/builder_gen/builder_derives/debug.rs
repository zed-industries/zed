use crate::builder::builder_gen::member::Member;
use crate::builder::builder_gen::models::BuilderGenCtx;
use crate::builder::builder_gen::top_level_config::DeriveConfig;
use crate::util::prelude::*;

impl BuilderGenCtx {
    pub(super) fn derive_debug(&self, derive: &DeriveConfig) -> TokenStream {
        let bon = &self.bon;

        let format_members = self.members.iter().filter_map(|member| {
            match member {
                Member::StartFn(member) => {
                    let member_ident = &member.ident;
                    let member_ident_str = member_ident.to_string();
                    let member_ty = &member.ty.norm;
                    Some(quote! {
                        output.field(
                            #member_ident_str,
                            #bon::__::better_errors::as_dyn_debug::<#member_ty>(
                                &self.#member_ident
                            )
                        );
                    })
                }
                Member::Field(member) => {
                    let member_ident = &member.ident;
                    let member_ident_str = member_ident.to_string();
                    let member_ty = &member.norm_ty;
                    Some(quote! {
                        output.field(
                            #member_ident_str,
                            #bon::__::better_errors::as_dyn_debug::<#member_ty>(
                                &self.#member_ident
                            )
                        );
                    })
                }
                Member::Named(member) => {
                    let member_index = &member.index;
                    let member_ident_str = &member.name.snake_raw_str;
                    let member_ty = member.underlying_norm_ty();
                    Some(quote! {
                        if let Some(value) = &self.__unsafe_private_named.#member_index {
                            output.field(
                                #member_ident_str,
                                #bon::__::better_errors::as_dyn_debug::<#member_ty>(value)
                            );
                        }
                    })
                }

                // The values for these members are computed only in the finishing
                // function where the builder is consumed, and they aren't stored
                // in the builder itself.
                Member::FinishFn(_) | Member::Skip(_) => None,
            }
        });

        let format_receiver = self.receiver().map(|receiver| {
            let ident = &receiver.field_ident;
            let ident_str = ident.to_string();
            let ty = &receiver.without_self_keyword;
            quote! {
                output.field(
                    #ident_str,
                    #bon::__::better_errors::as_dyn_debug::<#ty>(
                        &self.#ident
                    )
                );
            }
        });

        let debug = quote!(::core::fmt::Debug);
        let where_clause = self.where_clause_for_derive(&debug, derive);
        let state_mod = &self.state_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let state_var = &self.state_var;
        let builder_ident_str = builder_ident.to_string();

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::State
            >
            #debug for #builder_ident<
                #(#generic_args,)*
                #state_var
            >
            #where_clause
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut output = f.debug_struct(#builder_ident_str);

                    #format_receiver
                    #(#format_members)*

                    output.finish()
                }
            }
        }
    }
}
