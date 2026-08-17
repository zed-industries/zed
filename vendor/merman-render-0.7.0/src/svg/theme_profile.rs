use merman_core::MermaidConfig;
use serde_json::{Map, Value};
use std::sync::OnceLock;

use super::pipeline::{
    CssOverridePolicy, CssOverridePostprocessor, DropNativeDuplicateFallbacksPostprocessor,
    GitGraphBranchLabelBaselinePostprocessor, RootBackgroundPostprocessor,
    SanitizeCssPostprocessor, ScopedCssPostprocessor, SvgPipeline, SvgPipelinePreset,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostThemeAppearance {
    #[default]
    Light,
    Dark,
}

impl HostThemeAppearance {
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostThemePreset {
    /// Neutral light editor preview palette.
    #[default]
    EditorLight,
    /// Neutral dark editor preview palette.
    EditorDark,
    /// One Dark-inspired editor preview palette.
    OneDark,
    /// Gruvbox light-inspired editor preview palette.
    GruvboxLight,
    /// Gruvbox dark-inspired editor preview palette.
    GruvboxDark,
    /// Ayu light-inspired editor preview palette.
    AyuLight,
    /// Ayu dark-inspired editor preview palette.
    AyuDark,
}

impl HostThemePreset {
    /// All built-in host profile presets.
    pub const ALL: [Self; 7] = [
        Self::EditorLight,
        Self::EditorDark,
        Self::OneDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::AyuLight,
        Self::AyuDark,
    ];

    /// Stable `host_theme.preset` value accepted by bindings.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorLight => "editor-light",
            Self::EditorDark => "editor-dark",
            Self::OneDark => "one-dark",
            Self::GruvboxLight => "gruvbox-light",
            Self::GruvboxDark => "gruvbox-dark",
            Self::AyuLight => "ayu-light",
            Self::AyuDark => "ayu-dark",
        }
    }
}

/// Returns built-in host/editor theme preset names.
///
/// These are semantic host presets such as `one-dark` and are intentionally separate from Mermaid
/// core theme names returned by `merman_core::supported_themes()`.
pub fn supported_host_theme_presets() -> &'static [&'static str] {
    static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
    NAMES
        .get_or_init(|| {
            HostThemePreset::ALL
                .iter()
                .copied()
                .map(HostThemePreset::as_str)
                .collect()
        })
        .as_slice()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostThemeRoles {
    pub canvas: Option<String>,
    pub surface: Option<String>,
    pub surface_alt: Option<String>,
    pub surface_muted: Option<String>,
    pub text: Option<String>,
    pub subtle_text: Option<String>,
    pub border: Option<String>,
    pub line: Option<String>,
    pub edge_label_background: Option<String>,
    pub cluster_background: Option<String>,
    pub cluster_border: Option<String>,
    pub note_background: Option<String>,
    pub note_border: Option<String>,
    pub note_text: Option<String>,
    pub actor_background: Option<String>,
    pub actor_border: Option<String>,
    pub actor_text: Option<String>,
    pub activation_background: Option<String>,
    pub activation_border: Option<String>,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub success: Option<String>,
}

impl HostThemeRoles {
    fn has_values(&self) -> bool {
        self.canvas.is_some()
            || self.surface.is_some()
            || self.surface_alt.is_some()
            || self.surface_muted.is_some()
            || self.text.is_some()
            || self.subtle_text.is_some()
            || self.border.is_some()
            || self.line.is_some()
            || self.edge_label_background.is_some()
            || self.cluster_background.is_some()
            || self.cluster_border.is_some()
            || self.note_background.is_some()
            || self.note_border.is_some()
            || self.note_text.is_some()
            || self.actor_background.is_some()
            || self.actor_border.is_some()
            || self.actor_text.is_some()
            || self.activation_background.is_some()
            || self.activation_border.is_some()
            || self.error.is_some()
            || self.warning.is_some()
            || self.success.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostThemePipelinePreset {
    /// Keep Mermaid-parity SVG output.
    #[default]
    Parity,
    /// Keep native `<foreignObject>` labels and add readable SVG text fallbacks.
    ///
    /// This is useful for consumers that need both browser-like SVG and non-HTML label fallbacks.
    /// For browser/editor display surfaces, prefer [`Self::ResvgSafe`] if duplicate labels are a
    /// risk.
    Readable,
    /// Add readable fallback text, remove native `<foreignObject>` labels, and sanitize common
    /// rasterization hazards.
    ResvgSafe,
}

impl From<HostThemePipelinePreset> for SvgPipelinePreset {
    fn from(value: HostThemePipelinePreset) -> Self {
        match value {
            HostThemePipelinePreset::Parity => Self::Parity,
            HostThemePipelinePreset::Readable => Self::Readable,
            HostThemePipelinePreset::ResvgSafe => Self::ResvgSafe,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HostThemeRootBackground {
    #[default]
    None,
    Canvas,
    Color(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostThemeOutput {
    pub pipeline: HostThemePipelinePreset,
    pub css_override_policy: CssOverridePolicy,
    pub root_background: HostThemeRootBackground,
    pub drop_native_duplicate_fallbacks: bool,
    pub scoped_css: Option<String>,
}

impl Default for HostThemeOutput {
    fn default() -> Self {
        Self {
            pipeline: HostThemePipelinePreset::Parity,
            css_override_policy: CssOverridePolicy::Preserve,
            root_background: HostThemeRootBackground::None,
            drop_native_duplicate_fallbacks: false,
            scoped_css: None,
        }
    }
}

impl HostThemeOutput {
    /// Returns product-neutral defaults for editor previews and raster-oriented host surfaces.
    ///
    /// The preset selects `resvg-safe` output, strips existing `!important` CSS so host theme rules
    /// can win predictably, uses the profile canvas as the root SVG background, and enables
    /// duplicate fallback cleanup. Callers can still add scoped CSS or override individual fields.
    pub fn resvg_safe_editor() -> Self {
        Self {
            pipeline: HostThemePipelinePreset::ResvgSafe,
            css_override_policy: CssOverridePolicy::StripExistingImportant,
            root_background: HostThemeRootBackground::Canvas,
            drop_native_duplicate_fallbacks: true,
            scoped_css: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostThemeProfile {
    pub appearance: HostThemeAppearance,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
    pub roles: HostThemeRoles,
    pub series_palette: Vec<String>,
    pub output: HostThemeOutput,
    pub theme_variables: Map<String, Value>,
    pub site_config: Map<String, Value>,
}

impl Default for HostThemeProfile {
    fn default() -> Self {
        Self {
            appearance: HostThemeAppearance::Light,
            font_family: None,
            font_size: None,
            roles: HostThemeRoles::default(),
            series_palette: Vec::new(),
            output: HostThemeOutput::default(),
            theme_variables: Map::new(),
            site_config: Map::new(),
        }
    }
}

impl HostThemeProfile {
    pub fn builder() -> HostThemeProfileBuilder {
        HostThemeProfileBuilder::default()
    }

    pub fn from_preset(preset: HostThemePreset) -> Self {
        match preset {
            HostThemePreset::EditorLight => Self::editor_light(),
            HostThemePreset::EditorDark => Self::editor_dark(),
            HostThemePreset::OneDark => Self::one_dark(),
            HostThemePreset::GruvboxLight => Self::gruvbox_light(),
            HostThemePreset::GruvboxDark => Self::gruvbox_dark(),
            HostThemePreset::AyuLight => Self::ayu_light(),
            HostThemePreset::AyuDark => Self::ayu_dark(),
        }
    }

    pub fn editor_light() -> Self {
        Self {
            appearance: HostThemeAppearance::Light,
            font_family: Some(
                r#"Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif"#
                    .to_string(),
            ),
            font_size: Some("14px".to_string()),
            roles: HostThemeRoles {
                canvas: Some("#ffffff".to_string()),
                surface: Some("#f8fafc".to_string()),
                surface_alt: Some("#e2e8f0".to_string()),
                surface_muted: Some("#f1f5f9".to_string()),
                text: Some("#0f172a".to_string()),
                subtle_text: Some("#475569".to_string()),
                border: Some("#94a3b8".to_string()),
                line: Some("#64748b".to_string()),
                edge_label_background: Some("#ffffff".to_string()),
                cluster_background: Some("#f1f5f9".to_string()),
                cluster_border: Some("#cbd5e1".to_string()),
                note_background: Some("#fff7ed".to_string()),
                note_border: Some("#fdba74".to_string()),
                note_text: Some("#7c2d12".to_string()),
                actor_background: Some("#f8fafc".to_string()),
                actor_border: Some("#94a3b8".to_string()),
                actor_text: Some("#0f172a".to_string()),
                activation_background: Some("#e2e8f0".to_string()),
                activation_border: Some("#94a3b8".to_string()),
                error: Some("#dc2626".to_string()),
                warning: Some("#d97706".to_string()),
                success: Some("#059669".to_string()),
            },
            series_palette: vec![
                "#2563eb".to_string(),
                "#059669".to_string(),
                "#d97706".to_string(),
                "#7c3aed".to_string(),
                "#0891b2".to_string(),
                "#be123c".to_string(),
                "#a16207".to_string(),
                "#65a30d".to_string(),
            ],
            output: HostThemeOutput::resvg_safe_editor(),
            theme_variables: Map::new(),
            site_config: Map::new(),
        }
    }

    pub fn editor_dark() -> Self {
        Self {
            appearance: HostThemeAppearance::Dark,
            font_family: Some(
                r#"Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif"#
                    .to_string(),
            ),
            font_size: Some("14px".to_string()),
            roles: HostThemeRoles {
                canvas: Some("#0f172a".to_string()),
                surface: Some("#111827".to_string()),
                surface_alt: Some("#1f2937".to_string()),
                surface_muted: Some("#334155".to_string()),
                text: Some("#e5e7eb".to_string()),
                subtle_text: Some("#cbd5e1".to_string()),
                border: Some("#475569".to_string()),
                line: Some("#94a3b8".to_string()),
                edge_label_background: Some("#0f172a".to_string()),
                cluster_background: Some("#1e293b".to_string()),
                cluster_border: Some("#475569".to_string()),
                note_background: Some("#422006".to_string()),
                note_border: Some("#f59e0b".to_string()),
                note_text: Some("#fef3c7".to_string()),
                actor_background: Some("#1f2937".to_string()),
                actor_border: Some("#475569".to_string()),
                actor_text: Some("#e5e7eb".to_string()),
                activation_background: Some("#334155".to_string()),
                activation_border: Some("#64748b".to_string()),
                error: Some("#f87171".to_string()),
                warning: Some("#fbbf24".to_string()),
                success: Some("#34d399".to_string()),
            },
            series_palette: vec![
                "#60a5fa".to_string(),
                "#34d399".to_string(),
                "#f59e0b".to_string(),
                "#c084fc".to_string(),
                "#22d3ee".to_string(),
                "#fb7185".to_string(),
                "#facc15".to_string(),
                "#a3e635".to_string(),
            ],
            output: HostThemeOutput::resvg_safe_editor(),
            theme_variables: Map::new(),
            site_config: Map::new(),
        }
    }

    pub fn one_dark() -> Self {
        Self::editor_profile(
            HostThemeAppearance::Dark,
            HostThemeRoles {
                canvas: Some("#282c34".to_string()),
                surface: Some("#21252b".to_string()),
                surface_alt: Some("#2c313a".to_string()),
                surface_muted: Some("#3e4451".to_string()),
                text: Some("#abb2bf".to_string()),
                subtle_text: Some("#828997".to_string()),
                border: Some("#3e4451".to_string()),
                line: Some("#61afef".to_string()),
                edge_label_background: Some("#282c34".to_string()),
                cluster_background: Some("#2c313a".to_string()),
                cluster_border: Some("#3e4451".to_string()),
                note_background: Some("#3a2f1b".to_string()),
                note_border: Some("#e5c07b".to_string()),
                note_text: Some("#f0dca4".to_string()),
                actor_background: Some("#2c313a".to_string()),
                actor_border: Some("#3e4451".to_string()),
                actor_text: Some("#abb2bf".to_string()),
                activation_background: Some("#3e4451".to_string()),
                activation_border: Some("#5c6370".to_string()),
                error: Some("#e06c75".to_string()),
                warning: Some("#e5c07b".to_string()),
                success: Some("#98c379".to_string()),
            },
            [
                "#61afef", "#98c379", "#e5c07b", "#c678dd", "#56b6c2", "#e06c75", "#d19a66",
                "#be5046",
            ],
            HostThemeOutput::resvg_safe_editor(),
        )
    }

    pub fn gruvbox_light() -> Self {
        Self::editor_profile(
            HostThemeAppearance::Light,
            HostThemeRoles {
                canvas: Some("#fbf1c7".to_string()),
                surface: Some("#f2e5bc".to_string()),
                surface_alt: Some("#ebdbb2".to_string()),
                surface_muted: Some("#d5c4a1".to_string()),
                text: Some("#3c3836".to_string()),
                subtle_text: Some("#665c54".to_string()),
                border: Some("#d5c4a1".to_string()),
                line: Some("#7c6f64".to_string()),
                edge_label_background: Some("#fbf1c7".to_string()),
                cluster_background: Some("#ebdbb2".to_string()),
                cluster_border: Some("#d5c4a1".to_string()),
                note_background: Some("#f2e5bc".to_string()),
                note_border: Some("#d79921".to_string()),
                note_text: Some("#3c3836".to_string()),
                actor_background: Some("#ebdbb2".to_string()),
                actor_border: Some("#d5c4a1".to_string()),
                actor_text: Some("#3c3836".to_string()),
                activation_background: Some("#d5c4a1".to_string()),
                activation_border: Some("#bdae93".to_string()),
                error: Some("#cc241d".to_string()),
                warning: Some("#d79921".to_string()),
                success: Some("#98971a".to_string()),
            },
            [
                "#458588", "#98971a", "#d79921", "#b16286", "#689d6a", "#cc241d", "#d65d0e",
                "#427b58",
            ],
            HostThemeOutput::resvg_safe_editor(),
        )
    }

    pub fn gruvbox_dark() -> Self {
        Self::editor_profile(
            HostThemeAppearance::Dark,
            HostThemeRoles {
                canvas: Some("#282828".to_string()),
                surface: Some("#3c3836".to_string()),
                surface_alt: Some("#504945".to_string()),
                surface_muted: Some("#665c54".to_string()),
                text: Some("#ebdbb2".to_string()),
                subtle_text: Some("#d5c4a1".to_string()),
                border: Some("#665c54".to_string()),
                line: Some("#d5c4a1".to_string()),
                edge_label_background: Some("#282828".to_string()),
                cluster_background: Some("#3c3836".to_string()),
                cluster_border: Some("#665c54".to_string()),
                note_background: Some("#3c3836".to_string()),
                note_border: Some("#fabd2f".to_string()),
                note_text: Some("#fbf1c7".to_string()),
                actor_background: Some("#3c3836".to_string()),
                actor_border: Some("#665c54".to_string()),
                actor_text: Some("#ebdbb2".to_string()),
                activation_background: Some("#504945".to_string()),
                activation_border: Some("#7c6f64".to_string()),
                error: Some("#fb4934".to_string()),
                warning: Some("#fabd2f".to_string()),
                success: Some("#b8bb26".to_string()),
            },
            [
                "#83a598", "#b8bb26", "#fabd2f", "#d3869b", "#8ec07c", "#fb4934", "#fe8019",
                "#689d6a",
            ],
            HostThemeOutput::resvg_safe_editor(),
        )
    }

    pub fn ayu_light() -> Self {
        Self::editor_profile(
            HostThemeAppearance::Light,
            HostThemeRoles {
                canvas: Some("#fafafa".to_string()),
                surface: Some("#f3f4f5".to_string()),
                surface_alt: Some("#e6e8eb".to_string()),
                surface_muted: Some("#d9d7ce".to_string()),
                text: Some("#5c6773".to_string()),
                subtle_text: Some("#8a9199".to_string()),
                border: Some("#d9d7ce".to_string()),
                line: Some("#55b4d4".to_string()),
                edge_label_background: Some("#fafafa".to_string()),
                cluster_background: Some("#f3f4f5".to_string()),
                cluster_border: Some("#d9d7ce".to_string()),
                note_background: Some("#fff3bf".to_string()),
                note_border: Some("#ffaa33".to_string()),
                note_text: Some("#5c6773".to_string()),
                actor_background: Some("#f3f4f5".to_string()),
                actor_border: Some("#d9d7ce".to_string()),
                actor_text: Some("#5c6773".to_string()),
                activation_background: Some("#e6e8eb".to_string()),
                activation_border: Some("#d9d7ce".to_string()),
                error: Some("#f07171".to_string()),
                warning: Some("#ffaa33".to_string()),
                success: Some("#86b300".to_string()),
            },
            [
                "#55b4d4", "#86b300", "#ffaa33", "#a37acc", "#4cbf99", "#f07171", "#f2ae49",
                "#399ee6",
            ],
            HostThemeOutput::resvg_safe_editor(),
        )
    }

    pub fn ayu_dark() -> Self {
        Self::editor_profile(
            HostThemeAppearance::Dark,
            HostThemeRoles {
                canvas: Some("#0b0e14".to_string()),
                surface: Some("#11151c".to_string()),
                surface_alt: Some("#1f2430".to_string()),
                surface_muted: Some("#343b48".to_string()),
                text: Some("#bfbdb6".to_string()),
                subtle_text: Some("#8a9199".to_string()),
                border: Some("#343b48".to_string()),
                line: Some("#59c2ff".to_string()),
                edge_label_background: Some("#0b0e14".to_string()),
                cluster_background: Some("#1f2430".to_string()),
                cluster_border: Some("#343b48".to_string()),
                note_background: Some("#332a14".to_string()),
                note_border: Some("#ffb454".to_string()),
                note_text: Some("#ffdf99".to_string()),
                actor_background: Some("#1f2430".to_string()),
                actor_border: Some("#343b48".to_string()),
                actor_text: Some("#bfbdb6".to_string()),
                activation_background: Some("#343b48".to_string()),
                activation_border: Some("#4f5866".to_string()),
                error: Some("#f07178".to_string()),
                warning: Some("#ffb454".to_string()),
                success: Some("#aad94c".to_string()),
            },
            [
                "#59c2ff", "#aad94c", "#ffb454", "#d2a6ff", "#95e6cb", "#f07178", "#f29668",
                "#39bae6",
            ],
            HostThemeOutput::resvg_safe_editor(),
        )
    }

    fn editor_profile<const N: usize>(
        appearance: HostThemeAppearance,
        roles: HostThemeRoles,
        palette: [&str; N],
        output: HostThemeOutput,
    ) -> Self {
        Self {
            appearance,
            font_family: Some(
                r#"Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif"#
                    .to_string(),
            ),
            font_size: Some("14px".to_string()),
            roles,
            series_palette: palette.iter().map(|color| color.to_string()).collect(),
            output,
            theme_variables: Map::new(),
            site_config: Map::new(),
        }
    }

    pub fn compile(&self) -> CompiledHostTheme {
        let mut root = Map::new();

        let mut theme_variables = Map::new();
        let has_profile_theme_input = self.appearance.is_dark()
            || self.font_family.is_some()
            || self.font_size.is_some()
            || self.roles.has_values()
            || !self.series_palette.is_empty()
            || !self.theme_variables.is_empty();

        if has_profile_theme_input {
            root.insert("theme".to_string(), Value::String("base".to_string()));
            root.insert(
                "darkMode".to_string(),
                Value::Bool(self.appearance.is_dark()),
            );
            theme_variables.insert(
                "darkMode".to_string(),
                Value::Bool(self.appearance.is_dark()),
            );
        }

        if let Some(font_family) = self.font_family.as_deref().filter(|s| !s.trim().is_empty()) {
            root.insert(
                "fontFamily".to_string(),
                Value::String(font_family.trim().to_string()),
            );
            put_str(&mut theme_variables, "fontFamily", font_family);
        }
        if let Some(font_size) = self.font_size.as_deref().filter(|s| !s.trim().is_empty()) {
            put_str(&mut theme_variables, "fontSize", font_size);
        }

        put_theme_roles(&mut theme_variables, &self.roles);
        put_series_palette(&mut theme_variables, &self.series_palette);
        put_diagram_config(
            &mut root,
            &mut theme_variables,
            &self.roles,
            &self.series_palette,
        );

        merge_object(&mut theme_variables, &self.theme_variables);
        if !theme_variables.is_empty() {
            root.insert("themeVariables".to_string(), Value::Object(theme_variables));
        }
        merge_object(&mut root, &self.site_config);

        let canvas_color = self
            .roles
            .canvas
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .map(str::trim)
            .map(str::to_string);

        CompiledHostTheme {
            site_config: MermaidConfig::from_value(Value::Object(root)),
            output: CompiledHostThemeOutput {
                preset: self.output.pipeline.into(),
                css_override_policy: self.output.css_override_policy,
                root_background_color: match &self.output.root_background {
                    HostThemeRootBackground::None => None,
                    HostThemeRootBackground::Canvas => canvas_color,
                    HostThemeRootBackground::Color(color) => Some(color.clone()),
                },
                drop_native_duplicate_fallbacks: self.output.drop_native_duplicate_fallbacks,
                scoped_css: self.output.scoped_css.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HostThemeProfileBuilder {
    profile: HostThemeProfile,
}

impl HostThemeProfileBuilder {
    pub fn appearance(mut self, appearance: HostThemeAppearance) -> Self {
        self.profile.appearance = appearance;
        self
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.profile.font_family = Some(font_family.into());
        self
    }

    pub fn font_size(mut self, font_size: impl Into<String>) -> Self {
        self.profile.font_size = Some(font_size.into());
        self
    }

    pub fn roles(mut self, roles: HostThemeRoles) -> Self {
        self.profile.roles = roles;
        self
    }

    pub fn series_palette(mut self, palette: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.profile.series_palette = palette.into_iter().map(Into::into).collect();
        self
    }

    pub fn output(mut self, output: HostThemeOutput) -> Self {
        self.profile.output = output;
        self
    }

    pub fn theme_variable(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.profile
            .theme_variables
            .insert(key.into(), value.into());
        self
    }

    pub fn site_config(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.profile.site_config.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> HostThemeProfile {
        self.profile
    }
}

#[derive(Debug, Clone)]
pub struct CompiledHostTheme {
    pub site_config: MermaidConfig,
    pub output: CompiledHostThemeOutput,
}

impl CompiledHostTheme {
    pub fn into_parts(self) -> (MermaidConfig, CompiledHostThemeOutput) {
        (self.site_config, self.output)
    }

    pub fn pipeline(&self) -> SvgPipeline {
        self.output.pipeline()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledHostThemeOutput {
    pub preset: SvgPipelinePreset,
    pub css_override_policy: CssOverridePolicy,
    pub root_background_color: Option<String>,
    pub drop_native_duplicate_fallbacks: bool,
    pub scoped_css: Option<String>,
}

impl CompiledHostThemeOutput {
    pub fn pipeline(&self) -> SvgPipeline {
        let mut pipeline = SvgPipeline::from_preset(self.preset);

        if matches!(
            self.css_override_policy,
            CssOverridePolicy::StripExistingImportant
        ) {
            pipeline.push_postprocessor(CssOverridePostprocessor::strip_existing_important());
        }

        if self.drop_native_duplicate_fallbacks {
            pipeline.push_postprocessor(DropNativeDuplicateFallbacksPostprocessor);
        }

        if matches!(self.preset, SvgPipelinePreset::ResvgSafe) {
            pipeline.push_postprocessor(GitGraphBranchLabelBaselinePostprocessor);
        }

        if let Some(color) = self
            .root_background_color
            .as_deref()
            .filter(|color| !color.trim().is_empty())
        {
            pipeline.push_postprocessor(RootBackgroundPostprocessor::new(color.trim()));
        }

        if let Some(css) = self
            .scoped_css
            .as_deref()
            .filter(|css| !css.trim().is_empty())
        {
            pipeline.push_postprocessor(
                ScopedCssPostprocessor::new(css.to_string())
                    .with_override_policy(self.css_override_policy),
            );
            if matches!(self.preset, SvgPipelinePreset::ResvgSafe) {
                pipeline.push_postprocessor(SanitizeCssPostprocessor);
            }
        }

        pipeline
    }
}

fn put_theme_roles(theme_variables: &mut Map<String, Value>, roles: &HostThemeRoles) {
    let canvas = roles.canvas.as_deref();
    let surface = roles.surface.as_deref();
    let surface_alt = roles.surface_alt.as_deref().or(surface);
    let surface_muted = roles.surface_muted.as_deref().or(surface_alt);
    let text = roles.text.as_deref();
    let subtle_text = roles.subtle_text.as_deref().or(text);
    let border = roles.border.as_deref();
    let line = roles.line.as_deref().or(border);
    let error = roles.error.as_deref();
    let warning = roles.warning.as_deref();
    let success = roles.success.as_deref();

    put_opt(theme_variables, "background", canvas);
    put_opt(theme_variables, "primaryColor", surface);
    put_opt(theme_variables, "mainBkg", surface);
    put_opt(theme_variables, "secondaryColor", surface_alt);
    put_opt(theme_variables, "tertiaryColor", surface_muted);
    put_opt(theme_variables, "primaryTextColor", text);
    put_opt(theme_variables, "nodeTextColor", text);
    put_opt(theme_variables, "textColor", text);
    put_opt(theme_variables, "titleColor", text);
    put_opt(theme_variables, "secondaryTextColor", subtle_text);
    put_opt(theme_variables, "tertiaryTextColor", subtle_text);
    put_opt(theme_variables, "primaryBorderColor", border);
    put_opt(theme_variables, "nodeBorder", border);
    put_opt(theme_variables, "lineColor", line);
    put_opt(theme_variables, "arrowheadColor", line);
    put_opt(
        theme_variables,
        "edgeLabelBackground",
        roles.edge_label_background.as_deref().or(canvas),
    );

    put_opt(
        theme_variables,
        "clusterBkg",
        roles.cluster_background.as_deref().or(surface_alt),
    );
    put_opt(
        theme_variables,
        "clusterBorder",
        roles.cluster_border.as_deref().or(border),
    );

    put_opt(
        theme_variables,
        "noteBkgColor",
        roles.note_background.as_deref().or(surface_alt),
    );
    put_opt(
        theme_variables,
        "noteBorderColor",
        roles.note_border.as_deref().or(border),
    );
    put_opt(
        theme_variables,
        "noteTextColor",
        roles.note_text.as_deref().or(text),
    );

    put_opt(
        theme_variables,
        "actorBkg",
        roles.actor_background.as_deref().or(surface_alt),
    );
    put_opt(
        theme_variables,
        "actorBorder",
        roles.actor_border.as_deref().or(border),
    );
    put_opt(
        theme_variables,
        "actorTextColor",
        roles.actor_text.as_deref().or(text),
    );
    put_opt(theme_variables, "actorLineColor", line);
    put_opt(theme_variables, "signalColor", line.or(text));
    put_opt(theme_variables, "signalTextColor", text);
    put_opt(theme_variables, "labelTextColor", text);
    put_opt(theme_variables, "loopTextColor", text);
    put_opt(theme_variables, "labelBoxBkgColor", surface_alt);
    put_opt(theme_variables, "labelBoxBorderColor", border);
    put_opt(
        theme_variables,
        "activationBkgColor",
        roles.activation_background.as_deref().or(surface_muted),
    );
    put_opt(
        theme_variables,
        "activationBorderColor",
        roles.activation_border.as_deref().or(border),
    );

    put_opt(theme_variables, "classText", text);
    put_opt(theme_variables, "labelColor", text);
    put_opt(theme_variables, "transitionColor", line);
    put_opt(theme_variables, "transitionLabelColor", text);
    put_opt(theme_variables, "stateLabelColor", text);
    put_opt(theme_variables, "stateBkg", surface);
    put_opt(theme_variables, "stateBorder", border);
    put_opt(theme_variables, "specialStateColor", line);
    put_opt(theme_variables, "compositeBackground", canvas.or(surface));

    put_opt(theme_variables, "attributeBackgroundColorOdd", surface);
    put_opt(theme_variables, "attributeBackgroundColorEven", surface_alt);
    put_opt(theme_variables, "rowOdd", surface);
    put_opt(theme_variables, "rowEven", surface_alt);

    put_opt(theme_variables, "requirementBackground", surface);
    put_opt(theme_variables, "requirementBorderColor", border);
    put_opt(theme_variables, "requirementTextColor", text);
    put_opt(theme_variables, "relationColor", line);
    put_opt(
        theme_variables,
        "relationLabelBackground",
        roles.edge_label_background.as_deref().or(canvas),
    );
    put_opt(theme_variables, "relationLabelColor", text);
    put_opt(
        theme_variables,
        "requirementEdgeLabelBackground",
        roles.edge_label_background.as_deref().or(canvas),
    );

    put_opt(theme_variables, "pieTitleTextColor", text);
    put_opt(theme_variables, "pieSectionTextColor", text);
    put_opt(theme_variables, "pieLegendTextColor", subtle_text);
    put_opt(theme_variables, "pieStrokeColor", border);
    put_opt(theme_variables, "pieOuterStrokeColor", border);

    put_opt(theme_variables, "commitLabelColor", text);
    put_opt(
        theme_variables,
        "commitLabelBackground",
        roles.edge_label_background.as_deref().or(surface),
    );
    put_opt(theme_variables, "commitLineColor", line);
    put_opt(theme_variables, "tagLabelColor", text);
    put_opt(theme_variables, "tagLabelBackground", surface);
    put_opt(theme_variables, "tagLabelBorder", border);

    put_opt(theme_variables, "quadrant1Fill", surface);
    put_opt(theme_variables, "quadrant2Fill", surface_alt);
    put_opt(theme_variables, "quadrant3Fill", canvas.or(surface));
    put_opt(theme_variables, "quadrant4Fill", surface_muted);
    put_opt(theme_variables, "quadrant1TextFill", text);
    put_opt(theme_variables, "quadrant2TextFill", text);
    put_opt(theme_variables, "quadrant3TextFill", text);
    put_opt(theme_variables, "quadrant4TextFill", text);
    put_opt(theme_variables, "quadrantPointFill", line);
    put_opt(theme_variables, "quadrantPointTextFill", text);
    put_opt(theme_variables, "quadrantTitleFill", text);
    put_opt(theme_variables, "quadrantXAxisTextFill", subtle_text);
    put_opt(theme_variables, "quadrantYAxisTextFill", subtle_text);
    put_opt(theme_variables, "quadrantExternalBorderStrokeFill", border);
    put_opt(theme_variables, "quadrantInternalBorderStrokeFill", border);

    put_opt(theme_variables, "archEdgeColor", line);
    put_opt(theme_variables, "archEdgeArrowColor", line);
    put_opt(
        theme_variables,
        "archGroupBorderColor",
        roles.cluster_border.as_deref().or(border),
    );

    put_opt(theme_variables, "emUiFill", surface);
    put_opt(theme_variables, "emUiStroke", border);
    put_opt(theme_variables, "emRelationStroke", line);
    put_opt(theme_variables, "emArrowhead", line);
    put_opt(
        theme_variables,
        "emSwimlaneBackgroundOdd",
        roles.cluster_background.as_deref().or(surface_muted),
    );
    put_opt(
        theme_variables,
        "emSwimlaneBackgroundStroke",
        roles.cluster_border.as_deref().or(border),
    );

    put_opt(theme_variables, "taskTextDarkColor", text);
    put_opt(theme_variables, "taskTextClickableColor", line);
    put_opt(theme_variables, "taskTextColor", text);
    put_opt(theme_variables, "taskTextOutsideColor", subtle_text);
    put_opt(theme_variables, "taskBkgColor", surface);
    put_opt(theme_variables, "taskBorderColor", border);
    put_opt(theme_variables, "activeTaskBkgColor", surface_muted);
    put_opt(theme_variables, "activeTaskBorderColor", line);
    put_opt(theme_variables, "doneTaskBkgColor", success.or(surface_alt));
    put_opt(theme_variables, "doneTaskBorderColor", success.or(border));
    put_opt(theme_variables, "critBkgColor", error);
    put_opt(theme_variables, "critBorderColor", error.or(border));
    put_opt(theme_variables, "excludeBkgColor", surface_alt);
    put_opt(theme_variables, "gridColor", border);
    put_opt(
        theme_variables,
        "todayLineColor",
        warning.or(error).or(line),
    );
    put_opt(theme_variables, "vertLineColor", warning.or(line));
    put_opt(
        theme_variables,
        "sectionBkgColor",
        roles.cluster_background.as_deref().or(surface_alt),
    );
    put_opt(theme_variables, "sectionBkgColor2", surface_muted);
    put_opt(theme_variables, "altSectionBkgColor", canvas);

    put_opt(theme_variables, "errorBkgColor", roles.error.as_deref());
    put_opt(theme_variables, "errorTextColor", text);

    put_opt(theme_variables, "faceColor", surface);
    put_opt(
        theme_variables,
        "border2",
        roles.cluster_border.as_deref().or(border),
    );
}

fn put_series_palette(theme_variables: &mut Map<String, Value>, palette: &[String]) {
    if palette.is_empty() {
        return;
    }

    let mut xy = Map::new();
    xy.insert(
        "plotColorPalette".to_string(),
        Value::String(palette.join(",")),
    );
    xy.insert("accentColor".to_string(), Value::String(palette[0].clone()));
    theme_variables.insert("xyChart".to_string(), Value::Object(xy));

    for (index, color) in palette.iter().enumerate() {
        let label = readable_text_color(color);
        put_str(theme_variables, &format!("cScale{index}"), color);
        put_str(theme_variables, &format!("cScalePeer{index}"), color);
        put_str(theme_variables, &format!("cScaleLabel{index}"), &label);
        put_str(theme_variables, &format!("cScaleInv{index}"), &label);
        put_str(theme_variables, &format!("git{index}"), color);
        put_str(theme_variables, &format!("gitBranchLabel{index}"), &label);
        put_str(theme_variables, &format!("pie{}", index + 1), color);
        put_str(theme_variables, &format!("venn{}", index + 1), color);
        put_str(theme_variables, &format!("fillType{index}"), color);
        put_str(theme_variables, &format!("actor{index}"), color);
    }
}

fn put_diagram_config(
    root: &mut Map<String, Value>,
    theme_variables: &mut Map<String, Value>,
    roles: &HostThemeRoles,
    palette: &[String],
) {
    let text = roles.text.as_deref();
    let subtle_text = roles.subtle_text.as_deref().or(text);
    let surface = roles.surface.as_deref();
    let surface_alt = roles.surface_alt.as_deref().or(surface);
    let border = roles.border.as_deref();
    let line = roles.line.as_deref().or(border);
    let warning = roles.warning.as_deref();
    let success = roles.success.as_deref();

    let mut packet = Map::new();
    put_opt(&mut packet, "startByteColor", line);
    put_opt(&mut packet, "endByteColor", border.or(line));
    put_opt(&mut packet, "labelColor", text);
    put_opt(&mut packet, "titleColor", text);
    put_opt(&mut packet, "blockStrokeColor", border);
    put_opt(&mut packet, "blockFillColor", surface);
    put_nonempty_object(root, "packet", packet);

    let mut treemap = Map::new();
    put_opt(&mut treemap, "titleColor", text);
    put_opt(&mut treemap, "labelColor", text);
    put_opt(&mut treemap, "valueColor", subtle_text);
    put_opt(&mut treemap, "sectionStrokeColor", border);
    put_opt(&mut treemap, "sectionFillColor", surface_alt);
    put_opt(&mut treemap, "leafStrokeColor", border);
    put_opt(&mut treemap, "leafFillColor", surface);
    put_nonempty_object(root, "treemap", treemap);

    let mut tree_view = Map::new();
    put_opt(&mut tree_view, "labelColor", text);
    put_opt(&mut tree_view, "lineColor", line);
    if !tree_view.is_empty() {
        let entry = theme_variables.get("treeView");
        let mut merged = entry
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        merge_object(&mut merged, &tree_view);
        theme_variables.insert("treeView".to_string(), Value::Object(merged));
    }

    let mut radar = Map::new();
    put_opt(&mut radar, "axisColor", line);
    put_opt(&mut radar, "graticuleColor", border);
    put_nonempty_object(root, "radar", radar);

    let mut eventmodeling = Map::new();
    put_opt(
        &mut eventmodeling,
        "emProcessorFill",
        palette.get(3).map(String::as_str).or(surface_alt),
    );
    put_opt(&mut eventmodeling, "emProcessorStroke", border);
    put_opt(
        &mut eventmodeling,
        "emReadModelFill",
        palette
            .get(1)
            .map(String::as_str)
            .or(success)
            .or(surface_alt),
    );
    put_opt(&mut eventmodeling, "emReadModelStroke", success.or(border));
    put_opt(
        &mut eventmodeling,
        "emCommandFill",
        palette.first().map(String::as_str).or(surface_alt),
    );
    put_opt(&mut eventmodeling, "emCommandStroke", line.or(border));
    put_opt(
        &mut eventmodeling,
        "emEventFill",
        palette
            .get(2)
            .map(String::as_str)
            .or(warning)
            .or(surface_alt),
    );
    put_opt(&mut eventmodeling, "emEventStroke", warning.or(border));
    for (key, value) in eventmodeling {
        theme_variables.insert(key, value);
    }

    let mut c4 = Map::new();
    for prefix in [
        "person",
        "system",
        "system_db",
        "system_queue",
        "container",
        "container_db",
        "container_queue",
        "component",
        "component_db",
        "component_queue",
        "external_person",
        "external_system",
        "external_system_db",
        "external_system_queue",
        "external_container",
        "external_container_db",
        "external_container_queue",
        "external_component",
        "external_component_db",
        "external_component_queue",
    ] {
        put_opt(&mut c4, &format!("{prefix}_bg_color"), surface);
        put_opt(&mut c4, &format!("{prefix}_border_color"), border);
    }
    put_nonempty_object(root, "c4", c4);
}

fn put_nonempty_object(root: &mut Map<String, Value>, key: &str, object: Map<String, Value>) {
    if !object.is_empty() {
        root.insert(key.to_string(), Value::Object(object));
    }
}

fn put_opt(map: &mut Map<String, Value>, key: &str, value: Option<&str>) {
    if let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) {
        put_str(map, key, value);
    }
}

fn put_str(map: &mut Map<String, Value>, key: &str, value: &str) {
    map.insert(key.to_string(), Value::String(value.trim().to_string()));
}

fn merge_object(target: &mut Map<String, Value>, source: &Map<String, Value>) {
    for (key, value) in source {
        target.insert(key.clone(), value.clone());
    }
}

fn readable_text_color(color: &str) -> String {
    let Some((r, g, b)) = parse_hex_rgb(color) else {
        return "#ffffff".to_string();
    };
    let luminance = relative_luminance(r, g, b);
    if luminance > 0.45 {
        "#000000".to_string()
    } else {
        "#ffffff".to_string()
    }
}

fn parse_hex_rgb(color: &str) -> Option<(f64, f64, f64)> {
    let raw = color.trim().strip_prefix('#')?;
    if raw.len() != 6 || !raw.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }
    let r = u8::from_str_radix(&raw[0..2], 16).ok()? as f64 / 255.0;
    let g = u8::from_str_radix(&raw[2..4], 16).ok()? as f64 / 255.0;
    let b = u8::from_str_radix(&raw[4..6], 16).ok()? as f64 / 255.0;
    Some((r, g, b))
}

fn relative_luminance(r: f64, g: f64, b: f64) -> f64 {
    fn linear(channel: f64) -> f64 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_editor_profile_compiles_common_theme_variables() {
        let compiled = HostThemeProfile::editor_dark().compile();
        let cfg = compiled.site_config.as_value();
        let vars = cfg["themeVariables"].as_object().unwrap();

        assert_eq!(cfg["theme"], "base");
        assert_eq!(cfg["darkMode"], true);
        assert_eq!(vars["background"], "#0f172a");
        assert_eq!(vars["mainBkg"], "#111827");
        assert_eq!(vars["nodeTextColor"], "#e5e7eb");
        assert_eq!(vars["lineColor"], "#94a3b8");
        assert_eq!(vars["noteBkgColor"], "#422006");
        assert_eq!(vars["actorBkg"], "#1f2937");
        assert_eq!(
            vars["xyChart"]["plotColorPalette"],
            "#60a5fa,#34d399,#f59e0b,#c084fc,#22d3ee,#fb7185,#facc15,#a3e635"
        );
        assert_eq!(vars["pie1"], "#60a5fa");
        assert_eq!(vars["git0"], "#60a5fa");
        assert_eq!(vars["gitBranchLabel0"], "#ffffff");
    }

    #[test]
    fn common_editor_presets_compile_named_palettes() {
        let cases = [
            (HostThemePreset::EditorLight, "#ffffff", "#2563eb"),
            (HostThemePreset::EditorDark, "#0f172a", "#60a5fa"),
            (HostThemePreset::OneDark, "#282c34", "#61afef"),
            (HostThemePreset::GruvboxDark, "#282828", "#83a598"),
            (HostThemePreset::GruvboxLight, "#fbf1c7", "#458588"),
            (HostThemePreset::AyuDark, "#0b0e14", "#59c2ff"),
            (HostThemePreset::AyuLight, "#fafafa", "#55b4d4"),
        ];

        for (preset, background, first_series_color) in cases {
            let compiled = HostThemeProfile::from_preset(preset).compile();
            let cfg = compiled.site_config.as_value();
            let vars = cfg["themeVariables"].as_object().unwrap();

            assert_eq!(cfg["theme"], "base", "{preset:?}");
            assert_eq!(vars["background"], background, "{preset:?}");
            assert_eq!(vars["pie1"], first_series_color, "{preset:?}");
            assert_eq!(
                compiled.output.preset,
                SvgPipelinePreset::ResvgSafe,
                "{preset:?}"
            );
            assert_eq!(
                vars["xyChart"]["accentColor"], first_series_color,
                "{preset:?}"
            );
        }
    }

    #[test]
    fn host_theme_preset_names_are_binding_stable() {
        let names = HostThemePreset::ALL.map(HostThemePreset::as_str);

        assert_eq!(
            names,
            [
                "editor-light",
                "editor-dark",
                "one-dark",
                "gruvbox-light",
                "gruvbox-dark",
                "ayu-light",
                "ayu-dark"
            ]
        );
    }

    #[test]
    fn explicit_profile_theme_variables_override_derived_roles() {
        let profile = HostThemeProfile::builder()
            .roles(HostThemeRoles {
                border: Some("#111111".to_string()),
                ..HostThemeRoles::default()
            })
            .theme_variable("nodeBorder", "#abcdef")
            .build();

        let compiled = profile.compile();
        let vars = compiled.site_config.as_value()["themeVariables"]
            .as_object()
            .unwrap();

        assert_eq!(vars["nodeBorder"], "#abcdef");
        assert_eq!(vars["primaryBorderColor"], "#111111");
    }

    #[test]
    fn empty_profile_compiles_to_empty_site_config() {
        let compiled = HostThemeProfile::default().compile();

        assert_eq!(compiled.site_config.as_value(), &Value::Object(Map::new()));
        assert_eq!(compiled.output.preset, SvgPipelinePreset::Parity);
        assert!(compiled.output.root_background_color.is_none());
    }

    #[test]
    fn compiled_output_builds_host_pipeline() {
        let compiled = HostThemeProfile::editor_dark().compile();
        let pipeline = compiled.pipeline();
        let out = pipeline
            .process_to_string(
                r#"<svg id="host" style="background-color: white;"><style>.node{fill:red !important;}</style><text>A</text></svg>"#,
            )
            .unwrap();

        assert!(!out.contains("!important"));
        assert!(out.contains("background-color: #0f172a;"));
    }
}
