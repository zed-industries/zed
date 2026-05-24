use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use crate::MermaidTheme;

struct AccentStyle {
    fill: String,
    stroke: String,
    text: String,
}

fn compute_accent_styles(theme: &MermaidTheme) -> Vec<AccentStyle> {
    theme
        .accent_colors
        .iter()
        .map(|accent| {
            let stroke = crate::css_color(accent.foreground);
            let mut bg = accent.background;
            if theme.dark_mode {
                bg.l = (bg.l * 0.7).max(0.0);
            } else {
                bg.l = (bg.l * 1.3).min(1.0);
            }
            let fill = crate::css_color(bg);
            let text = if theme.dark_mode {
                "#ffffff".to_string()
            } else {
                "#000000".to_string()
            };
            AccentStyle { fill, stroke, text }
        })
        .collect()
}

struct AccentColors<I> {
    inner: I,
    accent_styles: Vec<AccentStyle>,
    accent_g_stack: Vec<Option<usize>>,
    node_counter: usize,
}

const SHAPE_TAGS: &[&[u8]] = &[b"rect", b"path", b"circle", b"polygon", b"ellipse"];

impl<I> AccentColors<I> {
    fn current_accent(&self) -> Option<usize> {
        self.accent_g_stack.iter().rev().find_map(|entry| *entry)
    }
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> AccentColors<I> {
    fn process_event(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_styles.is_empty() {
            return Ok(event);
        }

        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"g" => {
                if let Some(class_attr) = e.try_get_attribute("class")? {
                    let class = class_attr.unescape_value()?;
                    let classes: Vec<&str> = class.split_whitespace().collect();
                    if classes.contains(&"node") || classes.contains(&"stateGroup") {
                        let idx = self.node_counter % self.accent_styles.len();
                        self.node_counter += 1;
                        self.accent_g_stack.push(Some(idx));
                    } else {
                        self.accent_g_stack.push(None);
                    }
                } else {
                    self.accent_g_stack.push(None);
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                self.accent_g_stack.pop();
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if SHAPE_TAGS.contains(&e.name().as_ref()) =>
            {
                if let Some(accent_idx) = self.current_accent() {
                    let style = &self.accent_styles[accent_idx];
                    let is_start = matches!(&event, Event::Start(_));
                    let name = e.name();
                    let tag_name =
                        std::str::from_utf8(name.as_ref()).unwrap_or("rect");
                    let mut ne = BytesStart::new(tag_name.to_string());
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key.local_name().as_ref() {
                            b"fill" => ne.push_attribute(("fill", style.fill.as_str())),
                            b"stroke" => {
                                ne.push_attribute(("stroke", style.stroke.as_str()))
                            }
                            _ => ne.push_attribute(attr),
                        }
                    }
                    Ok(if is_start {
                        Event::Start(ne)
                    } else {
                        Event::Empty(ne)
                    })
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e)
                if e.name().as_ref() == b"text" =>
            {
                if let Some(accent_idx) = self.current_accent() {
                    let style = &self.accent_styles[accent_idx];
                    let is_start = matches!(&event, Event::Start(_));
                    let mut ne = BytesStart::new("text");
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.local_name().as_ref() == b"fill" {
                            ne.push_attribute(("fill", style.text.as_str()));
                        } else {
                            ne.push_attribute(attr);
                        }
                    }
                    Ok(if is_start {
                        Event::Start(ne)
                    } else {
                        Event::Empty(ne)
                    })
                } else {
                    Ok(event)
                }
            }

            _ => Ok(event),
        }
    }
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for AccentColors<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = match self.inner.next()? {
            Ok(ev) => ev,
            Err(e) => return Some(Err(e)),
        };
        Some(self.process_event(event))
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
    theme: &MermaidTheme,
) -> impl Iterator<Item = Result<Event<'a>>> {
    AccentColors {
        inner: events,
        accent_styles: compute_accent_styles(theme),
        accent_g_stack: Vec::new(),
        node_counter: 0,
    }
}
