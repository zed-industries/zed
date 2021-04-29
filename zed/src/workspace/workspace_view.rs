use super::{pane, Pane, PaneGroup, SplitDirection, Workspace};
use crate::{settings::Settings, watch};
use futures_core::{future::LocalBoxFuture, Future};
use gpui::{
    color::rgbu, elements::*, json::to_string_pretty, keymap::Binding, AnyViewHandle, AppContext,
    ClipboardItem, Entity, EntityTask, ModelHandle, MutableAppContext, View, ViewContext,
    ViewHandle,
};
use log::error;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn init(app: &mut MutableAppContext) {
    app.add_action("workspace:save", WorkspaceView::save_active_item);
    app.add_action("workspace:debug_elements", WorkspaceView::debug_elements);
    app.add_bindings(vec![
        Binding::new("cmd-s", "workspace:save", None),
        Binding::new("cmd-alt-i", "workspace:debug_elements", None),
    ]);
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
    fn save(&self, _: &mut ViewContext<Self>) -> LocalBoxFuture<'static, anyhow::Result<()>> {
        Box::pin(async { Ok(()) })
    }
    fn should_activate_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
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
    fn save(&self, ctx: &mut MutableAppContext) -> LocalBoxFuture<'static, anyhow::Result<()>>;
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

    fn save(&self, ctx: &mut MutableAppContext) -> LocalBoxFuture<'static, anyhow::Result<()>> {
        self.update(ctx, |item, ctx| item.save(ctx))
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

#[derive(Debug)]
pub struct State {
    pub modal: Option<usize>,
    pub center: PaneGroup,
}

pub struct WorkspaceView {
    pub workspace: ModelHandle<Workspace>,
    pub settings: watch::Receiver<Settings>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    loading_entries: HashSet<(usize, Arc<Path>)>,
}

impl WorkspaceView {
    pub fn new(
        workspace: ModelHandle<Workspace>,
        settings: watch::Receiver<Settings>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.observe(&workspace, Self::workspace_updated);

        let pane = ctx.add_view(|_| Pane::new(settings.clone()));
        let pane_id = pane.id();
        ctx.subscribe_to_view(&pane, move |me, _, event, ctx| {
            me.handle_pane_event(pane_id, event, ctx)
        });
        ctx.focus(&pane);

        WorkspaceView {
            workspace,
            modal: None,
            center: PaneGroup::new(pane.id()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            loading_entries: HashSet::new(),
            settings,
        }
    }

    pub fn contains_paths(&self, paths: &[PathBuf], app: &AppContext) -> bool {
        self.workspace.read(app).contains_paths(paths, app)
    }

    pub fn open_paths(
        &self,
        paths: &[PathBuf],
        ctx: &mut ViewContext<Self>,
    ) -> impl Future<Output = ()> {
        let entries = self
            .workspace
            .update(ctx, |workspace, ctx| workspace.open_paths(paths, ctx));
        let bg = ctx.background_executor().clone();
        let tasks = paths
            .iter()
            .cloned()
            .zip(entries.into_iter())
            .map(|(path, entry)| {
                ctx.spawn(
                    bg.spawn(async move { path.is_file() }),
                    |me, is_file, ctx| {
                        if is_file {
                            me.open_entry(entry, ctx)
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

    #[must_use]
    pub fn open_entry(
        &mut self,
        entry: (usize, Arc<Path>),
        ctx: &mut ViewContext<Self>,
    ) -> Option<EntityTask<()>> {
        if self.loading_entries.contains(&entry) {
            return None;
        }

        if self
            .active_pane()
            .update(ctx, |pane, ctx| pane.activate_entry(entry.clone(), ctx))
        {
            return None;
        }

        self.loading_entries.insert(entry.clone());

        match self.workspace.update(ctx, |workspace, ctx| {
            workspace.open_entry(entry.clone(), ctx)
        }) {
            Err(error) => {
                error!("{}", error);
                None
            }
            Ok(item) => {
                let settings = self.settings.clone();
                Some(ctx.spawn(item, move |me, item, ctx| {
                    me.loading_entries.remove(&entry);
                    match item {
                        Ok(item) => {
                            let item_view = item.add_view(ctx.window_id(), settings, ctx.as_mut());
                            me.add_item(item_view, ctx);
                        }
                        Err(error) => {
                            error!("{}", error);
                        }
                    }
                }))
            }
        }
    }

    pub fn save_active_item(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        self.active_pane.update(ctx, |pane, ctx| {
            if let Some(item) = pane.active_item() {
                let task = item.save(ctx.as_mut());
                ctx.spawn(task, |_, result, _| {
                    if let Err(e) = result {
                        // TODO - present this error to the user
                        error!("failed to save item: {:?}, ", e);
                    }
                })
                .detach()
            }
        });
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

    fn workspace_updated(&mut self, _: ModelHandle<Workspace>, ctx: &mut ViewContext<Self>) {
        ctx.notify();
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

impl Entity for WorkspaceView {
    type Event = ();
}

impl View for WorkspaceView {
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
mod tests {
    use super::{pane, Workspace, WorkspaceView};
    use crate::{settings, test::temp_tree, workspace::WorkspaceHandle as _};
    use gpui::App;
    use serde_json::json;
    use std::collections::HashSet;

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
            let workspace = app.add_model(|ctx| Workspace::new(vec![dir.path().into()], ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let entries = app.read(|ctx| workspace.file_entries(ctx));
            let file1 = entries[0].clone();
            let file2 = entries[1].clone();
            let file3 = entries[2].clone();

            let (_, workspace_view) =
                app.add_window(|ctx| WorkspaceView::new(workspace.clone(), settings, ctx));
            let pane = app.read(|ctx| workspace_view.read(ctx).active_pane().clone());

            // Open the first entry
            workspace_view
                .update(&mut app, |w, ctx| w.open_entry(file1.clone(), ctx))
                .unwrap()
                .await;
            app.read(|ctx| {
                let pane = pane.read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file1.clone())
                );
                assert_eq!(pane.items().len(), 1);
            });

            // Open the second entry
            workspace_view
                .update(&mut app, |w, ctx| w.open_entry(file2.clone(), ctx))
                .unwrap()
                .await;
            app.read(|ctx| {
                let pane = pane.read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file2.clone())
                );
                assert_eq!(pane.items().len(), 2);
            });

            // Open the first entry again. The existing pane item is activated.
            workspace_view.update(&mut app, |w, ctx| {
                assert!(w.open_entry(file1.clone(), ctx).is_none())
            });
            app.read(|ctx| {
                let pane = pane.read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file1.clone())
                );
                assert_eq!(pane.items().len(), 2);
            });

            // Open the third entry twice concurrently. Only one pane item is added.
            workspace_view
                .update(&mut app, |w, ctx| {
                    let task = w.open_entry(file3.clone(), ctx).unwrap();
                    assert!(w.open_entry(file3.clone(), ctx).is_none());
                    task
                })
                .await;
            app.read(|ctx| {
                let pane = pane.read(ctx);
                assert_eq!(
                    pane.active_item().unwrap().entry_id(ctx),
                    Some(file3.clone())
                );
                assert_eq!(pane.items().len(), 3);
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

            let workspace = app.add_model(|ctx| Workspace::new(vec![dir1.path().into()], ctx));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let (_, workspace_view) =
                app.add_window(|ctx| WorkspaceView::new(workspace.clone(), settings, ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;

            // Open a file within an existing worktree.
            app.update(|ctx| {
                workspace_view.update(ctx, |view, ctx| {
                    view.open_paths(&[dir1.path().join("a.txt")], ctx)
                })
            })
            .await;
            app.read(|ctx| {
                workspace_view
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
                workspace_view.update(ctx, |view, ctx| {
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
                workspace_view
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
            let workspace = app.add_model(|ctx| Workspace::new(vec![dir.path().into()], ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let entries = app.read(|ctx| workspace.file_entries(ctx));
            let file1 = entries[0].clone();

            let (window_id, workspace_view) =
                app.add_window(|ctx| WorkspaceView::new(workspace.clone(), settings, ctx));
            let pane_1 = app.read(|ctx| workspace_view.read(ctx).active_pane().clone());

            workspace_view
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
                let pane_2 = workspace_view.read(ctx).active_pane().clone();
                assert_ne!(pane_1, pane_2);

                let pane2_item = pane_2.read(ctx).active_item().unwrap();
                assert_eq!(pane2_item.entry_id(ctx.as_ref()), Some(file1.clone()));

                ctx.dispatch_action(window_id, vec![pane_2.id()], "pane:close_active_item", ());
                let workspace_view = workspace_view.read(ctx);
                assert_eq!(workspace_view.panes.len(), 1);
                assert_eq!(workspace_view.active_pane(), &pane_1);
            });
        });
    }
}
