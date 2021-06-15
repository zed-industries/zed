pub mod pane;
pub mod pane_group;

use crate::{
    editor::{Buffer, Editor},
    language::LanguageRegistry,
    rpc_client::RpcClient,
    settings::Settings,
    time::ReplicaId,
    util::SurfResultExt as _,
    worktree::{FileHandle, Worktree, WorktreeHandle},
    AppState,
};
use anyhow::{anyhow, Context as _};
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
    convert::TryFrom,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use surf::Url;
use zed_rpc::{proto, rest::CreateWorktreeResponse};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_global_action("workspace:open", open);
    cx.add_global_action("workspace:open_paths", open_paths);
    cx.add_action("workspace:save", Workspace::save_active_item);
    cx.add_action("workspace:debug_elements", Workspace::debug_elements);
    cx.add_action("workspace:new_file", Workspace::open_new_file);
    cx.add_action("workspace:share_worktree", Workspace::share_worktree);
    cx.add_bindings(vec![
        Binding::new("cmd-s", "workspace:save", None),
        Binding::new("cmd-alt-i", "workspace:debug_elements", None),
    ]);
    pane::init(cx);
}

pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub app_state: AppState,
}

fn open(app_state: &AppState, cx: &mut MutableAppContext) {
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

fn open_paths(params: &OpenParams, cx: &mut MutableAppContext) {
    log::info!("open paths {:?}", params.paths);

    // Open paths in existing workspace if possible
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = cx.root_view::<Workspace>(window_id) {
            if handle.update(cx, |view, cx| {
                if view.contains_paths(&params.paths, cx.as_ref()) {
                    let open_paths = view.open_paths(&params.paths, cx);
                    cx.foreground().spawn(open_paths).detach();
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
    cx.add_window(|cx| {
        let mut view = Workspace::new(
            0,
            params.app_state.settings.clone(),
            params.app_state.language_registry.clone(),
            cx,
        );
        let open_paths = view.open_paths(&params.paths, cx);
        cx.foreground().spawn(open_paths).detach();
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

    fn file(&self) -> Option<&FileHandle>;
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
    fn save(
        &mut self,
        _: Option<FileHandle>,
        _: &mut ViewContext<Self>,
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
    fn file<'a>(&'a self, cx: &'a AppContext) -> Option<&'a FileHandle>;
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
    fn save(
        &self,
        file: Option<FileHandle>,
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
    fn file<'a>(&'a self, cx: &'a AppContext) -> Option<&'a FileHandle> {
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

    fn save(
        &self,
        file: Option<FileHandle>,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save(file, cx))
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
    language_registry: Arc<LanguageRegistry>,
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
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let pane = cx.add_view(|_| Pane::new(settings.clone()));
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
            settings,
            language_registry,
            replica_id,
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
        self.worktrees
            .iter()
            .any(|worktree| worktree.read(cx).contains_abs_path(path))
    }

    pub fn worktree_scans_complete(&self, cx: &AppContext) -> impl Future<Output = ()> + 'static {
        let futures = self
            .worktrees
            .iter()
            .map(|worktree| worktree.read(cx).scan_complete())
            .collect::<Vec<_>>();
        async move {
            for future in futures {
                future.await;
            }
        }
    }

    pub fn open_paths(
        &mut self,
        abs_paths: &[PathBuf],
        cx: &mut ViewContext<Self>,
    ) -> impl Future<Output = ()> {
        let entries = abs_paths
            .iter()
            .cloned()
            .map(|path| self.file_for_path(&path, cx))
            .collect::<Vec<_>>();

        let bg = cx.background_executor().clone();
        let tasks = abs_paths
            .iter()
            .cloned()
            .zip(entries.into_iter())
            .map(|(abs_path, file)| {
                let is_file = bg.spawn(async move { abs_path.is_file() });
                cx.spawn(|this, mut cx| async move {
                    let file = file.await;
                    let is_file = is_file.await;
                    this.update(&mut cx, |this, cx| {
                        if is_file {
                            this.open_entry(file.entry_id(), cx)
                        } else {
                            None
                        }
                    })
                })
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

    fn file_for_path(&mut self, abs_path: &Path, cx: &mut ViewContext<Self>) -> Task<FileHandle> {
        for tree in self.worktrees.iter() {
            if let Ok(relative_path) = abs_path.strip_prefix(tree.read(cx).abs_path()) {
                return tree.file(relative_path, cx.as_mut());
            }
        }
        let worktree = self.add_worktree(&abs_path, cx);
        worktree.file(Path::new(""), cx.as_mut())
    }

    pub fn add_worktree(
        &mut self,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> ModelHandle<Worktree> {
        let worktree = cx.add_model(|cx| Worktree::new(path, cx));
        cx.observe_model(&worktree, |_, _, cx| cx.notify());
        self.worktrees.insert(worktree.clone());
        cx.notify();
        worktree
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

    pub fn open_new_file(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let buffer = cx.add_model(|cx| Buffer::new(self.replica_id, "", cx));
        let buffer_view =
            cx.add_view(|cx| Editor::for_buffer(buffer.clone(), self.settings.clone(), cx));
        self.items.push(ItemHandle::downgrade(&buffer));
        self.add_item_view(Box::new(buffer_view), cx);
    }

    #[must_use]
    pub fn open_entry(
        &mut self,
        entry: (usize, Arc<Path>),
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<()>> {
        // If the active pane contains a view for this file, then activate
        // that item view.
        if self
            .active_pane()
            .update(cx, |pane, cx| pane.activate_entry(entry.clone(), cx))
        {
            return None;
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
                        .map_or(false, |f| f.entry_id() == entry)
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
            self.add_item_view(view, cx);
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

        let file = worktree.file(path.clone(), cx.as_mut());
        if let Entry::Vacant(entry) = self.loading_items.entry(entry.clone()) {
            let (mut tx, rx) = postage::watch::channel();
            entry.insert(rx);
            let replica_id = self.replica_id;
            let language_registry = self.language_registry.clone();

            cx.as_mut()
                .spawn(|mut cx| async move {
                    let file = file.await;
                    let history = cx.read(|cx| file.load_history(cx));
                    let history = cx.background_executor().spawn(history).await;

                    *tx.borrow_mut() = Some(match history {
                        Ok(history) => Ok(Box::new(cx.add_model(|cx| {
                            let language = language_registry.select_language(path);
                            Buffer::from_history(
                                replica_id,
                                history,
                                Some(file),
                                language.cloned(),
                                cx,
                            )
                        }))),
                        Err(error) => Err(Arc::new(error)),
                    })
                })
                .detach();
        }

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
                match load_result {
                    Ok(item) => {
                        let weak_item = item.downgrade();
                        let view = weak_item
                            .add_view(cx.window_id(), settings, cx.as_mut())
                            .unwrap();
                        this.items.push(weak_item);
                        this.add_item_view(view, cx);
                    }
                    Err(error) => {
                        log::error!("error opening item: {}", error);
                    }
                }
            })
        }))
    }

    pub fn active_item(&self, cx: &ViewContext<Self>) -> Option<Box<dyn ItemViewHandle>> {
        self.active_pane().read(cx).active_item()
    }

    pub fn save_active_item(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if let Some(item) = self.active_item(cx) {
            let handle = cx.handle();
            if item.entry_id(cx.as_ref()).is_none() {
                let start_path = self
                    .worktrees
                    .iter()
                    .next()
                    .map_or(Path::new(""), |h| h.read(cx).abs_path())
                    .to_path_buf();
                cx.prompt_for_new_path(&start_path, move |path, cx| {
                    if let Some(path) = path {
                        cx.spawn(|mut cx| async move {
                            let file = handle
                                .update(&mut cx, |me, cx| me.file_for_path(&path, cx))
                                .await;
                            if let Err(error) = cx.update(|cx| item.save(Some(file), cx)).await {
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
                                if let Err(error) = cx.update(|cx| item.save(None, cx)).await {
                                    error!("failed to save item: {:?}, ", error);
                                }
                            })
                            .detach();
                        }
                    },
                );
            } else {
                cx.spawn(|_, mut cx| async move {
                    if let Err(error) = cx.update(|cx| item.save(None, cx)).await {
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

    fn share_worktree(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let zed_url = std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev".to_string());
        let executor = cx.background_executor().clone();

        let task = cx.spawn::<_, _, surf::Result<()>>(|this, mut cx| async move {
            let (user_id, access_token) =
                login(zed_url.clone(), cx.platform(), cx.background_executor()).await?;

            let mut response = surf::post(format!("{}/api/worktrees", &zed_url))
                .header(
                    "Authorization",
                    http_auth_basic::Credentials::new(&user_id, &access_token).as_http_header(),
                )
                .await
                .context("")?;

            let CreateWorktreeResponse {
                worktree_id,
                rpc_address,
            } = response.body_json().await?;

            eprintln!("got worktree response: {:?} {:?}", worktree_id, rpc_address);

            // TODO - If the `ZED_SERVER_URL` uses https, then wrap this stream in
            // a TLS stream using `native-tls`.
            let stream = smol::net::TcpStream::connect(rpc_address).await?;

            let rpc_client = RpcClient::new();
            let (connection_id, handler) = rpc_client.add_connection(stream).await;
            executor.spawn(handler).detach();

            let auth_response = rpc_client
                .request(
                    connection_id,
                    proto::Auth {
                        user_id: user_id.parse::<u64>()?,
                        access_token,
                    },
                )
                .await?;
            if !auth_response.credentials_valid {
                Err(anyhow!("failed to authenticate with RPC server"))?;
            }

            let share_task = this.update(&mut cx, |this, cx| {
                let worktree = this.worktrees.iter().next()?;
                Some(worktree.update(cx, |worktree, cx| {
                    worktree.share(rpc_client, connection_id, cx)
                }))
            });

            if let Some(share_task) = share_task {
                share_task.await?;
            }
            Ok(())
        });

        cx.spawn(|_, _| async move {
            if let Err(e) = task.await {
                log::error!("sharing failed: {}", e);
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
                self.add_item_view(clone, cx);
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

    fn add_item_view(&self, item: Box<dyn ItemViewHandle>, cx: &mut ViewContext<Self>) {
        let active_pane = self.active_pane();
        item.set_parent_pane(&active_pane, cx.as_mut());
        active_pane.update(cx, |pane, cx| {
            let item_idx = pane.add_item(item, cx);
            pane.activate_item(item_idx, cx);
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

fn login(
    zed_url: String,
    platform: Arc<dyn gpui::Platform>,
    executor: Arc<gpui::executor::Background>,
) -> Task<anyhow::Result<(String, String)>> {
    executor.clone().spawn(async move {
        if let Some((user_id, access_token)) = platform.read_credentials(&zed_url) {
            log::info!("already signed in. user_id: {}", user_id);
            return Ok((user_id, String::from_utf8(access_token).unwrap()));
        }

        // Generate a pair of asymmetric encryption keys. The public key will be used by the
        // zed server to encrypt the user's access token, so that it can'be intercepted by
        // any other app running on the user's device.
        let (public_key, private_key) =
            zed_rpc::auth::keypair().expect("failed to generate keypair for auth");
        let public_key_string =
            String::try_from(public_key).expect("failed to serialize public key for auth");

        // Start an HTTP server to receive the redirect from Zed's sign-in page.
        let server = tiny_http::Server::http("127.0.0.1:0").expect("failed to find open port");
        let port = server.server_addr().port();

        // Open the Zed sign-in page in the user's browser, with query parameters that indicate
        // that the user is signing in from a Zed app running on the same device.
        platform.open_url(&format!(
            "{}/sign_in?native_app_port={}&native_app_public_key={}",
            zed_url, port, public_key_string
        ));

        // Receive the HTTP request from the user's browser. Retrieve the user id and encrypted
        // access token from the query params.
        //
        // TODO - Avoid ever starting more than one HTTP server. Maybe switch to using a
        // custom URL scheme instead of this local HTTP server.
        let (user_id, access_token) = executor
            .spawn::<anyhow::Result<_>, _>(async move {
                if let Some(req) = server.recv_timeout(Duration::from_secs(10 * 60))? {
                    let path = req.url();
                    let mut user_id = None;
                    let mut access_token = None;
                    let url = Url::parse(&format!("http://example.com{}", path))
                        .context("failed to parse login notification url")?;
                    for (key, value) in url.query_pairs() {
                        if key == "access_token" {
                            access_token = Some(value.to_string());
                        } else if key == "user_id" {
                            user_id = Some(value.to_string());
                        }
                    }
                    req.respond(
                        tiny_http::Response::from_string(LOGIN_RESPONSE).with_header(
                            tiny_http::Header::from_bytes("Content-Type", "text/html").unwrap(),
                        ),
                    )
                    .context("failed to respond to login http request")?;
                    Ok(user_id.zip(access_token))
                } else {
                    Ok(None)
                }
            })
            .await?
            .ok_or_else(|| anyhow!(""))?;

        let access_token = private_key
            .decrypt_string(&access_token)
            .context("failed to decrypt access token")?;
        platform.activate(true);
        platform.write_credentials(&zed_url, &user_id, access_token.as_bytes());
        Ok((user_id.to_string(), access_token))
    })
}

const LOGIN_RESPONSE: &'static str = "
<!DOCTYPE html>
<html>
<script>window.close();</script>
</html>
";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::Editor,
        test::{build_app_state, temp_tree},
    };
    use serde_json::json;
    use std::{collections::HashSet, fs};
    use tempdir::TempDir;

    #[gpui::test]
    fn test_open_paths_action(cx: &mut gpui::MutableAppContext) {
        let app_state = build_app_state(cx.as_ref());

        init(cx);

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

        cx.dispatch_global_action(
            "workspace:open_paths",
            OpenParams {
                paths: vec![
                    dir.path().join("a").to_path_buf(),
                    dir.path().join("b").to_path_buf(),
                ],
                app_state: app_state.clone(),
            },
        );
        assert_eq!(cx.window_ids().count(), 1);

        cx.dispatch_global_action(
            "workspace:open_paths",
            OpenParams {
                paths: vec![dir.path().join("a").to_path_buf()],
                app_state: app_state.clone(),
            },
        );
        assert_eq!(cx.window_ids().count(), 1);
        let workspace_view_1 = cx
            .root_view::<Workspace>(cx.window_ids().next().unwrap())
            .unwrap();
        assert_eq!(workspace_view_1.read(cx).worktrees().len(), 2);

        cx.dispatch_global_action(
            "workspace:open_paths",
            OpenParams {
                paths: vec![
                    dir.path().join("b").to_path_buf(),
                    dir.path().join("c").to_path_buf(),
                ],
                app_state: app_state.clone(),
            },
        );
        assert_eq!(cx.window_ids().count(), 2);
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

        let (_, workspace) = cx.add_window(|cx| {
            let mut workspace =
                Workspace::new(0, app_state.settings, app_state.language_registry, cx);
            workspace.add_worktree(dir.path(), cx);
            workspace
        });

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

        // Open the third entry twice concurrently. Two pane items
        // are added.
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
            assert_eq!(pane_entries, &[file1, file2, file3.clone(), file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(mut cx: gpui::TestAppContext) {
        let dir1 = temp_tree(json!({
            "a.txt": "",
        }));
        let dir2 = temp_tree(json!({
            "b.txt": "",
        }));

        let app_state = cx.read(build_app_state);
        let (_, workspace) = cx.add_window(|cx| {
            let mut workspace =
                Workspace::new(0, app_state.settings, app_state.language_registry, cx);
            workspace.add_worktree(dir1.path(), cx);
            workspace
        });
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(&[dir1.path().join("a.txt")], cx)
            })
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
            workspace.update(cx, |view, cx| {
                view.open_paths(&[dir2.path().join("b.txt")], cx)
            })
        })
        .await;
        cx.read(|cx| {
            let worktree_roots = workspace
                .read(cx)
                .worktrees()
                .iter()
                .map(|w| w.read(cx).abs_path())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec![dir1.path(), &dir2.path().join("b.txt")]
                    .into_iter()
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
        let (window_id, workspace) = cx.add_window(|cx| {
            let mut workspace =
                Workspace::new(0, app_state.settings, app_state.language_registry, cx);
            workspace.add_worktree(dir.path(), cx);
            workspace
        });
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
        let (_, workspace) = cx.add_window(|cx| {
            let mut workspace =
                Workspace::new(0, app_state.settings, app_state.language_registry, cx);
            workspace.add_worktree(dir.path(), cx);
            workspace
        });
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
            workspace.open_new_file(&(), cx);
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
            workspace.open_new_file(&(), cx);
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
        let (window_id, workspace) = cx.add_window(|cx| {
            let mut workspace =
                Workspace::new(0, app_state.settings, app_state.language_registry, cx);
            workspace.add_worktree(dir.path(), cx);
            workspace
        });
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
