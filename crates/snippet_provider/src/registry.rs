use std::{path::Path, sync::Arc};

use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, Global, ReadGlobal, UpdateGlobal};
use parking_lot::RwLock;

use crate::{file_stem_to_key, Snippet, SnippetKind};

struct GlobalSnippetRegistry(Arc<SnippetRegistry>);

impl Global for GlobalSnippetRegistry {}

#[derive(Default)]
pub struct SnippetRegistry {
    snippets: RwLock<HashMap<SnippetKind, Vec<Arc<Snippet>>>>,
}

impl SnippetRegistry {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalSnippetRegistry::global(cx).0.clone()
    }

    pub fn try_global(cx: &AppContext) -> Option<Arc<Self>> {
        cx.try_global::<GlobalSnippetRegistry>()
            .map(|registry| registry.0.clone())
    }

    pub fn init_global(cx: &mut AppContext) {
        GlobalSnippetRegistry::set_global(cx, GlobalSnippetRegistry(Arc::new(Self::new())))
    }

    pub fn new() -> Self {
        Self {
            snippets: RwLock::new(HashMap::default()),
        }
    }

    pub fn register_snippets(&self, file_path: &Path, contents: &str) -> Result<()> {
        let snippets_in_file: crate::format::VSSnippetsFile = serde_json::from_str(contents)?;
        let kind = file_path
            .file_stem()
            .and_then(|stem| stem.to_str().and_then(file_stem_to_key));
        let snippets = crate::file_to_snippets(snippets_in_file);
        self.snippets.write().insert(kind, snippets);

        Ok(())
    }

    pub fn get_snippets(&self, kind: &SnippetKind) -> Vec<Arc<Snippet>> {
        self.snippets.read().get(kind).cloned().unwrap_or_default()
    }
}
