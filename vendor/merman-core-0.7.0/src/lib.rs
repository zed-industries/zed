#![forbid(unsafe_code)]
//! Mermaid parser + semantic model (headless).
//!
//! Design goals:
//! - 1:1 parity with the repository's pinned upstream Mermaid baseline
//! - deterministic, testable outputs (semantic snapshot goldens)
//! - runtime-agnostic async APIs (no specific executor required)

pub mod baseline;
pub mod common;
pub mod common_db;
pub mod config;
pub mod detect;
pub mod diagram;
pub mod diagrams;
pub mod entities;
pub mod error;
mod family;
pub mod generated;
pub mod geom;
pub mod models;
pub mod preprocess;
mod runtime;
pub mod sanitize;
mod theme;
pub mod time;
pub mod utils;

pub use config::MermaidConfig;
pub use detect::{Detector, DetectorRegistry};
pub use diagram::{
    DiagramRegistry, DiagramSemanticParser, ParsedDiagram, ParsedDiagramRender,
    RenderDiagramRegistry, RenderSemanticModel, RenderSemanticParser,
};
pub use error::{Error, Result};
pub use preprocess::{PreprocessResult, preprocess_diagram, preprocess_diagram_with_known_type};

/// Maximum nested diagram/include depth accepted by recursive parsers.
pub const MAX_DIAGRAM_NESTING_DEPTH: usize = 256;

/// Returns Mermaid theme names supported by the pinned baseline.
pub fn supported_themes() -> &'static [&'static str] {
    theme::SUPPORTED_THEME_NAMES
}

/// Returns supported diagram metadata names for binding and host capability discovery.
pub fn supported_diagrams() -> &'static [&'static str] {
    family::supported_diagram_metadata_ids()
}

/// Parser behavior switches shared by metadata, semantic JSON, and typed render-model parsing.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParseOptions {
    /// Return an `error` diagram model instead of an error when diagram parsing fails.
    pub suppress_errors: bool,
}

impl ParseOptions {
    /// Strict parsing (errors are returned).
    pub fn strict() -> Self {
        Self {
            suppress_errors: false,
        }
    }

    /// Lenient parsing: on parse failures, return an `error` diagram instead of returning an error.
    pub fn lenient() -> Self {
        Self {
            suppress_errors: true,
        }
    }
}

/// Metadata extracted before semantic diagram parsing.
#[derive(Debug, Clone)]
pub struct ParseMetadata {
    /// Mermaid diagram type id selected by detection or supplied by a known-type parse entrypoint.
    pub diagram_type: String,
    /// Parsed config overrides extracted from front-matter and directives.
    /// This mirrors Mermaid's `mermaidAPI.parse()` return shape.
    pub config: MermaidConfig,
    /// The effective config used for detection/parsing after applying site defaults.
    pub effective_config: MermaidConfig,
    /// Sanitized Mermaid title from front-matter/directives, when present.
    pub title: Option<String>,
}

/// Headless Mermaid parser engine.
///
/// An engine owns detector/parser registries and a site-level Mermaid configuration. It is cheap
/// to clone when callers need per-request option variants.
#[derive(Debug, Clone)]
pub struct Engine {
    registry: DetectorRegistry,
    diagram_registry: DiagramRegistry,
    render_diagram_registry: RenderDiagramRegistry,
    site_config: MermaidConfig,
    fixed_today_local: Option<chrono::NaiveDate>,
    fixed_local_offset_minutes: Option<i32>,
}

impl Default for Engine {
    fn default() -> Self {
        let site_config = generated::default_site_config();

        Self {
            registry: DetectorRegistry::for_pinned_mermaid_baseline(),
            diagram_registry: DiagramRegistry::for_pinned_mermaid_baseline(),
            render_diagram_registry: RenderDiagramRegistry::for_pinned_mermaid_baseline(),
            site_config,
            fixed_today_local: None,
            fixed_local_offset_minutes: None,
        }
    }
}

impl Engine {
    fn parse_timing_enabled() -> bool {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| {
            matches!(
                std::env::var("MERMAN_PARSE_TIMING").as_deref(),
                Ok("1") | Ok("true")
            )
        })
    }

    /// Creates an engine using the pinned Mermaid baseline registries and default site config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the "today" value used by diagrams that depend on local time (e.g. Gantt).
    ///
    /// This exists primarily to make fixture snapshots deterministic. By default, Mermaid uses the
    /// current local date.
    pub fn with_fixed_today(mut self, today: Option<chrono::NaiveDate>) -> Self {
        self.fixed_today_local = today;
        self
    }

    /// Overrides the local timezone offset (in minutes) used by diagrams that depend on local time
    /// semantics (notably Gantt).
    ///
    /// This exists primarily to make fixture snapshots deterministic across CI runners. When
    /// `None`, the system local timezone is used.
    pub fn with_fixed_local_offset_minutes(mut self, offset_minutes: Option<i32>) -> Self {
        self.fixed_local_offset_minutes = offset_minutes;
        self
    }

    /// Applies site-level Mermaid config defaults.
    pub fn with_site_config(mut self, mut site_config: MermaidConfig) -> Self {
        // Merge overrides onto Mermaid schema defaults so detectors keep working.
        config::mirror_legacy_font_family_into_theme_variables(&mut site_config);
        self.site_config.deep_merge(site_config.as_value());
        self
    }

    /// Returns the detector registry used for automatic diagram type detection.
    pub fn registry(&self) -> &DetectorRegistry {
        &self.registry
    }

    /// Returns a mutable detector registry for custom diagram detection.
    pub fn registry_mut(&mut self) -> &mut DetectorRegistry {
        &mut self.registry
    }

    /// Returns the semantic JSON parser registry.
    pub fn diagram_registry(&self) -> &DiagramRegistry {
        &self.diagram_registry
    }

    /// Returns a mutable semantic JSON parser registry for custom diagram adapters.
    pub fn diagram_registry_mut(&mut self) -> &mut DiagramRegistry {
        &mut self.diagram_registry
    }

    /// Returns the typed render-model parser registry.
    pub fn render_diagram_registry(&self) -> &RenderDiagramRegistry {
        &self.render_diagram_registry
    }

    /// Returns a mutable typed render-model parser registry.
    pub fn render_diagram_registry_mut(&mut self) -> &mut RenderDiagramRegistry {
        &mut self.render_diagram_registry
    }

    /// Synchronous variant of [`Engine::parse_metadata`].
    ///
    /// This is useful for UI render pipelines that are synchronous (e.g. immediate-mode UI),
    /// where introducing an async executor would be awkward. The parsing work is CPU-bound and
    /// does not perform I/O.
    pub fn parse_metadata_sync(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParseMetadata>> {
        let Some((_, meta)) = self.preprocess_and_detect(text, options)? else {
            return Ok(None);
        };
        Ok(Some(meta))
    }

    /// Parses metadata for an already-known diagram type (skips type detection).
    ///
    /// This is intended for integrations that already know the diagram type, e.g. Markdown fences
    /// like ````mermaid` / ` ```flowchart` / ` ```sequenceDiagram`.
    ///
    /// ## Example (Markdown fence)
    ///
    /// ```no_run
    /// use merman_core::{Engine, ParseOptions};
    ///
    /// let engine = Engine::new();
    ///
    /// // Your markdown parser provides the fence info string (e.g. "flowchart", "sequenceDiagram").
    /// let fence = "sequenceDiagram";
    /// let diagram = r#"sequenceDiagram
    ///   Alice->>Bob: Hello
    /// "#;
    ///
    /// // Map fence info strings to merman's internal diagram ids.
    /// let diagram_type = match fence {
    ///     "sequenceDiagram" => "sequence",
    ///     "flowchart" | "graph" => "flowchart-v2",
    ///     "stateDiagram" | "stateDiagram-v2" => "stateDiagram",
    ///     other => other,
    /// };
    ///
    /// let meta = engine
    ///     .parse_metadata_with_type_sync(diagram_type, diagram, ParseOptions::strict())?
    ///     .expect("diagram detected");
    /// # Ok::<(), merman_core::Error>(())
    /// ```
    pub fn parse_metadata_with_type_sync(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParseMetadata>> {
        let Some((_, meta)) = self.preprocess_and_assume_type(diagram_type, text, options)? else {
            return Ok(None);
        };
        Ok(Some(meta))
    }

    /// Async facade for [`Engine::parse_metadata_sync`].
    ///
    /// The work is CPU-bound and executes synchronously; this method exists for callers that
    /// prefer an async-shaped API.
    pub async fn parse_metadata(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParseMetadata>> {
        self.parse_metadata_sync(text, options)
    }

    /// Async facade for [`Engine::parse_metadata_with_type_sync`].
    ///
    /// The work is CPU-bound and executes synchronously.
    pub async fn parse_metadata_with_type(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParseMetadata>> {
        self.parse_metadata_with_type_sync(diagram_type, text, options)
    }

    /// Synchronous variant of [`Engine::parse_diagram`].
    ///
    /// Note: callers that want “always returns a diagram” behavior can set
    /// [`ParseOptions::suppress_errors`] to `true` to get an `error` diagram on parse failures.
    pub fn parse_diagram_sync(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagram>> {
        let timing_enabled = Self::parse_timing_enabled();
        let total_start = timing_enabled.then(web_time::Instant::now);

        let preprocess_start = timing_enabled.then(web_time::Instant::now);
        let Some((code, meta)) = self.preprocess_and_detect(text, options)? else {
            return Ok(None);
        };
        let preprocess = preprocess_start.map(|s| s.elapsed());

        let parse_start = timing_enabled.then(web_time::Instant::now);
        let parse = crate::runtime::with_fixed_today_local(self.fixed_today_local, || {
            crate::runtime::with_fixed_local_offset_minutes(self.fixed_local_offset_minutes, || {
                diagram::parse_or_unsupported(
                    &self.diagram_registry,
                    &meta.diagram_type,
                    &code,
                    &meta,
                )
            })
        });

        let mut model = match parse {
            Ok(v) => v,
            Err(err) => {
                if !options.suppress_errors {
                    return Err(err);
                }

                if let Some(start) = total_start {
                    let parse = parse_start.map(|s| s.elapsed()).unwrap_or_default();
                    eprintln!(
                        "[parse-timing] diagram=error total={:?} preprocess={:?} parse={:?} sanitize={:?} input_bytes={}",
                        start.elapsed(),
                        preprocess.unwrap_or_default(),
                        parse,
                        web_time::Duration::default(),
                        text.len(),
                    );
                }
                return Ok(Some(
                    crate::diagrams::error_diagram::suppressed_error_diagram(&meta),
                ));
            }
        };
        let parse = parse_start.map(|s| s.elapsed());

        let sanitize_start = timing_enabled.then(web_time::Instant::now);
        common_db::apply_common_db_sanitization(&mut model, &meta.effective_config);
        let sanitize = sanitize_start.map(|s| s.elapsed());

        if let Some(start) = total_start {
            eprintln!(
                "[parse-timing] diagram={} total={:?} preprocess={:?} parse={:?} sanitize={:?} input_bytes={}",
                meta.diagram_type,
                start.elapsed(),
                preprocess.unwrap_or_default(),
                parse.unwrap_or_default(),
                sanitize.unwrap_or_default(),
                text.len(),
            );
        }
        Ok(Some(ParsedDiagram { meta, model }))
    }

    /// Async facade for [`Engine::parse_diagram_sync`].
    ///
    /// The work is CPU-bound and executes synchronously.
    pub async fn parse_diagram(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagram>> {
        self.parse_diagram_sync(text, options)
    }

    /// Parses a diagram into a typed semantic model optimized for headless layout + SVG rendering.
    ///
    /// Unlike [`Engine::parse_diagram_sync`], this avoids constructing large
    /// `serde_json::Value` object trees for high-impact typed-first diagrams and instead returns
    /// typed semantic structs that the renderer can consume directly.
    ///
    /// Callers that need the semantic JSON model should continue using
    /// [`Engine::parse_diagram_sync`].
    pub fn parse_diagram_for_render_model_sync(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagramRender>> {
        self.parse_diagram_for_render_model_inner(text, options, |engine| {
            engine.preprocess_and_detect(text, options)
        })
    }

    /// Async facade for [`Engine::parse_diagram_for_render_model_sync`].
    ///
    /// The work is CPU-bound and executes synchronously.
    pub async fn parse_diagram_for_render_model(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagramRender>> {
        self.parse_diagram_for_render_model_sync(text, options)
    }

    /// Parses a diagram into a typed semantic render model when the diagram type is already known
    /// (skips type detection).
    ///
    /// This is the preferred entrypoint for Markdown renderers and editors that already know the
    /// diagram type from the code fence info string. It avoids the detection pass and can reduce a
    /// small fixed overhead in tight render loops.
    pub fn parse_diagram_for_render_model_with_type_sync(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagramRender>> {
        self.parse_diagram_for_render_model_inner(text, options, |engine| {
            engine.preprocess_and_assume_type(diagram_type, text, options)
        })
    }

    fn parse_diagram_for_render_model_inner(
        &self,
        text: &str,
        options: ParseOptions,
        preprocess: impl FnOnce(&Self) -> Result<Option<(String, ParseMetadata)>>,
    ) -> Result<Option<ParsedDiagramRender>> {
        let timing_enabled = Self::parse_timing_enabled();
        let total_start = timing_enabled.then(web_time::Instant::now);

        let preprocess_start = timing_enabled.then(web_time::Instant::now);
        let Some((code, meta)) = preprocess(self)? else {
            return Ok(None);
        };
        let preprocess = preprocess_start.map(|s| s.elapsed());

        let parse_start = timing_enabled.then(web_time::Instant::now);
        let parse_res = crate::runtime::with_fixed_today_local(self.fixed_today_local, || {
            crate::runtime::with_fixed_local_offset_minutes(self.fixed_local_offset_minutes, || {
                self.parse_render_semantic_model(&code, &meta)
            })
        });
        let parse = parse_start.map(|s| s.elapsed());

        let mut model = match parse_res {
            Ok(v) => v,
            Err(err) => {
                if !options.suppress_errors {
                    return Err(err);
                }

                if let Some(start) = total_start {
                    eprintln!(
                        "[parse-render-timing] diagram=error model=json total={:?} preprocess={:?} parse={:?} sanitize={:?} input_bytes={}",
                        start.elapsed(),
                        preprocess.unwrap_or_default(),
                        parse.unwrap_or_default(),
                        web_time::Duration::default(),
                        text.len(),
                    );
                }
                return Ok(Some(
                    crate::diagrams::error_diagram::suppressed_error_render_diagram(&meta),
                ));
            }
        };

        let sanitize_start = timing_enabled.then(web_time::Instant::now);
        model.sanitize_common_db_fields(&meta.effective_config);
        let sanitize = sanitize_start.map(|s| s.elapsed());

        if let Some(start) = total_start {
            eprintln!(
                "[parse-render-timing] diagram={} model={} total={:?} preprocess={:?} parse={:?} sanitize={:?} input_bytes={}",
                meta.diagram_type,
                model.kind(),
                start.elapsed(),
                preprocess.unwrap_or_default(),
                parse.unwrap_or_default(),
                sanitize.unwrap_or_default(),
                text.len(),
            );
        }

        Ok(Some(ParsedDiagramRender { meta, model }))
    }

    fn parse_render_semantic_model(
        &self,
        code: &str,
        meta: &ParseMetadata,
    ) -> Result<RenderSemanticModel> {
        if let Some(parser) = self.render_diagram_registry.get(&meta.diagram_type) {
            return parser(code, meta);
        }

        if !family::permits_json_render_fallback(&meta.diagram_type) {
            return Err(Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: format!(
                    "built-in diagram type `{}` is missing a typed render parser; JSON render fallback is reserved for error and custom diagram adapters",
                    meta.diagram_type
                ),
            });
        }

        diagram::parse_or_unsupported(&self.diagram_registry, &meta.diagram_type, code, meta)
            .map(RenderSemanticModel::Json)
    }

    /// Async facade for [`Engine::parse_diagram_for_render_model_with_type_sync`].
    ///
    /// The work is CPU-bound and executes synchronously.
    pub async fn parse_diagram_for_render_model_with_type(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagramRender>> {
        self.parse_diagram_for_render_model_with_type_sync(diagram_type, text, options)
    }

    /// Parses a diagram when the diagram type is already known (skips type detection).
    ///
    /// This is the preferred entrypoint for Markdown renderers and editors that already know the
    /// diagram type from the code fence info string. It avoids the detection pass and can reduce a
    /// small fixed overhead in tight render loops.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use merman_core::{Engine, ParseOptions};
    ///
    /// let engine = Engine::new();
    /// let input = "flowchart TD; A-->B;";
    ///
    /// let parsed = engine
    ///     .parse_diagram_with_type_sync("flowchart-v2", input, ParseOptions::strict())?
    ///     .expect("diagram detected");
    ///
    /// assert_eq!(parsed.meta.diagram_type, "flowchart-v2");
    /// # Ok::<(), merman_core::Error>(())
    /// ```
    pub fn parse_diagram_with_type_sync(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagram>> {
        let Some((code, meta)) = self.preprocess_and_assume_type(diagram_type, text, options)?
        else {
            return Ok(None);
        };

        let parse = crate::runtime::with_fixed_today_local(self.fixed_today_local, || {
            crate::runtime::with_fixed_local_offset_minutes(self.fixed_local_offset_minutes, || {
                diagram::parse_or_unsupported(
                    &self.diagram_registry,
                    &meta.diagram_type,
                    &code,
                    &meta,
                )
            })
        });

        let mut model = match parse {
            Ok(v) => v,
            Err(err) => {
                if !options.suppress_errors {
                    return Err(err);
                }

                return Ok(Some(
                    crate::diagrams::error_diagram::suppressed_error_diagram(&meta),
                ));
            }
        };
        common_db::apply_common_db_sanitization(&mut model, &meta.effective_config);
        Ok(Some(ParsedDiagram { meta, model }))
    }

    /// Async facade for [`Engine::parse_diagram_with_type_sync`].
    ///
    /// The work is CPU-bound and executes synchronously.
    pub async fn parse_diagram_with_type(
        &self,
        diagram_type: &str,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<ParsedDiagram>> {
        self.parse_diagram_with_type_sync(diagram_type, text, options)
    }

    /// Backward-compatible shorthand for [`Engine::parse_metadata`].
    pub async fn parse(&self, text: &str, options: ParseOptions) -> Result<Option<ParseMetadata>> {
        self.parse_metadata(text, options).await
    }

    fn preprocess_and_detect(
        &self,
        text: &str,
        options: ParseOptions,
    ) -> Result<Option<(String, ParseMetadata)>> {
        let pre = preprocess_diagram(text, &self.registry)?;
        if pre.code.trim_start().starts_with("---") {
            return Err(Error::MalformedFrontMatter);
        }

        let mut effective_config = self.site_config.clone();
        effective_config.deep_merge(pre.config.as_value());

        let diagram_type = match self
            .registry
            .detect_type_precleaned(&pre.code, &mut effective_config)
        {
            Ok(t) => t.to_string(),
            Err(err) => {
                if options.suppress_errors {
                    return Ok(None);
                }
                return Err(err);
            }
        };
        theme::apply_theme_defaults(&mut effective_config);

        let title = pre
            .title
            .as_ref()
            .map(|t| crate::sanitize::sanitize_text(t, &effective_config))
            .filter(|t| !t.is_empty());

        Ok(Some((
            pre.code,
            ParseMetadata {
                diagram_type,
                config: pre.config,
                effective_config,
                title,
            },
        )))
    }

    fn preprocess_and_assume_type(
        &self,
        diagram_type: &str,
        text: &str,
        _options: ParseOptions,
    ) -> Result<Option<(String, ParseMetadata)>> {
        let pre = preprocess_diagram_with_known_type(text, &self.registry, Some(diagram_type))?;
        if pre.code.trim_start().starts_with("---") {
            return Err(Error::MalformedFrontMatter);
        }

        let mut effective_config = self.site_config.clone();
        effective_config.deep_merge(pre.config.as_value());
        family::apply_known_type_detector_side_effects(diagram_type, &mut effective_config);
        theme::apply_theme_defaults(&mut effective_config);

        let title = pre
            .title
            .as_ref()
            .map(|t| crate::sanitize::sanitize_text(t, &effective_config))
            .filter(|t| !t.is_empty());

        Ok(Some((
            pre.code,
            ParseMetadata {
                diagram_type: diagram_type.to_string(),
                config: pre.config,
                effective_config,
                title,
            },
        )))
    }
}

#[cfg(test)]
mod tests;
