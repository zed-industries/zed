mod assets;
mod color;
mod theme_printer;
mod util;
mod vscode;
mod zed1;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use any_ascii::any_ascii;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use gpui::Hsla;
use indexmap::IndexMap;
use indoc::formatdoc;
use json_comments::StripComments;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::{TermLogger, TerminalMode};
use theme::{
    Appearance, FontWeightContent, HighlightStyleContent, PlayerColorContent, StatusColorsContent,
    ThemeColorsContent, ThemeContent, ThemeFamilyContent, ThemeStyleContent, UserTheme,
    UserThemeFamily,
};

use crate::theme_printer::UserThemeFamilyPrinter;
use crate::vscode::VsCodeTheme;
use crate::vscode::VsCodeThemeConverter;
use crate::zed1::theme::Theme as Zed1Theme;
use crate::zed1::{zed1_theme_licenses, Zed1ThemeConverter};

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeMetadata>,

    /// Overrides for specific syntax tokens.
    ///
    /// Use this to ensure certain Zed syntax tokens are matched
    /// to an exact set of scopes when it is not otherwise possible
    /// to rely on the default mappings in the theme importer.
    #[serde(default)]
    pub syntax: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeAppearanceJson {
    Light,
    Dark,
}

impl From<ThemeAppearanceJson> for Appearance {
    fn from(value: ThemeAppearanceJson) -> Self {
        match value {
            ThemeAppearanceJson::Light => Self::Light,
            ThemeAppearanceJson::Dark => Self::Dark,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub file_name: String,
    pub appearance: ThemeAppearanceJson,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether to warn when values are missing from the theme.
    #[arg(long)]
    warn_on_missing: bool,
}

fn main() -> Result<()> {
    const SOURCE_PATH: &str = "assets/themes/src/vscode";
    const OUT_PATH: &str = "crates/theme/src/themes";

    let args = Args::parse();

    let log_config = {
        let mut config = simplelog::ConfigBuilder::new();
        config
            .set_level_color(log::Level::Trace, simplelog::Color::Cyan)
            .set_level_color(log::Level::Info, simplelog::Color::Blue)
            .set_level_color(log::Level::Warn, simplelog::Color::Yellow)
            .set_level_color(log::Level::Error, simplelog::Color::Red);

        if !args.warn_on_missing {
            config.add_filter_ignore_str("theme_printer");
        }

        config.build()
    };

    TermLogger::init(LevelFilter::Trace, log_config, TerminalMode::Mixed)
        .expect("could not initialize logger");

    if 1 < 2 {
        let themes: Vec<UserThemeFamily> = Vec::new();
        // Uncomment this line when you need to regenerate themes.
        // let themes = theme::all_user_themes();

        let mut families = Vec::new();

        for family in themes {
            families.push(convert_family(family));
        }

        for family in families {
            let theme_family_slug = any_ascii(&family.name)
                .replace("(", "")
                .replace(")", "")
                .to_case(Case::Snake);

            let output_dir = PathBuf::from("assets/themes/").join(&theme_family_slug);

            fs::create_dir_all(&output_dir)?;

            let mut output_file =
                File::create(output_dir.join(format!("{theme_family_slug}.json")))?;

            let theme_json = serde_json::to_string_pretty(&family).unwrap();

            output_file.write_all(format!("{theme_json}\n").as_bytes())?;
        }

        return Ok(());
    }

    let mut theme_families = Vec::new();

    /// Whether VS Code themes should be imported.
    const IMPORT_VS_CODE_THEMES: bool = false;

    if IMPORT_VS_CODE_THEMES {
        log::info!("Loading themes source...");
        let vscode_themes_path = PathBuf::from_str(SOURCE_PATH)?;
        if !vscode_themes_path.exists() {
            return Err(anyhow!(format!(
                "Couldn't find {}, make sure it exists",
                SOURCE_PATH
            )));
        }

        for theme_family_dir in fs::read_dir(&vscode_themes_path)? {
            let theme_family_dir = theme_family_dir?;

            if !theme_family_dir.file_type()?.is_dir() {
                continue;
            }

            let theme_family_slug = theme_family_dir
                .path()
                .file_stem()
                .ok_or(anyhow!("no file stem"))
                .map(|stem| stem.to_string_lossy().to_string())?;

            let family_metadata_file = File::open(theme_family_dir.path().join("family.json"))
                .context(format!(
                    "no `family.json` found for '{}'",
                    theme_family_slug
                ))?;

            let license_file_path = theme_family_dir.path().join("LICENSE");

            if !license_file_path.exists() {
                log::info!("Skipping theme family '{}' because it does not have a LICENSE file. This theme will only be imported once a LICENSE file is provided.", theme_family_slug);
                continue;
            }

            let family_metadata: FamilyMetadata = serde_json::from_reader(family_metadata_file)
                .context(format!(
                    "failed to parse `family.json` for '{theme_family_slug}'"
                ))?;

            let mut themes = Vec::new();

            for theme_metadata in family_metadata.themes {
                log::info!("Converting '{}' theme", &theme_metadata.name);

                let theme_file_path = theme_family_dir.path().join(&theme_metadata.file_name);

                let theme_file = match File::open(&theme_file_path) {
                    Ok(file) => file,
                    Err(_) => {
                        log::info!("Failed to open file at path: {:?}", theme_file_path);
                        continue;
                    }
                };

                let theme_without_comments = StripComments::new(theme_file);
                let vscode_theme: VsCodeTheme = serde_json::from_reader(theme_without_comments)
                    .context(format!("failed to parse theme {theme_file_path:?}"))?;

                let converter = VsCodeThemeConverter::new(
                    vscode_theme,
                    theme_metadata,
                    family_metadata.syntax.clone(),
                );

                let theme = converter.convert()?;

                themes.push(theme);
            }

            let theme_family = UserThemeFamily {
                name: family_metadata.name.into(),
                author: family_metadata.author.into(),
                themes,
            };

            theme_families.push(theme_family);
        }
    }

    let zed1_themes_path = PathBuf::from_str("assets/themes")?;

    let zed1_theme_families = [
        "Andromeda",
        "Atelier",
        "Ayu",
        "Gruvbox",
        "One",
        "Rosé Pine",
        "Sandcastle",
        "Solarized",
        "Summercamp",
    ];

    let zed1_licenses_by_theme: HashMap<String, zed1::Zed1ThemeLicense> = HashMap::from_iter(
        zed1_theme_licenses()
            .into_iter()
            .map(|theme_license| (theme_license.theme.clone(), theme_license)),
    );

    let mut zed1_themes_by_family: IndexMap<String, Vec<UserTheme>> = IndexMap::from_iter(
        zed1_theme_families
            .into_iter()
            .map(|family| (family.to_string(), Vec::new())),
    );

    for entry in fs::read_dir(&zed1_themes_path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            continue;
        }

        match entry.path().extension() {
            None => continue,
            Some(extension) => {
                if extension != "json" {
                    continue;
                }
            }
        }

        let theme_file_path = entry.path();

        let theme_file = match File::open(&theme_file_path) {
            Ok(file) => file,
            Err(_) => {
                log::info!("Failed to open file at path: {:?}", theme_file_path);
                continue;
            }
        };

        let theme_without_comments = StripComments::new(theme_file);

        let zed1_theme: Zed1Theme = serde_json::from_reader(theme_without_comments)
            .context(format!("failed to parse theme {theme_file_path:?}"))?;

        let theme_name = zed1_theme.meta.name.clone();

        let converter = Zed1ThemeConverter::new(zed1_theme);

        let theme = converter.convert()?;

        let Some((_, themes_for_family)) = zed1_themes_by_family
            .iter_mut()
            .find(|(family, _)| theme_name.starts_with(*family))
        else {
            log::warn!("No theme family found for '{}'.", theme_name);
            continue;
        };

        themes_for_family.push(theme);
    }

    zed1_themes_by_family.sort_keys();

    let mut licenses = Vec::new();

    for (family, themes) in zed1_themes_by_family {
        let mut theme_family = UserThemeFamily {
            name: family,
            author: "Zed Industries".to_string(),
            themes,
        };

        theme_family
            .themes
            .sort_unstable_by_key(|theme| theme.name.clone());

        for theme in &theme_family.themes {
            let license = zed1_licenses_by_theme
                .get(&theme.name)
                .ok_or_else(|| anyhow!("missing license for theme: '{}'", theme.name))?;

            let license_header = match license.license_url.as_ref() {
                Some(license_url) => {
                    format!("[{theme_name}]({license_url})", theme_name = theme.name)
                }
                None => theme.name.clone(),
            };

            licenses.push(formatdoc!(
                "
                ## {license_header}

                {license_text}
                ********************************************************************************
                ",
                license_text = license.license_text
            ));
        }

        theme_families.push(theme_family);
    }

    let themes_output_path = PathBuf::from_str(OUT_PATH)?;

    if !themes_output_path.exists() {
        log::info!("Creating directory: {:?}", themes_output_path);
        fs::create_dir_all(&themes_output_path)?;
    }

    let mut mod_rs_file = File::create(themes_output_path.join(format!("mod.rs")))?;

    let mut theme_modules = Vec::new();

    for theme_family in theme_families {
        let theme_family_slug = any_ascii(&theme_family.name)
            .replace("(", "")
            .replace(")", "")
            .to_case(Case::Snake);

        let mut output_file =
            File::create(themes_output_path.join(format!("{theme_family_slug}.rs")))?;
        log::info!(
            "Creating file: {:?}",
            themes_output_path.join(format!("{theme_family_slug}.rs"))
        );

        let theme_module = format!(
            r#"
            // This file was generated by the `theme_importer`.
            // Be careful when modifying it by hand.

            use gpui::rgba;

            #[allow(unused)]
            use crate::{{
                Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
                UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
                UserFontWeight, UserFontStyle
            }};

            pub fn {theme_family_slug}() -> UserThemeFamily {{
                {theme_family_definition}
            }}
            "#,
            theme_family_definition = format!("{:#?}", UserThemeFamilyPrinter::new(theme_family))
        );

        output_file.write_all(theme_module.as_bytes())?;

        theme_modules.push(theme_family_slug);
    }

    theme_modules.sort();

    let themes_vector_contents = format!(
        r#"
        use crate::UserThemeFamily;

        pub(crate) fn all_user_themes() -> Vec<UserThemeFamily> {{
            vec![{all_themes}]
        }}
        "#,
        all_themes = theme_modules
            .iter()
            .map(|module| format!("{}()", module))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mod_rs_contents = format!(
        r#"
        // This file was generated by the `theme_importer`.
        // Be careful when modifying it by hand.

        {mod_statements}

        {use_statements}

        {themes_vector_contents}
        "#,
        mod_statements = theme_modules
            .iter()
            .map(|module| format!("mod {module};"))
            .collect::<Vec<_>>()
            .join("\n"),
        use_statements = theme_modules
            .iter()
            .map(|module| format!("pub use {module}::*;"))
            .collect::<Vec<_>>()
            .join("\n"),
        themes_vector_contents = themes_vector_contents
    );

    mod_rs_file.write_all(mod_rs_contents.as_bytes())?;

    log::info!("Writing LICENSES file...");

    let mut licenses_file = File::create(themes_output_path.join(format!("LICENSES")))?;

    licenses_file.write_all(licenses.join("\n").as_bytes())?;

    log::info!("Formatting themes...");

    let format_result = format_themes_crate()
        // We need to format a second time to catch all of the formatting issues.
        .and_then(|_| format_themes_crate());

    if let Err(err) = format_result {
        log::error!("Failed to format themes: {}", err);
    }

    log::info!("Done!");

    Ok(())
}

fn format_themes_crate() -> std::io::Result<std::process::Output> {
    Command::new("cargo")
        .args(["fmt", "--package", "theme"])
        .output()
}

fn convert_family(family: UserThemeFamily) -> ThemeFamilyContent {
    ThemeFamilyContent {
        name: family.name,
        author: family.author,
        themes: family.themes.into_iter().map(convert_theme).collect(),
    }
}

fn convert_theme(theme: UserTheme) -> ThemeContent {
    ThemeContent {
        name: theme.name,
        appearance: match theme.appearance {
            Appearance::Light => theme::AppearanceContent::Light,
            Appearance::Dark => theme::AppearanceContent::Dark,
        },
        style: convert_theme_styles(theme.styles),
    }
}

fn serialize_color(color: Hsla) -> String {
    let rgba = color.to_rgb();
    format!("#{:08x}", u32::from(rgba))
}

fn convert_theme_styles(styles: theme::UserThemeStylesRefinement) -> ThemeStyleContent {
    ThemeStyleContent {
        colors: ThemeColorsContent {
            border: styles.colors.border.map(serialize_color),
            border_variant: styles.colors.border_variant.map(serialize_color),
            border_focused: styles.colors.border_focused.map(serialize_color),
            border_selected: styles.colors.border_selected.map(serialize_color),
            border_transparent: styles.colors.border_transparent.map(serialize_color),
            border_disabled: styles.colors.border_disabled.map(serialize_color),
            elevated_surface_background: styles
                .colors
                .elevated_surface_background
                .map(serialize_color),
            surface_background: styles.colors.surface_background.map(serialize_color),
            background: styles.colors.background.map(serialize_color),
            element_background: styles.colors.element_background.map(serialize_color),
            element_hover: styles.colors.element_hover.map(serialize_color),
            element_active: styles.colors.element_active.map(serialize_color),
            element_selected: styles.colors.element_selected.map(serialize_color),
            element_disabled: styles.colors.element_disabled.map(serialize_color),
            drop_target_background: styles.colors.drop_target_background.map(serialize_color),
            ghost_element_background: styles.colors.ghost_element_background.map(serialize_color),
            ghost_element_hover: styles.colors.ghost_element_hover.map(serialize_color),
            ghost_element_active: styles.colors.ghost_element_active.map(serialize_color),
            ghost_element_selected: styles.colors.ghost_element_selected.map(serialize_color),
            ghost_element_disabled: styles.colors.ghost_element_disabled.map(serialize_color),
            text: styles.colors.text.map(serialize_color),
            text_muted: styles.colors.text_muted.map(serialize_color),
            text_placeholder: styles.colors.text_placeholder.map(serialize_color),
            text_disabled: styles.colors.text_disabled.map(serialize_color),
            text_accent: styles.colors.text_accent.map(serialize_color),
            icon: styles.colors.icon.map(serialize_color),
            icon_muted: styles.colors.icon_muted.map(serialize_color),
            icon_disabled: styles.colors.icon_disabled.map(serialize_color),
            icon_placeholder: styles.colors.icon_placeholder.map(serialize_color),
            icon_accent: styles.colors.icon_accent.map(serialize_color),
            status_bar_background: styles.colors.status_bar_background.map(serialize_color),
            title_bar_background: styles.colors.title_bar_background.map(serialize_color),
            toolbar_background: styles.colors.toolbar_background.map(serialize_color),
            tab_bar_background: styles.colors.tab_bar_background.map(serialize_color),
            tab_inactive_background: styles.colors.tab_inactive_background.map(serialize_color),
            tab_active_background: styles.colors.tab_active_background.map(serialize_color),
            search_match_background: styles.colors.search_match_background.map(serialize_color),
            panel_background: styles.colors.panel_background.map(serialize_color),
            panel_focused_border: styles.colors.panel_focused_border.map(serialize_color),
            pane_focused_border: styles.colors.pane_focused_border.map(serialize_color),
            scrollbar_thumb_background: styles
                .colors
                .scrollbar_thumb_background
                .map(serialize_color),
            scrollbar_thumb_hover_background: styles
                .colors
                .scrollbar_thumb_hover_background
                .map(serialize_color),
            scrollbar_thumb_border: styles.colors.scrollbar_thumb_border.map(serialize_color),
            scrollbar_track_background: styles
                .colors
                .scrollbar_track_background
                .map(serialize_color),
            scrollbar_track_border: styles.colors.scrollbar_track_border.map(serialize_color),
            editor_foreground: styles.colors.editor_foreground.map(serialize_color),
            editor_background: styles.colors.editor_background.map(serialize_color),
            editor_gutter_background: styles.colors.editor_gutter_background.map(serialize_color),
            editor_subheader_background: styles
                .colors
                .editor_subheader_background
                .map(serialize_color),
            editor_active_line_background: styles
                .colors
                .editor_active_line_background
                .map(serialize_color),
            editor_highlighted_line_background: styles
                .colors
                .editor_highlighted_line_background
                .map(serialize_color),
            editor_line_number: styles.colors.editor_line_number.map(serialize_color),
            editor_active_line_number: styles.colors.editor_active_line_number.map(serialize_color),
            editor_invisible: styles.colors.editor_invisible.map(serialize_color),
            editor_wrap_guide: styles.colors.editor_wrap_guide.map(serialize_color),
            editor_active_wrap_guide: styles.colors.editor_active_wrap_guide.map(serialize_color),
            editor_document_highlight_read_background: styles
                .colors
                .editor_document_highlight_read_background
                .map(serialize_color),
            editor_document_highlight_write_background: styles
                .colors
                .editor_document_highlight_write_background
                .map(serialize_color),
            terminal_background: styles.colors.terminal_background.map(serialize_color),
            terminal_foreground: styles.colors.terminal_foreground.map(serialize_color),
            terminal_bright_foreground: styles
                .colors
                .terminal_bright_foreground
                .map(serialize_color),
            terminal_dim_foreground: styles.colors.terminal_dim_foreground.map(serialize_color),
            terminal_ansi_black: styles.colors.terminal_ansi_black.map(serialize_color),
            terminal_ansi_bright_black: styles
                .colors
                .terminal_ansi_bright_black
                .map(serialize_color),
            terminal_ansi_dim_black: styles.colors.terminal_ansi_dim_black.map(serialize_color),
            terminal_ansi_red: styles.colors.terminal_ansi_red.map(serialize_color),
            terminal_ansi_bright_red: styles.colors.terminal_ansi_bright_red.map(serialize_color),
            terminal_ansi_dim_red: styles.colors.terminal_ansi_dim_red.map(serialize_color),
            terminal_ansi_green: styles.colors.terminal_ansi_green.map(serialize_color),
            terminal_ansi_bright_green: styles
                .colors
                .terminal_ansi_bright_green
                .map(serialize_color),
            terminal_ansi_dim_green: styles.colors.terminal_ansi_dim_green.map(serialize_color),
            terminal_ansi_yellow: styles.colors.terminal_ansi_yellow.map(serialize_color),
            terminal_ansi_bright_yellow: styles
                .colors
                .terminal_ansi_bright_yellow
                .map(serialize_color),
            terminal_ansi_dim_yellow: styles.colors.terminal_ansi_dim_yellow.map(serialize_color),
            terminal_ansi_blue: styles.colors.terminal_ansi_blue.map(serialize_color),
            terminal_ansi_bright_blue: styles.colors.terminal_ansi_bright_blue.map(serialize_color),
            terminal_ansi_dim_blue: styles.colors.terminal_ansi_dim_blue.map(serialize_color),
            terminal_ansi_magenta: styles.colors.terminal_ansi_magenta.map(serialize_color),
            terminal_ansi_bright_magenta: styles
                .colors
                .terminal_ansi_bright_magenta
                .map(serialize_color),
            terminal_ansi_dim_magenta: styles.colors.terminal_ansi_dim_magenta.map(serialize_color),
            terminal_ansi_cyan: styles.colors.terminal_ansi_cyan.map(serialize_color),
            terminal_ansi_bright_cyan: styles.colors.terminal_ansi_bright_cyan.map(serialize_color),
            terminal_ansi_dim_cyan: styles.colors.terminal_ansi_dim_cyan.map(serialize_color),
            terminal_ansi_white: styles.colors.terminal_ansi_white.map(serialize_color),
            terminal_ansi_bright_white: styles
                .colors
                .terminal_ansi_bright_white
                .map(serialize_color),
            terminal_ansi_dim_white: styles.colors.terminal_ansi_dim_white.map(serialize_color),
            link_text_hover: styles.colors.link_text_hover.map(serialize_color),
        },
        status: StatusColorsContent {
            conflict: styles.status.conflict.map(serialize_color),
            conflict_background: styles.status.conflict_background.map(serialize_color),
            conflict_border: styles.status.conflict_border.map(serialize_color),
            created: styles.status.created.map(serialize_color),
            created_background: styles.status.created_background.map(serialize_color),
            created_border: styles.status.created_border.map(serialize_color),
            deleted: styles.status.deleted.map(serialize_color),
            deleted_background: styles.status.deleted_background.map(serialize_color),
            deleted_border: styles.status.deleted_border.map(serialize_color),
            error: styles.status.error.map(serialize_color),
            error_background: styles.status.error_background.map(serialize_color),
            error_border: styles.status.error_border.map(serialize_color),
            hidden: styles.status.hidden.map(serialize_color),
            hidden_background: styles.status.hidden_background.map(serialize_color),
            hidden_border: styles.status.hidden_border.map(serialize_color),
            hint: styles.status.hint.map(serialize_color),
            hint_background: styles.status.hint_background.map(serialize_color),
            hint_border: styles.status.hint_border.map(serialize_color),
            ignored: styles.status.ignored.map(serialize_color),
            ignored_background: styles.status.ignored_background.map(serialize_color),
            ignored_border: styles.status.ignored_border.map(serialize_color),
            info: styles.status.info.map(serialize_color),
            info_background: styles.status.info_background.map(serialize_color),
            info_border: styles.status.info_border.map(serialize_color),
            modified: styles.status.modified.map(serialize_color),
            modified_background: styles.status.modified_background.map(serialize_color),
            modified_border: styles.status.modified_border.map(serialize_color),
            predictive: styles.status.predictive.map(serialize_color),
            predictive_background: styles.status.predictive_background.map(serialize_color),
            predictive_border: styles.status.predictive_border.map(serialize_color),
            renamed: styles.status.renamed.map(serialize_color),
            renamed_background: styles.status.renamed_background.map(serialize_color),
            renamed_border: styles.status.renamed_border.map(serialize_color),
            success: styles.status.success.map(serialize_color),
            success_background: styles.status.success_background.map(serialize_color),
            success_border: styles.status.success_border.map(serialize_color),
            unreachable: styles.status.unreachable.map(serialize_color),
            unreachable_background: styles.status.unreachable_background.map(serialize_color),
            unreachable_border: styles.status.unreachable_border.map(serialize_color),
            warning: styles.status.warning.map(serialize_color),
            warning_background: styles.status.warning_background.map(serialize_color),
            warning_border: styles.status.warning_border.map(serialize_color),
        },
        players: styles
            .player
            .map(|players| {
                players
                    .0
                    .into_iter()
                    .map(|player_color| PlayerColorContent {
                        cursor: Some(player_color.cursor).map(serialize_color),
                        background: Some(player_color.background).map(serialize_color),
                        selection: Some(player_color.selection).map(serialize_color),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        syntax: styles
            .syntax
            .map(|syntax| {
                IndexMap::from_iter(syntax.highlights.into_iter().map(|(name, style)| {
                    (
                        name,
                        HighlightStyleContent {
                            color: style.color.map(serialize_color),
                            font_style: style.font_style.map(|font_style| match font_style {
                                theme::UserFontStyle::Normal => theme::FontStyleContent::Normal,
                                theme::UserFontStyle::Italic => theme::FontStyleContent::Italic,
                                theme::UserFontStyle::Oblique => theme::FontStyleContent::Oblique,
                            }),
                            font_weight: style.font_weight.map(|font_weight| match font_weight.0 {
                                _ if font_weight.0 == 100.0 => FontWeightContent::Thin,
                                _ if font_weight.0 == 200.0 => FontWeightContent::ExtraLight,
                                _ if font_weight.0 == 300.0 => FontWeightContent::Light,
                                _ if font_weight.0 == 400.0 => FontWeightContent::Normal,
                                _ if font_weight.0 == 500.0 => FontWeightContent::Medium,
                                _ if font_weight.0 == 600.0 => FontWeightContent::Semibold,
                                _ if font_weight.0 == 700.0 => FontWeightContent::Bold,
                                _ if font_weight.0 == 800.0 => FontWeightContent::ExtraBold,
                                _ if font_weight.0 == 900.0 => FontWeightContent::Black,
                                _ => unreachable!(),
                            }),
                        },
                    )
                }))
            })
            .unwrap_or_default(),
    }
}
