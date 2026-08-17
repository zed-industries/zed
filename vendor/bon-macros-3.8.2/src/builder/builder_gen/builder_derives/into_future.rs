use crate::builder::builder_gen::models::BuilderGenCtx;
use crate::builder::builder_gen::top_level_config::IntoFutureConfig;
use crate::util::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use syn::visit_mut::VisitMut;

impl BuilderGenCtx {
    pub(super) fn derive_into_future(&self, config: &IntoFutureConfig) -> Result<TokenStream> {
        if self.finish_fn.asyncness.is_none() {
            // While it is technically possible to call a synchronous function
            // inside of the `IntoFuture::into_future()`, it's better force the
            // user to mark the function as `async` explicitly. Otherwise it may
            // indicate of some logic bug where the developer mistakenly marks
            // a function that could be sync with `derive(IntoFuture)`.
            bail!(
                &self.finish_fn.ident,
                "`#[builder(derive(IntoFuture(...)))` can only be used with async functions; \
                using it with a synchronous function is likely a mistake"
            );
        }

        if let Some(unsafety) = &self.finish_fn.unsafety {
            bail!(
                unsafety,
                "`#[builder(derive(IntoFuture(...)))` is not supported for unsafe functions \
                because `IntoFuture::into_future()` method is a safe method"
            );
        }

        if let Some(arg) = self.finish_fn_args().next() {
            bail!(
                &arg.config.finish_fn.span(),
                "`#[builder(derive(IntoFuture(...)))` is incompatible with `#[builder(finish_fn)]` members \
                because `IntoFuture::into_future()` method accepts zero parameters"
            );
        }

        let state_mod = &self.state_mod.ident;
        let builder_ident = &self.builder_type.ident;
        let state_var = &self.state_var;
        let finish_fn_ident = &self.finish_fn.ident;
        let box_ = &config.box_ident;

        let SignatureForIntoFuture {
            generics_decl,
            generic_args,
            where_clause,
            builder_lifetime,
            output_ty,
        } = self.signature_for_into_future();

        let state_lifetime = builder_lifetime
            .clone()
            .unwrap_or_else(|| syn::Lifetime::new("'static", Span::call_site()));

        let builder_lifetime = Option::into_iter(builder_lifetime);

        let send_bound = if config.is_send {
            quote! { + ::core::marker::Send }
        } else {
            quote! {}
        };

        let bon = &self.bon;

        let alloc = if cfg!(feature = "std") {
            quote!(::std)
        } else if cfg!(feature = "alloc") {
            quote!(#bon::__::alloc)
        } else {
            bail!(
                &config.box_ident,
                "`#[builder(derive(IntoFuture(Box)))]` requires either `std` or \
                `alloc` feature to be enabled"
            )
        };

        let tokens = quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::IsComplete + #state_lifetime
            >
            ::core::future::IntoFuture for #builder_ident<#(#generic_args,)* #state_var>
            #where_clause
            {
                type Output = #output_ty;
                type IntoFuture = ::core::pin::Pin<
                    #alloc::boxed::#box_<
                        dyn ::core::future::Future<Output = Self::Output>
                        #send_bound
                        #(+ #builder_lifetime)*
                    >
                >;

                fn into_future(self) -> Self::IntoFuture {
                    #alloc::boxed::#box_::pin(#builder_ident::#finish_fn_ident(self))
                }
            }
        };

        Ok(tokens)
    }

    /// Handle the special case for a builder that captures lifetimes.
    ///
    /// Collapse all lifetimes into a single `'builder` lifetime. This is
    /// because `dyn Trait` supports only a single `+ 'lifetime` bound.
    fn signature_for_into_future(&self) -> SignatureForIntoFuture<'_> {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;

        let output_ty = match &self.finish_fn.output {
            syn::ReturnType::Default => Cow::Owned(syn::parse_quote!(())),
            syn::ReturnType::Type(_, output_ty) => Cow::Borrowed(output_ty.as_ref()),
        };

        let contains_lifetimes = matches!(
            self.generics.args.first(),
            Some(syn::GenericArgument::Lifetime(_))
        );

        if !contains_lifetimes {
            return SignatureForIntoFuture {
                generics_decl: Cow::Borrowed(generics_decl),
                generic_args: Cow::Borrowed(generic_args),
                where_clause: where_clause.as_ref().map(Cow::Borrowed),
                builder_lifetime: None,
                output_ty,
            };
        }

        let builder_lifetime = syn::Lifetime::new("'builder", Span::call_site());

        let new_generic_args = generic_args
            .iter()
            .map(|arg| match arg {
                syn::GenericArgument::Lifetime(_) => {
                    syn::GenericArgument::Lifetime(builder_lifetime.clone())
                }
                _ => arg.clone(),
            })
            .collect::<Vec<_>>();

        let mut original_lifetimes = BTreeSet::new();
        let mut new_generics_decl = vec![syn::parse_quote!(#builder_lifetime)];

        for param in generics_decl {
            match param {
                syn::GenericParam::Lifetime(lifetime) => {
                    original_lifetimes.insert(&lifetime.lifetime.ident);
                }
                _ => {
                    new_generics_decl.push(param.clone());
                }
            }
        }

        let mut replace_lifetimes = ReplaceLifetimes {
            replacement: &builder_lifetime,
            original_lifetimes: &original_lifetimes,
        };

        for decl in &mut new_generics_decl {
            replace_lifetimes.visit_generic_param_mut(decl);
        }

        let mut new_where_clause = where_clause.clone();

        if let Some(where_clause) = &mut new_where_clause {
            replace_lifetimes.visit_where_clause_mut(where_clause);
        }

        let mut output_ty = output_ty.into_owned();

        replace_lifetimes.visit_type_mut(&mut output_ty);

        SignatureForIntoFuture {
            generics_decl: Cow::Owned(new_generics_decl),
            generic_args: Cow::Owned(new_generic_args),
            where_clause: new_where_clause.map(Cow::Owned),
            builder_lifetime: Some(builder_lifetime),
            output_ty: Cow::Owned(output_ty),
        }
    }
}

struct SignatureForIntoFuture<'a> {
    generics_decl: Cow<'a, [syn::GenericParam]>,
    generic_args: Cow<'a, [syn::GenericArgument]>,
    where_clause: Option<Cow<'a, syn::WhereClause>>,
    builder_lifetime: Option<syn::Lifetime>,
    output_ty: Cow<'a, syn::Type>,
}

struct ReplaceLifetimes<'a> {
    replacement: &'a syn::Lifetime,
    original_lifetimes: &'a BTreeSet<&'a syn::Ident>,
}

impl VisitMut for ReplaceLifetimes<'_> {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if self.original_lifetimes.contains(&lifetime.ident) {
            *lifetime = self.replacement.clone();
        }
    }

    fn visit_item_mut(&mut self, _: &mut syn::Item) {
        // Don't recurse into child items. They don't inherit the parent item's
        // lifetimes.
    }

    fn visit_bound_lifetimes_mut(&mut self, _: &mut syn::BoundLifetimes) {
        // Don't recurse into bound lifetime declarations. They introduce
        // local lifetimes that we should keep as is
    }
}
