use anyhow::Result;
use quick_xml::events::Event;

use super::{NodeTracker, accent_class_name, add_class, add_to_event, parse_translate};

pub(crate) struct ClassDiagramAccents {
    accent_count: usize,
    accent_g_stack: Vec<Option<usize>>,
    node_counter: usize,
    nodes: NodeTracker,
    current_text_accent: Option<usize>,
}

impl ClassDiagramAccents {
    pub(super) fn new(accent_count: usize) -> Self {
        Self {
            accent_count,
            accent_g_stack: Vec::new(),
            node_counter: 0,
            nodes: NodeTracker::default(),
            current_text_accent: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_count == 0 {
            return Ok(event);
        }

        match &event {
            Event::Start(e) if e.name().as_ref() == b"g" => {
                let is_node = if let Some(class_attr) = e.try_get_attribute("class")? {
                    let class = class_attr.unescape_value()?;
                    class
                        .split_whitespace()
                        .any(|token| token == "node" || token == "stateGroup")
                } else {
                    false
                };

                if is_node {
                    let accent_idx = self.node_counter % self.accent_count;
                    self.node_counter += 1;

                    if let Some((cx, cy)) = parse_translate(e) {
                        self.nodes.start_node(cx, cy, 30.0, accent_idx);
                    }

                    self.accent_g_stack.push(Some(accent_idx));
                    let new_elem = add_class(e, &accent_class_name(accent_idx))?;
                    return Ok(Event::Start(new_elem));
                }

                self.accent_g_stack.push(None);
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if let Some(entry) = self.accent_g_stack.pop() {
                    if entry.is_some() {
                        self.nodes.finish_node();
                    }
                }
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if matches!(
                    e.name().as_ref(),
                    b"rect" | b"path" | b"circle" | b"polygon" | b"ellipse"
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
                let is_start = matches!(event, Event::Start(_));
                let is_text = e.name().as_ref() == b"text";

                let accent_idx = if is_text {
                    self.nodes
                        .lookup_accent(e)
                        .or_else(|| super::current_stack_accent(&self.accent_g_stack))
                } else {
                    self.current_text_accent
                };

                if let Some(idx) = accent_idx {
                    if is_text && is_start {
                        self.current_text_accent = Some(idx);
                    }
                    return add_to_event(&event, e, &accent_class_name(idx));
                }

                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"text" => {
                self.current_text_accent = None;
                Ok(event)
            }

            _ => Ok(event),
        }
    }
}
