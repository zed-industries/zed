use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{AccentStyle, compute_accent_styles};
use crate::MermaidTheme;

pub(super) struct SequenceDiagramAccents {
    accent_styles: Vec<AccentStyle>,
    actor_bottom_counter: usize,
    actor_top_counter: usize,
    last_actor_accent: Option<usize>,
    current_text_accent: Option<usize>,
}

impl SequenceDiagramAccents {
    pub(super) fn new(theme: &MermaidTheme) -> Self {
        Self {
            accent_styles: compute_accent_styles(theme),
            actor_bottom_counter: 0,
            actor_top_counter: 0,
            last_actor_accent: None,
            current_text_accent: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_styles.is_empty() {
            return Ok(event);
        }

        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"rect" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.check_actor_rect(e)? {
                    Ok(self.rewrite_rect(e, idx, is_start))
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"text" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.check_actor_text(e)? {
                    self.current_text_accent = Some(idx);
                    Ok(self.rewrite_text_fill(e, idx, is_start, "text"))
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"tspan" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.current_text_accent {
                    Ok(self.rewrite_text_fill(e, idx, is_start, "tspan"))
                } else {
                    Ok(event)
                }
            }

            Event::End(e) if e.name().as_ref() == b"text" => {
                self.current_text_accent = None;
                Ok(event)
            }

            _ => Ok(event),
        }
    }

    fn check_actor_rect(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
        if e.name().as_ref() != b"rect" {
            return Ok(None);
        }
        let class_attr = match e.try_get_attribute("class")? {
            Some(a) => a,
            None => return Ok(None),
        };
        let class_val = class_attr.unescape_value()?;
        if class_val.contains("actor-bottom") {
            let idx = self.actor_bottom_counter % self.accent_styles.len();
            self.actor_bottom_counter += 1;
            self.last_actor_accent = Some(idx);
            Ok(Some(idx))
        } else if class_val.contains("actor-top") {
            let idx = self.actor_top_counter % self.accent_styles.len();
            self.actor_top_counter += 1;
            self.last_actor_accent = Some(idx);
            Ok(Some(idx))
        } else {
            Ok(None)
        }
    }

    fn check_actor_text(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
        let class_attr = match e.try_get_attribute("class")? {
            Some(a) => a,
            None => return Ok(None),
        };
        let class_val = class_attr.unescape_value()?;
        if class_val.contains("actor") && class_val.contains("actor-box") {
            Ok(self.last_actor_accent.take())
        } else {
            Ok(None)
        }
    }

    fn rewrite_rect<'a>(
        &self,
        e: &BytesStart<'_>,
        accent_idx: usize,
        is_start: bool,
    ) -> Event<'a> {
        let style = &self.accent_styles[accent_idx];
        let mut new_elem = BytesStart::new("rect");
        for attr in e.attributes().filter_map(|a| a.ok()) {
            match attr.key.local_name().as_ref() {
                b"fill" | b"stroke" | b"style" => {}
                _ => new_elem.push_attribute(attr),
            }
        }
        new_elem.push_attribute(("fill", style.fill.as_str()));
        new_elem.push_attribute(("stroke", style.stroke.as_str()));
        new_elem.push_attribute((
            "style",
            format!(
                "fill: {} !important; stroke: {} !important;",
                style.fill, style.stroke
            )
            .as_str(),
        ));
        if is_start {
            Event::Start(new_elem)
        } else {
            Event::Empty(new_elem)
        }
    }

    fn rewrite_text_fill<'a>(
        &self,
        e: &BytesStart<'_>,
        accent_idx: usize,
        is_start: bool,
        tag: &str,
    ) -> Event<'a> {
        let style = &self.accent_styles[accent_idx];
        let mut new_elem = BytesStart::new(tag.to_owned());
        for attr in e.attributes().filter_map(|a| a.ok()) {
            match attr.key.local_name().as_ref() {
                b"fill" | b"style" => {}
                _ => new_elem.push_attribute(attr),
            }
        }
        new_elem.push_attribute(("fill", style.text.as_str()));
        new_elem.push_attribute((
            "style",
            format!("fill: {} !important;", style.text).as_str(),
        ));
        if is_start {
            Event::Start(new_elem)
        } else {
            Event::Empty(new_elem)
        }
    }
}
