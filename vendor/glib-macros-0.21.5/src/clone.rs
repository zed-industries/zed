// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Expr, ExprAsync, ExprClosure, Token,
};

use crate::utils::crate_ident_new;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CaptureKind {
    Watch,
    Weak,
    WeakAllowNone,
    Strong,
    ToOwned,
}

impl TryFrom<&'_ Ident> for CaptureKind {
    type Error = syn::Error;

    fn try_from(s: &Ident) -> Result<Self, Self::Error> {
        Ok(match s.to_string().as_str() {
            "watch" => CaptureKind::Watch,
            "strong" => CaptureKind::Strong,
            "weak" => CaptureKind::Weak,
            "weak_allow_none" => CaptureKind::WeakAllowNone,
            "to_owned" => CaptureKind::ToOwned,
            _ => {
                // This is actually never shown to the user but we need some kind of error type for
                // TryFrom, () would be enough but then clippy complains.
                //
                // We'll keep it here in case it is useful somewhere at a later time.
                return Err(syn::Error::new(
                    s.span(),
                    format!("unknown capture type `{s}`"),
                ));
            }
        })
    }
}

#[derive(Default)]
pub(crate) enum UpgradeBehaviour {
    #[default]
    Unit,
    Panic,
    Default,
    Expression(Expr),
    Closure(ExprClosure),
}

impl UpgradeBehaviour {
    pub(crate) fn maybe_parse(
        attrs: &[Attribute],
        input: ParseStream,
    ) -> syn::Result<Option<Self>> {
        // Caller checked for empty
        let attr = &attrs[0];
        attr.meta.require_path_only()?;

        let Some(attr_name) = attr.path().get_ident() else {
            return Ok(None);
        };

        let upgrade_behaviour = match attr_name.to_string().as_str() {
            "upgrade_or" => {
                let expr = input.parse::<Expr>()?;
                input.parse::<Token![,]>()?;
                UpgradeBehaviour::Expression(expr)
            }
            "upgrade_or_else" => {
                let closure = input.parse::<ExprClosure>()?;
                if closure.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        &closure,
                        "`upgrade_or_else` closure needs to be a non-async closure",
                    ));
                }
                if !closure.inputs.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &closure,
                        "`upgrade_or_else` closure must not have any parameters",
                    ));
                }

                input.parse::<Token![,]>()?;
                UpgradeBehaviour::Closure(closure)
            }
            "upgrade_or_default" => UpgradeBehaviour::Default,
            "upgrade_or_panic" => UpgradeBehaviour::Panic,
            _ => {
                return Ok(None);
            }
        };

        if attrs.len() > 1 {
            return Err(syn::Error::new_spanned(
                &attrs[1],
                format!(
                    "upgrade failure attribute must not be followed by any other attributes. Found {} more attribute{}",
                    attrs.len() - 1,
                    if attrs.len() > 2 { "s" } else { "" },
            )));
        }

        let next_attrs = &input.call(Attribute::parse_outer)?;
        if !next_attrs.is_empty() {
            return Err(syn::Error::new_spanned(
                &next_attrs[0],
                format!(
                    "upgrade failure attribute must not be followed by any other attributes. Found {} more attribute{}",
                    next_attrs.len(),
                    if next_attrs.len() > 1 { "s" } else { "" },
                )
            ));
        }

        Ok(Some(upgrade_behaviour))
    }
}

pub(crate) struct Capture {
    pub(crate) name: Expr,
    pub(crate) alias: Option<Ident>,
    pub(crate) kind: CaptureKind,
}

impl Capture {
    pub(crate) fn maybe_parse(
        attrs: &[Attribute],
        input: ParseStream,
    ) -> syn::Result<Option<Self>> {
        // Caller checked for empty
        let attr = &attrs[0];

        let Some(attr_name) = attr.path().get_ident() else {
            return Ok(None);
        };
        let Ok(kind) = CaptureKind::try_from(attr_name) else {
            return Ok(None);
        };

        if attrs.len() > 1 {
            return Err(syn::Error::new_spanned(
                &attrs[1],
                "variable capture attributes must be followed by an identifier",
            ));
        }

        let mut alias = None;
        if let syn::Meta::List(ref list) = attr.meta {
            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_to") {
                    let value = meta.value()?;
                    let id = value.parse::<Ident>()?;
                    if alias.is_some() {
                        return Err(meta.error("multiple `rename_to` properties are not allowed"));
                    }
                    alias = Some(id);
                } else if let Some(ident) = meta.path.get_ident() {
                    return Err(
                        meta.error(
                            format!(
                                "unsupported capture attribute property `{ident}`: only `rename_to` is supported"
                            ),
                        ),
                    );
                } else {
                    return Err(meta.error("unsupported capture attribute property"));
                }
                Ok(())
            })?;
        }

        let name = input.parse::<Expr>()?;
        match name {
            Expr::Path(ref p) if p.path.get_ident().is_some() => {
                if p.path.get_ident().unwrap() == "self" && alias.is_none() {
                    return Err(
                        syn::Error::new_spanned(
                            attr,
                            "capture attribute for `self` requires usage of the `rename_to` attribute property",
                        ),
                    );
                }
                // Nothing to do, it's just an identifier
            }
            _ if alias.is_some() => {
                // Nothing to do, it's an alias
            }
            _ => {
                return Err(
                    syn::Error::new_spanned(
                        attr,
                        "capture attribute for an expression requires usage of the `rename_to` attribute property",
                    ),
                );
            }
        }

        input.parse::<Token![,]>()?;

        Ok(Some(Capture { name, alias, kind }))
    }

    pub(crate) fn alias(&self) -> TokenStream {
        if let Some(ref alias) = self.alias {
            alias.to_token_stream()
        } else {
            self.name.to_token_stream()
        }
    }

    pub(crate) fn outer_before_tokens(&self, crate_ident: &TokenStream) -> TokenStream {
        let alias = self.alias();
        let name = &self.name;
        match self.kind {
            CaptureKind::Watch => quote! {
                let #alias = #crate_ident::object::Watchable::watched_object(&#name);
            },
            CaptureKind::Weak | CaptureKind::WeakAllowNone => quote! {
                let #alias = #crate_ident::clone::Downgrade::downgrade(&#name);
            },
            CaptureKind::Strong => quote! {
                let #alias = #name.clone();
            },
            CaptureKind::ToOwned => quote! {
                let #alias = ::std::borrow::ToOwned::to_owned(&*#name);
            },
        }
    }

    pub(crate) fn outer_after_tokens(
        &self,
        crate_ident: &TokenStream,
        closure_ident: &Ident,
    ) -> TokenStream {
        let name = &self.name;
        match self.kind {
            CaptureKind::Watch => quote! {
                #crate_ident::object::Watchable::watch_closure(&#name, &#closure_ident);
            },
            _ => Default::default(),
        }
    }

    pub(crate) fn inner_before_tokens(
        &self,
        crate_ident: &TokenStream,
        weak_upgrade_failure_kind: &UpgradeBehaviour,
        upgrade_failure_closure_ident: &Ident,
        unit_return: Option<TokenStream>,
    ) -> TokenStream {
        let alias = self.alias();
        match self.kind {
            CaptureKind::Watch => {
                quote! {
                    let #alias = unsafe { #alias.borrow() };
                    let #alias = ::core::convert::AsRef::as_ref(&#alias);
                }
            }
            CaptureKind::Weak => match weak_upgrade_failure_kind {
                UpgradeBehaviour::Panic => {
                    let err_msg = format!(
                        "Failed to upgrade `{alias}`. If you don't want to panic, use `#[upgrade_or]`, `#[upgrade_or_else]` or `#[upgrade_or_default]`",
                    );
                    quote! {
                        let Some(#alias) = #crate_ident::clone::Upgrade::upgrade(&#alias) else {
                            panic!(#err_msg);
                        };
                    }
                }
                UpgradeBehaviour::Default
                | UpgradeBehaviour::Expression(_)
                | UpgradeBehaviour::Closure(_) => {
                    let err_msg = format!("Failed to upgrade `{alias}`");
                    quote! {
                        let Some(#alias) = #crate_ident::clone::Upgrade::upgrade(&#alias) else {
                            #crate_ident::g_debug!(
                                #crate_ident::CLONE_MACRO_LOG_DOMAIN,
                                #err_msg,
                            );
                            return (#upgrade_failure_closure_ident)();
                        };
                    }
                }
                UpgradeBehaviour::Unit => {
                    let err_msg = format!("Failed to upgrade `{alias}`");
                    let unit_return = unit_return.unwrap_or_else(|| {
                        quote! { return; }
                    });
                    quote! {
                        let Some(#alias) = #crate_ident::clone::Upgrade::upgrade(&#alias) else {
                            #crate_ident::g_debug!(
                                #crate_ident::CLONE_MACRO_LOG_DOMAIN,
                                #err_msg,
                            );
                            #unit_return
                        };
                    }
                }
            },
            CaptureKind::WeakAllowNone => quote! {
                let #alias = #crate_ident::clone::Upgrade::upgrade(&#alias);
            },
            _ => Default::default(),
        }
    }
}

#[derive(Clone)]
enum ClosureOrAsync {
    Closure(ExprClosure),
    Async(ExprAsync),
}

impl Parse for ClosureOrAsync {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;
        match expr {
            Expr::Async(async_) => {
                if async_.capture.is_none() {
                    return Err(syn::Error::new_spanned(
                        async_,
                        "async blocks need to capture variables by move. Please add the `move` keyword",
                    ));
                }

                Ok(ClosureOrAsync::Async(async_))
            }
            Expr::Closure(closure) => {
                if closure.capture.is_none() {
                    return Err(syn::Error::new_spanned(
                        closure,
                        "closures need to capture variables by move. Please add the `move` keyword",
                    ));
                }

                Ok(ClosureOrAsync::Closure(closure))
            }
            _ => Err(syn::Error::new_spanned(
                expr,
                "only closures and async blocks are supported",
            )),
        }
    }
}

impl ToTokens for ClosureOrAsync {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ClosureOrAsync::Closure(ref c) => c.to_tokens(tokens),
            ClosureOrAsync::Async(ref a) => a.to_tokens(tokens),
        }
    }
}

struct Clone {
    captures: Vec<Capture>,
    upgrade_behaviour: UpgradeBehaviour,
    body: ClosureOrAsync,
}

impl Parse for Clone {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected a closure or async block",
            ));
        }

        let mut captures: Vec<Capture> = vec![];
        let mut upgrade_behaviour: Option<(UpgradeBehaviour, Span)> = None;

        loop {
            // There must either be one or no attributes here. Multiple attributes are not
            // supported.
            //
            // If this is a capture attribute, it must be followed by an identifier.
            // If this is an upgrade failure attribute, it might be followed by a closure. After the
            // upgrade failure attribute there must not be any further attributes.
            //
            // If this is not an attribute then it is a closure, async closure or async block which
            // is handled outside the loop
            let attrs = input.call(Attribute::parse_outer)?;
            if attrs.is_empty() {
                break;
            };

            if let Some(capture) = Capture::maybe_parse(&attrs, input)? {
                if capture.kind == CaptureKind::Watch {
                    return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "watch variable captures are not supported",
                    ));
                }

                captures.push(capture);
            } else if let Some(behaviour) = UpgradeBehaviour::maybe_parse(&attrs, input)? {
                if upgrade_behaviour.is_some() {
                    return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "multiple upgrade failure attributes are not supported",
                    ));
                }

                upgrade_behaviour = Some((behaviour, attrs[0].span()));
                break;
            } else if let Some(ident) = attrs[0].path().get_ident() {
                return Err(syn::Error::new_spanned(
                        &attrs[0],
                        format!(
                            "unsupported attribute `{ident}`: only `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported",
                        ),
                ));
            } else {
                return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "unsupported attribute: only `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported",
                ));
            }
        }

        if let Some((_, ref span)) = upgrade_behaviour {
            if captures.iter().all(|c| c.kind != CaptureKind::Weak) {
                return Err(syn::Error::new(
                    *span,
                    "upgrade failure attribute can only be used together with weak variable captures",
                ));
            }
        }

        let upgrade_behaviour = upgrade_behaviour.map(|x| x.0).unwrap_or_default();

        // Following is a closure or async block
        let body = input.parse::<ClosureOrAsync>()?;

        // Trailing comma, if any
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Clone {
            captures,
            upgrade_behaviour,
            body,
        })
    }
}

impl ToTokens for Clone {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ident = crate_ident_new();

        let upgrade_failure_closure_ident =
            Ident::new("____upgrade_failure_closure", Span::call_site());

        let outer_before = self
            .captures
            .iter()
            .map(|c| c.outer_before_tokens(&crate_ident));
        let inner_before = self.captures.iter().map(|c| {
            c.inner_before_tokens(
                &crate_ident,
                &self.upgrade_behaviour,
                &upgrade_failure_closure_ident,
                None,
            )
        });

        let upgrade_failure_closure = match self.upgrade_behaviour {
            UpgradeBehaviour::Default => Some(quote! {
                let #upgrade_failure_closure_ident = ::std::default::Default::default;
            }),
            UpgradeBehaviour::Expression(ref expr) => Some(quote! {
                let #upgrade_failure_closure_ident = move || {
                    #expr
                };
            }),
            UpgradeBehaviour::Closure(ref closure) => Some(quote! {
                let #upgrade_failure_closure_ident = #closure;
            }),
            _ => None,
        };

        let body = match self.body {
            ClosureOrAsync::Closure(ref c) => {
                let ExprClosure {
                    attrs,
                    lifetimes,
                    constness,
                    movability,
                    asyncness,
                    capture,
                    or1_token,
                    inputs,
                    or2_token,
                    output,
                    body,
                } = c;

                quote! {
                    #(#attrs)*
                    #lifetimes
                    #constness
                    #movability
                    #asyncness
                    #capture
                    #or1_token
                    #inputs
                    #or2_token
                    #output
                    {
                        #upgrade_failure_closure
                        #(#inner_before)*
                        #body
                    }
                }
            }
            ClosureOrAsync::Async(ref a) => {
                let ExprAsync {
                    attrs,
                    async_token,
                    capture,
                    block,
                } = a;

                // Directly output the statements instead of the whole block including braces as we
                // already produce a block with braces below and otherwise a compiler warning about
                // unnecessary braces is wrongly emitted.
                let stmts = &block.stmts;

                quote! {
                    #(#attrs)*
                    #async_token
                    #capture
                    {
                        #upgrade_failure_closure
                        #(#inner_before)*
                        #(#stmts)*
                    }
                }
            }
        };

        tokens.extend(quote! {
            {
                #(#outer_before)*
                #body
            }
        });
    }
}

pub(crate) fn clone_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let clone = syn::parse_macro_input!(input as Clone);
    clone.into_token_stream().into()
}
