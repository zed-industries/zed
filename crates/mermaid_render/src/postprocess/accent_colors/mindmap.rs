use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{NodeRect, NodeRectBuilder};
use crate::MermaidTheme;

struct SectionClass {
    class_name: String,
}

pub(super) struct MindmapAccents {
    section_classes: Vec<SectionClass>,
    section_g_stack: Vec<Option<usize>>,
    node_rects: Vec<NodeRect>,
    building_node: Option<NodeRectBuilder>,
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
                        self.building_node = Some(NodeRectBuilder {
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
                        if let Some(builder) = self.building_node.take() {
                            self.node_rects.push(NodeRect {
                                cx: builder.cx,
                                cy: builder.cy,
                                half_height: builder.half_height,
                                accent_idx: builder.accent_idx,
                            });
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
                let is_start = matches!(event, Event::Start(_));

                let section_idx = self.current_section_accent().or_else(|| {
                    if is_text {
                        self.lookup_position_accent(e)
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
                        let modified = super::add_class(e, class_name)?;
                        return Ok(if is_start {
                            Event::Start(modified)
                        } else {
                            Event::Empty(modified)
                        });
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
        self.section_g_stack.iter().rev().find_map(|entry| *entry)
    }

    fn lookup_position_accent(&self, e: &BytesStart<'_>) -> Option<usize> {
        let x: f64 = e
            .try_get_attribute("x")
            .ok()??
            .unescape_value()
            .ok()?
            .parse()
            .ok()?;
        let y: f64 = e
            .try_get_attribute("y")
            .ok()??
            .unescape_value()
            .ok()?
            .parse()
            .ok()?;
        self.node_rects.iter().find_map(|rect| {
            let in_y = (y - rect.cy).abs() <= rect.half_height + 5.0;
            let in_x = (x - rect.cx).abs() <= rect.half_height * 2.0;
            (in_x && in_y).then_some(rect.accent_idx)
        })
    }

    fn section_class_name(&self, idx: usize) -> Option<&str> {
        self.section_classes
            .get(idx)
            .map(|s| s.class_name.as_str())
    }
}
