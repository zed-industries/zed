use gpui::{Pixels, px};
use settings::{RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// Whether opening a markdown file opens a markdown preview tab instead
    /// of a text editor.
    pub open_preview_on_file_open: bool,
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
            open_preview_on_file_open: content.open_preview_on_file_open.unwrap_or(false),
        }
    }
}
