//! Provides conversion from rustdoc's HTML output to Markdown.

mod html_element;
pub mod markdown;
mod markdown_writer;
pub mod structure;

use std::io::Read;

use anyhow::{Context, Result};
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

pub use crate::html_element::*;
pub use crate::markdown_writer::*;

/// Converts the provided HTML to Markdown.
pub fn convert_html_to_markdown(html: impl Read, handlers: &mut Vec<TagHandler>) -> Result<String> {
    let dom = parse_html(html).context("failed to parse HTML")?;

    let markdown_writer = MarkdownWriter::new();
    let markdown = markdown_writer
        .run(&dom.document, handlers)
        .context("failed to convert HTML to Markdown")?;

    Ok(markdown)
}

fn parse_html(mut html: impl Read) -> Result<RcDom> {
    let parse_options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), parse_options)
        .from_utf8()
        .read_from(&mut html)
        .context("failed to parse HTML document")?;

    Ok(dom)
}
