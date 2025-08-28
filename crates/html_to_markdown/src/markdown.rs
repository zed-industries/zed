use crate::html_element::HtmlElement;
use crate::markdown_writer::{HandleTag, HandlerOutcome, MarkdownWriter, StartTagOutcome};

pub struct WebpageChromeRemover;

impl HandleTag for WebpageChromeRemover {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "head" | "script" | "style" | "nav")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "head" | "script" | "style" | "nav" => return StartTagOutcome::Skip,
            _ => {}
        }

        StartTagOutcome::Continue
    }
}

pub struct ParagraphHandler;

impl HandleTag for ParagraphHandler {
    fn should_handle(&self, _tag: &str) -> bool {
        true
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        if tag.is_inline()
            && writer.is_inside("p")
            && let Some(parent) = writer.current_element_stack().iter().last()
            && !(parent.is_inline()
                || writer.markdown.ends_with(' ')
                || writer.markdown.ends_with('\n'))
        {
            writer.push_str(" ");
        }

        if tag.tag() == "p" {
            writer.push_blank_line()
        }
        StartTagOutcome::Continue
    }
}

pub struct HeadingHandler;

impl HandleTag for HeadingHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
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
        match tag.tag() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => writer.push_blank_line(),
            _ => {}
        }
    }
}

pub struct ListHandler;

impl HandleTag for ListHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "ul" | "ol" | "li")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "ul" | "ol" => writer.push_newline(),
            "li" => writer.push_str("- "),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag() {
            "ul" | "ol" => writer.push_newline(),
            "li" => writer.push_newline(),
            _ => {}
        }
    }
}

pub struct TableHandler {
    /// The number of columns in the current `<table>`.
    current_table_columns: usize,
    is_first_th: bool,
    is_first_td: bool,
}

impl TableHandler {
    pub fn new() -> Self {
        Self {
            current_table_columns: 0,
            is_first_th: true,
            is_first_td: true,
        }
    }
}

impl Default for TableHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleTag for TableHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "table" | "thead" | "tbody" | "tr" | "th" | "td")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
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
        match tag.tag() {
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

pub struct StyledTextHandler;

impl HandleTag for StyledTextHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "strong" | "em")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "strong" => writer.push_str("**"),
            "em" => writer.push_str("_"),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag() {
            "strong" => writer.push_str("**"),
            "em" => writer.push_str("_"),
            _ => {}
        }
    }
}

pub struct CodeHandler;

impl HandleTag for CodeHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "pre" | "code")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => writer.push_str("\n\n```\n"),
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag() {
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
            writer.push_str(text);
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}
