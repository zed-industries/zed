use super::member::{Member, PosFnMember};
use crate::util::prelude::*;

impl super::BuilderGenCtx {
    fn finish_fn_member_expr(&self, member: &Member) -> TokenStream {
        let member = match member {
            Member::Named(member) => member,
            Member::Skip(member) => {
                return member
                    .value
                    .as_ref()
                    .map(|value| self.sanitize_expr(value))
                    .unwrap_or_else(|| quote! { ::core::default::Default::default() });
            }
            Member::StartFn(member) => {
                let ident = &member.ident;
                return quote! { self.#ident };
            }
            Member::FinishFn(member) => {
                return member
                    .conversion()
                    .unwrap_or_else(|| member.ident.to_token_stream());
            }
            Member::Field(member) => {
                let ident = &member.ident;
                return quote! { self.#ident };
            }
        };

        let index = &member.index;

        let member_field = quote! {
            self.__unsafe_private_named.#index
        };

        let default = member
            .config
            .default
            .as_ref()
            .map(|default| default.value.as_ref());

        match default {
            Some(Some(default)) => {
                let default = if member.config.into.is_present() {
                    quote! { Into::into((|| #default)()) }
                } else {
                    quote! { #default }
                };

                // Special case for `const` because `unwrap_or_else` is not `const`
                // and closure calls aren't supported in `const` contexts at the time
                // of this writing (Rust 1.86.0).
                if self.const_.is_some() {
                    return quote! {
                        match #member_field {
                            Some(value) => value,
                            None => #default,
                        }
                    };
                }

                quote! {
                    ::core::option::Option::unwrap_or_else(#member_field, || #default)
                }
            }
            Some(None) => {
                quote! {
                    ::core::option::Option::unwrap_or_default(#member_field)
                }
            }
            None => {
                // For `Option` the default value is always `None`. So we can just return
                // the value of the member field itself (which is already an `Option<T>`).
                if member.is_special_option_ty() {
                    return member_field;
                }

                // SAFETY: we know that the member is set because we are in
                // the `finish` function where this method uses the trait
                // bound of `IsSet` for every required member. It's also
                // not possible to intervene with the builder's state from
                // the outside because all members of the builder are considered
                // private (we even generate random names for them to make it
                // impossible to access them from the outside in the same module).
                //
                // We also make sure to use fully qualified paths to methods
                // involved in setting the value for the required member to make
                // sure no trait/function in scope can override the behavior.

                // Special case for `const` mode where `unwrap_unchecked` is
                // unstable in Rust <1.83.0.
                if self.const_.is_some() {
                    return quote! {
                        match #member_field {
                            Some(value) => value,
                            // SAFETY: see the big safety comment above
                            None => unsafe { ::core::hint::unreachable_unchecked() },
                        }
                    };
                }

                quote! {
                    // SAFETY: see the big safety comment above
                    unsafe {
                        ::core::option::Option::unwrap_unchecked(#member_field)
                    }
                }
            }
        }
    }

    pub(super) fn finish_fn(&self) -> TokenStream {
        let members_vars_decls = self.members.iter().map(|member| {
            let expr = self.finish_fn_member_expr(member);
            let var_ident = member.orig_ident();

            // The type hint is necessary in some cases to assist the compiler
            // in type inference.
            //
            // For example, if the expression is passed to a function that accepts
            // an impl Trait such as `impl Default`, and the expression itself looks
            // like `Default::default()`. In this case nothing hints to the compiler
            // the resulting type of the expression, so we add a type hint via an
            // intermediate variable here.
            //
            // This variable can also be accessed by other member's `default`
            // or `skip` expressions.
            let ty = member.norm_ty();

            quote! {
                let #var_ident: #ty = #expr;
            }
        });

        let state_mod = &self.state_mod.ident;

        let finish_fn_params = self.finish_fn_args().map(PosFnMember::fn_input_param);

        let body = &self.finish_fn.body.generate(self);
        let asyncness = &self.finish_fn.asyncness;
        let unsafety = &self.finish_fn.unsafety;
        let special_attrs = &self.finish_fn.special_attrs;
        let attrs = &self.finish_fn.attrs;
        let finish_fn_vis = &self.finish_fn.vis;
        let finish_fn_ident = &self.finish_fn.ident;
        let output = &self.finish_fn.output;
        let state_var = &self.state_var;
        let const_ = &self.const_;

        // `#[target_feature]` is not compatible with `#[inline(always)]`,
        // so we need to downgrade it to `#[inline]
        let inline_attr = self
            .finish_fn
            .special_attrs
            .iter()
            .find_map(|attr| {
                attr.meta
                    .path()
                    .is_ident("target_feature")
                    .then(|| quote! { #[inline] })
            })
            .unwrap_or_else(|| quote! { #[inline(always)] });

        quote! {
            #(#attrs)*
            #inline_attr
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,

                // This lint flags any function that returns a possibly `!Send` future.
                // However, it doesn't apply in the generic context where the future is
                // `Send` if the generic parameters are `Send` as well, so we just suppress
                // this lint. See the issue: https://github.com/rust-lang/rust-clippy/issues/6947
                clippy::future_not_send,
                clippy::missing_const_for_fn,
            )]
            #(#special_attrs)*
            #finish_fn_vis #const_ #asyncness #unsafety fn #finish_fn_ident(
                self,
                #(#finish_fn_params,)*
            ) #output
            where
                #state_var: #state_mod::IsComplete
            {
                #(#members_vars_decls)*
                #body
            }
        }
    }
}
