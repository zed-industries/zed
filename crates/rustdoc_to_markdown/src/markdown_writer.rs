use std::cell::RefCell;
use std::collections::VecDeque;

use anyhow::Result;
use html5ever::Attribute;
use markup5ever_rcdom::{Handle, NodeData};

#[derive(Debug, Clone)]
struct HtmlElement {
    tag: String,
    attrs: RefCell<Vec<Attribute>>,
}

enum StartTagOutcome {
    Continue,
    Skip,
}

pub struct MarkdownWriter {
    current_element_stack: VecDeque<HtmlElement>,
    /// The Markdown output.
    markdown: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        Self {
            current_element_stack: VecDeque::new(),
            markdown: String::new(),
        }
    }

    fn is_inside(&self, tag: &str) -> bool {
        self.current_element_stack
            .iter()
            .any(|parent_element| parent_element.tag == tag)
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
        Ok(self.markdown.trim().to_string())
    }

    fn visit_node(&mut self, node: &Handle) -> Result<()> {
        let mut current_element = None;

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
                let tag_name = name.local.to_string();
                if !tag_name.is_empty() {
                    current_element = Some(HtmlElement {
                        tag: tag_name,
                        attrs: attrs.clone(),
                    });
                }
            }
            NodeData::Text { ref contents } => {
                let text = contents.borrow().to_string();
                self.visit_text(text)?;
            }
        }

        if let Some(current_element) = current_element.as_ref() {
            match self.start_tag(&current_element) {
                StartTagOutcome::Continue => {}
                StartTagOutcome::Skip => return Ok(()),
            }

            self.current_element_stack
                .push_back(current_element.clone());
        }

        for child in node.children.borrow().iter() {
            self.visit_node(child)?;
        }

        self.current_element_stack.pop_back();

        if let Some(current_element) = current_element {
            self.end_tag(&current_element);
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: &HtmlElement) -> StartTagOutcome {
        match tag.tag.as_str() {
            "head" | "script" | "nav" => return StartTagOutcome::Skip,
            "h1" => self.push_str("\n# "),
            "h2" => self.push_str("\n## "),
            "h3" => self.push_str("\n### "),
            "h4" => self.push_str("\n#### "),
            "h5" => self.push_str("\n##### "),
            "h6" => self.push_str("\n###### "),
            "code" => {
                if !self.is_inside("pre") {
                    self.push_str("`")
                }
            }
            "pre" => self.push_str("\n```\n"),
            "ul" | "ol" => self.push_newline(),
            "li" => self.push_str("- "),
            "summary" => {
                if tag.attrs.borrow().iter().any(|attr| {
                    attr.name.local.to_string() == "class" && attr.value.to_string() == "hideme"
                }) {
                    return StartTagOutcome::Skip;
                }
            }
            "div" | "span" => {
                let classes_to_skip = ["nav-container", "sidebar-elems", "out-of-band"];

                if tag.attrs.borrow().iter().any(|attr| {
                    attr.name.local.to_string() == "class"
                        && attr
                            .value
                            .split(' ')
                            .any(|class| classes_to_skip.contains(&class.trim()))
                }) {
                    return StartTagOutcome::Skip;
                }

                if tag.attrs.borrow().iter().any(|attr| {
                    attr.name.local.to_string() == "class" && attr.value.to_string() == "item-name"
                }) {
                    self.push_str("`");
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn end_tag(&mut self, tag: &HtmlElement) {
        match tag.tag.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => self.push_str("\n\n"),
            "code" => {
                if !self.is_inside("pre") {
                    self.push_str("`")
                }
            }
            "pre" => self.push_str("\n```\n"),
            "ul" | "ol" => self.push_newline(),
            "li" => self.push_newline(),
            "div" => {
                if tag.attrs.borrow().iter().any(|attr| {
                    attr.name.local.to_string() == "class" && attr.value.to_string() == "item-name"
                }) {
                    self.push_str("`: ");
                }
            }
            _ => {}
        }
    }

    fn visit_text(&mut self, text: String) -> Result<()> {
        if self.is_inside("pre") {
            self.push_str(&text);
            return Ok(());
        }

        let trimmed_text = text.trim_matches(|char| char == '\n' || char == '\r' || char == '§');
        self.push_str(trimmed_text);

        Ok(())
    }
}
