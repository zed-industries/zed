use super::SpannedKey;
use crate::util::prelude::*;

pub(crate) fn parse_docs_without_self_mentions(
    context: &'static str,
    meta: &syn::Meta,
) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    let docs = parse_docs(meta)?;
    reject_self_mentions_in_docs(context, &docs)?;
    Ok(docs)
}

pub(crate) fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    let meta = meta.require_list()?;

    meta.require_curly_braces_delim()?;

    let attrs = meta.parse_args_with(syn::Attribute::parse_outer)?;

    for attr in &attrs {
        if !attr.is_doc_expr() {
            bail!(attr, "expected a doc comment");
        }
    }

    SpannedKey::new(&meta.path, attrs)
}

/// Validates the docs for the presence of `Self` mentions to prevent users from
/// shooting themselves in the foot where they would think that `Self` resolves
/// to the current item the docs were placed on, when in fact the docs are moved
/// to a different context where `Self` has a different meaning.
pub(crate) fn reject_self_mentions_in_docs(
    context: &'static str,
    attrs: &[syn::Attribute],
) -> Result {
    for attr in attrs {
        let doc = match attr.as_doc_expr() {
            Some(doc) => doc,
            _ => continue,
        };

        let doc = match &doc {
            syn::Expr::Lit(doc) => doc,
            _ => continue,
        };

        let doc = match &doc.lit {
            syn::Lit::Str(doc) => doc,
            _ => continue,
        };

        let self_references = ["[`Self`]", "[Self]"];

        if self_references
            .iter()
            .any(|self_ref| doc.value().contains(self_ref))
        {
            bail!(
                &doc.span(),
                "the documentation should not reference `Self` because it will \
                be moved to the {context} where `Self` changes meaning, which \
                may confuse the reader of this code; use explicit type names instead.",
            );
        }
    }

    Ok(())
}
