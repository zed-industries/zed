use gpui::Rgba;
use indoc::indoc;
use serde_derive::Deserialize;
use theme2::{FabricSurface, FabricSurfaceState, FabricTheme};

fn main() {
    use std::fs::{self, DirEntry};
    use std::path::Path;

    let legacy_themes_path = Path::new(env!("PWD")).join("crates/theme2/legacy_themes");
    dbg!(&legacy_themes_path);
    if legacy_themes_path.exists() {
        let legacy_theme_files =
            fs::read_dir(legacy_themes_path).expect("Failed to read legacy themes directory");
        let mut mods = Vec::new();

        for entry in legacy_theme_files {
            let entry: DirEntry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let theme_json =
                    fs::read_to_string(&path).expect(&format!("Failed to read {:?}", path));
                let legacy_theme: LegacyTheme = serde_json::from_str(&theme_json).expect(&format!(
                    "Failed to parse JSON to LegacyTheme from {:?}",
                    path
                ));

                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let mod_name = format!(
                    "{}",
                    name.replace(" ", "_").to_lowercase().replace("é", "e") // Hack for Rosé Pine
                );
                mods.push(mod_name.clone());

                let theme = FabricTheme {
                    name,

                    cotton: FabricSurface {
                        default: FabricSurfaceState {
                            background: legacy_theme.middle.base.default.background,
                            border: legacy_theme.middle.base.default.border,
                            foreground: legacy_theme.middle.base.default.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.default.foreground,
                            ),
                        },
                        hovered: FabricSurfaceState {
                            background: legacy_theme.middle.base.hovered.background,
                            border: legacy_theme.middle.base.hovered.border,
                            foreground: legacy_theme.middle.base.hovered.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.hovered.foreground,
                            ),
                        },
                        pressed: FabricSurfaceState {
                            background: legacy_theme.middle.base.pressed.background,
                            border: legacy_theme.middle.base.pressed.border,
                            foreground: legacy_theme.middle.base.pressed.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.pressed.foreground,
                            ),
                        },
                        active: FabricSurfaceState {
                            background: legacy_theme.middle.base.active.background,
                            border: legacy_theme.middle.base.active.border,
                            foreground: legacy_theme.middle.base.active.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.active.foreground,
                            ),
                        },
                        disabled: FabricSurfaceState {
                            background: legacy_theme.middle.base.disabled.background,
                            border: legacy_theme.middle.base.disabled.border,
                            foreground: legacy_theme.middle.base.disabled.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.disabled.foreground,
                            ),
                        },
                        inverted: FabricSurfaceState {
                            background: legacy_theme.middle.base.inverted.background,
                            border: legacy_theme.middle.base.inverted.border,
                            foreground: legacy_theme.middle.base.inverted.foreground,
                            secondary_foreground: Some(
                                legacy_theme.middle.variant.inverted.foreground,
                            ),
                        },
                    },
                    linen: FabricSurface::from(legacy_theme.lowest.on.clone()),
                    denim: FabricSurface {
                        default: FabricSurfaceState {
                            background: legacy_theme.lowest.base.default.background,
                            border: legacy_theme.lowest.base.default.border,
                            foreground: legacy_theme.lowest.base.default.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.default.foreground,
                            ),
                        },
                        hovered: FabricSurfaceState {
                            background: legacy_theme.lowest.base.hovered.background,
                            border: legacy_theme.lowest.base.hovered.border,
                            foreground: legacy_theme.lowest.base.hovered.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.hovered.foreground,
                            ),
                        },
                        pressed: FabricSurfaceState {
                            background: legacy_theme.lowest.base.pressed.background,
                            border: legacy_theme.lowest.base.pressed.border,
                            foreground: legacy_theme.lowest.base.pressed.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.pressed.foreground,
                            ),
                        },
                        active: FabricSurfaceState {
                            background: legacy_theme.lowest.base.active.background,
                            border: legacy_theme.lowest.base.active.border,
                            foreground: legacy_theme.lowest.base.active.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.active.foreground,
                            ),
                        },
                        disabled: FabricSurfaceState {
                            background: legacy_theme.lowest.base.disabled.background,
                            border: legacy_theme.lowest.base.disabled.border,
                            foreground: legacy_theme.lowest.base.disabled.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.disabled.foreground,
                            ),
                        },
                        inverted: FabricSurfaceState {
                            background: legacy_theme.lowest.base.inverted.background,
                            border: legacy_theme.lowest.base.inverted.border,
                            foreground: legacy_theme.lowest.base.inverted.foreground,
                            secondary_foreground: Some(
                                legacy_theme.lowest.variant.inverted.foreground,
                            ),
                        },
                    }, // Assuming silk maps to 'on' at middle elevation
                    silk: FabricSurface::from(legacy_theme.middle.on.clone()),
                    satin: FabricSurface::from(legacy_theme.lowest.accent.clone()),
                    positive: FabricSurface::from(legacy_theme.lowest.positive.clone()),
                    warning: FabricSurface::from(legacy_theme.lowest.warning.clone()),
                    negative: FabricSurface::from(legacy_theme.lowest.negative.clone()),
                };

                let indented_theme = format!("{:#?}", theme)
                    .lines()
                    .map(|line| format!("    {}", line))
                    .collect::<Vec<String>>()
                    .join("\n");

                let module_source = format!(
                    indoc! {r#"
                        use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
                        use gpui::rgba;

                        pub fn {}() -> FabricTheme {{
                        {}
                        }}
                    "#},
                    mod_name, indented_theme,
                );

                let module_path = Path::new(env!("PWD"))
                    .join(format!("crates/theme2/src/fabric_themes/{}.rs", mod_name));
                fs::write(&module_path, module_source)
                    .expect(&format!("Failed to write to {:?}", module_path));
                println!("Wrote FabricTheme to file {:?}", module_path);
            }
        }

        let mod_rs_path = Path::new(env!("PWD")).join("crates/theme2/src/fabric_themes/mod.rs");
        let mut mod_file_content = String::new();

        for mod_name in mods.iter() {
            mod_file_content.push_str(&format!("pub mod {};\n", mod_name));
        }
        mod_file_content.push_str("\n");
        for mod_name in mods.iter() {
            mod_file_content.push_str(&format!("pub use {}::{};\n", mod_name, mod_name));
        }

        fs::write(&mod_rs_path, mod_file_content)
            .expect(&format!("Failed to write to {:?}", mod_rs_path));
        println!("Wrote module declarations to file {:?}", mod_rs_path);
    } else {
        eprintln!("Legacy themes directory does not exist");
    }
}

impl From<LegacySurface> for FabricSurface {
    fn from(legacy: LegacySurface) -> Self {
        FabricSurface {
            default: FabricSurfaceState {
                background: legacy.default.background,
                border: legacy.default.border,
                foreground: legacy.default.foreground,
                secondary_foreground: None, // Assuming no secondary_foreground in LegacySurface
            },
            hovered: FabricSurfaceState {
                background: legacy.hovered.background,
                border: legacy.hovered.border,
                foreground: legacy.hovered.foreground,
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: legacy.pressed.background,
                border: legacy.pressed.border,
                foreground: legacy.pressed.foreground,
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: legacy.active.background,
                border: legacy.active.border,
                foreground: legacy.active.foreground,
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: legacy.disabled.background,
                border: legacy.disabled.border,
                foreground: legacy.disabled.foreground,
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: legacy.inverted.background,
                border: legacy.inverted.border,
                foreground: legacy.inverted.foreground,
                secondary_foreground: None,
            },
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LegacySurfaceState {
    background: Rgba,
    border: Rgba,
    foreground: Rgba,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LegacySurface {
    default: LegacySurfaceState,
    hovered: LegacySurfaceState,
    pressed: LegacySurfaceState,
    active: LegacySurfaceState,
    disabled: LegacySurfaceState,
    inverted: LegacySurfaceState,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LegacyElevation {
    base: LegacySurface,
    variant: LegacySurface,
    on: LegacySurface,
    accent: LegacySurface,
    positive: LegacySurface,
    warning: LegacySurface,
    negative: LegacySurface,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LegacyTheme {
    lowest: LegacyElevation,
    middle: LegacyElevation,
    highest: LegacyElevation,
}
