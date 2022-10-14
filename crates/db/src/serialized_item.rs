use std::path::PathBuf;

use anyhow::Result;

use super::Db;

impl Db {}

#[derive(PartialEq, Eq, Hash)]
pub enum SerializedItemKind {
    Editor,
    Terminal,
    ProjectSearch,
    Diagnostics,
}

pub enum SerializedItem {
    Editor(PathBuf, String),
    Terminal,
    ProjectSearch(String),
    Diagnostics,
}
