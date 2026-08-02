use gpui::{Pixels, px};
use settings::{PlantUmlRenderMode, RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// How PlantUML diagrams are rendered.
    pub plantuml_render_mode: PlantUmlRenderMode,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.markdown_preview.clone().unwrap_or_default();
        let max_width = if content.limit_content_width.unwrap_or(true) {
            content.max_width.map(px)
        } else {
            None
        };
        Self {
            max_width,
            plantuml_render_mode: content.plantuml_render_mode.unwrap_or_default(),
        }
    }
}
