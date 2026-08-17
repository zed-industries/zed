use super::member::PosFnMember;
use crate::util::prelude::*;

impl super::BuilderGenCtx {
    pub(super) fn start_fn(&self) -> syn::ItemFn {
        let builder_ident = &self.builder_type.ident;
        let docs = &self.start_fn.docs;
        let vis = &self.start_fn.vis;

        let start_fn_ident = &self.start_fn.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generated code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics = self.start_fn.generics.as_ref().unwrap_or(&self.generics);

        let generics_decl = &generics.decl_without_defaults;
        let where_clause = &generics.where_clause;
        let generic_args = &self.generics.args;

        let receiver = self.receiver();

        let receiver_field_init = receiver.map(|receiver| {
            let ident = &receiver.field_ident;
            let self_token = &receiver.with_self_keyword.self_token;
            quote! {
                #ident: #self_token,
            }
        });

        let receiver = receiver.map(|receiver| {
            let mut receiver = receiver.with_self_keyword.clone();

            if receiver.reference.is_none() {
                receiver.mutability = None;
            }

            quote! { #receiver, }
        });

        let start_fn_params = self.start_fn_args().map(PosFnMember::fn_input_param);

        // Assign `start_fn_args` to intermediate variables, which may be used
        // by custom fields init expressions. This is needed only if there is
        // a conversion configured for the `start_fn` members, otherwise these
        // are already available in scope as function arguments directly.
        let start_fn_vars = self.start_fn_args().filter_map(|member| {
            let ident = &member.ident;
            let ty = &member.ty.orig;
            let conversion = member.conversion()?;

            Some(quote! {
                let #ident: #ty = #conversion;
            })
        });

        let start_fn_args_fields_idents = self.start_fn_args().map(|member| &member.ident);

        // Create custom fields in separate variables. This way custom fields
        // declared lower in the struct definition can reference custom fields
        // declared higher in their init expressions.
        let custom_fields_vars = self.custom_fields().map(|field| {
            let ident = &field.ident;
            let ty = &field.norm_ty;
            let init = field
                .init
                .as_ref()
                .map(|init| self.sanitize_expr(init))
                .unwrap_or_else(|| quote! { ::core::default::Default::default() });

            quote! {
                let #ident: #ty = #init;
            }
        });

        let custom_fields_idents = self.custom_fields().map(|field| &field.ident);

        let ide_hints = self.ide_hints();

        // `Default` trait implementation is provided only for tuples up to 12
        // elements in the standard library 😳:
        // https://github.com/rust-lang/rust/blob/67bb749c2e1cf503fee64842963dd3e72a417a3f/library/core/src/tuple.rs#L213
        let named_members_field_init =
            if self.named_members().take(13).count() <= 12 && self.const_.is_none() {
                quote!(::core::default::Default::default())
            } else {
                let none = format_ident!("None");
                let nones = self.named_members().map(|_| &none);
                quote! {
                    (#(#nones,)*)
                }
            };

        let const_ = &self.const_;
        // add the `clippy::needless_lifetimes` lint if before rust version 1.87
        // Rust version 1.87 includes a clippy change where `needless_lifetimes`
        // was split with the more complex part of the lint going to
        // `elidable_lifetime_names`. For versions since 1.87 we want to block
        // `elidable_lifetime_names` (See
        // https://github.com/elastio/bon/pull/341#discussion_r2398893516 for
        // an explanation).
        let needless_lifetime_lint = if rustversion::cfg!(before(1.87)) {
            format_ident!("needless_lifetimes")
        } else {
            format_ident!("elidable_lifetime_names")
        };

        // Construct using a span which links to our original implementation.
        // This ensures rustdoc doesn't just link every method to the macro
        // callsite.
        syn::parse_quote_spanned! {self.start_fn.span=>
            #(#docs)*
            #[inline(always)]
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We normalize `Self` references intentionally to simplify code generation
                clippy::use_self,
                // Let's keep it as non-const for now to avoid restricting ourselfves to only
                // const operations.
                clippy::missing_const_for_fn,
                clippy::#needless_lifetime_lint
            )]
            #vis #const_ fn #start_fn_ident< #(#generics_decl),* >(
                #receiver
                #(#start_fn_params,)*
            ) -> #builder_ident< #(#generic_args,)* >
            #where_clause
            {
                #ide_hints
                #( #start_fn_vars )*
                #( #custom_fields_vars )*

                #builder_ident {
                    __unsafe_private_phantom: ::core::marker::PhantomData,
                    #( #custom_fields_idents, )*
                    #receiver_field_init
                    #( #start_fn_args_fields_idents, )*
                    __unsafe_private_named: #named_members_field_init,
                }
            }
        }
    }
}
