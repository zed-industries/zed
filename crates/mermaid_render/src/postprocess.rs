//! Post-processing of [`merman`]-produced SVGs for rasterization with `usvg`/`resvg`.
//!
//! Each submodule is a specific pass that tweaks the SVG event iterator in a particular way.
//!
//! We always produce and consume [`Event`]s with a short lifetime.
//! [`Event<'a>`] is backed internally by a [`Cow<'a, [u8]>`](std::borrow::Cow),
//! so we don't have lifetime issues when we need to mutate the text in an
//! [`Event`], but also don't force allocating a new [`String`] each time.
//!
//! Many modules contain internal structs that implement [`Iterator`] to make
//! reasoning about lifetimes simpler, but these are private implementation
//! details.

mod accent_colors;
mod element_fixup;
mod fallback_fixup;
mod foreignobject_wrap;
mod inject_css;
mod strip_foreignobject;
mod strip_invalid_css;
pub(crate) mod util;

use anyhow::{Context as _, Result};
use quick_xml::Reader;
use quick_xml::events::Event;

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

    let events = accent_colors::process(events, theme);
    let events = strip_invalid_css::process(events);
    let events = inject_css::process(events, theme, &svg_id);

    let mut writer = quick_xml::Writer::new(Vec::with_capacity(svg.len()));
    for event in events {
        writer.write_event(event?)?;
    }
    String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")
}

fn extract_svg_id(svg: &str) -> String {
    let mut reader = Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    for event in ReaderIter::new(reader) {
        let Ok(Event::Start(e) | Event::Empty(e)) = event else {
            continue;
        };
        if e.name().as_ref() == b"svg" {
            return e
                .try_get_attribute("id")
                .ok()
                .flatten()
                .and_then(|a| a.unescape_value().ok())
                .map(|v| v.into_owned())
                .unwrap_or_default();
        }
    }
    String::new()
}

struct ReaderIter<'a> {
    reader: Reader<&'a [u8]>,
    done: bool,
}

impl<'a> ReaderIter<'a> {
    fn new(reader: Reader<&'a [u8]>) -> Self {
        Self {
            reader,
            done: false,
        }
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
