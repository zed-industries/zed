pub mod menu;
pub mod pane;
pub mod pane_group;
pub mod settings;
pub mod sidebar;
mod status_bar;

use anyhow::{anyhow, Result};
use client::{Authenticate, ChannelList, Client, User, UserStore};
use clock::ReplicaId;
use collections::HashSet;
use gpui::{
    action,
    color::Color,
    elements::*,
    geometry::{vector::vec2f, PathBuilder},
    json::{self, to_string_pretty, ToJson},
    keymap::Binding,
    platform::{CursorStyle, WindowOptions},
    AnyModelHandle, AnyViewHandle, AppContext, ClipboardItem, Entity, ImageData, ModelContext,
    ModelHandle, MutableAppContext, PathPromptOptions, PromptLevel, RenderContext, Task, View,
    ViewContext, ViewHandle, WeakModelHandle, WeakViewHandle,
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
    any::{Any, TypeId},
    cell::RefCell,
    future::Future,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    rc::Rc,
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
    pane::init(cx);
    menu::init(cx);

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
    cx.add_action(
        |workspace: &mut Workspace, _: &Save, cx: &mut ViewContext<Workspace>| {
            workspace.save_active_item(cx).detach_and_log_err(cx);
        },
    );
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
    pub path_openers: Arc<[Box<dyn PathOpener>]>,
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

pub trait PathOpener {
    fn open(
        &self,
        project: &mut Project,
        path: ProjectPath,
        cx: &mut ModelContext<Project>,
    ) -> Option<Task<Result<Box<dyn ItemHandle>>>>;
}

pub trait Item: Entity + Sized {
    type View: ItemView;

    fn build_view(
        handle: ModelHandle<Self>,
        workspace: &Workspace,
        nav_history: ItemNavHistory,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View;

    fn project_path(&self) -> Option<ProjectPath>;
}

pub trait ItemView: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) {}
    fn item_id(&self, cx: &AppContext) -> usize;
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox;
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
    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn can_save_as(&self, cx: &AppContext) -> bool;
    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn should_activate_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_close_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
    fn act_as_type(
        &self,
        type_id: TypeId,
        self_handle: &ViewHandle<Self>,
        _: &AppContext,
    ) -> Option<AnyViewHandle> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle.into())
        } else {
            None
        }
    }
}

pub trait ItemHandle: Send + Sync {
    fn id(&self) -> usize;
    fn add_view(
        &self,
        window_id: usize,
        workspace: &Workspace,
        nav_history: Rc<RefCell<NavHistory>>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle>;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn downgrade(&self) -> Box<dyn WeakItemHandle>;
    fn to_any(&self) -> AnyModelHandle;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
}

pub trait WeakItemHandle {
    fn id(&self) -> usize;
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>>;
}

pub trait ItemViewHandle: 'static {
    fn item_id(&self, cx: &AppContext) -> usize;
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn boxed_clone(&self) -> Box<dyn ItemViewHandle>;
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemViewHandle>>;
    fn added_to_pane(&mut self, cx: &mut ViewContext<Pane>);
    fn deactivated(&self, cx: &mut MutableAppContext);
    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext);
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
    fn can_save_as(&self, cx: &AppContext) -> bool;
    fn save(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyViewHandle>;
}

pub trait WeakItemViewHandle {
    fn id(&self) -> usize;
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemViewHandle>>;
}

impl<T: Item> ItemHandle for ModelHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn add_view(
        &self,
        window_id: usize,
        workspace: &Workspace,
        nav_history: Rc<RefCell<NavHistory>>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        Box::new(cx.add_view(window_id, |cx| {
            let nav_history = ItemNavHistory::new(nav_history, &cx.handle());
            T::build_view(self.clone(), workspace, nav_history, cx)
        }))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.downgrade())
    }

    fn to_any(&self) -> AnyModelHandle {
        self.clone().into()
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path()
    }
}

impl ItemHandle for Box<dyn ItemHandle> {
    fn id(&self) -> usize {
        ItemHandle::id(self.as_ref())
    }

    fn add_view(
        &self,
        window_id: usize,
        workspace: &Workspace,
        nav_history: Rc<RefCell<NavHistory>>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        ItemHandle::add_view(self.as_ref(), window_id, workspace, nav_history, cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        self.as_ref().boxed_clone()
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        self.as_ref().downgrade()
    }

    fn to_any(&self) -> AnyModelHandle {
        self.as_ref().to_any()
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.as_ref().project_path(cx)
    }
}

impl<T: Item> WeakItemHandle for WeakModelHandle<T> {
    fn id(&self) -> usize {
        WeakModelHandle::id(self)
    }

    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        WeakModelHandle::<T>::upgrade(self, cx).map(|i| Box::new(i) as Box<dyn ItemHandle>)
    }
}

impl Hash for Box<dyn WeakItemHandle> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for Box<dyn WeakItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Box<dyn WeakItemHandle> {}

impl dyn ItemViewHandle {
    pub fn downcast<T: View>(&self) -> Option<ViewHandle<T>> {
        self.to_any().downcast()
    }

    pub fn act_as<T: View>(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        self.act_as_type(TypeId::of::<T>(), cx)
            .and_then(|t| t.downcast())
    }
}

impl<T: ItemView> ItemViewHandle for ViewHandle<T> {
    fn item_id(&self, cx: &AppContext) -> usize {
        self.read(cx).item_id(cx)
    }

    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox {
        self.read(cx).tab_content(style, cx)
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

    fn added_to_pane(&mut self, cx: &mut ViewContext<Pane>) {
        cx.subscribe(self, |pane, item, event, cx| {
            if T::should_close_item_on_event(event) {
                pane.close_item(item.id(), cx);
                return;
            }
            if T::should_activate_item_on_event(event) {
                if let Some(ix) = pane.index_for_item_view(&item) {
                    pane.activate_item(ix, cx);
                    pane.activate(cx);
                }
            }
            if T::should_update_tab_on_event(event) {
                cx.notify()
            }
        })
        .detach();
    }

    fn deactivated(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.deactivated(cx));
    }

    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.navigate(data, cx));
    }

    fn save(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.save(project, cx))
    }

    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(project, abs_path, cx))
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

    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyViewHandle> {
        self.read(cx).act_as_type(type_id, self, cx)
    }
}

impl Into<AnyViewHandle> for Box<dyn ItemViewHandle> {
    fn into(self) -> AnyViewHandle {
        self.to_any()
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

impl<T: ItemView> WeakItemViewHandle for WeakViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemViewHandle>> {
        self.upgrade(cx)
            .map(|v| Box::new(v) as Box<dyn ItemViewHandle>)
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
    pub path_openers: Arc<[Box<dyn PathOpener>]>,
}

impl WorkspaceParams {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut MutableAppContext) -> Self {
        let fs = project::FakeFs::new(cx.background().clone());
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
            path_openers: Arc::from([]),
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
            path_openers: app_state.path_openers.clone(),
        }
    }
}

pub struct Workspace {
    pub settings: watch::Receiver<Settings>,
    weak_self: WeakViewHandle<Self>,
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
    path_openers: Arc<[Box<dyn PathOpener>]>,
    items: HashSet<Box<dyn WeakItemHandle>>,
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
                    if let Some(this) = this.upgrade(cx) {
                        this.update(cx, |_, cx| cx.notify());
                    }
                })
            }
        });

        Workspace {
            modal: None,
            weak_self: cx.weak_handle(),
            center: PaneGroup::new(pane.clone()),
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
            path_openers: params.path_openers.clone(),
            items: Default::default(),
            _observe_current_user,
        }
    }

    pub fn weak_handle(&self) -> WeakViewHandle<Self> {
        self.weak_self.clone()
    }

    pub fn settings(&self) -> watch::Receiver<Settings> {
        self.settings.clone()
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

    pub fn worktrees<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
        self.project.read(cx).worktrees(cx)
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
                            Some(
                                this.update(&mut cx, |this, cx| this.open_path(project_path, cx))
                                    .await,
                            )
                        } else {
                            None
                        }
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

    fn project_path_for_path(
        &self,
        abs_path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<ProjectPath>> {
        let entry = self.project().update(cx, |project, cx| {
            project.find_or_create_local_worktree(abs_path, false, cx)
        });
        cx.spawn(|_, cx| async move {
            let (worktree, path) = entry.await?;
            Ok(ProjectPath {
                worktree_id: worktree.read_with(&cx, |t, _| t.id()),
                path: path.into(),
            })
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

    pub fn open_path(
        &mut self,
        path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemViewHandle>, Arc<anyhow::Error>>> {
        let load_task = self.load_path(path, cx);
        let pane = self.active_pane().clone().downgrade();
        cx.spawn(|this, mut cx| async move {
            let item = load_task.await?;
            this.update(&mut cx, |this, cx| {
                let pane = pane
                    .upgrade(cx)
                    .ok_or_else(|| anyhow!("could not upgrade pane reference"))?;
                Ok(this.open_item_in_pane(item, &pane, cx))
            })
        })
    }

    pub fn load_path(
        &mut self,
        path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>>> {
        if let Some(existing_item) = self.item_for_path(&path, cx) {
            return Task::ready(Ok(existing_item));
        }

        let project_path = path.clone();
        let path_openers = self.path_openers.clone();
        self.project.update(cx, |project, cx| {
            for opener in path_openers.iter() {
                if let Some(task) = opener.open(project, project_path.clone(), cx) {
                    return task;
                }
            }
            Task::ready(Err(anyhow!("no opener found for path {:?}", project_path)))
        })
    }

    fn item_for_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.items
            .iter()
            .filter_map(|i| i.upgrade(cx))
            .find(|i| i.project_path(cx).as_ref() == Some(path))
    }

    pub fn item_of_type<T: Item>(&self, cx: &AppContext) -> Option<ModelHandle<T>> {
        self.items
            .iter()
            .find_map(|i| i.upgrade(cx).and_then(|i| i.to_any().downcast()))
    }

    pub fn active_item(&self, cx: &AppContext) -> Option<Box<dyn ItemViewHandle>> {
        self.active_pane().read(cx).active_item()
    }

    fn active_project_path(&self, cx: &ViewContext<Self>) -> Option<ProjectPath> {
        self.active_item(cx).and_then(|item| item.project_path(cx))
    }

    pub fn save_active_item(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let project = self.project.clone();
        if let Some(item) = self.active_item(cx) {
            if item.can_save(cx) {
                if item.has_conflict(cx.as_ref()) {
                    const CONFLICT_MESSAGE: &'static str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";

                    let mut answer = cx.prompt(
                        PromptLevel::Warning,
                        CONFLICT_MESSAGE,
                        &["Overwrite", "Cancel"],
                    );
                    cx.spawn(|_, mut cx| async move {
                        let answer = answer.recv().await;
                        if answer == Some(0) {
                            cx.update(|cx| item.save(project, cx)).await?;
                        }
                        Ok(())
                    })
                } else {
                    item.save(project, cx)
                }
            } else if item.can_save_as(cx) {
                let worktree = self.worktrees(cx).next();
                let start_abs_path = worktree
                    .and_then(|w| w.read(cx).as_local())
                    .map_or(Path::new(""), |w| w.abs_path())
                    .to_path_buf();
                let mut abs_path = cx.prompt_for_new_path(&start_abs_path);
                cx.spawn(|_, mut cx| async move {
                    if let Some(abs_path) = abs_path.recv().await.flatten() {
                        cx.update(|cx| item.save_as(project, abs_path, cx)).await?;
                    }
                    Ok(())
                })
            } else {
                Task::ready(Ok(()))
            }
        } else {
            Task::ready(Ok(()))
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

    pub fn open_item<T>(
        &mut self,
        item_handle: T,
        cx: &mut ViewContext<Self>,
    ) -> Box<dyn ItemViewHandle>
    where
        T: 'static + ItemHandle,
    {
        self.open_item_in_pane(item_handle, &self.active_pane().clone(), cx)
    }

    pub fn open_item_in_pane<T>(
        &mut self,
        item_handle: T,
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> Box<dyn ItemViewHandle>
    where
        T: 'static + ItemHandle,
    {
        self.items.insert(item_handle.downgrade());
        pane.update(cx, |pane, cx| pane.open_item(item_handle, self, cx))
    }

    pub fn activate_pane_for_item(
        &mut self,
        item: &dyn ItemHandle,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let pane = self.panes.iter().find_map(|pane| {
            if pane.read(cx).contains_item(item) {
                Some(pane.clone())
            } else {
                None
            }
        });
        if let Some(pane) = pane {
            self.activate_pane(pane.clone(), cx);
            true
        } else {
            false
        }
    }

    pub fn activate_item(&mut self, item: &dyn ItemHandle, cx: &mut ViewContext<Self>) -> bool {
        let result = self.panes.iter().find_map(|pane| {
            if let Some(ix) = pane.read(cx).index_for_item(item) {
                Some((pane.clone(), ix))
            } else {
                None
            }
        });
        if let Some((pane, ix)) = result {
            self.activate_pane(pane.clone(), cx);
            pane.update(cx, |pane, cx| pane.activate_item(ix, cx));
            true
        } else {
            false
        }
    }

    pub fn activate_next_pane(&mut self, cx: &mut ViewContext<Self>) {
        let ix = self
            .panes
            .iter()
            .position(|pane| pane == &self.active_pane)
            .unwrap();
        let next_ix = (ix + 1) % self.panes.len();
        self.activate_pane(self.panes[next_ix].clone(), cx);
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
                new_pane.update(cx, |new_pane, cx| new_pane.add_item_view(clone, cx));
            }
        }
        self.center.split(&pane, &new_pane, direction).unwrap();
        cx.notify();
        new_pane
    }

    fn remove_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.center.remove(&pane).unwrap() {
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
                                .with_color(theme.workspace.titlebar.offline_icon.color)
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
                                .with_children(self.render_share_icon(theme, cx))
                                .with_children(self.render_collaborators(theme, cx))
                                .with_child(self.render_current_user(
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
            .filter_map(|collaborator| {
                Some(self.render_avatar(
                    collaborator.user.avatar.clone()?,
                    collaborator.replica_id,
                    theme,
                ))
            })
            .collect()
    }

    fn render_current_user(
        &self,
        user: Option<&Arc<User>>,
        replica_id: ReplicaId,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        if let Some(avatar) = user.and_then(|user| user.avatar.clone()) {
            self.render_avatar(avatar, replica_id, theme)
        } else {
            MouseEventHandler::new::<Authenticate, _, _>(0, cx, |state, _| {
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

    fn render_avatar(
        &self,
        avatar: Arc<ImageData>,
        replica_id: ReplicaId,
        theme: &Theme,
    ) -> ElementBox {
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
    }

    fn render_share_icon(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> Option<ElementBox> {
        if self.project().read(cx).is_local() && self.client.user_id().is_some() {
            enum Share {}

            let color = if self.project().read(cx).is_shared() {
                theme.workspace.titlebar.share_icon_active_color
            } else {
                theme.workspace.titlebar.share_icon_color
            };
            Some(
                MouseEventHandler::new::<Share, _, _>(0, cx, |_, _| {
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
                                .with_child(ChildView::new(&self.status_bar).boxed())
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
                    .with_children(self.modal.as_ref().map(|m| ChildView::new(m).boxed()))
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
    let mut paths = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: true,
        multiple: true,
    });
    cx.spawn(|mut cx| async move {
        if let Some(paths) = paths.recv().await.flatten() {
            cx.update(|cx| cx.dispatch_global_action(OpenPaths(OpenParams { paths, app_state })));
        }
    })
    .detach();
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
        if let Some(workspace_handle) = cx.root_view::<Workspace>(window_id) {
            if workspace_handle.update(cx, |workspace, cx| {
                if workspace.contains_paths(abs_paths, cx.as_ref()) {
                    cx.activate_window(window_id);
                    existing = Some(workspace_handle.clone());
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
