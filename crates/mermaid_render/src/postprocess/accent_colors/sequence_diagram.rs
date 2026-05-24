use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use super::{accent_class_name, add_class};

pub(super) struct SequenceDiagramAccents {
    accent_count: usize,
    actor_bottom_counter: usize,
    actor_top_counter: usize,
    last_actor_accent: Option<usize>,
    current_text_accent: Option<usize>,
}

impl SequenceDiagramAccents {
    pub(super) fn new(accent_count: usize) -> Self {
        Self {
            accent_count,
            actor_bottom_counter: 0,
            actor_top_counter: 0,
            last_actor_accent: None,
            current_text_accent: None,
        }
    }

    pub(super) fn process_event<'a>(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        if self.accent_count == 0 {
            return Ok(event);
        }

        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"rect" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.check_actor_rect(e)? {
                    let new_elem = add_class(e, &accent_class_name(idx))?;
                    Ok(if is_start {
                        Event::Start(new_elem)
                    } else {
                        Event::Empty(new_elem)
                    })
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"text" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.check_actor_text(e)? {
                    self.current_text_accent = Some(idx);
                    let new_elem = add_class(e, &accent_class_name(idx))?;
                    Ok(if is_start {
                        Event::Start(new_elem)
                    } else {
                        Event::Empty(new_elem)
                    })
                } else {
                    Ok(event)
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"tspan" => {
                let is_start = matches!(event, Event::Start(_));
                if let Some(idx) = self.current_text_accent {
                    let new_elem = add_class(e, &accent_class_name(idx))?;
                    Ok(if is_start {
                        Event::Start(new_elem)
                    } else {
                        Event::Empty(new_elem)
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

    fn check_actor_rect(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
        if e.name().as_ref() != b"rect" {
            return Ok(None);
        }
        let class_attr = match e.try_get_attribute("class")? {
            Some(a) => a,
            None => return Ok(None),
        };
        let class_val = class_attr.unescape_value()?;
        if class_val.contains("actor-bottom") {
            let idx = self.actor_bottom_counter % self.accent_count;
            self.actor_bottom_counter += 1;
            self.last_actor_accent = Some(idx);
            Ok(Some(idx))
        } else if class_val.contains("actor-top") {
            let idx = self.actor_top_counter % self.accent_count;
            self.actor_top_counter += 1;
            self.last_actor_accent = Some(idx);
            Ok(Some(idx))
        } else {
            Ok(None)
        }
    }

    fn check_actor_text(&mut self, e: &BytesStart<'_>) -> Result<Option<usize>> {
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
