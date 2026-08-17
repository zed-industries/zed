mod builder_decl;
mod builder_derives;
mod finish_fn;
mod getters;
mod member;
mod models;
mod setters;
mod start_fn;
mod state_mod;
mod top_level_config;

pub(crate) mod input_fn;
pub(crate) mod input_struct;
pub(crate) use top_level_config::TopLevelConfig;

use crate::util::prelude::*;
use getters::GettersCtx;
use member::{CustomField, Member, MemberOrigin, NamedMember, PosFnMember, RawMember};
use models::{AssocMethodCtxParams, AssocMethodReceiverCtx, BuilderGenCtx, FinishFnBody, Generics};
use setters::SettersCtx;

pub(crate) struct MacroOutput {
    pub(crate) start_fn: syn::ItemFn,
    pub(crate) other_items: TokenStream,
}

impl BuilderGenCtx {
    fn receiver(&self) -> Option<&AssocMethodReceiverCtx> {
        self.assoc_method_ctx.as_ref()?.receiver.as_ref()
    }

    fn named_members(&self) -> impl Iterator<Item = &NamedMember> {
        self.members.iter().filter_map(Member::as_named)
    }

    fn custom_fields(&self) -> impl Iterator<Item = &CustomField> {
        self.members.iter().filter_map(Member::as_field)
    }

    fn start_fn_args(&self) -> impl Iterator<Item = &PosFnMember> {
        self.members.iter().filter_map(Member::as_start_fn)
    }

    fn finish_fn_args(&self) -> impl Iterator<Item = &PosFnMember> {
        self.members.iter().filter_map(Member::as_finish_fn)
    }

    fn stateful_members(&self) -> impl Iterator<Item = &NamedMember> {
        self.named_members().filter(|member| member.is_stateful())
    }

    pub(crate) fn output(self) -> Result<MacroOutput> {
        let mut start_fn = self.start_fn();
        let state_mod = state_mod::StateModGenCtx::new(&self).state_mod();
        let builder_decl = self.builder_decl();
        let builder_impl = self.builder_impl()?;
        let builder_derives = self.builder_derives()?;

        let default_allows = syn::parse_quote!(#[allow(
            // We have a `deprecated` lint on all `bon::__` items which we
            // use in the generated code extensively
            deprecated
        )]);

        let allows = self.allow_attrs.iter().cloned().chain([default_allows]);

        // -- Postprocessing --
        // Here we parse all items back and add the `allow` attributes to them.
        let other_items = quote! {
            #builder_decl
            #builder_impl
            #builder_derives
            #state_mod
        };

        let other_items_str = other_items.to_string();

        let other_items: syn::File = syn::parse2(other_items).map_err(|err| {
            err!(
                &Span::call_site(),
                "bug in the `bon` crate: the macro generated code that contains syntax errors; \
                please report this issue at our Github repository: \
                https://github.com/elastio/bon;\n\
                syntax error in generated code: {err:#?};\n\
                generated code:\n\
                ```rust
                {other_items_str}\n\
                ```",
            )
        })?;

        let mut other_items = other_items.items;

        for item in &mut other_items {
            if let Some(attrs) = item.attrs_mut() {
                attrs.extend(allows.clone());
            }
        }

        start_fn.attrs.extend(allows);

        Ok(MacroOutput {
            start_fn,
            other_items: quote!(#(#other_items)*),
        })
    }

    fn builder_impl(&self) -> Result<TokenStream> {
        let finish_fn = self.finish_fn();
        let accessor_methods = self
            .named_members()
            .map(|member| {
                let setters = SettersCtx::new(self, member).setter_methods()?;
                let getters = GettersCtx::new(self, member)
                    .map(GettersCtx::getter_methods)
                    .transpose()?
                    .unwrap_or_default();

                // Output all accessor methods for the same member adjecently.
                // This is important in the generated rustdoc output, because
                // rustdoc lists methods in the order they appear in the source.
                Ok([setters, getters])
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten();

        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;
        let builder_ident = &self.builder_type.ident;
        let state_mod = &self.state_mod.ident;
        let state_var = &self.state_var;

        let allows = allow_warnings_on_member_types();

        Ok(quote! {
            #allows
            // Ignore dead code warnings because some setter/getter methods may
            // not be used
            #[allow(dead_code)]
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::State
            >
            #builder_ident<#(#generic_args,)* #state_var>
            #where_clause
            {
                #finish_fn
                #(#accessor_methods)*
            }
        })
    }

    /// Generates code that has no meaning to the compiler, but it helps
    /// IDEs to provide better code highlighting, completions and other
    /// hints.
    fn ide_hints(&self) -> TokenStream {
        let type_patterns = self
            .on
            .iter()
            .map(|params| &params.type_pattern)
            .collect::<Vec<_>>();

        if type_patterns.is_empty() {
            return quote! {};
        }

        quote! {
            // This is wrapped in a special cfg set by `rust-analyzer` to enable this
            // code for rust-analyzer's analysis only, but prevent the code from being
            // compiled by `rustc`. Rust Analyzer should be able to use the syntax
            // provided inside of the block to figure out the semantic meaning of
            // the tokens passed to the attribute.
            #[allow(unexpected_cfgs)]
            {
                #[cfg(rust_analyzer)]
                {
                    // Let IDEs know that these are type patterns like the ones that
                    // could be written in a type annotation for a variable. Note that
                    // we don't initialize the variable with any value because we don't
                    // have any meaningful value to assign to this variable, especially
                    // because its type may contain wildcard patterns like `_`. This is
                    // used only to signal the IDEs that these tokens are meant to be
                    // type patterns by placing them in the context where type patterns
                    // are expected.
                    let _: (#(#type_patterns,)*);
                }
            }
        }
    }

    fn phantom_data(&self) -> TokenStream {
        let member_types = self.members.iter().filter_map(|member| {
            match member {
                // The types of these members already appear in the struct as regular fields.
                Member::StartFn(_) | Member::Field(_) | Member::Named(_) => None,
                Member::FinishFn(member) => Some(member.ty.norm.as_ref()),
                Member::Skip(member) => Some(member.norm_ty.as_ref()),
            }
        });

        let receiver_ty = self
            .assoc_method_ctx
            .as_ref()
            .map(|ctx| ctx.self_ty.as_ref());

        let generic_types = self.generics.args.iter().filter_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => Some(ty),
            _ => None,
        });

        let types = std::iter::empty()
            .chain(receiver_ty)
            .chain(member_types)
            .chain(generic_types)
            .map(|ty| {
                // Wrap `ty` in another phantom data because it can be `?Sized`,
                // and simply using it as a type of the tuple member would
                // be wrong, because tuple's members must be sized.
                //
                // We also wrap this in an `fn() -> ...` to make the compiler think
                // that the builder doesn't "own" an instance of the given type.
                // This removes unnecessary requirements when evaluating the
                // applicability of the auto traits.
                quote!(fn() -> ::core::marker::PhantomData<#ty>)
            });

        let lifetimes = self.generics.args.iter().filter_map(|arg| match arg {
            syn::GenericArgument::Lifetime(lifetime) => Some(lifetime),
            _ => None,
        });

        let state_var = &self.state_var;

        quote! {
            ::core::marker::PhantomData<(
                // We have to store the builder state in phantom data otherwise it
                // would be reported as an unused type parameter.
                //
                // We also wrap this in an `fn() -> ...` to make the compiler think
                // that the builder doesn't "own" an instance of the given type.
                // This removes unnecessary requirements when evaluating the
                // applicability of the auto traits.
                fn() -> #state_var,

                // Even though lifetimes will most likely be used somewhere in
                // member types, it is not guaranteed in case of functions/methods,
                // so we mention them all separately. This covers a special case
                // for function builders where the lifetime can be entirely unused
                // (the language permis that).
                //
                // This edge case was discovered thanks to @tonywu6 ❤️:
                // https://github.com/elastio/bon/issues/206
                #( &#lifetimes (), )*

                // There is an interesting quirk with lifetimes in Rust, which is the
                // reason why we thoughtlessly store all the function parameter types
                // in phantom data here.
                //
                // Suppose a function was defined with an argument of type `&'a T`
                // and then we generate an impl block (simplified):
                //
                // ```
                // impl<'a, T, U> for Foo<U>
                // where
                //     U: Into<&'a T>,
                // {}
                // ```
                // Then compiler will complain with the message "the parameter type `T`
                // may not live long enough". So we would need to manually add the bound
                // `T: 'a` to fix this. However, it's hard to infer such a bound in macro
                // context. A workaround for that would be to store the `&'a T` inside of
                // the struct itself, which auto-implies this bound for us implicitly.
                //
                // That's a weird implicit behavior in Rust, I suppose there is a reasonable
                // explanation for it, I just didn't care to research it yet ¯\_(ツ)_/¯.
                #(#types,)*
            )>
        }
    }

    /// Wrap the user-supplied expression in a closure to prevent it from
    /// breaking out of the surrounding scope.
    ///
    /// Closures aren't supported in `const` contexts at the time of this writing
    /// (Rust 1.86.0), so we don't wrap the expression in a closure in `const` case
    /// but we require the expression to be simple so that it doesn't break out of
    /// the surrounding scope.
    ///
    /// This function assumes the expression underwent a `require_embeddable_const_expr`
    /// check if `const` is enabled. Search for it in code to see where it happens.
    fn sanitize_expr(&self, expr: &syn::Expr) -> TokenStream {
        if self.const_.is_some() {
            return quote!(#expr);
        }

        quote! {
            (|| #expr)()
        }
    }
}

fn allow_warnings_on_member_types() -> TokenStream {
    quote! {
        // This warning may occur when the original unnormalized syntax was
        // using parens around an `impl Trait` like that:
        // ```
        // &(impl Clone + Default)
        // ```
        // in which case the normalized version will be:
        // ```
        // &(T)
        // ```
        //
        // And it triggers the warning. We just suppress it here.
        #[allow(unused_parens)]
    }
}
