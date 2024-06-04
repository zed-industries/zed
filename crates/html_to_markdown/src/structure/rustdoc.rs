use crate::html_element::HtmlElement;
use crate::markdown_writer::{HandleTag, HandlerOutcome, MarkdownWriter, StartTagOutcome};

pub struct RustdocHeadingHandler;

impl HandleTag for RustdocHeadingHandler {
    fn should_handle(&self, _tag: &str) -> bool {
        // We're only handling text, so we don't need to visit any tags.
        false
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("h1")
            || writer.is_inside("h2")
            || writer.is_inside("h3")
            || writer.is_inside("h4")
            || writer.is_inside("h5")
            || writer.is_inside("h6")
        {
            let text = text
                .trim_matches(|char| char == '\n' || char == '\r' || char == '§')
                .replace('\n', " ");
            writer.push_str(&text);

            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

pub struct RustdocCodeHandler;

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

pub struct RustdocTableHandler {
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

const RUSTDOC_ITEM_NAME_CLASS: &str = "item-name";

pub struct RustdocItemHandler;

impl RustdocItemHandler {
    /// Returns whether we're currently inside of an `.item-name` element, which
    /// rustdoc uses to display Rust items in a list.
    fn is_inside_item_name(writer: &MarkdownWriter) -> bool {
        writer
            .current_element_stack()
            .iter()
            .any(|element| element.has_class(RUSTDOC_ITEM_NAME_CLASS))
    }
}

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
                if Self::is_inside_item_name(writer) && tag.has_class("stab") {
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

                if Self::is_inside_item_name(writer) && tag.has_class("stab") {
                    writer.push_str("]");
                }
            }
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if Self::is_inside_item_name(writer)
            && !writer.is_inside("span")
            && !writer.is_inside("code")
        {
            writer.push_str(&format!("`{text}`"));
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

pub struct RustdocChromeRemover;

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
