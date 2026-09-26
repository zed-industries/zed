use gpui::Pixels;
use markdown::{MermaidAlignment, MermaidLayout};
use settings::{IntoGpui, RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// Whether to automatically open Markdown files in the preview.
    pub open_markdown_files_in_preview: bool,
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// How Mermaid diagrams are laid out.
    pub mermaid_layout: MermaidLayout,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.markdown_preview.clone().unwrap_or_default();
        let max_width = if content.limit_content_width.unwrap_or(true) {
            content.max_width.map(IntoGpui::into_gpui)
        } else {
            None
        };
        let mermaid_max_width = if content.limit_mermaid_width.unwrap_or(false) {
            content.mermaid_max_width.map(IntoGpui::into_gpui)
        } else {
            None
        };
        let mermaid_alignment = match content.mermaid_alignment.unwrap_or_default() {
            settings::MermaidAlignment::Left => MermaidAlignment::Left,
            settings::MermaidAlignment::Center => MermaidAlignment::Center,
            settings::MermaidAlignment::Right => MermaidAlignment::Right,
        };
        Self {
            open_markdown_files_in_preview: content.open_markdown_files_in_preview.unwrap_or(false),
            max_width,
            mermaid_layout: MermaidLayout {
                max_width: mermaid_max_width,
                alignment: mermaid_alignment,
                width_follows_diagram: content.mermaid_width_follows_diagram.unwrap_or(false),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;
    use serde_json::json;

    #[test]
    fn test_mermaid_inactive_width_does_not_limit_markdown() {
        for limit_content_width in [false, true] {
            for mermaid_max_width in [800, 1400] {
                let content = serde_json::from_value(json!({
                    "markdown_preview": {
                        "limit_content_width": limit_content_width,
                        "max_width": 900,
                        "mermaid_max_width": mermaid_max_width
                    }
                }))
                .expect("valid preview settings");
                let settings = MarkdownPreviewSettings::from_settings(&content);
                assert_eq!(settings.max_width, limit_content_width.then_some(px(900.)));
                assert_eq!(settings.mermaid_layout, MermaidLayout::default());
            }
        }
    }

    #[test]
    fn test_mermaid_settings_keep_width_flags_and_alignment_independent() {
        for limit_mermaid_width in [false, true] {
            for width_follows_diagram in [false, true] {
                let content = serde_json::from_value(json!({
                    "markdown_preview": {
                        "limit_content_width": false,
                        "limit_mermaid_width": limit_mermaid_width,
                        "mermaid_max_width": 1200,
                        "mermaid_width_follows_diagram": width_follows_diagram,
                        "mermaid_alignment": "center"
                    }
                }))
                .expect("valid preview settings");
                let settings = MarkdownPreviewSettings::from_settings(&content);
                assert_eq!(settings.max_width, None);
                assert_eq!(
                    settings.mermaid_layout.max_width,
                    limit_mermaid_width.then_some(px(1200.))
                );
                assert_eq!(
                    settings.mermaid_layout.width_follows_diagram,
                    width_follows_diagram
                );
                assert_eq!(settings.mermaid_layout.alignment, MermaidAlignment::Center);
                assert_eq!(
                    settings.mermaid_layout.has_width_override(),
                    limit_mermaid_width || width_follows_diagram
                );
            }
        }
    }
}
