use std::sync::Arc;

use anyhow::{Context as _, Result};
use gpui::{Refineable as _, WindowBackgroundAppearance};
use gpui_util::ResultExt as _;
use settings_content::{AccentContent, PlayerColorContent};
use theme::{
    AccentColors, Appearance, AppearanceContent, PlayerColor, PlayerColors, StatusColors,
    SyntaxTheme, SystemColors, Theme, ThemeColors, ThemeFamily, ThemeRegistry, ThemeStyles,
    default_color_scales, try_parse_color,
};

use crate::{
    ThemeContent, ThemeFamilyContent, WindowBackgroundContent, status_colors_refinement,
    syntax_overrides, theme_colors_refinement,
};

/// Loads the themes bundled with an application's asset source into the registry.
pub fn load_bundled_themes(registry: &ThemeRegistry) {
    let theme_paths = registry
        .assets()
        .list("themes/")
        .expect("failed to list theme assets")
        .into_iter()
        .filter(|path| path.ends_with(".json"));

    for path in theme_paths {
        let Some(theme) = registry.assets().load(&path).log_err().flatten() else {
            continue;
        };

        let Some(theme_family) = serde_json::from_slice(&theme)
            .with_context(|| format!("failed to parse theme at path \"{path}\""))
            .log_err()
        else {
            continue;
        };

        let refined = refine_theme_family(theme_family);
        registry.insert_theme_families([refined]);
    }
}

/// Loads a serialized theme family into the registry.
pub fn load_user_theme(registry: &ThemeRegistry, bytes: &[u8]) -> Result<()> {
    let theme = deserialize_user_theme(bytes)?;
    let refined = refine_theme_family(theme);
    registry.insert_theme_families([refined]);
    Ok(())
}

/// Deserializes a theme family from the given bytes.
pub fn deserialize_user_theme(bytes: &[u8]) -> Result<ThemeFamilyContent> {
    let theme_family: ThemeFamilyContent = serde_json_lenient::from_slice(bytes)?;

    for theme in &theme_family.themes {
        if theme
            .style
            .colors
            .deprecated_scrollbar_thumb_background
            .is_some()
        {
            log::warn!(
                r#"Theme "{theme_name}" is using a deprecated style property: scrollbar_thumb.background. Use `scrollbar.thumb.background` instead."#,
                theme_name = theme.name
            )
        }
    }

    Ok(theme_family)
}

/// Refines a serialized theme family into runtime themes.
pub fn refine_theme_family(theme_family_content: ThemeFamilyContent) -> ThemeFamily {
    let id = uuid::Uuid::new_v4().to_string();
    let name = theme_family_content.name.clone();
    let author = theme_family_content.author.clone();

    let themes = theme_family_content
        .themes
        .iter()
        .map(refine_theme)
        .collect();

    ThemeFamily {
        id,
        name: name.into(),
        author: author.into(),
        themes,
        scales: default_color_scales(),
    }
}

/// Refines serialized theme content into a runtime theme.
pub fn refine_theme(theme_content: &ThemeContent) -> Theme {
    let appearance = match theme_content.appearance {
        AppearanceContent::Light => Appearance::Light,
        AppearanceContent::Dark => Appearance::Dark,
    };

    let mut refined_status_colors = match theme_content.appearance {
        AppearanceContent::Light => StatusColors::light(),
        AppearanceContent::Dark => StatusColors::dark(),
    };
    let mut status_colors_refinement = status_colors_refinement(&theme_content.style.status);
    theme::apply_status_color_defaults(&mut status_colors_refinement);
    refined_status_colors.refine(&status_colors_refinement);

    let mut refined_player_colors = match theme_content.appearance {
        AppearanceContent::Light => PlayerColors::light(),
        AppearanceContent::Dark => PlayerColors::dark(),
    };
    merge_player_colors(&mut refined_player_colors, &theme_content.style.players);

    let mut refined_theme_colors = match theme_content.appearance {
        AppearanceContent::Light => ThemeColors::light(),
        AppearanceContent::Dark => ThemeColors::dark(),
    };
    let mut theme_colors_refinement = theme_colors_refinement(
        &theme_content.style.colors,
        &status_colors_refinement,
        theme_content.appearance == AppearanceContent::Light,
    );
    theme::apply_theme_color_defaults(&mut theme_colors_refinement, &refined_player_colors);
    refined_theme_colors.refine(&theme_colors_refinement);

    let mut refined_accent_colors = match theme_content.appearance {
        AppearanceContent::Light => AccentColors::light(),
        AppearanceContent::Dark => AccentColors::dark(),
    };
    merge_accent_colors(&mut refined_accent_colors, &theme_content.style.accents);

    let syntax_theme = Arc::new(SyntaxTheme::new(syntax_overrides(&theme_content.style)));

    let window_background_appearance = theme_content
        .style
        .window_background_appearance
        .map(|appearance| match appearance {
            WindowBackgroundContent::Opaque => WindowBackgroundAppearance::Opaque,
            WindowBackgroundContent::Transparent => WindowBackgroundAppearance::Transparent,
            WindowBackgroundContent::Blurred => WindowBackgroundAppearance::Blurred,
        })
        .unwrap_or_default();

    Theme {
        id: uuid::Uuid::new_v4().to_string(),
        name: theme_content.name.clone().into(),
        appearance,
        styles: ThemeStyles {
            system: SystemColors::default(),
            window_background_appearance,
            accents: refined_accent_colors,
            colors: refined_theme_colors,
            status: refined_status_colors,
            player: refined_player_colors,
            syntax: syntax_theme,
        },
    }
}

/// Merges serialized player color overrides into runtime colors.
pub fn merge_player_colors(
    player_colors: &mut PlayerColors,
    user_player_colors: &[PlayerColorContent],
) {
    if user_player_colors.is_empty() {
        return;
    }

    for (index, player) in user_player_colors.iter().enumerate() {
        let cursor = player
            .cursor
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let background = player
            .background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let selection = player
            .selection
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());

        if let Some(player_color) = player_colors.0.get_mut(index) {
            *player_color = PlayerColor {
                cursor: cursor.unwrap_or(player_color.cursor),
                background: background.unwrap_or(player_color.background),
                selection: selection.unwrap_or(player_color.selection),
            };
        } else {
            player_colors.0.push(PlayerColor {
                cursor: cursor.unwrap_or_default(),
                background: background.unwrap_or_default(),
                selection: selection.unwrap_or_default(),
            });
        }
    }
}

/// Merges serialized accent color overrides into runtime colors.
pub fn merge_accent_colors(accent_colors: &mut AccentColors, user_accent_colors: &[AccentContent]) {
    if user_accent_colors.is_empty() {
        return;
    }

    let colors = user_accent_colors
        .iter()
        .filter_map(|accent_color| {
            accent_color
                .0
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
        })
        .collect::<Vec<_>>();

    if !colors.is_empty() {
        accent_colors.0 = Arc::from(colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_and_refines_a_theme_family() {
        let family = deserialize_user_theme(
            br##"{
                "name": "Test",
                "author": "Zed",
                "themes": [{
                    "name": "Test Dark",
                    "appearance": "dark",
                    "style": {
                        "background": "#112233",
                        "syntax": {
                            "keyword": {
                                "color": "#abcdef",
                                "font_style": "italic",
                                "font_weight": 1000
                            }
                        }
                    }
                }]
            }"##,
        )
        .expect("theme family should deserialize");

        let family = refine_theme_family(family);

        assert_eq!(family.name.as_ref(), "Test");
        assert_eq!(family.themes.len(), 1);
        assert_eq!(family.themes[0].name.as_ref(), "Test Dark");
        assert_eq!(family.themes[0].appearance, Appearance::Dark);
        assert_eq!(
            family.themes[0].styles.colors.background,
            try_parse_color("#112233").expect("background color should parse")
        );
        let keyword_style = family.themes[0]
            .styles
            .syntax
            .style_for_name("keyword")
            .expect("keyword style should exist");
        assert_eq!(keyword_style.font_style, Some(gpui::FontStyle::Italic));
        assert_eq!(keyword_style.font_weight, Some(gpui::FontWeight(950.)));
    }
}
