#![deny(missing_docs)]

//! # Theme Settings
//!
//! This crate provides theme settings integration for Zed,
//! bridging the theme system with the settings infrastructure.

mod settings;

use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use gpui::{App, Font, Pixels, px};
use theme::{
    DEFAULT_DARK_THEME, DEFAULT_ICON_THEME_NAME, GlobalTheme, LoadThemes, SystemAppearance, Theme,
    ThemeRegistry, ThemeSettingsProvider,
};

use crate::settings::adjust_buffer_font_size;
pub use crate::settings::{
    AgentBufferFontSize, AgentUiFontSize, BufferLineHeight, FontFamilyName,
    GitCommitBufferFontSize, IconThemeName, IconThemeSelection, MarkdownPreviewFontSize,
    ThemeAppearanceMode, ThemeName, ThemeSelection, ThemeSettings, adjust_agent_buffer_font_size,
    adjust_agent_ui_font_size, adjust_git_commit_buffer_font_size,
    adjust_markdown_preview_font_size, adjust_ui_font_size, adjusted_font_size, appearance_to_mode,
    clamp_font_size, default_theme, observe_buffer_font_size_adjustment,
    reset_agent_buffer_font_size, reset_agent_ui_font_size, reset_buffer_font_size,
    reset_git_commit_buffer_font_size, reset_markdown_preview_font_size, reset_ui_font_size,
    set_icon_theme, set_mode, set_theme, setup_ui_font,
};
pub use theme::UiDensity;
pub use theme_schema::{
    FontStyleContent, FontWeightContent, HighlightStyleContent, StatusColorsContent,
    ThemeColorsContent, ThemeContent, ThemeFamilyContent, ThemeStyleContent,
    WindowBackgroundContent, deserialize_user_theme, load_bundled_themes, load_user_theme,
    merge_accent_colors, merge_player_colors, refine_theme, refine_theme_family,
    status_colors_refinement, syntax_overrides, theme_colors_refinement,
};

struct ThemeSettingsProviderImpl;

impl ThemeSettingsProvider for ThemeSettingsProviderImpl {
    fn ui_font<'a>(&'a self, cx: &'a App) -> &'a Font {
        &ThemeSettings::get_global(cx).ui_font
    }

    fn buffer_font<'a>(&'a self, cx: &'a App) -> &'a Font {
        &ThemeSettings::get_global(cx).buffer_font
    }

    fn ui_font_size(&self, cx: &App) -> Pixels {
        ThemeSettings::get_global(cx).ui_font_size(cx)
    }

    fn buffer_font_size(&self, cx: &App) -> Pixels {
        ThemeSettings::get_global(cx).buffer_font_size(cx)
    }

    fn ui_density(&self, cx: &App) -> UiDensity {
        ThemeSettings::get_global(cx).ui_density
    }
}

/// Initialize the theme system with settings integration.
///
/// This is the full initialization for the application. It calls [`theme::init`]
/// and then wires up settings observation for theme/font changes.
pub fn init(themes_to_load: LoadThemes, cx: &mut App) {
    let load_user_themes = matches!(&themes_to_load, LoadThemes::All(_));

    theme::init(themes_to_load, cx);
    theme::set_theme_settings_provider(Box::new(ThemeSettingsProviderImpl), cx);

    if load_user_themes {
        let registry = ThemeRegistry::global(cx);
        load_bundled_themes(&registry);
    }

    let theme = configured_theme(cx);
    let icon_theme = configured_icon_theme(cx);
    GlobalTheme::update_theme(cx, theme);
    GlobalTheme::update_icon_theme(cx, icon_theme);

    let settings = ThemeSettings::get_global(cx);

    let mut prev_buffer_font_size_settings = settings.buffer_font_size_settings();
    let mut prev_ui_font_size_settings = settings.ui_font_size_settings();
    let mut prev_agent_ui_font_size_settings = settings.agent_ui_font_size_settings();
    let mut prev_agent_buffer_font_size_settings = settings.agent_buffer_font_size_settings();
    let mut prev_git_commit_buffer_font_size_settings =
        settings.git_commit_buffer_font_size_settings();
    let mut prev_markdown_preview_font_size_settings =
        settings.markdown_preview_font_size_settings();
    let mut prev_theme_name = settings.theme.name(SystemAppearance::global(cx).0);
    let mut prev_icon_theme_name = settings.icon_theme.name(SystemAppearance::global(cx).0);
    let mut prev_theme_overrides = (
        settings.experimental_theme_overrides.clone(),
        settings.theme_overrides.clone(),
    );

    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = ThemeSettings::get_global(cx);

        let buffer_font_size_settings = settings.buffer_font_size_settings();
        let ui_font_size_settings = settings.ui_font_size_settings();
        let agent_ui_font_size_settings = settings.agent_ui_font_size_settings();
        let agent_buffer_font_size_settings = settings.agent_buffer_font_size_settings();
        let git_commit_buffer_font_size_settings = settings.git_commit_buffer_font_size_settings();
        let markdown_preview_font_size_settings = settings.markdown_preview_font_size_settings();
        let theme_name = settings.theme.name(SystemAppearance::global(cx).0);
        let icon_theme_name = settings.icon_theme.name(SystemAppearance::global(cx).0);
        let theme_overrides = (
            settings.experimental_theme_overrides.clone(),
            settings.theme_overrides.clone(),
        );

        if buffer_font_size_settings != prev_buffer_font_size_settings {
            prev_buffer_font_size_settings = buffer_font_size_settings;
            reset_buffer_font_size(cx);
        }

        if ui_font_size_settings != prev_ui_font_size_settings {
            prev_ui_font_size_settings = ui_font_size_settings;
            reset_ui_font_size(cx);
        }

        if agent_ui_font_size_settings != prev_agent_ui_font_size_settings {
            prev_agent_ui_font_size_settings = agent_ui_font_size_settings;
            reset_agent_ui_font_size(cx);
        }

        if agent_buffer_font_size_settings != prev_agent_buffer_font_size_settings {
            prev_agent_buffer_font_size_settings = agent_buffer_font_size_settings;
            reset_agent_buffer_font_size(cx);
        }

        if git_commit_buffer_font_size_settings != prev_git_commit_buffer_font_size_settings {
            prev_git_commit_buffer_font_size_settings = git_commit_buffer_font_size_settings;
            reset_git_commit_buffer_font_size(cx);
        }

        if markdown_preview_font_size_settings != prev_markdown_preview_font_size_settings {
            prev_markdown_preview_font_size_settings = markdown_preview_font_size_settings;
            reset_markdown_preview_font_size(cx);
        }

        if theme_name != prev_theme_name || theme_overrides != prev_theme_overrides {
            prev_theme_name = theme_name;
            prev_theme_overrides = theme_overrides;
            reload_theme(cx);
        }

        if icon_theme_name != prev_icon_theme_name {
            prev_icon_theme_name = icon_theme_name;
            reload_icon_theme(cx);
        }
    })
    .detach();
}

fn configured_theme(cx: &mut App) -> Arc<Theme> {
    let themes = ThemeRegistry::default_global(cx);
    let theme_settings = ThemeSettings::get_global(cx);
    let system_appearance = SystemAppearance::global(cx);

    let theme_name = theme_settings.theme.name(*system_appearance);

    let theme = match themes.get(&theme_name.0) {
        Ok(theme) => theme,
        Err(err) => {
            if themes.extensions_loaded() {
                log::error!("{err}");
            }
            themes
                .get(default_theme(*system_appearance))
                .unwrap_or_else(|_| themes.get(DEFAULT_DARK_THEME).unwrap())
        }
    };
    theme_settings.apply_theme_overrides(theme)
}

fn configured_icon_theme(cx: &mut App) -> Arc<theme::IconTheme> {
    let themes = ThemeRegistry::default_global(cx);
    let theme_settings = ThemeSettings::get_global(cx);
    let system_appearance = SystemAppearance::global(cx);

    let icon_theme_name = theme_settings.icon_theme.name(*system_appearance);

    match themes.get_icon_theme(&icon_theme_name.0) {
        Ok(theme) => theme,
        Err(err) => {
            if themes.extensions_loaded() {
                log::error!("{err}");
            }
            themes.get_icon_theme(DEFAULT_ICON_THEME_NAME).unwrap()
        }
    }
}

/// Reloads the current theme from settings.
pub fn reload_theme(cx: &mut App) {
    let theme = configured_theme(cx);
    GlobalTheme::update_theme(cx, theme);
    cx.refresh_windows();
}

/// Reloads the current icon theme from settings.
pub fn reload_icon_theme(cx: &mut App) {
    let icon_theme = configured_icon_theme(cx);
    GlobalTheme::update_icon_theme(cx, icon_theme);
    cx.refresh_windows();
}

/// Increases the buffer font size by 1 pixel, without persisting the result in the settings.
/// This will be effective until the app is restarted.
pub fn increase_buffer_font_size(cx: &mut App) {
    adjust_buffer_font_size(cx, |size| size + px(1.0));
}

/// Decreases the buffer font size by 1 pixel, without persisting the result in the settings.
/// This will be effective until the app is restarted.
pub fn decrease_buffer_font_size(cx: &mut App) {
    adjust_buffer_font_size(cx, |size| size - px(1.0));
}
