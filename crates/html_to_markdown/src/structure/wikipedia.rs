use crate::HandleTag;
use crate::html_element::HtmlElement;
use crate::markdown_writer::{HandlerOutcome, MarkdownWriter, StartTagOutcome};

pub struct WikipediaChromeRemover;

impl HandleTag for WikipediaChromeRemover {
    fn should_handle(&self, _tag: &str) -> bool {
        true
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "head" | "script" | "style" | "nav" => return StartTagOutcome::Skip,
            "sup" => {
                if tag.has_class("reference") {
                    return StartTagOutcome::Skip;
                }
            }
            "div" | "span" | "a" => {
                if tag.attr("id").as_deref() == Some("p-lang-btn") {
                    return StartTagOutcome::Skip;
                }

                if tag.attr("id").as_deref() == Some("p-search") {
                    return StartTagOutcome::Skip;
                }

                let classes_to_skip = ["noprint", "mw-editsection", "mw-jump-link"];
                if tag.has_any_classes(&classes_to_skip) {
                    return StartTagOutcome::Skip;
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
}

pub struct WikipediaInfoboxHandler;

impl HandleTag for WikipediaInfoboxHandler {
    fn should_handle(&self, tag: &str) -> bool {
        tag == "table"
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        if tag.tag() == "table" && tag.has_class("infobox") {
            return StartTagOutcome::Skip;
        }

        StartTagOutcome::Continue
    }
}

pub struct WikipediaCodeHandler {
    language: Option<String>,
}

impl WikipediaCodeHandler {
    pub fn new() -> Self {
        Self { language: None }
    }
}

impl Default for WikipediaCodeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleTag for WikipediaCodeHandler {
    fn should_handle(&self, tag: &str) -> bool {
        matches!(tag, "div" | "pre" | "code")
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
            "div" => {
                let classes = tag.classes();
                self.language = classes.iter().find_map(|class| {
                    if let Some((_, language)) = class.split_once("mw-highlight-lang-") {
                        Some(language.trim().to_owned())
                    } else {
                        None
                    }
                });
            }
            "pre" => {
                writer.push_blank_line();
                writer.push_str("```");
                if let Some(language) = self.language.take() {
                    writer.push_str(&language);
                }
                writer.push_newline();
            }
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::{TagHandler, convert_html_to_markdown, markdown};

    use super::*;

    fn wikipedia_handlers() -> Vec<TagHandler> {
        vec![
            Rc::new(RefCell::new(markdown::ParagraphHandler)),
            Rc::new(RefCell::new(markdown::HeadingHandler)),
            Rc::new(RefCell::new(markdown::ListHandler)),
            Rc::new(RefCell::new(markdown::StyledTextHandler)),
            Rc::new(RefCell::new(WikipediaChromeRemover)),
        ]
    }

    #[test]
    fn test_citation_references_get_removed() {
        let html = indoc! {r##"
            <p>Rust began as a personal project in 2006 by <a href="/wiki/Mozilla" title="Mozilla">Mozilla</a> Research employee Graydon Hoare.<sup id="cite_ref-MITTechReview_23-0" class="reference"><a href="#cite_note-MITTechReview-23">[20]</a></sup> Mozilla began sponsoring the project in 2009 as a part of the ongoing development of an experimental <a href="/wiki/Browser_engine" title="Browser engine">browser engine</a> called <a href="/wiki/Servo_(software)" title="Servo (software)">Servo</a>,<sup id="cite_ref-infoq2012_24-0" class="reference"><a href="#cite_note-infoq2012-24">[21]</a></sup> which was officially announced by Mozilla in 2010.<sup id="cite_ref-MattAsay_25-0" class="reference"><a href="#cite_note-MattAsay-25">[22]</a></sup><sup id="cite_ref-26" class="reference"><a href="#cite_note-26">[23]</a></sup> Rust's memory and ownership system was influenced by <a href="/wiki/Region-based_memory_management" title="Region-based memory management">region-based memory management</a> in languages such as <a href="/wiki/Cyclone_(programming_language)" title="Cyclone (programming language)">Cyclone</a> and ML Kit.<sup id="cite_ref-influences_8-13" class="reference"><a href="#cite_note-influences-8">[5]</a></sup>
            </p>
        "##};
        let expected = indoc! {"
            Rust began as a personal project in 2006 by Mozilla Research employee Graydon Hoare.  Mozilla began sponsoring the project in 2009 as a part of the ongoing development of an experimental browser engine called Servo,  which was officially announced by Mozilla in 2010.  Rust's memory and ownership system was influenced by region-based memory management in languages such as Cyclone and ML Kit.
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut wikipedia_handlers()).unwrap(),
            expected
        )
    }
}
