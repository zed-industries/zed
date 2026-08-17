use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde_json::{Map, Value, json};

const MAX_PACKET_SIZE: usize = 10_000;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct PacketDiagramRenderModel {
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub packet: Vec<Vec<PacketRenderBlock>>,
}

impl PacketDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PacketRenderBlock {
    pub start: i64,
    pub end: i64,
    pub bits: i64,
    pub label: String,
}

#[derive(Debug, Clone)]
struct PacketBlock {
    start: Option<i64>,
    end: Option<i64>,
    bits: Option<i64>,
    label: String,
}

enum PacketParseOutput {
    Empty,
    Model(PacketDiagramRenderModel),
}

type PacketWord = Vec<PacketRenderBlock>;

pub fn parse_packet(code: &str, meta: &ParseMetadata) -> Result<Value> {
    match parse_packet_model(code, meta)? {
        PacketParseOutput::Empty => Ok(json!({})),
        PacketParseOutput::Model(model) => {
            let mut out = Map::with_capacity(6);
            out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
            out.insert("title".to_string(), json!(model.title));
            out.insert("accTitle".to_string(), json!(model.acc_title));
            out.insert("accDescr".to_string(), json!(model.acc_descr));
            out.insert("packet".to_string(), json!(model.packet));
            out.insert(
                "config".to_string(),
                crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
            );
            Ok(Value::Object(out))
        }
    }
}

pub fn parse_packet_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<PacketDiagramRenderModel> {
    match parse_packet_model(code, meta)? {
        PacketParseOutput::Empty => Ok(PacketDiagramRenderModel::default()),
        PacketParseOutput::Model(model) => Ok(model),
    }
}

fn parse_packet_model(code: &str, meta: &ParseMetadata) -> Result<PacketParseOutput> {
    let mut lines = code.lines();

    let mut header = None;
    for line in &mut lines {
        let t = strip_inline_comment(line).trim();
        if !t.is_empty() {
            header = Some(t.to_string());
            break;
        }
    }

    let Some(header) = header else {
        return Ok(PacketParseOutput::Empty);
    };

    if !is_packet_header(&header) {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected packet".to_string(),
        });
    }

    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;
    let mut blocks: Vec<PacketBlock> = Vec::new();

    for line in lines {
        let t = strip_inline_comment(line).trim();
        if t.is_empty() {
            continue;
        }

        if let Some(v) = parse_title(t) {
            title = Some(v);
            continue;
        }
        if let Some(v) = parse_key_value(t, "accTitle") {
            acc_title = Some(v);
            continue;
        }
        if let Some(v) = parse_acc_descr(t) {
            acc_descr = Some(v);
            continue;
        }

        if let Some(block) = parse_packet_block(t) {
            blocks.push(block);
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unexpected packet statement: {t}"),
        });
    }

    let bits_per_row = config_i64(&meta.effective_config, "packet.bitsPerRow").unwrap_or(32);
    let packet = populate_packet(blocks, bits_per_row)?;

    Ok(PacketParseOutput::Model(PacketDiagramRenderModel {
        title,
        acc_title,
        acc_descr,
        packet,
    }))
}

fn populate_packet(blocks: Vec<PacketBlock>, bits_per_row: i64) -> Result<Vec<PacketWord>> {
    let mut packet: Vec<PacketWord> = Vec::new();
    let mut last_bit: i64 = -1;
    let mut word: PacketWord = Vec::new();
    let mut row: i64 = 1;

    for mut block in blocks {
        if let (Some(start), Some(end)) = (block.start, block.end) {
            if end < start {
                return Err(Error::DiagramParse {
                    diagram_type: "packet".to_string(),
                    message: format!(
                        "Packet block {start} - {end} is invalid. End must be greater than start."
                    ),
                });
            }
        }

        let start = block.start.unwrap_or(last_bit + 1);
        let end_for_msg = block.end.unwrap_or(start);
        if start != last_bit + 1 {
            return Err(Error::DiagramParse {
                diagram_type: "packet".to_string(),
                message: format!(
                    "Packet block {start} - {end_for_msg} is not contiguous. It should start from {}.",
                    last_bit + 1
                ),
            });
        }

        if block.bits == Some(0) {
            return Err(Error::DiagramParse {
                diagram_type: "packet".to_string(),
                message: format!("Packet block {start} is invalid. Cannot have a zero bit field."),
            });
        }

        let end = block.end.unwrap_or(start + block.bits.unwrap_or(1) - 1);
        let bits = block.bits.unwrap_or(end - start + 1);
        last_bit = end;

        let mut cur = PacketRenderBlock {
            start,
            end,
            bits,
            label: std::mem::take(&mut block.label),
        };

        while word.len() <= (bits_per_row + 1) as usize && packet.len() < MAX_PACKET_SIZE {
            let (fitting, next) = get_next_fitting_block(cur, row, bits_per_row)?;
            let reached_row_end = fitting.end + 1 == row * bits_per_row;
            word.push(fitting);
            if reached_row_end {
                if !word.is_empty() {
                    packet.push(std::mem::take(&mut word));
                }
                row += 1;
            }
            let Some(next) = next else {
                break;
            };
            cur = next;
        }
    }

    if !word.is_empty() {
        packet.push(word);
    }

    Ok(packet)
}

fn get_next_fitting_block(
    block: PacketRenderBlock,
    row: i64,
    bits_per_row: i64,
) -> Result<(PacketRenderBlock, Option<PacketRenderBlock>)> {
    if block.start > block.end {
        return Err(Error::DiagramParse {
            diagram_type: "packet".to_string(),
            message: format!(
                "Block start {} is greater than block end {}.",
                block.start, block.end
            ),
        });
    }

    if block.end < row * bits_per_row {
        return Ok((block, None));
    }

    let row_end = row * bits_per_row - 1;
    let row_start = row * bits_per_row;
    Ok((
        PacketRenderBlock {
            start: block.start,
            end: row_end,
            label: block.label.clone(),
            bits: row_end - block.start,
        },
        Some(PacketRenderBlock {
            start: row_start,
            end: block.end,
            label: block.label,
            bits: block.end - row_start,
        }),
    ))
}

fn strip_inline_comment(line: &str) -> &str {
    match line.find("%%") {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn is_packet_header(line: &str) -> bool {
    let t = line.trim_start();
    // Mermaid CLI treats the header keyword as a standalone token.
    // Do not accept arbitrary trailing text like `packet diagrams`.
    t == "packet" || t == "packet-beta"
}

fn parse_title(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("title") {
        return None;
    }
    let rest = t.strip_prefix("title")?.trim_start();
    Some(rest.to_string())
}

fn parse_key_value(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with(key) {
        return None;
    }
    let rest = t.strip_prefix(key)?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    Some(rest.to_string())
}

fn parse_acc_descr(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = t.strip_prefix("accDescr")?.trim_start();
    if let Some(rest) = rest.strip_prefix(':') {
        return Some(rest.trim_start().to_string());
    }
    if let Some(rest) = rest.strip_prefix('{') {
        let end = rest.find('}')?;
        return Some(rest[..end].to_string());
    }
    None
}

fn parse_packet_block(line: &str) -> Option<PacketBlock> {
    let mut rest = line.trim_start();

    let (start, end, bits) = if let Some(after_plus) = rest.strip_prefix('+') {
        let (bits, tail) = parse_int_token(after_plus.trim_start())?;
        rest = tail;
        (None, None, Some(bits))
    } else {
        let (start, tail) = parse_int_token(rest)?;
        rest = tail.trim_start();
        let mut end = None;
        if let Some(after_dash) = rest.strip_prefix('-') {
            let (e, tail) = parse_int_token(after_dash.trim_start())?;
            end = Some(e);
            rest = tail;
        }
        (Some(start), end, None)
    };

    let rest2 = rest.trim_start();
    let rest2 = rest2.strip_prefix(':')?.trim_start();
    let (label, tail) = parse_quoted_string(rest2)?;
    if !tail.trim().is_empty() {
        return None;
    }

    Some(PacketBlock {
        start,
        end,
        bits,
        label,
    })
}

fn parse_int_token(input: &str) -> Option<(i64, &str)> {
    let mut idx = 0usize;
    for c in input.chars() {
        if c.is_ascii_digit() {
            idx += c.len_utf8();
        } else {
            break;
        }
    }
    if idx == 0 {
        return None;
    }
    let token = &input[..idx];
    if token.len() > 1 && token.starts_with('0') {
        return None;
    }
    let value: i64 = token.parse().ok()?;
    Some((value, &input[idx..]))
}

fn parse_quoted_string(input: &str) -> Option<(String, &str)> {
    let mut chars = input.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let mut out = String::new();
    let mut escaped = false;
    let mut idx = 1;
    for c in chars {
        idx += c.len_utf8();
        if escaped {
            out.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some((out, &input[idx..]));
        }
        out.push(c);
    }
    None
}

fn config_i64(config: &MermaidConfig, dotted_path: &str) -> Option<i64> {
    let mut cur = config.as_value();
    for segment in dotted_path.split('.') {
        cur = cur.as_object()?.get(segment)?;
    }
    match cur {
        Value::Number(n) => n.as_i64().or_else(|| n.as_u64().map(|v| v as i64)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions};
    use futures::executor::block_on;
    use serde_json::json;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    fn parse_err(text: &str) -> String {
        let engine = Engine::new();
        match block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err() {
            Error::DiagramParse { message, .. } => message,
            other => other.to_string(),
        }
    }

    #[test]
    fn packet_beta_header_is_accepted() {
        let model = parse("packet-beta");
        assert_eq!(model["packet"], json!([]));
    }

    #[test]
    fn packet_header_is_accepted() {
        let model = parse("packet");
        assert_eq!(model["packet"], json!([]));
    }

    #[test]
    fn packet_header_does_not_accept_trailing_text() {
        assert_eq!(parse_err("packet diagrams"), "expected packet");
    }

    #[test]
    fn packet_data_and_title_are_parsed() {
        let model = parse(
            r#"packet
title Packet diagram
accTitle: Packet accTitle
accDescr: Packet accDescription
0-10: "test"
"#,
        );
        assert_eq!(model["title"], json!("Packet diagram"));
        assert_eq!(model["accTitle"], json!("Packet accTitle"));
        assert_eq!(model["accDescr"], json!("Packet accDescription"));
        assert_eq!(
            model["packet"],
            json!([
              [
                {
                  "bits": 11,
                  "end": 10,
                  "label": "test",
                  "start": 0
                }
              ]
            ])
        );
    }

    #[test]
    fn packet_single_bits_are_supported() {
        let model = parse(
            r#"packet
0-10: "test"
11: "single"
"#,
        );
        assert_eq!(
            model["packet"],
            json!([
              [
                {
                  "bits": 11,
                  "end": 10,
                  "label": "test",
                  "start": 0
                },
                {
                  "bits": 1,
                  "end": 11,
                  "label": "single",
                  "start": 11
                }
              ]
            ])
        );
    }

    #[test]
    fn packet_bit_counts_are_supported() {
        let model = parse(
            r#"packet
+8: "byte"
+16: "word"
"#,
        );
        assert_eq!(
            model["packet"],
            json!([
              [
                {
                  "bits": 8,
                  "end": 7,
                  "label": "byte",
                  "start": 0
                },
                {
                  "bits": 16,
                  "end": 23,
                  "label": "word",
                  "start": 8
                }
              ]
            ])
        );
    }

    #[test]
    fn packet_splits_into_multiple_rows() {
        let model = parse(
            r#"packet
0-10: "test"
11-90: "multiple"
"#,
        );
        assert_eq!(
            model["packet"],
            json!([
              [
                {
                  "bits": 11,
                  "end": 10,
                  "label": "test",
                  "start": 0
                },
                {
                  "bits": 20,
                  "end": 31,
                  "label": "multiple",
                  "start": 11
                }
              ],
              [
                {
                  "bits": 31,
                  "end": 63,
                  "label": "multiple",
                  "start": 32
                }
              ],
              [
                {
                  "bits": 26,
                  "end": 90,
                  "label": "multiple",
                  "start": 64
                }
              ]
            ])
        );
    }

    #[test]
    fn packet_splits_into_multiple_rows_at_exact_length() {
        let model = parse(
            r#"packet
0-16: "test"
17-63: "multiple"
"#,
        );
        assert_eq!(
            model["packet"],
            json!([
              [
                {
                  "bits": 17,
                  "end": 16,
                  "label": "test",
                  "start": 0
                },
                {
                  "bits": 14,
                  "end": 31,
                  "label": "multiple",
                  "start": 17
                }
              ],
              [
                {
                  "bits": 31,
                  "end": 63,
                  "label": "multiple",
                  "start": 32
                }
              ]
            ])
        );
    }

    #[test]
    fn packet_errors_if_numbers_are_not_continuous() {
        let err = parse_err(
            r#"packet
0-16: "test"
18-20: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 18 - 20 is not contiguous. It should start from 17."
        );
    }

    #[test]
    fn packet_errors_if_numbers_are_not_continuous_with_bit_counts() {
        let err = parse_err(
            r#"packet
+16: "test"
18-20: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 18 - 20 is not contiguous. It should start from 16."
        );
    }

    #[test]
    fn packet_errors_if_single_number_is_not_continuous() {
        let err = parse_err(
            r#"packet
0-16: "test"
18: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 18 - 18 is not contiguous. It should start from 17."
        );
    }

    #[test]
    fn packet_errors_if_single_number_is_not_continuous_with_bit_counts() {
        let err = parse_err(
            r#"packet
+16: "test"
18: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 18 - 18 is not contiguous. It should start from 16."
        );
    }

    #[test]
    fn packet_errors_if_single_number_is_not_continuous_2() {
        let err = parse_err(
            r#"packet
0-16: "test"
17: "good"
19: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 19 - 19 is not contiguous. It should start from 18."
        );
    }

    #[test]
    fn packet_errors_if_end_is_less_than_start() {
        let err = parse_err(
            r#"packet
0-16: "test"
25-20: "error"
"#,
        );
        assert_eq!(
            err,
            "Packet block 25 - 20 is invalid. End must be greater than start."
        );
    }

    #[test]
    fn packet_errors_if_bit_count_is_zero() {
        let err = parse_err(
            r#"packet
+0: "test"
"#,
        );
        assert_eq!(
            err,
            "Packet block 0 is invalid. Cannot have a zero bit field."
        );
    }
}
