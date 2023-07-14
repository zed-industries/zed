use std::{path::Path, str, sync::Arc};

use collections::HashMap;

use gpui::{AppContext, AssetSource};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct TypeConfig {
    icon: Arc<str>,
}

#[derive(Deserialize, Debug)]
pub struct FileAssociations {
    suffixes: HashMap<String, String>,
    types: HashMap<String, TypeConfig>,
}

pub const TEXT_FILE_ASSET: &'static str = "icons/file_icons/quill/file.svg";
const DIRECTORY_TYPE: &'static str = "directory";
const EXPANDED_DIRECTORY_TYPE: &'static str = "expanded_directory";

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    cx.set_global(FileAssociations::new(assets))
}

impl FileAssociations {
    pub fn new(assets: impl AssetSource) -> Self {
        assets
            .load("icons/file_icons/file_types.json")
            .map(|file| {
                serde_json::from_str::<FileAssociations>(str::from_utf8(&file).unwrap()).unwrap()
            })
            .unwrap_or_else(|_| FileAssociations {
                suffixes: HashMap::default(),
                types: HashMap::default(),
            })
    }

    pub fn get_icon(path: &Path, cx: &AppContext) -> Option<Arc<str>> {
        if !cx.has_global::<Self>() {
            return None;
        }

        let this = cx.global::<Self>();
        let suffix = path.extension()?.to_str()?;

        this.suffixes
            .get(suffix)
            .and_then(|type_str| this.types.get(type_str))
            .map(|type_config| type_config.icon.clone())
    }

    pub fn get_folder_icon(expanded: bool, cx: &AppContext) -> Option<Arc<str>> {
        if !cx.has_global::<Self>() {
            return None;
        }

        let this = cx.global::<Self>();

        let key = if expanded {
            EXPANDED_DIRECTORY_TYPE
        } else {
            DIRECTORY_TYPE
        };

        this.types
            .get(key)
            .map(|type_config| type_config.icon.clone())
    }
}
