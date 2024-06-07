use crate::html_element::HtmlElement;
use crate::markdown_writer::{MarkdownWriter, StartTagOutcome};
use crate::HandleTag;

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
        match tag.tag.as_str() {
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

                let classes_to_skip = ["mw-editsection", "mw-jump-link"];
                if tag.has_any_classes(&classes_to_skip) {
                    return StartTagOutcome::Skip;
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::{convert_html_to_markdown, markdown};

    use super::*;

    fn wikipedia_handlers() -> Vec<Box<dyn HandleTag>> {
        vec![
            Box::new(markdown::ParagraphHandler),
            Box::new(markdown::HeadingHandler),
            Box::new(markdown::ListHandler),
            Box::new(markdown::StyledTextHandler),
            Box::new(WikipediaChromeRemover),
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
            convert_html_to_markdown(html.as_bytes(), wikipedia_handlers()).unwrap(),
            expected
        )
    }
}
