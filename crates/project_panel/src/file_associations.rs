use std::{path::Path, str, sync::Arc};

use collections::HashMap;

use gpui::{AppContext, AssetSource};
use serde_derive::Deserialize;
use util::iife;

#[derive(Deserialize, Debug)]
struct TypeConfig {
    icon: Arc<str>,
}

#[derive(Deserialize, Debug)]
pub struct FileAssociations {
    suffixes: HashMap<String, String>,
    types: HashMap<String, TypeConfig>,
}

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
                suffixes: HashMap::default(),
                types: HashMap::default(),
            })
    }

    pub fn get_icon(path: &Path, cx: &AppContext) -> Arc<str> {
        iife!({
            let this = cx.has_global::<Self>().then(|| cx.global::<Self>())?;

            // FIXME: Associate a type with the languages and have the file's langauge
            //        override these associations
            iife!({
                let suffix = path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .and_then(|file_name| {
                        file_name
                            .find('.')
                            .and_then(|dot_index| file_name.get(dot_index + 1..))
                    })?;

                this.suffixes
                    .get(suffix)
                    .and_then(|type_str| this.types.get(type_str))
                    .map(|type_config| type_config.icon.clone())
            })
            .or_else(|| this.types.get("default").map(|config| config.icon.clone()))
        })
        .unwrap_or_else(|| Arc::from("".to_string()))
    }

    pub fn get_folder_icon(expanded: bool, cx: &AppContext) -> Arc<str> {
        iife!({
            let this = cx.has_global::<Self>().then(|| cx.global::<Self>())?;

            let key = if expanded {
                EXPANDED_DIRECTORY_TYPE
            } else {
                COLLAPSED_DIRECTORY_TYPE
            };

            this.types
                .get(key)
                .map(|type_config| type_config.icon.clone())
        })
        .unwrap_or_else(|| Arc::from("".to_string()))
    }

    pub fn get_chevron_icon(expanded: bool, cx: &AppContext) -> Arc<str> {
        iife!({
            let this = cx.has_global::<Self>().then(|| cx.global::<Self>())?;

            let key = if expanded {
                EXPANDED_CHEVRON_TYPE
            } else {
                COLLAPSED_CHEVRON_TYPE
            };

            this.types
                .get(key)
                .map(|type_config| type_config.icon.clone())
        })
        .unwrap_or_else(|| Arc::from("".to_string()))
    }
}
