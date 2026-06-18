use gpui::{DefiniteLength, Hsla, Pixels, Rgba, px, relative, rems};
use settings::{IntoGpui, RegisterSetting, Settings};

/// The default margin (in pixels) used when the setting is unset or a length
/// string fails to parse.
const DEFAULT_MARGIN_PX: f32 = 16.0;
/// Percentage margins are clamped to this fraction of the preview width so a
/// stray large value can't swallow the whole pane.
const MAX_MARGIN_FRACTION: f32 = 0.40;

/// The resolved settings for the markdown preview.
///
/// The user-facing schema lives in `settings::MarkdownPreviewSettingsContent`,
/// which is declared on `ProjectSettingsContent` so it can be overridden per
/// project in `.zed/settings.json`. `from_settings` collapses the optional
/// content fields into concrete values, applying the built-in fallbacks,
/// parsing colors, and clamping percentage margins.
#[derive(Clone, Debug, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    /// The maximum width of the rendered markdown content, or `None` to render
    /// content edge to edge.
    pub max_width: Option<Pixels>,
    /// Padding around the rendered preview content. Either an absolute length
    /// (px/rems) or a fraction of the preview width.
    pub margin: DefiniteLength,
    /// Per-level heading style overrides.
    pub headings: markdown::HeadingStyles,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let preview = content.project.markdown_preview.as_ref();
        let max_width = preview.and_then(|preview| {
            if preview.limit_content_width.unwrap_or(true) {
                preview.max_width.map(px)
            } else {
                None
            }
        });
        let margin = resolve_margin(preview.and_then(|preview| preview.margin.as_ref()));
        let headings = preview
            .and_then(|preview| preview.headings.as_ref())
            .map(|headings| markdown::HeadingStyles {
                h1: resolve_heading_style(headings.h1.as_ref()),
                h2: resolve_heading_style(headings.h2.as_ref()),
                h3: resolve_heading_style(headings.h3.as_ref()),
                h4: resolve_heading_style(headings.h4.as_ref()),
                h5: resolve_heading_style(headings.h5.as_ref()),
                h6: resolve_heading_style(headings.h6.as_ref()),
            })
            .unwrap_or_default();
        Self {
            max_width,
            margin,
            headings,
        }
    }
}

fn default_margin() -> DefiniteLength {
    DefiniteLength::Absolute(px(DEFAULT_MARGIN_PX).into())
}

fn resolve_margin(margin: Option<&settings::MarkdownPreviewMargin>) -> DefiniteLength {
    match margin {
        None => default_margin(),
        Some(settings::MarkdownPreviewMargin::Pixels(pixels)) => {
            DefiniteLength::Absolute(px(pixels.max(0.0)).into())
        }
        Some(settings::MarkdownPreviewMargin::Length(length)) => {
            parse_length(length).unwrap_or_else(default_margin)
        }
    }
}

/// Parses a CSS-like length string (`"16px"`, `"1rem"`, `"5%"`) by delegating to
/// gpui's canonical [`DefiniteLength`] parser, then clamps percentages to
/// [`MAX_MARGIN_FRACTION`]. Returns `None` for anything unparseable so the caller
/// can fall back to the default.
fn parse_length(length: &str) -> Option<DefiniteLength> {
    match DefiniteLength::try_from(length.trim()).ok()? {
        DefiniteLength::Fraction(fraction) => {
            Some(relative(fraction.clamp(0.0, MAX_MARGIN_FRACTION)))
        }
        absolute @ DefiniteLength::Absolute(_) => Some(absolute),
    }
}

fn resolve_heading_style(
    content: Option<&settings::HeadingStyleContent>,
) -> markdown::HeadingStyle {
    let Some(content) = content else {
        return markdown::HeadingStyle::default();
    };
    markdown::HeadingStyle {
        font_size: content.font_size.map(|size| rems(size).into()),
        color: content
            .color
            .as_deref()
            .and_then(|color| Rgba::try_from(color).map(Hsla::from).ok()),
        font_weight: content.font_weight.map(|weight| weight.into_gpui()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::{FontWeightContent, MarkdownPreviewMargin};

    fn px_margin(pixels: f32) -> DefiniteLength {
        DefiniteLength::Absolute(px(pixels).into())
    }

    #[test]
    fn margin_resolution() {
        // Unset → default 16px.
        assert_eq!(resolve_margin(None), px_margin(16.0));
        // Bare number → pixels.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Pixels(128.0))),
            px_margin(128.0)
        );
        // Negative pixels clamped to zero.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Pixels(-10.0))),
            px_margin(0.0)
        );
        // "px" / "rem" length strings.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Length("64px".into()))),
            px_margin(64.0)
        );
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Length("2rem".into()))),
            DefiniteLength::Absolute(rems(2.0).into())
        );
        // Percentage → fraction of the preview width.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Length("5%".into()))),
            relative(5.0 / 100.0)
        );
        // Percentage clamped to the 40% maximum.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Length("80%".into()))),
            relative(MAX_MARGIN_FRACTION)
        );
        // Unparseable string → default.
        assert_eq!(
            resolve_margin(Some(&MarkdownPreviewMargin::Length("nonsense".into()))),
            px_margin(16.0)
        );
    }

    #[test]
    fn heading_style_resolution() {
        use gpui::{FontWeight, rgb};

        // No override → empty style (defers to the built-in size, theme color,
        // and theme weight).
        let style = resolve_heading_style(None);
        assert_eq!(style.font_size, None);
        assert_eq!(style.color, None);
        assert_eq!(style.font_weight, None);

        // A font size (rems), a valid hex color, and a font weight.
        let content = settings::HeadingStyleContent {
            font_size: Some(2.0),
            color: Some("#ff0000".into()),
            font_weight: Some(FontWeightContent(700.0)),
        };
        let style = resolve_heading_style(Some(&content));
        assert_eq!(style.font_size, Some(rems(2.0).into()));
        assert_eq!(style.color, Some(Hsla::from(rgb(0xff0000))));
        assert_eq!(style.font_weight, Some(FontWeight::BOLD));

        // Defensive: an invalid color string resolves to None (so rendering falls
        // back to the theme's heading color), and a missing size/weight stays None.
        let content = settings::HeadingStyleContent {
            font_size: None,
            color: Some("not-a-color".into()),
            font_weight: None,
        };
        let style = resolve_heading_style(Some(&content));
        assert_eq!(style.font_size, None);
        assert_eq!(style.color, None);
        assert_eq!(style.font_weight, None);
    }

    #[gpui::test]
    fn project_override(cx: &mut gpui::TestAppContext) {
        use gpui::BorrowAppContext;
        use settings::SettingsLocation;
        use util::rel_path::RelPath;

        cx.update(|cx| {
            if !cx.has_global::<settings::SettingsStore>() {
                settings::init(cx);
            }
            if !cx.has_global::<theme::GlobalTheme>() {
                theme_settings::init(theme::LoadThemes::JustBase, cx);
            }
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                // User-level settings define both `max_width` and `margin`.
                store
                    .set_user_settings(
                        r#"{"markdown_preview": {"max_width": 900, "margin": 20}}"#,
                        cx,
                    )
                    .result()
                    .unwrap();
                // Project-level override (like a .zed/settings.json) sets *only*
                // `margin` for worktree 0.
                store
                    .set_local_settings(
                        settings::WorktreeId::from_usize(0),
                        settings::LocalSettingsPath::InWorktree(RelPath::empty_arc()),
                        settings::LocalSettingsKind::Settings,
                        Some(r#"{"markdown_preview": {"margin": 50}}"#),
                        cx,
                    )
                    .unwrap();
            });

            // No location → the user-level values.
            let user = MarkdownPreviewSettings::get(None, cx);
            assert_eq!(user.margin, px_margin(20.0));
            assert_eq!(user.max_width, Some(px(900.0)));

            // A file inside worktree 0 → the project override wins for `margin`,
            // but the merge is field-by-field, so the user's `max_width` is kept.
            let in_project = SettingsLocation {
                worktree_id: settings::WorktreeId::from_usize(0),
                path: RelPath::empty(),
            };
            let project = MarkdownPreviewSettings::get(Some(in_project), cx);
            assert_eq!(project.margin, px_margin(50.0));
            assert_eq!(project.max_width, Some(px(900.0)));

            // A file in a worktree without an override → falls back to the user
            // values entirely.
            let other_worktree = SettingsLocation {
                worktree_id: settings::WorktreeId::from_usize(1),
                path: RelPath::empty(),
            };
            let other = MarkdownPreviewSettings::get(Some(other_worktree), cx);
            assert_eq!(other.margin, px_margin(20.0));
            assert_eq!(other.max_width, Some(px(900.0)));
        });
    }
}
