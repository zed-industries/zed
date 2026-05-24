use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{NodeRect, NodeRectBuilder};
use crate::MermaidTheme;

struct SectionText {
    section_index: i32,
    is_root: bool,
    text_color: String,
}

pub(super) struct MindmapAccents {
    section_texts: Vec<SectionText>,
    section_g_stack: Vec<Option<usize>>,
    node_rects: Vec<NodeRect>,
    building_node: Option<NodeRectBuilder>,
    current_text_accent: Option<usize>,
}

impl MindmapAccents {
    pub(super) fn new(theme: &MermaidTheme) -> Self {
        let mut section_texts = Vec::new();

        let text_color = |color_idx: usize| -> String {
            let bg = theme.git_branch_colors[color_idx];
            crate::css_color(crate::postprocess::util::text_color_for_background(bg))
        };

        section_texts.push(SectionText {
            section_index: -1,
            is_root: true,
            text_color: text_color(0),
        });
        section_texts.push(SectionText {
            section_index: -1,
            is_root: false,
            text_color: text_color(1),
        });
        for i in 0..=10 {
            section_texts.push(SectionText {
                section_index: i,
                is_root: false,
                text_color: text_color(2 + (i as usize % 6)),
            });
        }

        Self {
            section_texts,
            section_g_stack: Vec::new(),
            node_rects: Vec::new(),
            building_node: None,
            current_text_accent: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        match &event {
            Event::Start(e) if e.name().as_ref() == b"g" => {
                let section_info = self.parse_section_class(e)?;
                if let Some((accent_idx, _is_root)) = section_info {
                    if let Some((_tx, ty)) = super::parse_translate(e) {
                        self.building_node = Some(NodeRectBuilder {
                            cx: _tx,
                            cy: ty,
                            half_height: 0.0,
                            accent_idx,
                        });
                    }
                    self.section_g_stack.push(Some(accent_idx));
                } else {
                    self.section_g_stack.push(None);
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if let Some(maybe_accent) = self.section_g_stack.pop() {
                    if maybe_accent.is_some() {
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
                let accent_idx = self.current_section_accent().or_else(|| {
                    if e.name().as_ref() == b"text" {
                        self.lookup_position_accent(e)
                    } else {
                        None
                    }
                });

                if e.name().as_ref() == b"text" {
                    self.current_text_accent = accent_idx;
                }

                let idx = accent_idx.or(self.current_text_accent);

                if let Some(idx) = idx {
                    if let Some(color) = self.accent_text_color(idx) {
                        return rewrite_text_fill(e, &event, color);
                    }
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

    fn parse_section_class(&self, e: &BytesStart<'_>) -> Result<Option<(usize, bool)>> {
        let class_attr = match e.try_get_attribute("class")? {
            Some(attr) => attr,
            None => return Ok(None),
        };
        let class = class_attr.unescape_value()?;
        let tokens: Vec<&str> = class.split_whitespace().collect();

        let is_root = tokens.contains(&"section-root");

        for token in &tokens {
            if let Some(rest) = token.strip_prefix("section-") {
                if rest == "-1" {
                    let idx = if is_root { 0 } else { 1 };
                    return Ok(Some((idx, is_root)));
                }
                if let Ok(i) = rest.parse::<u32>() {
                    if i <= 10 {
                        let idx = 2 + (i as usize % 6);
                        return Ok(Some((idx, is_root)));
                    }
                }
            }
        }

        Ok(None)
    }

    fn current_section_accent(&self) -> Option<usize> {
        self.section_g_stack
            .iter()
            .rev()
            .find_map(|entry| *entry)
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

    fn accent_text_color(&self, accent_idx: usize) -> Option<&str> {
        self.section_texts
            .iter()
            .find(|s| {
                if s.section_index == -1 {
                    if s.is_root {
                        accent_idx == 0
                    } else {
                        accent_idx == 1
                    }
                } else {
                    2 + (s.section_index as usize % 6) == accent_idx
                }
            })
            .map(|s| s.text_color.as_str())
    }
}

fn rewrite_text_fill<'a>(
    e: &BytesStart<'a>,
    event: &Event<'a>,
    fill_color: &str,
) -> Result<Event<'a>> {
    let is_tspan = e.name().as_ref() == b"tspan";
    let is_start = matches!(event, Event::Start(_));
    let tag = if is_tspan { "tspan" } else { "text" };
    let mut new_elem = BytesStart::new(tag);
    for attr in e.attributes() {
        let attr = attr?;
        match attr.key.local_name().as_ref() {
            b"fill" | b"style" => {}
            _ => new_elem.push_attribute(attr),
        }
    }
    new_elem.push_attribute(("fill", fill_color));
    new_elem.push_attribute(("style", &*format!("fill: {fill_color} !important;")));
    Ok(if is_start {
        Event::Start(new_elem)
    } else {
        Event::Empty(new_elem)
    })
}
