mod format;
mod registry;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use collections::{BTreeMap, BTreeSet, HashMap};
use format::VSSnippetsFile;
use fs::Fs;
use futures::stream::StreamExt;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext, Task, WeakModel};
pub use registry::*;
use util::ResultExt;

pub fn init(cx: &mut AppContext) {
    SnippetRegistry::init_global(cx);
}

// Is `None` if the snippet file is global.
type SnippetKind = Option<String>;
fn file_stem_to_key(stem: &str) -> SnippetKind {
    if stem == "snippets" {
        None
    } else {
        Some(stem.to_owned())
    }
}

fn file_to_snippets(file_contents: VSSnippetsFile) -> Vec<Arc<Snippet>> {
    let mut snippets = vec![];
    for (prefix, snippet) in file_contents.snippets {
        let prefixes = snippet
            .prefix
            .map_or_else(move || vec![prefix], |prefixes| prefixes.into());
        let description = snippet
            .description
            .map(|description| description.to_string());
        let body = snippet.body.to_string();
        if snippet::Snippet::parse(&body).log_err().is_none() {
            continue;
        };
        snippets.push(Arc::new(Snippet {
            body,
            prefix: prefixes,
            description,
        }));
    }
    snippets
}
// Snippet with all of the metadata
#[derive(Debug)]
pub struct Snippet {
    pub prefix: Vec<String>,
    pub body: String,
    pub description: Option<String>,
}

async fn process_updates(
    this: WeakModel<SnippetProvider>,
    entries: Vec<PathBuf>,
    mut cx: AsyncAppContext,
) -> Result<()> {
    let fs = this.update(&mut cx, |this, _| this.fs.clone())?;
    for entry_path in entries {
        if !entry_path
            .extension()
            .map_or(false, |extension| extension == "json")
        {
            continue;
        }
        let entry_metadata = fs.metadata(&entry_path).await;
        // Entry could have been removed, in which case we should no longer show completions for it.
        let entry_exists = entry_metadata.is_ok();
        if entry_metadata.map_or(false, |entry| entry.map_or(false, |e| e.is_dir)) {
            // Don't process dirs.
            continue;
        }
        let Some(stem) = entry_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let key = file_stem_to_key(stem);

        let contents = if entry_exists {
            fs.load(&entry_path).await.ok()
        } else {
            None
        };

        this.update(&mut cx, move |this, _| {
            let snippets_of_kind = this.snippets.entry(key).or_default();
            if entry_exists {
                let Some(file_contents) = contents else {
                    return;
                };
                let Ok(as_json) = serde_json::from_str::<VSSnippetsFile>(&file_contents) else {
                    return;
                };
                let snippets = file_to_snippets(as_json);
                *snippets_of_kind.entry(entry_path).or_default() = snippets;
            } else {
                snippets_of_kind.remove(&entry_path);
            }
        })?;
    }
    Ok(())
}

async fn initial_scan(
    this: WeakModel<SnippetProvider>,
    path: Arc<Path>,
    mut cx: AsyncAppContext,
) -> Result<()> {
    let fs = this.update(&mut cx, |this, _| this.fs.clone())?;
    let entries = fs.read_dir(&path).await;
    if let Ok(entries) = entries {
        let entries = entries
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;
        process_updates(this, entries, cx).await?;
    }
    Ok(())
}

pub struct SnippetProvider {
    fs: Arc<dyn Fs>,
    snippets: HashMap<SnippetKind, BTreeMap<PathBuf, Vec<Arc<Snippet>>>>,
}

impl SnippetProvider {
    pub fn new(
        fs: Arc<dyn Fs>,
        dirs_to_watch: BTreeSet<PathBuf>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(move |cx| {
            let mut this = Self {
                fs,
                snippets: Default::default(),
            };

            let mut task_handles = vec![];
            for dir in dirs_to_watch {
                task_handles.push(this.watch_directory(&dir, cx));
            }
            cx.spawn(|_, _| async move {
                futures::future::join_all(task_handles).await;
            })
            .detach();

            this
        })
    }

    /// Add directory to be watched for content changes
    fn watch_directory(&mut self, path: &Path, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let path: Arc<Path> = Arc::from(path);

        cx.spawn(|this, mut cx| async move {
            let fs = this.update(&mut cx, |this, _| this.fs.clone())?;
            let watched_path = path.clone();
            let watcher = fs.watch(&watched_path, Duration::from_secs(1));
            initial_scan(this.clone(), path, cx.clone()).await?;

            let (mut entries, _) = watcher.await;
            while let Some(entries) = entries.next().await {
                process_updates(this.clone(), entries, cx.clone()).await?;
            }
            Ok(())
        })
    }

    fn lookup_snippets<'a>(
        &'a self,
        language: &'a SnippetKind,
        cx: &AppContext,
    ) -> Vec<Arc<Snippet>> {
        let mut user_snippets: Vec<_> = self
            .snippets
            .get(&language)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .flat_map(|(_, snippets)| snippets.into_iter())
            .collect();

        let Some(registry) = SnippetRegistry::try_global(cx) else {
            return user_snippets;
        };

        let registry_snippets = registry.get_snippets(language);
        user_snippets.extend(registry_snippets);

        user_snippets
    }

    pub fn snippets_for(&self, language: SnippetKind, cx: &AppContext) -> Vec<Arc<Snippet>> {
        let mut requested_snippets = self.lookup_snippets(&language, cx);

        if language.is_some() {
            // Look up global snippets as well.
            requested_snippets.extend(self.lookup_snippets(&None, cx));
        }
        requested_snippets
    }
}
