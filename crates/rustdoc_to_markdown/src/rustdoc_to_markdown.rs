mod markdown_writer;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

pub use crate::markdown_writer::*;

pub fn convert_rustdoc_to_markdown() {
    let parse_options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let html = include_str!("/Users/maxdeviant/projects/zed/target/doc/gpui/index.html");

    let dom = parse_document(RcDom::default(), parse_options)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    let markdown_writer = MarkdownWriter::new();
    let markdown = markdown_writer.run(&dom.document).unwrap();

    println!("{}", markdown);
}

#[cfg(test)]
mod tests {}
