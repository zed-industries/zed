pub mod lsp_status;
pub mod menu;
pub mod pane;
pub mod pane_group;
pub mod sidebar;
mod status_bar;
mod toolbar;
mod waiting_room;

use anyhow::{anyhow, Context, Result};
use client::{
    proto, Authenticate, Client, Contact, PeerId, Subscription, TypedEnvelope, User, UserStore,
};
use clock::ReplicaId;
use collections::{hash_map, HashMap, HashSet};
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    impl_internal_actions,
    json::{self, ToJson},
    platform::{CursorStyle, WindowOptions},
    AnyModelHandle, AnyViewHandle, AppContext, AsyncAppContext, Border, Entity, ImageData,
    ModelHandle, MutableAppContext, PathPromptOptions, PromptLevel, RenderContext, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use language::LanguageRegistry;
use log::error;
pub use pane::*;
pub use pane_group::*;
use postage::prelude::Stream;
use project::{fs, Fs, Project, ProjectEntryId, ProjectPath, Worktree};
use settings::Settings;
use sidebar::{Side, Sidebar, SidebarButtons, ToggleSidebarItem, ToggleSidebarItemFocus};
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt,
    future::Future,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use theme::{Theme, ThemeRegistry};
pub use toolbar::{ToolbarItemLocation, ToolbarItemView};
use util::ResultExt;
use waiting_room::WaitingRoom;

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

actions!(
    workspace,
    [
        Open,
        OpenNew,
        Unfollow,
        Save,
        ActivatePreviousPane,
        ActivateNextPane,
        FollowNextCollaborator,
    ]
);

#[derive(Clone)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone)]
pub struct ToggleFollow(pub PeerId);

#[derive(Clone)]
pub struct JoinProject {
    pub contact: Arc<Contact>,
    pub project_index: usize,
}

impl_internal_actions!(workspace, [OpenPaths, ToggleFollow, JoinProject]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    pane::init(cx);

    cx.add_global_action(open);
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |action: &OpenPaths, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_paths(&action.paths, &app_state, cx).detach();
            }
        }
    });
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &OpenNew, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_new(&app_state, cx)
            }
        }
    });
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |action: &JoinProject, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                join_project(action.contact.clone(), action.project_index, &app_state, cx);
            }
        }
    });

    cx.add_async_action(Workspace::toggle_follow);
    cx.add_async_action(Workspace::follow_next_collaborator);
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
    cx.add_action(Workspace::toggle_sidebar_item);
    cx.add_action(Workspace::toggle_sidebar_item_focus);
    cx.add_action(|workspace: &mut Workspace, _: &ActivatePreviousPane, cx| {
        workspace.activate_previous_pane(cx)
    });
    cx.add_action(|workspace: &mut Workspace, _: &ActivateNextPane, cx| {
        workspace.activate_next_pane(cx)
    });

    let client = &app_state.client;
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
                    I::from_state_proto(pane, project, state, cx).map(|task| {
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
    pub build_window_options: fn() -> WindowOptions<'static>,
    pub initialize_workspace: fn(&mut Workspace, &Arc<AppState>, &mut ViewContext<Workspace>),
}

pub trait Item: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }
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
    fn reload(
        &mut self,
        project: ModelHandle<Project>,
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
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn from_state_proto(
        pane: ViewHandle<Pane>,
        project: ModelHandle<Project>,
        state: &mut Option<proto::view::Variant>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>>;
    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &mut self,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Result<()>;

    fn set_leader_replica_id(&mut self, leader_replica_id: Option<u16>, cx: &mut ViewContext<Self>);
    fn should_unfollow_on_event(event: &Self::Event, cx: &AppContext) -> bool;
}

pub trait FollowableItemHandle: ItemHandle {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext);
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()>;
    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool;
}

impl<T: FollowableItem> FollowableItemHandle for ViewHandle<T> {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| {
            this.set_leader_replica_id(leader_replica_id, cx)
        })
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        self.read(cx).to_state_proto(cx)
    }

    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool {
        if let Some(event) = event.downcast_ref() {
            self.read(cx).add_event_to_update_proto(event, update, cx)
        } else {
            false
        }
    }

    fn apply_update_proto(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()> {
        self.update(cx, |this, cx| this.apply_update_proto(message, cx))
    }

    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool {
        if let Some(event) = event.downcast_ref() {
            T::should_unfollow_on_event(event, cx)
        } else {
            false
        }
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
    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool;
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
    fn reload(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext)
        -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyViewHandle>;
    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut MutableAppContext,
        callback: Box<dyn FnOnce(&mut MutableAppContext)>,
    ) -> gpui::Subscription;
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

    fn set_nav_history(&self, nav_history: Rc<RefCell<NavHistory>>, cx: &mut MutableAppContext) {
        self.update(cx, |item, cx| {
            item.set_nav_history(ItemNavHistory::new(nav_history, &cx.handle()), cx);
        })
    }

    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>> {
        self.update(cx, |item, cx| {
            cx.add_option_view(|cx| item.clone_on_split(cx))
        })
        .map(|handle| Box::new(handle) as Box<dyn ItemHandle>)
    }

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some(followed_item) = self.to_followable_item_handle(cx) {
            if let Some(message) = followed_item.to_state_proto(cx) {
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

        let pending_update = Rc::new(RefCell::new(None));
        let pending_update_scheduled = Rc::new(AtomicBool::new(false));
        let pane = pane.downgrade();
        cx.subscribe(self, move |workspace, item, event, cx| {
            let pane = if let Some(pane) = pane.upgrade(cx) {
                pane
            } else {
                log::error!("unexpected item event after pane was dropped");
                return;
            };

            if let Some(item) = item.to_followable_item_handle(cx) {
                let leader_id = workspace.leader_for_pane(&pane);

                if leader_id.is_some() && item.should_unfollow_on_event(event, cx) {
                    workspace.unfollow(&pane, cx);
                }

                if item.add_event_to_update_proto(event, &mut *pending_update.borrow_mut(), cx)
                    && !pending_update_scheduled.load(SeqCst)
                {
                    pending_update_scheduled.store(true, SeqCst);
                    cx.after_window_update({
                        let pending_update = pending_update.clone();
                        let pending_update_scheduled = pending_update_scheduled.clone();
                        move |this, cx| {
                            pending_update_scheduled.store(false, SeqCst);
                            this.update_followers(
                                proto::update_followers::Variant::UpdateView(proto::UpdateView {
                                    id: item.id() as u64,
                                    variant: pending_update.borrow_mut().take(),
                                    leader_id: leader_id.map(|id| id.0),
                                }),
                                cx,
                            );
                        }
                    });
                }
            }

            if T::should_close_item_on_event(event) {
                Pane::close_item(workspace, pane, item.id(), cx).detach_and_log_err(cx);
                return;
            }

            if T::should_activate_item_on_event(event) {
                pane.update(cx, |pane, cx| {
                    if let Some(ix) = pane.index_for_item(&item) {
                        pane.activate_item(ix, true, true, cx);
                        pane.activate(cx);
                    }
                });
            }

            if T::should_update_tab_on_event(event) {
                pane.update(cx, |_, cx| cx.notify());
            }
        })
        .detach();
    }

    fn deactivated(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.deactivated(cx));
    }

    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool {
        self.update(cx, |this, cx| this.navigate(data, cx))
    }

    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.read(cx).has_conflict(cx)
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        self.read(cx).can_save(cx)
    }

    fn can_save_as(&self, cx: &AppContext) -> bool {
        self.read(cx).can_save_as(cx)
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

    fn reload(
        &self,
        project: ModelHandle<Project>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.reload(project, cx))
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

    fn on_release(
        &self,
        cx: &mut MutableAppContext,
        callback: Box<dyn FnOnce(&mut MutableAppContext)>,
    ) -> gpui::Subscription {
        cx.observe_release(self, move |_, cx| callback(cx))
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

pub trait Notification: View {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool;
}

pub trait NotificationHandle {
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
}

impl<T: Notification> NotificationHandle for ViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }
}

impl Into<AnyViewHandle> for &dyn NotificationHandle {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}

impl AppState {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut MutableAppContext) -> Arc<Self> {
        let settings = Settings::test(cx);
        cx.set_global(settings);

        let fs = project::FakeFs::new(cx.background().clone());
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let themes = ThemeRegistry::new((), cx.font_cache().clone());
        Arc::new(Self {
            client,
            themes,
            fs,
            languages,
            user_store,
            initialize_workspace: |_, _, _| {},
            build_window_options: || Default::default(),
        })
    }
}

pub enum Event {
    PaneAdded(ViewHandle<Pane>),
    ContactRequestedJoin(u64),
}

pub struct Workspace {
    weak_self: WeakViewHandle<Self>,
    client: Arc<Client>,
    user_store: ModelHandle<client::UserStore>,
    remote_entity_subscription: Option<Subscription>,
    fs: Arc<dyn Fs>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    left_sidebar: ViewHandle<Sidebar>,
    right_sidebar: ViewHandle<Sidebar>,
    panes: Vec<ViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    status_bar: ViewHandle<StatusBar>,
    notifications: Vec<(TypeId, usize, Box<dyn NotificationHandle>)>,
    project: ModelHandle<Project>,
    leader_state: LeaderState,
    follower_states_by_leader: FollowerStatesByLeader,
    last_leaders_by_pane: HashMap<WeakViewHandle<Pane>, PeerId>,
    _observe_current_user: Task<()>,
}

#[derive(Default)]
struct LeaderState {
    followers: HashSet<PeerId>,
}

type FollowerStatesByLeader = HashMap<PeerId, HashMap<ViewHandle<Pane>, FollowerState>>;

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
    pub fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&project, |_, project, cx| {
            if project.read(cx).is_read_only() {
                cx.blur();
            }
            cx.notify()
        })
        .detach();

        cx.subscribe(&project, move |this, project, event, cx| {
            match event {
                project::Event::RemoteIdChanged(remote_id) => {
                    this.project_remote_id_changed(*remote_id, cx);
                }
                project::Event::CollaboratorLeft(peer_id) => {
                    this.collaborator_left(*peer_id, cx);
                }
                _ => {}
            }
            if project.read(cx).is_read_only() {
                cx.blur();
            }
            cx.notify()
        })
        .detach();

        let pane = cx.add_view(|cx| Pane::new(cx));
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
        cx.emit(Event::PaneAdded(pane.clone()));

        let fs = project.read(cx).fs().clone();
        let user_store = project.read(cx).user_store();
        let client = project.read(cx).client();
        let mut current_user = user_store.read(cx).watch_current_user().clone();
        let mut connection_status = client.status().clone();
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

        let left_sidebar = cx.add_view(|_| Sidebar::new(Side::Left));
        let right_sidebar = cx.add_view(|_| Sidebar::new(Side::Right));
        let left_sidebar_buttons = cx.add_view(|cx| SidebarButtons::new(left_sidebar.clone(), cx));
        let right_sidebar_buttons =
            cx.add_view(|cx| SidebarButtons::new(right_sidebar.clone(), cx));
        let status_bar = cx.add_view(|cx| {
            let mut status_bar = StatusBar::new(&pane.clone(), cx);
            status_bar.add_left_item(left_sidebar_buttons, cx);
            status_bar.add_right_item(right_sidebar_buttons, cx);
            status_bar
        });

        let mut this = Workspace {
            modal: None,
            weak_self,
            center: PaneGroup::new(pane.clone()),
            panes: vec![pane.clone()],
            active_pane: pane.clone(),
            status_bar,
            notifications: Default::default(),
            client,
            remote_entity_subscription: None,
            user_store,
            fs,
            left_sidebar,
            right_sidebar,
            project,
            leader_state: Default::default(),
            follower_states_by_leader: Default::default(),
            last_leaders_by_pane: Default::default(),
            _observe_current_user,
        };
        this.project_remote_id_changed(this.project.read(cx).remote_id(), cx);
        this
    }

    pub fn weak_handle(&self) -> WeakViewHandle<Self> {
        self.weak_self.clone()
    }

    pub fn left_sidebar(&self) -> &ViewHandle<Sidebar> {
        &self.left_sidebar
    }

    pub fn right_sidebar(&self) -> &ViewHandle<Sidebar> {
        &self.right_sidebar
    }

    pub fn status_bar(&self) -> &ViewHandle<StatusBar> {
        &self.status_bar
    }

    pub fn user_store(&self) -> &ModelHandle<UserStore> {
        &self.user_store
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
        mut abs_paths: Vec<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>> {
        let fs = self.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn(|this, mut cx| async move {
            let mut entries = Vec::new();
            for path in &abs_paths {
                entries.push(
                    this.update(&mut cx, |this, cx| this.project_path_for_path(path, cx))
                        .await
                        .ok(),
                );
            }

            let tasks = abs_paths
                .iter()
                .cloned()
                .zip(entries.into_iter())
                .map(|(abs_path, project_path)| {
                    let this = this.clone();
                    cx.spawn(|mut cx| {
                        let fs = fs.clone();
                        async move {
                            let project_path = project_path?;
                            if fs.is_file(&abs_path).await {
                                Some(
                                    this.update(&mut cx, |this, cx| {
                                        this.open_path(project_path, true, cx)
                                    })
                                    .await,
                                )
                            } else {
                                None
                            }
                        }
                    })
                })
                .collect::<Vec<_>>();

            futures::future::join_all(tasks).await
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

    /// Returns the modal that was toggled closed if it was open.
    pub fn toggle_modal<V, F>(
        &mut self,
        cx: &mut ViewContext<Self>,
        add_view: F,
    ) -> Option<ViewHandle<V>>
    where
        V: 'static + View,
        F: FnOnce(&mut Self, &mut ViewContext<Self>) -> ViewHandle<V>,
    {
        cx.notify();
        // Whatever modal was visible is getting clobbered. If its the same type as V, then return
        // it. Otherwise, create a new modal and set it as active.
        let already_open_modal = self.modal.take().and_then(|modal| modal.downcast::<V>());
        if let Some(already_open_modal) = already_open_modal {
            cx.focus_self();
            Some(already_open_modal)
        } else {
            let modal = add_view(self, cx);
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

    pub fn show_notification<V: Notification>(
        &mut self,
        id: usize,
        cx: &mut ViewContext<Self>,
        build_notification: impl FnOnce(&mut ViewContext<Self>) -> ViewHandle<V>,
    ) {
        let type_id = TypeId::of::<V>();
        if self
            .notifications
            .iter()
            .all(|(existing_type_id, existing_id, _)| {
                (*existing_type_id, *existing_id) != (type_id, id)
            })
        {
            let notification = build_notification(cx);
            cx.subscribe(&notification, move |this, handle, event, cx| {
                if handle.read(cx).should_dismiss_notification_on_event(event) {
                    this.dismiss_notification(type_id, id, cx);
                }
            })
            .detach();
            self.notifications
                .push((type_id, id, Box::new(notification)));
            cx.notify();
        }
    }

    fn dismiss_notification(&mut self, type_id: TypeId, id: usize, cx: &mut ViewContext<Self>) {
        self.notifications
            .retain(|(existing_type_id, existing_id, _)| {
                if (*existing_type_id, *existing_id) == (type_id, id) {
                    cx.notify();
                    false
                } else {
                    true
                }
            });
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
        let sidebar = match action.side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, |sidebar, cx| {
            sidebar.toggle_item(action.item_index, cx);
            sidebar.active_item().map(|item| item.to_any())
        });
        if let Some(active_item) = active_item {
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
        let sidebar = match action.side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, |sidebar, cx| {
            sidebar.activate_item(action.item_index, cx);
            sidebar.active_item().cloned()
        });
        if let Some(active_item) = active_item {
            if active_item.is_focused(cx) {
                cx.focus_self();
            } else {
                cx.focus(active_item.to_any());
            }
        }
        cx.notify();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = cx.add_view(|cx| Pane::new(cx));
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
        cx.emit(Event::PaneAdded(pane.clone()));
        pane
    }

    pub fn add_item(&mut self, item: Box<dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        let pane = self.active_pane().clone();
        Pane::add_item(self, pane, item, true, true, cx);
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        focus_item: bool,
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
                    focus_item,
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

        let entry_id = project_item.read(cx).entry_id(cx);
        if let Some(item) = entry_id
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
            pane.update(cx, |pane, cx| pane.activate_item(ix, true, true, cx));
            true
        } else {
            false
        }
    }

    pub fn activate_next_pane(&mut self, cx: &mut ViewContext<Self>) {
        let next_pane = {
            let panes = self.center.panes();
            let ix = panes
                .iter()
                .position(|pane| **pane == self.active_pane)
                .unwrap();
            let next_ix = (ix + 1) % panes.len();
            panes[next_ix].clone()
        };
        self.activate_pane(next_pane, cx);
    }

    pub fn activate_previous_pane(&mut self, cx: &mut ViewContext<Self>) {
        let prev_pane = {
            let panes = self.center.panes();
            let ix = panes
                .iter()
                .position(|pane| **pane == self.active_pane)
                .unwrap();
            let prev_ix = if ix == 0 { panes.len() - 1 } else { ix - 1 };
            panes[prev_ix].clone()
        };
        self.activate_pane(prev_pane, cx);
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
                pane::Event::ActivateItem { local } => {
                    if *local {
                        self.unfollow(&pane, cx);
                    }
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
                Pane::add_item(self, new_pane.clone(), clone, true, true, cx);
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
            self.last_leaders_by_pane.remove(&pane.downgrade());
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

    fn project_remote_id_changed(&mut self, remote_id: Option<u64>, cx: &mut ViewContext<Self>) {
        if let Some(remote_id) = remote_id {
            self.remote_entity_subscription =
                Some(self.client.add_view_for_remote_entity(remote_id, cx));
        } else {
            self.remote_entity_subscription.take();
        }
    }

    fn collaborator_left(&mut self, peer_id: PeerId, cx: &mut ViewContext<Self>) {
        self.leader_state.followers.remove(&peer_id);
        if let Some(states_by_pane) = self.follower_states_by_leader.remove(&peer_id) {
            for state in states_by_pane.into_values() {
                for item in state.items_by_leader_view_id.into_values() {
                    if let FollowerItem::Loaded(item) = item {
                        item.set_leader_replica_id(None, cx);
                    }
                }
            }
        }
        cx.notify();
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
                return None;
            }
        }

        self.last_leaders_by_pane
            .insert(pane.downgrade(), leader_id);
        self.follower_states_by_leader
            .entry(leader_id)
            .or_default()
            .insert(pane.clone(), Default::default());
        cx.notify();

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

    pub fn follow_next_collaborator(
        &mut self,
        _: &FollowNextCollaborator,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let collaborators = self.project.read(cx).collaborators();
        let next_leader_id = if let Some(leader_id) = self.leader_for_pane(&self.active_pane) {
            let mut collaborators = collaborators.keys().copied();
            while let Some(peer_id) = collaborators.next() {
                if peer_id == leader_id {
                    break;
                }
            }
            collaborators.next()
        } else if let Some(last_leader_id) =
            self.last_leaders_by_pane.get(&self.active_pane.downgrade())
        {
            if collaborators.contains_key(last_leader_id) {
                Some(*last_leader_id)
            } else {
                None
            }
        } else {
            None
        };

        next_leader_id
            .or_else(|| collaborators.keys().copied().next())
            .and_then(|leader_id| self.toggle_follow(&ToggleFollow(leader_id), cx))
    }

    pub fn unfollow(
        &mut self,
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> Option<PeerId> {
        for (leader_id, states_by_pane) in &mut self.follower_states_by_leader {
            let leader_id = *leader_id;
            if let Some(state) = states_by_pane.remove(&pane) {
                for (_, item) in state.items_by_leader_view_id {
                    if let FollowerItem::Loaded(item) = item {
                        item.set_leader_replica_id(None, cx);
                    }
                }

                if states_by_pane.is_empty() {
                    self.follower_states_by_leader.remove(&leader_id);
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
                return Some(leader_id);
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
        let mut worktree_root_names = String::new();
        {
            let mut worktrees = self.project.read(cx).visible_worktrees(cx).peekable();
            while let Some(worktree) = worktrees.next() {
                worktree_root_names.push_str(worktree.read(cx).root_name());
                if worktrees.peek().is_some() {
                    worktree_root_names.push_str(", ");
                }
            }
        }

        ConstrainedBox::new(
            Container::new(
                Stack::new()
                    .with_child(
                        Label::new(worktree_root_names, theme.workspace.titlebar.title.clone())
                            .aligned()
                            .left()
                            .boxed(),
                    )
                    .with_child(
                        Align::new(
                            Flex::row()
                                .with_children(self.render_collaborators(theme, cx))
                                .with_children(self.render_current_user(
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
    ) -> Option<ElementBox> {
        let status = *self.client.status().borrow();
        if let Some(avatar) = user.and_then(|user| user.avatar.clone()) {
            Some(self.render_avatar(avatar, replica_id, None, theme, cx))
        } else if matches!(status, client::Status::UpgradeRequired) {
            None
        } else {
            Some(
                MouseEventHandler::new::<Authenticate, _, _>(0, cx, |state, _| {
                    let style = theme
                        .workspace
                        .titlebar
                        .sign_in_prompt
                        .style_for(state, false);
                    Label::new("Sign in".to_string(), style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .on_click(|_, cx| cx.dispatch_action(Authenticate))
                .with_cursor_style(CursorStyle::PointingHand)
                .aligned()
                .boxed(),
            )
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
        let replica_color = theme.editor.replica_selection_style(replica_id).cursor;
        let is_followed = peer_id.map_or(false, |peer_id| {
            self.follower_states_by_leader.contains_key(&peer_id)
        });
        let mut avatar_style = theme.workspace.titlebar.avatar;
        if is_followed {
            avatar_style.border = Border::all(1.0, replica_color);
        }
        let content = Stack::new()
            .with_child(
                Image::new(avatar)
                    .with_style(avatar_style)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_width)
                    .aligned()
                    .boxed(),
            )
            .with_child(
                AvatarRibbon::new(replica_color)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_ribbon.width)
                    .with_height(theme.workspace.titlebar.avatar_ribbon.height)
                    .aligned()
                    .bottom()
                    .boxed(),
            )
            .constrained()
            .with_width(theme.workspace.titlebar.avatar_width)
            .contained()
            .with_margin_left(theme.workspace.titlebar.avatar_margin)
            .boxed();

        if let Some(peer_id) = peer_id {
            MouseEventHandler::new::<ToggleFollow, _, _>(replica_id.into(), cx, move |_, _| content)
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |_, cx| cx.dispatch_action(ToggleFollow(peer_id)))
                .boxed()
        } else {
            content
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

    fn render_notifications(&self, theme: &theme::Workspace) -> Option<ElementBox> {
        if self.notifications.is_empty() {
            None
        } else {
            Some(
                Flex::column()
                    .with_children(self.notifications.iter().map(|(_, _, notification)| {
                        ChildView::new(notification.as_ref())
                            .contained()
                            .with_style(theme.notification)
                            .boxed()
                    }))
                    .constrained()
                    .with_width(theme.notifications.width)
                    .contained()
                    .with_style(theme.notifications.container)
                    .aligned()
                    .bottom()
                    .right()
                    .boxed(),
            )
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
                                let variant = item.to_state_proto(cx)?;
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
                                item.apply_update_proto(variant, cx).log_err();
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
                                    item.apply_update_proto(update, cx)
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

    pub fn leader_for_pane(&self, pane: &ViewHandle<Pane>) -> Option<PeerId> {
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
            Pane::add_item(self, pane.clone(), item.boxed_clone(), false, false, cx);
            if pane == self.active_pane {
                pane.update(cx, |pane, cx| pane.focus_active_item(cx));
            }
            cx.notify();
        }
        None
    }
}

impl Entity for Workspace {
    type Event = Event;
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
                                Flex::row()
                                    .with_children(
                                        if self.left_sidebar.read(cx).active_item().is_some() {
                                            Some(
                                                ChildView::new(&self.left_sidebar)
                                                    .flex(0.8, false)
                                                    .boxed(),
                                            )
                                        } else {
                                            None
                                        },
                                    )
                                    .with_child(
                                        FlexItem::new(self.center.render(
                                            &theme,
                                            &self.follower_states_by_leader,
                                            self.project.read(cx).collaborators(),
                                        ))
                                        .flex(1., true)
                                        .boxed(),
                                    )
                                    .with_children(
                                        if self.right_sidebar.read(cx).active_item().is_some() {
                                            Some(
                                                ChildView::new(&self.right_sidebar)
                                                    .flex(0.8, false)
                                                    .boxed(),
                                            )
                                        } else {
                                            None
                                        },
                                    )
                                    .boxed()
                            })
                            .with_children(self.modal.as_ref().map(|m| {
                                ChildView::new(m)
                                    .contained()
                                    .with_style(theme.workspace.modal)
                                    .aligned()
                                    .top()
                                    .boxed()
                            }))
                            .with_children(self.render_notifications(&theme.workspace))
                            .flex(1.0, true)
                            .boxed(),
                    )
                    .with_child(ChildView::new(&self.status_bar).boxed())
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
        _: RectF,
        _: RectF,
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

impl std::fmt::Debug for OpenPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenPaths")
            .field("paths", &self.paths)
            .finish()
    }
}

fn open(_: &Open, cx: &mut MutableAppContext) {
    let mut paths = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: true,
        multiple: true,
    });
    cx.spawn(|mut cx| async move {
        if let Some(paths) = paths.recv().await.flatten() {
            cx.update(|cx| cx.dispatch_global_action(OpenPaths { paths }));
        }
    })
    .detach();
}

pub struct WorkspaceCreated(WeakViewHandle<Workspace>);

pub fn open_paths(
    abs_paths: &[PathBuf],
    app_state: &Arc<AppState>,
    cx: &mut MutableAppContext,
) -> Task<(
    ViewHandle<Workspace>,
    Vec<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>,
)> {
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

    let app_state = app_state.clone();
    let abs_paths = abs_paths.to_vec();
    cx.spawn(|mut cx| async move {
        let workspace = if let Some(existing) = existing {
            existing
        } else {
            let contains_directory =
                futures::future::join_all(abs_paths.iter().map(|path| app_state.fs.is_file(path)))
                    .await
                    .contains(&false);

            cx.add_window((app_state.build_window_options)(), |cx| {
                let mut workspace = Workspace::new(
                    Project::local(
                        app_state.client.clone(),
                        app_state.user_store.clone(),
                        app_state.languages.clone(),
                        app_state.fs.clone(),
                        cx,
                    ),
                    cx,
                );
                (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                if contains_directory {
                    workspace.toggle_sidebar_item(
                        &ToggleSidebarItem {
                            side: Side::Left,
                            item_index: 0,
                        },
                        cx,
                    );
                }
                workspace
            })
            .1
        };

        let items = workspace
            .update(&mut cx, |workspace, cx| workspace.open_paths(abs_paths, cx))
            .await;
        (workspace, items)
    })
}

pub fn join_project(
    contact: Arc<Contact>,
    project_index: usize,
    app_state: &Arc<AppState>,
    cx: &mut MutableAppContext,
) {
    let project_id = contact.projects[project_index].id;

    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(workspace) = cx.root_view::<Workspace>(window_id) {
            if workspace.read(cx).project().read(cx).remote_id() == Some(project_id) {
                cx.activate_window(window_id);
                return;
            }
        }
    }

    cx.add_window((app_state.build_window_options)(), |cx| {
        WaitingRoom::new(contact, project_index, app_state.clone(), cx)
    });
}

fn open_new(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    let (window_id, workspace) = cx.add_window((app_state.build_window_options)(), |cx| {
        let mut workspace = Workspace::new(
            Project::local(
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx,
            ),
            cx,
        );
        (app_state.initialize_workspace)(&mut workspace, app_state, cx);
        workspace
    });
    cx.dispatch_action(window_id, vec![workspace.id()], &OpenNew);
}
