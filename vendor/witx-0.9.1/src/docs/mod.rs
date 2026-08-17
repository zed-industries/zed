mod ast;
mod md;

use crate::ast::Document;
use md::{MdNodeRef, MdRoot, ToMarkdown};
use std::{
    collections::{hash_map, HashSet},
    iter::FromIterator,
};

/// Enables generating Markdown formatted content.
pub trait Documentation {
    fn to_md(&self) -> String;
}

/// Helper function which given input `text` and a `HashSet` of existing links converts
/// any slice of the form '`{link}`' into either
/// 1. "[`{link}`](#{md_link})" where `md_link` is `link` with "::" replaced with "."
///    (in Markdown, scoping should be done with ".") if `md_link` exists in the `HashSet`
/// 2. "`{link}`" otherwise. That is, if `md_link` could not be found in the `HashSet`, we
///    just leave what we've consumed.
fn parse_links<S: AsRef<str>>(text: S, existing_links: &HashSet<String>) -> String {
    let text = text.as_ref();
    let mut parsed_text = String::with_capacity(text.len());
    let mut link = String::with_capacity(text.len());
    let mut is_link = false;

    for ch in text.chars() {
        match (ch, is_link) {
            // Found the beginning of a link!
            ('`', false) => {
                is_link = true;
            }
            // Reached the end, expand into a link!
            ('`', true) => {
                // Sanitise scoping by replacing "::" with '.'
                let md_link = link.replace("::", ".");
                // Before committing to pasting the link in,
                // first verify that it actually exists.
                let expanded = if let Some(_) = existing_links.get(&md_link) {
                    format!("[`{}`](#{})", link, md_link)
                } else {
                    log::warn!(
                        "Link [`{}`](#{}) could not be found in the document!",
                        link,
                        md_link
                    );
                    format!("`{}`", link)
                };
                parsed_text.push_str(&expanded);
                link.drain(..);
                is_link = false;
            }
            (ch, false) => parsed_text.push(ch),
            (ch, true) => link.push(ch),
        }
    }

    parsed_text
}

impl Documentation for Document {
    fn to_md(&self) -> String {
        let root = MdNodeRef::new(MdRoot::default());
        self.generate(root.clone());
        // Get all children of the `root` element.
        let children = root.borrow().children();
        // Gather all existing links in the document into a set.
        let existing_links: HashSet<String, hash_map::RandomState> = HashSet::from_iter(
            children
                .iter()
                .filter_map(|x| x.any_ref().id().map(String::from)),
        );
        // Traverse each docs section of each child, and parse links
        // logging a warning in case the generated is invalid.
        for child in children {
            let docs_with_links = child
                .any_ref()
                .docs()
                .map(|docs| parse_links(docs, &existing_links));
            if let Some(docs) = docs_with_links {
                child.any_ref_mut().set_docs(&docs);
            }
        }
        format!("{}", root)
    }
}
