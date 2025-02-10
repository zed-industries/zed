use std::sync::Arc;
use std::{path::Path, str};

use collections::HashMap;

use gpui::{App, AssetSource, Global, SharedString};
use serde_derive::Deserialize;
use settings::Settings;
use theme::{IconTheme, ThemeRegistry, ThemeSettings};
use util::paths::PathExt;

#[derive(Deserialize, Debug)]
pub struct FileIcons {
    stems: HashMap<String, String>,
    suffixes: HashMap<String, String>,
}

impl Global for FileIcons {}

pub const FILE_TYPES_ASSET: &str = "icons/file_icons/file_types.json";

pub fn init(assets: impl AssetSource, cx: &mut App) {
    cx.set_global(FileIcons::new(assets))
}

impl FileIcons {
    pub fn get(cx: &App) -> &Self {
        cx.global::<FileIcons>()
    }

    pub fn new(assets: impl AssetSource) -> Self {
        assets
            .load(FILE_TYPES_ASSET)
            .ok()
            .flatten()
            .and_then(|file| serde_json::from_str::<FileIcons>(str::from_utf8(&file).unwrap()).ok())
            .unwrap_or_else(|| FileIcons {
                stems: HashMap::default(),
                suffixes: HashMap::default(),
            })
    }

    pub fn get_icon(path: &Path, cx: &App) -> Option<SharedString> {
        let this = cx.try_global::<Self>()?;

        let get_icon_from_suffix = |suffix: &str| -> Option<SharedString> {
            this.stems
                .get(suffix)
                .or_else(|| this.suffixes.get(suffix))
                .and_then(|typ| this.get_icon_for_type(typ, cx))
        };
        // TODO: Associate a type with the languages and have the file's language
        //       override these associations

        // check if file name is in suffixes
        // e.g. catch file named `eslint.config.js` instead of `.eslint.config.js`
        if let Some(typ) = path.file_name().and_then(|typ| typ.to_str()) {
            let maybe_path = get_icon_from_suffix(typ);
            if maybe_path.is_some() {
                return maybe_path;
            }
        }

        // primary case: check if the files extension or the hidden file name
        // matches some icon path
        if let Some(suffix) = path.extension_or_hidden_file_name() {
            let maybe_path = get_icon_from_suffix(suffix);
            if maybe_path.is_some() {
                return maybe_path;
            }
        }

        // this _should_ only happen when the file is hidden (has leading '.')
        // and is not a "special" file we have an icon (e.g. not `.eslint.config.js`)
        // that should be caught above. In the remaining cases, we want to check
        // for a normal supported extension e.g. `.data.json` -> `json`
        let extension = path.extension().and_then(|ext| ext.to_str());
        if let Some(extension) = extension {
            let maybe_path = get_icon_from_suffix(extension);
            if maybe_path.is_some() {
                return maybe_path;
            }
        }
        return this.get_icon_for_type("default", cx);
    }

    fn default_icon_theme(cx: &App) -> Option<Arc<IconTheme>> {
        let theme_registry = ThemeRegistry::global(cx);
        theme_registry.default_icon_theme().ok()
    }

    pub fn get_icon_for_type(&self, typ: &str, cx: &App) -> Option<SharedString> {
        fn get_icon_for_type(icon_theme: &Arc<IconTheme>, typ: &str) -> Option<SharedString> {
            icon_theme
                .file_icons
                .get(typ)
                .map(|icon_definition| icon_definition.path.clone())
        }

        get_icon_for_type(&ThemeSettings::get_global(cx).active_icon_theme, typ).or_else(|| {
            Self::default_icon_theme(cx).and_then(|icon_theme| get_icon_for_type(&icon_theme, typ))
        })
    }

    pub fn get_folder_icon(expanded: bool, cx: &App) -> Option<SharedString> {
        fn get_folder_icon(icon_theme: &Arc<IconTheme>, expanded: bool) -> Option<SharedString> {
            if expanded {
                icon_theme.directory_icons.expanded.clone()
            } else {
                icon_theme.directory_icons.collapsed.clone()
            }
        }

        get_folder_icon(&ThemeSettings::get_global(cx).active_icon_theme, expanded).or_else(|| {
            Self::default_icon_theme(cx)
                .and_then(|icon_theme| get_folder_icon(&icon_theme, expanded))
        })
    }

    pub fn get_chevron_icon(expanded: bool, cx: &App) -> Option<SharedString> {
        fn get_chevron_icon(icon_theme: &Arc<IconTheme>, expanded: bool) -> Option<SharedString> {
            if expanded {
                icon_theme.chevron_icons.expanded.clone()
            } else {
                icon_theme.chevron_icons.collapsed.clone()
            }
        }

        get_chevron_icon(&ThemeSettings::get_global(cx).active_icon_theme, expanded).or_else(|| {
            Self::default_icon_theme(cx)
                .and_then(|icon_theme| get_chevron_icon(&icon_theme, expanded))
        })
    }
}
