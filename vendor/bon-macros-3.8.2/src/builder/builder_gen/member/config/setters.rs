use crate::parsing::{ItemSigConfig, ItemSigConfigParsing, SpannedKey};
use crate::util::prelude::*;
use darling::ast::NestedMeta;
use darling::FromMeta;
use syn::punctuated::Punctuated;

const DOCS_CONTEXT: &str = "builder struct's impl block";

fn parse_setter_fn(meta: &syn::Meta) -> Result<SpannedKey<ItemSigConfig>> {
    let params = ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some(DOCS_CONTEXT),
    }
    .parse()?;

    SpannedKey::new(meta.path(), params)
}

fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions(DOCS_CONTEXT, meta)
}

#[derive(Debug, Default)]
pub(crate) struct SettersConfig {
    pub(crate) name: Option<SpannedKey<syn::Ident>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) doc: SettersDocConfig,
    pub(crate) fns: SettersFnsConfig,
}

impl FromMeta for SettersConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let meta: Punctuated<syn::Meta, syn::Token![,]> =
            crate::parsing::parse_paren_meta_list_with_terminated(meta)?;

        let (docs, remaining_meta): (Vec<_>, Vec<_>) = meta
            .into_iter()
            .partition(|meta| meta.path().is_ident("doc"));

        let doc = SettersDocConfig::from_docs_entries(docs)?;

        #[derive(FromMeta)]
        struct Parsed {
            name: Option<SpannedKey<syn::Ident>>,
            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(flatten)]
            fns: SettersFnsConfig,
        }

        let remaining_meta = remaining_meta
            .into_iter()
            .map(NestedMeta::Meta)
            .collect::<Vec<_>>();

        let parsed: Parsed = Parsed::from_list(&remaining_meta)?;

        Ok(Self {
            name: parsed.name,
            vis: parsed.vis,
            fns: parsed.fns,
            doc,
        })
    }
}

#[derive(Debug, Default, FromMeta)]
pub(crate) struct SettersFnsConfig {
    /// Config for the setter that accepts the value of type T for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `{member}` without any prefix or suffix.
    #[darling(default, with = parse_setter_fn, map = Some)]
    pub(crate) some_fn: Option<SpannedKey<ItemSigConfig>>,

    /// The setter that accepts the value of type `Option<T>` for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `maybe_{member}`.
    #[darling(default, with = parse_setter_fn, map = Some)]
    pub(crate) option_fn: Option<SpannedKey<ItemSigConfig>>,
}

#[derive(Debug, Default)]
pub(crate) struct SettersDocConfig {
    /// Overrides the content of the doc comments.
    pub(crate) content: Option<SpannedKey<Vec<syn::Attribute>>>,

    /// Overrides the look of the default value showcase in the docs header.
    pub(crate) default: Option<SpannedKey<SettersDocDefaultConfig>>,
}

impl SettersDocConfig {
    fn from_docs_entries(docs: Vec<syn::Meta>) -> Result<Self> {
        let mut doc_config = None;
        let mut doc_content = None;

        for doc in docs {
            match doc.require_list()?.delimiter {
                syn::MacroDelimiter::Paren(_) => {
                    if doc_config.is_some() {
                        bail!(&doc, "repeated `doc(...)` attribute is not allowed");
                    }
                    doc_config = Some(doc);
                }
                syn::MacroDelimiter::Brace(_) => {
                    if doc_content.is_some() {
                        bail!(&doc, "repeated `doc {{...}}` attribute is not allowed");
                    }
                    doc_content = Some(doc);
                }
                syn::MacroDelimiter::Bracket(_) => {
                    bail!(&doc, "wrong delimiter, expected doc(...) or doc {{...}}",);
                }
            }
        }

        #[derive(FromMeta)]
        struct Parsed {
            #[darling(with = crate::parsing::parse_non_empty_paren_meta_list)]
            default: Option<SpannedKey<SettersDocDefaultConfig>>,
        }

        let config = doc_config
            .as_ref()
            .map(crate::parsing::parse_non_empty_paren_meta_list::<Parsed>)
            .transpose()?;

        let content = doc_content.as_ref().map(parse_docs).transpose()?;

        let mut me = Self {
            content,
            default: None,
        };

        if let Some(Parsed { default }) = config {
            me.default = default;
            // More keys may be added here in the future
        }

        Ok(me)
    }
}

#[derive(Debug, Default, FromMeta)]
pub(crate) struct SettersDocDefaultConfig {
    /// If `true`, the default value showcase in the docs header will be skipped.
    pub(crate) skip: darling::util::Flag,
}
