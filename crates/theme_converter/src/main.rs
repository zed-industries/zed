mod theme_printer;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Debug};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use gpui2::{hsla, rgb, serde_json, AssetSource, Hsla, SharedString};
use log::LevelFilter;
use rust_embed::RustEmbed;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use simplelog::SimpleLogger;
use theme2::{PlayerTheme, SyntaxTheme};

use crate::theme_printer::ThemePrinter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the theme to convert.
    theme: String,
}

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let args = Args::parse();

    let (json_theme, legacy_theme) = load_theme(args.theme)?;

    let theme = convert_theme(json_theme, legacy_theme)?;

    println!("{:#?}", ThemePrinter::new(theme));

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "themes/**/*"]
#[include = "sounds/**/*"]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl PlayerThemeColors {
    pub fn new(theme: &LegacyTheme, ix: usize) -> Self {
        if ix < theme.players.len() {
            Self {
                cursor: theme.players[ix].cursor,
                selection: theme.players[ix].selection,
            }
        } else {
            Self {
                cursor: rgb::<Hsla>(0xff00ff),
                selection: rgb::<Hsla>(0xff00ff),
            }
        }
    }
}

impl From<PlayerThemeColors> for PlayerTheme {
    fn from(value: PlayerThemeColors) -> Self {
        Self {
            cursor: value.cursor,
            selection: value.selection,
        }
    }
}

fn convert_theme(json_theme: JsonTheme, legacy_theme: LegacyTheme) -> Result<theme2::Theme> {
    let transparent = hsla(0.0, 0.0, 0.0, 0.0);

    let players: [PlayerTheme; 8] = [
        PlayerThemeColors::new(&legacy_theme, 0).into(),
        PlayerThemeColors::new(&legacy_theme, 1).into(),
        PlayerThemeColors::new(&legacy_theme, 2).into(),
        PlayerThemeColors::new(&legacy_theme, 3).into(),
        PlayerThemeColors::new(&legacy_theme, 4).into(),
        PlayerThemeColors::new(&legacy_theme, 5).into(),
        PlayerThemeColors::new(&legacy_theme, 6).into(),
        PlayerThemeColors::new(&legacy_theme, 7).into(),
    ];

    let theme = theme2::Theme {
        metadata: theme2::ThemeMetadata {
            name: legacy_theme.name.clone().into(),
            is_light: legacy_theme.is_light,
        },
        transparent,
        mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
        mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
        mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
        border: legacy_theme.lowest.base.default.border,
        border_variant: legacy_theme.lowest.variant.default.border,
        border_focused: legacy_theme.lowest.accent.default.border,
        border_transparent: transparent,
        elevated_surface: legacy_theme.lowest.base.default.background,
        surface: legacy_theme.middle.base.default.background,
        background: legacy_theme.lowest.base.default.background,
        filled_element: legacy_theme.lowest.base.default.background,
        filled_element_hover: hsla(0.0, 0.0, 100.0, 0.12),
        filled_element_active: hsla(0.0, 0.0, 100.0, 0.16),
        filled_element_selected: legacy_theme.lowest.accent.default.background,
        filled_element_disabled: transparent,
        ghost_element: transparent,
        ghost_element_hover: hsla(0.0, 0.0, 100.0, 0.08),
        ghost_element_active: hsla(0.0, 0.0, 100.0, 0.12),
        ghost_element_selected: legacy_theme.lowest.accent.default.background,
        ghost_element_disabled: transparent,
        text: legacy_theme.lowest.base.default.foreground,
        text_muted: legacy_theme.lowest.variant.default.foreground,
        /// TODO: map this to a real value
        text_placeholder: legacy_theme.lowest.negative.default.foreground,
        text_disabled: legacy_theme.lowest.base.disabled.foreground,
        text_accent: legacy_theme.lowest.accent.default.foreground,
        icon_muted: legacy_theme.lowest.variant.default.foreground,
        syntax: SyntaxTheme {
            highlights: json_theme
                .editor
                .syntax
                .iter()
                .map(|(token, style)| (token.clone(), style.color.clone().into()))
                .collect(),
        },
        status_bar: legacy_theme.lowest.base.default.background,
        title_bar: legacy_theme.lowest.base.default.background,
        toolbar: legacy_theme.highest.base.default.background,
        tab_bar: legacy_theme.middle.base.default.background,
        editor: legacy_theme.highest.base.default.background,
        editor_subheader: legacy_theme.middle.base.default.background,
        terminal: legacy_theme.highest.base.default.background,
        editor_active_line: legacy_theme.highest.on.default.background,
        image_fallback_background: legacy_theme.lowest.base.default.background,

        git_created: legacy_theme.lowest.positive.default.foreground,
        git_modified: legacy_theme.lowest.accent.default.foreground,
        git_deleted: legacy_theme.lowest.negative.default.foreground,
        git_conflict: legacy_theme.lowest.warning.default.foreground,
        git_ignored: legacy_theme.lowest.base.disabled.foreground,
        git_renamed: legacy_theme.lowest.warning.default.foreground,

        players,
    };

    Ok(theme)
}

#[derive(Deserialize)]
struct JsonTheme {
    pub editor: JsonEditorTheme,
    pub base_theme: serde_json::Value,
}

#[derive(Deserialize)]
struct JsonEditorTheme {
    pub syntax: HashMap<String, JsonSyntaxStyle>,
}

#[derive(Deserialize)]
struct JsonSyntaxStyle {
    pub color: Hsla,
}

/// Loads the [`Theme`] with the given name.
fn load_theme(name: String) -> Result<(JsonTheme, LegacyTheme)> {
    let theme_contents = Assets::get(&format!("themes/{name}.json"))
        .with_context(|| format!("theme file not found: '{name}'"))?;

    let json_theme: JsonTheme = serde_json::from_str(std::str::from_utf8(&theme_contents.data)?)
        .context("failed to parse legacy theme")?;

    let legacy_theme: LegacyTheme = serde_json::from_value(json_theme.base_theme.clone())
        .context("failed to parse `base_theme`")?;

    Ok((json_theme, legacy_theme))
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct LegacyTheme {
    pub name: String,
    pub is_light: bool,
    pub lowest: Layer,
    pub middle: Layer,
    pub highest: Layer,
    pub popover_shadow: Shadow,
    pub modal_shadow: Shadow,
    #[serde(deserialize_with = "deserialize_player_colors")]
    pub players: Vec<PlayerColors>,
    #[serde(deserialize_with = "deserialize_syntax_colors")]
    pub syntax: HashMap<String, Hsla>,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Layer {
    pub base: StyleSet,
    pub variant: StyleSet,
    pub on: StyleSet,
    pub accent: StyleSet,
    pub positive: StyleSet,
    pub warning: StyleSet,
    pub negative: StyleSet,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct StyleSet {
    #[serde(rename = "default")]
    pub default: ContainerColors,
    pub hovered: ContainerColors,
    pub pressed: ContainerColors,
    pub active: ContainerColors,
    pub disabled: ContainerColors,
    pub inverted: ContainerColors,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct ContainerColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct PlayerColors {
    pub selection: Hsla,
    pub cursor: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Shadow {
    pub blur: u8,
    pub color: Hsla,
    pub offset: Vec<u8>,
}

fn deserialize_player_colors<'de, D>(deserializer: D) -> Result<Vec<PlayerColors>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PlayerArrayVisitor;

    impl<'de> Visitor<'de> for PlayerArrayVisitor {
        type Value = Vec<PlayerColors>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object with integer keys")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(
            self,
            mut map: A,
        ) -> Result<Self::Value, A::Error> {
            let mut players = Vec::with_capacity(8);
            while let Some((key, value)) = map.next_entry::<usize, PlayerColors>()? {
                if key < 8 {
                    players.push(value);
                } else {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(key as u64),
                        &"a key in range 0..7",
                    ));
                }
            }
            Ok(players)
        }
    }

    deserializer.deserialize_map(PlayerArrayVisitor)
}

fn deserialize_syntax_colors<'de, D>(deserializer: D) -> Result<HashMap<String, Hsla>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ColorWrapper {
        color: Hsla,
    }

    struct SyntaxVisitor;

    impl<'de> Visitor<'de> for SyntaxVisitor {
        type Value = HashMap<String, Hsla>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with keys and objects with a single color field as values")
        }

        fn visit_map<M>(self, mut map: M) -> Result<HashMap<String, Hsla>, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut result = HashMap::new();
            while let Some(key) = map.next_key()? {
                let wrapper: ColorWrapper = map.next_value()?; // Deserialize values as Hsla
                result.insert(key, wrapper.color);
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(SyntaxVisitor)
}
