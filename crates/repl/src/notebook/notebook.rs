#![allow(unused)]
use collections::HashMap;
use gpui::View;
use serde::{Deserialize, Serialize};

use crate::notebook::DeserializedCell;

use super::{Cell, CellId};

pub(crate) const DEFAULT_NOTEBOOK_FORMAT: i32 = 4;
pub(crate) const DEFAULT_NOTEBOOK_FORMAT_MINOR: i32 = 0;

pub struct Notebook {
    metadata: DeserializedMetadata,
    nbformat: i32,
    nbformat_minor: i32,
    cells: HashMap<CellId, View<Cell>>,
}

impl Default for Notebook {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            nbformat: DEFAULT_NOTEBOOK_FORMAT,
            nbformat_minor: DEFAULT_NOTEBOOK_FORMAT_MINOR,
            cells: HashMap::default(),
        }
    }
}

impl Default for DeserializedMetadata {
    fn default() -> Self {
        Self {
            kernelspec: None,
            language_info: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedNotebook {
    pub metadata: DeserializedMetadata,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    pub cells: Vec<DeserializedCell>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedMetadata {
    kernelspec: Option<DeserializedKernelSpec>,
    language_info: Option<DeserializedLanguageInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedKernelSpec {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedLanguageInfo {
    name: String,
    version: Option<String>,
    #[serde(default)]
    codemirror_mode: Option<CodemirrorMode>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CodemirrorMode {
    String(String),
    Object(serde_json::Value),
}

impl Default for CodemirrorMode {
    fn default() -> Self {
        CodemirrorMode::String(String::new())
    }
}

pub fn deserialize_notebook(notebook: &str) -> Result<DeserializedNotebook, serde_json::Error> {
    match serde_json::from_str(notebook) {
        Ok(deserialized) => Ok(deserialized),
        Err(e) => {
            eprintln!("Error deserializing notebook: {:?}", e);
            eprintln!("Error occurs at line {}, column {}", e.line(), e.column());
            eprintln!(
                "Nearby JSON: {}",
                &notebook[e.column().saturating_sub(20)..e.column().saturating_add(20)]
            );
            Err(e)
        }
    }
}
