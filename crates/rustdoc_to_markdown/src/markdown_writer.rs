use std::cell::RefCell;
use std::collections::VecDeque;

use anyhow::Result;
use html5ever::Attribute;
use markup5ever_rcdom::{Handle, NodeData};

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

    fn is_inside(&self, tag: &str) -> bool {
        self.current_tag_stack
            .iter()
            .any(|parent_tag| parent_tag.contains(tag))
    }

    /// Appends the given string slice onto the end of the Markdown output.
    fn push_str(&mut self, str: &str) {
        self.markdown.push_str(str);
    }

    /// Appends a newline to the end of the Markdown output.
    fn push_newline(&mut self) {
        self.push_str("\n");
    }

    pub fn run(mut self, root_node: &Handle) -> Result<String> {
        self.visit_node(&root_node)?;
        Ok(self.markdown)
    }

    fn visit_node(&mut self, node: &Handle) -> Result<()> {
        let mut tag_name = String::new();

        match node.data {
            NodeData::Document
            | NodeData::Doctype { .. }
            | NodeData::ProcessingInstruction { .. }
            | NodeData::Comment { .. } => {
                // Currently left unimplemented, as we're not interested in this data
                // at this time.
            }
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                tag_name = name.local.to_string();
                self.start_tag(&tag_name, attrs);
            }
            NodeData::Text { ref contents } => {
                let text = contents.borrow().to_string();
                self.visit_text(text)?;
            }
        }

        if !tag_name.is_empty() {
            self.current_tag_stack.push_back(tag_name);
        }

        for child in node.children.borrow().iter() {
            self.visit_node(child)?;
        }

        if let Some(tag_name) = self.current_tag_stack.pop_back() {
            self.end_tag(&tag_name);
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: &str, _attrs: &RefCell<Vec<Attribute>>) {
        match tag {
            "h1" => self.push_str("# "),
            "h2" => self.push_str("## "),
            "h3" => self.push_str("### "),
            "h4" => self.push_str("#### "),
            "h5" => self.push_str("##### "),
            "h6" => self.push_str("###### "),
            "code" => self.push_str("`"),
            "pre" => self.push_str("\n```\n"),
            "li" => self.push_str("- "),
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: &str) {
        match tag {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => self.push_newline(),
            "code" => self.push_str("`"),
            "pre" => self.push_str("\n```\n"),
            "li" => self.push_newline(),
            _ => {}
        }
    }

    fn visit_text(&mut self, text: String) -> Result<()> {
        if self.is_inside("script") || self.is_inside("nav") {
            return Ok(());
        }

        if self.is_inside("pre") {
            self.push_str(&text);
            return Ok(());
        }

        self.push_str(&format!("{text}"));

        Ok(())
    }
}
