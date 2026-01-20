use std::{path::Path, sync::Arc};

use anyhow::Result;
use collections::HashMap;
use gpui::{App, Global, ReadGlobal, UpdateGlobal};
use parking_lot::RwLock;

use crate::{Snippet, SnippetKind, file_stem_to_key};

struct GlobalSnippetRegistry(Arc<SnippetRegistry>);

impl Global for GlobalSnippetRegistry {}

#[derive(Default)]
pub struct SnippetRegistry {
    snippets: RwLock<HashMap<SnippetKind, Vec<Arc<Snippet>>>>,
}

impl SnippetRegistry {
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalSnippetRegistry::global(cx).0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Arc<Self>> {
        cx.try_global::<GlobalSnippetRegistry>()
            .map(|registry| registry.0.clone())
    }

    pub fn init_global(cx: &mut App) {
        GlobalSnippetRegistry::set_global(cx, GlobalSnippetRegistry(Arc::new(Self::new())))
    }

    pub fn new() -> Self {
        Self {
            snippets: RwLock::new(HashMap::default()),
        }
    }

    pub fn register_snippets(&self, file_path: &Path, contents: &str) -> Result<()> {
        let snippets_in_file: crate::format::VsSnippetsFile =
            serde_json_lenient::from_str(contents)?;
        let kind = file_path
            .file_stem()
            .and_then(|stem| stem.to_str().and_then(file_stem_to_key));
        let snippets = crate::file_to_snippets(snippets_in_file, file_path);
        self.snippets.write().insert(kind, snippets);

        Ok(())
    }

    pub fn get_snippets(&self, kind: &SnippetKind) -> Vec<Arc<Snippet>> {
        self.snippets.read().get(kind).cloned().unwrap_or_default()
    }
}
