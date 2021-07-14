pub mod pane;
pub mod pane_group;

use crate::{
    editor::{Buffer, Editor},
    fs::Fs,
    language::LanguageRegistry,
    rpc,
    settings::Settings,
    worktree::{File, Worktree},
    AppState,
};
use anyhow::{anyhow, Result};
use gpui::{
    color::rgbu, elements::*, json::to_string_pretty, keymap::Binding, AnyViewHandle, AppContext,
    ClipboardItem, Entity, ModelHandle, MutableAppContext, PathPromptOptions, PromptLevel, Task,
    View, ViewContext, ViewHandle, WeakModelHandle,
};
use log::error;
pub use pane::*;
pub use pane_group::*;
use postage::watch;
use smol::prelude::*;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_global_action("workspace:open", open);
    cx.add_global_action(
        "workspace:open_paths",
        |params: &OpenParams, cx: &mut MutableAppContext| open_paths(params, cx).detach(),
    );
    cx.add_global_action("workspace:new_file", open_new);
    cx.add_global_action("workspace:join_worktree", join_worktree);
    cx.add_action("workspace:save", Workspace::save_active_item);
    cx.add_action("workspace:debug_elements", Workspace::debug_elements);
    cx.add_action("workspace:new_file", Workspace::open_new_file);
    cx.add_action("workspace:share_worktree", Workspace::share_worktree);
    cx.add_action("workspace:join_worktree", Workspace::join_worktree);
    cx.add_bindings(vec![
        Binding::new("cmd-s", "workspace:save", None),
        Binding::new("cmd-alt-i", "workspace:debug_elements", None),
    ]);
    pane::init(cx);
}

pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub app_state: Arc<AppState>,
}

fn open(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    let app_state = app_state.clone();
    cx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, cx| {
            if let Some(paths) = paths {
                cx.dispatch_global_action("workspace:open_paths", OpenParams { paths, app_state });
            }
        },
    );
}

fn open_paths(params: &OpenParams, cx: &mut MutableAppContext) -> Task<()> {
    log::info!("open paths {:?}", params.paths);

    // Open paths in existing workspace if possible
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = cx.root_view::<Workspace>(window_id) {
            let task = handle.update(cx, |view, cx| {
                if view.contains_paths(&params.paths, cx.as_ref()) {
                    log::info!("open paths on existing workspace");
                    Some(view.open_paths(&params.paths, cx))
                } else {
                    None
                }
            });

            if let Some(task) = task {
                return task;
            }
        }
    }

    log::info!("open new workspace");

    // Add a new workspace if necessary
    let (_, workspace) = cx.add_window(|cx| Workspace::new(&params.app_state, cx));
    workspace.update(cx, |workspace, cx| workspace.open_paths(&params.paths, cx))
}

fn open_new(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_window(|cx| {
        let mut view = Workspace::new(app_state.as_ref(), cx);
        view.open_new_file(&app_state, cx);
        view
    });
}

fn join_worktree(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_window(|cx| {
        let mut view = Workspace::new(app_state.as_ref(), cx);
        view.join_worktree(&app_state, cx);
        view
    });
}

pub trait Item: Entity + Sized {
    type View: ItemView;

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View;

    fn file(&self) -> Option<&File>;
}

pub trait ItemView: View {
    fn title(&self, cx: &AppContext) -> String;
    fn entry_id(&self, cx: &AppContext) -> Option<(usize, Arc<Path>)>;
    fn clone_on_split(&self, _: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
    fn is_dirty(&self, _: &AppContext) -> bool {
        false
    }
    fn has_conflict(&self, _: &AppContext) -> bool {
        false
    }
    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>>;
    fn save_as(
        &mut self,
        worktree: &ModelHandle<Worktree>,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>>;
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
    fn file<'a>(&'a self, cx: &'a AppContext) -> Option<&'a File>;
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemViewHandle>>;
    fn alive(&self, cx: &AppContext) -> bool;
}

pub trait ItemViewHandle: Send + Sync {
    fn title(&self, cx: &AppContext) -> String;
    fn entry_id(&self, cx: &AppContext) -> Option<(usize, Arc<Path>)>;
    fn boxed_clone(&self) -> Box<dyn ItemViewHandle>;
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>>;
    fn set_parent_pane(&self, pane: &ViewHandle<Pane>, cx: &mut MutableAppContext);
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn save(&self, cx: &mut MutableAppContext) -> Result<Task<Result<()>>>;
    fn save_as(
        &self,
        worktree: &ModelHandle<Worktree>,
        path: &Path,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>>;
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
    fn file<'a>(&'a self, cx: &'a AppContext) -> Option<&'a File> {
        self.upgrade(cx).and_then(|h| h.read(cx).file())
    }

    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemViewHandle>> {
        if let Some(handle) = self.upgrade(cx.as_ref()) {
            Some(Box::new(cx.add_view(window_id, |cx| {
                T::build_view(handle, settings, cx)
            })))
        } else {
            None
        }
    }

    fn alive(&self, cx: &AppContext) -> bool {
        self.upgrade(cx).is_some()
    }
}

impl<T: ItemView> ItemViewHandle for ViewHandle<T> {
    fn title(&self, cx: &AppContext) -> String {
        self.read(cx).title(cx)
    }

    fn entry_id(&self, cx: &AppContext) -> Option<(usize, Arc<Path>)> {
        self.read(cx).entry_id(cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemViewHandle> {
        Box::new(self.clone())
    }

    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>> {
        self.update(cx, |item, cx| {
            cx.add_option_view(|cx| item.clone_on_split(cx))
        })
        .map(|handle| Box::new(handle) as Box<dyn ItemViewHandle>)
    }

    fn set_parent_pane(&self, pane: &ViewHandle<Pane>, cx: &mut MutableAppContext) {
        pane.update(cx, |_, cx| {
            cx.subscribe_to_view(self, |pane, item, event, cx| {
                if T::should_activate_item_on_event(event) {
                    if let Some(ix) = pane.item_index(&item) {
                        pane.activate_item(ix, cx);
                        pane.activate(cx);
                    }
                }
                if T::should_update_tab_on_event(event) {
                    cx.notify()
                }
            })
        })
    }

    fn save(&self, cx: &mut MutableAppContext) -> Result<Task<Result<()>>> {
        self.update(cx, |item, cx| item.save(cx))
    }

    fn save_as(
        &self,
        worktree: &ModelHandle<Worktree>,
        path: &Path,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(worktree, path, cx))
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.read(cx).has_conflict(cx)
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
    languages: Arc<LanguageRegistry>,
    rpc: rpc::Client,
    fs: Arc<dyn Fs>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    worktrees: HashSet<ModelHandle<Worktree>>,
    items: Vec<Box<dyn WeakItemHandle>>,
    loading_items: HashMap<
        (usize, Arc<Path>),
        postage::watch::Receiver<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>,
    >,
}

impl Workspace {
    pub fn new(app_state: &AppState, cx: &mut ViewContext<Self>) -> Self {
        let pane = cx.add_view(|_| Pane::new(app_state.settings.clone()));
        let pane_id = pane.id();
        cx.subscribe_to_view(&pane, move |me, _, event, cx| {
            me.handle_pane_event(pane_id, event, cx)
        });
        cx.focus(&pane);

        Workspace {
            modal: None,
            center: PaneGroup::new(pane.id()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            settings: app_state.settings.clone(),
            languages: app_state.languages.clone(),
            rpc: app_state.rpc.clone(),
            fs: app_state.fs.clone(),
            worktrees: Default::default(),
            items: Default::default(),
            loading_items: Default::default(),
        }
    }

    pub fn worktrees(&self) -> &HashSet<ModelHandle<Worktree>> {
        &self.worktrees
    }

    pub fn contains_paths(&self, paths: &[PathBuf], cx: &AppContext) -> bool {
        paths.iter().all(|path| self.contains_path(&path, cx))
    }

    pub fn contains_path(&self, path: &Path, cx: &AppContext) -> bool {
        for worktree in &self.worktrees {
            let worktree = worktree.read(cx).as_local();
            if worktree.map_or(false, |w| w.contains_abs_path(path)) {
                return true;
            }
        }
        false
    }

    pub fn worktree_scans_complete(&self, cx: &AppContext) -> impl Future<Output = ()> + 'static {
        let futures = self
            .worktrees
            .iter()
            .filter_map(|worktree| worktree.read(cx).as_local())
            .map(|worktree| worktree.scan_complete())
            .collect::<Vec<_>>();
        async move {
            for future in futures {
                future.await;
            }
        }
    }

    pub fn open_paths(&mut self, abs_paths: &[PathBuf], cx: &mut ViewContext<Self>) -> Task<()> {
        let entries = abs_paths
            .iter()
            .cloned()
            .map(|path| self.entry_id_for_path(&path, cx))
            .collect::<Vec<_>>();

        let fs = self.fs.clone();
        let tasks = abs_paths
            .iter()
            .cloned()
            .zip(entries.into_iter())
            .map(|(abs_path, entry_id)| {
                cx.spawn(|this, mut cx| {
                    let fs = fs.clone();
                    async move {
                        let entry_id = entry_id.await?;
                        if fs.is_file(&abs_path).await {
                            if let Some(entry) =
                                this.update(&mut cx, |this, cx| this.open_entry(entry_id, cx))
                            {
                                entry.await;
                            }
                        }
                        Ok(())
                    }
                })
            })
            .collect::<Vec<Task<Result<()>>>>();

        cx.foreground().spawn(async move {
            for task in tasks {
                if let Err(error) = task.await {
                    log::error!("error opening paths {}", error);
                }
            }
        })
    }

    fn worktree_for_abs_path(
        &self,
        abs_path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        let abs_path: Arc<Path> = Arc::from(abs_path);
        cx.spawn(|this, mut cx| async move {
            let mut entry_id = None;
            this.read_with(&cx, |this, cx| {
                for tree in this.worktrees.iter() {
                    if let Some(relative_path) = tree
                        .read(cx)
                        .as_local()
                        .and_then(|t| abs_path.strip_prefix(t.abs_path()).ok())
                    {
                        entry_id = Some((tree.clone(), relative_path.into()));
                        break;
                    }
                }
            });

            if let Some(entry_id) = entry_id {
                Ok(entry_id)
            } else {
                let worktree = this
                    .update(&mut cx, |this, cx| this.add_worktree(&abs_path, cx))
                    .await?;
                Ok((worktree, PathBuf::new()))
            }
        })
    }

    fn entry_id_for_path(
        &self,
        abs_path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<(usize, Arc<Path>)>> {
        let entry = self.worktree_for_abs_path(abs_path, cx);
        cx.spawn(|_, _| async move {
            let (worktree, path) = entry.await?;
            Ok((worktree.id(), path.into()))
        })
    }

    pub fn add_worktree(
        &self,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let languages = self.languages.clone();
        let fs = self.fs.clone();
        let path = Arc::from(path);
        cx.spawn(|this, mut cx| async move {
            let worktree = Worktree::open_local(path, languages, fs, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                cx.observe_model(&worktree, |_, _, cx| cx.notify());
                this.worktrees.insert(worktree.clone());
                cx.notify();
            });
            Ok(worktree)
        })
    }

    pub fn toggle_modal<V, F>(&mut self, cx: &mut ViewContext<Self>, add_view: F)
    where
        V: 'static + View,
        F: FnOnce(&mut ViewContext<Self>, &mut Self) -> ViewHandle<V>,
    {
        if self.modal.as_ref().map_or(false, |modal| modal.is::<V>()) {
            self.modal.take();
            cx.focus_self();
        } else {
            let modal = add_view(cx, self);
            cx.focus(&modal);
            self.modal = Some(modal.into());
        }
        cx.notify();
    }

    pub fn modal(&self) -> Option<&AnyViewHandle> {
        self.modal.as_ref()
    }

    pub fn dismiss_modal(&mut self, cx: &mut ViewContext<Self>) {
        if self.modal.take().is_some() {
            cx.focus(&self.active_pane);
            cx.notify();
        }
    }

    pub fn open_new_file(&mut self, _: &AppState, cx: &mut ViewContext<Self>) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
        let buffer_view =
            cx.add_view(|cx| Editor::for_buffer(buffer.clone(), self.settings.clone(), cx));
        self.items.push(ItemHandle::downgrade(&buffer));
        self.active_pane()
            .add_item_view(Box::new(buffer_view), cx.as_mut());
    }

    #[must_use]
    pub fn open_entry(
        &mut self,
        entry: (usize, Arc<Path>),
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<()>> {
        let pane = self.active_pane().clone();
        if self.activate_or_open_existing_entry(entry.clone(), &pane, cx) {
            return None;
        }

        let (worktree_id, path) = entry.clone();

        let worktree = match self.worktrees.get(&worktree_id).cloned() {
            Some(worktree) => worktree,
            None => {
                log::error!("worktree {} does not exist", worktree_id);
                return None;
            }
        };

        if let Entry::Vacant(entry) = self.loading_items.entry(entry.clone()) {
            let (mut tx, rx) = postage::watch::channel();
            entry.insert(rx);

            cx.as_mut()
                .spawn(|mut cx| async move {
                    let buffer = worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.open_buffer(path.as_ref(), cx)
                        })
                        .await;
                    *tx.borrow_mut() = Some(
                        buffer
                            .map(|buffer| Box::new(buffer) as Box<dyn ItemHandle>)
                            .map_err(Arc::new),
                    );
                })
                .detach();
        }

        let pane = pane.downgrade();
        let settings = self.settings.clone();
        let mut watch = self.loading_items.get(&entry).unwrap().clone();

        Some(cx.spawn(|this, mut cx| async move {
            let load_result = loop {
                if let Some(load_result) = watch.borrow().as_ref() {
                    break load_result.clone();
                }
                watch.next().await;
            };

            this.update(&mut cx, |this, cx| {
                this.loading_items.remove(&entry);
                if let Some(pane) = pane.upgrade(&cx) {
                    match load_result {
                        Ok(item) => {
                            // By the time loading finishes, the entry could have been already added
                            // to the pane. If it was, we activate it, otherwise we'll store the
                            // item and add a new view for it.
                            if !this.activate_or_open_existing_entry(entry, &pane, cx) {
                                let weak_item = item.downgrade();
                                let view = weak_item
                                    .add_view(cx.window_id(), settings, cx.as_mut())
                                    .unwrap();
                                this.items.push(weak_item);
                                pane.add_item_view(view, cx.as_mut());
                            }
                        }
                        Err(error) => {
                            log::error!("error opening item: {}", error);
                        }
                    }
                }
            })
        }))
    }

    fn activate_or_open_existing_entry(
        &mut self,
        entry: (usize, Arc<Path>),
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        // If the pane contains a view for this file, then activate
        // that item view.
        if pane.update(cx, |pane, cx| pane.activate_entry(entry.clone(), cx)) {
            return true;
        }

        // Otherwise, if this file is already open somewhere in the workspace,
        // then add another view for it.
        let settings = self.settings.clone();
        let mut view_for_existing_item = None;
        self.items.retain(|item| {
            if item.alive(cx.as_ref()) {
                if view_for_existing_item.is_none()
                    && item
                        .file(cx.as_ref())
                        .map_or(false, |file| file.entry_id() == entry)
                {
                    view_for_existing_item = Some(
                        item.add_view(cx.window_id(), settings.clone(), cx.as_mut())
                            .unwrap(),
                    );
                }
                true
            } else {
                false
            }
        });
        if let Some(view) = view_for_existing_item {
            pane.add_item_view(view, cx.as_mut());
            true
        } else {
            false
        }
    }

    pub fn active_item(&self, cx: &ViewContext<Self>) -> Option<Box<dyn ItemViewHandle>> {
        self.active_pane().read(cx).active_item()
    }

    pub fn save_active_item(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if let Some(item) = self.active_item(cx) {
            let handle = cx.handle();
            if item.entry_id(cx.as_ref()).is_none() {
                let worktree = self.worktrees.iter().next();
                let start_abs_path = worktree
                    .and_then(|w| w.read(cx).as_local())
                    .map_or(Path::new(""), |w| w.abs_path())
                    .to_path_buf();
                cx.prompt_for_new_path(&start_abs_path, move |abs_path, cx| {
                    if let Some(abs_path) = abs_path {
                        cx.spawn(|mut cx| async move {
                            let result = match handle
                                .update(&mut cx, |this, cx| {
                                    this.worktree_for_abs_path(&abs_path, cx)
                                })
                                .await
                            {
                                Ok((worktree, path)) => {
                                    handle
                                        .update(&mut cx, |_, cx| {
                                            item.save_as(&worktree, &path, cx.as_mut())
                                        })
                                        .await
                                }
                                Err(error) => Err(error),
                            };

                            if let Err(error) = result {
                                error!("failed to save item: {:?}, ", error);
                            }
                        })
                        .detach()
                    }
                });
                return;
            } else if item.has_conflict(cx.as_ref()) {
                const CONFLICT_MESSAGE: &'static str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";

                cx.prompt(
                    PromptLevel::Warning,
                    CONFLICT_MESSAGE,
                    &["Overwrite", "Cancel"],
                    move |answer, cx| {
                        if answer == 0 {
                            cx.spawn(|mut cx| async move {
                                if let Err(error) = cx.update(|cx| item.save(cx)).unwrap().await {
                                    error!("failed to save item: {:?}, ", error);
                                }
                            })
                            .detach();
                        }
                    },
                );
            } else {
                cx.spawn(|_, mut cx| async move {
                    if let Err(error) = cx.update(|cx| item.save(cx)).unwrap().await {
                        error!("failed to save item: {:?}, ", error);
                    }
                })
                .detach();
            }
        }
    }

    pub fn debug_elements(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        match to_string_pretty(&cx.debug_elements()) {
            Ok(json) => {
                let kib = json.len() as f32 / 1024.;
                cx.as_mut().write_to_clipboard(ClipboardItem::new(json));
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

    fn share_worktree(&mut self, app_state: &Arc<AppState>, cx: &mut ViewContext<Self>) {
        let rpc = self.rpc.clone();
        let platform = cx.platform();
        let router = app_state.rpc_router.clone();

        let task = cx.spawn(|this, mut cx| async move {
            rpc.log_in_and_connect(router, cx.clone()).await?;

            let share_task = this.update(&mut cx, |this, cx| {
                let worktree = this.worktrees.iter().next()?;
                worktree.update(cx, |worktree, cx| {
                    let worktree = worktree.as_local_mut()?;
                    Some(worktree.share(rpc, cx))
                })
            });

            if let Some(share_task) = share_task {
                let (worktree_id, access_token) = share_task.await?;
                let worktree_url = rpc::encode_worktree_url(worktree_id, &access_token);
                log::info!("wrote worktree url to clipboard: {}", worktree_url);
                platform.write_to_clipboard(ClipboardItem::new(worktree_url));
            }
            surf::Result::Ok(())
        });

        cx.spawn(|_, _| async move {
            if let Err(e) = task.await {
                log::error!("sharing failed: {:?}", e);
            }
        })
        .detach();
    }

    fn join_worktree(&mut self, app_state: &Arc<AppState>, cx: &mut ViewContext<Self>) {
        let rpc = self.rpc.clone();
        let languages = self.languages.clone();
        let router = app_state.rpc_router.clone();

        let task = cx.spawn(|this, mut cx| async move {
            rpc.log_in_and_connect(router, cx.clone()).await?;

            let worktree_url = cx
                .platform()
                .read_from_clipboard()
                .ok_or_else(|| anyhow!("failed to read url from clipboard"))?;
            let (worktree_id, access_token) = rpc::decode_worktree_url(worktree_url.text())
                .ok_or_else(|| anyhow!("failed to decode worktree url"))?;
            log::info!("read worktree url from clipboard: {}", worktree_url.text());

            let worktree =
                Worktree::open_remote(rpc.clone(), worktree_id, access_token, languages, &mut cx)
                    .await?;
            this.update(&mut cx, |workspace, cx| {
                cx.observe_model(&worktree, |_, _, cx| cx.notify());
                workspace.worktrees.insert(worktree);
                cx.notify();
            });

            surf::Result::Ok(())
        });

        cx.spawn(|_, _| async move {
            if let Err(e) = task.await {
                log::error!("joining failed: {}", e);
            }
        })
        .detach();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = cx.add_view(|_| Pane::new(self.settings.clone()));
        let pane_id = pane.id();
        cx.subscribe_to_view(&pane, move |me, _, event, cx| {
            me.handle_pane_event(pane_id, event, cx)
        });
        self.panes.push(pane.clone());
        self.activate_pane(pane.clone(), cx);
        pane
    }

    fn activate_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        self.active_pane = pane;
        cx.focus(&self.active_pane);
        cx.notify();
    }

    fn handle_pane_event(
        &mut self,
        pane_id: usize,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(pane) = self.pane(pane_id) {
            match event {
                pane::Event::Split(direction) => {
                    self.split_pane(pane, *direction, cx);
                }
                pane::Event::Remove => {
                    self.remove_pane(pane, cx);
                }
                pane::Event::Activate => {
                    self.activate_pane(pane, cx);
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
        cx: &mut ViewContext<Self>,
    ) -> ViewHandle<Pane> {
        let new_pane = self.add_pane(cx);
        self.activate_pane(new_pane.clone(), cx);
        if let Some(item) = pane.read(cx).active_item() {
            if let Some(clone) = item.clone_on_split(cx.as_mut()) {
                new_pane.add_item_view(clone, cx.as_mut());
            }
        }
        self.center
            .split(pane.id(), new_pane.id(), direction)
            .unwrap();
        cx.notify();
        new_pane
    }

    fn remove_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.center.remove(pane.id()).unwrap() {
            self.panes.retain(|p| p != &pane);
            self.activate_pane(self.panes.last().unwrap().clone(), cx);
        }
    }

    fn pane(&self, pane_id: usize) -> Option<ViewHandle<Pane>> {
        self.panes.iter().find(|pane| pane.id() == pane_id).cloned()
    }

    pub fn active_pane(&self) -> &ViewHandle<Pane> {
        &self.active_pane
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

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.active_pane);
    }
}

#[cfg(test)]
pub trait WorkspaceHandle {
    fn file_entries(&self, cx: &AppContext) -> Vec<(usize, Arc<Path>)>;
}

#[cfg(test)]
impl WorkspaceHandle for ViewHandle<Workspace> {
    fn file_entries(&self, cx: &AppContext) -> Vec<(usize, Arc<Path>)> {
        self.read(cx)
            .worktrees()
            .iter()
            .flat_map(|tree| {
                let tree_id = tree.id();
                tree.read(cx)
                    .files(0)
                    .map(move |f| (tree_id, f.path().clone()))
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::Editor,
        fs::FakeFs,
        test::{build_app_state, temp_tree},
        worktree::WorktreeHandle,
    };
    use serde_json::json;
    use std::{collections::HashSet, fs};
    use tempdir::TempDir;

    #[gpui::test]
    async fn test_open_paths_action(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
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

        cx.update(|cx| {
            open_paths(
                &OpenParams {
                    paths: vec![
                        dir.path().join("a").to_path_buf(),
                        dir.path().join("b").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                },
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| {
            open_paths(
                &OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    app_state: app_state.clone(),
                },
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_view_1 = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        workspace_view_1.read_with(&cx, |workspace, _| {
            assert_eq!(workspace.worktrees().len(), 2)
        });

        cx.update(|cx| {
            open_paths(
                &OpenParams {
                    paths: vec![
                        dir.path().join("b").to_path_buf(),
                        dir.path().join("c").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                },
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 2);
    }

    #[gpui::test]
    async fn test_open_entry(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "a": {
                "file1": "contents 1",
                "file2": "contents 2",
                "file3": "contents 3",
            },
        }));

        let app_state = cx.read(build_app_state);

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(dir.path(), cx)
            })
            .await
            .unwrap();

        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_entries(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        // Open the first entry
        workspace
            .update(&mut cx, |w, cx| w.open_entry(file1.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().entry_id(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().len(), 1);
        });

        // Open the second entry
        workspace
            .update(&mut cx, |w, cx| w.open_entry(file2.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().entry_id(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.items().len(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        workspace.update(&mut cx, |w, cx| {
            assert!(w.open_entry(file1.clone(), cx).is_none())
        });
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().entry_id(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().len(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        workspace.update(&mut cx, |w, cx| {
            w.split_pane(w.active_pane().clone(), SplitDirection::Right, cx);
            assert!(w.open_entry(file2.clone(), cx).is_none());
            assert_eq!(
                w.active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .entry_id(cx.as_ref()),
                Some(file2.clone())
            );
        });

        // Open the third entry twice concurrently. Only one pane item is added.
        let (t1, t2) = workspace.update(&mut cx, |w, cx| {
            (
                w.open_entry(file3.clone(), cx).unwrap(),
                w.open_entry(file3.clone(), cx).unwrap(),
            )
        });
        t1.await;
        t2.await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().entry_id(cx),
                Some(file3.clone())
            );
            let pane_entries = pane
                .items()
                .iter()
                .map(|i| i.entry_id(cx).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pane_entries, &[file1, file2, file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(mut cx: gpui::TestAppContext) {
        let fs = FakeFs::new();
        fs.insert_dir("/dir1").await.unwrap();
        fs.insert_dir("/dir2").await.unwrap();
        fs.insert_file("/dir1/a.txt", "".into()).await.unwrap();
        fs.insert_file("/dir2/b.txt", "".into()).await.unwrap();

        let mut app_state = cx.read(build_app_state);
        Arc::get_mut(&mut app_state).unwrap().fs = Arc::new(fs);

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree("/dir1".as_ref(), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| view.open_paths(&["/dir1/a.txt".into()], cx))
        })
        .await;
        cx.read(|cx| {
            assert_eq!(
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .title(cx),
                "a.txt"
            );
        });

        // Open a file outside of any existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| view.open_paths(&["/dir2/b.txt".into()], cx))
        })
        .await;
        cx.read(|cx| {
            let worktree_roots = workspace
                .read(cx)
                .worktrees()
                .iter()
                .map(|w| w.read(cx).as_local().unwrap().abs_path())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec!["/dir1", "/dir2/b.txt"]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );
            assert_eq!(
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .title(cx),
                "b.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "a.txt": "",
        }));

        let app_state = cx.read(build_app_state);
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(dir.path(), cx)
            })
            .await
            .unwrap();
        let tree = cx.read(|cx| {
            let mut trees = workspace.read(cx).worktrees().iter();
            trees.next().unwrap().clone()
        });
        tree.flush_fs_events(&cx).await;

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(&[dir.path().join("a.txt")], cx)
            })
        })
        .await;
        let editor = cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            let item = pane.active_item().unwrap();
            item.to_any().downcast::<Editor>().unwrap()
        });

        cx.update(|cx| editor.update(cx, |editor, cx| editor.insert(&"x".to_string(), cx)));
        fs::write(dir.path().join("a.txt"), "changed").unwrap();
        editor
            .condition(&cx, |editor, cx| editor.has_conflict(cx))
            .await;
        cx.read(|cx| assert!(editor.is_dirty(cx)));

        cx.update(|cx| workspace.update(cx, |w, cx| w.save_active_item(&(), cx)));
        cx.simulate_prompt_answer(window_id, 0);
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| assert!(!editor.has_conflict(cx)));
    }

    #[gpui::test]
    async fn test_open_and_save_new_file(mut cx: gpui::TestAppContext) {
        let dir = TempDir::new("test-new-file").unwrap();
        let app_state = cx.read(build_app_state);
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(dir.path(), cx)
            })
            .await
            .unwrap();
        let tree = cx.read(|cx| {
            workspace
                .read(cx)
                .worktrees()
                .iter()
                .next()
                .unwrap()
                .clone()
        });
        tree.flush_fs_events(&cx).await;

        // Create a new untitled buffer
        let editor = workspace.update(&mut cx, |workspace, cx| {
            workspace.open_new_file(&app_state, cx);
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(&mut cx, |editor, cx| {
            assert!(!editor.is_dirty(cx.as_ref()));
            assert_eq!(editor.title(cx.as_ref()), "untitled");
            editor.insert(&"hi".to_string(), cx);
            assert!(editor.is_dirty(cx.as_ref()));
        });

        // Save the buffer. This prompts for a filename.
        workspace.update(&mut cx, |workspace, cx| workspace.save_active_item(&(), cx));
        cx.simulate_new_path_selection(|parent_dir| {
            assert_eq!(parent_dir, dir.path());
            Some(parent_dir.join("the-new-name"))
        });
        cx.read(|cx| {
            assert!(editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "untitled");
        });

        // When the save completes, the buffer's title is updated.
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name");
        });

        // Edit the file and save it again. This time, there is no filename prompt.
        editor.update(&mut cx, |editor, cx| {
            editor.insert(&" there".to_string(), cx);
            assert_eq!(editor.is_dirty(cx.as_ref()), true);
        });
        workspace.update(&mut cx, |workspace, cx| workspace.save_active_item(&(), cx));
        assert!(!cx.did_prompt_for_new_path());
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| assert_eq!(editor.title(cx), "the-new-name"));

        // Open the same newly-created file in another pane item. The new editor should reuse
        // the same buffer.
        workspace.update(&mut cx, |workspace, cx| {
            workspace.open_new_file(&app_state, cx);
            workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
            assert!(workspace
                .open_entry((tree.id(), Path::new("the-new-name").into()), cx)
                .is_none());
        });
        let editor2 = workspace.update(&mut cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap()
        });
        cx.read(|cx| {
            assert_eq!(editor2.read(cx).buffer(), editor.read(cx).buffer());
        })
    }

    #[gpui::test]
    async fn test_new_empty_workspace(mut cx: gpui::TestAppContext) {
        cx.update(init);

        let app_state = cx.read(build_app_state);
        cx.dispatch_global_action("workspace:new_file", app_state);
        let window_id = *cx.window_ids().first().unwrap();
        let workspace = cx.root_view::<Workspace>(window_id).unwrap();
        workspace.update(&mut cx, |workspace, cx| {
            let editor = workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap();
            assert!(editor.read(cx).text(cx.as_ref()).is_empty());
        });
    }

    #[gpui::test]
    async fn test_pane_actions(mut cx: gpui::TestAppContext) {
        cx.update(|cx| pane::init(cx));

        let dir = temp_tree(json!({
            "a": {
                "file1": "contents 1",
                "file2": "contents 2",
                "file3": "contents 3",
            },
        }));

        let app_state = cx.read(build_app_state);
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(dir.path(), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_entries(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        workspace
            .update(&mut cx, |w, cx| w.open_entry(file1.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            assert_eq!(
                pane_1.read(cx).active_item().unwrap().entry_id(cx),
                Some(file1.clone())
            );
        });

        cx.dispatch_action(window_id, vec![pane_1.id()], "pane:split_right", ());
        cx.update(|cx| {
            let pane_2 = workspace.read(cx).active_pane().clone();
            assert_ne!(pane_1, pane_2);

            let pane2_item = pane_2.read(cx).active_item().unwrap();
            assert_eq!(pane2_item.entry_id(cx.as_ref()), Some(file1.clone()));

            cx.dispatch_action(window_id, vec![pane_2.id()], "pane:close_active_item", ());
            let workspace_view = workspace.read(cx);
            assert_eq!(workspace_view.panes.len(), 1);
            assert_eq!(workspace_view.active_pane(), &pane_1);
        });
    }
}
