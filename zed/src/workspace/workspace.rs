use super::{ItemView, ItemViewHandle};
use crate::{
    editor::Buffer,
    settings::Settings,
    time::ReplicaId,
    watch,
    worktree::{Worktree, WorktreeHandle as _},
};
use anyhow::anyhow;
use gpui::{
    App, AppContext, Entity, Handle, ModelContext, ModelHandle, MutableAppContext, ViewContext,
};
use smol::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};

pub trait Item
where
    Self: Sized,
{
    type View: ItemView;
    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self::View>,
    ) -> Self::View;
}

pub trait ItemHandle: Debug + Send + Sync {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        app: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle>;
    fn id(&self) -> usize;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
}

impl<T: 'static + Item> ItemHandle for ModelHandle<T> {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        app: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        Box::new(app.add_view(window_id, |ctx| T::build_view(self.clone(), settings, ctx)))
    }

    fn id(&self) -> usize {
        Handle::id(self)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

pub type OpenResult = Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>;

#[derive(Clone)]
enum OpenedItem {
    Loading(watch::Receiver<Option<OpenResult>>),
    Loaded(Box<dyn ItemHandle>),
}

pub struct Workspace {
    replica_id: ReplicaId,
    worktrees: HashSet<ModelHandle<Worktree>>,
    items: HashMap<(usize, usize), OpenedItem>,
}

impl Workspace {
    pub fn new(paths: Vec<PathBuf>, ctx: &mut ModelContext<Self>) -> Self {
        let mut workspace = Self {
            replica_id: 0,
            worktrees: HashSet::new(),
            items: HashMap::new(),
        };
        workspace.open_paths(&paths, ctx);
        workspace
    }

    pub fn worktrees(&self) -> &HashSet<ModelHandle<Worktree>> {
        &self.worktrees
    }

    pub fn contains_paths(&self, paths: &[PathBuf], app: &AppContext) -> bool {
        paths.iter().all(|path| self.contains_path(&path, app))
    }

    pub fn contains_path(&self, path: &Path, app: &AppContext) -> bool {
        self.worktrees
            .iter()
            .any(|worktree| worktree.as_ref(app).contains_path(path))
    }

    pub fn open_paths(&mut self, paths: &[PathBuf], ctx: &mut ModelContext<Self>) {
        for path in paths.iter().cloned() {
            self.open_path(path, ctx);
        }
    }

    pub fn open_path<'a>(&'a mut self, path: PathBuf, ctx: &mut ModelContext<Self>) {
        for tree in self.worktrees.iter() {
            if tree.as_ref(ctx).contains_path(&path) {
                return;
            }
        }

        let worktree = ctx.add_model(|ctx| Worktree::new(ctx.model_id(), path, Some(ctx)));
        ctx.observe(&worktree, Self::on_worktree_updated);
        self.worktrees.insert(worktree);
        ctx.notify();
    }

    pub fn open_entry(
        &mut self,
        entry: (usize, usize),
        ctx: &mut ModelContext<'_, Self>,
    ) -> anyhow::Result<Pin<Box<dyn Future<Output = OpenResult> + Send>>> {
        if let Some(item) = self.items.get(&entry).cloned() {
            return Ok(async move {
                match item {
                    OpenedItem::Loaded(handle) => {
                        return Ok(handle);
                    }
                    OpenedItem::Loading(rx) => loop {
                        rx.updated().await;

                        if let Some(result) = smol::block_on(rx.read()).clone() {
                            return result;
                        }
                    },
                }
            }
            .boxed());
        }

        let worktree = self
            .worktrees
            .get(&entry.0)
            .cloned()
            .ok_or(anyhow!("worktree {} does not exist", entry.0,))?;

        let replica_id = self.replica_id;
        let file = worktree.file(entry.1, ctx.app())?;
        let history = file.load_history(ctx.app());
        let buffer = async move { Ok(Buffer::from_history(replica_id, file, history.await?)) };

        let (mut tx, rx) = watch::channel(None);
        self.items.insert(entry, OpenedItem::Loading(rx));
        let _ = ctx.spawn(
            buffer,
            move |me, buffer: anyhow::Result<Buffer>, ctx| match buffer {
                Ok(buffer) => {
                    let handle = Box::new(ctx.add_model(|_| buffer)) as Box<dyn ItemHandle>;
                    me.items.insert(entry, OpenedItem::Loaded(handle.clone()));
                    let _ = ctx.spawn(
                        async move {
                            tx.update(|value| *value = Some(Ok(handle))).await;
                        },
                        |_, _, _| {},
                    );
                }
                Err(error) => {
                    let _ = ctx.spawn(
                        async move {
                            tx.update(|value| *value = Some(Err(Arc::new(error)))).await;
                        },
                        |_, _, _| {},
                    );
                }
            },
        );

        self.open_entry(entry, ctx)
    }

    fn on_worktree_updated(&mut self, _: ModelHandle<Worktree>, ctx: &mut ModelContext<Self>) {
        ctx.notify();
    }
}

impl Entity for Workspace {
    type Event = ();
}

#[cfg(test)]
pub trait WorkspaceHandle {
    fn file_entries(&self, app: &App) -> Vec<(usize, usize)>;
}

#[cfg(test)]
impl WorkspaceHandle for ModelHandle<Workspace> {
    fn file_entries(&self, app: &App) -> Vec<(usize, usize)> {
        self.read(&app, |w, app| {
            w.worktrees()
                .iter()
                .flat_map(|tree| {
                    let tree_id = tree.id();
                    tree.as_ref(app)
                        .files()
                        .map(move |file| (tree_id, file.entry_id))
                })
                .collect::<Vec<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::temp_tree;
    use gpui::App;
    use serde_json::json;

    #[test]
    fn test_open_entry() -> Result<(), Arc<anyhow::Error>> {
        App::test((), |mut app| async move {
            let dir = temp_tree(json!({
                "a": {
                    "aa": "aa contents",
                    "ab": "ab contents",
                },
            }));

            let workspace = app.add_model(|ctx| Workspace::new(vec![dir.path().into()], ctx));
            app.finish_pending_tasks().await; // Open and populate worktree.

            // Get the first file entry.
            let entry = workspace.read(&app, |w, app| {
                let tree = w.worktrees.iter().next().unwrap();
                let entry_id = tree.as_ref(app).files().next().unwrap().entry_id;
                (tree.id(), entry_id)
            });

            // Open the same entry twice before it finishes loading.
            let (future_1, future_2) = workspace.update(&mut app, |w, app| {
                (
                    w.open_entry(entry, app).unwrap(),
                    w.open_entry(entry, app).unwrap(),
                )
            });

            let handle_1 = future_1.await?;
            let handle_2 = future_2.await?;
            assert_eq!(handle_1.id(), handle_2.id());

            // Open the same entry again now that it has loaded
            let handle_3 = workspace
                .update(&mut app, |w, app| w.open_entry(entry, app).unwrap())
                .await?;

            assert_eq!(handle_3.id(), handle_1.id());

            Ok(())
        })
    }
}
