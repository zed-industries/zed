use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{
    compute_accent_styles, parse_path_half_height, parse_translate, AccentStyle, NodeRect,
    NodeRectBuilder,
};
use crate::MermaidTheme;

const SHAPE_TAGS: &[&[u8]] = &[b"rect", b"path", b"circle", b"polygon", b"ellipse"];

pub(crate) struct ClassDiagramAccents {
    accent_styles: Vec<AccentStyle>,
    accent_g_stack: Vec<Option<usize>>,
    node_counter: usize,
    node_rects: Vec<NodeRect>,
    building_node: Option<NodeRectBuilder>,
    current_text_accent: Option<usize>,
}

impl ClassDiagramAccents {
    pub(super) fn new(theme: &MermaidTheme) -> Self {
        Self {
            accent_styles: compute_accent_styles(theme),
            accent_g_stack: Vec::new(),
            node_counter: 0,
            node_rects: Vec::new(),
            building_node: None,
            current_text_accent: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_styles.is_empty() {
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
                    let accent_idx = self.node_counter % self.accent_styles.len();
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
                } else {
                    self.accent_g_stack.push(None);
                }

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
                let is_start = matches!(event, Event::Start(_));

                if e.name().as_ref() == b"path" {
                    if let Some(ref mut builder) = self.building_node {
                        if let Some(hh) = parse_path_half_height(e) {
                            if hh > builder.half_height {
                                builder.half_height = hh;
                            }
                        }
                    }
                }

                if let Some(accent_idx) = self.current_accent() {
                    let style = &self.accent_styles[accent_idx];
                    let rewritten = rewrite_shape(e, &style.fill, &style.stroke)?;
                    Ok(if is_start {
                        Event::Start(rewritten)
                    } else {
                        Event::Empty(rewritten)
                    })
                } else {
                    Ok(event)
                }
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
                    let style = &self.accent_styles[idx];
                    let name = e.name();
                    let tag_name = std::str::from_utf8(name.as_ref()).unwrap_or("text");
                    let rewritten = rewrite_text(e, tag_name, &style.text)?;
                    Ok(if is_start {
                        Event::Start(rewritten)
                    } else {
                        Event::Empty(rewritten)
                    })
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

fn rewrite_shape<'a>(
    e: &BytesStart<'_>,
    fill: &str,
    stroke: &str,
) -> Result<BytesStart<'a>> {
    let name = e.name();
    let tag_name = std::str::from_utf8(name.as_ref()).unwrap_or("rect");
    let mut new_elem = BytesStart::new(tag_name.to_owned());
    let mut existing_style = String::new();

    for attr in e.attributes() {
        let attr = attr?;
        match attr.key.local_name().as_ref() {
            b"fill" | b"stroke" => {}
            b"style" => {
                existing_style = attr.unescape_value()?.into_owned();
            }
            _ => new_elem.push_attribute(attr),
        }
    }

    new_elem.push_attribute(("fill", fill));
    new_elem.push_attribute(("stroke", stroke));

    let mut merged_style = format!("fill: {fill} !important; stroke: {stroke} !important;");
    if !existing_style.is_empty() {
        merged_style.push(' ');
        merged_style.push_str(&existing_style);
    }
    new_elem.push_attribute(("style", merged_style.as_str()));

    Ok(new_elem)
}

fn rewrite_text<'a>(
    e: &BytesStart<'_>,
    tag_name: &str,
    text_color: &str,
) -> Result<BytesStart<'a>> {
    let mut new_elem = BytesStart::new(tag_name.to_owned());
    let mut existing_style = String::new();

    for attr in e.attributes() {
        let attr = attr?;
        match attr.key.local_name().as_ref() {
            b"fill" => {}
            b"style" => {
                existing_style = attr.unescape_value()?.into_owned();
            }
            _ => new_elem.push_attribute(attr),
        }
    }

    new_elem.push_attribute(("fill", text_color));

    let mut merged_style = format!("fill: {text_color} !important;");
    if !existing_style.is_empty() {
        merged_style.push(' ');
        merged_style.push_str(&existing_style);
    }
    new_elem.push_attribute(("style", merged_style.as_str()));

    Ok(new_elem)
}
