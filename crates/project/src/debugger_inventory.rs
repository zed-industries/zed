use std::{
    borrow::Cow,
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};

use dap::config_templates::DebuggerConfigTemplate;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use gpui::{AppContext, Context, Model, ModelContext, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use task::static_source::TrackedFile;

pub struct DebuggerInventory {
    sources: Vec<SourceInInventory>,
    update_sender: UnboundedSender<()>,
    _update_pooler: Task<anyhow::Result<()>>,
}

impl DebuggerInventory {
    pub fn new(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|cx| {
            let (update_sender, mut rx) = unbounded();
            let _update_pooler = cx.spawn(|this, mut cx| async move {
                while let Some(()) = rx.next().await {
                    this.update(&mut cx, |_, cx| {
                        cx.notify();
                    })?;
                }
                Ok(())
            });
            Self {
                sources: Vec::new(),
                update_sender,
                _update_pooler,
            }
        })
    }

    pub fn remove_source(&mut self, abs_path: &PathBuf) {
        todo!();
    }

    pub fn add_source(
        &mut self,
        kind: DebuggerConfigSourceKind,
        create_source: impl FnOnce(UnboundedSender<()>, &mut AppContext) -> StaticSource,
        cx: &mut ModelContext<Self>,
    ) {
        let abs_path = kind.abs_path();
        if abs_path.is_some() {
            if let Some(a) = self.sources.iter().find(|s| s.kind.abs_path() == abs_path) {
                log::debug!("Source for path {abs_path:?} already exists, not adding. Old kind: {OLD_KIND:?}, new kind: {kind:?}", OLD_KIND = a.kind);
                return;
            }
        }

        let source = create_source(self.update_sender.clone(), cx);
        let source = SourceInInventory { source, kind };
        self.sources.push(source);
        cx.notify();
    }
}

/// Kind of a source the tasks are fetched from, used to display more source information in the UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DebuggerConfigSourceKind {
    /// Tasks from the worktree's .zed/task.json
    Worktree {
        id: usize,
        abs_path: PathBuf,
        id_base: Cow<'static, str>,
    },
    /// ~/.config/zed/task.json - like global files with task definitions, applicable to any path
    AbsPath {
        id_base: Cow<'static, str>,
        abs_path: PathBuf,
    },
    /// Languages-specific tasks coming from extensions.
    Language { name: Arc<str> },
}

impl DebuggerConfigSourceKind {
    fn abs_path(&self) -> Option<&Path> {
        match self {
            Self::AbsPath { abs_path, .. } | Self::Worktree { abs_path, .. } => Some(abs_path),
            Self::Language { .. } => None,
        }
    }
}

struct SourceInInventory {
    source: StaticSource,
    kind: DebuggerConfigSourceKind,
}

pub struct StaticSource {
    configs: TrackedFile<DebuggerConfigTemplates>,
}

impl StaticSource {
    pub fn new(debugger_configs: TrackedFile<DebuggerConfigTemplates>) -> Self {
        Self {
            configs: debugger_configs,
        }
    }
}

/// A group of Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DebuggerConfigTemplates(pub Vec<DebuggerConfigTemplate>);
