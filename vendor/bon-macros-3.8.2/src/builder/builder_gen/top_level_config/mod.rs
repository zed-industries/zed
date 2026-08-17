mod on;

pub(crate) use on::OnConfig;

use crate::parsing::{BonCratePath, ItemSigConfig, ItemSigConfigParsing, SpannedKey};
use crate::util::prelude::*;
use darling::ast::NestedMeta;
use darling::FromMeta;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::ItemFn;

fn parse_finish_fn(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder struct's impl block"),
    }
    .parse()
}

fn parse_builder_type(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder struct"),
    }
    .parse()
}

fn parse_state_mod(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder's state module"),
    }
    .parse()
}

fn parse_start_fn(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: None,
    }
    .parse()
}

#[derive(Debug, FromMeta)]
pub(crate) struct TopLevelConfig {
    /// Specifies whether the generated functions should be `const`.
    ///
    /// It is marked as `#[darling(skip)]` because `const` is a keyword, that
    /// can't be parsed as a `syn::Ident` and therefore as a `syn::Meta` item.
    /// We manually parse it from the beginning `builder(...)`.
    #[darling(skip)]
    pub(crate) const_: Option<syn::Token![const]>,

    /// Overrides the path to the `bon` crate. This is useful when the macro is
    /// wrapped in another macro that also reexports `bon`.
    #[darling(rename = "crate", default)]
    pub(crate) bon: BonCratePath,

    #[darling(default, with = parse_start_fn)]
    pub(crate) start_fn: ItemSigConfig,

    #[darling(default, with = parse_finish_fn)]
    pub(crate) finish_fn: ItemSigConfig,

    #[darling(default, with = parse_builder_type)]
    pub(crate) builder_type: ItemSigConfig,

    #[darling(default, with = parse_state_mod)]
    pub(crate) state_mod: ItemSigConfig,

    #[darling(multiple, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) on: Vec<OnConfig>,

    /// Specifies the derives to apply to the builder.
    #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) derive: DerivesConfig,
}

impl TopLevelConfig {
    pub(crate) fn parse_for_fn(fn_item: &ItemFn, config: Option<TokenStream>) -> Result<Self> {
        let other_configs = fn_item
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("builder"))
            .map(|attr| {
                if let syn::Meta::List(_) = attr.meta {
                    crate::parsing::require_non_empty_paren_meta_list_or_name_value(&attr.meta)?;
                }
                let meta_list = darling::util::parse_attribute_to_meta_list(attr)?;
                Ok(meta_list.tokens)
            });

        let configs = config
            .map(Ok)
            .into_iter()
            .chain(other_configs)
            .collect::<Result<Vec<_>>>()?;

        let me = Self::parse_for_any(configs)?;

        if me.start_fn.name.is_none() {
            let ItemSigConfig { name: _, vis, docs } = &me.start_fn;

            let unexpected_param = None
                .or_else(|| vis.as_ref().map(SpannedKey::key))
                .or_else(|| docs.as_ref().map(SpannedKey::key));

            if let Some(unexpected_param) = unexpected_param {
                bail!(
                    unexpected_param,
                    "#[builder(start_fn({unexpected_param}))] requires that you \
                    also specify #[builder(start_fn(name))] which makes the starting \
                    function not to replace the positional function under the #[builder] \
                    attribute; by default (without the explicit #[builder(start_fn(name))]) \
                    the name, visibility and documentation of the positional \
                    function are all copied to the starting function, and the positional \
                    function under the #[builder] attribute becomes private with \
                    #[doc(hidden)] and it's renamed (the name is not guaranteed \
                    to be stable) to make it inaccessible even within the current module",
                );
            }
        }

        Ok(me)
    }

    pub(crate) fn parse_for_struct(configs: Vec<TokenStream>) -> Result<Self> {
        Self::parse_for_any(configs)
    }

    fn parse_for_any(mut configs: Vec<TokenStream>) -> Result<Self> {
        fn parse_const_prefix(
            parse: syn::parse::ParseStream<'_>,
        ) -> syn::Result<(Option<syn::Token![const]>, TokenStream)> {
            let const_ = parse.parse().ok();
            if const_.is_some() && !parse.is_empty() {
                parse.parse::<syn::Token![,]>()?;
            }

            let rest = parse.parse()?;
            Ok((const_, rest))
        }

        // Try to parse the first token of the first config as `const` token.
        // We have to do this manually because `syn` doesn't support parsing
        // keywords in the `syn::Meta` keys. Yeah, unfortunately it means that
        // the users must ensure they place `const` right at the beginning of
        // their `#[builder(...)]` attributes.
        let mut const_ = None;

        if let Some(config) = configs.first_mut() {
            (const_, *config) = parse_const_prefix.parse2(std::mem::take(config))?;
        }

        let configs = configs
            .into_iter()
            .map(NestedMeta::parse_meta_list)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        // This is a temporary hack. We only allow `on(_, required)` as the
        // first `on(...)` clause. Instead we should implement an extended design:
        // https://github.com/elastio/bon/issues/152
        let mut on_configs = configs
            .iter()
            .enumerate()
            .filter_map(|(i, meta)| match meta {
                NestedMeta::Meta(syn::Meta::List(meta)) if meta.path.is_ident("on") => {
                    Some((i, meta))
                }
                _ => None,
            })
            .peekable();

        while let Some((i, _)) = on_configs.next() {
            if let Some((j, next_on)) = on_configs.peek() {
                if *j != i + 1 {
                    bail!(
                        next_on,
                        "this `on(...)` clause is out of order; all `on(...)` \
                        clauses must be consecutive; there shouldn't be any \
                        other parameters between them",
                    )
                }
            }
        }

        let me = Self {
            const_,
            ..Self::from_list(&configs)?
        };

        if let Some(on) = me.on.iter().skip(1).find(|on| on.required.is_present()) {
            bail!(
                &on.required.span(),
                "`required` can only be specified in the first `on(...)` clause; \
                this restriction may be lifted in the future",
            );
        }

        if let Some(first_on) = me.on.first().filter(|on| on.required.is_present()) {
            if !matches!(first_on.type_pattern, syn::Type::Infer(_)) {
                bail!(
                    &first_on.type_pattern,
                    "`required` can only be used with the wildcard type pattern \
                    i.e. `on(_, required)`; this restriction may be lifted in the future",
                );
            }
        }

        Ok(me)
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
pub(crate) struct DerivesConfig {
    #[darling(rename = "Clone")]
    pub(crate) clone: Option<DeriveConfig>,

    #[darling(rename = "Debug")]
    pub(crate) debug: Option<DeriveConfig>,

    #[darling(rename = "Into")]
    pub(crate) into: darling::util::Flag,

    #[darling(rename = "IntoFuture")]
    pub(crate) into_future: Option<IntoFutureConfig>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DeriveConfig {
    pub(crate) bounds: Option<Punctuated<syn::WherePredicate, syn::Token![,]>>,
}

#[derive(Debug, Clone)]
pub(crate) struct IntoFutureConfig {
    pub(crate) box_ident: syn::Ident,
    pub(crate) is_send: bool,
}

impl syn::parse::Parse for IntoFutureConfig {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        // Parse "Box" as the required first argument.
        let box_ident: syn::Ident = input.parse()?;
        if box_ident != "Box" {
            return Err(syn::Error::new(
                box_ident.span(),
                "expected `Box` as the first argument, only boxed futures are supported",
            ));
        }

        // Check for optional ", ?Send" part.
        let is_send = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;

            // Parse "?Send" as a single unit.
            if input.peek(syn::Token![?]) {
                input.parse::<syn::Token![?]>()?;
                let send_ident: syn::Ident = input.parse()?;
                if send_ident != "Send" {
                    return Err(syn::Error::new(
                        send_ident.span(),
                        "expected `Send` after ?",
                    ));
                }
                false
            } else {
                return Err(input.error("expected `?Send` as the second argument"));
            }
        } else {
            true
        };

        // Ensure no trailing tokens.
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after arguments"));
        }

        Ok(Self { box_ident, is_send })
    }
}

impl FromMeta for IntoFutureConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => bail!(meta, "expected an attribute of form `IntoFuture(Box, ...)`"),
        };

        meta.require_parens_delim()?;

        let me = syn::parse2(meta.tokens.clone())?;

        Ok(me)
    }
}

impl FromMeta for DeriveConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::Path(_) = meta {
            return Ok(Self { bounds: None });
        }

        meta.require_list()?.require_parens_delim()?;

        #[derive(FromMeta)]
        struct Parsed {
            #[darling(with = crate::parsing::parse_paren_meta_list_with_terminated)]
            bounds: Punctuated<syn::WherePredicate, syn::Token![,]>,
        }

        let Parsed { bounds } = Parsed::from_meta(meta)?;

        Ok(Self {
            bounds: Some(bounds),
        })
    }
}
