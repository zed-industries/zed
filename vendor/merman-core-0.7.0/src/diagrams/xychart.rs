use crate::sanitize::sanitize_text;
use crate::{Error, ParseMetadata, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value, json};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum XyChartAxisRenderModel {
    #[serde(rename = "band")]
    Band {
        #[serde(default)]
        title: String,
        #[serde(default)]
        categories: Vec<String>,
    },
    #[serde(rename = "linear")]
    Linear {
        #[serde(default)]
        title: String,
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum XyChartPlotType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "bar")]
    Bar,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XyChartPlotRenderModel {
    #[serde(rename = "type")]
    pub plot_type: XyChartPlotType,
    pub values: Vec<f64>,
    pub data: Vec<(String, Option<f64>)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XyChartDiagramRenderModel {
    #[serde(default)]
    pub orientation: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(rename = "xAxis")]
    pub x_axis: XyChartAxisRenderModel,
    #[serde(rename = "yAxis")]
    pub y_axis: XyChartAxisRenderModel,
    #[serde(default)]
    pub plots: Vec<XyChartPlotRenderModel>,
}

impl XyChartDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }

    pub(crate) fn to_compat_json(&self, meta: &ParseMetadata) -> Value {
        let mut out = Map::with_capacity(10);
        out.insert(
            "orientation".to_string(),
            Value::String(self.orientation.clone()),
        );
        out.insert("title".to_string(), option_string_value(&self.title));
        out.insert("accTitle".to_string(), option_string_value(&self.acc_title));
        out.insert("accDescr".to_string(), option_string_value(&self.acc_descr));
        out.insert("xAxis".to_string(), axis_value(&self.x_axis));
        out.insert("yAxis".to_string(), axis_value(&self.y_axis));
        out.insert("plots".to_string(), plots_value(&self.plots));
        out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
        out.insert(
            "config".to_string(),
            crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
        );
        Value::Object(out)
    }
}

fn option_string_value(value: &Option<String>) -> Value {
    value
        .as_ref()
        .map(|value| Value::String(value.clone()))
        .unwrap_or(Value::Null)
}

fn optional_f64_value(value: Option<f64>) -> Value {
    value
        .and_then(Number::from_f64)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn f64_value(value: f64) -> Value {
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn string_array_value(values: &[String]) -> Value {
    Value::Array(values.iter().cloned().map(Value::String).collect())
}

fn axis_value(axis: &XyChartAxisRenderModel) -> Value {
    let mut out = Map::new();
    match axis {
        XyChartAxisRenderModel::Band { title, categories } => {
            out.insert("type".to_string(), Value::String("band".to_string()));
            out.insert("title".to_string(), Value::String(title.clone()));
            out.insert("categories".to_string(), string_array_value(categories));
        }
        XyChartAxisRenderModel::Linear { title, min, max } => {
            out.insert("type".to_string(), Value::String("linear".to_string()));
            out.insert("title".to_string(), Value::String(title.clone()));
            out.insert("min".to_string(), optional_f64_value(*min));
            out.insert("max".to_string(), optional_f64_value(*max));
        }
    }
    Value::Object(out)
}

fn plots_value(plots: &[XyChartPlotRenderModel]) -> Value {
    Value::Array(plots.iter().map(plot_value).collect())
}

fn plot_value(plot: &XyChartPlotRenderModel) -> Value {
    let mut out = Map::new();
    out.insert(
        "type".to_string(),
        Value::String(plot_type_name(plot.plot_type)),
    );
    out.insert(
        "values".to_string(),
        Value::Array(plot.values.iter().copied().map(f64_value).collect()),
    );
    out.insert("data".to_string(), plot_data_value(&plot.data));
    Value::Object(out)
}

fn plot_type_name(plot_type: XyChartPlotType) -> String {
    match plot_type {
        XyChartPlotType::Line => "line".to_string(),
        XyChartPlotType::Bar => "bar".to_string(),
    }
}

fn plot_data_value(data: &[(String, Option<f64>)]) -> Value {
    Value::Array(
        data.iter()
            .map(|(category, value)| {
                Value::Array(vec![
                    Value::String(category.clone()),
                    optional_f64_value(*value),
                ])
            })
            .collect(),
    )
}

#[derive(Debug, Clone)]
enum AxisData {
    Band {
        title: String,
        categories: Vec<String>,
    },
    Linear {
        title: String,
        min: f64,
        max: f64,
    },
}

#[derive(Debug, Clone)]
struct Plot {
    plot_type: XyChartPlotType,
    values: Vec<f64>,
    data: Vec<(String, Option<f64>)>,
}

#[derive(Debug, Clone)]
struct XyChartState {
    orientation: String,
    x_axis: AxisData,
    y_axis: AxisData,
    plots: Vec<Plot>,
    has_set_x_axis: bool,
    has_set_y_axis: bool,
}

impl XyChartState {
    fn new(meta: &ParseMetadata) -> Self {
        let orientation = meta
            .effective_config
            .get_str("xyChart.chartOrientation")
            .unwrap_or("vertical")
            .to_string();
        Self {
            orientation,
            x_axis: AxisData::Band {
                title: String::new(),
                categories: Vec::new(),
            },
            y_axis: AxisData::Linear {
                title: String::new(),
                min: f64::INFINITY,
                max: f64::NEG_INFINITY,
            },
            plots: Vec::new(),
            has_set_x_axis: false,
            has_set_y_axis: false,
        }
    }

    fn set_orientation(&mut self, o: &str) {
        if o.eq_ignore_ascii_case("horizontal") {
            self.orientation = "horizontal".to_string();
        } else {
            self.orientation = "vertical".to_string();
        }
    }

    fn set_x_axis_title(&mut self, title: &str, meta: &ParseMetadata) {
        let t = sanitize_text(title.trim(), &meta.effective_config);
        match &mut self.x_axis {
            AxisData::Band { title, .. } => *title = t,
            AxisData::Linear { title, .. } => *title = t,
        }
    }

    fn set_y_axis_title(&mut self, title: &str, meta: &ParseMetadata) {
        let t = sanitize_text(title.trim(), &meta.effective_config);
        match &mut self.y_axis {
            AxisData::Linear { title, .. } => *title = t,
            AxisData::Band { title, .. } => *title = t,
        }
    }

    fn set_x_axis_range(&mut self, min: f64, max: f64) {
        let title = match &self.x_axis {
            AxisData::Band { title, .. } => title.clone(),
            AxisData::Linear { title, .. } => title.clone(),
        };
        self.x_axis = AxisData::Linear { title, min, max };
        self.has_set_x_axis = true;
    }

    fn set_x_axis_band(&mut self, categories: Vec<String>, meta: &ParseMetadata) {
        let title = match &self.x_axis {
            AxisData::Band { title, .. } => title.clone(),
            AxisData::Linear { title, .. } => title.clone(),
        };
        let categories = categories
            .into_iter()
            .map(|c| sanitize_text(c.trim(), &meta.effective_config))
            .collect::<Vec<_>>();
        self.x_axis = AxisData::Band { title, categories };
        self.has_set_x_axis = true;
    }

    fn set_y_axis_range(&mut self, min: f64, max: f64) {
        let title = match &self.y_axis {
            AxisData::Linear { title, .. } => title.clone(),
            AxisData::Band { title, .. } => title.clone(),
        };
        self.y_axis = AxisData::Linear { title, min, max };
        self.has_set_y_axis = true;
    }

    fn set_y_axis_range_from_plot_data(&mut self, data: &[f64]) {
        let min_value = data.iter().copied().fold(f64::INFINITY, |a, b| a.min(b));
        let max_value = data
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));

        let (prev_min, prev_max, title) = match &self.y_axis {
            AxisData::Linear { min, max, title } => (*min, *max, title.clone()),
            AxisData::Band { title, .. } => (f64::INFINITY, f64::NEG_INFINITY, title.clone()),
        };

        self.y_axis = AxisData::Linear {
            title,
            min: prev_min.min(min_value),
            max: prev_max.max(max_value),
        };
    }

    fn transform_data_without_category(&mut self, data: &[f64]) -> Vec<(String, Option<f64>)> {
        if data.is_empty() {
            return Vec::new();
        }

        if !self.has_set_x_axis {
            let (prev_min, prev_max) = match &self.x_axis {
                AxisData::Linear { min, max, .. } => (*min, *max),
                AxisData::Band { .. } => (f64::INFINITY, f64::NEG_INFINITY),
            };
            self.set_x_axis_range(prev_min.min(1.0), prev_max.max(data.len() as f64));
        }

        if !self.has_set_y_axis {
            self.set_y_axis_range_from_plot_data(data);
        }

        match &self.x_axis {
            AxisData::Band { categories, .. } => categories
                .iter()
                .enumerate()
                .map(|(i, c)| (c.clone(), data.get(i).copied()))
                .collect(),
            AxisData::Linear { min, max, .. } => {
                let denom = (data.len() as f64) - 1.0;
                let step = (*max - *min) / denom;
                let mut cats = Vec::new();
                let mut i = *min;
                while i <= *max {
                    cats.push(format!("{i}"));
                    i += step;
                    if denom == 0.0 {
                        break;
                    }
                }
                cats.into_iter()
                    .enumerate()
                    .map(|(idx, c)| (c, data.get(idx).copied()))
                    .collect()
            }
        }
    }

    fn add_line_data(&mut self, data: Vec<f64>) {
        let pairs = self.transform_data_without_category(&data);
        self.plots.push(Plot {
            plot_type: XyChartPlotType::Line,
            values: data,
            data: pairs,
        });
    }

    fn add_bar_data(&mut self, data: Vec<f64>) {
        let pairs = self.transform_data_without_category(&data);
        self.plots.push(Plot {
            plot_type: XyChartPlotType::Bar,
            values: data,
            data: pairs,
        });
    }

    fn into_render_model(
        self,
        title: Option<String>,
        acc_title: Option<String>,
        acc_descr: Option<String>,
    ) -> XyChartDiagramRenderModel {
        XyChartDiagramRenderModel {
            orientation: self.orientation,
            title,
            acc_title,
            acc_descr,
            x_axis: axis_data_to_render_model(self.x_axis),
            y_axis: axis_data_to_render_model(self.y_axis),
            plots: self
                .plots
                .into_iter()
                .map(|p| XyChartPlotRenderModel {
                    plot_type: p.plot_type,
                    values: p.values,
                    data: p.data,
                })
                .collect(),
        }
    }
}

pub fn parse_xychart(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let Some(model) = parse_xychart_model(code, meta)? else {
        return Ok(json!({}));
    };

    Ok(model.to_compat_json(meta))
}

pub fn parse_xychart_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<XyChartDiagramRenderModel> {
    Ok(parse_xychart_model(code, meta)?.unwrap_or_else(empty_render_model))
}

fn parse_xychart_model(
    code: &str,
    meta: &ParseMetadata,
) -> Result<Option<XyChartDiagramRenderModel>> {
    let cleaned = strip_comments(code);
    let statements = split_statements(&cleaned);

    let mut it = statements.into_iter().filter(|s| !s.trim().is_empty());
    let Some(header_stmt) = it.next() else {
        return Ok(None);
    };

    let mut state = XyChartState::new(meta);
    parse_header(&header_stmt, &mut state)?;

    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;

    for stmt in it {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }

        if let Some(rest) = strip_keyword(stmt, "title") {
            let t = parse_text(rest)?;
            title = Some(t.trim().to_string());
            continue;
        }
        if let Some(rest) = strip_keyword(stmt, "accTitle") {
            let rest = rest.trim_start();
            let Some(v) = rest.strip_prefix(':') else {
                return Err(Error::DiagramParse {
                    diagram_type: "xychart".to_string(),
                    message: "expected ':' after accTitle".to_string(),
                });
            };
            acc_title = Some(v.trim().to_string());
            continue;
        }
        if let Some(rest) = strip_keyword(stmt, "accDescr") {
            let rest = rest.trim_start();
            if let Some(v) = rest.strip_prefix(':') {
                acc_descr = Some(v.trim().to_string());
                continue;
            }
            if let Some(after) = rest.strip_prefix('{') {
                let Some(end) = after.find('}') else {
                    return Err(Error::DiagramParse {
                        diagram_type: "xychart".to_string(),
                        message: "unterminated accDescr block".to_string(),
                    });
                };
                let trailing = &after[end + 1..];
                if !trailing.trim().is_empty() {
                    return Err(Error::DiagramParse {
                        diagram_type: "xychart".to_string(),
                        message: "unexpected trailing tokens after accDescr block".to_string(),
                    });
                }
                acc_descr = Some(after[..end].trim().to_string());
                continue;
            }
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "expected ':' or '{' after accDescr".to_string(),
            });
        }

        if let Some(rest) = strip_keyword(stmt, "x-axis") {
            parse_x_axis(rest, &mut state, meta)?;
            continue;
        }
        if let Some(rest) = strip_keyword(stmt, "y-axis") {
            parse_y_axis(rest, &mut state, meta)?;
            continue;
        }
        if let Some(rest) = strip_keyword(stmt, "line") {
            let (_plot_title, data) = parse_plot_stmt(rest)?;
            state.add_line_data(data);
            continue;
        }
        if let Some(rest) = strip_keyword(stmt, "bar") {
            let (_plot_title, data) = parse_plot_stmt(rest)?;
            state.add_bar_data(data);
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: format!("unexpected xychart statement: {stmt}"),
        });
    }

    Ok(Some(state.into_render_model(title, acc_title, acc_descr)))
}

fn empty_render_model() -> XyChartDiagramRenderModel {
    XyChartDiagramRenderModel {
        orientation: "vertical".to_string(),
        title: None,
        acc_title: None,
        acc_descr: None,
        x_axis: XyChartAxisRenderModel::Band {
            title: String::new(),
            categories: Vec::new(),
        },
        y_axis: XyChartAxisRenderModel::Linear {
            title: String::new(),
            min: None,
            max: None,
        },
        plots: Vec::new(),
    }
}

fn axis_data_to_render_model(axis: AxisData) -> XyChartAxisRenderModel {
    match axis {
        AxisData::Band { title, categories } => XyChartAxisRenderModel::Band { title, categories },
        AxisData::Linear { title, min, max } => {
            let min = min.is_finite().then_some(min);
            let max = max.is_finite().then_some(max);
            XyChartAxisRenderModel::Linear { title, min, max }
        }
    }
}

fn parse_header(stmt: &str, state: &mut XyChartState) -> Result<()> {
    let t = stmt.trim();
    let lower = t.to_ascii_lowercase();
    let (prefix, rest) = if lower.starts_with("xychart-beta") {
        ("xychart-beta", &t["xychart-beta".len()..])
    } else if lower.starts_with("xychart") {
        ("xychart", &t["xychart".len()..])
    } else {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "expected xychart".to_string(),
        });
    };

    let rem = rest.trim();
    if rem.is_empty() {
        return Ok(());
    }
    if !rest.starts_with(char::is_whitespace) {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: format!("unexpected token after {prefix}: {rem}"),
        });
    }

    if rem.eq_ignore_ascii_case("vertical") || rem.eq_ignore_ascii_case("horizontal") {
        state.set_orientation(rem);
        return Ok(());
    }

    Err(Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: format!("invalid chart orientation: {rem}"),
    })
}

fn strip_keyword<'a>(stmt: &'a str, kw: &str) -> Option<&'a str> {
    let s = stmt.trim_start();
    let lower = s.to_ascii_lowercase();
    let kw_lower = kw.to_ascii_lowercase();
    if !lower.starts_with(&kw_lower) {
        return None;
    }
    Some(&s[kw.len()..])
}

fn parse_text(input: &str) -> Result<String> {
    let t = input.trim_start();
    if let Some(body) = t.strip_prefix("\"`") {
        let Some(end) = body.find("`\"") else {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unterminated markdown string".to_string(),
            });
        };
        let s = &body[..end];
        let rest = &body[end + 2..];
        if !rest.trim().is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unexpected trailing tokens after text".to_string(),
            });
        }
        return Ok(s.to_string());
    }

    if let Some(body) = t.strip_prefix('"') {
        let Some(end) = body.find('"') else {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unterminated string".to_string(),
            });
        };
        let s = &body[..end];
        let rest = &body[end + 1..];
        if !rest.trim().is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unexpected trailing tokens after text".to_string(),
            });
        }
        return Ok(s.to_string());
    }

    let mut out = String::new();
    for part in t.split_whitespace() {
        out.push_str(part);
    }
    if out.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "expected text".to_string(),
        });
    }
    Ok(out)
}

fn parse_number(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    // Accept +, -, integers, decimals, and leading dot decimals.
    let ok = t
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == '.');
    if !ok {
        return None;
    }
    t.parse::<f64>().ok()
}

fn parse_x_axis(rest: &str, state: &mut XyChartState, meta: &ParseMetadata) -> Result<()> {
    let t = rest.trim();
    if t.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "x-axis requires data".to_string(),
        });
    }

    if t.starts_with('[') {
        state.set_x_axis_title("", meta);
        let cats = parse_text_list_in_brackets(t)?;
        state.set_x_axis_band(cats, meta);
        return Ok(());
    }

    if let Some((min, max)) = try_parse_range(t)? {
        state.set_x_axis_title("", meta);
        state.set_x_axis_range(min, max);
        return Ok(());
    }

    let (title, tail) = parse_text_prefix(t)?;
    state.set_x_axis_title(&title, meta);
    let tail = tail.trim_start();
    if tail.is_empty() {
        return Ok(());
    }
    if tail.starts_with('[') {
        let cats = parse_text_list_in_brackets(tail)?;
        state.set_x_axis_band(cats, meta);
        return Ok(());
    }
    if let Some((min, max)) = try_parse_range(tail)? {
        state.set_x_axis_range(min, max);
        return Ok(());
    }

    Err(Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: "invalid x-axis data".to_string(),
    })
}

fn parse_y_axis(rest: &str, state: &mut XyChartState, meta: &ParseMetadata) -> Result<()> {
    let t = rest.trim();
    if t.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "y-axis requires data".to_string(),
        });
    }

    if let Some((min, max)) = try_parse_range(t)? {
        state.set_y_axis_title("", meta);
        state.set_y_axis_range(min, max);
        return Ok(());
    }

    if t.starts_with('[') {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "y-axis does not support band data".to_string(),
        });
    }

    let (title, tail) = parse_text_prefix(t)?;
    state.set_y_axis_title(&title, meta);
    let tail = tail.trim_start();
    if tail.is_empty() {
        return Ok(());
    }

    if tail.starts_with('[') {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "y-axis does not support band data".to_string(),
        });
    }

    if let Some((min, max)) = try_parse_range(tail)? {
        state.set_y_axis_range(min, max);
        return Ok(());
    }

    Err(Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: "invalid y-axis data".to_string(),
    })
}

fn parse_plot_stmt(rest: &str) -> Result<(String, Vec<f64>)> {
    let t = rest.trim();
    if t.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "plot requires data".to_string(),
        });
    }

    if t.starts_with('[') {
        let data = parse_number_list_in_brackets(t)?;
        if data.is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "plot data cannot be empty".to_string(),
            });
        }
        return Ok((String::new(), data));
    }

    let (title, tail) = parse_text_prefix(t)?;
    let tail = tail.trim_start();
    if !tail.starts_with('[') {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "plot data missing".to_string(),
        });
    }
    let data = parse_number_list_in_brackets(tail)?;
    if data.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "plot data cannot be empty".to_string(),
        });
    }
    Ok((title, data))
}

fn try_parse_range(input: &str) -> Result<Option<(f64, f64)>> {
    let mut s = input.trim_start();
    let Some((a_str, tail)) = take_number_token(s) else {
        return Ok(None);
    };
    s = tail.trim_start();
    if !s.starts_with("-->") {
        return Ok(None);
    }
    s = &s[3..];
    let Some((b_str, tail)) = take_number_token(s.trim_start()) else {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "expected number".to_string(),
        });
    };
    if !tail.trim().is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "unexpected trailing tokens after range".to_string(),
        });
    }
    let a = parse_number(&a_str).ok_or_else(|| Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: "invalid number".to_string(),
    })?;
    let b = parse_number(&b_str).ok_or_else(|| Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: "invalid number".to_string(),
    })?;
    Ok(Some((a, b)))
}

fn take_number_token(input: &str) -> Option<(String, &str)> {
    let mut idx = 0usize;
    for (i, ch) in input.char_indices() {
        if i == 0 && (ch == '+' || ch == '-') {
            idx = i + ch.len_utf8();
            continue;
        }
        if ch.is_ascii_digit() || ch == '.' {
            idx = i + ch.len_utf8();
            continue;
        }
        break;
    }
    if idx == 0 {
        return None;
    }
    Some((input[..idx].to_string(), &input[idx..]))
}

fn parse_text_prefix(input: &str) -> Result<(String, &str)> {
    let t = input.trim_start();
    if let Some(body) = t.strip_prefix("\"`") {
        let Some(end) = body.find("`\"") else {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unterminated markdown string".to_string(),
            });
        };
        let s = &body[..end];
        let rest = &body[end + 2..];
        return Ok((s.to_string(), rest));
    }
    if let Some(body) = t.strip_prefix('"') {
        let Some(end) = body.find('"') else {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unterminated string".to_string(),
            });
        };
        let s = &body[..end];
        let rest = &body[end + 1..];
        return Ok((s.to_string(), rest));
    }
    let mut end = t.len();
    for (i, ch) in t.char_indices() {
        if ch.is_whitespace() || ch == '[' {
            end = i;
            break;
        }
    }
    let head = &t[..end];
    if head.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "expected text".to_string(),
        });
    }
    Ok((head.to_string(), &t[end..]))
}

fn parse_text_list_in_brackets(input: &str) -> Result<Vec<String>> {
    let t = input.trim_start();
    let inner = extract_bracket_inner(t)?;
    let parts = split_top_level_commas(inner);
    let mut out = Vec::new();
    for p in parts {
        let p = p.trim();
        if p.is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "empty category".to_string(),
            });
        }
        out.push(parse_text(p)?);
    }
    Ok(out)
}

fn parse_number_list_in_brackets(input: &str) -> Result<Vec<f64>> {
    let t = input.trim_start();
    let inner = extract_bracket_inner(t)?;
    let parts = split_top_level_commas(inner);
    let mut out = Vec::new();
    for p in parts {
        let p = p.trim();
        if p.is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "empty number".to_string(),
            });
        }
        let n = parse_number(p).ok_or_else(|| Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: format!("invalid number: {p}"),
        })?;
        out.push(n);
    }
    Ok(out)
}

fn extract_bracket_inner(input: &str) -> Result<&str> {
    let t = input.trim_start();
    if !t.starts_with('[') {
        return Err(Error::DiagramParse {
            diagram_type: "xychart".to_string(),
            message: "expected '['".to_string(),
        });
    }
    let mut in_quote = false;
    let mut in_md = false;
    let mut idx = 1usize;
    while idx < t.len() {
        let ch = t.as_bytes()[idx] as char;
        if in_md {
            if t[idx..].starts_with("`\"") {
                in_md = false;
                idx += 2;
                continue;
            }
            idx += 1;
            continue;
        }
        if in_quote {
            if ch == '"' {
                in_quote = false;
            }
            idx += 1;
            continue;
        }
        if t[idx..].starts_with("\"`") {
            in_md = true;
            idx += 2;
            continue;
        }
        if ch == '"' {
            in_quote = true;
            idx += 1;
            continue;
        }
        if ch == '[' {
            return Err(Error::DiagramParse {
                diagram_type: "xychart".to_string(),
                message: "unbalanced '['".to_string(),
            });
        }
        if ch == ']' {
            let inner = &t[1..idx];
            let rest = &t[idx + 1..];
            if !rest.trim().is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: "xychart".to_string(),
                    message: "unexpected trailing tokens after ']'".to_string(),
                });
            }
            return Ok(inner);
        }
        idx += 1;
    }

    Err(Error::DiagramParse {
        diagram_type: "xychart".to_string(),
        message: "unbalanced ']'".to_string(),
    })
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut in_quote = false;
    let mut in_md = false;
    let mut start = 0usize;
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i] as char;
        if in_md {
            if input[i..].starts_with("`\"") {
                in_md = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if in_quote {
            if ch == '"' {
                in_quote = false;
            }
            i += 1;
            continue;
        }
        if input[i..].starts_with("\"`") {
            in_md = true;
            i += 2;
            continue;
        }
        if ch == '"' {
            in_quote = true;
            i += 1;
            continue;
        }
        if ch == ',' {
            out.push(&input[start..i]);
            start = i + 1;
            i += 1;
            continue;
        }
        i += 1;
    }
    out.push(&input[start..]);
    out
}

fn strip_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for line in input.split_inclusive('\n') {
        let mut in_quote = false;
        let mut chars = line.char_indices().peekable();
        let mut cut = line.len();
        while let Some((idx, ch)) = chars.next() {
            if in_quote {
                if ch == '"' {
                    in_quote = false;
                }
                continue;
            }
            if ch == '"' {
                in_quote = true;
                continue;
            }
            if ch == '%' && chars.peek().is_some_and(|(_, n)| *n == '%') {
                cut = idx;
                break;
            }
        }
        let kept = &line[..cut];
        if kept.trim_start().starts_with("%%{") {
            continue;
        }
        if kept.trim_start().starts_with("%%") {
            continue;
        }
        out.push_str(kept);
    }
    out
}

fn split_statements(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    let mut in_md = false;
    let mut bracket_depth = 0i64;
    let mut brace_depth = 0i64;

    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_md {
            cur.push(ch);
            if ch == '`' && chars.peek() == Some(&'"') {
                cur.push('"');
                chars.next();
                in_md = false;
            }
            continue;
        }
        if in_quote {
            cur.push(ch);
            if ch == '"' {
                in_quote = false;
            }
            continue;
        }

        if ch == '"' && chars.peek() == Some(&'`') {
            cur.push('"');
            cur.push('`');
            chars.next();
            in_md = true;
            continue;
        }

        if ch == '"' {
            cur.push(ch);
            in_quote = true;
            continue;
        }

        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            '{' => brace_depth += 1,
            '}' => brace_depth -= 1,
            _ => {}
        }

        if (ch == '\n' || ch == ';') && bracket_depth == 0 && brace_depth == 0 {
            out.push(std::mem::take(&mut cur));
            continue;
        }

        cur.push(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
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
    fn xychart_header_only_is_accepted() {
        let model = parse("xychart");
        assert_eq!(model["plots"], json!([]));
    }

    #[test]
    fn xychart_invalid_header_throws() {
        let err = parse_err("xychart-1");
        assert!(err.contains("unexpected"));
    }

    #[test]
    fn xychart_orientation_is_parsed() {
        let model = parse("xychart horizontal");
        assert_eq!(model["orientation"], json!("horizontal"));
    }

    #[test]
    fn xychart_orientation_invalid_throws() {
        let err = parse_err("xychart abc");
        assert!(err.contains("invalid chart orientation"));
    }

    #[test]
    fn xychart_title_parses_quoted_and_unquoted() {
        let model = parse("xychart\ntitle \"This is a title\"");
        assert_eq!(model["title"], json!("This is a title"));

        let model = parse("xychart\ntitle oneLinertitle");
        assert_eq!(model["title"], json!("oneLinertitle"));
    }

    #[test]
    fn xychart_parses_axis_band_and_range_and_plots() {
        let model = parse(
            r#"xychart horizontal
title "Basic xychart"
x-axis "this is x axis" [category1, "category 2", category3]
y-axis yaxisText 10 --> 150
bar barTitle1 [23, 45, 56.6]
line lineTitle1 [11, 45.5, 67, 23]
"#,
        );
        assert_eq!(model["orientation"], json!("horizontal"));
        assert_eq!(model["xAxis"]["type"], json!("band"));
        assert_eq!(
            model["xAxis"]["categories"],
            json!(["category1", "category 2", "category3"])
        );
        assert_eq!(model["yAxis"]["min"], json!(10.0));
        assert_eq!(model["yAxis"]["max"], json!(150.0));
        assert_eq!(model["plots"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn xychart_plot_requires_nonempty_data() {
        let err = parse_err("xychart\nline \"t\" [ ]");
        assert!(err.contains("empty"));
        let err = parse_err("xychart\nline \"t\"");
        assert!(err.contains("missing") || err.contains("requires"));
    }

    #[test]
    fn xychart_accepts_line_without_whitespace_after_keyword() {
        let model = parse("xychart\nline[1,2,3]");
        assert_eq!(model["plots"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn xychart_acc_title_requires_colon() {
        let err = parse_err("xychart\naccTitle hello");
        assert!(err.contains("accTitle"));
    }

    #[test]
    fn xychart_rejects_invalid_x_axis_range_like_upstream() {
        let err = parse_err("xychart\nx-axis xAxisName aaa --> 33\n");
        assert!(err.contains("invalid"));
    }

    #[test]
    fn xychart_rejects_unbalanced_x_axis_brackets_like_upstream() {
        let err = parse_err("xychart\nx-axis xAxisName [ \"cat1\" [ cat2a ]\n");
        assert!(err.contains("unbalanced"));
        let err = parse_err("xychart\nx-axis xAxisName [ \"cat1\" , cat2a ] ]\n");
        assert!(err.contains("unexpected") || err.contains("unbalanced"));
    }

    #[test]
    fn xychart_rejects_invalid_y_axis_range_like_upstream() {
        let err = parse_err("xychart\ny-axis yAxisName 45.5 --> abc\n");
        assert!(err.contains("expected number") || err.contains("invalid"));
    }

    #[test]
    fn xychart_rejects_y_axis_band_data_like_upstream() {
        let err = parse_err("xychart\ny-axis yAxisName [ 45.3, 33 ]\n");
        assert!(err.contains("does not support") || err.contains("band"));
    }

    #[test]
    fn xychart_rejects_unbalanced_plot_brackets_like_upstream() {
        let err = parse_err("xychart\nline \"t\" [  +23 [ -45  , 56.6 ]\n");
        assert!(err.contains("unbalanced") || err.contains("expected"));
        let err = parse_err("xychart\nbar \"t\" [  +23 , -45  ] 56.6 ]\n");
        assert!(err.contains("unexpected") || err.contains("unbalanced"));
    }

    #[test]
    fn xychart_rejects_invalid_plot_commas_and_numbers_like_upstream() {
        let err = parse_err("xychart\nline \"t\" [  +23 ,  , -45  , 56.6 ]\n");
        assert!(err.contains("empty") || err.contains("invalid"));
        let err = parse_err("xychart\nbar \"t\" [  +23 , -4aa5  , 56.6 ]\n");
        assert!(err.contains("invalid number"));
    }
}
