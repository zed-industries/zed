use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{AccentStackEntry, NodeTracker};

pub(super) struct MindmapAccents {
    section_classes: Vec<String>,
    section_g_stack: Vec<AccentStackEntry>,
    nodes: NodeTracker,
    current_text_section: Option<usize>,
}

impl MindmapAccents {
    pub(super) fn new() -> Self {
        Self {
            section_classes: Vec::new(),
            section_g_stack: Vec::new(),
            nodes: NodeTracker::default(),
            current_text_section: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        match &event {
            Event::Start(e) if e.name().as_ref() == b"g" => {
                if super::is_foreign_object_fallback_group(e)? {
                    self.section_g_stack.push(AccentStackEntry::none());
                    return Ok(event);
                }

                let section_idx = self.parse_section_class(e)?;
                if let Some(idx) = section_idx {
                    let tracks_node = if let Some((tx, ty)) = super::parse_translate(e) {
                        self.nodes.start_node(tx, ty, 0.0, idx);
                        true
                    } else {
                        false
                    };
                    self.section_g_stack
                        .push(AccentStackEntry::accent(idx, tracks_node));
                } else {
                    self.section_g_stack.push(AccentStackEntry::none());
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if let Some(entry) = self.section_g_stack.pop() {
                    if entry.tracks_node() {
                        self.nodes.maybe_finish_node();
                    }
                }
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if matches!(
                    e.name().as_ref(),
                    b"path" | b"rect" | b"circle" | b"polygon" | b"ellipse"
                ) =>
            {
                if e.name().as_ref() == b"path" {
                    self.nodes.update_half_height(e);
                }
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if e.name().as_ref() == b"text" || e.name().as_ref() == b"tspan" =>
            {
                let section_idx = self.current_section_accent().or_else(|| {
                    if e.name().as_ref() == b"text" {
                        self.nodes.lookup_accent(e)
                    } else {
                        None
                    }
                });

                if e.name().as_ref() == b"text" {
                    self.current_text_section = section_idx;
                }

                let idx = section_idx.or(self.current_text_section);

                if let Some(idx) = idx {
                    if let Some(class_name) = self.section_class_name(idx) {
                        return super::add_to_event(&event, e, class_name);
                    }
                }

                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"text" => {
                self.current_text_section = None;
                Ok(event)
            }

            _ => Ok(event),
        }
    }

    fn parse_section_class(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
        let class_attr = match e.try_get_attribute("class")? {
            Some(attr) => attr,
            None => return Ok(None),
        };
        let class = class_attr.unescape_value()?;
        let is_root = class.split_whitespace().any(|t| t == "section-root");

        for token in class.split_whitespace() {
            if let Some(rest) = token.strip_prefix("section-") {
                if rest == "-1" || rest.parse::<u32>().is_ok() {
                    let class_name = if is_root {
                        "section-root section--1".to_string()
                    } else {
                        format!("section-{rest}")
                    };
                    let idx = self.section_classes.len();
                    self.section_classes.push(class_name);
                    return Ok(Some(idx));
                }
            }
        }
        Ok(None)
    }

    fn current_section_accent(&self) -> Option<usize> {
        super::current_stack_accent(&self.section_g_stack)
    }

    fn section_class_name(&self, idx: usize) -> Option<&str> {
        self.section_classes.get(idx).map(|s| s.as_str())
    }
}
