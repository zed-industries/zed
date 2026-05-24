use anyhow::{Context as _, Result};
use quick_xml::escape;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};

const PIXELS_PER_CHAR: f64 = 8.0;

pub(super) fn process(svg: &str) -> Result<String> {
    let mut reader = Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    let mut writer = Writer::new(Vec::new());

    let mut foreign_object_depth: usize = 0;
    let mut container_width: f64 = 0.0;
    let mut buffer: Vec<Event<'_>> = Vec::new();
    let mut plain_text: String = String::new();

    loop {
        let event = match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => event,
            Err(e) => return Err(e).context("failed to parse SVG in foreignObject wrap pass"),
        };

        let is_fo_start =
            matches!(&event, Event::Start(e) if e.name().as_ref() == b"foreignObject");
        let is_fo_end =
            matches!(&event, Event::End(e) if e.name().as_ref() == b"foreignObject");

        if is_fo_start {
            if foreign_object_depth == 0 {
                container_width = if let Event::Start(ref e) = event {
                    parse_width_attr(e)?
                } else {
                    0.0
                };
                plain_text.clear();
                buffer.clear();
            }
            buffer.push(event);
            foreign_object_depth += 1;
        } else if is_fo_end {
            foreign_object_depth = foreign_object_depth.saturating_sub(1);
            buffer.push(event);
            if foreign_object_depth == 0 {
                emit_buffered(
                    std::mem::take(&mut buffer),
                    &plain_text,
                    container_width,
                    &mut writer,
                )?;
                plain_text.clear();
            }
        } else if foreign_object_depth > 0 {
            if let Event::Text(ref t) = event {
                if let Ok(decoded) = t.decode() {
                    if let Ok(unescaped) = escape::unescape(&decoded) {
                        plain_text.push_str(&unescaped);
                    }
                }
            }
            buffer.push(event);
        } else {
            writer.write_event(event)?;
        }
    }

    String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")
}

fn parse_width_attr(e: &BytesStart<'_>) -> Result<f64> {
    if let Some(attr) = e.try_get_attribute("width")? {
        let val = attr.unescape_value()?;
        Ok(val.parse().unwrap_or(0.0))
    } else {
        Ok(0.0)
    }
}

fn emit_buffered(
    buffer: Vec<Event<'_>>,
    plain_text: &str,
    container_width: f64,
    writer: &mut Writer<Vec<u8>>,
) -> Result<()> {
    let max_chars = if container_width > 0.0 {
        (container_width / PIXELS_PER_CHAR).floor() as usize
    } else {
        usize::MAX
    };

    let needs_wrap = max_chars < usize::MAX && plain_text.chars().count() > max_chars;

    for event in buffer {
        match event {
            Event::Text(t) => {
                let processed = {
                    let decoded = t.decode().unwrap_or_default();
                    let text =
                        escape::unescape(&decoded).unwrap_or_else(|_| decoded.clone());
                    emit_text_content(&text, needs_wrap, max_chars, writer)?
                };
                if !processed {
                    writer.write_event(Event::Text(t))?;
                }
            }
            other => {
                writer.write_event(other)?;
            }
        }
    }
    Ok(())
}

fn emit_text_content(
    text: &str,
    needs_wrap: bool,
    max_chars: usize,
    writer: &mut Writer<Vec<u8>>,
) -> Result<bool> {
    let has_literal_newlines = text.contains("\\n");

    if !has_literal_newlines && !needs_wrap {
        return Ok(false);
    }

    let segments: Vec<&str> = if has_literal_newlines {
        text.split("\\n").collect()
    } else {
        vec![text]
    };

    let mut first_segment = true;
    for segment in &segments {
        if !first_segment {
            writer.write_event(Event::Empty(BytesStart::new("br")))?;
        }
        first_segment = false;

        if needs_wrap {
            wrap_segment(segment, max_chars, writer)?;
        } else {
            writer.write_event(Event::Text(BytesText::from_escaped(
                escape::escape(*segment),
            )))?;
        }
    }

    Ok(true)
}

fn wrap_segment(
    segment: &str,
    max_chars: usize,
    writer: &mut Writer<Vec<u8>>,
) -> Result<()> {
    if max_chars == 0 {
        writer.write_event(Event::Text(BytesText::from_escaped(
            escape::escape(segment),
        )))?;
        return Ok(());
    }

    let words: Vec<&str> = segment.split_whitespace().collect();
    if words.is_empty() {
        writer.write_event(Event::Text(BytesText::from_escaped(
            escape::escape(segment),
        )))?;
        return Ok(());
    }

    let mut current_line = String::new();
    let mut first_line = true;

    for word in &words {
        let new_len = if current_line.is_empty() {
            word.chars().count()
        } else {
            current_line.chars().count() + 1 + word.chars().count()
        };

        if !current_line.is_empty() && new_len > max_chars {
            if !first_line {
                writer.write_event(Event::Empty(BytesStart::new("br")))?;
            }
            first_line = false;
            writer.write_event(Event::Text(BytesText::from_escaped(
                escape::escape(&current_line),
            )))?;
            current_line.clear();
        }

        if current_line.is_empty() {
            current_line.push_str(word);
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        if !first_line {
            writer.write_event(Event::Empty(BytesStart::new("br")))?;
        }
        writer.write_event(Event::Text(BytesText::from_escaped(
            escape::escape(&current_line),
        )))?;
    }

    Ok(())
}
