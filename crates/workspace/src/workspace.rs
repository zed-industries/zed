pub mod lsp_status;
pub mod menu;
pub mod pane;
pub mod pane_group;
pub mod settings;
pub mod sidebar;
mod status_bar;

use anyhow::{anyhow, Context, Result};
use client::{
    proto, Authenticate, ChannelList, Client, PeerId, Subscription, TypedEnvelope, User, UserStore,
};
use clock::ReplicaId;
use collections::{hash_map, HashMap, HashSet};
use gpui::{
    action,
    color::Color,
    elements::*,
    geometry::{vector::vec2f, PathBuilder},
    json::{self, to_string_pretty, ToJson},
    keymap::Binding,
    platform::{CursorStyle, WindowOptions},
    AnyModelHandle, AnyViewHandle, AppContext, AsyncAppContext, ClipboardItem, Entity, ImageData,
    ModelHandle, MutableAppContext, PathPromptOptions, PromptLevel, RenderContext, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use language::LanguageRegistry;
use log::error;
pub use pane::*;
pub use pane_group::*;
use postage::prelude::Stream;
use project::{fs, Fs, Project, ProjectEntryId, ProjectPath, Worktree};
pub use settings::Settings;
use sidebar::{Side, Sidebar, SidebarItemId, ToggleSidebarItem, ToggleSidebarItemFocus};
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt,
    future::Future,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use theme::{Theme, ThemeRegistry};
use util::ResultExt;

type ProjectItemBuilders = HashMap<
    TypeId,
    fn(usize, ModelHandle<Project>, AnyModelHandle, &mut MutableAppContext) -> Box<dyn ItemHandle>,
>;

type FollowableItemBuilder = fn(
    ViewHandle<Pane>,
    ModelHandle<Project>,
    &mut Option<proto::view::Variant>,
    &mut MutableAppContext,
) -> Option<Task<Result<Box<dyn FollowableItemHandle>>>>;
type FollowableItemBuilders = HashMap<
    TypeId,
    (
        FollowableItemBuilder,
        fn(AnyViewHandle) -> Box<dyn FollowableItemHandle>,
    ),
>;

action!(Open, Arc<AppState>);
action!(OpenNew, Arc<AppState>);
action!(OpenPaths, OpenParams);
action!(ToggleShare);
action!(ToggleFollow, PeerId);
action!(Unfollow);
action!(JoinProject, JoinProjectParams);
action!(Save);
action!(DebugElements);

pub fn init(client: &Arc<Client>, cx: &mut MutableAppContext) {
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
    cx.add_async_action(Workspace::toggle_follow);
    cx.add_action(
        |workspace: &mut Workspace, _: &Unfollow, cx: &mut ViewContext<Workspace>| {
            let pane = workspace.active_pane().clone();
            workspace.unfollow(&pane, cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &Save, cx: &mut ViewContext<Workspace>| {
            workspace.save_active_item(cx).detach_and_log_err(cx);
        },
    );
    cx.add_action(Workspace::debug_elements);
    cx.add_action(Workspace::toggle_sidebar_item);
    cx.add_action(Workspace::toggle_sidebar_item_focus);
    cx.add_bindings(vec![
        Binding::new("cmd-alt-shift-U", Unfollow, None),
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

    client.add_view_request_handler(Workspace::handle_follow);
    client.add_view_message_handler(Workspace::handle_unfollow);
    client.add_view_message_handler(Workspace::handle_update_followers);
}

pub fn register_project_item<I: ProjectItem>(cx: &mut MutableAppContext) {
    cx.update_default_global(|builders: &mut ProjectItemBuilders, _| {
        builders.insert(TypeId::of::<I::Item>(), |window_id, project, model, cx| {
            let item = model.downcast::<I::Item>().unwrap();
            Box::new(cx.add_view(window_id, |cx| I::for_project_item(project, item, cx)))
        });
    });
}

pub fn register_followable_item<I: FollowableItem>(cx: &mut MutableAppContext) {
    cx.update_default_global(|builders: &mut FollowableItemBuilders, _| {
        builders.insert(
            TypeId::of::<I>(),
            (
                |pane, project, state, cx| {
                    I::for_state_message(pane, project, state, cx).map(|task| {
                        cx.foreground()
                            .spawn(async move { Ok(Box::new(task.await?) as Box<_>) })
                    })
                },
                |this| Box::new(this.downcast::<I>().unwrap()),
            ),
        );
    });
}

pub struct AppState {
    pub languages: Arc<LanguageRegistry>,
    pub themes: Arc<ThemeRegistry>,
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub channel_list: ModelHandle<client::ChannelList>,
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

pub trait Item: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) {}
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
    fn set_nav_history(&mut self, _: ItemNavHistory, _: &mut ViewContext<Self>);
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

pub trait ProjectItem: Item {
    type Item: project::Item;

    fn for_project_item(
        project: ModelHandle<Project>,
        item: ModelHandle<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self;
}

pub trait FollowableItem: Item {
    fn for_state_message(
        pane: ViewHandle<Pane>,
        project: ModelHandle<Project>,
        state: &mut Option<proto::view::Variant>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>>;
    fn set_leader_replica_id(&mut self, leader_replica_id: Option<u16>, cx: &mut ViewContext<Self>);
    fn to_state_message(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn to_update_message(
        &self,
        event: &Self::Event,
        cx: &AppContext,
    ) -> Option<proto::update_view::Variant>;
    fn apply_update_message(
        &mut self,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Result<()>;
}

pub trait FollowableItemHandle: ItemHandle {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext);
    fn to_state_message(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn to_update_message(
        &self,
        event: &dyn Any,
        cx: &AppContext,
    ) -> Option<proto::update_view::Variant>;
    fn apply_update_message(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()>;
}

impl<T: FollowableItem> FollowableItemHandle for ViewHandle<T> {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| {
            this.set_leader_replica_id(leader_replica_id, cx)
        })
    }

    fn to_state_message(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        self.read(cx).to_state_message(cx)
    }

    fn to_update_message(
        &self,
        event: &dyn Any,
        cx: &AppContext,
    ) -> Option<proto::update_view::Variant> {
        self.read(cx).to_update_message(event.downcast_ref()?, cx)
    }

    fn apply_update_message(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()> {
        self.update(cx, |this, cx| this.apply_update_message(message, cx))
    }
}

pub trait ItemHandle: 'static + fmt::Debug {
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn set_nav_history(&self, nav_history: Rc<RefCell<NavHistory>>, cx: &mut MutableAppContext);
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    );
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
    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>>;
}

pub trait WeakItemHandle {
    fn id(&self) -> usize;
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>>;
}

impl dyn ItemHandle {
    pub fn downcast<T: View>(&self) -> Option<ViewHandle<T>> {
        self.to_any().downcast()
    }

    pub fn act_as<T: View>(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        self.act_as_type(TypeId::of::<T>(), cx)
            .and_then(|t| t.downcast())
    }
}

impl<T: Item> ItemHandle for ViewHandle<T> {
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox {
        self.read(cx).tab_content(style, cx)
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path(cx)
    }

    fn project_entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId> {
        self.read(cx).project_entry_id(cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>> {
        self.update(cx, |item, cx| {
            cx.add_option_view(|cx| item.clone_on_split(cx))
        })
        .map(|handle| Box::new(handle) as Box<dyn ItemHandle>)
    }

    fn set_nav_history(&self, nav_history: Rc<RefCell<NavHistory>>, cx: &mut MutableAppContext) {
        self.update(cx, |item, cx| {
            item.set_nav_history(ItemNavHistory::new(nav_history, &cx.handle()), cx);
        })
    }

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some(followed_item) = self.to_followable_item_handle(cx) {
            if let Some(message) = followed_item.to_state_message(cx) {
                workspace.update_followers(
                    proto::update_followers::Variant::CreateView(proto::View {
                        id: followed_item.id() as u64,
                        variant: Some(message),
                        leader_id: workspace.leader_for_pane(&pane).map(|id| id.0),
                    }),
                    cx,
                );
            }
        }

        let pane = pane.downgrade();
        cx.subscribe(self, move |workspace, item, event, cx| {
            let pane = if let Some(pane) = pane.upgrade(cx) {
                pane
            } else {
                log::error!("unexpected item event after pane was dropped");
                return;
            };

            if T::should_close_item_on_event(event) {
                pane.update(cx, |pane, cx| pane.close_item(item.id(), cx));
                return;
            }

            if T::should_activate_item_on_event(event) {
                pane.update(cx, |pane, cx| {
                    if let Some(ix) = pane.index_for_item(&item) {
                        pane.activate_item(ix, cx);
                        pane.activate(cx);
                    }
                });
            }

            if T::should_update_tab_on_event(event) {
                pane.update(cx, |_, cx| cx.notify());
            }

            if let Some(message) = item
                .to_followable_item_handle(cx)
                .and_then(|i| i.to_update_message(event, cx))
            {
                workspace.update_followers(
                    proto::update_followers::Variant::UpdateView(proto::UpdateView {
                        id: item.id() as u64,
                        variant: Some(message),
                        leader_id: workspace.leader_for_pane(&pane).map(|id| id.0),
                    }),
                    cx,
                );
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

    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>> {
        if cx.has_global::<FollowableItemBuilders>() {
            let builders = cx.global::<FollowableItemBuilders>();
            let item = self.to_any();
            Some(builders.get(&item.view_type())?.1(item))
        } else {
            None
        }
    }
}

impl Into<AnyViewHandle> for Box<dyn ItemHandle> {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}

impl<T: Item> WeakItemHandle for WeakViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.upgrade(cx).map(|v| Box::new(v) as Box<dyn ItemHandle>)
    }
}

#[derive(Clone)]
pub struct WorkspaceParams {
    pub project: ModelHandle<Project>,
    pub client: Arc<Client>,
    pub fs: Arc<dyn Fs>,
    pub languages: Arc<LanguageRegistry>,
    pub user_store: ModelHandle<UserStore>,
    pub channel_list: ModelHandle<ChannelList>,
}

impl WorkspaceParams {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut MutableAppContext) -> Self {
        let settings = Settings::test(cx);
        cx.set_global(settings);

        let fs = project::FakeFs::new(cx.background().clone());
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::new(|_| async move {
            Ok(client::http::ServerResponse::new(404))
        });
        let client = Client::new(http_client.clone());
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
            user_store,
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
            user_store: app_state.user_store.clone(),
            channel_list: app_state.channel_list.clone(),
        }
    }
}

pub struct Workspace {
    weak_self: WeakViewHandle<Self>,
    client: Arc<Client>,
    user_store: ModelHandle<client::UserStore>,
    remote_entity_subscription: Option<Subscription>,
    fs: Arc<dyn Fs>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    left_sidebar: Sidebar,
    right_sidebar: Sidebar,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    status_bar: ViewHandle<StatusBar>,
    project: ModelHandle<Project>,
    leader_state: LeaderState,
    follower_states_by_leader: HashMap<PeerId, HashMap<ViewHandle<Pane>, FollowerState>>,
    _observe_current_user: Task<()>,
}

#[derive(Default)]
struct LeaderState {
    followers: HashSet<PeerId>,
}

#[derive(Default)]
struct FollowerState {
    active_view_id: Option<u64>,
    items_by_leader_view_id: HashMap<u64, FollowerItem>,
}

#[derive(Debug)]
enum FollowerItem {
    Loading(Vec<proto::update_view::Variant>),
    Loaded(Box<dyn FollowableItemHandle>),
}

impl Workspace {
    pub fn new(params: &WorkspaceParams, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&params.project, |_, project, cx| {
            if project.read(cx).is_read_only() {
                cx.blur();
            }
            cx.notify()
        })
        .detach();

        cx.subscribe(&params.project, move |this, project, event, cx| {
            if let project::Event::RemoteIdChanged(remote_id) = event {
                this.project_remote_id_changed(*remote_id, cx);
            }
            if project.read(cx).is_read_only() {
                cx.blur();
            }
            cx.notify()
        })
        .detach();

        let pane = cx.add_view(|_| Pane::new());
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

        let status_bar = cx.add_view(|cx| StatusBar::new(&pane, cx));
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

        let weak_self = cx.weak_handle();

        cx.emit_global(WorkspaceCreated(weak_self.clone()));

        let mut this = Workspace {
            modal: None,
            weak_self,
            center: PaneGroup::new(pane.clone()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            status_bar,
            client: params.client.clone(),
            remote_entity_subscription: None,
            user_store: params.user_store.clone(),
            fs: params.fs.clone(),
            left_sidebar: Sidebar::new(Side::Left),
            right_sidebar: Sidebar::new(Side::Right),
            project: params.project.clone(),
            leader_state: Default::default(),
            follower_states_by_leader: Default::default(),
            _observe_current_user,
        };
        this.project_remote_id_changed(this.project.read(cx).remote_id(), cx);
        this
    }

    pub fn weak_handle(&self) -> WeakViewHandle<Self> {
        self.weak_self.clone()
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
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>> {
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
            project.find_or_create_local_worktree(abs_path, true, cx)
        });
        cx.spawn(|_, cx| async move {
            let (worktree, path) = entry.await?;
            Ok(ProjectPath {
                worktree_id: worktree.read_with(&cx, |t, _| t.id()),
                path: path.into(),
            })
        })
    }

    // Returns the model that was toggled closed if it was open
    pub fn toggle_modal<V, F>(
        &mut self,
        cx: &mut ViewContext<Self>,
        add_view: F,
    ) -> Option<ViewHandle<V>>
    where
        V: 'static + View,
        F: FnOnce(&mut ViewContext<Self>, &mut Self) -> ViewHandle<V>,
    {
        cx.notify();
        // Whatever modal was visible is getting clobbered. If its the same type as V, then return
        // it. Otherwise, create a new modal and set it as active.
        let already_open_modal = self.modal.take().and_then(|modal| modal.downcast::<V>());
        if let Some(already_open_modal) = already_open_modal {
            cx.focus_self();
            Some(already_open_modal)
        } else {
            let modal = add_view(cx, self);
            cx.focus(&modal);
            self.modal = Some(modal.into());
            None
        }
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

    pub fn items<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = &Box<dyn ItemHandle>> {
        self.panes.iter().flat_map(|pane| pane.read(cx).items())
    }

    pub fn item_of_type<T: Item>(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        self.items_of_type(cx).max_by_key(|item| item.id())
    }

    pub fn items_of_type<'a, T: Item>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ViewHandle<T>> {
        self.panes
            .iter()
            .flat_map(|pane| pane.read(cx).items_of_type())
    }

    pub fn active_item(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
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
        let pane = cx.add_view(|_| Pane::new());
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

    pub fn add_item(&mut self, item: Box<dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        let pane = self.active_pane().clone();
        Pane::add_item(self, pane, item, cx);
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>> {
        let pane = self.active_pane().downgrade();
        let task = self.load_path(path.into(), cx);
        cx.spawn(|this, mut cx| async move {
            let (project_entry_id, build_item) = task.await?;
            let pane = pane
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("pane was closed"))?;
            this.update(&mut cx, |this, cx| {
                Ok(Pane::open_item(
                    this,
                    pane,
                    project_entry_id,
                    cx,
                    build_item,
                ))
            })
        })
    }

    pub(crate) fn load_path(
        &mut self,
        path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Task<
        Result<(
            ProjectEntryId,
            impl 'static + FnOnce(&mut MutableAppContext) -> Box<dyn ItemHandle>,
        )>,
    > {
        let project = self.project().clone();
        let project_item = project.update(cx, |project, cx| project.open_path(path, cx));
        let window_id = cx.window_id();
        cx.as_mut().spawn(|mut cx| async move {
            let (project_entry_id, project_item) = project_item.await?;
            let build_item = cx.update(|cx| {
                cx.default_global::<ProjectItemBuilders>()
                    .get(&project_item.model_type())
                    .ok_or_else(|| anyhow!("no item builder for project item"))
                    .cloned()
            })?;
            let build_item =
                move |cx: &mut MutableAppContext| build_item(window_id, project, project_item, cx);
            Ok((project_entry_id, build_item))
        })
    }

    pub fn open_project_item<T>(
        &mut self,
        project_item: ModelHandle<T::Item>,
        cx: &mut ViewContext<Self>,
    ) -> ViewHandle<T>
    where
        T: ProjectItem,
    {
        use project::Item as _;

        if let Some(item) = project_item
            .read(cx)
            .entry_id(cx)
            .and_then(|entry_id| self.active_pane().read(cx).item_for_entry(entry_id, cx))
            .and_then(|item| item.downcast())
        {
            self.activate_item(&item, cx);
            return item;
        }

        let item = cx.add_view(|cx| T::for_project_item(self.project().clone(), project_item, cx));
        self.add_item(Box::new(item.clone()), cx);
        item
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
        if self.active_pane != pane {
            self.active_pane = pane.clone();
            self.status_bar.update(cx, |status_bar, cx| {
                status_bar.set_active_pane(&self.active_pane, cx);
            });
            cx.focus(&self.active_pane);
            cx.notify();
        }

        self.update_followers(
            proto::update_followers::Variant::UpdateActiveView(proto::UpdateActiveView {
                id: self.active_item(cx).map(|item| item.id() as u64),
                leader_id: self.leader_for_pane(&pane).map(|id| id.0),
            }),
            cx,
        );
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
                Pane::add_item(self, new_pane.clone(), clone, cx);
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
            self.unfollow(&pane, cx);
            cx.notify();
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

    fn project_remote_id_changed(&mut self, remote_id: Option<u64>, cx: &mut ViewContext<Self>) {
        if let Some(remote_id) = remote_id {
            self.remote_entity_subscription =
                Some(self.client.add_view_for_remote_entity(remote_id, cx));
        } else {
            self.remote_entity_subscription.take();
        }
    }

    pub fn toggle_follow(
        &mut self,
        ToggleFollow(leader_id): &ToggleFollow,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let leader_id = *leader_id;
        let pane = self.active_pane().clone();

        if let Some(prev_leader_id) = self.unfollow(&pane, cx) {
            if leader_id == prev_leader_id {
                cx.notify();
                return None;
            }
        }

        self.follower_states_by_leader
            .entry(leader_id)
            .or_default()
            .insert(pane.clone(), Default::default());

        let project_id = self.project.read(cx).remote_id()?;
        let request = self.client.request(proto::Follow {
            project_id,
            leader_id: leader_id.0,
        });
        Some(cx.spawn_weak(|this, mut cx| async move {
            let response = request.await?;
            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, _| {
                    let state = this
                        .follower_states_by_leader
                        .get_mut(&leader_id)
                        .and_then(|states_by_pane| states_by_pane.get_mut(&pane))
                        .ok_or_else(|| anyhow!("following interrupted"))?;
                    state.active_view_id = response.active_view_id;
                    Ok::<_, anyhow::Error>(())
                })?;
                Self::add_views_from_leader(this, leader_id, vec![pane], response.views, &mut cx)
                    .await?;
            }
            Ok(())
        }))
    }

    pub fn unfollow(
        &mut self,
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> Option<PeerId> {
        for (leader_id, states_by_pane) in &mut self.follower_states_by_leader {
            if let Some(state) = states_by_pane.remove(&pane) {
                for (_, item) in state.items_by_leader_view_id {
                    if let FollowerItem::Loaded(item) = item {
                        item.set_leader_replica_id(None, cx);
                    }
                }

                if states_by_pane.is_empty() {
                    if let Some(project_id) = self.project.read(cx).remote_id() {
                        self.client
                            .send(proto::Unfollow {
                                project_id,
                                leader_id: leader_id.0,
                            })
                            .log_err();
                    }
                }

                cx.notify();
                return Some(*leader_id);
            }
        }
        None
    }

    fn render_connection_status(&self, cx: &mut RenderContext<Self>) -> Option<ElementBox> {
        let theme = &cx.global::<Settings>().theme;
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
                                .with_children(self.render_connection_status(cx))
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
                    Some(collaborator.peer_id),
                    theme,
                    cx,
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
            self.render_avatar(avatar, replica_id, None, theme, cx)
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
        peer_id: Option<PeerId>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let content = Stack::new()
            .with_child(
                Image::new(avatar)
                    .with_style(theme.workspace.titlebar.avatar)
                    .constrained()
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
            .constrained()
            .with_width(theme.workspace.right_sidebar.width)
            .boxed();

        if let Some(peer_id) = peer_id {
            MouseEventHandler::new::<ToggleFollow, _, _>(replica_id.into(), cx, move |_, _| content)
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |cx| cx.dispatch_action(ToggleFollow(peer_id)))
                .boxed()
        } else {
            content
        }
    }

    fn render_share_icon(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> Option<ElementBox> {
        if self.project().read(cx).is_local() && self.client.user_id().is_some() {
            let color = if self.project().read(cx).is_shared() {
                theme.workspace.titlebar.share_icon_active_color
            } else {
                theme.workspace.titlebar.share_icon_color
            };
            Some(
                MouseEventHandler::new::<ToggleShare, _, _>(0, cx, |_, _| {
                    Align::new(
                        Svg::new("icons/broadcast-24.svg")
                            .with_color(color)
                            .constrained()
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

    fn render_disconnected_overlay(&self, cx: &AppContext) -> Option<ElementBox> {
        if self.project.read(cx).is_read_only() {
            let theme = &cx.global::<Settings>().theme;
            Some(
                EventHandler::new(
                    Label::new(
                        "Your connection to the remote project has been lost.".to_string(),
                        theme.workspace.disconnected_overlay.text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(theme.workspace.disconnected_overlay.container)
                    .boxed(),
                )
                .capture(|_, _, _| true)
                .boxed(),
            )
        } else {
            None
        }
    }

    // RPC handlers

    async fn handle_follow(
        this: ViewHandle<Self>,
        envelope: TypedEnvelope<proto::Follow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FollowResponse> {
        this.update(&mut cx, |this, cx| {
            this.leader_state
                .followers
                .insert(envelope.original_sender_id()?);

            let active_view_id = this
                .active_item(cx)
                .and_then(|i| i.to_followable_item_handle(cx))
                .map(|i| i.id() as u64);
            Ok(proto::FollowResponse {
                active_view_id,
                views: this
                    .panes()
                    .iter()
                    .flat_map(|pane| {
                        let leader_id = this.leader_for_pane(pane).map(|id| id.0);
                        pane.read(cx).items().filter_map({
                            let cx = &cx;
                            move |item| {
                                let id = item.id() as u64;
                                let item = item.to_followable_item_handle(cx)?;
                                let variant = item.to_state_message(cx)?;
                                Some(proto::View {
                                    id,
                                    leader_id,
                                    variant: Some(variant),
                                })
                            }
                        })
                    })
                    .collect(),
            })
        })
    }

    async fn handle_unfollow(
        this: ViewHandle<Self>,
        envelope: TypedEnvelope<proto::Unfollow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            this.leader_state
                .followers
                .remove(&envelope.original_sender_id()?);
            Ok(())
        })
    }

    async fn handle_update_followers(
        this: ViewHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        match envelope
            .payload
            .variant
            .ok_or_else(|| anyhow!("invalid update"))?
        {
            proto::update_followers::Variant::UpdateActiveView(update_active_view) => {
                this.update(&mut cx, |this, cx| {
                    this.update_leader_state(leader_id, cx, |state, _| {
                        state.active_view_id = update_active_view.id;
                    });
                    Ok::<_, anyhow::Error>(())
                })
            }
            proto::update_followers::Variant::UpdateView(update_view) => {
                this.update(&mut cx, |this, cx| {
                    let variant = update_view
                        .variant
                        .ok_or_else(|| anyhow!("missing update view variant"))?;
                    this.update_leader_state(leader_id, cx, |state, cx| {
                        let variant = variant.clone();
                        match state
                            .items_by_leader_view_id
                            .entry(update_view.id)
                            .or_insert(FollowerItem::Loading(Vec::new()))
                        {
                            FollowerItem::Loaded(item) => {
                                item.apply_update_message(variant, cx).log_err();
                            }
                            FollowerItem::Loading(updates) => updates.push(variant),
                        }
                    });
                    Ok(())
                })
            }
            proto::update_followers::Variant::CreateView(view) => {
                let panes = this.read_with(&cx, |this, _| {
                    this.follower_states_by_leader
                        .get(&leader_id)
                        .into_iter()
                        .flat_map(|states_by_pane| states_by_pane.keys())
                        .cloned()
                        .collect()
                });
                Self::add_views_from_leader(this.clone(), leader_id, panes, vec![view], &mut cx)
                    .await?;
                Ok(())
            }
        }
        .log_err();

        Ok(())
    }

    async fn add_views_from_leader(
        this: ViewHandle<Self>,
        leader_id: PeerId,
        panes: Vec<ViewHandle<Pane>>,
        views: Vec<proto::View>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let project = this.read_with(cx, |this, _| this.project.clone());
        let replica_id = project
            .read_with(cx, |project, _| {
                project
                    .collaborators()
                    .get(&leader_id)
                    .map(|c| c.replica_id)
            })
            .ok_or_else(|| anyhow!("no such collaborator {}", leader_id))?;

        let item_builders = cx.update(|cx| {
            cx.default_global::<FollowableItemBuilders>()
                .values()
                .map(|b| b.0)
                .collect::<Vec<_>>()
                .clone()
        });

        let mut item_tasks_by_pane = HashMap::default();
        for pane in panes {
            let mut item_tasks = Vec::new();
            let mut leader_view_ids = Vec::new();
            for view in &views {
                let mut variant = view.variant.clone();
                if variant.is_none() {
                    Err(anyhow!("missing variant"))?;
                }
                for build_item in &item_builders {
                    let task =
                        cx.update(|cx| build_item(pane.clone(), project.clone(), &mut variant, cx));
                    if let Some(task) = task {
                        item_tasks.push(task);
                        leader_view_ids.push(view.id);
                        break;
                    } else {
                        assert!(variant.is_some());
                    }
                }
            }

            item_tasks_by_pane.insert(pane, (item_tasks, leader_view_ids));
        }

        for (pane, (item_tasks, leader_view_ids)) in item_tasks_by_pane {
            let items = futures::future::try_join_all(item_tasks).await?;
            this.update(cx, |this, cx| {
                let state = this
                    .follower_states_by_leader
                    .get_mut(&leader_id)?
                    .get_mut(&pane)?;

                for (id, item) in leader_view_ids.into_iter().zip(items) {
                    item.set_leader_replica_id(Some(replica_id), cx);
                    match state.items_by_leader_view_id.entry(id) {
                        hash_map::Entry::Occupied(e) => {
                            let e = e.into_mut();
                            if let FollowerItem::Loading(updates) = e {
                                for update in updates.drain(..) {
                                    item.apply_update_message(update, cx)
                                        .context("failed to apply view update")
                                        .log_err();
                                }
                            }
                            *e = FollowerItem::Loaded(item);
                        }
                        hash_map::Entry::Vacant(e) => {
                            e.insert(FollowerItem::Loaded(item));
                        }
                    }
                }

                Some(())
            });
        }
        this.update(cx, |this, cx| this.leader_updated(leader_id, cx));

        Ok(())
    }

    fn update_followers(
        &self,
        update: proto::update_followers::Variant,
        cx: &AppContext,
    ) -> Option<()> {
        let project_id = self.project.read(cx).remote_id()?;
        if !self.leader_state.followers.is_empty() {
            self.client
                .send(proto::UpdateFollowers {
                    project_id,
                    follower_ids: self.leader_state.followers.iter().map(|f| f.0).collect(),
                    variant: Some(update),
                })
                .log_err();
        }
        None
    }

    fn leader_for_pane(&self, pane: &ViewHandle<Pane>) -> Option<PeerId> {
        self.follower_states_by_leader
            .iter()
            .find_map(|(leader_id, state)| {
                if state.contains_key(pane) {
                    Some(*leader_id)
                } else {
                    None
                }
            })
    }

    fn update_leader_state(
        &mut self,
        leader_id: PeerId,
        cx: &mut ViewContext<Self>,
        mut update_fn: impl FnMut(&mut FollowerState, &mut ViewContext<Self>),
    ) {
        for (_, state) in self
            .follower_states_by_leader
            .get_mut(&leader_id)
            .into_iter()
            .flatten()
        {
            update_fn(state, cx);
        }
        self.leader_updated(leader_id, cx);
    }

    fn leader_updated(&mut self, leader_id: PeerId, cx: &mut ViewContext<Self>) -> Option<()> {
        let mut items_to_add = Vec::new();
        for (pane, state) in self.follower_states_by_leader.get(&leader_id)? {
            if let Some(active_item) = state
                .active_view_id
                .and_then(|id| state.items_by_leader_view_id.get(&id))
            {
                if let FollowerItem::Loaded(item) = active_item {
                    items_to_add.push((pane.clone(), item.boxed_clone()));
                }
            }
        }

        for (pane, item) in items_to_add {
            Pane::add_item(self, pane.clone(), item.boxed_clone(), cx);
            cx.notify();
        }
        None
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
        let theme = cx.global::<Settings>().theme.clone();
        Stack::new()
            .with_child(
                Flex::column()
                    .with_child(self.render_titlebar(&theme, cx))
                    .with_child(
                        Stack::new()
                            .with_child({
                                let mut content = Flex::row();
                                content.add_child(self.left_sidebar.render(&theme, cx));
                                if let Some(element) =
                                    self.left_sidebar.render_active_item(&theme, cx)
                                {
                                    content.add_child(Flexible::new(0.8, false, element).boxed());
                                }
                                content.add_child(
                                    Flex::column()
                                        .with_child(
                                            Flexible::new(1., true, self.center.render(&theme))
                                                .boxed(),
                                        )
                                        .with_child(ChildView::new(&self.status_bar).boxed())
                                        .flexible(1., true)
                                        .boxed(),
                                );
                                if let Some(element) =
                                    self.right_sidebar.render_active_item(&theme, cx)
                                {
                                    content.add_child(Flexible::new(0.8, false, element).boxed());
                                }
                                content.add_child(self.right_sidebar.render(&theme, cx));
                                content.boxed()
                            })
                            .with_children(self.modal.as_ref().map(|m| ChildView::new(m).boxed()))
                            .flexible(1.0, true)
                            .boxed(),
                    )
                    .contained()
                    .with_background_color(theme.workspace.background)
                    .boxed(),
            )
            .with_children(self.render_disconnected_overlay(cx))
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

pub struct WorkspaceCreated(WeakViewHandle<Workspace>);

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
        Ok(cx.update(|cx| {
            cx.add_window((app_state.build_window_options)(), |cx| {
                (app_state.build_workspace)(project, &app_state, cx)
            })
            .1
        }))
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
