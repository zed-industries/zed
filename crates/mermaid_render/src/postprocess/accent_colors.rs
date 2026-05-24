use std::collections::VecDeque;

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
            let text = crate::css_color(crate::postprocess::util::text_color_for_background(bg));
            AccentStyle { fill, stroke, text }
        })
        .collect()
}

struct AccentColors<I> {
    inner: I,
    accent_styles: Vec<AccentStyle>,
    accent_g_stack: Vec<Option<usize>>,
    node_counter: usize,
    actor_bottom_counter: usize,
    actor_top_counter: usize,
    last_actor_accent: Option<usize>,
    /// Queue of accent indices for fallback text groups.
    /// Populated as we encounter `<g class="label">` inside node groups;
    /// consumed as we encounter `<g data-merman-foreignobject="fallback">` groups.
    fallback_accent_queue: VecDeque<usize>,
    current_fallback_accent: Option<usize>,
    fallback_depth: usize,
    /// Accent index of the `<text>` element we're currently inside, so that
    /// child `<tspan>` elements can receive the same fill override.
    current_text_accent: Option<usize>,
}

const SHAPE_TAGS: &[&[u8]] = &[b"rect", b"path", b"circle", b"polygon", b"ellipse"];

impl<I> AccentColors<I> {
    fn current_accent(&self) -> Option<usize> {
        self.accent_g_stack.iter().rev().find_map(|entry| *entry)
    }

    fn check_actor_rect(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
        if self.accent_styles.is_empty() || e.name().as_ref() != b"rect" {
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
        if self.accent_styles.is_empty() {
            return Ok(None);
        }
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
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> AccentColors<I> {
    fn process_event(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_styles.is_empty() {
            return Ok(event);
        }

        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"g" => {
                if let Some(fo_attr) = e.try_get_attribute("data-merman-foreignobject")? {
                    if fo_attr.value.as_ref() == b"fallback" {
                        self.fallback_depth = 1;
                        self.current_fallback_accent = self.fallback_accent_queue.pop_front();
                        self.accent_g_stack.push(None);
                        return Ok(event);
                    }
                }
                if let Some(class_attr) = e.try_get_attribute("class")? {
                    let class = class_attr.unescape_value()?;
                    let classes: Vec<&str> = class.split_whitespace().collect();
                    if classes.contains(&"node") || classes.contains(&"stateGroup") {
                        let idx = self.node_counter % self.accent_styles.len();
                        self.node_counter += 1;
                        self.accent_g_stack.push(Some(idx));
                    } else if classes.contains(&"label") {
                        if let Some(accent_idx) = self.current_accent() {
                            self.fallback_accent_queue.push_back(accent_idx);
                        }
                        self.accent_g_stack.push(None);
                    } else {
                        self.accent_g_stack.push(None);
                    }
                } else {
                    self.accent_g_stack.push(None);
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if self.fallback_depth > 0 {
                    self.fallback_depth -= 1;
                    if self.fallback_depth == 0 {
                        self.current_fallback_accent = None;
                    }
                }
                self.accent_g_stack.pop();
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e)
                if SHAPE_TAGS.contains(&e.name().as_ref()) =>
            {
                let actor_accent = self.check_actor_rect(e)?;
                let accent_idx = actor_accent.or_else(|| self.current_accent());

                if let Some(accent_idx) = accent_idx {
                    let accent = &self.accent_styles[accent_idx];
                    let is_start = matches!(&event, Event::Start(_));
                    let name = e.name();
                    let tag_name = std::str::from_utf8(name.as_ref()).unwrap_or("rect");
                    let mut ne = BytesStart::new(tag_name.to_string());
                    let existing_style = e
                        .try_get_attribute("style")?
                        .map(|a| a.unescape_value().map(|v| v.to_string()))
                        .transpose()?
                        .unwrap_or_default();
                    let mut merged = existing_style;
                    if !merged.is_empty() && !merged.ends_with(';') {
                        merged.push(';');
                    }
                    merged.push_str(&format!(
                        "fill: {} !important; stroke: {} !important;",
                        accent.fill, accent.stroke,
                    ));
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key.local_name().as_ref() {
                            b"fill" | b"stroke" | b"style" => {}
                            _ => ne.push_attribute(attr),
                        }
                    }
                    ne.push_attribute(("fill", accent.fill.as_str()));
                    ne.push_attribute(("stroke", accent.stroke.as_str()));
                    ne.push_attribute(("style", merged.as_str()));
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
                if e.name().as_ref() == b"text" || e.name().as_ref() == b"tspan" =>
            {
                let is_tspan = e.name().as_ref() == b"tspan";
                let accent_idx = if is_tspan {
                    self.current_text_accent
                } else {
                    let actor_accent = self.check_actor_text(e)?;
                    let fallback_accent = self.current_fallback_accent;
                    actor_accent
                        .or(fallback_accent)
                        .or_else(|| self.current_accent())
                };

                if let Some(accent_idx) = accent_idx {
                    if !is_tspan && matches!(&event, Event::Start(_)) {
                        self.current_text_accent = Some(accent_idx);
                    }
                    let accent = &self.accent_styles[accent_idx];
                    let is_start = matches!(&event, Event::Start(_));
                    let tag_name = if is_tspan { "tspan" } else { "text" };
                    let mut ne = BytesStart::new(tag_name);
                    let existing_style = e
                        .try_get_attribute("style")?
                        .map(|a| a.unescape_value().map(|v| v.to_string()))
                        .transpose()?
                        .unwrap_or_default();
                    let mut merged = existing_style;
                    if !merged.is_empty() && !merged.ends_with(';') {
                        merged.push(';');
                    }
                    merged.push_str(&format!("fill: {} !important;", accent.text));
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key.local_name().as_ref() {
                            b"fill" | b"style" => {}
                            _ => ne.push_attribute(attr),
                        }
                    }
                    ne.push_attribute(("fill", accent.text.as_str()));
                    ne.push_attribute(("style", merged.as_str()));
                    Ok(if is_start {
                        Event::Start(ne)
                    } else {
                        Event::Empty(ne)
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
        actor_bottom_counter: 0,
        actor_top_counter: 0,
        last_actor_accent: None,
        fallback_accent_queue: VecDeque::new(),
        current_fallback_accent: None,
        fallback_depth: 0,
        current_text_accent: None,
    }
}
