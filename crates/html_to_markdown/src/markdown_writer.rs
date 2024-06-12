use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::OnceLock;

use anyhow::Result;
use markup5ever_rcdom::{Handle, NodeData};
use regex::Regex;

use crate::html_element::HtmlElement;

fn empty_line_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^\s*$").unwrap())
}

fn more_than_three_newlines_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\n{3,}").unwrap())
}

pub enum StartTagOutcome {
    Continue,
    Skip,
}

pub type TagHandler = Rc<RefCell<dyn HandleTag>>;

pub struct MarkdownWriter {
    current_element_stack: VecDeque<HtmlElement>,
    pub(crate) markdown: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        Self {
            current_element_stack: VecDeque::new(),
            markdown: String::new(),
        }
    }

    pub fn current_element_stack(&self) -> &VecDeque<HtmlElement> {
        &self.current_element_stack
    }

    pub fn is_inside(&self, tag: &str) -> bool {
        self.current_element_stack
            .iter()
            .any(|parent_element| parent_element.tag() == tag)
    }

    /// Appends the given string slice onto the end of the Markdown output.
    pub fn push_str(&mut self, str: &str) {
        self.markdown.push_str(str);
    }

    /// Appends a newline to the end of the Markdown output.
    pub fn push_newline(&mut self) {
        self.push_str("\n");
    }

    /// Appends a blank line to the end of the Markdown output.
    pub fn push_blank_line(&mut self) {
        self.push_str("\n\n");
    }

    pub fn run(mut self, root_node: &Handle, handlers: &mut Vec<TagHandler>) -> Result<String> {
        self.visit_node(&root_node, handlers)?;
        Ok(Self::prettify_markdown(self.markdown))
    }

    fn prettify_markdown(markdown: String) -> String {
        let markdown = empty_line_regex().replace_all(&markdown, "");
        let markdown = more_than_three_newlines_regex().replace_all(&markdown, "\n\n");

        markdown.trim().to_string()
    }

    fn visit_node(&mut self, node: &Handle, handlers: &mut [TagHandler]) -> Result<()> {
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
                    current_element = Some(HtmlElement::new(tag_name, attrs.clone()));
                }
            }
            NodeData::Text { ref contents } => {
                let text = contents.borrow().to_string();
                self.visit_text(text, handlers)?;
            }
        }

        if let Some(current_element) = current_element.as_ref() {
            match self.start_tag(&current_element, handlers) {
                StartTagOutcome::Continue => {}
                StartTagOutcome::Skip => return Ok(()),
            }

            self.current_element_stack
                .push_back(current_element.clone());
        }

        for child in node.children.borrow().iter() {
            self.visit_node(child, handlers)?;
        }

        if let Some(current_element) = current_element {
            self.current_element_stack.pop_back();
            self.end_tag(&current_element, handlers);
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: &HtmlElement, handlers: &mut [TagHandler]) -> StartTagOutcome {
        for handler in handlers {
            if handler.borrow().should_handle(tag.tag()) {
                match handler.borrow_mut().handle_tag_start(tag, self) {
                    StartTagOutcome::Continue => {}
                    StartTagOutcome::Skip => return StartTagOutcome::Skip,
                }
            }
        }

        StartTagOutcome::Continue
    }

    fn end_tag(&mut self, tag: &HtmlElement, handlers: &mut [TagHandler]) {
        for handler in handlers {
            if handler.borrow().should_handle(tag.tag()) {
                handler.borrow_mut().handle_tag_end(tag, self);
            }
        }
    }

    fn visit_text(&mut self, text: String, handlers: &mut [TagHandler]) -> Result<()> {
        for handler in handlers {
            match handler.borrow_mut().handle_text(&text, self) {
                HandlerOutcome::Handled => return Ok(()),
                HandlerOutcome::NoOp => {}
            }
        }

        let text = text
            .trim_matches(|char| char == '\n' || char == '\r' || char == '\t')
            .replace('\n', " ");

        self.push_str(&text);

        Ok(())
    }
}

pub enum HandlerOutcome {
    Handled,
    NoOp,
}

pub trait HandleTag {
    /// Returns whether this handler should handle the given tag.
    fn should_handle(&self, tag: &str) -> bool;

    /// Handles the start of the given tag.
    fn handle_tag_start(
        &mut self,
        _tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        StartTagOutcome::Continue
    }

    /// Handles the end of the given tag.
    fn handle_tag_end(&mut self, _tag: &HtmlElement, _writer: &mut MarkdownWriter) {}

    fn handle_text(&mut self, _text: &str, _writer: &mut MarkdownWriter) -> HandlerOutcome {
        HandlerOutcome::NoOp
    }
}
