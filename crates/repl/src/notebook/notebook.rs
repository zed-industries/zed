#![allow(unused)]
use anyhow::{Context, Result};
use collections::HashMap;
use gpui::View;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{deserialize_cells, Cell, CellId, DeserializedCell};

pub(crate) const DEFAULT_NOTEBOOK_FORMAT: i32 = 4;
pub(crate) const DEFAULT_NOTEBOOK_FORMAT_MINOR: i32 = 0;

impl Default for DeserializedMetadata {
    fn default() -> Self {
        Self {
            kernelspec: None,
            language_info: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeserializedNotebook {
    pub metadata: DeserializedMetadata,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    #[serde(deserialize_with = "deserialize_cells")]
    pub cells: Vec<DeserializedCell>,
}

#[derive(Deserialize, Debug)]
pub struct DeserializedMetadata {
    pub kernelspec: Option<DeserializedKernelSpec>,
    pub language_info: Option<DeserializedLanguageInfo>,
}

#[derive(Deserialize, Debug)]
pub struct DeserializedKernelSpec {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct DeserializedLanguageInfo {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub codemirror_mode: Option<CodemirrorMode>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CodemirrorMode {
    String(String),
    Object(Value),
}

impl Default for CodemirrorMode {
    fn default() -> Self {
        CodemirrorMode::String(String::new())
    }
}

pub fn deserialize_notebook(notebook: &str) -> Result<DeserializedNotebook> {
    let deserialized: DeserializedNotebook =
        serde_json::from_str(notebook).context("Failed to deserialize notebook")?;
    Ok(deserialized)
}
