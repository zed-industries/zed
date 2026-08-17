use crate::parsing::SpannedKey;
use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug, Default)]
pub(crate) struct GetterConfig {
    pub(crate) name: Option<SpannedKey<syn::Ident>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,

    /// Returns `&T` if [`None`]
    pub(crate) kind: Option<SpannedKey<GetterKind>>,
}

#[derive(Debug)]
pub(crate) enum GetterKind {
    /// Returns `T` via [`Copy`]
    Copy,

    /// Returns `T` via [`Clone`]
    Clone,

    /// Returns `&<T as Deref>::Target`.
    /// If the type is `None`, it will be inferred from the member's type.
    Deref(Option<Box<syn::Type>>),
}

impl FromMeta for GetterConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::Path(_) = meta {
            return Ok(Self::default());
        }

        // Reject empty parens such as `#[builder(getter())]`
        crate::parsing::require_non_empty_paren_meta_list_or_name_value(meta)?;

        // Nested `Parsed` struct used as a helper for parsing the verbose form
        #[derive(FromMeta)]
        struct Parsed {
            name: Option<SpannedKey<syn::Ident>>,
            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(rename = "doc", default, with = parse_docs, map = Some)]
            docs: Option<SpannedKey<Vec<syn::Attribute>>>,

            copy: Option<SpannedKey<()>>,
            clone: Option<SpannedKey<()>>,

            #[darling(default, map = Some, with = parse_deref)]
            deref: Option<SpannedKey<Option<Box<syn::Type>>>>,
        }

        let Parsed {
            name,
            vis,
            docs,
            copy,
            clone,
            deref,
        } = Parsed::from_meta(meta)?;

        let kinds = [
            copy.map(|cfg| cfg.with_value(GetterKind::Copy)),
            clone.map(|cfg| cfg.with_value(GetterKind::Clone)),
            deref.map(|ty| ty.map_value(GetterKind::Deref)),
        ];

        let kinds = kinds.into_iter().flatten().collect::<Vec<_>>();

        if let [kind1, kind2, ..] = kinds.as_slice() {
            bail!(
                &kind1.key,
                "`{}` can't be specified together with `{}`",
                kind1.key,
                kind2.key
            );
        }

        let kind = kinds.into_iter().next();

        Ok(Self {
            name,
            vis,
            docs,
            kind,
        })
    }
}

fn parse_deref(meta: &syn::Meta) -> Result<SpannedKey<Option<Box<syn::Type>>>> {
    let value = match meta {
        syn::Meta::NameValue(_) => bail!(
            meta,
            "expected `getter(deref)` or `getter(deref(...))` syntax"
        ),
        syn::Meta::Path(_) => None,
        syn::Meta::List(meta) => Some(syn::parse2(meta.tokens.clone())?),
    };

    SpannedKey::new(meta.path(), value)
}

fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions("builder struct's impl block", meta)
}
