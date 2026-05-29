//! Converts literal `\n` escape sequences inside `<foreignObject>` elements
//! into `<br/>` tags so that line breaks render correctly.
//!
//! ```xml
//! <!-- before -->
//! <foreignObject>Hello\nWorld</foreignObject>
//!
//! <!-- after -->
//! <foreignObject>Hello<br/>World</foreignObject>
//! ```

use anyhow::{Context as _, Result};
use quick_xml::escape;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};

pub(super) fn process(svg: &str) -> Result<String> {
    let mut reader = Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    let mut writer = Writer::new(Vec::with_capacity(svg.len()));

    let mut foreign_object_depth: usize = 0;
    let mut buffer = Vec::new();

    loop {
        let event = match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => event,
            Err(e) => return Err(e).context("failed to parse SVG in foreignObject wrap pass"),
        };

        let is_fo_start =
            matches!(&event, Event::Start(e) if e.name().as_ref() == b"foreignObject");
        let is_fo_end = matches!(&event, Event::End(e) if e.name().as_ref() == b"foreignObject");

        if is_fo_start {
            if foreign_object_depth == 0 {
                buffer.clear();
            }
            buffer.push(event);
            foreign_object_depth += 1;
        } else if is_fo_end {
            foreign_object_depth = foreign_object_depth.saturating_sub(1);
            buffer.push(event);
            if foreign_object_depth == 0 {
                emit_buffered(std::mem::take(&mut buffer), &mut writer)?;
            }
        } else if foreign_object_depth > 0 {
            buffer.push(event);
        } else {
            writer.write_event(event)?;
        }
    }

    String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")
}

fn emit_buffered(buffer: Vec<Event<'_>>, writer: &mut Writer<Vec<u8>>) -> Result<()> {
    for event in buffer {
        match event {
            Event::Text(t) => {
                let processed = {
                    let decoded = t.decode().unwrap_or_default();
                    let text = escape::unescape(&decoded).unwrap_or_else(|_| decoded.clone());
                    emit_text_content(&text, writer)?
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

fn emit_text_content(text: &str, writer: &mut Writer<Vec<u8>>) -> Result<bool> {
    if !text.contains("\\n") {
        return Ok(false);
    }

    let mut first_segment = true;
    for segment in text.split("\\n") {
        if !first_segment {
            writer.write_event(Event::Empty(BytesStart::new("br")))?;
        }
        first_segment = false;
        writer.write_event(Event::Text(BytesText::from_escaped(escape::escape(
            segment,
        ))))?;
    }

    Ok(true)
}
