mod markdown_writer;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

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
    let mut markdown_writer = MarkdownWriter::new();

    walk(&dom.document, &mut markdown_writer);

    println!("{}", markdown_writer.markdown());
}

fn walk(node: &Handle, output: &mut MarkdownWriter) {
    let mut tag_name = String::new();

    match node.data {
        NodeData::Document
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. }
        | NodeData::Comment { .. } => {}
        NodeData::Element { ref name, .. } => {
            tag_name = name.local.to_string();
        }
        NodeData::Text { ref contents } => {
            let text = contents.borrow().to_string();

            if output.is_inside_heading() {
                output.push_str(&format!("# {text}"));
                output.push_newline();
            } else {
                output.push_str(&format!("{text}"));
                output.push_newline();
            }
        }
    }

    output.push_tag(tag_name);

    for child in node.children.borrow().iter() {
        walk(child, output);
    }

    output.pop_tag();
}
