use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::OnceLock;

use anyhow::Result;
use html5ever::Attribute;
use markup5ever_rcdom::{Handle, NodeData};
use regex::Regex;

fn empty_line_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"^\s*$").unwrap())
}

fn more_than_three_newlines_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\n{3,}").unwrap())
}

#[derive(Debug, Clone)]
struct HtmlElement {
    tag: String,
    attrs: RefCell<Vec<Attribute>>,
}

impl HtmlElement {
    /// Returns the attribute with the specified name.
    pub fn attr(&self, name: &str) -> Option<String> {
        self.attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.to_string() == name)
            .map(|attr| attr.value.to_string())
    }

    /// Returns the list of classes on this [`HtmlElement`].
    pub fn classes(&self) -> Vec<String> {
        self.attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.to_string() == "class")
            .map(|attr| {
                attr.value
                    .split(' ')
                    .map(|class| class.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Returns whether this [`HtmlElement`] has the specified class.
    pub fn has_class(&self, class: &str) -> bool {
        self.has_any_classes(&[class])
    }

    /// Returns whether this [`HtmlElement`] has any of the specified classes.
    pub fn has_any_classes(&self, classes: &[&str]) -> bool {
        self.attrs.borrow().iter().any(|attr| {
            attr.name.local.to_string() == "class"
                && attr
                    .value
                    .split(' ')
                    .any(|class| classes.contains(&class.trim()))
        })
    }
}

const RUSTDOC_ITEM_NAME_CLASS: &str = "item-name";

enum StartTagOutcome {
    Continue,
    Skip,
}

pub struct MarkdownWriter {
    current_element_stack: VecDeque<HtmlElement>,
    /// The number of columns in the current `<table>`.
    current_table_columns: usize,
    is_first_th: bool,
    is_first_td: bool,
    /// The Markdown output.
    markdown: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        Self {
            current_element_stack: VecDeque::new(),
            current_table_columns: 0,
            is_first_th: true,
            is_first_td: true,
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
        self.visit_node(&root_node)?;
        Ok(Self::prettify_markdown(self.markdown))
    }

    fn prettify_markdown(markdown: String) -> String {
        let markdown = empty_line_regex().replace_all(&markdown, "");
        let markdown = more_than_three_newlines_regex().replace_all(&markdown, "\n\n");

        markdown.trim().to_string()
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

        if let Some(current_element) = current_element {
            self.current_element_stack.pop_back();
            self.end_tag(&current_element);
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: &HtmlElement) -> StartTagOutcome {
        match tag.tag.as_str() {
            "head" | "script" | "nav" => return StartTagOutcome::Skip,
            "h1" => self.push_str("\n\n# "),
            "h2" => self.push_str("\n\n## "),
            "h3" => self.push_str("\n\n### "),
            "h4" => self.push_str("\n\n#### "),
            "h5" => self.push_str("\n\n##### "),
            "h6" => self.push_str("\n\n###### "),
            "code" => {
                if !self.is_inside("pre") {
                    self.push_str("`");
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

                self.push_str(&format!("\n\n```{language}\n"));
            }
            "ul" | "ol" => self.push_newline(),
            "li" => self.push_str("- "),
            "thead" => self.push_blank_line(),
            "tr" => self.push_newline(),
            "th" => {
                self.current_table_columns += 1;
                if self.is_first_th {
                    self.is_first_th = false;
                } else {
                    self.push_str(" ");
                }
                self.push_str("| ");
            }
            "td" => {
                if self.is_first_td {
                    self.is_first_td = false;
                } else {
                    self.push_str(" ");
                }
                self.push_str("| ");
            }
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
                    self.push_str(" [");
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
                    self.push_str("`");
                }
            }
            "pre" => self.push_str("\n```\n"),
            "ul" | "ol" => self.push_newline(),
            "li" => self.push_newline(),
            "thead" => {
                self.push_newline();
                for ix in 0..self.current_table_columns {
                    if ix > 0 {
                        self.push_str(" ");
                    }
                    self.push_str("| ---");
                }
                self.push_str(" |");
                self.is_first_th = true;
            }
            "tr" => {
                self.push_str(" |");
                self.is_first_td = true;
            }
            "table" => {
                self.current_table_columns = 0;
            }
            "div" | "span" => {
                if tag.has_class(RUSTDOC_ITEM_NAME_CLASS) {
                    self.push_str(": ");
                }

                if self.is_inside_item_name() && tag.has_class("stab") {
                    self.push_str("]");
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

        if self.is_inside_item_name() && !self.is_inside("span") && !self.is_inside("code") {
            self.push_str(&format!("`{trimmed_text}`"));
            return Ok(());
        }

        self.push_str(trimmed_text);

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
