pub mod pane;
pub mod pane_group;
pub mod settings;
pub mod sidebar;
mod status_bar;

use anyhow::{anyhow, Result};
use client::{Authenticate, ChannelList, Client, User, UserStore};
use clock::ReplicaId;
use gpui::{
    action,
    color::Color,
    elements::*,
    geometry::{vector::vec2f, PathBuilder},
    json::{self, to_string_pretty, ToJson},
    keymap::Binding,
    platform::{CursorStyle, WindowOptions},
    AnyViewHandle, AppContext, ClipboardItem, Entity, ModelContext, ModelHandle, MutableAppContext,
    PathPromptOptions, PromptLevel, RenderContext, Task, View, ViewContext, ViewHandle,
    WeakModelHandle,
};
use language::LanguageRegistry;
use log::error;
pub use pane::*;
pub use pane_group::*;
use parking_lot::Mutex;
use postage::{prelude::Stream, watch};
use project::{fs, Fs, Project, ProjectPath, Worktree};
pub use settings::Settings;
use sidebar::{Side, Sidebar, SidebarItemId, ToggleSidebarItem, ToggleSidebarItemFocus};
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::{Theme, ThemeRegistry};

action!(Open, Arc<AppState>);
action!(OpenNew, Arc<AppState>);
action!(OpenPaths, OpenParams);
action!(ToggleShare);
action!(JoinProject, JoinProjectParams);
action!(Save);
action!(DebugElements);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_global_action(open);
    cx.add_global_action(move |action: &OpenPaths, cx: &mut MutableAppContext| {
        open_paths(&action.0.paths, &action.0.app_state, cx).detach();
    });
    cx.add_global_action(move |action: &OpenNew, cx: &mut MutableAppContext| {
        open_new(&action.0, cx)
    });
    cx.add_global_action(move |action: &JoinProject, cx: &mut MutableAppContext| {
        join_project(action.0.project_id, &action.0.app_state, cx).detach();
    });

    cx.add_action(Workspace::toggle_share);
    cx.add_action(Workspace::save_active_item);
    cx.add_action(Workspace::debug_elements);
    cx.add_action(Workspace::toggle_sidebar_item);
    cx.add_action(Workspace::toggle_sidebar_item_focus);
    cx.add_bindings(vec![
        Binding::new("cmd-s", Save, None),
        Binding::new("cmd-alt-i", DebugElements, None),
        Binding::new(
            "cmd-shift-!",
            ToggleSidebarItem(SidebarItemId {
                side: Side::Left,
                item_index: 0,
            }),
            None,
        ),
        Binding::new(
            "cmd-1",
            ToggleSidebarItemFocus(SidebarItemId {
                side: Side::Left,
                item_index: 0,
            }),
            None,
        ),
    ]);
    pane::init(cx);
}

pub struct AppState {
    pub settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    pub settings: watch::Receiver<Settings>,
    pub languages: Arc<LanguageRegistry>,
    pub themes: Arc<ThemeRegistry>,
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub channel_list: ModelHandle<client::ChannelList>,
    pub entry_openers: Arc<[Box<dyn EntryOpener>]>,
    pub build_window_options: &'static dyn Fn() -> WindowOptions<'static>,
    pub build_workspace: &'static dyn Fn(
        ModelHandle<Project>,
        &Arc<AppState>,
        &mut ViewContext<Workspace>,
    ) -> Workspace,
}

#[derive(Clone)]
pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub app_state: Arc<AppState>,
}

#[derive(Clone)]
pub struct JoinProjectParams {
    pub project_id: u64,
    pub app_state: Arc<AppState>,
}

pub trait EntryOpener {
    fn open(
        &self,
        worktree: &mut Worktree,
        path: ProjectPath,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<Box<dyn ItemHandle>>>>;
}

pub trait Item: Entity + Sized {
    type View: ItemView;

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View;

    fn project_path(&self) -> Option<ProjectPath>;
}

pub trait ItemView: View {
    fn title(&self, cx: &AppContext) -> String;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
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
    fn can_save(&self, cx: &AppContext) -> bool;
    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>>;
    fn can_save_as(&self, cx: &AppContext) -> bool;
    fn save_as(
        &mut self,
        worktree: ModelHandle<Worktree>,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>>;
    fn should_activate_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_close_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
}

pub trait ItemHandle: Send + Sync {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle>;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn downgrade(&self) -> Box<dyn WeakItemHandle>;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
}

pub trait WeakItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>>;
}

pub trait ItemViewHandle {
    fn title(&self, cx: &AppContext) -> String;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn boxed_clone(&self) -> Box<dyn ItemViewHandle>;
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>>;
    fn set_parent_pane(&self, pane: &ViewHandle<Pane>, cx: &mut MutableAppContext);
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
    fn can_save_as(&self, cx: &AppContext) -> bool;
    fn save(&self, cx: &mut MutableAppContext) -> Result<Task<Result<()>>>;
    fn save_as(
        &self,
        worktree: ModelHandle<Worktree>,
        path: &Path,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>>;
}

impl<T: Item> ItemHandle for ModelHandle<T> {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        Box::new(cx.add_view(window_id, |cx| T::build_view(self.clone(), settings, cx)))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.downgrade())
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path()
    }
}

impl ItemHandle for Box<dyn ItemHandle> {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        ItemHandle::add_view(self.as_ref(), window_id, settings, cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        self.as_ref().boxed_clone()
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        self.as_ref().downgrade()
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.as_ref().project_path(cx)
    }
}

impl<T: Item> WeakItemHandle for WeakModelHandle<T> {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        WeakModelHandle::<T>::upgrade(*self, cx).map(|i| Box::new(i) as Box<dyn ItemHandle>)
    }
}

impl<T: ItemView> ItemViewHandle for ViewHandle<T> {
    fn title(&self, cx: &AppContext) -> String {
        self.read(cx).title(cx)
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path(cx)
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
            cx.subscribe(self, |pane, item, event, cx| {
                if T::should_close_item_on_event(event) {
                    pane.close_item(item.id(), cx);
                    return;
                }
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
            .detach();
        });
    }

    fn save(&self, cx: &mut MutableAppContext) -> Result<Task<Result<()>>> {
        self.update(cx, |item, cx| item.save(cx))
    }

    fn save_as(
        &self,
        worktree: ModelHandle<Worktree>,
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

    fn can_save(&self, cx: &AppContext) -> bool {
        self.read(cx).can_save(cx)
    }

    fn can_save_as(&self, cx: &AppContext) -> bool {
        self.read(cx).can_save_as(cx)
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

#[derive(Clone)]
pub struct WorkspaceParams {
    pub project: ModelHandle<Project>,
    pub client: Arc<Client>,
    pub fs: Arc<dyn Fs>,
    pub languages: Arc<LanguageRegistry>,
    pub settings: watch::Receiver<Settings>,
    pub user_store: ModelHandle<UserStore>,
    pub channel_list: ModelHandle<ChannelList>,
    pub entry_openers: Arc<[Box<dyn EntryOpener>]>,
}

impl WorkspaceParams {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut MutableAppContext) -> Self {
        let fs = Arc::new(project::FakeFs::new());
        let languages = Arc::new(LanguageRegistry::new());
        let http_client = client::test::FakeHttpClient::new(|_| async move {
            Ok(client::http::ServerResponse::new(404))
        });
        let client = Client::new(http_client.clone());
        let theme =
            gpui::fonts::with_font_cache(cx.font_cache().clone(), || theme::Theme::default());
        let settings = Settings::new("Courier", cx.font_cache(), Arc::new(theme)).unwrap();
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project = Project::local(
            client.clone(),
            user_store.clone(),
            languages.clone(),
            fs.clone(),
            cx,
        );
        Self {
            project,
            channel_list: cx
                .add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
            client,
            fs,
            languages,
            settings: watch::channel_with(settings).1,
            user_store,
            entry_openers: Arc::from([]),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn local(app_state: &Arc<AppState>, cx: &mut MutableAppContext) -> Self {
        Self {
            project: Project::local(
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx,
            ),
            client: app_state.client.clone(),
            fs: app_state.fs.clone(),
            languages: app_state.languages.clone(),
            settings: app_state.settings.clone(),
            user_store: app_state.user_store.clone(),
            channel_list: app_state.channel_list.clone(),
            entry_openers: app_state.entry_openers.clone(),
        }
    }
}

pub struct Workspace {
    pub settings: watch::Receiver<Settings>,
    client: Arc<Client>,
    user_store: ModelHandle<client::UserStore>,
    fs: Arc<dyn Fs>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    left_sidebar: Sidebar,
    right_sidebar: Sidebar,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    status_bar: ViewHandle<StatusBar>,
    project: ModelHandle<Project>,
    entry_openers: Arc<[Box<dyn EntryOpener>]>,
    items: Vec<Box<dyn WeakItemHandle>>,
    _observe_current_user: Task<()>,
}

impl Workspace {
    pub fn new(params: &WorkspaceParams, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&params.project, |_, _, cx| cx.notify()).detach();

        let pane = cx.add_view(|_| Pane::new(params.settings.clone()));
        let pane_id = pane.id();
        cx.observe(&pane, move |me, _, cx| {
            let active_entry = me.active_project_path(cx);
            me.project
                .update(cx, |project, cx| project.set_active_path(active_entry, cx));
        })
        .detach();
        cx.subscribe(&pane, move |me, _, event, cx| {
            me.handle_pane_event(pane_id, event, cx)
        })
        .detach();
        cx.focus(&pane);

        let status_bar = cx.add_view(|cx| StatusBar::new(&pane, params.settings.clone(), cx));
        let mut current_user = params.user_store.read(cx).watch_current_user().clone();
        let mut connection_status = params.client.status().clone();
        let _observe_current_user = cx.spawn_weak(|this, mut cx| async move {
            current_user.recv().await;
            connection_status.recv().await;
            let mut stream =
                Stream::map(current_user, drop).merge(Stream::map(connection_status, drop));

            while stream.recv().await.is_some() {
                cx.update(|cx| {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(cx, |_, cx| cx.notify());
                    }
                })
            }
        });

        Workspace {
            modal: None,
            center: PaneGroup::new(pane.id()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            status_bar,
            settings: params.settings.clone(),
            client: params.client.clone(),
            user_store: params.user_store.clone(),
            fs: params.fs.clone(),
            left_sidebar: Sidebar::new(Side::Left),
            right_sidebar: Sidebar::new(Side::Right),
            project: params.project.clone(),
            entry_openers: params.entry_openers.clone(),
            items: Default::default(),
            _observe_current_user,
        }
    }

    pub fn left_sidebar_mut(&mut self) -> &mut Sidebar {
        &mut self.left_sidebar
    }

    pub fn right_sidebar_mut(&mut self) -> &mut Sidebar {
        &mut self.right_sidebar
    }

    pub fn status_bar(&self) -> &ViewHandle<StatusBar> {
        &self.status_bar
    }

    pub fn project(&self) -> &ModelHandle<Project> {
        &self.project
    }

    pub fn worktrees<'a>(&self, cx: &'a AppContext) -> &'a [ModelHandle<Worktree>] {
        &self.project.read(cx).worktrees()
    }

    pub fn contains_paths(&self, paths: &[PathBuf], cx: &AppContext) -> bool {
        paths.iter().all(|path| self.contains_path(&path, cx))
    }

    pub fn contains_path(&self, path: &Path, cx: &AppContext) -> bool {
        for worktree in self.worktrees(cx) {
            let worktree = worktree.read(cx).as_local();
            if worktree.map_or(false, |w| w.contains_abs_path(path)) {
                return true;
            }
        }
        false
    }

    pub fn worktree_scans_complete(&self, cx: &AppContext) -> impl Future<Output = ()> + 'static {
        let futures = self
            .worktrees(cx)
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

    pub fn open_paths(
        &mut self,
        abs_paths: &[PathBuf],
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemViewHandle>, Arc<anyhow::Error>>>>> {
        let entries = abs_paths
            .iter()
            .cloned()
            .map(|path| self.project_path_for_path(&path, cx))
            .collect::<Vec<_>>();

        let fs = self.fs.clone();
        let tasks = abs_paths
            .iter()
            .cloned()
            .zip(entries.into_iter())
            .map(|(abs_path, project_path)| {
                cx.spawn(|this, mut cx| {
                    let fs = fs.clone();
                    async move {
                        let project_path = project_path.await.ok()?;
                        if fs.is_file(&abs_path).await {
                            if let Some(entry) =
                                this.update(&mut cx, |this, cx| this.open_path(project_path, cx))
                            {
                                return Some(entry.await);
                            }
                        }
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        cx.foreground().spawn(async move {
            let mut items = Vec::new();
            for task in tasks {
                items.push(task.await);
            }
            items
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
                for tree in this.worktrees(cx) {
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

    fn project_path_for_path(
        &self,
        abs_path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<ProjectPath>> {
        let entry = self.worktree_for_abs_path(abs_path, cx);
        cx.spawn(|_, cx| async move {
            let (worktree, path) = entry.await?;
            Ok(ProjectPath {
                worktree_id: worktree.read_with(&cx, |t, _| t.id()),
                path: path.into(),
            })
        })
    }

    pub fn add_worktree(
        &self,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        self.project
            .update(cx, |project, cx| project.add_local_worktree(path, cx))
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

    #[must_use]
    pub fn open_path(
        &mut self,
        project_path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<Box<dyn ItemViewHandle>, Arc<anyhow::Error>>>> {
        let pane = self.active_pane().clone();
        if let Some(existing_item) =
            self.activate_or_open_existing_entry(project_path.clone(), &pane, cx)
        {
            return Some(cx.foreground().spawn(async move { Ok(existing_item) }));
        }

        let worktree = match self
            .project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        {
            Some(worktree) => worktree,
            None => {
                log::error!("worktree {} does not exist", project_path.worktree_id);
                return None;
            }
        };

        let project_path = project_path.clone();
        let entry_openers = self.entry_openers.clone();
        let task = worktree.update(cx, |worktree, cx| {
            for opener in entry_openers.iter() {
                if let Some(task) = opener.open(worktree, project_path.clone(), cx) {
                    return Some(task);
                }
            }
            log::error!("no opener for path {:?} found", project_path);
            None
        })?;

        let pane = pane.downgrade();
        Some(cx.spawn(|this, mut cx| async move {
            let load_result = task.await;
            this.update(&mut cx, |this, cx| {
                let pane = pane
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("could not upgrade pane reference"))?;
                let item = load_result?;

                // By the time loading finishes, the entry could have been already added
                // to the pane. If it was, we activate it, otherwise we'll store the
                // item and add a new view for it.
                if let Some(existing) =
                    this.activate_or_open_existing_entry(project_path, &pane, cx)
                {
                    Ok(existing)
                } else {
                    Ok(this.add_item(item, cx))
                }
            })
        }))
    }

    fn activate_or_open_existing_entry(
        &mut self,
        project_path: ProjectPath,
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> Option<Box<dyn ItemViewHandle>> {
        // If the pane contains a view for this file, then activate
        // that item view.
        if let Some(existing_item_view) =
            pane.update(cx, |pane, cx| pane.activate_entry(project_path.clone(), cx))
        {
            return Some(existing_item_view);
        }

        // Otherwise, if this file is already open somewhere in the workspace,
        // then add another view for it.
        let settings = self.settings.clone();
        let mut view_for_existing_item = None;
        self.items.retain(|item| {
            if let Some(item) = item.upgrade(cx) {
                if view_for_existing_item.is_none()
                    && item
                        .project_path(cx)
                        .map_or(false, |item_project_path| item_project_path == project_path)
                {
                    view_for_existing_item =
                        Some(item.add_view(cx.window_id(), settings.clone(), cx.as_mut()));
                }
                true
            } else {
                false
            }
        });
        if let Some(view) = view_for_existing_item {
            pane.add_item_view(view.boxed_clone(), cx.as_mut());
            Some(view)
        } else {
            None
        }
    }

    pub fn active_item(&self, cx: &AppContext) -> Option<Box<dyn ItemViewHandle>> {
        self.active_pane().read(cx).active_item()
    }

    fn active_project_path(&self, cx: &ViewContext<Self>) -> Option<ProjectPath> {
        self.active_item(cx).and_then(|item| item.project_path(cx))
    }

    pub fn save_active_item(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        if let Some(item) = self.active_item(cx) {
            let handle = cx.handle();
            if item.can_save(cx) {
                if item.has_conflict(cx.as_ref()) {
                    const CONFLICT_MESSAGE: &'static str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";

                    cx.prompt(
                        PromptLevel::Warning,
                        CONFLICT_MESSAGE,
                        &["Overwrite", "Cancel"],
                        move |answer, cx| {
                            if answer == 0 {
                                cx.spawn(|mut cx| async move {
                                    if let Err(error) = cx.update(|cx| item.save(cx)).unwrap().await
                                    {
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
            } else if item.can_save_as(cx) {
                let worktree = self.worktrees(cx).first();
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
                                            item.save_as(worktree, &path, cx.as_mut())
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
            }
        }
    }

    pub fn toggle_sidebar_item(&mut self, action: &ToggleSidebarItem, cx: &mut ViewContext<Self>) {
        let sidebar = match action.0.side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        sidebar.toggle_item(action.0.item_index);
        if let Some(active_item) = sidebar.active_item() {
            cx.focus(active_item);
        } else {
            cx.focus_self();
        }
        cx.notify();
    }

    pub fn toggle_sidebar_item_focus(
        &mut self,
        action: &ToggleSidebarItemFocus,
        cx: &mut ViewContext<Self>,
    ) {
        let sidebar = match action.0.side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        sidebar.activate_item(action.0.item_index);
        if let Some(active_item) = sidebar.active_item() {
            if active_item.is_focused(cx) {
                cx.focus_self();
            } else {
                cx.focus(active_item);
            }
        }
        cx.notify();
    }

    pub fn debug_elements(&mut self, _: &DebugElements, cx: &mut ViewContext<Self>) {
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

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = cx.add_view(|_| Pane::new(self.settings.clone()));
        let pane_id = pane.id();
        cx.observe(&pane, move |me, _, cx| {
            let active_entry = me.active_project_path(cx);
            me.project
                .update(cx, |project, cx| project.set_active_path(active_entry, cx));
        })
        .detach();
        cx.subscribe(&pane, move |me, _, event, cx| {
            me.handle_pane_event(pane_id, event, cx)
        })
        .detach();
        self.panes.push(pane.clone());
        self.activate_pane(pane.clone(), cx);
        pane
    }

    pub fn add_item<T>(
        &mut self,
        item_handle: T,
        cx: &mut ViewContext<Self>,
    ) -> Box<dyn ItemViewHandle>
    where
        T: ItemHandle,
    {
        let view = item_handle.add_view(cx.window_id(), self.settings.clone(), cx);
        self.items.push(item_handle.downgrade());
        self.active_pane()
            .add_item_view(view.boxed_clone(), cx.as_mut());
        view
    }

    fn activate_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        self.active_pane = pane;
        self.status_bar.update(cx, |status_bar, cx| {
            status_bar.set_active_pane(&self.active_pane, cx);
        });
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

    pub fn split_pane(
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

    pub fn panes(&self) -> &[ViewHandle<Pane>] {
        &self.panes
    }

    fn pane(&self, pane_id: usize) -> Option<ViewHandle<Pane>> {
        self.panes.iter().find(|pane| pane.id() == pane_id).cloned()
    }

    pub fn active_pane(&self) -> &ViewHandle<Pane> {
        &self.active_pane
    }

    fn toggle_share(&mut self, _: &ToggleShare, cx: &mut ViewContext<Self>) {
        self.project.update(cx, |project, cx| {
            if project.is_local() {
                if project.is_shared() {
                    project.unshare(cx).detach();
                } else {
                    project.share(cx).detach();
                }
            }
        });
    }

    fn render_connection_status(&self) -> Option<ElementBox> {
        let theme = &self.settings.borrow().theme;
        match &*self.client.status().borrow() {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                Container::new(
                    Align::new(
                        ConstrainedBox::new(
                            Svg::new("icons/offline-14.svg")
                                .with_color(theme.workspace.titlebar.icon_color)
                                .boxed(),
                        )
                        .with_width(theme.workspace.titlebar.offline_icon.width)
                        .boxed(),
                    )
                    .boxed(),
                )
                .with_style(theme.workspace.titlebar.offline_icon.container)
                .boxed(),
            ),
            client::Status::UpgradeRequired => Some(
                Label::new(
                    "Please update Zed to collaborate".to_string(),
                    theme.workspace.titlebar.outdated_warning.text.clone(),
                )
                .contained()
                .with_style(theme.workspace.titlebar.outdated_warning.container)
                .aligned()
                .boxed(),
            ),
            _ => None,
        }
    }

    fn render_titlebar(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        ConstrainedBox::new(
            Container::new(
                Stack::new()
                    .with_child(
                        Align::new(
                            Label::new("zed".into(), theme.workspace.titlebar.title.clone())
                                .boxed(),
                        )
                        .boxed(),
                    )
                    .with_child(
                        Align::new(
                            Flex::row()
                                .with_children(self.render_share_icon(cx))
                                .with_children(self.render_collaborators(theme, cx))
                                .with_child(self.render_avatar(
                                    self.user_store.read(cx).current_user().as_ref(),
                                    self.project.read(cx).replica_id(),
                                    theme,
                                    cx,
                                ))
                                .with_children(self.render_connection_status())
                                .boxed(),
                        )
                        .right()
                        .boxed(),
                    )
                    .boxed(),
            )
            .with_style(theme.workspace.titlebar.container)
            .boxed(),
        )
        .with_height(theme.workspace.titlebar.height)
        .named("titlebar")
    }

    fn render_collaborators(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> Vec<ElementBox> {
        let mut collaborators = self
            .project
            .read(cx)
            .collaborators()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        collaborators.sort_unstable_by_key(|collaborator| collaborator.replica_id);
        collaborators
            .into_iter()
            .map(|collaborator| {
                self.render_avatar(Some(&collaborator.user), collaborator.replica_id, theme, cx)
            })
            .collect()
    }

    fn render_avatar(
        &self,
        user: Option<&Arc<User>>,
        replica_id: ReplicaId,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        if let Some(avatar) = user.and_then(|user| user.avatar.clone()) {
            ConstrainedBox::new(
                Stack::new()
                    .with_child(
                        ConstrainedBox::new(
                            Image::new(avatar)
                                .with_style(theme.workspace.titlebar.avatar)
                                .boxed(),
                        )
                        .with_width(theme.workspace.titlebar.avatar_width)
                        .aligned()
                        .boxed(),
                    )
                    .with_child(
                        AvatarRibbon::new(theme.editor.replica_selection_style(replica_id).cursor)
                            .constrained()
                            .with_width(theme.workspace.titlebar.avatar_ribbon.width)
                            .with_height(theme.workspace.titlebar.avatar_ribbon.height)
                            .aligned()
                            .bottom()
                            .boxed(),
                    )
                    .boxed(),
            )
            .with_width(theme.workspace.right_sidebar.width)
            .boxed()
        } else {
            MouseEventHandler::new::<Authenticate, _, _, _>(0, cx, |state, _| {
                let style = if state.hovered {
                    &theme.workspace.titlebar.hovered_sign_in_prompt
                } else {
                    &theme.workspace.titlebar.sign_in_prompt
                };
                Label::new("Sign in".to_string(), style.text.clone())
                    .contained()
                    .with_style(style.container)
                    .boxed()
            })
            .on_click(|cx| cx.dispatch_action(Authenticate))
            .with_cursor_style(CursorStyle::PointingHand)
            .aligned()
            .boxed()
        }
    }

    fn render_share_icon(&self, cx: &mut RenderContext<Self>) -> Option<ElementBox> {
        if self.project().read(cx).is_local() && self.client.user_id().is_some() {
            enum Share {}

            let color = if self.project().read(cx).is_shared() {
                Color::green()
            } else {
                Color::red()
            };
            Some(
                MouseEventHandler::new::<Share, _, _, _>(0, cx, |_, _| {
                    Align::new(
                        ConstrainedBox::new(
                            Svg::new("icons/broadcast-24.svg").with_color(color).boxed(),
                        )
                        .with_width(24.)
                        .boxed(),
                    )
                    .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(|cx| cx.dispatch_action(ToggleShare))
                .boxed(),
            )
        } else {
            None
        }
    }
}

impl Entity for Workspace {
    type Event = ();
}

impl View for Workspace {
    fn ui_name() -> &'static str {
        "Workspace"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme;
        Flex::column()
            .with_child(self.render_titlebar(&theme, cx))
            .with_child(
                Stack::new()
                    .with_child({
                        let mut content = Flex::row();
                        content.add_child(self.left_sidebar.render(&settings, cx));
                        if let Some(element) = self.left_sidebar.render_active_item(&settings, cx) {
                            content.add_child(Flexible::new(0.8, false, element).boxed());
                        }
                        content.add_child(
                            Flex::column()
                                .with_child(
                                    Flexible::new(1., true, self.center.render(&settings.theme))
                                        .boxed(),
                                )
                                .with_child(ChildView::new(self.status_bar.id()).boxed())
                                .flexible(1., true)
                                .boxed(),
                        );
                        if let Some(element) = self.right_sidebar.render_active_item(&settings, cx)
                        {
                            content.add_child(Flexible::new(0.8, false, element).boxed());
                        }
                        content.add_child(self.right_sidebar.render(&settings, cx));
                        content.boxed()
                    })
                    .with_children(self.modal.as_ref().map(|m| ChildView::new(m.id()).boxed()))
                    .flexible(1.0, true)
                    .boxed(),
            )
            .contained()
            .with_background_color(settings.theme.workspace.background)
            .named("workspace")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.active_pane);
    }
}

pub trait WorkspaceHandle {
    fn file_project_paths(&self, cx: &AppContext) -> Vec<ProjectPath>;
}

impl WorkspaceHandle for ViewHandle<Workspace> {
    fn file_project_paths(&self, cx: &AppContext) -> Vec<ProjectPath> {
        self.read(cx)
            .worktrees(cx)
            .iter()
            .flat_map(|worktree| {
                let worktree_id = worktree.read(cx).id();
                worktree.read(cx).files(true, 0).map(move |f| ProjectPath {
                    worktree_id,
                    path: f.path.clone(),
                })
            })
            .collect::<Vec<_>>()
    }
}

pub struct AvatarRibbon {
    color: Color,
}

impl AvatarRibbon {
    pub fn new(color: Color) -> AvatarRibbon {
        AvatarRibbon { color }
    }
}

impl Element for AvatarRibbon {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        _: gpui::geometry::rect::RectF,
        _: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let mut path = PathBuilder::new();
        path.reset(bounds.lower_left());
        path.curve_to(
            bounds.origin() + vec2f(bounds.height(), 0.),
            bounds.origin(),
        );
        path.line_to(bounds.upper_right() - vec2f(bounds.height(), 0.));
        path.curve_to(bounds.lower_right(), bounds.upper_right());
        path.line_to(bounds.lower_left());
        cx.scene.push_path(path.build(self.color, None));
    }

    fn dispatch_event(
        &mut self,
        _: &gpui::Event,
        _: gpui::geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut gpui::EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::DebugContext,
    ) -> gpui::json::Value {
        json::json!({
            "type": "AvatarRibbon",
            "bounds": bounds.to_json(),
            "color": self.color.to_json(),
        })
    }
}

impl std::fmt::Debug for OpenParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenParams")
            .field("paths", &self.paths)
            .finish()
    }
}

fn open(action: &Open, cx: &mut MutableAppContext) {
    let app_state = action.0.clone();
    cx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, cx| {
            if let Some(paths) = paths {
                cx.dispatch_global_action(OpenPaths(OpenParams { paths, app_state }));
            }
        },
    );
}

pub fn open_paths(
    abs_paths: &[PathBuf],
    app_state: &Arc<AppState>,
    cx: &mut MutableAppContext,
) -> Task<ViewHandle<Workspace>> {
    log::info!("open paths {:?}", abs_paths);

    // Open paths in existing workspace if possible
    let mut existing = None;
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(workspace) = cx.root_view::<Workspace>(window_id) {
            if workspace.update(cx, |view, cx| {
                if view.contains_paths(abs_paths, cx.as_ref()) {
                    existing = Some(workspace.clone());
                    true
                } else {
                    false
                }
            }) {
                break;
            }
        }
    }

    let workspace = existing.unwrap_or_else(|| {
        cx.add_window((app_state.build_window_options)(), |cx| {
            let project = Project::local(
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx,
            );
            (app_state.build_workspace)(project, &app_state, cx)
        })
        .1
    });

    let task = workspace.update(cx, |workspace, cx| workspace.open_paths(abs_paths, cx));
    cx.spawn(|_| async move {
        task.await;
        workspace
    })
}

pub fn join_project(
    project_id: u64,
    app_state: &Arc<AppState>,
    cx: &mut MutableAppContext,
) -> Task<Result<ViewHandle<Workspace>>> {
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(workspace) = cx.root_view::<Workspace>(window_id) {
            if workspace.read(cx).project().read(cx).remote_id() == Some(project_id) {
                return Task::ready(Ok(workspace));
            }
        }
    }

    let app_state = app_state.clone();
    cx.spawn(|mut cx| async move {
        let project = Project::remote(
            project_id,
            app_state.client.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            &mut cx,
        )
        .await?;
        let (_, workspace) = cx.update(|cx| {
            cx.add_window((app_state.build_window_options)(), |cx| {
                (app_state.build_workspace)(project, &app_state, cx)
            })
        });
        Ok(workspace)
    })
}

fn open_new(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    let (window_id, workspace) = cx.add_window((app_state.build_window_options)(), |cx| {
        let project = Project::local(
            app_state.client.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            cx,
        );
        (app_state.build_workspace)(project, &app_state, cx)
    });
    cx.dispatch_action(window_id, vec![workspace.id()], &OpenNew(app_state.clone()));
}
