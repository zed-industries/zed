pub use merman_ascii::{
    AsciiCharset, AsciiError, AsciiRenderOptions, AsciiRenderer, render_class, render_er,
    render_flowchart, render_model, render_sequence, render_xychart,
};
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum HeadlessAsciiError {
    #[error(transparent)]
    Parse(#[from] merman_core::Error),
    #[error(transparent)]
    Ascii(#[from] merman_ascii::AsciiError),
}

pub type Result<T> = std::result::Result<T, HeadlessAsciiError>;

/// Synchronous ASCII/Unicode render helper (executor-free).
///
/// The Mermaid source is parsed by `merman-core`; the typed render model is then rendered by
/// `merman-ascii`. Supported diagram families currently include flowchart, sequenceDiagram,
/// classDiagram, erDiagram, and xychart.
pub fn render_ascii_sync(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    ascii_options: &AsciiRenderOptions,
) -> Result<Option<String>> {
    let (ascii_options, text) = apply_mermaid_ascii_directives(ascii_options, text);
    let Some(parsed) = engine.parse_diagram_for_render_model_sync(text.as_ref(), parse_options)?
    else {
        return Ok(None);
    };

    Ok(Some(merman_ascii::render_model(
        &parsed.model,
        &ascii_options,
    )?))
}

pub async fn render_ascii(
    engine: &merman_core::Engine,
    text: &str,
    parse_options: merman_core::ParseOptions,
    ascii_options: &AsciiRenderOptions,
) -> Result<Option<String>> {
    // This async API is runtime-agnostic: rendering is CPU-bound and does not perform I/O.
    // It executes synchronously and does not yield.
    render_ascii_sync(engine, text, parse_options, ascii_options)
}

/// Convenience wrapper that bundles an [`merman_core::Engine`] and ASCII render options.
///
/// This is intended for terminal, log, documentation, and chat-surface integrations that want
/// stable text output without wiring parsing and rendering parameters on every call.
#[derive(Clone)]
pub struct HeadlessAsciiRenderer {
    pub engine: merman_core::Engine,
    pub parse: merman_core::ParseOptions,
    pub ascii: AsciiRenderOptions,
}

impl Default for HeadlessAsciiRenderer {
    fn default() -> Self {
        Self {
            engine: merman_core::Engine::new(),
            parse: merman_core::ParseOptions::default(),
            ascii: AsciiRenderOptions::default(),
        }
    }
}

impl HeadlessAsciiRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_site_config(mut self, site_config: merman_core::MermaidConfig) -> Self {
        self.engine = self.engine.with_site_config(site_config);
        self
    }

    pub fn with_fixed_today(mut self, today: Option<chrono::NaiveDate>) -> Self {
        self.engine = self.engine.with_fixed_today(today);
        self
    }

    pub fn with_fixed_local_offset_minutes(mut self, offset_minutes: Option<i32>) -> Self {
        self.engine = self.engine.with_fixed_local_offset_minutes(offset_minutes);
        self
    }

    pub fn with_parse_options(mut self, parse: merman_core::ParseOptions) -> Self {
        self.parse = parse;
        self
    }

    pub fn with_strict_parsing(self) -> Self {
        self.with_parse_options(merman_core::ParseOptions::strict())
    }

    pub fn with_lenient_parsing(self) -> Self {
        self.with_parse_options(merman_core::ParseOptions::lenient())
    }

    pub fn with_ascii_options(mut self, ascii: AsciiRenderOptions) -> Self {
        self.ascii = ascii;
        self
    }

    pub fn with_charset(mut self, charset: AsciiCharset) -> Self {
        self.ascii.charset = charset;
        self
    }

    pub fn parse_metadata_sync(&self, text: &str) -> Result<Option<merman_core::ParseMetadata>> {
        Ok(self.engine.parse_metadata_sync(text, self.parse)?)
    }

    pub fn parse_diagram_sync(&self, text: &str) -> Result<Option<merman_core::ParsedDiagram>> {
        Ok(self.engine.parse_diagram_sync(text, self.parse)?)
    }

    pub fn render_model(
        &self,
        model: &merman_core::diagram::RenderSemanticModel,
    ) -> Result<String> {
        Ok(merman_ascii::render_model(model, &self.ascii)?)
    }

    pub fn render_ascii_sync(&self, text: &str) -> Result<Option<String>> {
        render_ascii_sync(&self.engine, text, self.parse, &self.ascii)
    }

    pub async fn render_ascii(&self, text: &str) -> Result<Option<String>> {
        render_ascii_sync(&self.engine, text, self.parse, &self.ascii)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaddingAxis {
    X,
    Y,
}

fn apply_mermaid_ascii_directives<'a>(
    ascii_options: &AsciiRenderOptions,
    source: &'a str,
) -> (AsciiRenderOptions, Cow<'a, str>) {
    let mut options = *ascii_options;
    let mut changed = false;
    let mut output = String::new();
    let mut before_diagram = true;

    for line in source.lines() {
        let trimmed = line.trim();
        if before_diagram {
            if let Some((axis, value)) = parse_padding_directive(trimmed) {
                match axis {
                    PaddingAxis::X => options.graph_padding_x = value,
                    PaddingAxis::Y => options.graph_padding_y = value,
                }
                changed = true;
                continue;
            }
            if is_diagram_header(trimmed) {
                before_diagram = false;
            }
        }
        output.push_str(line);
        output.push('\n');
    }

    if changed {
        (options, Cow::Owned(output))
    } else {
        (options, Cow::Borrowed(source))
    }
}

#[cfg(test)]
mod headless_ascii_renderer_tests {
    use super::*;
    use serde_json::Value;

    fn task_by_id<'a>(model: &'a Value, id: &str) -> &'a Value {
        model["tasks"]
            .as_array()
            .expect("Gantt tasks should be an array")
            .iter()
            .find(|task| task["id"].as_str() == Some(id))
            .unwrap_or_else(|| panic!("missing Gantt task {id} in {model}"))
    }

    #[test]
    fn headless_ascii_renderer_fixed_time_controls_semantic_parse() {
        let renderer = HeadlessAsciiRenderer::new()
            .with_fixed_today(Some(
                chrono::NaiveDate::from_ymd_opt(2026, 2, 15).expect("valid fixed today"),
            ))
            .with_fixed_local_offset_minutes(Some(0));
        let parsed = renderer
            .parse_diagram_sync(
                r#"gantt
dateFormat MM-DD
section Demo
Missing year: id1,03-01,1d
Missing ref: id2,after missing,1d
"#,
            )
            .unwrap()
            .unwrap();

        assert_eq!(
            task_by_id(&parsed.model, "id1")["startTime"].as_i64(),
            Some(1_772_323_200_000)
        );
        assert_eq!(
            task_by_id(&parsed.model, "id2")["startTime"].as_i64(),
            Some(1_771_113_600_000)
        );
    }
}

fn parse_padding_directive(line: &str) -> Option<(PaddingAxis, usize)> {
    let (key, value) = line.split_once('=')?;
    let axis = if key.trim().eq_ignore_ascii_case("paddingX") {
        PaddingAxis::X
    } else if key.trim().eq_ignore_ascii_case("paddingY") {
        PaddingAxis::Y
    } else {
        return None;
    };
    let value = value.trim().parse().ok()?;
    Some((axis, value))
}

fn is_diagram_header(line: &str) -> bool {
    line.starts_with("graph ") || line.starts_with("flowchart ")
}
