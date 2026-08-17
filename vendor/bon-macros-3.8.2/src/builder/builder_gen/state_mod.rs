use super::BuilderGenCtx;
use crate::util::prelude::*;

pub(super) struct StateModGenCtx<'a> {
    base: &'a BuilderGenCtx,
    stateful_members_snake: Vec<&'a syn::Ident>,
    stateful_members_pascal: Vec<&'a syn::Ident>,
    sealed_item_decl: TokenStream,
    sealed_item_impl: TokenStream,
}

impl<'a> StateModGenCtx<'a> {
    pub(super) fn new(builder_gen: &'a BuilderGenCtx) -> Self {
        Self {
            base: builder_gen,

            stateful_members_snake: builder_gen
                .stateful_members()
                .map(|member| &member.name.snake)
                .collect(),

            stateful_members_pascal: builder_gen
                .stateful_members()
                .map(|member| &member.name.pascal)
                .collect(),

            // A const item in a trait makes it non-object safe, which is convenient,
            // because we want that restriction in this case.
            sealed_item_decl: quote! {
                #[doc(hidden)]
                const SEALED: sealed::Sealed;
            },

            sealed_item_impl: quote! {
                const SEALED: sealed::Sealed = sealed::Sealed;
            },
        }
    }

    pub(super) fn state_mod(&self) -> TokenStream {
        let bon = &self.base.bon;
        let vis = &self.base.state_mod.vis;
        let vis_child = &self.base.state_mod.vis_child;
        let vis_child_child = &self.base.state_mod.vis_child_child;

        let state_mod_docs = &self.base.state_mod.docs;
        let state_mod_ident = &self.base.state_mod.ident;

        let state_trait = self.state_trait();
        let is_complete_trait = self.is_complete_trait();
        let members_names_mod = self.members_names_mod();
        let state_transitions = self.state_transitions();

        quote! {
            #[allow(
                // These are intentional. By default, the builder module is private
                // and can't be accessed outside of the module where the builder
                // type is defined. This makes the builder type "anonymous" to
                // the outside modules, which is a good thing if users don't want
                // to expose this API surface.
                //
                // Also, there are some genuinely private items like the `Sealed`
                // enum and members "name" enums that we don't want to expose even
                // to the module that defines the builder. These APIs are not
                // public, and users instead should only reference the traits
                // and state transition type aliases from here.
                unnameable_types, unreachable_pub, clippy::redundant_pub_crate
            )]
            #( #state_mod_docs )*
            #vis mod #state_mod_ident {
                #[doc(inline)]
                #vis_child use #bon::__::{IsSet, IsUnset};
                use #bon::__::{Set, Unset};

                mod sealed {
                    #vis_child_child struct Sealed;
                }

                #state_trait
                #is_complete_trait
                #members_names_mod
                #state_transitions
            }
        }
    }

    fn state_transitions(&self) -> TokenStream {
        // Not using `Iterator::zip` here to make it possible to scale this in
        // case if we add more vecs here. We are not using `Itertools`, so
        // its `multiunzip` is not available.
        let mut set_members_structs = Vec::with_capacity(self.stateful_members_snake.len());
        let mut state_impls = Vec::with_capacity(self.stateful_members_snake.len());

        let vis_child = &self.base.state_mod.vis_child;
        let sealed_item_impl = &self.sealed_item_impl;

        for member in self.base.stateful_members() {
            let member_pascal = &member.name.pascal;

            let docs = format!(
                "Represents a [`State`] that has [`IsSet`] implemented for [`State::{member_pascal}`].\n\n\
                The state for all other members is left the same as in the input state.",
            );

            let struct_ident = format_ident!("Set{}", member.name.pascal_str);

            set_members_structs.push(quote! {
                #[doc = #docs]
                #vis_child struct #struct_ident<S: State = Empty>(
                    // We `S` in an `fn() -> ...` to make the compiler think
                    // that the builder doesn't "own" an instance of `S`.
                    // This removes unnecessary requirements when evaluating the
                    // applicability of the auto traits.
                    ::core::marker::PhantomData<fn() -> S>
                );
            });

            let states = self.base.stateful_members().map(|other_member| {
                if other_member.is(member) {
                    let member_snake = &member.name.snake;
                    quote! {
                        Set<members::#member_snake>
                    }
                } else {
                    let member_pascal = &other_member.name.pascal;
                    quote! {
                        S::#member_pascal
                    }
                }
            });

            let stateful_members_pascal = &self.stateful_members_pascal;

            state_impls.push(quote! {
                #[doc(hidden)]
                impl<S: State> State for #struct_ident<S> {
                    #(
                        type #stateful_members_pascal = #states;
                    )*
                    #sealed_item_impl
                }
            });
        }

        let stateful_members_snake = &self.stateful_members_snake;
        let stateful_members_pascal = &self.stateful_members_pascal;

        quote! {
            /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
            ///
            /// This is the initial state of the builder before any setters are called.
            #vis_child struct Empty(());

            #( #set_members_structs )*

            #[doc(hidden)]
            impl State for Empty {
                #(
                    type #stateful_members_pascal = Unset<members::#stateful_members_snake>;
                )*
                #sealed_item_impl
            }

            #( #state_impls )*

        }
    }

    fn state_trait(&self) -> TokenStream {
        let assoc_types_docs = self.stateful_members_snake.iter().map(|member_snake| {
            format!(
                "Type state of the member `{member_snake}`.\n\
                \n\
                It can implement either [`IsSet`] or [`IsUnset`]",
            )
        });

        let vis_child = &self.base.state_mod.vis_child;
        let sealed_item_decl = &self.sealed_item_decl;
        let stateful_members_pascal = &self.stateful_members_pascal;

        let docs_suffix = if stateful_members_pascal.is_empty() {
            ""
        } else {
            "\n\n\
            You can use the associated types of this trait to control the state of individual members \
            with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with \
            the `Set*` structs available in this module."
        };

        let docs = format!(
            "Builder's type state specifies if members are set or not (unset).{docs_suffix}"
        );

        quote! {
            #[doc = #docs]
            #vis_child trait State: ::core::marker::Sized {
                #(
                    #[doc = #assoc_types_docs]
                    type #stateful_members_pascal;
                )*
                #sealed_item_decl
            }
        }
    }

    fn is_complete_trait(&self) -> TokenStream {
        let required_members_pascal = self
            .base
            .named_members()
            .filter(|member| member.is_required())
            .map(|member| &member.name.pascal)
            .collect::<Vec<_>>();

        // Associated types bounds syntax that provides implied bounds for them
        // is available only since Rust 1.79.0. So this is an opt-in feature that
        // bumps the MSRV of the crate. See more details in the comment on this
        // cargo feature's declaration in `bon/Cargo.toml`.
        let maybe_assoc_type_bounds = cfg!(feature = "implied-bounds").then(|| {
            quote! {
                < #( #required_members_pascal: IsSet, )* >
            }
        });

        let vis_child = &self.base.state_mod.vis_child;
        let sealed_item_decl = &self.sealed_item_decl;
        let sealed_item_impl = &self.sealed_item_impl;

        let builder_ident = &self.base.builder_type.ident;
        let finish_fn = &self.base.finish_fn.ident;

        let docs = format!(
            "Marker trait that indicates that all required members are set.\n\n\
            In this state, you can finish building by calling the method \
            [`{builder_ident}::{finish_fn}()`](super::{builder_ident}::{finish_fn}())",
        );

        quote! {
            #[doc = #docs]
            #vis_child trait IsComplete: State #maybe_assoc_type_bounds {
                #sealed_item_decl
            }

            #[doc(hidden)]
            impl<S: State> IsComplete for S
            where
                #(
                    S::#required_members_pascal: IsSet,
                )*
            {
                #sealed_item_impl
            }
        }
    }

    fn members_names_mod(&self) -> TokenStream {
        let vis_child_child = &self.base.state_mod.vis_child_child;
        let stateful_members_snake = &self.stateful_members_snake;

        // The message is defined separately to make it single-line in the
        // generated code. This simplifies the task of removing unnecessary
        // attributes from the generated code when preparing for demo purposes.
        let deprecated_msg = "\
            this should not be used directly; it is an implementation detail; \
            use the Set* type aliases to control the \
            state of members instead";

        quote! {
            #[deprecated = #deprecated_msg]
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            mod members {
                #(
                    #vis_child_child struct #stateful_members_snake(());
                )*
            }
        }
    }
}
