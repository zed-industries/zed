use std::collections::VecDeque;

use markup5ever_rcdom::{Handle, NodeData};

use crate::{walk_node, Visitor};

pub struct MarkdownWriter {
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

    pub fn markdown(&self) -> &str {
        &self.markdown
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

impl Visitor for MarkdownWriter {
    type Error = ();

    fn visit_node(&mut self, node: &Handle) -> Result<(), Self::Error> {
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

                if self.is_inside_heading() {
                    self.push_str(&format!("# {text}"));
                    self.push_newline();
                } else {
                    self.push_str(&format!("{text}"));
                    self.push_newline();
                }
            }
        }

        self.push_tag(tag_name);
        walk_node(self, node)?;
        self.pop_tag();

        Ok(())
    }
}
