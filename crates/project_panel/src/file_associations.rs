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
    stems: HashMap<String, String>,
    suffixes: HashMap<String, String>,
    types: HashMap<String, TypeConfig>,
}

impl Global for FileAssociations {}

const COLLAPSED_DIRECTORY_TYPE: &str = "collapsed_folder";
const EXPANDED_DIRECTORY_TYPE: &str = "expanded_folder";
const COLLAPSED_CHEVRON_TYPE: &str = "collapsed_chevron";
const EXPANDED_CHEVRON_TYPE: &str = "expanded_chevron";
pub const FILE_TYPES_ASSET: &str = "icons/file_icons/file_types.json";

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
                stems: HashMap::default(),
                suffixes: HashMap::default(),
                types: HashMap::default(),
            })
    }

    pub fn get_icon(path: &Path, cx: &AppContext) -> Option<Arc<str>> {
        let this = cx.try_global::<Self>()?;

        // FIXME: Associate a type with the languages and have the file's language
        //        override these associations
        maybe!({
            let suffix = path.icon_stem_or_suffix()?;

            if let Some(type_str) = this.stems.get(suffix) {
                return this
                    .types
                    .get(type_str)
                    .map(|type_config| type_config.icon.clone());
            }

            this.suffixes
                .get(suffix)
                .and_then(|type_str| this.types.get(type_str))
                .map(|type_config| type_config.icon.clone())
        })
        .or_else(|| this.types.get("default").map(|config| config.icon.clone()))
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
