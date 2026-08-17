use quote::quote;
use syn::{parse::Parser, Attribute, Meta};

use crate::args::Attrs;
use crate::error;

const DOC: &str = "doc";
const OPT_ATTR: &str = "optfield";

pub trait AttrGenerator {
    fn no_docs(&self) -> bool;

    fn error_action_text(&self) -> String;

    fn original_attrs(&self) -> &[Attribute];

    fn attrs_arg(&self) -> &Option<Attrs>;

    fn custom_docs(&self) -> Option<Meta> {
        None
    }

    fn keep_original_docs(&self) -> bool {
        !self.no_docs()
    }

    fn compute_capacity(&self) -> usize {
        use Attrs::*;

        let orig_len = self.original_attrs().len();

        match self.attrs_arg() {
            Some(Replace(v)) | Some(Add(v)) => orig_len + v.len(),
            _ => orig_len,
        }
    }

    fn generate(&self) -> Vec<Attribute> {
        use Attrs::*;

        let attrs_arg = self.attrs_arg();

        // if no attrs and no docs should be set, remove all attrs
        if self.no_docs() && attrs_arg.is_none() {
            return Vec::new();
        }

        let mut new_attrs = Vec::with_capacity(self.compute_capacity());

        if let Some(d) = self.custom_docs() {
            new_attrs.push(d);
        }

        for attr in self.original_attrs() {
            let mut add_attr = self.keep_original_docs() && is_doc_attr(attr);

            if let Some(Keep) | Some(Add(_)) = attrs_arg {
                if !is_doc_attr(attr) {
                    add_attr = true;
                }
            }

            if attr.path().is_ident(OPT_ATTR) {
                add_attr = false
            }

            if add_attr {
                new_attrs.push(attr.meta.clone());
            }
        }

        if let Some(Replace(v)) | Some(Add(v)) = attrs_arg {
            new_attrs.extend(v.clone());
        }

        let attrs_tokens = quote! {
            #(#[#new_attrs])*
        };

        Attribute::parse_outer
            .parse2(attrs_tokens)
            .unwrap_or_else(|e| panic!("{}", error::unexpected(self.error_action_text(), e)))
    }
}

pub fn is_doc_attr(attr: &Attribute) -> bool {
    attr.path().is_ident(DOC)
}
