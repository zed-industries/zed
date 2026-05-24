mod class_diagram;
mod mindmap;
mod sequence_diagram;

use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use crate::MermaidTheme;

pub(crate) struct AccentStyle {
    pub fill: String,
    pub stroke: String,
    pub text: String,
}

pub(crate) fn compute_accent_styles(theme: &MermaidTheme) -> Vec<AccentStyle> {
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

pub(crate) struct NodeRect {
    pub cx: f64,
    pub cy: f64,
    pub half_height: f64,
    pub accent_idx: usize,
}

pub(crate) struct NodeRectBuilder {
    pub cx: f64,
    pub cy: f64,
    pub half_height: f64,
    pub accent_idx: usize,
}

pub(crate) fn parse_translate(e: &BytesStart<'_>) -> Option<(f64, f64)> {
    let attr = e.try_get_attribute("transform").ok()??;
    let val = attr.unescape_value().ok()?;
    let inner = val.strip_prefix("translate(")?.strip_suffix(')')?;
    let (x_str, y_str) = inner.split_once(',')?;
    Some((x_str.trim().parse().ok()?, y_str.trim().parse().ok()?))
}

pub(crate) fn parse_path_half_height(e: &BytesStart<'_>) -> Option<f64> {
    let attr = e.try_get_attribute("d").ok()??;
    let d = attr.unescape_value().ok()?;
    let rest = d.strip_prefix('M')?.trim_start();
    let mut chars = rest.chars().peekable();
    // Skip x value
    while chars.peek().is_some_and(|c| *c != ' ' && *c != ',') {
        chars.next();
    }
    while chars.peek().is_some_and(|c| *c == ' ' || *c == ',') {
        chars.next();
    }
    let y_str: String = chars.take_while(|c| *c != ' ' && *c != ',').collect();
    let y: f64 = y_str.parse().ok()?;
    Some(y.abs())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramType {
    Mindmap,
    ClassDiagram,
    SequenceDiagram,
    Unhandled,
}

fn detect_diagram_type(e: &BytesStart<'_>) -> DiagramType {
    let class = match e
        .try_get_attribute("class")
        .ok()
        .flatten()
        .and_then(|a| a.unescape_value().ok())
    {
        Some(c) => c,
        // No class attribute — could be a sequence or ER diagram.
        // Default to sequence handler which only activates when actor
        // elements are found, so it's safe for ER diagrams too.
        None => return DiagramType::SequenceDiagram,
    };

    for token in class.split_whitespace() {
        match token {
            "mindmap" => return DiagramType::Mindmap,
            "classDiagram" => return DiagramType::ClassDiagram,
            // Sequence diagrams don't have a distinguishing class; detection
            // is handled by looking for actor elements in the sequence handler.
            // Flowcharts, state diagrams, journeys, etc. don't need accent
            // processing (handled by CSS or by other passes).
            "flowchart" | "statediagram" | "journey" => return DiagramType::Unhandled,
            _ => {}
        }
    }

    // Empty or unrecognized class — could be a sequence or ER diagram.
    // Default to sequence handler which activates only when actor elements
    // are found.
    DiagramType::SequenceDiagram
}

enum Handler {
    Pending,
    Mindmap(mindmap::MindmapAccents),
    ClassDiagram(class_diagram::ClassDiagramAccents),
    Sequence(sequence_diagram::SequenceDiagramAccents),
    Passthrough,
}

struct AccentColors<I> {
    inner: I,
    theme: MermaidTheme,
    handler: Handler,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for AccentColors<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = match self.inner.next()? {
            Ok(ev) => ev,
            Err(e) => return Some(Err(e)),
        };

        // Detect diagram type from the root <svg> element.
        if matches!(self.handler, Handler::Pending) {
            if let Event::Start(e) | Event::Empty(e) = &event {
                if e.name().as_ref() == b"svg" {
                    let diagram_type = detect_diagram_type(e);
                    self.handler = match diagram_type {
                        DiagramType::Mindmap => {
                            Handler::Mindmap(mindmap::MindmapAccents::new(&self.theme))
                        }
                        DiagramType::ClassDiagram => {
                            Handler::ClassDiagram(class_diagram::ClassDiagramAccents::new(
                                &self.theme,
                            ))
                        }
                        DiagramType::SequenceDiagram => {
                            Handler::Sequence(sequence_diagram::SequenceDiagramAccents::new(
                                &self.theme,
                            ))
                        }
                        DiagramType::Unhandled => Handler::Passthrough,
                    };
                }
            }
        }

        match &mut self.handler {
            Handler::Mindmap(h) => Some(h.process_event(event)),
            Handler::ClassDiagram(h) => Some(h.process_event(event)),
            Handler::Sequence(h) => Some(h.process_event(event)),
            Handler::Passthrough | Handler::Pending => Some(Ok(event)),
        }
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
    theme: &MermaidTheme,
) -> impl Iterator<Item = Result<Event<'a>>> {
    AccentColors {
        inner: events,
        theme: theme.clone(),
        handler: Handler::Pending,
    }
}
