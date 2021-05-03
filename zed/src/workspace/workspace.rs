use super::ItemViewHandle;
use crate::{
    editor::{Buffer, BufferView},
    settings::Settings,
    time::ReplicaId,
    watch,
    worktree::{Worktree, WorktreeHandle as _},
};
use anyhow::anyhow;
use futures_core::future::LocalBoxFuture;
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
use smol::prelude::*;
use std::{collections::hash_map::Entry, future};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct Workspace {
    replica_id: ReplicaId,
    worktrees: HashSet<ModelHandle<Worktree>>,
    buffers: HashMap<
        (usize, u64),
        postage::watch::Receiver<Option<Result<ModelHandle<Buffer>, Arc<anyhow::Error>>>>,
    >,
}

impl Workspace {
    pub fn new(paths: Vec<PathBuf>, ctx: &mut ModelContext<Self>) -> Self {
        let mut workspace = Self {
            replica_id: 0,
            worktrees: Default::default(),
            buffers: Default::default(),
        };
        workspace.open_paths(&paths, ctx);
        workspace
    }

    pub fn worktrees(&self) -> &HashSet<ModelHandle<Worktree>> {
        &self.worktrees
    }

    pub fn worktree_scans_complete(&self, ctx: &AppContext) -> impl Future<Output = ()> + 'static {
        let futures = self
            .worktrees
            .iter()
            .map(|worktree| worktree.read(ctx).scan_complete())
            .collect::<Vec<_>>();
        async move {
            for future in futures {
                future.await;
            }
        }
    }

    pub fn contains_paths(&self, paths: &[PathBuf], app: &AppContext) -> bool {
        paths.iter().all(|path| self.contains_path(&path, app))
    }

    pub fn contains_path(&self, path: &Path, app: &AppContext) -> bool {
        self.worktrees
            .iter()
            .any(|worktree| worktree.read(app).contains_abs_path(path))
    }

    pub fn open_paths(
        &mut self,
        paths: &[PathBuf],
        ctx: &mut ModelContext<Self>,
    ) -> Vec<(usize, Arc<Path>)> {
        paths
            .iter()
            .cloned()
            .map(move |path| self.open_path(path, ctx))
            .collect()
    }

    fn open_path(&mut self, path: PathBuf, ctx: &mut ModelContext<Self>) -> (usize, Arc<Path>) {
        for tree in self.worktrees.iter() {
            if let Ok(relative_path) = path.strip_prefix(tree.read(ctx).abs_path()) {
                return (tree.id(), relative_path.into());
            }
        }

        let worktree = ctx.add_model(|ctx| Worktree::new(path.clone(), ctx));
        let worktree_id = worktree.id();
        ctx.observe(&worktree, Self::on_worktree_updated);
        self.worktrees.insert(worktree);
        ctx.notify();
        (worktree_id, Path::new("").into())
    }

    pub fn open_entry(
        &mut self,
        (worktree_id, path): (usize, Arc<Path>),
        window_id: usize,
        settings: watch::Receiver<Settings>,
        ctx: &mut ModelContext<Self>,
    ) -> LocalBoxFuture<'static, Result<Box<dyn ItemViewHandle>, Arc<anyhow::Error>>> {
        let worktree = match self.worktrees.get(&worktree_id).cloned() {
            Some(worktree) => worktree,
            None => {
                return future::ready(Err(Arc::new(anyhow!(
                    "worktree {} does not exist",
                    worktree_id
                ))))
                .boxed_local();
            }
        };

        let inode = match worktree.read(ctx).inode_for_path(&path) {
            Some(inode) => inode,
            None => {
                return future::ready(Err(Arc::new(anyhow!("path {:?} does not exist", path))))
                    .boxed_local();
            }
        };

        let file = match worktree.file(path.clone(), ctx.as_ref()) {
            Some(file) => file,
            None => {
                return future::ready(Err(Arc::new(anyhow!("path {:?} does not exist", path))))
                    .boxed_local()
            }
        };

        if let Entry::Vacant(entry) = self.buffers.entry((worktree_id, inode)) {
            let (mut tx, rx) = postage::watch::channel();
            entry.insert(rx);
            let history = file.load_history(ctx.as_ref());
            let replica_id = self.replica_id;
            let buffer = ctx
                .background_executor()
                .spawn(async move { Ok(Buffer::from_history(replica_id, history.await?)) });
            ctx.spawn(buffer, move |_, from_history_result, ctx| {
                *tx.borrow_mut() = Some(match from_history_result {
                    Ok(buffer) => Ok(ctx.add_model(|_| buffer)),
                    Err(error) => Err(Arc::new(error)),
                })
            })
            .detach()
        }

        let mut watch = self.buffers.get(&(worktree_id, inode)).unwrap().clone();
        ctx.spawn(
            async move {
                loop {
                    if let Some(load_result) = watch.borrow().as_ref() {
                        return load_result.clone();
                    }
                    watch.next().await;
                }
            },
            move |_, load_result, ctx| {
                load_result.map(|buffer_handle| {
                    Box::new(ctx.as_mut().add_view(window_id, |ctx| {
                        BufferView::for_buffer(buffer_handle, Some(file), settings, ctx)
                    })) as Box<dyn ItemViewHandle>
                })
            },
        )
        .boxed_local()
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
    fn file_entries(&self, app: &AppContext) -> Vec<(usize, Arc<Path>)>;
}

#[cfg(test)]
impl WorkspaceHandle for ModelHandle<Workspace> {
    fn file_entries(&self, app: &AppContext) -> Vec<(usize, Arc<Path>)> {
        self.read(app)
            .worktrees()
            .iter()
            .flat_map(|tree| {
                let tree_id = tree.id();
                tree.read(app)
                    .files(0)
                    .map(move |f| (tree_id, f.path().clone()))
            })
            .collect::<Vec<_>>()
    }
}
