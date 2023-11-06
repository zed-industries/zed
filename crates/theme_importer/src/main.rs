mod theme_printer;
mod util;
mod vscode;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use gpui::serde_json;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::SimpleLogger;
use theme::{default_color_scales, Appearance, ThemeFamily};
use vscode::VsCodeThemeConverter;

use crate::theme_printer::ThemeFamilyPrinter;
use crate::vscode::VsCodeTheme;

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeMetadata>,
}

#[derive(Debug, Deserialize)]
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

fn main() -> Result<()> {
    const SOURCE_PATH: &str = "assets/themes/src/vscode";
    const OUT_PATH: &str = "crates/theme2/src/themes";

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let themes_output_path = PathBuf::from_str(OUT_PATH)?;

    if !themes_output_path.exists() {
        println!("Creating directory: {:?}", themes_output_path);
        fs::create_dir_all(&themes_output_path)?;
    }

    // We create mod.rs at the beginning to prevent `mod themes;`/`pub use themes::*;` from being
    // invalid in the theme crate root.
    println!(
        "Creating file: {:?}",
        themes_output_path.join(format!("mod.rs"))
    );

    let mut mod_rs_file = File::create(themes_output_path.join(format!("mod.rs")))?;

    println!("Loading themes source...");
    let vscode_themes_path = PathBuf::from_str(SOURCE_PATH)?;
    if !vscode_themes_path.exists() {
        return Err(anyhow!(format!(
            "Couldn't find {}, make sure it exists",
            SOURCE_PATH
        )));
    }

    let mut theme_families = Vec::new();

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
            println!("Skipping theme family '{}' because it does not have a LICENSE file. This theme will only be imported once a LICENSE file is provided.", theme_family_slug);
            continue;
        }

        let family_metadata: FamilyMetadata = serde_json::from_reader(family_metadata_file)
            .context(format!(
                "failed to parse `family.json` for '{theme_family_slug}'"
            ))?;

        let mut themes = Vec::new();

        for theme_metadata in family_metadata.themes {
            let theme_file_path = theme_family_dir.path().join(&theme_metadata.file_name);

            let theme_file = match File::open(&theme_file_path) {
                Ok(file) => file,
                Err(_) => {
                    println!("Failed to open file at path: {:?}", theme_file_path);
                    continue;
                }
            };

            let vscode_theme: VsCodeTheme = serde_json::from_reader(theme_file)
                .context(format!("failed to parse theme {theme_file_path:?}"))?;

            let converter = VsCodeThemeConverter::new(vscode_theme, theme_metadata);

            let theme = converter.convert()?;

            themes.push(theme);
        }

        let theme_family = ThemeFamily {
            id: uuid::Uuid::new_v4().to_string(),
            name: family_metadata.name.into(),
            author: family_metadata.author.into(),
            themes,
            scales: default_color_scales(),
        };

        theme_families.push(theme_family);
    }

    let mut theme_modules = Vec::new();

    for theme_family in theme_families {
        let theme_family_slug = theme_family.name.to_string().to_case(Case::Snake);

        let mut output_file =
            File::create(themes_output_path.join(format!("{theme_family_slug}.rs")))?;
        println!(
            "Creating file: {:?}",
            themes_output_path.join(format!("{theme_family_slug}.rs"))
        );

        let theme_module = format!(
            r#"
            use gpui::rgba;

            use crate::{{
                default_color_scales, Appearance, GitStatusColors, PlayerColor, PlayerColors, StatusColors,
                SyntaxTheme, SystemColors, ThemeColors, ThemeFamily, ThemeStyles, ThemeVariant,
            }};

            pub fn {theme_family_slug}() -> ThemeFamily {{
                {theme_family_definition}
            }}
            "#,
            theme_family_definition = format!("{:#?}", ThemeFamilyPrinter::new(theme_family))
        );

        output_file.write_all(theme_module.as_bytes())?;

        theme_modules.push(theme_family_slug);
    }

    let themes_vector_contents = format!(
        r#"
        use crate::ThemeFamily;

        pub(crate) fn all_imported_themes() -> Vec<ThemeFamily> {{
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

    Ok(())
}
