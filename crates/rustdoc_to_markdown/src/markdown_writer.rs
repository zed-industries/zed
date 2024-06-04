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

pub struct MarkdownOutput(String);

impl MarkdownOutput {
    pub fn new() -> Self {
        Self(String::new())
    }

    /// Appends the given string slice onto the end of the Markdown output.
    fn push_str(&mut self, str: &str) {
        self.0.push_str(str);
    }

    /// Appends a newline to the end of the Markdown output.
    fn push_newline(&mut self) {
        self.push_str("\n");
    }

    /// Appends a blank line to the end of the Markdown output.
    fn push_blank_line(&mut self) {
        self.push_str("\n\n");
    }
}

pub struct MarkdownWriter {
    current_element_stack: VecDeque<HtmlElement>,
    handlers: Vec<Box<dyn HandleTag>>,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn HandleTag>> = Vec::new();
        handlers.push(Box::new(RustdocTableHandler::new()));

        Self {
            current_element_stack: VecDeque::new(),
            handlers,
        }
    }

    fn is_inside(&self, tag: &str) -> bool {
        self.current_element_stack
            .iter()
            .any(|parent_element| parent_element.tag == tag)
    }

    pub fn run(mut self, root_node: &Handle) -> Result<String> {
        let mut output = MarkdownOutput::new();
        self.visit_node(&root_node, &mut output)?;
        Ok(Self::prettify_markdown(output.0))
    }

    fn prettify_markdown(markdown: String) -> String {
        let markdown = empty_line_regex().replace_all(&markdown, "");
        let markdown = more_than_three_newlines_regex().replace_all(&markdown, "\n\n");

        markdown.trim().to_string()
    }

    fn visit_node(&mut self, node: &Handle, output: &mut MarkdownOutput) -> Result<()> {
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
                self.visit_text(text, output)?;
            }
        }

        if let Some(current_element) = current_element.as_ref() {
            match self.start_tag(&current_element, output) {
                StartTagOutcome::Continue => {}
                StartTagOutcome::Skip => return Ok(()),
            }

            self.current_element_stack
                .push_back(current_element.clone());
        }

        for child in node.children.borrow().iter() {
            self.visit_node(child, output)?;
        }

        if let Some(current_element) = current_element {
            self.current_element_stack.pop_back();
            self.end_tag(&current_element, output);
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: &HtmlElement, output: &mut MarkdownOutput) -> StartTagOutcome {
        for handler in &mut self.handlers {
            if handler.should_handle(tag.tag.as_str()) {
                match handler.handle_tag_start(tag, output) {
                    StartTagOutcome::Continue => {}
                    StartTagOutcome::Skip => return StartTagOutcome::Skip,
                }
            }
        }

        if tag.is_inline() && self.is_inside("p") {
            if let Some(parent) = self.current_element_stack.iter().last() {
                if !parent.is_inline() {
                    if !(output.0.ends_with(' ') || output.0.ends_with('\n')) {
                        output.push_str(" ");
                    }
                }
            }
        }

        match tag.tag.as_str() {
            "head" | "script" | "nav" => return StartTagOutcome::Skip,
            "h1" => output.push_str("\n\n# "),
            "h2" => output.push_str("\n\n## "),
            "h3" => output.push_str("\n\n### "),
            "h4" => output.push_str("\n\n#### "),
            "h5" => output.push_str("\n\n##### "),
            "h6" => output.push_str("\n\n###### "),
            "p" => output.push_blank_line(),
            "strong" => output.push_str("**"),
            "em" => output.push_str("_"),
            "code" => {
                if !self.is_inside("pre") {
                    output.push_str("`");
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

                output.push_str(&format!("\n\n```{language}\n"));
            }
            "ul" | "ol" => output.push_newline(),
            "li" => output.push_str("- "),
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

                if self.is_inside_item_name() && tag.has_class("stab") {
                    output.push_str(" [");
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn end_tag(&mut self, tag: &HtmlElement, output: &mut MarkdownOutput) {
        for handler in &mut self.handlers {
            if handler.should_handle(tag.tag.as_str()) {
                handler.handle_tag_end(tag, output);
            }
        }

        match tag.tag.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => output.push_str("\n\n"),
            "strong" => output.push_str("**"),
            "em" => output.push_str("_"),
            "code" => {
                if !self.is_inside("pre") {
                    output.push_str("`");
                }
            }
            "pre" => output.push_str("\n```\n"),
            "ul" | "ol" => output.push_newline(),
            "li" => output.push_newline(),
            "div" | "span" => {
                if tag.has_class(RUSTDOC_ITEM_NAME_CLASS) {
                    output.push_str(": ");
                }

                if self.is_inside_item_name() && tag.has_class("stab") {
                    output.push_str("]");
                }
            }
            _ => {}
        }
    }

    fn visit_text(&mut self, text: String, output: &mut MarkdownOutput) -> Result<()> {
        if self.is_inside("pre") {
            output.push_str(&text);
            return Ok(());
        }

        let text = text
            .trim_matches(|char| char == '\n' || char == '\r' || char == '§')
            .replace('\n', " ");

        if self.is_inside_item_name() && !self.is_inside("span") && !self.is_inside("code") {
            output.push_str(&format!("`{text}`"));
            return Ok(());
        }

        output.push_str(&text);

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

trait HandleTag {
    fn should_handle(&self, tag: &str) -> bool;

    fn handle_tag_start(
        &mut self,
        _tag: &HtmlElement,
        _output: &mut MarkdownOutput,
    ) -> StartTagOutcome {
        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, _tag: &HtmlElement, _output: &mut MarkdownOutput) {
        ()
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
        output: &mut MarkdownOutput,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "thead" => output.push_blank_line(),
            "tr" => output.push_newline(),
            "th" => {
                self.current_table_columns += 1;
                if self.is_first_th {
                    self.is_first_th = false;
                } else {
                    output.push_str(" ");
                }
                output.push_str("| ");
            }
            "td" => {
                if self.is_first_td {
                    self.is_first_td = false;
                } else {
                    output.push_str(" ");
                }
                output.push_str("| ");
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, output: &mut MarkdownOutput) {
        match tag.tag.as_str() {
            "thead" => {
                output.push_newline();
                for ix in 0..self.current_table_columns {
                    if ix > 0 {
                        output.push_str(" ");
                    }
                    output.push_str("| ---");
                }
                output.push_str(" |");
                self.is_first_th = true;
            }
            "tr" => {
                output.push_str(" |");
                self.is_first_td = true;
            }
            "table" => {
                self.current_table_columns = 0;
            }
            _ => {}
        }
    }
}
