use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use crate::MermaidTheme;

struct DiagramRecolor<I> {
    inner: I,
    git_colors: [String; 8],
    in_legend: bool,
    legend_color_idx: usize,
    in_plot: bool,
    plot_depth: usize,
    plot_path_done: bool,
    pie_color_idx: usize,
}

fn wrap_event<'a>(original_was_start: bool, elem: BytesStart<'a>) -> Event<'a> {
    if original_was_start {
        Event::Start(elem)
    } else {
        Event::Empty(elem)
    }
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> DiagramRecolor<I> {
    fn process_event(&mut self, event: Event<'a>) -> Result<Event<'a>> {
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
                let is_start = matches!(event, Event::Start(_));

                if self.in_legend && self.legend_color_idx < 8 {
                    let color = &self.git_colors[self.legend_color_idx];
                    self.legend_color_idx += 1;
                    self.in_legend = false;
                    let mut new_elem = BytesStart::new("rect");
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.local_name().as_ref() == b"style" {
                            new_elem.push_attribute((
                                "style",
                                format!("fill: {color}; stroke: {color};").as_str(),
                            ));
                        } else {
                            new_elem.push_attribute(attr);
                        }
                    }
                    Ok(wrap_event(is_start, new_elem))
                } else if self.in_plot {
                    let bar_color = &self.git_colors[0];
                    let mut new_elem = BytesStart::new("rect");
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key.local_name().as_ref() {
                            b"fill" => new_elem.push_attribute(("fill", bar_color.as_str())),
                            b"stroke" => new_elem.push_attribute(("stroke", bar_color.as_str())),
                            _ => new_elem.push_attribute(attr),
                        }
                    }
                    Ok(wrap_event(is_start, new_elem))
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"path" => {
                let is_start = matches!(event, Event::Start(_));

                let class_val = e
                    .try_get_attribute("class")?
                    .map(|a| a.unescape_value())
                    .transpose()?;

                if class_val.as_deref() == Some("pieCircle") {
                    let color = &self.git_colors[self.pie_color_idx % 8];
                    self.pie_color_idx += 1;
                    let mut new_elem = BytesStart::new("path");
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.local_name().as_ref() == b"fill" {
                            new_elem.push_attribute(("fill", color.as_str()));
                        } else {
                            new_elem.push_attribute(attr);
                        }
                    }
                    Ok(wrap_event(is_start, new_elem))
                } else if self.in_plot
                    && !self.plot_path_done
                    && e.try_get_attribute("stroke")?.is_some()
                {
                    self.plot_path_done = true;
                    let line_color = &self.git_colors[1];
                    let mut new_elem = BytesStart::new("path");
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.local_name().as_ref() == b"stroke" {
                            new_elem.push_attribute(("stroke", line_color.as_str()));
                        } else {
                            new_elem.push_attribute(attr);
                        }
                    }
                    Ok(wrap_event(is_start, new_elem))
                } else {
                    Ok(event)
                }
            }

            _ => Ok(event),
        }
    }
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for DiagramRecolor<I> {
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
    let git_colors: [String; 8] = theme.git_branch_colors.map(crate::css_color);
    DiagramRecolor {
        inner: events,
        git_colors,
        in_legend: false,
        legend_color_idx: 0,
        in_plot: false,
        plot_depth: 0,
        plot_path_done: false,
        pie_color_idx: 0,
    }
}
