//! Provides conversion from rustdoc's HTML output to Markdown.

#![deny(missing_docs)]

mod markdown_writer;

use std::io::Read;

use anyhow::{Context, Result};
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

use crate::markdown_writer::MarkdownWriter;

/// Converts the provided rustdoc HTML to Markdown.
pub fn convert_rustdoc_to_markdown(mut html: impl Read) -> Result<String> {
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
        .context("failed to parse rustdoc HTML")?;

    let markdown_writer = MarkdownWriter::new();
    let markdown = markdown_writer
        .run(&dom.document)
        .context("failed to convert rustdoc to HTML")?;

    Ok(markdown)
}
