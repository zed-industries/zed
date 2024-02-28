use std::{path::Path, str, sync::Arc};

use collections::HashMap;

use gpui::{AppContext, AssetSource, Global};
use serde_derive::Deserialize;
use util::{maybe, paths::PathExt};

#[derive(Deserialize, Debug)]
struct TypeConfig {
    icon: Arc<str>,
}

#[derive(Deserialize, Debug)]
pub struct FileAssociations {
    names: HashMap<String, String>,
    suffixes: HashMap<String, String>,
    types: HashMap<String, TypeConfig>,
}

impl Global for FileAssociations {}

const COLLAPSED_DIRECTORY_TYPE: &'static str = "collapsed_folder";
const EXPANDED_DIRECTORY_TYPE: &'static str = "expanded_folder";
const COLLAPSED_CHEVRON_TYPE: &'static str = "collapsed_chevron";
const EXPANDED_CHEVRON_TYPE: &'static str = "expanded_chevron";
pub const FILE_TYPES_ASSET: &'static str = "icons/file_icons/file_types.json";

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    cx.set_global(FileAssociations::new(assets))
}

impl FileAssociations {
    pub fn new(assets: impl AssetSource) -> Self {
        assets
            .load("icons/file_icons/file_types.json")
            .and_then(|file| {
                serde_json::from_str::<FileAssociations>(str::from_utf8(&file).unwrap())
                    .map_err(Into::into)
            })
            .unwrap_or_else(|_| FileAssociations {
                names: HashMap::default(),
                suffixes: HashMap::default(),
                types: HashMap::default(),
            })
    }

    pub fn get_icon(path: &Path, cx: &AppContext) -> Option<Arc<str>> {
        let this = cx.try_global::<Self>()?;

        // FIXME: Associate a type with the languages and have the file's language
        //        override these associations

        // First, try to find an icon based on the file name
        let name = path.file_name()?.to_str();
        let icon_from_name = name.and_then(|name_str| {
            this.names
                .get(name_str)
                .and_then(|type_str| this.types.get(type_str))
                .map(|type_config| type_config.icon.clone())
        });

        // If no icon is found based on the file name, try to find an icon based on the file extension
        let suffix = path.icon_suffix();
        let icon_from_suffix = suffix.and_then(|suffix_str| {
            this.suffixes
                .get(suffix_str)
                .and_then(|type_str| this.types.get(type_str))
                .map(|type_config| type_config.icon.clone())
        });

        // Return the icon found based on the file name or extension, or fallback to default icon
        icon_from_name.or(icon_from_suffix).or_else(|| this.types.get("default").map(|config| config.icon.clone()))
    }


    pub fn get_folder_icon(expanded: bool, cx: &AppContext) -> Option<Arc<str>> {
        let this = cx.try_global::<Self>()?;

        let key = if expanded {
            EXPANDED_DIRECTORY_TYPE
        } else {
            COLLAPSED_DIRECTORY_TYPE
        };

        this.types
            .get(key)
            .map(|type_config| type_config.icon.clone())
    }

    pub fn get_chevron_icon(expanded: bool, cx: &AppContext) -> Option<Arc<str>> {
        let this = cx.try_global::<Self>()?;

        let key = if expanded {
            EXPANDED_CHEVRON_TYPE
        } else {
            COLLAPSED_CHEVRON_TYPE
        };

        this.types
            .get(key)
            .map(|type_config| type_config.icon.clone())
    }
}
