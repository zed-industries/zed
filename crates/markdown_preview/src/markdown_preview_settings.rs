use gpui::{Pixels, px};
use settings::{RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// Whether to render YAML frontmatter as a table.
    pub render_frontmatter: bool,
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
            render_frontmatter: content.render_frontmatter.unwrap_or(true),
        }
    }
}
