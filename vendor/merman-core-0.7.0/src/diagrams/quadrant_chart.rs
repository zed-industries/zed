use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuadrantChartStyles {
    pub radius: Option<i64>,
    pub color: Option<String>,
    pub stroke_color: Option<String>,
    pub stroke_width: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuadrantChartPointModel {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub class_name: Option<String>,
    pub styles: QuadrantChartStyles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuadrantChartQuadrantsModel {
    pub quadrant1_text: String,
    pub quadrant2_text: String,
    pub quadrant3_text: String,
    pub quadrant4_text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuadrantChartAxesModel {
    pub x_axis_left_text: String,
    pub x_axis_right_text: String,
    pub y_axis_bottom_text: String,
    pub y_axis_top_text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuadrantChartRenderModel {
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub quadrants: QuadrantChartQuadrantsModel,
    pub axes: QuadrantChartAxesModel,
    pub points: Vec<QuadrantChartPointModel>,
    pub classes: BTreeMap<String, QuadrantChartStyles>,
}

impl QuadrantChartRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Default)]
struct QuadrantDb {
    quadrant1_text: String,
    quadrant2_text: String,
    quadrant3_text: String,
    quadrant4_text: String,
    x_axis_left_text: String,
    x_axis_right_text: String,
    y_axis_bottom_text: String,
    y_axis_top_text: String,
    points: Vec<QuadrantChartPointModel>,
    classes: HashMap<String, QuadrantChartStyles>,
}

impl QuadrantDb {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn set_quadrant_text(&mut self, idx: u8, text: &str, config: &MermaidConfig) {
        let t = sanitize_text(text.trim(), config);
        match idx {
            1 => self.quadrant1_text = t,
            2 => self.quadrant2_text = t,
            3 => self.quadrant3_text = t,
            4 => self.quadrant4_text = t,
            _ => {}
        }
    }

    fn set_x_axis_left(&mut self, text: &str, config: &MermaidConfig) {
        self.x_axis_left_text = sanitize_text(text.trim(), config);
    }

    fn set_x_axis_right(&mut self, text: &str, config: &MermaidConfig) {
        self.x_axis_right_text = sanitize_text(text.trim(), config);
    }

    fn set_y_axis_bottom(&mut self, text: &str, config: &MermaidConfig) {
        self.y_axis_bottom_text = sanitize_text(text.trim(), config);
    }

    fn set_y_axis_top(&mut self, text: &str, config: &MermaidConfig) {
        self.y_axis_top_text = sanitize_text(text.trim(), config);
    }

    fn add_class(&mut self, class_name: &str, styles: &[String]) -> Result<()> {
        let parsed = parse_styles(styles)?;
        self.classes.insert(class_name.to_string(), parsed);
        Ok(())
    }

    fn add_point(
        &mut self,
        text: &str,
        class_name: Option<String>,
        x: f64,
        y: f64,
        styles: &[String],
        config: &MermaidConfig,
    ) -> Result<()> {
        let styles_obj = parse_styles(styles)?;
        let text = sanitize_text(text.trim(), config);
        let p = QuadrantChartPointModel {
            text,
            x,
            y,
            class_name,
            styles: styles_obj,
        };
        self.points.insert(0, p);
        Ok(())
    }
}

fn parse_styles(styles: &[String]) -> Result<QuadrantChartStyles> {
    let mut out = QuadrantChartStyles::default();
    for raw in styles {
        let style = raw.trim();
        if style.is_empty() {
            continue;
        }
        let (key, value) = style.split_once(':').ok_or_else(|| Error::DiagramParse {
            diagram_type: "quadrantChart".to_string(),
            message: format!("style named {style} is not supported."),
        })?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "radius" => {
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(Error::DiagramParse {
                        diagram_type: "quadrantChart".to_string(),
                        message: format!(
                            "value for {key} {value} is invalid, please use a valid number"
                        ),
                    });
                }
                out.radius = Some(value.parse::<i64>().map_err(|e| Error::DiagramParse {
                    diagram_type: "quadrantChart".to_string(),
                    message: e.to_string(),
                })?);
            }
            "color" => {
                if !is_valid_hex_code(value) {
                    return Err(Error::DiagramParse {
                        diagram_type: "quadrantChart".to_string(),
                        message: format!(
                            "value for {key} {value} is invalid, please use a valid hex code"
                        ),
                    });
                }
                out.color = Some(value.to_string());
            }
            "stroke-color" => {
                if !is_valid_hex_code(value) {
                    return Err(Error::DiagramParse {
                        diagram_type: "quadrantChart".to_string(),
                        message: format!(
                            "value for {key} {value} is invalid, please use a valid hex code"
                        ),
                    });
                }
                out.stroke_color = Some(value.to_string());
            }
            "stroke-width" => {
                if !is_valid_px(value) {
                    return Err(Error::DiagramParse {
                        diagram_type: "quadrantChart".to_string(),
                        message: format!(
                            "value for {key} {value} is invalid, please use a valid number of pixels (eg. 10px)"
                        ),
                    });
                }
                out.stroke_width = Some(value.to_string());
            }
            _ => {
                return Err(Error::DiagramParse {
                    diagram_type: "quadrantChart".to_string(),
                    message: format!("style named {key} is not supported."),
                });
            }
        }
    }
    Ok(out)
}

fn is_valid_hex_code(value: &str) -> bool {
    let v = value.strip_prefix('#').unwrap_or(value);
    (v.len() == 3 || v.len() == 6) && v.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_valid_px(value: &str) -> bool {
    let Some(num) = value.strip_suffix("px") else {
        return false;
    };
    !num.is_empty() && num.chars().all(|c| c.is_ascii_digit())
}

fn next_char_at(s: &str, idx: usize) -> Option<char> {
    s.get(idx..)?.chars().next()
}

fn strip_inline_comment(line: &str) -> &str {
    let mut in_quotes = false;
    let mut i = 0usize;
    while i + 1 < line.len() {
        let Some(ch) = next_char_at(line, i) else {
            break;
        };
        if ch == '"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }
        if !in_quotes && line[i..].starts_with("%%") {
            return &line[..i];
        }
        i += ch.len_utf8();
    }
    line
}

fn is_axis_delim_at(s: &str, idx: usize) -> Option<(usize, usize)> {
    let bytes = s.as_bytes();
    if idx >= bytes.len() {
        return None;
    }
    if bytes[idx] != b'-' {
        return None;
    }
    let mut j = idx;
    let mut dash_count = 0usize;
    while j < bytes.len() && bytes[j] == b'-' {
        dash_count += 1;
        j += 1;
    }
    if dash_count < 2 {
        return None;
    }
    if j < bytes.len() && bytes[j] == b'>' {
        Some((idx, j + 1))
    } else {
        None
    }
}

fn split_axis_text(s: &str) -> Option<(String, Option<String>)> {
    let mut in_quotes = false;
    let mut i = 0usize;
    while i < s.len() {
        let Some(ch) = next_char_at(s, i) else {
            break;
        };
        if ch == '"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }
        if !in_quotes {
            if let Some((start, end)) = is_axis_delim_at(s, i) {
                let left = s[..start].trim().to_string();
                let right = s[end..].trim().to_string();
                return Some((left, if right.is_empty() { None } else { Some(right) }));
            }
        }
        i += ch.len_utf8();
    }
    None
}

fn parse_text_value(raw: &str) -> Result<String> {
    let t = raw.trim();
    if t.starts_with("\"`") {
        let inner = t
            .strip_prefix("\"`")
            .and_then(|v| v.strip_suffix("`\""))
            .ok_or_else(|| Error::DiagramParse {
                diagram_type: "quadrantChart".to_string(),
                message: "unterminated markdown string".to_string(),
            })?;
        return Ok(inner.to_string());
    }
    if t.starts_with('"') {
        let inner = t
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .ok_or_else(|| Error::DiagramParse {
                diagram_type: "quadrantChart".to_string(),
                message: "unterminated string".to_string(),
            })?;
        return Ok(inner.to_string());
    }
    Ok(t.to_string())
}

fn parse_unit_interval_token(raw: &str) -> Result<f64> {
    let s = raw.trim();
    if s == "1" {
        return Ok(1.0);
    }
    if s == "0" {
        return Ok(0.0);
    }
    if let Some(rest) = s.strip_prefix("0.") {
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            return s.parse::<f64>().map_err(|e| Error::DiagramParse {
                diagram_type: "quadrantChart".to_string(),
                message: e.to_string(),
            });
        }
    }
    Err(Error::DiagramParse {
        diagram_type: "quadrantChart".to_string(),
        message: "invalid point coordinate".to_string(),
    })
}

fn parse_style_list(rest: &str) -> Vec<String> {
    rest.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn find_point_colon(s: &str) -> Option<usize> {
    let mut in_quotes = false;
    let mut i = 0usize;
    while i < s.len() {
        let Some(ch) = next_char_at(s, i) else {
            break;
        };
        if ch == '"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }
        if !in_quotes && ch == ':' {
            let mut j = i + 1;
            while j < s.len() {
                let Some(c2) = next_char_at(s, j) else {
                    break;
                };
                if c2.is_whitespace() {
                    j += c2.len_utf8();
                    continue;
                }
                if c2 == '[' {
                    return Some(i);
                }
                break;
            }
        }
        i += ch.len_utf8();
    }
    None
}

fn parse_point_statement(line: &str) -> Result<Option<PointStatement>> {
    let Some(colon_idx) = find_point_colon(line) else {
        return Ok(None);
    };
    let head = line[..colon_idx].trim_end().to_string();
    let tail = &line[colon_idx + 1..];

    let (class_name, label_raw) = if let Some(pos) = head.rfind(":::") {
        let (a, b) = head.split_at(pos);
        let class = b.trim_start_matches(":::").trim();
        if !class.is_empty() && class.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            (Some(class.to_string()), a.to_string())
        } else {
            (None, head.clone())
        }
    } else {
        (None, head.clone())
    };

    let label = parse_text_value(label_raw.trim())?;

    let t = tail.trim_start();
    let Some(after_bracket) = t.strip_prefix('[') else {
        return Err(Error::DiagramParse {
            diagram_type: "quadrantChart".to_string(),
            message: "expected '[' after ':'".to_string(),
        });
    };
    let (inside, after) = after_bracket
        .split_once(']')
        .ok_or_else(|| Error::DiagramParse {
            diagram_type: "quadrantChart".to_string(),
            message: "unterminated point coordinate; missing ']'".to_string(),
        })?;

    let mut xy = inside.split(',');
    let x_raw = xy.next().unwrap_or("").trim();
    let y_raw = xy.next().unwrap_or("").trim();
    let x = parse_unit_interval_token(x_raw)?;
    let y = parse_unit_interval_token(y_raw)?;

    let styles = parse_style_list(after);
    Ok(Some((label, class_name, x, y, styles)))
}

type PointStatement = (String, Option<String>, f64, f64, Vec<String>);

fn split_semicolons(line: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut in_quotes = false;
    let mut start = 0usize;
    let mut i = 0usize;
    while i < line.len() {
        let Some(ch) = next_char_at(line, i) else {
            break;
        };
        if ch == '"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }
        if !in_quotes && ch == ';' {
            out.push(&line[start..i]);
            start = i + 1;
            i += 1;
            continue;
        }
        i += ch.len_utf8();
    }
    out.push(&line[start..]);
    out
}

fn parse_colon_value_ci(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !t
        .get(..key.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(key))
    {
        return None;
    }
    let mut rest = &t[key.len()..];
    rest = rest.trim_start();
    if !rest.starts_with(':') {
        return None;
    }
    Some(rest[1..].trim().to_string())
}

fn parse_keyword_rest_ci(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !t
        .get(..key.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(key))
    {
        return None;
    }
    let rest = &t[key.len()..];
    Some(rest.trim_start().to_string())
}

pub fn parse_quadrant_chart(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let model = parse_quadrant_chart_model(code, meta)?;
    let mut out = Map::with_capacity(9);
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert("title".to_string(), json!(model.title));
    out.insert("accTitle".to_string(), json!(model.acc_title));
    out.insert("accDescr".to_string(), json!(model.acc_descr));
    out.insert("quadrants".to_string(), json!(model.quadrants));
    out.insert("axes".to_string(), json!(model.axes));
    out.insert("points".to_string(), json!(model.points));
    out.insert("classes".to_string(), json!(model.classes));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Ok(Value::Object(out))
}

pub fn parse_quadrant_chart_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<QuadrantChartRenderModel> {
    parse_quadrant_chart_model(code, meta)
}

fn parse_quadrant_chart_model(
    code: &str,
    meta: &ParseMetadata,
) -> Result<QuadrantChartRenderModel> {
    let mut db = QuadrantDb::default();
    db.clear();

    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;

    let mut saw_header = false;
    let mut in_acc_descr_block = false;
    let mut acc_descr_buf = String::new();

    for raw_line in code.lines() {
        let raw_line = raw_line.trim_end_matches('\r');
        if raw_line.trim().is_empty() {
            continue;
        }

        let raw_line = strip_inline_comment(raw_line);
        if raw_line.trim().is_empty() {
            continue;
        }

        if in_acc_descr_block {
            if let Some(end_idx) = raw_line.find('}') {
                acc_descr_buf.push_str(&raw_line[..end_idx]);
                acc_descr = Some(acc_descr_buf.trim().to_string());
                acc_descr_buf.clear();
                in_acc_descr_block = false;
                continue;
            }
            acc_descr_buf.push_str(raw_line);
            acc_descr_buf.push('\n');
            continue;
        }

        for stmt in split_semicolons(raw_line) {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            if stmt.trim_start().starts_with("%%") {
                continue;
            }

            if !saw_header {
                if stmt.eq_ignore_ascii_case("quadrantChart") {
                    saw_header = true;
                    continue;
                }
                return Err(Error::DiagramParse {
                    diagram_type: "quadrantChart".to_string(),
                    message: "expected quadrantChart".to_string(),
                });
            }

            if let Some(v) = parse_colon_value_ci(stmt, "accTitle") {
                acc_title = Some(v);
                continue;
            }
            if let Some(rest) = parse_keyword_rest_ci(stmt, "accDescr") {
                let rest = rest.trim_start();
                if let Some(after_lbrace) = rest.strip_prefix('{') {
                    in_acc_descr_block = true;
                    let after = after_lbrace.trim_start();
                    if !after.is_empty() {
                        acc_descr_buf.push_str(after);
                        acc_descr_buf.push('\n');
                    }
                    continue;
                }
                if let Some(v) = rest.strip_prefix(':') {
                    acc_descr = Some(v.trim().to_string());
                    continue;
                }
            }

            if let Some(rest) = parse_keyword_rest_ci(stmt, "title") {
                title = Some(rest.trim().to_string());
                continue;
            }

            if let Some(rest) = parse_keyword_rest_ci(stmt, "x-axis") {
                let rest = rest.trim_start();
                if let Some((left_raw, right_raw)) = split_axis_text(rest) {
                    let mut left = parse_text_value(&left_raw)?;
                    if right_raw.is_none() {
                        left.push_str(" ⟶");
                    }
                    db.set_x_axis_left(&left, &meta.effective_config);
                    if let Some(r) = right_raw {
                        let right = parse_text_value(&r)?;
                        db.set_x_axis_right(&right, &meta.effective_config);
                    }
                } else {
                    let left = parse_text_value(rest)?;
                    db.set_x_axis_left(&left, &meta.effective_config);
                }
                continue;
            }

            if let Some(rest) = parse_keyword_rest_ci(stmt, "y-axis") {
                let rest = rest.trim_start();
                if let Some((bottom_raw, top_raw)) = split_axis_text(rest) {
                    let mut bottom = parse_text_value(&bottom_raw)?;
                    if top_raw.is_none() {
                        bottom.push_str(" ⟶");
                    }
                    db.set_y_axis_bottom(&bottom, &meta.effective_config);
                    if let Some(t) = top_raw {
                        let top = parse_text_value(&t)?;
                        db.set_y_axis_top(&top, &meta.effective_config);
                    }
                } else {
                    let bottom = parse_text_value(rest)?;
                    db.set_y_axis_bottom(&bottom, &meta.effective_config);
                }
                continue;
            }

            let mut matched_quadrant = false;
            for (idx, kw) in [
                (1u8, "quadrant-1"),
                (2, "quadrant-2"),
                (3, "quadrant-3"),
                (4, "quadrant-4"),
            ] {
                if let Some(rest) = parse_keyword_rest_ci(stmt, kw) {
                    let t = parse_text_value(&rest)?;
                    db.set_quadrant_text(idx, &t, &meta.effective_config);
                    matched_quadrant = true;
                    break;
                }
            }
            if matched_quadrant {
                continue;
            }

            if let Some(rest) = parse_keyword_rest_ci(stmt, "classDef") {
                let mut parts = rest.trim_start().splitn(2, char::is_whitespace);
                let name = parts.next().unwrap_or("").trim();
                let style_str = parts.next().unwrap_or("").trim();
                if name.is_empty() {
                    return Err(Error::DiagramParse {
                        diagram_type: "quadrantChart".to_string(),
                        message: "expected classDef name".to_string(),
                    });
                }
                let styles = parse_style_list(style_str);
                db.add_class(name, &styles)?;
                continue;
            }

            if let Some((label, class_name, x, y, styles)) = parse_point_statement(stmt)? {
                db.add_point(&label, class_name, x, y, &styles, &meta.effective_config)?;
                continue;
            }

            return Err(Error::DiagramParse {
                diagram_type: "quadrantChart".to_string(),
                message: format!("Unrecognized statement: {stmt}"),
            });
        }
    }

    if !saw_header {
        return Err(Error::DiagramParse {
            diagram_type: "quadrantChart".to_string(),
            message: "expected quadrantChart".to_string(),
        });
    }

    Ok(QuadrantChartRenderModel {
        title,
        acc_title,
        acc_descr,
        quadrants: QuadrantChartQuadrantsModel {
            quadrant1_text: db.quadrant1_text,
            quadrant2_text: db.quadrant2_text,
            quadrant3_text: db.quadrant3_text,
            quadrant4_text: db.quadrant4_text,
        },
        axes: QuadrantChartAxesModel {
            x_axis_left_text: db.x_axis_left_text,
            x_axis_right_text: db.x_axis_right_text,
            y_axis_bottom_text: db.y_axis_bottom_text,
            y_axis_top_text: db.y_axis_top_text,
        },
        points: db.points,
        classes: db.classes.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated;
    use crate::{Engine, ParseOptions, RenderSemanticModel};
    use futures::executor::block_on;

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

    fn axes(model: &Value) -> &Value {
        &model["axes"]
    }

    fn quadrants(model: &Value) -> &Value {
        &model["quadrants"]
    }

    fn points(model: &Value) -> Vec<Value> {
        model["points"].as_array().cloned().unwrap_or_default()
    }

    #[test]
    fn errors_without_header() {
        let meta = ParseMetadata {
            diagram_type: "quadrantChart".to_string(),
            config: MermaidConfig::default(),
            effective_config: generated::default_site_config(),
            title: None,
        };
        let err = parse_quadrant_chart("quadrant-1 do\n", &meta)
            .unwrap_err()
            .to_string();
        assert!(err.contains("expected quadrantChart"));
    }

    #[test]
    fn header_only_is_allowed() {
        let model = parse("quadrantChart\n");
        assert_eq!(model["type"].as_str().unwrap(), "quadrantChart");
        assert!(model["title"].is_null());
    }

    #[test]
    fn parses_x_axis_text_and_missing_right_side() {
        let model = parse("quadrantChart\nx-axis urgent --> not urgent\n");
        assert_eq!(axes(&model)["xAxisLeftText"].as_str().unwrap(), "urgent");
        assert_eq!(
            axes(&model)["xAxisRightText"].as_str().unwrap(),
            "not urgent"
        );

        let model = parse("quadrantChart\nx-AxIs \"Urgent(* +=[?\"  --> \n");
        assert_eq!(
            axes(&model)["xAxisLeftText"].as_str().unwrap(),
            "Urgent(* +=[? ⟶"
        );
        assert_eq!(axes(&model)["xAxisRightText"].as_str().unwrap(), "");
    }

    #[test]
    fn parses_y_axis_text_and_missing_top_side() {
        let model = parse("quadrantChart\ny-axis urgent --> not urgent\n");
        assert_eq!(axes(&model)["yAxisBottomText"].as_str().unwrap(), "urgent");
        assert_eq!(axes(&model)["yAxisTopText"].as_str().unwrap(), "not urgent");

        let model = parse("quadrantChart\ny-AxIs \"Urgent(* +=[?\"  --> \n");
        assert_eq!(
            axes(&model)["yAxisBottomText"].as_str().unwrap(),
            "Urgent(* +=[? ⟶"
        );
        assert_eq!(axes(&model)["yAxisTopText"].as_str().unwrap(), "");
    }

    #[test]
    fn parses_quadrant_text_and_title() {
        let model = parse("quadrantChart\nquadrant-1 Plan\nquadrant-2 \"Do(* +=[?\"\n");
        assert_eq!(quadrants(&model)["quadrant1Text"].as_str().unwrap(), "Plan");
        assert_eq!(
            quadrants(&model)["quadrant2Text"].as_str().unwrap(),
            "Do(* +=[?"
        );

        let model = parse("quadrantChart\ntitle \"this is title (* +=[?\"\n");
        assert_eq!(
            model["title"].as_str().unwrap(),
            "\"this is title (* +=[?\""
        );
    }

    #[test]
    fn parses_points_and_validates_coordinate_range() {
        let model = parse("quadrantChart\npoint1: [0.1, 0.4]\n");
        let pts = points(&model);
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0]["text"].as_str().unwrap(), "point1");
        assert_eq!(pts[0]["x"].as_f64().unwrap(), 0.1);
        assert_eq!(pts[0]["y"].as_f64().unwrap(), 0.4);

        let model = parse("quadrantChart\n\"Point1 : (* +=[?\": [1, 0]\n");
        let pts = points(&model);
        assert_eq!(pts[0]["text"].as_str().unwrap(), "Point1 : (* +=[?");
        assert_eq!(pts[0]["x"].as_f64().unwrap(), 1.0);
        assert_eq!(pts[0]["y"].as_f64().unwrap(), 0.0);

        let err = parse_err("quadrantChart\nPoint1 : [1.2, 0.4]\n");
        assert!(err.contains("invalid point coordinate"));
    }

    #[test]
    fn parses_point_styles_and_classes() {
        let model = parse(
            "quadrantChart\nclassDef class1 color: #109060, radius : 10, stroke-color: #310085, stroke-width: 10px\nPoint A:::class1: [0.9, 0.0]\n",
        );
        let classes = model["classes"].as_object().unwrap();
        let class1 = classes.get("class1").unwrap();
        assert_eq!(class1["color"].as_str().unwrap(), "#109060");
        assert_eq!(class1["radius"].as_i64().unwrap(), 10);
        assert_eq!(class1["strokeColor"].as_str().unwrap(), "#310085");
        assert_eq!(class1["strokeWidth"].as_str().unwrap(), "10px");

        let pts = points(&model);
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0]["className"].as_str().unwrap(), "class1");

        let model = parse(
            "quadrantChart\nIncorta: [0.20, 0.30] radius: 10 ,color: #ff0000 ,stroke-color: #ff00ff ,stroke-width: 10px\n",
        );
        let pts = points(&model);
        let styles = &pts[0]["styles"];
        assert_eq!(styles["radius"].as_i64().unwrap(), 10);
        assert_eq!(styles["color"].as_str().unwrap(), "#ff0000");
        assert_eq!(styles["strokeColor"].as_str().unwrap(), "#ff00ff");
        assert_eq!(styles["strokeWidth"].as_str().unwrap(), "10px");
    }

    #[test]
    fn parses_whole_chart_example() {
        let model = parse(
            "quadrantChart\n\
title Analytics and Business Intelligence Platforms\n\
x-axis \"Completeness of Vision ?\" --> \"x-axis-2\"\n\
y-axis Ability to Execute --> \"y-axis-2\"\n\
quadrant-1 Leaders\n\
quadrant-2 Challengers\n\
quadrant-3 Niche\n\
quadrant-4 Visionaries\n\
Microsoft: [0.75, 0.75]\n\
Salesforce: [0.55, 0.60]\n\
IBM: [0.51, 0.40]\n\
Incorta: [0.20, 0.30]\n",
        );
        assert_eq!(
            axes(&model)["xAxisLeftText"].as_str().unwrap(),
            "Completeness of Vision ?"
        );
        assert_eq!(axes(&model)["xAxisRightText"].as_str().unwrap(), "x-axis-2");
        assert_eq!(
            axes(&model)["yAxisBottomText"].as_str().unwrap(),
            "Ability to Execute"
        );
        assert_eq!(axes(&model)["yAxisTopText"].as_str().unwrap(), "y-axis-2");
        assert_eq!(
            quadrants(&model)["quadrant1Text"].as_str().unwrap(),
            "Leaders"
        );
        assert_eq!(
            quadrants(&model)["quadrant4Text"].as_str().unwrap(),
            "Visionaries"
        );
        assert_eq!(points(&model).len(), 4);
    }

    #[test]
    fn parse_quadrant_chart_render_model_uses_typed_variant_without_changing_json_parse() {
        let engine = Engine::new();
        let input = r##"
quadrantChart
title Typed Quadrant
accTitle: Quadrant accTitle
accDescr: Quadrant accDescription
x-axis Low --> High
y-axis Bottom --> Top
quadrant-1 Expand
quadrant-2 Maintain
quadrant-3 Evaluate
quadrant-4 Retire
classDef priority color: #109060, radius : 10, stroke-color: #310085, stroke-width: 10px
Project A:::priority : [0.2, 0.8]
"##;

        let parsed = engine
            .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();

        assert_eq!(parsed.meta.diagram_type, "quadrantChart");
        match parsed.model {
            RenderSemanticModel::QuadrantChart(model) => {
                assert_eq!(model.title.as_deref(), Some("Typed Quadrant"));
                assert_eq!(model.acc_title.as_deref(), Some("Quadrant accTitle"));
                assert_eq!(model.acc_descr.as_deref(), Some("Quadrant accDescription"));
                assert_eq!(model.axes.x_axis_left_text, "Low");
                assert_eq!(model.axes.x_axis_right_text, "High");
                assert_eq!(model.quadrants.quadrant1_text, "Expand");
                assert_eq!(model.points.len(), 1);
                assert_eq!(model.points[0].text, "Project A");
                assert_eq!(model.points[0].class_name.as_deref(), Some("priority"));
                assert_eq!(model.classes["priority"].radius, Some(10));
            }
            other => panic!("quadrantChart render parse should return typed model, got {other:?}"),
        }

        let parsed_json = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed_json.model["type"], json!("quadrantChart"));
        assert_eq!(parsed_json.model["title"], json!("Typed Quadrant"));
        assert_eq!(parsed_json.model["accTitle"], json!("Quadrant accTitle"));
        assert_eq!(parsed_json.model["axes"]["xAxisLeftText"], json!("Low"));
        assert_eq!(
            parsed_json.model["quadrants"]["quadrant1Text"],
            json!("Expand")
        );
        assert_eq!(parsed_json.model["points"][0]["text"], json!("Project A"));
        assert_eq!(
            parsed_json.model["classes"]["priority"]["radius"],
            json!(10)
        );
        assert!(parsed_json.model.get("config").is_some());
    }

    #[test]
    fn parse_styles_matches_quadrantdb_spec() {
        let styles = vec![
            "radius: 10".to_string(),
            "color: #ff0000".to_string(),
            "stroke-color: #ff00ff".to_string(),
            "stroke-width: 10px".to_string(),
        ];
        let obj = parse_styles(&styles).unwrap();
        assert_eq!(obj.radius, Some(10));
        assert_eq!(obj.color.as_deref(), Some("#ff0000"));
        assert_eq!(obj.stroke_color.as_deref(), Some("#ff00ff"));
        assert_eq!(obj.stroke_width.as_deref(), Some("10px"));

        let err = parse_styles(&["test_name: value".to_string()])
            .unwrap_err()
            .to_string();
        assert!(err.contains("style named test_name is not supported."));

        let obj = parse_styles(&[]).unwrap();
        assert_eq!(obj.radius, None);
        assert!(obj.color.is_none());

        let err = parse_styles(&["radius: f".to_string()])
            .unwrap_err()
            .to_string();
        assert!(err.contains("value for radius f is invalid, please use a valid number"));
    }
}
