mod class_diagram;
mod mindmap;
mod sequence_diagram;

use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use crate::MermaidTheme;

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

/// Returns the CSS class name for a given accent index (e.g., `"zed-accent-0"`).
pub(crate) fn accent_class_name(index: usize) -> String {
    format!("zed-accent-{index}")
}

/// Adds a CSS class to an element, preserving any existing classes.
pub(crate) fn add_class<'a>(e: &BytesStart<'_>, class_to_add: &str) -> Result<BytesStart<'a>> {
    let name = e.name();
    let tag = std::str::from_utf8(name.as_ref()).unwrap_or("g");
    let mut new_elem = BytesStart::new(tag.to_owned());
    let mut class_found = false;
    for attr in e.attributes() {
        let attr = attr?;
        if attr.key.local_name().as_ref() == b"class" {
            let existing = attr.unescape_value()?;
            let new_class = format!("{existing} {class_to_add}");
            new_elem.push_attribute(("class", new_class.as_str()));
            class_found = true;
        } else {
            new_elem.push_attribute(attr);
        }
    }
    if !class_found {
        new_elem.push_attribute(("class", class_to_add));
    }
    Ok(new_elem)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramType {
    Flowchart,
    Mindmap,
    ClassDiagram,
    StateDiagram,
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
        None => return DiagramType::SequenceDiagram,
    };

    for token in class.split_whitespace() {
        match token {
            "flowchart" => return DiagramType::Flowchart,
            "mindmap" => return DiagramType::Mindmap,
            "classDiagram" => return DiagramType::ClassDiagram,
            "statediagram" => return DiagramType::StateDiagram,
            "journey" => return DiagramType::Unhandled,
            _ => {}
        }
    }

    DiagramType::SequenceDiagram
}

enum Handler {
    Pending,
    Flowchart(class_diagram::ClassDiagramAccents),
    Mindmap(mindmap::MindmapAccents),
    ClassDiagram(class_diagram::ClassDiagramAccents),
    StateDiagram(class_diagram::ClassDiagramAccents),
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

        if matches!(self.handler, Handler::Pending) {
            if let Event::Start(e) | Event::Empty(e) = &event {
                if e.name().as_ref() == b"svg" {
                    let diagram_type = detect_diagram_type(e);
                    let count = self.theme.accent_colors.len();
                    self.handler = match diagram_type {
                        DiagramType::Flowchart => {
                            Handler::Flowchart(class_diagram::ClassDiagramAccents::new(count))
                        }
                        DiagramType::Mindmap => {
                            Handler::Mindmap(mindmap::MindmapAccents::new(&self.theme))
                        }
                        DiagramType::ClassDiagram => {
                            Handler::ClassDiagram(class_diagram::ClassDiagramAccents::new(count))
                        }
                        DiagramType::StateDiagram => {
                            Handler::StateDiagram(class_diagram::ClassDiagramAccents::new(count))
                        }
                        DiagramType::SequenceDiagram => {
                            Handler::Sequence(sequence_diagram::SequenceDiagramAccents::new(count))
                        }
                        DiagramType::Unhandled => Handler::Passthrough,
                    };
                }
            }
        }

        match &mut self.handler {
            Handler::Flowchart(h) => Some(h.process_event(event)),
            Handler::Mindmap(h) => Some(h.process_event(event)),
            Handler::ClassDiagram(h) => Some(h.process_event(event)),
            Handler::StateDiagram(h) => Some(h.process_event(event)),
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
