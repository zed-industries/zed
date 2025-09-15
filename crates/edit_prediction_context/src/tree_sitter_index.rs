use anyhow::Result;
use anyhow::anyhow;
use ignore::Walk;
use multimap::MultiMap;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::outline::OutlineId;
use crate::treesitter_util::language_for_file;
use crate::zed_code::LanguageName;
use crate::{
    outline::{OutlineItem, query_outline_items},
    treesitter_util::parse_source,
    zed_code::Language,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct Identifier(pub Arc<str>);

#[derive(Debug)]
pub struct IdentifierIndex {
    pub identifier_to_definitions:
        HashMap<(Identifier, LanguageName), MultiMap<Arc<Path>, OutlineItem>>,
    pub path_to_source: HashMap<Arc<Path>, String>,
    pub path_to_items: HashMap<Arc<Path>, Vec<OutlineItem>>,
    pub outline_id_to_item: HashMap<OutlineId, OutlineItem>,
}

impl IdentifierIndex {
    pub fn index_path(languages: &[Arc<Language>], path: &Path) -> Result<IdentifierIndex> {
        let mut identifier_to_definitions = HashMap::new();
        let mut path_to_source = HashMap::new();
        let mut path_to_items = HashMap::new();
        let mut outline_id_to_item = HashMap::new();

        for entry in Walk::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.metadata().unwrap().is_file())
        {
            let file_path = entry.path();
            let Some(language) = language_for_file(languages, file_path) else {
                continue;
            };
            if !language.supports_references {
                continue;
            }
            let source = fs::read_to_string(file_path)
                .map_err(|e| anyhow!("Failed to read file {:?}: {}", file_path, e))?;
            let tree = parse_source(&language, &source);

            let mut outline_items = query_outline_items(&language, &tree, &source);
            outline_items.sort_by_key(|item| item.item_range.start);
            for outline_item in outline_items.iter() {
                let identifier = Identifier(outline_item.name(&source).into());
                let definitions: &mut MultiMap<Arc<Path>, OutlineItem> = identifier_to_definitions
                    .entry((identifier, language.name.clone()))
                    .or_default();
                definitions.insert(file_path.into(), outline_item.clone());
                outline_id_to_item.insert(outline_item.id, outline_item.clone());
            }
            path_to_source.insert(file_path.into(), source);
            path_to_items.insert(file_path.into(), outline_items);
        }

        Ok(IdentifierIndex {
            identifier_to_definitions,
            path_to_source,
            path_to_items,
            outline_id_to_item,
        })
    }
}
