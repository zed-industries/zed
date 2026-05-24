use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{
    accent_class_name, add_class, parse_path_half_height, parse_translate, NodeRect,
    NodeRectBuilder,
};

const SHAPE_TAGS: &[&[u8]] = &[b"rect", b"path", b"circle", b"polygon", b"ellipse"];

pub(crate) struct ClassDiagramAccents {
    accent_count: usize,
    accent_g_stack: Vec<Option<usize>>,
    node_counter: usize,
    node_rects: Vec<NodeRect>,
    building_node: Option<NodeRectBuilder>,
    current_text_accent: Option<usize>,
}

impl ClassDiagramAccents {
    pub(super) fn new(accent_count: usize) -> Self {
        Self {
            accent_count,
            accent_g_stack: Vec::new(),
            node_counter: 0,
            node_rects: Vec::new(),
            building_node: None,
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
                        self.building_node = Some(NodeRectBuilder {
                            cx,
                            cy,
                            half_height: 30.0,
                            accent_idx,
                        });
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
                if SHAPE_TAGS.iter().any(|tag| e.name().as_ref() == *tag) =>
            {
                if e.name().as_ref() == b"path" {
                    if let Some(ref mut builder) = self.building_node {
                        if let Some(hh) = parse_path_half_height(e) {
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
                let is_start = matches!(event, Event::Start(_));
                let is_text = e.name().as_ref() == b"text";

                let accent_idx = if is_text {
                    self.lookup_position_accent(e)
                        .or_else(|| self.current_accent())
                } else {
                    self.current_text_accent
                };

                if let Some(idx) = accent_idx {
                    if is_text && is_start {
                        self.current_text_accent = Some(idx);
                    }
                    let new_elem = add_class(e, &accent_class_name(idx))?;
                    return Ok(if is_start {
                        Event::Start(new_elem)
                    } else {
                        Event::Empty(new_elem)
                    });
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

    fn current_accent(&self) -> Option<usize> {
        self.accent_g_stack.iter().rev().find_map(|entry| *entry)
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
}
