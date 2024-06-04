use std::collections::VecDeque;
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

const RUSTDOC_ITEM_NAME_CLASS: &str = "item-name";

enum StartTagOutcome {
    Continue,
    Skip,
}

pub struct MarkdownWriter {
    current_element_stack: VecDeque<HtmlElement>,
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

    /// Appends a blank line to the end of the Markdown output.
    fn push_blank_line(&mut self) {
        self.push_str("\n\n");
    }

    pub fn run(mut self, root_node: &Handle) -> Result<String> {
        let mut handlers: Vec<Box<dyn HandleTag>> = Vec::new();
        handlers.push(Box::new(HeadingHandler));
        handlers.push(Box::new(ListHandler));
        handlers.push(Box::new(RustdocChromeRemover));
        handlers.push(Box::new(RustdocCodeHandler));
        handlers.push(Box::new(RustdocTableHandler::new()));
        handlers.push(Box::new(RustdocItemHandler));

        self.visit_node(&root_node, &mut handlers)?;
        Ok(Self::prettify_markdown(self.markdown))
    }

    fn prettify_markdown(markdown: String) -> String {
        let markdown = empty_line_regex().replace_all(&markdown, "");
        let markdown = more_than_three_newlines_regex().replace_all(&markdown, "\n\n");

        markdown.trim().to_string()
    }

    fn visit_node(&mut self, node: &Handle, handlers: &mut [Box<dyn HandleTag>]) -> Result<()> {
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

    fn start_tag(
        &mut self,
        tag: &HtmlElement,
        handlers: &mut [Box<dyn HandleTag>],
    ) -> StartTagOutcome {
        if tag.is_inline() && self.is_inside("p") {
            if let Some(parent) = self.current_element_stack.iter().last() {
                if !parent.is_inline() {
                    if !(self.markdown.ends_with(' ') || self.markdown.ends_with('\n')) {
                        self.push_str(" ");
                    }
                }
            }
        }

        for handler in handlers {
            if handler.should_handle(tag.tag.as_str()) {
                match handler.handle_tag_start(tag, self) {
                    StartTagOutcome::Continue => {}
                    StartTagOutcome::Skip => return StartTagOutcome::Skip,
                }
            }
        }

        match tag.tag.as_str() {
            "p" => self.push_blank_line(),
            "strong" => self.push_str("**"),
            "em" => self.push_str("_"),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn end_tag(&mut self, tag: &HtmlElement, handlers: &mut [Box<dyn HandleTag>]) {
        for handler in handlers {
            if handler.should_handle(tag.tag.as_str()) {
                handler.handle_tag_end(tag, self);
            }
        }

        match tag.tag.as_str() {
            "strong" => self.push_str("**"),
            "em" => self.push_str("_"),
            _ => {}
        }
    }

    fn visit_text(&mut self, text: String, handlers: &mut [Box<dyn HandleTag>]) -> Result<()> {
        let mut did_handle = false;

        for handler in handlers {
            match handler.handle_text(&text, self) {
                HandlerOutcome::Handled => did_handle = true,
                HandlerOutcome::NoOp => {}
            }
        }

        if did_handle {
            return Ok(());
        }

        let text = text
            .trim_matches(|char| char == '\n' || char == '\r' || char == '§')
            .replace('\n', " ");

        self.push_str(&text);

        Ok(())
    }

    /// Returns whether we're currently inside of an `.item-name` element, which
    /// rustdoc uses to display Rust items in a list.
    fn is_inside_item_name(&self) -> bool {
        self.current_element_stack
            .iter()
            .any(|element| element.has_class(RUSTDOC_ITEM_NAME_CLASS))
    }
}

enum HandlerOutcome {
    Handled,
    NoOp,
}

trait HandleTag {
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

struct HeadingHandler;

impl HandleTag for HeadingHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "h1" => writer.push_str("\n\n# "),
            "h2" => writer.push_str("\n\n## "),
            "h3" => writer.push_str("\n\n### "),
            "h4" => writer.push_str("\n\n#### "),
            "h5" => writer.push_str("\n\n##### "),
            "h6" => writer.push_str("\n\n###### "),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => writer.push_blank_line(),
            _ => {}
        }
    }
}

struct ListHandler;

impl HandleTag for ListHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "ul" | "ol" | "li" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "ul" | "ol" => writer.push_newline(),
            "li" => writer.push_str("- "),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "ul" | "ol" => writer.push_newline(),
            "li" => writer.push_newline(),
            _ => {}
        }
    }
}

struct RustdocCodeHandler;

impl HandleTag for RustdocCodeHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "pre" | "code" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => {
                let classes = tag.classes();
                let is_rust = classes.iter().any(|class| class == "rust");
                let language = is_rust
                    .then(|| "rs")
                    .or_else(|| {
                        classes.iter().find_map(|class| {
                            if let Some((_, language)) = class.split_once("language-") {
                                Some(language.trim())
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or("");

                writer.push_str(&format!("\n\n```{language}\n"));
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => writer.push_str("\n```\n"),
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("pre") {
            writer.push_str(&text);
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

struct RustdocTableHandler {
    /// The number of columns in the current `<table>`.
    current_table_columns: usize,
    is_first_th: bool,
    is_first_td: bool,
}

impl RustdocTableHandler {
    pub fn new() -> Self {
        Self {
            current_table_columns: 0,
            is_first_th: true,
            is_first_td: true,
        }
    }
}

impl HandleTag for RustdocTableHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "table" | "thead" | "tbody" | "tr" | "th" | "td" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "thead" => writer.push_blank_line(),
            "tr" => writer.push_newline(),
            "th" => {
                self.current_table_columns += 1;
                if self.is_first_th {
                    self.is_first_th = false;
                } else {
                    writer.push_str(" ");
                }
                writer.push_str("| ");
            }
            "td" => {
                if self.is_first_td {
                    self.is_first_td = false;
                } else {
                    writer.push_str(" ");
                }
                writer.push_str("| ");
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "thead" => {
                writer.push_newline();
                for ix in 0..self.current_table_columns {
                    if ix > 0 {
                        writer.push_str(" ");
                    }
                    writer.push_str("| ---");
                }
                writer.push_str(" |");
                self.is_first_th = true;
            }
            "tr" => {
                writer.push_str(" |");
                self.is_first_td = true;
            }
            "table" => {
                self.current_table_columns = 0;
            }
            _ => {}
        }
    }
}

struct RustdocItemHandler;

impl HandleTag for RustdocItemHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "div" | "span" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "div" | "span" => {
                if writer.is_inside_item_name() && tag.has_class("stab") {
                    writer.push_str(" [");
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "div" | "span" => {
                if tag.has_class(RUSTDOC_ITEM_NAME_CLASS) {
                    writer.push_str(": ");
                }

                if writer.is_inside_item_name() && tag.has_class("stab") {
                    writer.push_str("]");
                }
            }
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside_item_name() && !writer.is_inside("span") && !writer.is_inside("code") {
            writer.push_str(&format!("`{text}`"));
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

struct RustdocChromeRemover;

impl HandleTag for RustdocChromeRemover {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "head" | "script" | "nav" | "summary" | "button" | "div" | "span" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "head" | "script" | "nav" => return StartTagOutcome::Skip,
            "summary" => {
                if tag.has_class("hideme") {
                    return StartTagOutcome::Skip;
                }
            }
            "button" => {
                if tag.attr("id").as_deref() == Some("copy-path") {
                    return StartTagOutcome::Skip;
                }
            }
            "div" | "span" => {
                let classes_to_skip = ["nav-container", "sidebar-elems", "out-of-band"];
                if tag.has_any_classes(&classes_to_skip) {
                    return StartTagOutcome::Skip;
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
}
