mod accent_colors;
mod diagram_recolor;
mod element_fixup;
mod fallback_fixup;
mod foreignobject_wrap;
mod inject_css;
mod sanitize_nan;
mod strip_foreignobject;
mod strip_invalid_css;
pub(crate) mod util;

use anyhow::{Context as _, Result};
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::MermaidTheme;

pub(super) fn postprocess(svg: &str, theme: &MermaidTheme) -> Result<String> {
    // Pass 1: foreignObject preparation (\n fix + word wrapping)
    let svg = foreignobject_wrap::process(svg)?;

    // Add <text> fallbacks alongside <foreignObject> elements
    let svg = merman::render::foreign_object_label_fallback_svg_text(&svg);

    // Extract SVG id for CSS scoping (quick scan of the first element)
    let svg_id = extract_svg_id(&svg);

    // Pass 2: themed post-processing pipeline.
    // Each adapter takes an iterator of events and returns an iterator of events.
    // Events borrow from the `svg` string — no .into_owned() per event.
    let mut reader = Reader::from_str(&svg);
    reader.config_mut().check_end_names = false;
    let events = ReaderIter::new(reader);
    let events = strip_foreignobject::process(events);
    let events = fallback_fixup::process(events, theme);
    let events = element_fixup::process(events, theme);
    let events = diagram_recolor::process(events, theme);
    let events = accent_colors::process(events, theme);
    let events = strip_invalid_css::process(events);
    let events = inject_css::process(events, theme, &svg_id);
    let events = sanitize_nan::process(events);

    let mut writer = quick_xml::Writer::new(Vec::new());
    for event in events {
        writer.write_event(event?)?;
    }
    String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")
}

fn extract_svg_id(svg: &str) -> String {
    let mut reader = Reader::from_str(svg);
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"svg" {
                    return e
                        .try_get_attribute("id")
                        .ok()
                        .flatten()
                        .and_then(|a| a.unescape_value().ok())
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                }
            }
            Ok(Event::Eof) | Err(_) => return String::new(),
            _ => {}
        }
    }
}

struct ReaderIter<'a> {
    reader: Reader<&'a [u8]>,
    done: bool,
}

impl<'a> ReaderIter<'a> {
    fn new(reader: Reader<&'a [u8]>) -> Self {
        Self { reader, done: false }
    }
}

impl<'a> Iterator for ReaderIter<'a> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.reader.read_event() {
            Ok(Event::Eof) => {
                self.done = true;
                None
            }
            Ok(event) => Some(Ok(event)),
            Err(e) => {
                self.done = true;
                Some(Err(e.into()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_theme() -> MermaidTheme {
        MermaidTheme::default()
    }

    #[test]
    fn hsl_nan_in_text_content_is_preserved() {
        let svg = r##"<svg id="test" xmlns="http://www.w3.org/2000/svg"><style>.node rect { fill: red; }</style><text fill="#333">hsl(NaN, 0%, 50%)</text></svg>"##;
        let result = postprocess(svg, &default_theme()).unwrap();
        assert!(
            result.contains("hsl(NaN, 0%, 50%)"),
            "NaN in <text> content should be preserved, got: {result}"
        );
    }

    #[test]
    fn strip_css_handles_style_element_with_attributes() {
        let svg = r#"<svg id="test" xmlns="http://www.w3.org/2000/svg"><style type="text/css">@keyframes bounce { 0% { transform: scale(1); } 100% { transform: scale(1.1); } } .node rect { fill: red; }</style><rect width="10" height="10"/></svg>"#;
        let result = postprocess(svg, &default_theme()).unwrap();
        assert!(
            !result.contains("@keyframes"),
            "Unsupported @keyframes should be stripped from <style type=\"text/css\">, got: {result}"
        );
        assert!(
            result.contains(".node rect"),
            "Regular CSS rules should survive stripping, got: {result}"
        );
    }
}
