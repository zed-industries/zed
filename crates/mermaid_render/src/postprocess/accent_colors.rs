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

fn chart_color_class_name(index: usize) -> String {
    format!("zed-chart-{index}")
}

/// Wraps `add_class` and preserves the `Start`/`Empty` variant of the original event.
pub(crate) fn add_to_event<'a>(ev: &Event<'_>, e: &BytesStart<'_>, cl: &str) -> Result<Event<'a>> {
    let new_elem = add_class(e, cl)?;
    Ok(match ev {
        Event::Start(_) => Event::Start(new_elem),
        _ => Event::Empty(new_elem),
    })
}

/// Adds a CSS class to an element, preserving any existing classes.
pub(crate) fn add_class<'a>(e: &BytesStart<'_>, class_to_add: &str) -> Result<BytesStart<'a>> {
    let name = e.name();
    let tag = std::str::from_utf8(name.as_ref())?;
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

pub(crate) fn current_stack_accent(stack: &[Option<usize>]) -> Option<usize> {
    stack.iter().rev().find_map(|entry| *entry)
}

pub(crate) fn lookup_position_accent(node_rects: &[NodeRect], e: &BytesStart<'_>) -> Option<usize> {
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
    node_rects.iter().find_map(|rect| {
        let in_y = (y - rect.cy).abs() <= rect.half_height + 5.0;
        let in_x = (x - rect.cx).abs() <= rect.half_height * 2.0;
        (in_x && in_y).then_some(rect.accent_idx)
    })
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
    in_legend: bool,
    legend_color_idx: usize,
    in_plot: bool,
    plot_depth: usize,
    plot_path_done: bool,
    pie_color_idx: usize,
    quadrant_point_idx: usize,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> AccentColors<I> {
    fn process_chart_colors(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"g" => {
                if self.in_plot {
                    self.plot_depth += 1;
                }
                if let Some(class_attr) = e.try_get_attribute("class")? {
                    let class = class_attr.unescape_value()?;
                    if class.as_ref() == "plot" {
                        self.in_plot = true;
                        self.plot_depth = 1;
                        self.plot_path_done = false;
                    } else if class.as_ref() == "legend" {
                        self.in_legend = true;
                    } else if class.as_ref() == "data-point" {
                        let accent_count = self.theme.accent_colors.len();
                        if accent_count > 0 {
                            let idx = self.quadrant_point_idx % accent_count;
                            self.quadrant_point_idx += 1;
                            return add_to_event(&event, e, &accent_class_name(idx));
                        }
                    }
                }
                Ok(event)
            }

            Event::End(e) if e.name().as_ref() == b"g" => {
                if self.in_plot {
                    self.plot_depth -= 1;
                    if self.plot_depth == 0 {
                        self.in_plot = false;
                    }
                }
                Ok(event)
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"rect" => {
                if self.in_legend && self.legend_color_idx < 8 {
                    let class = chart_color_class_name(self.legend_color_idx);
                    self.legend_color_idx += 1;
                    self.in_legend = false;
                    add_to_event(&event, e, &class)
                } else if self.in_plot {
                    add_to_event(&event, e, &chart_color_class_name(0))
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"path" => {
                let class_val = e
                    .try_get_attribute("class")?
                    .map(|a| a.unescape_value())
                    .transpose()?;

                if class_val.as_deref() == Some("pieCircle") {
                    let class = chart_color_class_name(self.pie_color_idx % 8);
                    self.pie_color_idx += 1;
                    add_to_event(&event, e, &class)
                } else if self.in_plot
                    && !self.plot_path_done
                    && e.try_get_attribute("stroke")?.is_some()
                {
                    self.plot_path_done = true;
                    add_to_event(&event, e, &chart_color_class_name(1))
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

        let event = match &mut self.handler {
            Handler::Flowchart(h) | Handler::ClassDiagram(h) | Handler::StateDiagram(h) => {
                h.process_event(event)
            }
            Handler::Mindmap(h) => h.process_event(event),
            Handler::Sequence(h) => h.process_event(event),
            Handler::Passthrough | Handler::Pending => Ok(event),
        };

        Some(match event {
            Ok(event) => self.process_chart_colors(event),
            err => err,
        })
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
        in_legend: false,
        legend_color_idx: 0,
        in_plot: false,
        plot_depth: 0,
        plot_path_done: false,
        pie_color_idx: 0,
        quadrant_point_idx: 0,
    }
}
