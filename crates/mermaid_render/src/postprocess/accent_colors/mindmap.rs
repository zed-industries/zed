use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::NodeRect;
use crate::MermaidTheme;

struct SectionClass {
    class_name: String,
}

pub(super) struct MindmapAccents {
    section_classes: Vec<SectionClass>,
    section_g_stack: Vec<Option<usize>>,
    node_rects: Vec<NodeRect>,
    building_node: Option<NodeRect>,
    current_text_section: Option<usize>,
}

impl MindmapAccents {
    pub(super) fn new(_theme: &MermaidTheme) -> Self {
        Self {
            section_classes: Vec::new(),
            section_g_stack: Vec::new(),
            node_rects: Vec::new(),
            building_node: None,
            current_text_section: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        match &event {
            Event::Start(e) if e.name().as_ref() == b"g" => {
                let section_idx = self.parse_section_class(e)?;
                if let Some(idx) = section_idx {
                    if let Some((tx, ty)) = super::parse_translate(e) {
                        self.building_node = Some(NodeRect {
                            cx: tx,
                            cy: ty,
                            half_height: 0.0,
                            accent_idx: idx,
                        });
                    }
                    self.section_g_stack.push(Some(idx));
                } else {
                    self.section_g_stack.push(None);
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if let Some(maybe_section) = self.section_g_stack.pop() {
                    if maybe_section.is_some() {
                        if let Some(rect) = self.building_node.take() {
                            self.node_rects.push(rect);
                        }
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
                if let Some(ref mut builder) = self.building_node {
                    if e.name().as_ref() == b"path" {
                        if let Some(hh) = super::parse_path_half_height(e) {
                            if hh > builder.half_height {
                                builder.half_height = hh;
                            }
                        }
                    }
                }
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if e.name().as_ref() == b"text" || e.name().as_ref() == b"tspan" =>
            {
                let is_text = e.name().as_ref() == b"text";

                let section_idx = self.current_section_accent().or_else(|| {
                    if is_text {
                        super::lookup_position_accent(&self.node_rects, e)
                    } else {
                        None
                    }
                });

                if is_text {
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
        let tokens: Vec<&str> = class.split_whitespace().collect();
        let is_root = tokens.contains(&"section-root");

        for token in &tokens {
            if let Some(rest) = token.strip_prefix("section-") {
                if rest == "-1" || rest.parse::<u32>().is_ok() {
                    let class_name = if is_root {
                        "section-root section--1".to_string()
                    } else {
                        format!("section-{rest}")
                    };
                    let idx = self.section_classes.len();
                    self.section_classes.push(SectionClass { class_name });
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
        self.section_classes.get(idx).map(|s| s.class_name.as_str())
    }
}
