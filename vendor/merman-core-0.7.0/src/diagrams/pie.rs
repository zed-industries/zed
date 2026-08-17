use crate::{Error, ParseMetadata, Result};
use serde_json::{Value, json};
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct PieDiagramRenderModel {
    #[serde(rename = "showData")]
    pub show_data: bool,
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub sections: Vec<PieRenderSection>,
}

impl PieDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PieRenderSection {
    pub label: String,
    pub value: f64,
}

enum PieParseOutput {
    Empty,
    ExpectedPie,
    Model(PieDiagramRenderModel),
}

pub fn parse_pie(code: &str, meta: &ParseMetadata) -> Result<Value> {
    match parse_pie_model(code, meta)? {
        PieParseOutput::Empty => Ok(json!({})),
        PieParseOutput::ExpectedPie => Ok(json!({ "error": "expected pie" })),
        PieParseOutput::Model(model) => Ok(json!({
            "type": meta.diagram_type,
            "showData": model.show_data,
            "title": model.title,
            "accTitle": model.acc_title,
            "accDescr": model.acc_descr,
            "sections": model.sections,
        })),
    }
}

pub fn parse_pie_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<PieDiagramRenderModel> {
    match parse_pie_model(code, meta)? {
        PieParseOutput::Empty => Ok(PieDiagramRenderModel::default()),
        PieParseOutput::ExpectedPie => Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected pie".to_string(),
        }),
        PieParseOutput::Model(model) => Ok(model),
    }
}

fn parse_pie_model(code: &str, meta: &ParseMetadata) -> Result<PieParseOutput> {
    let mut raw_lines = code.lines();

    let mut header: Option<String> = None;
    for line in &mut raw_lines {
        let t = strip_inline_comment(line).trim();
        if !t.is_empty() {
            header = Some(t.to_string());
            break;
        }
    }

    let Some(header) = header else {
        return Ok(PieParseOutput::Empty);
    };

    let mut it0 = header.split_whitespace();
    let Some(first) = it0.next() else {
        return Ok(PieParseOutput::Empty);
    };
    if first != "pie" {
        return Ok(PieParseOutput::ExpectedPie);
    }

    let mut show_data = false;
    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;
    let mut unsupported: Option<String> = None;

    fn token_boundary_ok(s: &str, token_len: usize) -> bool {
        let Some(rest) = s.get(token_len..) else {
            return true;
        };
        match rest.chars().next() {
            None => true,
            Some(c) => c.is_whitespace(),
        }
    }

    let header_after = header
        .trim_start_matches(|c: char| c.is_whitespace())
        .strip_prefix("pie")
        .unwrap_or("");
    let mut rest = header_after.trim_start();
    while !rest.is_empty() {
        if rest.starts_with("showData") && token_boundary_ok(rest, "showData".len()) {
            show_data = true;
            rest = rest["showData".len()..].trim_start();
            continue;
        }
        if rest.starts_with("title") && token_boundary_ok(rest, "title".len()) {
            let after = rest["title".len()..].trim_start();
            title = Some(after.to_string());
            rest = "";
            continue;
        }
        if rest.starts_with("accTitle") {
            if let Some(v) = parse_key_value(rest, "accTitle") {
                acc_title = Some(v);
                rest = "";
                continue;
            }
        }
        if rest.starts_with("accDescr") {
            if let Some(v) = parse_acc_descr_inline(rest) {
                acc_descr = Some(v);
                rest = "";
                continue;
            }
            if starts_acc_descr_block(rest) {
                let mut parts: Vec<String> = Vec::new();
                for next_line in raw_lines.by_ref() {
                    let s = strip_inline_comment(next_line);
                    if s.contains('}') {
                        let before = s.split('}').next().unwrap_or("").trim();
                        if !before.is_empty() {
                            parts.push(before.to_string());
                        }
                        break;
                    }
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    parts.push(trimmed.to_string());
                }
                acc_descr = Some(parts.join("\n"));
                rest = "";
                continue;
            }
        }
        unsupported = Some(rest.split_whitespace().next().unwrap_or(rest).to_string());
        break;
    }

    if let Some(tok) = unsupported {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unexpected pie header token: {tok}"),
        });
    }

    let mut sections: Vec<PieRenderSection> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let mut lines = raw_lines.peekable();
    while let Some(line) = lines.next() {
        let t = strip_inline_comment(line).trim();
        if t.is_empty() {
            continue;
        }

        if let Some(v) = parse_title_statement(t) {
            title = Some(v);
            continue;
        }

        if let Some(v) = parse_key_value(t, "accTitle") {
            acc_title = Some(v);
            continue;
        }

        if let Some(v) = parse_acc_descr_inline(t) {
            acc_descr = Some(v);
            continue;
        }

        if starts_acc_descr_block(t) {
            let mut parts: Vec<String> = Vec::new();
            for next_line in lines.by_ref() {
                let s = strip_inline_comment(next_line);
                if s.contains('}') {
                    let before = s.split('}').next().unwrap_or("").trim();
                    if !before.is_empty() {
                        parts.push(before.to_string());
                    }
                    break;
                }
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    continue;
                }
                parts.push(trimmed.to_string());
            }
            acc_descr = Some(parts.join("\n"));
            continue;
        }

        if let Some((label, value)) = parse_section(t) {
            if value < 0.0 {
                return Err(Error::DiagramParse {
                    diagram_type: meta.diagram_type.clone(),
                    message: format!(
                        "\"{label}\" has invalid value: {value}. Negative values are not allowed in pie charts. All slice values must be >= 0."
                    ),
                });
            }
            if seen.insert(label.clone()) {
                sections.push(PieRenderSection { label, value });
            }
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unexpected pie statement: {t}"),
        });
    }

    Ok(PieParseOutput::Model(PieDiagramRenderModel {
        show_data,
        title,
        acc_title,
        acc_descr,
        sections,
    }))
}

fn strip_inline_comment(line: &str) -> &str {
    match line.find("%%") {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn parse_title_statement(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("title") {
        return None;
    }
    let rest = t.strip_prefix("title")?;
    match rest.chars().next() {
        None => Some(String::new()),
        Some(c) if c.is_whitespace() => Some(rest.trim_start().to_string()),
        _ => None,
    }
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

fn parse_acc_descr_inline(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = t.strip_prefix("accDescr")?.trim_start();
    if let Some(rest) = rest.strip_prefix(':') {
        return Some(rest.trim_start().to_string());
    }
    None
}

fn starts_acc_descr_block(line: &str) -> bool {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return false;
    }
    let rest = t.trim_start_matches("accDescr").trim_start();
    rest.starts_with('{')
}

fn parse_section(line: &str) -> Option<(String, f64)> {
    let t = line.trim_start();
    let (label, rest) = parse_quoted_string(t)?;
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();

    let mut num = String::new();
    for c in rest.chars() {
        if c.is_ascii_digit() || c == '-' || c == '.' {
            num.push(c);
        } else {
            break;
        }
    }
    if num.is_empty() {
        return None;
    }
    let value: f64 = num.parse().ok()?;
    Some((label, value))
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

#[cfg(test)]
mod tests {
    use crate::{Engine, ParseOptions};

    #[test]
    fn pie_supports_title_statement_after_header() {
        let engine = Engine::new();
        let input = r#"
pie showData
  title Market Share
  "A" : 1
  "B" : 2
"#;

        let parsed = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .expect("diagram detected");

        assert_eq!(parsed.meta.diagram_type, "pie");
        assert_eq!(
            parsed.model.get("title").and_then(|v| v.as_str()),
            Some("Market Share")
        );
        assert_eq!(
            parsed.model.get("showData").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn pie_supports_header_acc_title_inline() {
        let engine = Engine::new();
        let input = r#"
pie accTitle: sample wow
  "A" : 1
"#;

        let parsed = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .expect("diagram detected");

        assert_eq!(parsed.meta.diagram_type, "pie");
        assert_eq!(
            parsed.model.get("accTitle").and_then(|v| v.as_str()),
            Some("sample wow")
        );
    }

    #[test]
    fn pie_supports_header_acc_descr_block() {
        let engine = Engine::new();
        let input = r#"
pie accDescr {
  first line
  second line
}
  "A" : 1
"#;

        let parsed = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .expect("diagram detected");

        assert_eq!(parsed.meta.diagram_type, "pie");
        assert_eq!(
            parsed.model.get("accDescr").and_then(|v| v.as_str()),
            Some("first line\nsecond line")
        );
    }
}
