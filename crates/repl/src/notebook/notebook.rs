#![allow(unused)]
use collections::HashMap;
use gpui::View;
use serde::{Deserialize, Serialize};

use crate::notebook::DeserializedCell;

use super::{Cell, CellId};

pub(crate) const DEFAULT_NOTEBOOK_FORMAT: i32 = 4;
pub(crate) const DEFAULT_NOTEBOOK_FORMAT_MINOR: i32 = 0;

pub struct NotebookData {
    metadata: DeserializedMetadata,
    nbformat: i32,
    nbformat_minor: i32,
    cells: HashMap<CellId, View<Cell>>,
}

impl NotebookData {}

impl Default for NotebookData {
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
    metadata: DeserializedMetadata,
    nbformat: i32,
    nbformat_minor: i32,
    cells: Vec<DeserializedCell>,
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
    // Zed can ignore this field, only writing to it for compatibility
    codemirror_mode: Option<String>,
}

fn deserialize_notebook(notebook: &str) -> Result<DeserializedNotebook, serde_json::Error> {
    serde_json::from_str(notebook)
}
