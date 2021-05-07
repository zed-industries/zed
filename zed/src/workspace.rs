pub mod pane;
pub mod pane_group;
use crate::{
    editor::{Buffer, BufferView},
    settings::Settings,
    time::ReplicaId,
    watch::{self, Receiver},
    worktree::{FileHandle, Worktree, WorktreeHandle},
};
use futures_core::{future::LocalBoxFuture, Future};
use gpui::{
    color::rgbu, elements::*, json::to_string_pretty, keymap::Binding, AnyViewHandle, AppContext,
    ClipboardItem, Entity, EntityTask, ModelHandle, MutableAppContext, PathPromptOptions, View,
    ViewContext, ViewHandle, WeakModelHandle,
};
use log::error;
pub use pane::*;
pub use pane_group::*;
use smol::prelude::*;
use std::{collections::HashMap, path::PathBuf};
use std::{
    collections::{hash_map::Entry, HashSet},
    path::Path,
    sync::Arc,
};

pub fn init(app: &mut MutableAppContext) {
    app.add_global_action("workspace:open", open);
    app.add_global_action("workspace:open_paths", open_paths);
    app.add_global_action("app:quit", quit);
    app.add_action("workspace:save", Workspace::save_active_item);
    app.add_action("workspace:debug_elements", Workspace::debug_elements);
    app.add_action("workspace:new_file", Workspace::open_new_file);
    app.add_bindings(vec![
        Binding::new("cmd-s", "workspace:save", None),
        Binding::new("cmd-alt-i", "workspace:debug_elements", None),
    ]);
    pane::init(app);
}

pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub settings: watch::Receiver<Settings>,
}

fn open(settings: &Receiver<Settings>, ctx: &mut MutableAppContext) {
    let settings = settings.clone();
    ctx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, ctx| {
            if let Some(paths) = paths {
                ctx.dispatch_global_action("workspace:open_paths", OpenParams { paths, settings });
            }
        },
    );
}

fn open_paths(params: &OpenParams, app: &mut MutableAppContext) {
    log::info!("open paths {:?}", params.paths);

    // Open paths in existing workspace if possible
    for window_id in app.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = app.root_view::<Workspace>(window_id) {
            if handle.update(app, |view, ctx| {
                if view.contains_paths(&params.paths, ctx.as_ref()) {
                    let open_paths = view.open_paths(&params.paths, ctx);
                    ctx.foreground().spawn(open_paths).detach();
                    log::info!("open paths on existing workspace");
                    true
                } else {
                    false
                }
            }) {
                return;
            }
        }
    }

    log::info!("open new workspace");

    // Add a new workspace if necessary
    app.add_window(|ctx| {
        let mut view = Workspace::new(0, params.settings.clone(), ctx);
        let open_paths = view.open_paths(&params.paths, ctx);
        ctx.foreground().spawn(open_paths).detach();
        view
    });
}

fn quit(_: &(), app: &mut MutableAppContext) {
    app.platform().quit();
}

pub trait Item: Entity + Sized {
    type View: ItemView;

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self::View>,
    ) -> Self::View;

    fn file(&self) -> Option<&FileHandle>;
}

pub trait ItemView: View {
    fn title(&self, app: &AppContext) -> String;
    fn entry_id(&self, app: &AppContext) -> Option<(usize, Arc<Path>)>;
    fn clone_on_split(&self, _: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
    fn is_dirty(&self, _: &AppContext) -> bool {
        false
    }
    fn save(
        &mut self,
        _: Option<FileHandle>,
        _: &mut ViewContext<Self>,
    ) -> LocalBoxFuture<'static, anyhow::Result<()>>;
    fn should_activate_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
}

pub trait ItemHandle: Send + Sync {
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn downgrade(&self) -> Box<dyn WeakItemHandle>;
}

pub trait WeakItemHandle: Send + Sync {
    fn file<'a>(&'a self, ctx: &'a AppContext) -> Option<&'a FileHandle>;
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        app: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemViewHandle>>;
    fn alive(&self, ctx: &AppContext) -> bool;
}

pub trait ItemViewHandle: Send + Sync {
    fn title(&self, app: &AppContext) -> String;
    fn entry_id(&self, app: &AppContext) -> Option<(usize, Arc<Path>)>;
    fn boxed_clone(&self) -> Box<dyn ItemViewHandle>;
    fn clone_on_split(&self, app: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>>;
    fn set_parent_pane(&self, pane: &ViewHandle<Pane>, app: &mut MutableAppContext);
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, ctx: &AppContext) -> bool;
    fn save(
        &self,
        file: Option<FileHandle>,
        ctx: &mut MutableAppContext,
    ) -> LocalBoxFuture<'static, anyhow::Result<()>>;
}

impl<T: Item> ItemHandle for ModelHandle<T> {
    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.downgrade())
    }
}

impl<T: Item> WeakItemHandle for WeakModelHandle<T> {
    fn file<'a>(&'a self, ctx: &'a AppContext) -> Option<&'a FileHandle> {
        self.upgrade(ctx).and_then(|h| h.read(ctx).file())
    }

    fn add_view(
        &self,
        window_id: usize,
        settings: Receiver<Settings>,
        ctx: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemViewHandle>> {
        if let Some(handle) = self.upgrade(ctx.as_ref()) {
            Some(Box::new(ctx.add_view(window_id, |ctx| {
                T::build_view(handle, settings, ctx)
            })))
        } else {
            None
        }
    }

    fn alive(&self, ctx: &AppContext) -> bool {
        self.upgrade(ctx).is_some()
    }
}

impl<T: ItemView> ItemViewHandle for ViewHandle<T> {
    fn title(&self, app: &AppContext) -> String {
        self.read(app).title(app)
    }

    fn entry_id(&self, app: &AppContext) -> Option<(usize, Arc<Path>)> {
        self.read(app).entry_id(app)
    }

    fn boxed_clone(&self) -> Box<dyn ItemViewHandle> {
        Box::new(self.clone())
    }

    fn clone_on_split(&self, app: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>> {
        self.update(app, |item, ctx| {
            ctx.add_option_view(|ctx| item.clone_on_split(ctx))
        })
        .map(|handle| Box::new(handle) as Box<dyn ItemViewHandle>)
    }

    fn set_parent_pane(&self, pane: &ViewHandle<Pane>, app: &mut MutableAppContext) {
        pane.update(app, |_, ctx| {
            ctx.subscribe_to_view(self, |pane, item, event, ctx| {
                if T::should_activate_item_on_event(event) {
                    if let Some(ix) = pane.item_index(&item) {
                        pane.activate_item(ix, ctx);
                        pane.activate(ctx);
                    }
                }
                if T::should_update_tab_on_event(event) {
                    ctx.notify()
                }
            })
        })
    }

    fn save(
        &self,
        file: Option<FileHandle>,
        ctx: &mut MutableAppContext,
    ) -> LocalBoxFuture<'static, anyhow::Result<()>> {
        self.update(ctx, |item, ctx| item.save(file, ctx))
    }

    fn is_dirty(&self, ctx: &AppContext) -> bool {
        self.read(ctx).is_dirty(ctx)
    }

    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }
}

impl Clone for Box<dyn ItemViewHandle> {
    fn clone(&self) -> Box<dyn ItemViewHandle> {
        self.boxed_clone()
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}

#[derive(Debug)]
pub struct State {
    pub modal: Option<usize>,
    pub center: PaneGroup,
}

pub struct Workspace {
    pub settings: watch::Receiver<Settings>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    replica_id: ReplicaId,
    worktrees: HashSet<ModelHandle<Worktree>>,
    items: Vec<Box<dyn WeakItemHandle>>,
    loading_items: HashMap<
        (usize, Arc<Path>),
        postage::watch::Receiver<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>,
    >,
}

impl Workspace {
    pub fn new(
        replica_id: ReplicaId,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let pane = ctx.add_view(|_| Pane::new(settings.clone()));
        let pane_id = pane.id();
        ctx.subscribe_to_view(&pane, move |me, _, event, ctx| {
            me.handle_pane_event(pane_id, event, ctx)
        });
        ctx.focus(&pane);

        Workspace {
            modal: None,
            center: PaneGroup::new(pane.id()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            settings,
            replica_id,
            worktrees: Default::default(),
            items: Default::default(),
            loading_items: Default::default(),
        }
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
            .any(|worktree| worktree.read(app).contains_abs_path(path))
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

    pub fn open_paths(
        &mut self,
        paths: &[PathBuf],
        ctx: &mut ViewContext<Self>,
    ) -> impl Future<Output = ()> {
        let entries = paths
            .iter()
            .cloned()
            .map(|path| self.file_for_path(&path, ctx))
            .collect::<Vec<_>>();

        let bg = ctx.background_executor().clone();
        let tasks = paths
            .iter()
            .cloned()
            .zip(entries.into_iter())
            .map(|(abs_path, file)| {
                ctx.spawn(
                    bg.spawn(async move { abs_path.is_file() }),
                    move |me, is_file, ctx| {
                        if is_file {
                            me.open_entry(file.entry_id(), ctx)
                        } else {
                            None
                        }
                    },
                )
            })
            .collect::<Vec<_>>();
        async move {
            for task in tasks {
                if let Some(task) = task.await {
                    task.await;
                }
            }
        }
    }

    fn file_for_path(&mut self, abs_path: &Path, ctx: &mut ViewContext<Self>) -> FileHandle {
        for tree in self.worktrees.iter() {
            if let Ok(relative_path) = abs_path.strip_prefix(tree.read(ctx).abs_path()) {
                return tree.file(relative_path, ctx.as_ref());
            }
        }
        let worktree = self.add_worktree(&abs_path, ctx);
        worktree.file(Path::new(""), ctx.as_ref())
    }

    pub fn add_worktree(
        &mut self,
        path: &Path,
        ctx: &mut ViewContext<Self>,
    ) -> ModelHandle<Worktree> {
        let worktree = ctx.add_model(|ctx| Worktree::new(path, ctx));
        ctx.observe_model(&worktree, |_, _, ctx| ctx.notify());
        self.worktrees.insert(worktree.clone());
        ctx.notify();
        worktree
    }

    pub fn toggle_modal<V, F>(&mut self, ctx: &mut ViewContext<Self>, add_view: F)
    where
        V: 'static + View,
        F: FnOnce(&mut ViewContext<Self>, &mut Self) -> ViewHandle<V>,
    {
        if self.modal.as_ref().map_or(false, |modal| modal.is::<V>()) {
            self.modal.take();
            ctx.focus_self();
        } else {
            let modal = add_view(ctx, self);
            ctx.focus(&modal);
            self.modal = Some(modal.into());
        }
        ctx.notify();
    }

    pub fn modal(&self) -> Option<&AnyViewHandle> {
        self.modal.as_ref()
    }

    pub fn dismiss_modal(&mut self, ctx: &mut ViewContext<Self>) {
        if self.modal.take().is_some() {
            ctx.focus(&self.active_pane);
            ctx.notify();
        }
    }

    pub fn open_new_file(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let buffer = ctx.add_model(|ctx| Buffer::new(self.replica_id, "", ctx));
        let buffer_view =
            ctx.add_view(|ctx| BufferView::for_buffer(buffer.clone(), self.settings.clone(), ctx));
        self.items.push(ItemHandle::downgrade(&buffer));
        self.add_item(Box::new(buffer_view), ctx);
    }

    #[must_use]
    pub fn open_entry(
        &mut self,
        entry: (usize, Arc<Path>),
        ctx: &mut ViewContext<Self>,
    ) -> Option<EntityTask<()>> {
        // If the active pane contains a view for this file, then activate
        // that item view.
        if self
            .active_pane()
            .update(ctx, |pane, ctx| pane.activate_entry(entry.clone(), ctx))
        {
            return None;
        }

        let window_id = ctx.window_id();
        let settings = self.settings.clone();

        // Otherwise, if this file is already open somewhere in the workspace,
        // then add another view for it.
        let mut i = 0;
        while i < self.items.len() {
            let item = &self.items[i];
            if item.alive(ctx.as_ref()) {
                if item
                    .file(ctx.as_ref())
                    .map_or(false, |f| f.entry_id() == entry)
                {
                    self.add_item(
                        item.add_view(window_id, settings.clone(), ctx.as_mut())
                            .unwrap(),
                        ctx,
                    );
                    return None;
                }
                i += 1;
            } else {
                self.items.remove(i);
            }
        }

        let (worktree_id, path) = entry.clone();

        let worktree = match self.worktrees.get(&worktree_id).cloned() {
            Some(worktree) => worktree,
            None => {
                log::error!("worktree {} does not exist", worktree_id);
                return None;
            }
        };

        let file = worktree.file(path.clone(), ctx.as_ref());
        if file.is_deleted() {
            log::error!("path {:?} does not exist", path);
            return None;
        }

        if let Entry::Vacant(entry) = self.loading_items.entry(entry.clone()) {
            let (mut tx, rx) = postage::watch::channel();
            entry.insert(rx);
            let replica_id = self.replica_id;
            let history = ctx
                .background_executor()
                .spawn(file.load_history(ctx.as_ref()));
            ctx.spawn(history, move |_, history, ctx| {
                *tx.borrow_mut() = Some(match history {
                    Ok(history) => Ok(Box::new(ctx.add_model(|ctx| {
                        Buffer::from_history(replica_id, history, Some(file), ctx)
                    }))),
                    Err(error) => Err(Arc::new(error)),
                })
            })
            .detach()
        }

        let mut watch = self.loading_items.get(&entry).unwrap().clone();
        Some(ctx.spawn(
            async move {
                loop {
                    if let Some(load_result) = watch.borrow().as_ref() {
                        return load_result.clone();
                    }
                    watch.next().await;
                }
            },
            move |me, load_result, ctx| {
                me.loading_items.remove(&entry);
                match load_result {
                    Ok(item) => {
                        let weak_item = item.downgrade();
                        let view = weak_item
                            .add_view(window_id, settings, ctx.as_mut())
                            .unwrap();
                        me.items.push(weak_item);
                        me.add_item(view, ctx);
                    }
                    Err(error) => {
                        log::error!("error opening item: {}", error);
                    }
                }
            },
        ))
    }

    pub fn active_item(&self, ctx: &ViewContext<Self>) -> Option<Box<dyn ItemViewHandle>> {
        self.active_pane().read(ctx).active_item()
    }

    pub fn save_active_item(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if let Some(item) = self.active_item(ctx) {
            if item.entry_id(ctx.as_ref()).is_none() {
                let handle = ctx.handle();
                let start_path = self
                    .worktrees
                    .iter()
                    .next()
                    .map_or(Path::new(""), |h| h.read(ctx).abs_path())
                    .to_path_buf();
                ctx.prompt_for_new_path(&start_path, move |path, ctx| {
                    if let Some(path) = path {
                        handle.update(ctx, move |this, ctx| {
                            let file = this.file_for_path(&path, ctx);
                            let task = item.save(Some(file), ctx.as_mut());
                            ctx.spawn(task, move |_, result, _| {
                                if let Err(e) = result {
                                    error!("failed to save item: {:?}, ", e);
                                }
                            })
                            .detach()
                        })
                    }
                });
                return;
            }

            let task = item.save(None, ctx.as_mut());
            ctx.spawn(task, |_, result, _| {
                if let Err(e) = result {
                    error!("failed to save item: {:?}, ", e);
                }
            })
            .detach()
        }
    }

    pub fn debug_elements(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        match to_string_pretty(&ctx.debug_elements()) {
            Ok(json) => {
                let kib = json.len() as f32 / 1024.;
                ctx.as_mut().write_to_clipboard(ClipboardItem::new(json));
                log::info!(
                    "copied {:.1} KiB of element debug JSON to the clipboard",
                    kib
                );
            }
            Err(error) => {
                log::error!("error debugging elements: {}", error);
            }
        };
    }

    fn add_pane(&mut self, ctx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = ctx.add_view(|_| Pane::new(self.settings.clone()));
        let pane_id = pane.id();
        ctx.subscribe_to_view(&pane, move |me, _, event, ctx| {
            me.handle_pane_event(pane_id, event, ctx)
        });
        self.panes.push(pane.clone());
        self.activate_pane(pane.clone(), ctx);
        pane
    }

    fn activate_pane(&mut self, pane: ViewHandle<Pane>, ctx: &mut ViewContext<Self>) {
        self.active_pane = pane;
        ctx.focus(&self.active_pane);
        ctx.notify();
    }

    fn handle_pane_event(
        &mut self,
        pane_id: usize,
        event: &pane::Event,
        ctx: &mut ViewContext<Self>,
    ) {
        if let Some(pane) = self.pane(pane_id) {
            match event {
                pane::Event::Split(direction) => {
                    self.split_pane(pane, *direction, ctx);
                }
                pane::Event::Remove => {
                    self.remove_pane(pane, ctx);
                }
                pane::Event::Activate => {
                    self.activate_pane(pane, ctx);
                }
            }
        } else {
            error!("pane {} not found", pane_id);
        }
    }

    fn split_pane(
        &mut self,
        pane: ViewHandle<Pane>,
        direction: SplitDirection,
        ctx: &mut ViewContext<Self>,
    ) -> ViewHandle<Pane> {
        let new_pane = self.add_pane(ctx);
        self.activate_pane(new_pane.clone(), ctx);
        if let Some(item) = pane.read(ctx).active_item() {
            if let Some(clone) = item.clone_on_split(ctx.as_mut()) {
                self.add_item(clone, ctx);
            }
        }
        self.center
            .split(pane.id(), new_pane.id(), direction)
            .unwrap();
        ctx.notify();
        new_pane
    }

    fn remove_pane(&mut self, pane: ViewHandle<Pane>, ctx: &mut ViewContext<Self>) {
        if self.center.remove(pane.id()).unwrap() {
            self.panes.retain(|p| p != &pane);
            self.activate_pane(self.panes.last().unwrap().clone(), ctx);
        }
    }

    fn pane(&self, pane_id: usize) -> Option<ViewHandle<Pane>> {
        self.panes.iter().find(|pane| pane.id() == pane_id).cloned()
    }

    pub fn active_pane(&self) -> &ViewHandle<Pane> {
        &self.active_pane
    }

    fn add_item(&self, item: Box<dyn ItemViewHandle>, ctx: &mut ViewContext<Self>) {
        let active_pane = self.active_pane();
        item.set_parent_pane(&active_pane, ctx.as_mut());
        active_pane.update(ctx, |pane, ctx| {
            let item_idx = pane.add_item(item, ctx);
            pane.activate_item(item_idx, ctx);
        });
    }
}

impl Entity for Workspace {
    type Event = ();
}

impl View for Workspace {
    fn ui_name() -> &'static str {
        "Workspace"
    }

    fn render(&self, _: &AppContext) -> ElementBox {
        Container::new(
            // self.center.render(bump)
            Stack::new()
                .with_child(self.center.render())
                .with_children(self.modal.as_ref().map(|m| ChildView::new(m.id()).boxed()))
                .boxed(),
        )
        .with_background_color(rgbu(0xea, 0xea, 0xeb))
        .named("workspace")
    }

    fn on_focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus(&self.active_pane);
    }
}

#[cfg(test)]
pub trait WorkspaceHandle {
    fn file_entries(&self, app: &AppContext) -> Vec<(usize, Arc<Path>)>;
}

#[cfg(test)]
impl WorkspaceHandle for ViewHandle<Workspace> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::BufferView, settings, test::temp_tree};
    use gpui::App;
    use serde_json::json;
    use std::collections::HashSet;
    use std::time;
    use tempdir::TempDir;

    #[test]
    fn test_open_paths_action() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;

            init(app);

            let dir = temp_tree(json!({
                "a": {
                    "aa": null,
                    "ab": null,
                },
                "b": {
                    "ba": null,
                    "bb": null,
                },
                "c": {
                    "ca": null,
                    "cb": null,
                },
            }));

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![
                        dir.path().join("a").to_path_buf(),
                        dir.path().join("b").to_path_buf(),
                    ],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().count(), 1);

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().count(), 1);
            let workspace_view_1 = app
                .root_view::<Workspace>(app.window_ids().next().unwrap())
                .unwrap();
            assert_eq!(workspace_view_1.read(app).worktrees().len(), 2);

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![
                        dir.path().join("b").to_path_buf(),
                        dir.path().join("c").to_path_buf(),
                    ],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().count(), 2);
        });
    }

    #[test]
    fn test_open_entry() {
        App::test_async((), |mut app| async move {
            let dir = temp_tree(json!({
                "a": {
                    "file1": "contents 1",
                    "file2": "contents 2",
                    "file3": "contents 3",
                },
            }));

            let settings = settings::channel(&app.font_cache()).unwrap().1;

            let (_, workspace) = app.add_window(|ctx| {
                let mut workspace = Workspace::new(0, settings, ctx);
                workspace.add_worktree(dir.path(), ctx);
                workspace
            });

            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let entries = app.read(|ctx| workspace.file_entries(ctx));
            let file1 = entries[0].clone();
            let file2 = entries[1].clone();
            let file3 = entries[2].clone();

            // Open the first entry
            workspace
                .update(&mut app, |w, ctx| w.open_entry(file1.clone(), ctx))
                .unwrap()
                .await;
            app.read(|ctx| {
                let pane = workspace.read(ctx).active_pane().read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file1.clone())
                );
                assert_eq!(pane.items().len(), 1);
            });

            // Open the second entry
            workspace
                .update(&mut app, |w, ctx| w.open_entry(file2.clone(), ctx))
                .unwrap()
                .await;
            app.read(|ctx| {
                let pane = workspace.read(ctx).active_pane().read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file2.clone())
                );
                assert_eq!(pane.items().len(), 2);
            });

            // Open the first entry again. The existing pane item is activated.
            workspace.update(&mut app, |w, ctx| {
                assert!(w.open_entry(file1.clone(), ctx).is_none())
            });
            app.read(|ctx| {
                let pane = workspace.read(ctx).active_pane().read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file1.clone())
                );
                assert_eq!(pane.items().len(), 2);
            });

            // Split the pane with the first entry, then open the second entry again.
            workspace.update(&mut app, |w, ctx| {
                w.split_pane(w.active_pane().clone(), SplitDirection::Right, ctx);
                assert!(w.open_entry(file2.clone(), ctx).is_none());
                assert_eq!(
                    w.active_pane()
                        .read(ctx)
                        .active_item()
                        .unwrap()
                        .entry_id(ctx.as_ref()),
                    Some(file2.clone())
                );
            });

            // Open the third entry twice concurrently. Two pane items
            // are added.
            let (t1, t2) = workspace.update(&mut app, |w, ctx| {
                (
                    w.open_entry(file3.clone(), ctx).unwrap(),
                    w.open_entry(file3.clone(), ctx).unwrap(),
                )
            });
            t1.await;
            t2.await;
            app.read(|ctx| {
                let pane = workspace.read(ctx).active_pane().read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file3.clone())
                );
                let pane_entries = pane
                    .items()
                    .iter()
                    .map(|i| i.entry_id(ctx).unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(pane_entries, &[file1, file2, file3.clone(), file3]);
            });
        });
    }

    #[test]
    fn test_open_paths() {
        App::test_async((), |mut app| async move {
            let dir1 = temp_tree(json!({
                "a.txt": "",
            }));
            let dir2 = temp_tree(json!({
                "b.txt": "",
            }));

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, workspace) = app.add_window(|ctx| {
                let mut workspace = Workspace::new(0, settings, ctx);
                workspace.add_worktree(dir1.path(), ctx);
                workspace
            });
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;

            // Open a file within an existing worktree.
            app.update(|ctx| {
                workspace.update(ctx, |view, ctx| {
                    view.open_paths(&[dir1.path().join("a.txt")], ctx)
                })
            })
            .await;
            app.read(|ctx| {
                workspace
                    .read(ctx)
                    .active_pane()
                    .read(ctx)
                    .active_item()
                    .unwrap()
                    .title(ctx)
                    == "a.txt"
            });

            // Open a file outside of any existing worktree.
            app.update(|ctx| {
                workspace.update(ctx, |view, ctx| {
                    view.open_paths(&[dir2.path().join("b.txt")], ctx)
                })
            })
            .await;
            app.update(|ctx| {
                let worktree_roots = workspace
                    .read(ctx)
                    .worktrees()
                    .iter()
                    .map(|w| w.read(ctx).abs_path())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    worktree_roots,
                    vec![dir1.path(), &dir2.path().join("b.txt")]
                        .into_iter()
                        .collect(),
                );
            });
            app.read(|ctx| {
                workspace
                    .read(ctx)
                    .active_pane()
                    .read(ctx)
                    .active_item()
                    .unwrap()
                    .title(ctx)
                    == "b.txt"
            });
        });
    }

    #[test]
    fn test_open_and_save_new_file() {
        App::test_async((), |mut app| async move {
            let dir = TempDir::new("test-new-file").unwrap();
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, workspace) = app.add_window(|ctx| {
                let mut workspace = Workspace::new(0, settings, ctx);
                workspace.add_worktree(dir.path(), ctx);
                workspace
            });
            let worktree = app.read(|ctx| {
                workspace
                    .read(ctx)
                    .worktrees()
                    .iter()
                    .next()
                    .unwrap()
                    .clone()
            });

            // Create a new untitled buffer
            let editor = workspace.update(&mut app, |workspace, ctx| {
                workspace.open_new_file(&(), ctx);
                workspace
                    .active_item(ctx)
                    .unwrap()
                    .to_any()
                    .downcast::<BufferView>()
                    .unwrap()
            });
            editor.update(&mut app, |editor, ctx| {
                assert!(!editor.is_dirty(ctx.as_ref()));
                assert_eq!(editor.title(ctx.as_ref()), "untitled");
                editor.insert(&"hi".to_string(), ctx);
                assert!(editor.is_dirty(ctx.as_ref()));
            });

            // Save the buffer. This prompts for a filename.
            workspace.update(&mut app, |workspace, ctx| {
                workspace.save_active_item(&(), ctx)
            });
            app.simulate_new_path_selection(|parent_dir| {
                assert_eq!(parent_dir, dir.path());
                Some(parent_dir.join("the-new-name"))
            });
            app.read(|ctx| {
                assert!(editor.is_dirty(ctx));
                assert_eq!(editor.title(ctx), "untitled");
            });

            // When the save completes, the buffer's title is updated.
            editor
                .condition(&app, |editor, ctx| !editor.is_dirty(ctx))
                .await;
            worktree
                .condition_with_duration(time::Duration::from_millis(500), &app, |worktree, _| {
                    worktree.inode_for_path("the-new-name").is_some()
                })
                .await;
            app.read(|ctx| assert_eq!(editor.title(ctx), "the-new-name"));

            // Edit the file and save it again. This time, there is no filename prompt.
            editor.update(&mut app, |editor, ctx| {
                editor.insert(&" there".to_string(), ctx);
                assert_eq!(editor.is_dirty(ctx.as_ref()), true);
            });
            workspace.update(&mut app, |workspace, ctx| {
                workspace.save_active_item(&(), ctx)
            });
            assert!(!app.did_prompt_for_new_path());
            editor
                .condition(&app, |editor, ctx| !editor.is_dirty(ctx))
                .await;
            app.read(|ctx| assert_eq!(editor.title(ctx), "the-new-name"));

            // Open the same newly-created file in another pane item. The new editor should reuse
            // the same buffer.
            workspace.update(&mut app, |workspace, ctx| {
                workspace.open_new_file(&(), ctx);
                workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, ctx);
                assert!(workspace
                    .open_entry((worktree.id(), Path::new("the-new-name").into()), ctx)
                    .is_none());
            });
            let editor2 = workspace.update(&mut app, |workspace, ctx| {
                workspace
                    .active_item(ctx)
                    .unwrap()
                    .to_any()
                    .downcast::<BufferView>()
                    .unwrap()
            });
            app.read(|ctx| {
                assert_eq!(editor2.read(ctx).buffer(), editor.read(ctx).buffer());
            })
        });
    }

    #[test]
    fn test_pane_actions() {
        App::test_async((), |mut app| async move {
            app.update(|ctx| pane::init(ctx));

            let dir = temp_tree(json!({
                "a": {
                    "file1": "contents 1",
                    "file2": "contents 2",
                    "file3": "contents 3",
                },
            }));

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (window_id, workspace) = app.add_window(|ctx| {
                let mut workspace = Workspace::new(0, settings, ctx);
                workspace.add_worktree(dir.path(), ctx);
                workspace
            });
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let entries = app.read(|ctx| workspace.file_entries(ctx));
            let file1 = entries[0].clone();

            let pane_1 = app.read(|ctx| workspace.read(ctx).active_pane().clone());

            workspace
                .update(&mut app, |w, ctx| w.open_entry(file1.clone(), ctx))
                .unwrap()
                .await;
            app.read(|ctx| {
                assert_eq!(
                    pane_1.read(ctx).active_item().unwrap().entry_id(ctx),
                    Some(file1.clone())
                );
            });

            app.dispatch_action(window_id, vec![pane_1.id()], "pane:split_right", ());
            app.update(|ctx| {
                let pane_2 = workspace.read(ctx).active_pane().clone();
                assert_ne!(pane_1, pane_2);

                let pane2_item = pane_2.read(ctx).active_item().unwrap();
                assert_eq!(pane2_item.entry_id(ctx.as_ref()), Some(file1.clone()));

                ctx.dispatch_action(window_id, vec![pane_2.id()], "pane:close_active_item", ());
                let workspace_view = workspace.read(ctx);
                assert_eq!(workspace_view.panes.len(), 1);
                assert_eq!(workspace_view.active_pane(), &pane_1);
            });
        });
    }
}
