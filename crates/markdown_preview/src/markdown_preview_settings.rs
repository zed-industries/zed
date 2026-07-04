use gpui::{FontWeight, Pixels, Rems, px};
use settings::settings_content::HeadingBorder;
use settings::{RegisterSetting, Settings};

/// The settings for the markdown preview.
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// Per-heading-level overrides. Every attribute of every level defaults
    /// to `None`, in which case the built-in per-level default applies.
    pub headings: Headings,
    /// Line height (rem) for paragraphs and list items.
    pub line_height: Option<Rems>,
    /// Bottom margin (rem) between paragraphs.
    pub paragraph_spacing: Option<Rems>,
    /// Bottom margin (rem) between list items.
    pub list_item_spacing: Option<Rems>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Headings {
    pub h1: HeadingLevel,
    pub h2: HeadingLevel,
    pub h3: HeadingLevel,
    pub h4: HeadingLevel,
    pub h5: HeadingLevel,
    pub h6: HeadingLevel,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeadingLevel {
    pub font_size: Option<Rems>,
    pub bold: Option<bool>,
    pub border: Option<HeadingBorder>,
    pub spacing_before: Option<Rems>,
    pub spacing_after: Option<Rems>,
}

impl HeadingLevel {
    pub fn font_weight(&self) -> Option<FontWeight> {
        self.bold.map(|bold| {
            if bold {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            }
        })
    }
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.markdown_preview.clone().unwrap_or_default();
        let max_width = if content.limit_content_width.unwrap_or(true) {
            content.max_width.map(px)
        } else {
            None
        };

        fn parse_level(
            content: Option<settings::settings_content::HeadingLevelContent>,
        ) -> HeadingLevel {
            let Some(content) = content else {
                return HeadingLevel::default();
            };
            HeadingLevel {
                font_size: content.font_size.map(Rems),
                bold: content.bold,
                border: content.border,
                spacing_before: content.spacing_before.map(Rems),
                spacing_after: content.spacing_after.map(Rems),
            }
        }

        let headings = content
            .headings
            .map(|h| Headings {
                h1: parse_level(h.h1),
                h2: parse_level(h.h2),
                h3: parse_level(h.h3),
                h4: parse_level(h.h4),
                h5: parse_level(h.h5),
                h6: parse_level(h.h6),
            })
            .unwrap_or_default();

        Self {
            max_width,
            headings,
            line_height: content.line_height.map(Rems),
            paragraph_spacing: content.paragraph_spacing.map(Rems),
            list_item_spacing: content.list_item_spacing.map(Rems),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::SettingsContent;
    use settings::settings_content::MarkdownPreviewSettingsContent;

    #[test]
    fn defaults_leave_style_overrides_unset() {
        let content = SettingsContent {
            markdown_preview: Some(MarkdownPreviewSettingsContent::default()),
            ..Default::default()
        };
        let settings = MarkdownPreviewSettings::from_settings(&content);
        assert!(settings.line_height.is_none());
        assert!(settings.paragraph_spacing.is_none());
        assert!(settings.list_item_spacing.is_none());
        // Every attribute of every heading level should be None.
        for level in [
            &settings.headings.h1,
            &settings.headings.h2,
            &settings.headings.h3,
            &settings.headings.h4,
            &settings.headings.h5,
            &settings.headings.h6,
        ] {
            assert!(level.font_size.is_none());
            assert!(level.bold.is_none());
            assert!(level.border.is_none());
            assert!(level.spacing_before.is_none());
            assert!(level.spacing_after.is_none());
        }
    }

    #[test]
    fn parses_provided_headings() {
        let content = SettingsContent {
            markdown_preview: Some(MarkdownPreviewSettingsContent {
                headings: Some(settings::settings_content::HeadingsContent {
                    h1: Some(settings::settings_content::HeadingLevelContent {
                        font_size: Some(2.0),
                        bold: Some(true),
                        border: Some(HeadingBorder::Above),
                        spacing_before: Some(1.5),
                        spacing_after: Some(0.75),
                    }),
                    h4: Some(settings::settings_content::HeadingLevelContent {
                        bold: Some(false),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                line_height: Some(1.6),
                ..Default::default()
            }),
            ..Default::default()
        };
        let settings = MarkdownPreviewSettings::from_settings(&content);
        assert_eq!(settings.line_height, Some(Rems(1.6)));
        assert_eq!(settings.headings.h1.font_size, Some(Rems(2.0)));
        assert_eq!(settings.headings.h1.bold, Some(true));
        assert_eq!(settings.headings.h1.border, Some(HeadingBorder::Above));
        assert_eq!(settings.headings.h1.spacing_before, Some(Rems(1.5)));
        assert_eq!(settings.headings.h1.spacing_after, Some(Rems(0.75)));
        assert_eq!(settings.headings.h4.bold, Some(false));
        assert!(settings.headings.h4.font_size.is_none());
        assert!(settings.headings.h2.font_size.is_none());
    }
}
