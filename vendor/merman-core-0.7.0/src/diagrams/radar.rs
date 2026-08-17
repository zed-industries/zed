use crate::{Error, ParseMetadata, Result};
use serde_json::{Map, Value, json};

#[derive(Debug, Clone)]
struct AxisAst {
    name: String,
    label: Option<String>,
}

#[derive(Debug, Clone)]
struct EntryAst {
    axis: Option<String>,
    value: Value,
}

#[derive(Debug, Clone)]
struct CurveAst {
    name: String,
    label: Option<String>,
    entries: Vec<EntryAst>,
}

#[derive(Debug, Clone)]
enum OptionValueAst {
    Bool(bool),
    Number(Value),
    Graticule(String),
}

#[derive(Debug, Clone)]
struct OptionAst {
    name: String,
    value: OptionValueAst,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RadarRenderAxis {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RadarRenderCurve {
    pub name: String,
    pub label: String,
    #[serde(default)]
    pub entries: Vec<Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RadarRenderOptions {
    #[serde(rename = "showLegend")]
    pub show_legend: bool,
    pub ticks: Value,
    pub max: Option<Value>,
    pub min: Value,
    pub graticule: String,
}

impl Default for RadarRenderOptions {
    fn default() -> Self {
        Self {
            show_legend: true,
            ticks: json!(5),
            max: None,
            min: json!(0),
            graticule: "circle".to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct RadarDiagramRenderModel {
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub axes: Vec<RadarRenderAxis>,
    #[serde(default)]
    pub curves: Vec<RadarRenderCurve>,
    #[serde(default)]
    pub options: RadarRenderOptions,
}

impl RadarDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone)]
struct RadarDb {
    title: Option<String>,
    acc_title: Option<String>,
    acc_descr: Option<String>,
    axes: Vec<RadarRenderAxis>,
    curves: Vec<RadarRenderCurve>,
    options: RadarRenderOptions,
}

impl RadarDb {
    fn new() -> Self {
        Self {
            title: None,
            acc_title: None,
            acc_descr: None,
            axes: Vec::new(),
            curves: Vec::new(),
            options: RadarRenderOptions::default(),
        }
    }

    fn set_axes(&mut self, axes: Vec<AxisAst>) {
        self.axes = axes
            .into_iter()
            .map(|a| RadarRenderAxis {
                label: a.label.unwrap_or_else(|| a.name.clone()),
                name: a.name,
            })
            .collect();
    }

    fn set_curves(&mut self, curves: Vec<CurveAst>) -> Result<()> {
        let axes = self.axes.clone();
        self.curves = curves
            .into_iter()
            .map(|c| {
                let label = c.label.clone().unwrap_or_else(|| c.name.clone());
                let entries = compute_curve_entries(&axes, &c.entries)?;
                Ok(RadarRenderCurve {
                    name: c.name,
                    label,
                    entries,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn set_options(&mut self, options: Vec<OptionAst>) {
        let mut last: std::collections::HashMap<String, OptionValueAst> =
            std::collections::HashMap::new();
        for opt in options {
            last.insert(opt.name, opt.value);
        }

        if let Some(OptionValueAst::Bool(v)) = last.get("showLegend") {
            self.options.show_legend = *v;
        }
        if let Some(OptionValueAst::Number(v)) = last.get("ticks") {
            self.options.ticks = v.clone();
        }
        if let Some(OptionValueAst::Number(v)) = last.get("max") {
            self.options.max = Some(v.clone());
        }
        if let Some(OptionValueAst::Number(v)) = last.get("min") {
            self.options.min = v.clone();
        }
        if let Some(OptionValueAst::Graticule(v)) = last.get("graticule") {
            self.options.graticule = v.clone();
        }
    }

    #[inline]
    fn semantic_value(&self, meta: &ParseMetadata) -> Value {
        let mut out = Map::with_capacity(8);
        out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
        out.insert("title".to_string(), json!(self.title));
        out.insert("accTitle".to_string(), json!(self.acc_title));
        out.insert("accDescr".to_string(), json!(self.acc_descr));
        out.insert(
            "axes".to_string(),
            Value::Array(
                self.axes
                    .iter()
                    .map(|a| json!({"name": a.name, "label": a.label}))
                    .collect(),
            ),
        );
        out.insert(
            "curves".to_string(),
            Value::Array(
                self.curves
                    .iter()
                    .map(|c| json!({"name": c.name, "label": c.label, "entries": c.entries}))
                    .collect(),
            ),
        );
        out.insert(
            "options".to_string(),
            json!({
                "showLegend": self.options.show_legend,
                "ticks": self.options.ticks,
                "max": self.options.max,
                "min": self.options.min,
                "graticule": self.options.graticule,
            }),
        );
        out.insert(
            "config".to_string(),
            crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
        );
        Value::Object(out)
    }

    #[inline]
    fn into_render_model(self) -> RadarDiagramRenderModel {
        RadarDiagramRenderModel {
            title: self.title,
            acc_title: self.acc_title,
            acc_descr: self.acc_descr,
            axes: self.axes,
            curves: self.curves,
            options: self.options,
        }
    }
}

fn compute_curve_entries(axes: &[RadarRenderAxis], entries: &[EntryAst]) -> Result<Vec<Value>> {
    if entries.is_empty() {
        return Ok(Vec::new());
    }

    if entries[0].axis.is_none() {
        return Ok(entries.iter().map(|e| e.value.clone()).collect());
    }

    if axes.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "radar".to_string(),
            message: "Axes must be populated before curves for reference entries".to_string(),
        });
    }

    axes.iter()
        .map(|axis| {
            let found = entries
                .iter()
                .find(|e| e.axis.as_deref() == Some(&axis.name));
            let Some(found) = found else {
                return Err(Error::DiagramParse {
                    diagram_type: "radar".to_string(),
                    message: format!("Missing entry for axis {}", axis.label),
                });
            };
            Ok(found.value.clone())
        })
        .collect()
}

pub fn parse_radar(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let Some(db) = parse_radar_db(code, meta)? else {
        return Ok(json!({}));
    };
    Ok(db.semantic_value(meta))
}

pub fn parse_radar_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<RadarDiagramRenderModel> {
    parse_radar_db(code, meta).map(|db| db.map(RadarDb::into_render_model).unwrap_or_default())
}

#[inline]
fn parse_radar_db(code: &str, meta: &ParseMetadata) -> Result<Option<RadarDb>> {
    let mut lines = code.lines().peekable();

    let header = loop {
        let Some(line) = lines.next() else {
            return Ok(None);
        };
        let t = strip_inline_comment(line).trim();
        if t.is_empty() {
            continue;
        }
        break t.to_string();
    };

    if !is_radar_header(&header) {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected radar-beta".to_string(),
        });
    }

    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;

    let mut axes: Vec<AxisAst> = Vec::new();
    let mut curves: Vec<CurveAst> = Vec::new();
    let mut options: Vec<OptionAst> = Vec::new();

    while let Some(raw) = lines.next() {
        let t = strip_inline_comment(raw).trim().to_string();
        if t.is_empty() {
            continue;
        }

        if let Some(v) = parse_title(&t) {
            title = Some(v);
            continue;
        }
        if let Some(v) = parse_key_value(&t, "accTitle") {
            acc_title = Some(v);
            continue;
        }
        if let Some(v) = parse_acc_descr(&t) {
            acc_descr = Some(v);
            continue;
        }

        if let Some(rest) = t.strip_prefix("axis") {
            let rest = rest.trim_start();
            if rest.is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: "radar".to_string(),
                    message: "axis statement must include at least one axis".to_string(),
                });
            }
            axes.extend(
                parse_axes_list(rest).map_err(|message| Error::DiagramParse {
                    diagram_type: "radar".to_string(),
                    message,
                })?,
            );
            continue;
        }

        if t.trim_start().starts_with("curve") {
            let mut stmt = t;
            if stmt.contains('{') && !braces_balanced_outside_quotes(&stmt) {
                while let Some(next) = lines.peek().copied() {
                    let next = strip_inline_comment(next);
                    stmt.push('\n');
                    stmt.push_str(next);
                    lines.next();
                    if braces_balanced_outside_quotes(&stmt) {
                        break;
                    }
                }
            }
            curves.extend(
                parse_curves_stmt(&stmt).map_err(|message| Error::DiagramParse {
                    diagram_type: "radar".to_string(),
                    message,
                })?,
            );
            continue;
        }

        if let Some(opt) = parse_option_stmt(&t).map_err(|message| Error::DiagramParse {
            diagram_type: "radar".to_string(),
            message,
        })? {
            options.push(opt);
            continue;
        }

        if let Some(many) = parse_option_list_stmt(&t).map_err(|message| Error::DiagramParse {
            diagram_type: "radar".to_string(),
            message,
        })? {
            options.extend(many);
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: "radar".to_string(),
            message: format!("unexpected radar statement: {}", t.trim()),
        });
    }

    let mut db = RadarDb::new();
    db.title = title;
    db.acc_title = acc_title;
    db.acc_descr = acc_descr;
    db.set_axes(axes);
    db.set_curves(curves)?;
    db.set_options(options);

    Ok(Some(db))
}

fn strip_inline_comment(line: &str) -> &str {
    match line.find("%%") {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn is_radar_header(line: &str) -> bool {
    let t = line.trim();
    t == "radar-beta"
        || t == "radar-beta:"
        || (t.starts_with("radar-beta") && t[9..].trim_start().starts_with(':'))
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

fn braces_balanced_outside_quotes(s: &str) -> bool {
    let mut in_quote: Option<char> = None;
    let mut escaped = false;
    let mut depth = 0i64;
    for ch in s.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quote.is_some() {
            escaped = true;
            continue;
        }
        if let Some(q) = in_quote {
            if ch == q {
                in_quote = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_quote = Some(ch);
            continue;
        }
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
        }
    }
    depth == 0
}

fn parse_axes_list(input: &str) -> std::result::Result<Vec<AxisAst>, String> {
    let mut p = TokenParser::new(input);
    let mut out = Vec::new();
    loop {
        p.skip_ws();
        if p.eof() {
            break;
        }
        let name = p.parse_id().ok_or_else(|| "expected axis id".to_string())?;
        p.skip_ws();
        let label = if p.try_consume('[') {
            p.skip_ws();
            let s = p
                .parse_quoted_string()
                .ok_or_else(|| "expected quoted axis label".to_string())?;
            p.skip_ws();
            if !p.try_consume(']') {
                return Err("expected ']'".to_string());
            }
            Some(s)
        } else {
            None
        };
        out.push(AxisAst { name, label });
        p.skip_ws();
        if p.try_consume(',') {
            // Upstream Mermaid rejects a trailing comma at the end of an `axis` list.
            p.skip_ws();
            if p.eof() {
                return Err("unexpected trailing ',' in axis list".to_string());
            }
            continue;
        }
        if p.eof() {
            break;
        }
        return Err("expected ',' or end of axis list".to_string());
    }
    Ok(out)
}

fn parse_curves_stmt(input: &str) -> std::result::Result<Vec<CurveAst>, String> {
    let rest = input
        .trim_start()
        .strip_prefix("curve")
        .ok_or_else(|| "expected curve".to_string())?
        .trim_start();
    if rest.trim().is_empty() {
        return Err("expected curve id".to_string());
    }

    let chunks = split_top_level(rest, ',');
    let mut curves = Vec::new();
    for chunk in chunks {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        curves.push(parse_curve(chunk)?);
    }
    Ok(curves)
}

fn parse_curve(input: &str) -> std::result::Result<CurveAst, String> {
    let mut p = TokenParser::new(input);
    p.skip_ws();
    let name = p
        .parse_id()
        .ok_or_else(|| "expected curve id".to_string())?;
    p.skip_ws();
    let label = if p.try_consume('[') {
        p.skip_ws();
        let s = p
            .parse_quoted_string()
            .ok_or_else(|| "expected quoted curve label".to_string())?;
        p.skip_ws();
        if !p.try_consume(']') {
            return Err("expected ']'".to_string());
        }
        p.skip_ws();
        Some(s)
    } else {
        None
    };

    if !p.try_consume('{') {
        return Err("expected '{'".to_string());
    }

    let entries_str = p.take_until_matching_brace()?;
    let entries = parse_entries(&entries_str)?;

    p.skip_ws();
    if !p.eof() {
        return Err("unexpected trailing tokens after curve".to_string());
    }

    let has_detailed = entries.iter().any(|e| e.axis.is_some());
    let has_numeric = entries.iter().any(|e| e.axis.is_none());
    if has_detailed && has_numeric {
        return Err("mixed detailed and numeric entries are not supported".to_string());
    }

    Ok(CurveAst {
        name,
        label,
        entries,
    })
}

fn parse_entries(input: &str) -> std::result::Result<Vec<EntryAst>, String> {
    let items = split_top_level(input, ',');
    let mut out = Vec::new();
    for item in items {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        // Try detailed first: <ID> ':'? <NUMBER>
        let mut p = TokenParser::new(item);
        p.skip_ws();
        let start_pos = p.pos;
        if let Some(axis) = p.parse_id() {
            p.skip_ws();
            p.try_consume(':');
            p.skip_ws();
            if let Some(num) = p.parse_number_value() {
                p.skip_ws();
                if p.eof() {
                    out.push(EntryAst {
                        axis: Some(axis),
                        value: num,
                    });
                    continue;
                }
            }
        }
        p.pos = start_pos;

        // Otherwise numeric: <NUMBER>
        p.skip_ws();
        let num = p
            .parse_number_value()
            .ok_or_else(|| "expected entry number".to_string())?;
        p.skip_ws();
        if !p.eof() {
            return Err("unexpected trailing tokens in entry".to_string());
        }
        out.push(EntryAst {
            axis: None,
            value: num,
        });
    }
    Ok(out)
}

fn parse_option_stmt(input: &str) -> std::result::Result<Option<OptionAst>, String> {
    let mut p = TokenParser::new(input);
    p.skip_ws();
    let name = match p.parse_id().as_deref() {
        Some("showLegend") => "showLegend",
        Some("ticks") => "ticks",
        Some("max") => "max",
        Some("min") => "min",
        Some("graticule") => "graticule",
        _ => return Ok(None),
    }
    .to_string();
    p.skip_ws();

    if name == "showLegend" {
        let v = p
            .parse_bool()
            .ok_or_else(|| "expected boolean".to_string())?;
        p.skip_ws();
        if !p.eof() {
            return Err("unexpected trailing tokens after option".to_string());
        }
        return Ok(Some(OptionAst {
            name,
            value: OptionValueAst::Bool(v),
        }));
    }

    if name == "graticule" {
        let v = p
            .parse_id()
            .ok_or_else(|| "expected graticule".to_string())?;
        if v != "circle" && v != "polygon" {
            return Err("expected graticule".to_string());
        }
        p.skip_ws();
        if !p.eof() {
            return Err("unexpected trailing tokens after option".to_string());
        }
        return Ok(Some(OptionAst {
            name,
            value: OptionValueAst::Graticule(v),
        }));
    }

    let v = p
        .parse_number_value()
        .ok_or_else(|| "expected number".to_string())?;
    p.skip_ws();
    if !p.eof() {
        return Err("unexpected trailing tokens after option".to_string());
    }
    Ok(Some(OptionAst {
        name,
        value: OptionValueAst::Number(v),
    }))
}

fn parse_option_list_stmt(input: &str) -> std::result::Result<Option<Vec<OptionAst>>, String> {
    if !input.contains(',') {
        return Ok(None);
    }
    let chunks = split_top_level(input, ',');
    let mut out = Vec::new();
    for chunk in chunks {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        let Some(opt) = parse_option_stmt(chunk)? else {
            return Ok(None);
        };
        out.push(opt);
    }
    if out.is_empty() {
        return Ok(None);
    }
    Ok(Some(out))
}

fn split_top_level(input: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote: Option<char> = None;
    let mut escaped = false;
    let mut brace_depth = 0i64;
    let mut bracket_depth = 0i64;
    for ch in input.chars() {
        if escaped {
            cur.push(ch);
            escaped = false;
            continue;
        }
        if let Some(q) = in_quote {
            if ch == '\\' {
                cur.push(ch);
                escaped = true;
                continue;
            }
            if ch == q {
                in_quote = None;
            }
            cur.push(ch);
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_quote = Some(ch);
            cur.push(ch);
            continue;
        }
        match ch {
            '{' => brace_depth += 1,
            '}' => brace_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            _ => {}
        }
        if ch == delim && brace_depth == 0 && bracket_depth == 0 {
            out.push(std::mem::take(&mut cur));
            continue;
        }
        cur.push(ch);
    }
    out.push(cur);
    out
}

struct TokenParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> TokenParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
                continue;
            }
            break;
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn try_consume(&mut self, ch: char) -> bool {
        if self.input[self.pos..].starts_with(ch) {
            self.pos += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn parse_id(&mut self) -> Option<String> {
        let s = &self.input[self.pos..];
        let mut chars = s.chars();
        let first = chars.next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        let mut idx = first.len_utf8();
        let mut last = first;
        for ch in chars {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                idx += ch.len_utf8();
                last = ch;
            } else {
                break;
            }
        }
        if last == '-' {
            return None;
        }
        let raw = &s[..idx];
        self.pos += idx;
        Some(raw.to_string())
    }

    fn parse_bool(&mut self) -> Option<bool> {
        if self.input[self.pos..].starts_with("true") {
            self.pos += 4;
            return Some(true);
        }
        if self.input[self.pos..].starts_with("false") {
            self.pos += 5;
            return Some(false);
        }
        None
    }

    fn parse_number_value(&mut self) -> Option<Value> {
        let s = &self.input[self.pos..];
        let mut idx = 0usize;
        let mut saw_dot = false;
        for ch in s.chars() {
            if ch.is_ascii_digit() {
                idx += ch.len_utf8();
                continue;
            }
            if ch == '.' && !saw_dot {
                saw_dot = true;
                idx += 1;
                continue;
            }
            break;
        }
        if idx == 0 {
            return None;
        }
        let token = &s[..idx];

        if saw_dot {
            if token.ends_with('.') {
                return None;
            }
            let v: f64 = token.parse().ok()?;
            self.pos += idx;
            let n = serde_json::Number::from_f64(v)?;
            return Some(Value::Number(n));
        }

        if token.len() > 1 && token.starts_with('0') {
            return None;
        }
        let v: i64 = token.parse().ok()?;
        self.pos += idx;
        Some(Value::Number(serde_json::Number::from(v)))
    }

    fn parse_quoted_string(&mut self) -> Option<String> {
        let quote = self.peek_char()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        self.pos += 1;
        let mut out = String::new();
        let mut escaped = false;
        while let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
            if escaped {
                out.push(ch);
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                return Some(out);
            }
            out.push(ch);
        }
        None
    }

    fn take_until_matching_brace(&mut self) -> std::result::Result<String, String> {
        let mut depth = 1i64;
        let mut in_quote: Option<char> = None;
        let mut escaped = false;
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
            if escaped {
                out.push(ch);
                escaped = false;
                continue;
            }
            if let Some(q) = in_quote {
                if ch == '\\' {
                    out.push(ch);
                    escaped = true;
                    continue;
                }
                if ch == q {
                    in_quote = None;
                }
                out.push(ch);
                continue;
            }
            if ch == '"' || ch == '\'' {
                in_quote = Some(ch);
                out.push(ch);
                continue;
            }
            if ch == '{' {
                depth += 1;
                out.push(ch);
                continue;
            }
            if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return Ok(out);
                }
                out.push(ch);
                continue;
            }
            out.push(ch);
        }
        Err("unterminated '{' in curve".to_string())
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
    fn radar_parses_simple_definition() {
        let _ = parse(
            r#"radar-beta
axis A,B,C
curve mycurve{1,2,3}"#,
        );
    }

    #[test]
    fn radar_errors_on_empty_axis_statement() {
        assert_eq!(
            parse_err(
                r#"radar-beta
axis"#,
            ),
            "axis statement must include at least one axis"
        );
    }

    #[test]
    fn radar_parses_title_and_data() {
        let model = parse(
            r#"radar-beta
title Radar diagram
accTitle: Radar accTitle
accDescr: Radar accDescription
axis A["Axis A"], B["Axis B"] ,C["Axis C"]
curve mycurve["My Curve"]{1,2,3}
"#,
        );
        assert_eq!(model["title"], json!("Radar diagram"));
        assert_eq!(model["accTitle"], json!("Radar accTitle"));
        assert_eq!(model["accDescr"], json!("Radar accDescription"));
        assert_eq!(
            model["axes"],
            json!([
                {"name": "A", "label": "Axis A"},
                {"name": "B", "label": "Axis B"},
                {"name": "C", "label": "Axis C"},
            ])
        );
        assert_eq!(
            model["curves"],
            json!([
                {"name": "mycurve", "label": "My Curve", "entries": [1,2,3]},
            ])
        );
        assert_eq!(
            model["options"],
            json!({"showLegend": true, "ticks": 5, "max": Value::Null, "min": 0, "graticule": "circle"})
        );
    }

    #[test]
    fn radar_parses_options() {
        let model = parse(
            r#"radar-beta
ticks 10
showLegend false
graticule polygon
min 1
max 10
"#,
        );
        assert_eq!(
            model["options"],
            json!({"showLegend": false, "ticks": 10, "max": 10, "min": 1, "graticule": "polygon"})
        );
    }

    #[test]
    fn radar_errors_on_empty_curve_stmt() {
        let err = parse_err(
            r#"radar-beta
axis my-axis
curve
"#,
        );
        assert_eq!(err, "expected curve id");
    }

    #[test]
    fn radar_errors_on_mixed_numeric_and_detailed_curve_entries() {
        let err = parse_err(
            r#"radar-beta
axis ax1, ax2
curve my-curve { 1, ax1 2 }"#,
        );
        assert_eq!(err, "mixed detailed and numeric entries are not supported");
    }

    #[test]
    fn radar_orders_detailed_curve_entries_by_axes() {
        let model = parse(
            r#"radar-beta
axis A,B,C
curve mycurve{ C: 3, A: 1, B: 2 }"#,
        );
        assert_eq!(
            model["curves"],
            json!([
                {"name": "mycurve", "label": "mycurve", "entries": [1,2,3]},
            ])
        );
    }

    #[test]
    fn radar_accepts_header_with_colon() {
        let _ = parse(
            r#"radar-beta:
axis A,B,C
curve mycurve{1,2,3}"#,
        );
    }

    #[test]
    fn radar_ignores_comment_lines() {
        let _ = parse(
            r#"radar-beta
%% This is a comment
axis A,B,C
%% This is another comment
curve mycurve{1,2,3}
"#,
        );
    }

    #[test]
    fn radar_errors_on_missing_axis_entry() {
        let err = parse_err(
            r#"radar-beta
axis A["Axis A"], B["Axis B"], C["Axis C"]
curve mycurve{ C: 3, A: 1 }"#,
        );
        assert_eq!(err, "Missing entry for axis Axis B");
    }

    #[test]
    fn radar_parses_config_override_directive() {
        let model = parse(
            r#"
%%{init: {'radar': {'marginTop': 80, 'axisLabelFactor': 1.25}}}%%
radar-beta
axis A,B,C
curve mycurve{1,2,3}
"#,
        );
        assert_eq!(model["config"]["radar"]["marginTop"], json!(80));
    }
}
