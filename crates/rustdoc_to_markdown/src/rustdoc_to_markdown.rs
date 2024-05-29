use std::collections::VecDeque;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

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

    println!("{}", markdown_writer.markdown);
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

struct MarkdownWriter {
    current_tag_stack: VecDeque<String>,
    /// The Markdown output.
    markdown: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        Self {
            current_tag_stack: VecDeque::new(),
            markdown: String::new(),
        }
    }

    pub fn push_tag(&mut self, tag: String) {
        if tag.is_empty() {
            return;
        }

        self.current_tag_stack.push_back(tag);
    }

    pub fn pop_tag(&mut self) {
        self.current_tag_stack.pop_back();
    }

    pub fn is_inside(&self, tag: &str) -> bool {
        self.current_tag_stack
            .iter()
            .any(|parent_tag| parent_tag.contains(tag))
    }

    pub fn is_inside_heading(&self) -> bool {
        ["h1", "h2", "h3", "h4", "h5", "h6"]
            .into_iter()
            .any(|heading| self.is_inside(heading))
    }

    /// Appends the given string slice onto the end of the Markdown output.
    pub fn push_str(&mut self, str: &str) {
        self.markdown.push_str(str);
    }

    /// Appends a newline to the end of the Markdown output.
    pub fn push_newline(&mut self) {
        self.push_str("\n");
    }
}
