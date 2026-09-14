//! Zed-specific post-processing of [`merman`]-produced SVGs.
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
mod inject_css;
mod strip_foreignobject;
pub(crate) mod util;

use anyhow::{Context as _, Result};
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};

use crate::MermaidTheme;

pub(super) fn postprocess(svg: &str, theme: &MermaidTheme, custom_palette: bool) -> Result<String> {
    // merman already applies the generic resvg-safe cleanup before this point.
    // The remaining passes are Zed-specific theme and accent adjustments.
    let svg_id = extract_svg_id(svg);

    let mut reader = Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    let events = ReaderIter::new(reader);
    // merman's resvg-safe pipeline already removes foreignObject elements and
    // replaces their labels with native <text> fallback groups. This pass keeps
    // those fallback labels, but drops any that merely duplicate a native
    // <text> (e.g. user journey renders some labels both ways).
    let events = strip_foreignobject::process(events, svg);
    let events = events.map(|event| preserve_fallback_text_color(event?));
    let events = element_fixup::process(events, theme, custom_palette);
    let events: Box<dyn Iterator<Item = Result<Event<'_>>>> = if !custom_palette {
        let events = accent_colors::process(events, theme);
        Box::new(inject_css::process(events, theme, &svg_id))
    } else {
        Box::new(events)
    };

    let mut writer = quick_xml::Writer::new(Vec::with_capacity(svg.len()));
    for event in events {
        writer.write_event(event?)?;
    }
    String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")
}

fn preserve_fallback_text_color(event: Event<'_>) -> Result<Event<'_>> {
    let element = match &event {
        Event::Start(element) | Event::Empty(element) => element,
        _ => return Ok(event),
    };
    let class = match element.name().as_ref() {
        b"g" if accent_colors::is_foreign_object_fallback_group(element)? => {
            "merman-foreignobject-fallback"
        }
        b"text"
            if element
                .try_get_attribute("class")?
                .map(|class| {
                    class
                        .normalized_value(XmlVersion::Implicit1_0)
                        .map(|class| {
                            class
                                .split_whitespace()
                                .any(|class| class == "merman-foreignobject-fallback-text")
                        })
                })
                .transpose()?
                .unwrap_or(false) =>
        {
            "merman-foreignobject-fallback-text"
        }
        _ => return Ok(event),
    };
    // Copied node classes apply shape fill rules to the moved fallback text.
    let mut replacement = element_fixup::rewrite_attr(element, b"class", class)?;
    if element.name().as_ref() == b"text"
        && let Some(fill) = element.try_get_attribute("fill")?
    {
        let previous_style = element.try_get_attribute("style")?;
        let mut style = previous_style
            .as_ref()
            .map(|style| {
                style
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map(|style| style.into_owned())
            })
            .transpose()?
            .unwrap_or_default();
        // Keep the color merman resolved in the original HTML label context.
        style.push_str(&format!(
            ";fill:{} !important;",
            fill.normalized_value(XmlVersion::Implicit1_0)?
        ));
        if previous_style.is_some() {
            replacement = element_fixup::rewrite_attr(&replacement, b"style", &style)?;
        } else {
            replacement.push_attribute(("style", style.as_str()));
        }
    }
    Ok(match event {
        Event::Start(_) => Event::Start(replacement),
        _ => Event::Empty(replacement),
    })
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
                .and_then(|a| a.normalized_value(XmlVersion::Implicit1_0).ok())
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
