use gpui::Pixels;
use settings::{IntoGpui, RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// Whether to automatically open Markdown files in the preview.
    pub open_markdown_files_in_preview: bool,
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.markdown_preview.clone().unwrap_or_default();
        let max_width = if content.limit_content_width.unwrap_or(true) {
            content.max_width.map(IntoGpui::into_gpui)
        } else {
            None
        };
        Self {
            open_markdown_files_in_preview: content.open_markdown_files_in_preview.unwrap_or(false),
            max_width,
        }
    }
}
