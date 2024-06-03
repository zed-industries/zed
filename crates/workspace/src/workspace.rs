pub mod dock;
pub mod item;
mod modal_layer;
pub mod notifications;
pub mod pane;
pub mod pane_group;
mod persistence;
pub mod searchable;
pub mod shared_screen;
mod status_bar;
pub mod tasks;
mod toolbar;
mod workspace_settings;

use anyhow::{anyhow, Context as _, Result};
use call::{call_settings::CallSettings, ActiveCall};
use client::{
    proto::{self, ErrorCode, PeerId},
    ChannelId, Client, ErrorExt, ProjectId, Status, TypedEnvelope, UserStore,
};
use collections::{hash_map, HashMap, HashSet};
use derive_more::{Deref, DerefMut};
use dock::{Dock, DockPosition, Panel, PanelButtons, PanelHandle};
use futures::{
    channel::{mpsc, oneshot},
    future::try_join_all,
    Future, FutureExt, StreamExt,
};
use gpui::{
    actions, canvas, impl_actions, point, relative, size, Action, AnyElement, AnyView, AnyWeakView,
    AppContext, AsyncAppContext, AsyncWindowContext, Bounds, DevicePixels, DragMoveEvent,
    ElementId, Entity as _, EntityId, EventEmitter, FocusHandle, FocusableView, Global,
    GlobalElementId, KeyContext, Keystroke, LayoutId, ManagedView, Model, ModelContext,
    PathPromptOptions, Point, PromptLevel, Render, Size, Subscription, Task, View, WeakView,
    WindowBounds, WindowHandle, WindowOptions,
};
use item::{
    FollowableItem, FollowableItemHandle, Item, ItemHandle, ItemSettings, PreviewTabsSettings,
    ProjectItem,
};
use itertools::Itertools;
use language::{LanguageRegistry, Rope};
use lazy_static::lazy_static;
pub use modal_layer::*;
use node_runtime::NodeRuntime;
use notifications::{simple_message_notification::MessageNotification, NotificationHandle};
pub use pane::*;
pub use pane_group::*;
use persistence::{model::SerializedWorkspace, SerializedWindowBounds, DB};
pub use persistence::{
    model::{ItemId, LocalPaths, SerializedDevServerProject, SerializedWorkspaceLocation},
    WorkspaceDb, DB as WORKSPACE_DB,
};
use postage::stream::Stream;
use project::{Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId};
use serde::Deserialize;
use settings::Settings;
use shared_screen::SharedScreen;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use std::{
    any::TypeId,
    borrow::Cow,
    cell::RefCell,
    cmp,
    collections::hash_map::DefaultHasher,
    env,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc, Weak},
    time::Duration,
};
use task::SpawnInTerminal;
use theme::{ActiveTheme, SystemAppearance, ThemeSettings};
pub use toolbar::{Toolbar, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};
pub use ui;
use ui::{
    div, h_flex, Context as _, Div, Element, FluentBuilder, InteractiveElement as _, IntoElement,
    Label, ParentElement as _, Pixels, SharedString, Styled as _, ViewContext, VisualContext as _,
    WindowContext,
};
use util::{maybe, ResultExt};
use uuid::Uuid;
pub use workspace_settings::{
    AutosaveSetting, RestoreOnStartupBehaviour, TabBarSettings, WorkspaceSettings,
};

use crate::persistence::{
    model::{DockData, DockStructure, SerializedItem, SerializedPane, SerializedPaneGroup},
    SerializedAxis,
};
use crate::{notifications::NotificationId, persistence::model::LocalPathsOrder};

lazy_static! {
    static ref ZED_WINDOW_SIZE: Option<Size<DevicePixels>> = env::var("ZED_WINDOW_SIZE")
        .ok()
        .as_deref()
        .and_then(parse_pixel_size_env_var);
    static ref ZED_WINDOW_POSITION: Option<Point<DevicePixels>> = env::var("ZED_WINDOW_POSITION")
        .ok()
        .as_deref()
        .and_then(parse_pixel_position_env_var);
}

#[derive(Clone, PartialEq)]
pub struct RemoveWorktreeFromProject(pub WorktreeId);

actions!(
    workspace,
    [
        Open,
        OpenInTerminal,
        NewFile,
        NewWindow,
        CloseWindow,
        AddFolderToProject,
        Unfollow,
        SaveAs,
        SaveWithoutFormat,
        ReloadActiveItem,
        ActivatePreviousPane,
        ActivateNextPane,
        FollowNextCollaborator,
        NewTerminal,
        NewCenterTerminal,
        NewSearch,
        Feedback,
        Welcome,
        ToggleZoom,
        ToggleLeftDock,
        ToggleRightDock,
        ToggleBottomDock,
        ToggleCenteredLayout,
        CloseAllDocks,
    ]
);

#[derive(Clone, PartialEq)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePane(pub usize);

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePaneInDirection(pub SplitDirection);

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwapPaneInDirection(pub SplitDirection);

#[derive(Clone, Deserialize, PartialEq)]
pub struct NewFileInDirection(pub SplitDirection);

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAll {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Save {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseAllItemsAndPanes {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseInactiveTabsAndPanes {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct SendKeystrokes(pub String);

#[derive(Clone, Deserialize, PartialEq, Default)]
pub struct Restart {
    pub binary_path: Option<PathBuf>,
}

impl_actions!(
    workspace,
    [
        ActivatePane,
        ActivatePaneInDirection,
        CloseAllItemsAndPanes,
        CloseInactiveTabsAndPanes,
        NewFileInDirection,
        OpenTerminal,
        Restart,
        Save,
        SaveAll,
        SwapPaneInDirection,
        SendKeystrokes,
    ]
);

pub struct Toast {
    id: NotificationId,
    msg: Cow<'static, str>,
    on_click: Option<(Cow<'static, str>, Arc<dyn Fn(&mut WindowContext)>)>,
}

impl Toast {
    pub fn new<I: Into<Cow<'static, str>>>(id: NotificationId, msg: I) -> Self {
        Toast {
            id,
            msg: msg.into(),
            on_click: None,
        }
    }

    pub fn on_click<F, M>(mut self, message: M, on_click: F) -> Self
    where
        M: Into<Cow<'static, str>>,
        F: Fn(&mut WindowContext) + 'static,
    {
        self.on_click = Some((message.into(), Arc::new(on_click)));
        self
    }
}

impl PartialEq for Toast {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.msg == other.msg
            && self.on_click.is_some() == other.on_click.is_some()
    }
}

impl Clone for Toast {
    fn clone(&self) -> Self {
        Toast {
            id: self.id.clone(),
            msg: self.msg.clone(),
            on_click: self.on_click.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct OpenTerminal {
    pub working_directory: PathBuf,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceId(i64);

impl StaticColumnCount for WorkspaceId {}
impl Bind for WorkspaceId {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.0.bind(statement, start_index)
    }
}
impl Column for WorkspaceId {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        i64::column(statement, start_index)
            .map(|(i, next_index)| (Self(i), next_index))
            .with_context(|| format!("Failed to read WorkspaceId at index {start_index}"))
    }
}
pub fn init_settings(cx: &mut AppContext) {
    WorkspaceSettings::register(cx);
    ItemSettings::register(cx);
    PreviewTabsSettings::register(cx);
    TabBarSettings::register(cx);
}

pub fn init(app_state: Arc<AppState>, cx: &mut AppContext) {
    init_settings(cx);
    notifications::init(cx);

    cx.on_action(Workspace::close_global);
    cx.on_action(restart);

    cx.on_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &Open, cx: &mut AppContext| {
            let paths = cx.prompt_for_paths(PathPromptOptions {
                files: true,
                directories: true,
                multiple: true,
            });

            if let Some(app_state) = app_state.upgrade() {
                cx.spawn(move |cx| async move {
                    if let Some(paths) = paths.await.log_err().flatten() {
                        cx.update(|cx| {
                            open_paths(&paths, app_state, OpenOptions::default(), cx)
                                .detach_and_log_err(cx)
                        })
                        .ok();
                    }
                })
                .detach();
            }
        }
    });
}

#[derive(Clone, Default, Deref, DerefMut)]
struct ProjectItemOpeners(Vec<ProjectItemOpener>);

type ProjectItemOpener = fn(
    &Model<Project>,
    &ProjectPath,
    &mut WindowContext,
)
    -> Option<Task<Result<(Option<ProjectEntryId>, WorkspaceItemBuilder)>>>;

type WorkspaceItemBuilder = Box<dyn FnOnce(&mut ViewContext<Pane>) -> Box<dyn ItemHandle>>;

impl Global for ProjectItemOpeners {}

/// Registers a [ProjectItem] for the app. When opening a file, all the registered
/// items will get a chance to open the file, starting from the project item that
/// was added last.
pub fn register_project_item<I: ProjectItem>(cx: &mut AppContext) {
    let builders = cx.default_global::<ProjectItemOpeners>();
    builders.push(|project, project_path, cx| {
        let project_item = <I::Item as project::Item>::try_open(&project, project_path, cx)?;
        let project = project.clone();
        Some(cx.spawn(|cx| async move {
            let project_item = project_item.await?;
            let project_entry_id: Option<ProjectEntryId> =
                project_item.read_with(&cx, |item, cx| project::Item::entry_id(item, cx))?;
            let build_workspace_item = Box::new(|cx: &mut ViewContext<Pane>| {
                Box::new(cx.new_view(|cx| I::for_project_item(project, project_item, cx)))
                    as Box<dyn ItemHandle>
            }) as Box<_>;
            Ok((project_entry_id, build_workspace_item))
        }))
    });
}

type FollowableItemBuilder = fn(
    View<Pane>,
    View<Workspace>,
    ViewId,
    &mut Option<proto::view::Variant>,
    &mut WindowContext,
) -> Option<Task<Result<Box<dyn FollowableItemHandle>>>>;

#[derive(Default, Deref, DerefMut)]
struct FollowableItemBuilders(
    HashMap<
        TypeId,
        (
            FollowableItemBuilder,
            fn(&AnyView) -> Box<dyn FollowableItemHandle>,
        ),
    >,
);

impl Global for FollowableItemBuilders {}

pub fn register_followable_item<I: FollowableItem>(cx: &mut AppContext) {
    let builders = cx.default_global::<FollowableItemBuilders>();
    builders.insert(
        TypeId::of::<I>(),
        (
            |pane, workspace, id, state, cx| {
                I::from_state_proto(pane, workspace, id, state, cx).map(|task| {
                    cx.foreground_executor()
                        .spawn(async move { Ok(Box::new(task.await?) as Box<_>) })
                })
            },
            |this| Box::new(this.clone().downcast::<I>().unwrap()),
        ),
    );
}

#[derive(Default, Deref, DerefMut)]
struct ItemDeserializers(
    HashMap<
        Arc<str>,
        fn(
            Model<Project>,
            WeakView<Workspace>,
            WorkspaceId,
            ItemId,
            &mut ViewContext<Pane>,
        ) -> Task<Result<Box<dyn ItemHandle>>>,
    >,
);

impl Global for ItemDeserializers {}

pub fn register_deserializable_item<I: Item>(cx: &mut AppContext) {
    if let Some(serialized_item_kind) = I::serialized_item_kind() {
        let deserializers = cx.default_global::<ItemDeserializers>();
        deserializers.insert(
            Arc::from(serialized_item_kind),
            |project, workspace, workspace_id, item_id, cx| {
                let task = I::deserialize(project, workspace, workspace_id, item_id, cx);
                cx.foreground_executor()
                    .spawn(async { Ok(Box::new(task.await?) as Box<_>) })
            },
        );
    }
}

pub struct AppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Model<UserStore>,
    pub workspace_store: Model<WorkspaceStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub build_window_options: fn(Option<Uuid>, &mut AppContext) -> WindowOptions,
    pub node_runtime: Arc<dyn NodeRuntime>,
}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

pub struct WorkspaceStore {
    workspaces: HashSet<WindowHandle<Workspace>>,
    client: Arc<Client>,
    _subscriptions: Vec<client::Subscription>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Follower {
    project_id: Option<u64>,
    peer_id: PeerId,
}

impl AppState {
    pub fn global(cx: &AppContext) -> Weak<Self> {
        cx.global::<GlobalAppState>().0.clone()
    }
    pub fn try_global(cx: &AppContext) -> Option<Weak<Self>> {
        cx.try_global::<GlobalAppState>()
            .map(|state| state.0.clone())
    }
    pub fn set_global(state: Weak<AppState>, cx: &mut AppContext) {
        cx.set_global(GlobalAppState(state));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut AppContext) -> Arc<Self> {
        use node_runtime::FakeNodeRuntime;
        use settings::SettingsStore;
        use ui::Context as _;

        if !cx.has_global::<SettingsStore>() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }

        let fs = fs::FakeFs::new(cx.background_executor().clone());
        let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let clock = Arc::new(clock::FakeSystemClock::default());
        let http_client = http::FakeHttpClient::with_404_response();
        let client = Client::new(clock, http_client.clone(), cx);
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new_model(|cx| WorkspaceStore::new(client.clone(), cx));

        theme::init(theme::LoadThemes::JustBase, cx);
        client::init(&client, cx);
        crate::init_settings(cx);

        Arc::new(Self {
            client,
            fs,
            languages,
            user_store,
            workspace_store,
            node_runtime: FakeNodeRuntime::new(),
            build_window_options: |_, _| Default::default(),
        })
    }
}

struct DelayedDebouncedEditAction {
    task: Option<Task<()>>,
    cancel_channel: Option<oneshot::Sender<()>>,
}

impl DelayedDebouncedEditAction {
    fn new() -> DelayedDebouncedEditAction {
        DelayedDebouncedEditAction {
            task: None,
            cancel_channel: None,
        }
    }

    fn fire_new<F>(&mut self, delay: Duration, cx: &mut ViewContext<Workspace>, func: F)
    where
        F: 'static + Send + FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> Task<Result<()>>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            _ = channel.send(());
        }

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        let previous_task = self.task.take();
        self.task = Some(cx.spawn(move |workspace, mut cx| async move {
            let mut timer = cx.background_executor().timer(delay).fuse();
            if let Some(previous_task) = previous_task {
                previous_task.await;
            }

            futures::select_biased! {
                _ = receiver => return,
                    _ = timer => {}
            }

            if let Some(result) = workspace
                .update(&mut cx, |workspace, cx| (func)(workspace, cx))
                .log_err()
            {
                result.await.log_err();
            }
        }));
    }
}

pub enum Event {
    PaneAdded(View<Pane>),
    PaneRemoved,
    ItemAdded,
    ItemRemoved,
    ActiveItemChanged,
    ContactRequestedJoin(u64),
    WorkspaceCreated(WeakView<Workspace>),
    SpawnTask(SpawnInTerminal),
    OpenBundledFile {
        text: Cow<'static, str>,
        title: &'static str,
        language: &'static str,
    },
    ZoomChanged,
}

pub enum OpenVisible {
    All,
    None,
    OnlyFiles,
    OnlyDirectories,
}

type PromptForNewPath = Box<
    dyn Fn(&mut Workspace, &mut ViewContext<Workspace>) -> oneshot::Receiver<Option<ProjectPath>>,
>;

/// Collects everything project-related for a certain window opened.
/// In some way, is a counterpart of a window, as the [`WindowHandle`] could be downcast into `Workspace`.
///
/// A `Workspace` usually consists of 1 or more projects, a central pane group, 3 docks and a status bar.
/// The `Workspace` owns everybody's state and serves as a default, "global context",
/// that can be used to register a global action to be triggered from any place in the window.
pub struct Workspace {
    weak_self: WeakView<Self>,
    workspace_actions: Vec<Box<dyn Fn(Div, &mut ViewContext<Self>) -> Div>>,
    zoomed: Option<AnyWeakView>,
    zoomed_position: Option<DockPosition>,
    center: PaneGroup,
    left_dock: View<Dock>,
    bottom_dock: View<Dock>,
    right_dock: View<Dock>,
    panes: Vec<View<Pane>>,
    panes_by_item: HashMap<EntityId, WeakView<Pane>>,
    active_pane: View<Pane>,
    last_active_center_pane: Option<WeakView<Pane>>,
    last_active_view_id: Option<proto::ViewId>,
    status_bar: View<StatusBar>,
    modal_layer: View<ModalLayer>,
    titlebar_item: Option<AnyView>,
    notifications: Vec<(NotificationId, Box<dyn NotificationHandle>)>,
    project: Model<Project>,
    follower_states: HashMap<View<Pane>, FollowerState>,
    last_leaders_by_pane: HashMap<WeakView<Pane>, PeerId>,
    window_edited: bool,
    active_call: Option<(Model<ActiveCall>, Vec<Subscription>)>,
    leader_updates_tx: mpsc::UnboundedSender<(PeerId, proto::UpdateFollowers)>,
    database_id: Option<WorkspaceId>,
    app_state: Arc<AppState>,
    dispatching_keystrokes: Rc<RefCell<Vec<Keystroke>>>,
    _subscriptions: Vec<Subscription>,
    _apply_leader_updates: Task<Result<()>>,
    _observe_current_user: Task<Result<()>>,
    _schedule_serialize: Option<Task<()>>,
    pane_history_timestamp: Arc<AtomicUsize>,
    bounds: Bounds<Pixels>,
    centered_layout: bool,
    bounds_save_task_queued: Option<Task<()>>,
    on_prompt_for_new_path: Option<PromptForNewPath>,
}

impl EventEmitter<Event> for Workspace {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewId {
    pub creator: PeerId,
    pub id: u64,
}

#[derive(Default)]
struct FollowerState {
    leader_id: PeerId,
    active_view_id: Option<ViewId>,
    items_by_leader_view_id: HashMap<ViewId, Box<dyn FollowableItemHandle>>,
}

impl Workspace {
    const DEFAULT_PADDING: f32 = 0.2;
    const MAX_PADDING: f32 = 0.4;

    pub fn new(
        workspace_id: Option<WorkspaceId>,
        project: Model<Project>,
        app_state: Arc<AppState>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&project, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&project, move |this, _, event, cx| {
            match event {
                project::Event::RemoteIdChanged(_) => {
                    this.update_window_title(cx);
                }

                project::Event::CollaboratorLeft(peer_id) => {
                    this.collaborator_left(*peer_id, cx);
                }

                project::Event::WorktreeRemoved(_) | project::Event::WorktreeAdded => {
                    this.update_window_title(cx);
                    this.serialize_workspace(cx);
                }

                project::Event::DisconnectedFromHost => {
                    this.update_window_edited(cx);
                    let panes_to_unfollow: Vec<View<Pane>> =
                        this.follower_states.keys().map(|k| k.clone()).collect();
                    for pane in panes_to_unfollow {
                        this.unfollow(&pane, cx);
                    }
                    cx.disable_focus();
                }

                project::Event::Closed => {
                    cx.remove_window();
                }

                project::Event::DeletedEntry(entry_id) => {
                    for pane in this.panes.iter() {
                        pane.update(cx, |pane, cx| {
                            pane.handle_deleted_project_item(*entry_id, cx)
                        });
                    }
                }

                project::Event::Notification(message) => {
                    struct ProjectNotification;

                    this.show_notification(
                        NotificationId::unique::<ProjectNotification>(),
                        cx,
                        |cx| cx.new_view(|_| MessageNotification::new(message.clone())),
                    )
                }

                project::Event::LanguageServerPrompt(request) => {
                    struct LanguageServerPrompt;

                    let mut hasher = DefaultHasher::new();
                    request.lsp_name.as_str().hash(&mut hasher);
                    let id = hasher.finish();

                    this.show_notification(
                        NotificationId::identified::<LanguageServerPrompt>(id as usize),
                        cx,
                        |cx| {
                            cx.new_view(|_| {
                                notifications::LanguageServerPrompt::new(request.clone())
                            })
                        },
                    );
                }

                _ => {}
            }
            cx.notify()
        })
        .detach();

        cx.on_focus_lost(|this, cx| {
            let focus_handle = this.focus_handle(cx);
            cx.focus(&focus_handle);
        })
        .detach();

        let weak_handle = cx.view().downgrade();
        let pane_history_timestamp = Arc::new(AtomicUsize::new(0));

        let center_pane = cx.new_view(|cx| {
            Pane::new(
                weak_handle.clone(),
                project.clone(),
                pane_history_timestamp.clone(),
                None,
                NewFile.boxed_clone(),
                cx,
            )
        });
        cx.subscribe(&center_pane, Self::handle_pane_event).detach();

        cx.focus_view(&center_pane);
        cx.emit(Event::PaneAdded(center_pane.clone()));

        let window_handle = cx.window_handle().downcast::<Workspace>().unwrap();
        app_state.workspace_store.update(cx, |store, _| {
            store.workspaces.insert(window_handle);
        });

        let mut current_user = app_state.user_store.read(cx).watch_current_user();
        let mut connection_status = app_state.client.status();
        let _observe_current_user = cx.spawn(|this, mut cx| async move {
            current_user.next().await;
            connection_status.next().await;
            let mut stream =
                Stream::map(current_user, drop).merge(Stream::map(connection_status, drop));

            while stream.recv().await.is_some() {
                this.update(&mut cx, |_, cx| cx.notify())?;
            }
            anyhow::Ok(())
        });

        // All leader updates are enqueued and then processed in a single task, so
        // that each asynchronous operation can be run in order.
        let (leader_updates_tx, mut leader_updates_rx) =
            mpsc::unbounded::<(PeerId, proto::UpdateFollowers)>();
        let _apply_leader_updates = cx.spawn(|this, mut cx| async move {
            while let Some((leader_id, update)) = leader_updates_rx.next().await {
                Self::process_leader_update(&this, leader_id, update, &mut cx)
                    .await
                    .log_err();
            }

            Ok(())
        });

        cx.emit(Event::WorkspaceCreated(weak_handle.clone()));

        let left_dock = Dock::new(DockPosition::Left, cx);
        let bottom_dock = Dock::new(DockPosition::Bottom, cx);
        let right_dock = Dock::new(DockPosition::Right, cx);
        let left_dock_buttons = cx.new_view(|cx| PanelButtons::new(left_dock.clone(), cx));
        let bottom_dock_buttons = cx.new_view(|cx| PanelButtons::new(bottom_dock.clone(), cx));
        let right_dock_buttons = cx.new_view(|cx| PanelButtons::new(right_dock.clone(), cx));
        let status_bar = cx.new_view(|cx| {
            let mut status_bar = StatusBar::new(&center_pane.clone(), cx);
            status_bar.add_left_item(left_dock_buttons, cx);
            status_bar.add_right_item(right_dock_buttons, cx);
            status_bar.add_right_item(bottom_dock_buttons, cx);
            status_bar
        });

        let modal_layer = cx.new_view(|_| ModalLayer::new());

        let mut active_call = None;
        if let Some(call) = ActiveCall::try_global(cx) {
            let call = call.clone();
            let subscriptions = vec![cx.subscribe(&call, Self::on_active_call_event)];
            active_call = Some((call, subscriptions));
        }

        let subscriptions = vec![
            cx.observe_window_activation(Self::on_window_activation_changed),
            cx.observe_window_bounds(move |this, cx| {
                if this.bounds_save_task_queued.is_some() {
                    return;
                }
                this.bounds_save_task_queued = Some(cx.spawn(|this, mut cx| async move {
                    cx.background_executor()
                        .timer(Duration::from_millis(100))
                        .await;
                    this.update(&mut cx, |this, cx| {
                        if let Some(display) = cx.display() {
                            if let Some(display_uuid) = display.uuid().log_err() {
                                let window_bounds = cx.window_bounds();
                                if let Some(database_id) = workspace_id {
                                    cx.background_executor()
                                        .spawn(DB.set_window_open_status(
                                            database_id,
                                            SerializedWindowBounds(window_bounds),
                                            display_uuid,
                                        ))
                                        .detach_and_log_err(cx);
                                }
                            }
                        }
                        this.bounds_save_task_queued.take();
                    })
                    .ok();
                }));
                cx.notify();
            }),
            cx.observe_window_appearance(|_, cx| {
                let window_appearance = cx.appearance();

                *SystemAppearance::global_mut(cx) = SystemAppearance(window_appearance.into());

                ThemeSettings::reload_current_theme(cx);
            }),
            cx.observe(&left_dock, |this, _, cx| {
                this.serialize_workspace(cx);
                cx.notify();
            }),
            cx.observe(&bottom_dock, |this, _, cx| {
                this.serialize_workspace(cx);
                cx.notify();
            }),
            cx.observe(&right_dock, |this, _, cx| {
                this.serialize_workspace(cx);
                cx.notify();
            }),
            cx.on_release(|this, window, cx| {
                this.app_state.workspace_store.update(cx, |store, _| {
                    let window = window.downcast::<Self>().unwrap();
                    store.workspaces.remove(&window);
                })
            }),
        ];

        cx.defer(|this, cx| {
            this.update_window_title(cx);
        });
        Workspace {
            weak_self: weak_handle.clone(),
            zoomed: None,
            zoomed_position: None,
            center: PaneGroup::new(center_pane.clone()),
            panes: vec![center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane.clone(),
            last_active_center_pane: Some(center_pane.downgrade()),
            last_active_view_id: None,
            status_bar,
            modal_layer,
            titlebar_item: None,
            notifications: Default::default(),
            left_dock,
            bottom_dock,
            right_dock,
            project: project.clone(),
            follower_states: Default::default(),
            last_leaders_by_pane: Default::default(),
            dispatching_keystrokes: Default::default(),
            window_edited: false,
            active_call,
            database_id: workspace_id,
            app_state,
            _observe_current_user,
            _apply_leader_updates,
            _schedule_serialize: None,
            leader_updates_tx,
            _subscriptions: subscriptions,
            pane_history_timestamp,
            workspace_actions: Default::default(),
            // This data will be incorrect, but it will be overwritten by the time it needs to be used.
            bounds: Default::default(),
            centered_layout: false,
            bounds_save_task_queued: None,
            on_prompt_for_new_path: None,
        }
    }

    fn new_local(
        abs_paths: Vec<PathBuf>,
        app_state: Arc<AppState>,
        requesting_window: Option<WindowHandle<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<
        anyhow::Result<(
            WindowHandle<Workspace>,
            Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>,
        )>,
    > {
        let project_handle = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            cx,
        );

        cx.spawn(|mut cx| async move {
            let serialized_workspace: Option<SerializedWorkspace> =
                persistence::DB.workspace_for_roots(abs_paths.as_slice());

            let mut paths_to_open = abs_paths;

            let paths_order = serialized_workspace
                .as_ref()
                .map(|ws| &ws.location)
                .and_then(|loc| match loc {
                    SerializedWorkspaceLocation::Local(_, order) => Some(order.order()),
                    _ => None,
                });

            if let Some(paths_order) = paths_order {
                paths_to_open = paths_order
                    .iter()
                    .filter_map(|i| paths_to_open.get(*i).cloned())
                    .collect::<Vec<_>>();
                if paths_order.iter().enumerate().any(|(i, &j)| i != j) {
                    project_handle
                        .update(&mut cx, |project, _| {
                            project.set_worktrees_reordered(true);
                        })
                        .log_err();
                }
            }

            // Get project paths for all of the abs_paths
            let mut worktree_roots: HashSet<Arc<Path>> = Default::default();
            let mut project_paths: Vec<(PathBuf, Option<ProjectPath>)> =
                Vec::with_capacity(paths_to_open.len());
            for path in paths_to_open.into_iter() {
                if let Some((worktree, project_entry)) = cx
                    .update(|cx| {
                        Workspace::project_path_for_path(project_handle.clone(), &path, true, cx)
                    })?
                    .await
                    .log_err()
                {
                    worktree_roots.extend(worktree.update(&mut cx, |tree, _| tree.abs_path()).ok());
                    project_paths.push((path, Some(project_entry)));
                } else {
                    project_paths.push((path, None));
                }
            }

            let workspace_id = if let Some(serialized_workspace) = serialized_workspace.as_ref() {
                serialized_workspace.id
            } else {
                DB.next_id().await.unwrap_or_else(|_| Default::default())
            };

            let window = if let Some(window) = requesting_window {
                cx.update_window(window.into(), |_, cx| {
                    cx.replace_root_view(|cx| {
                        Workspace::new(
                            Some(workspace_id),
                            project_handle.clone(),
                            app_state.clone(),
                            cx,
                        )
                    });
                })?;
                window
            } else {
                let window_bounds_override = window_bounds_env_override();

                let (window_bounds, display) = if let Some(bounds) = window_bounds_override {
                    (Some(WindowBounds::Windowed(bounds)), None)
                } else {
                    let restorable_bounds = serialized_workspace
                        .as_ref()
                        .and_then(|workspace| Some((workspace.display?, workspace.window_bounds?)))
                        .or_else(|| {
                            let (display, window_bounds) = DB.last_window().log_err()?;
                            Some((display?, window_bounds?))
                        });

                    if let Some((serialized_display, serialized_status)) = restorable_bounds {
                        (Some(serialized_status.0), Some(serialized_display))
                    } else {
                        (None, None)
                    }
                };

                // Use the serialized workspace to construct the new window
                let mut options = cx.update(|cx| (app_state.build_window_options)(display, cx))?;
                options.window_bounds = window_bounds;
                let centered_layout = serialized_workspace
                    .as_ref()
                    .map(|w| w.centered_layout)
                    .unwrap_or(false);
                cx.open_window(options, {
                    let app_state = app_state.clone();
                    let project_handle = project_handle.clone();
                    move |cx| {
                        cx.new_view(|cx| {
                            let mut workspace =
                                Workspace::new(Some(workspace_id), project_handle, app_state, cx);
                            workspace.centered_layout = centered_layout;
                            workspace
                        })
                    }
                })?
            };

            notify_if_database_failed(window, &mut cx);
            let opened_items = window
                .update(&mut cx, |_workspace, cx| {
                    open_items(serialized_workspace, project_paths, app_state, cx)
                })?
                .await
                .unwrap_or_default();

            window
                .update(&mut cx, |_, cx| cx.activate_window())
                .log_err();
            Ok((window, opened_items))
        })
    }

    pub fn weak_handle(&self) -> WeakView<Self> {
        self.weak_self.clone()
    }

    pub fn left_dock(&self) -> &View<Dock> {
        &self.left_dock
    }

    pub fn bottom_dock(&self) -> &View<Dock> {
        &self.bottom_dock
    }

    pub fn right_dock(&self) -> &View<Dock> {
        &self.right_dock
    }

    pub fn is_edited(&self) -> bool {
        self.window_edited
    }

    pub fn add_panel<T: Panel>(&mut self, panel: View<T>, cx: &mut WindowContext) {
        let dock = match panel.position(cx) {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        };

        dock.update(cx, |dock, cx| {
            dock.add_panel(panel, self.weak_self.clone(), cx)
        });
    }

    pub fn status_bar(&self) -> &View<StatusBar> {
        &self.status_bar
    }

    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    pub fn user_store(&self) -> &Model<UserStore> {
        &self.app_state.user_store
    }

    pub fn project(&self) -> &Model<Project> {
        &self.project
    }

    pub fn recent_navigation_history(
        &self,
        limit: Option<usize>,
        cx: &AppContext,
    ) -> Vec<(ProjectPath, Option<PathBuf>)> {
        let mut abs_paths_opened: HashMap<PathBuf, HashSet<ProjectPath>> = HashMap::default();
        let mut history: HashMap<ProjectPath, (Option<PathBuf>, usize)> = HashMap::default();
        for pane in &self.panes {
            let pane = pane.read(cx);
            pane.nav_history()
                .for_each_entry(cx, |entry, (project_path, fs_path)| {
                    if let Some(fs_path) = &fs_path {
                        abs_paths_opened
                            .entry(fs_path.clone())
                            .or_default()
                            .insert(project_path.clone());
                    }
                    let timestamp = entry.timestamp;
                    match history.entry(project_path) {
                        hash_map::Entry::Occupied(mut entry) => {
                            let (_, old_timestamp) = entry.get();
                            if &timestamp > old_timestamp {
                                entry.insert((fs_path, timestamp));
                            }
                        }
                        hash_map::Entry::Vacant(entry) => {
                            entry.insert((fs_path, timestamp));
                        }
                    }
                });
        }

        history
            .into_iter()
            .sorted_by_key(|(_, (_, timestamp))| *timestamp)
            .map(|(project_path, (fs_path, _))| (project_path, fs_path))
            .rev()
            .filter(|(history_path, abs_path)| {
                let latest_project_path_opened = abs_path
                    .as_ref()
                    .and_then(|abs_path| abs_paths_opened.get(abs_path))
                    .and_then(|project_paths| {
                        project_paths
                            .iter()
                            .max_by(|b1, b2| b1.worktree_id.cmp(&b2.worktree_id))
                    });

                match latest_project_path_opened {
                    Some(latest_project_path_opened) => latest_project_path_opened == history_path,
                    None => true,
                }
            })
            .take(limit.unwrap_or(usize::MAX))
            .collect()
    }

    fn navigate_history(
        &mut self,
        pane: WeakView<Pane>,
        mode: NavigationMode,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<()>> {
        let to_load = if let Some(pane) = pane.upgrade() {
            pane.update(cx, |pane, cx| {
                pane.focus(cx);
                loop {
                    // Retrieve the weak item handle from the history.
                    let entry = pane.nav_history_mut().pop(mode, cx)?;

                    // If the item is still present in this pane, then activate it.
                    if let Some(index) = entry
                        .item
                        .upgrade()
                        .and_then(|v| pane.index_for_item(v.as_ref()))
                    {
                        let prev_active_item_index = pane.active_item_index();
                        pane.nav_history_mut().set_mode(mode);
                        pane.activate_item(index, true, true, cx);
                        pane.nav_history_mut().set_mode(NavigationMode::Normal);

                        let mut navigated = prev_active_item_index != pane.active_item_index();
                        if let Some(data) = entry.data {
                            navigated |= pane.active_item()?.navigate(data, cx);
                        }

                        if navigated {
                            break None;
                        }
                    }
                    // If the item is no longer present in this pane, then retrieve its
                    // project path in order to reopen it.
                    else {
                        break pane
                            .nav_history()
                            .path_for_item(entry.item.id())
                            .map(|(project_path, _)| (project_path, entry));
                    }
                }
            })
        } else {
            None
        };

        if let Some((project_path, entry)) = to_load {
            // If the item was no longer present, then load it again from its previous path.
            let task = self.load_path(project_path, cx);
            cx.spawn(|workspace, mut cx| async move {
                let task = task.await;
                let mut navigated = false;
                if let Some((project_entry_id, build_item)) = task.log_err() {
                    let prev_active_item_id = pane.update(&mut cx, |pane, _| {
                        pane.nav_history_mut().set_mode(mode);
                        pane.active_item().map(|p| p.item_id())
                    })?;

                    pane.update(&mut cx, |pane, cx| {
                        let item = pane.open_item(
                            project_entry_id,
                            true,
                            entry.is_preview,
                            cx,
                            build_item,
                        );
                        navigated |= Some(item.item_id()) != prev_active_item_id;
                        pane.nav_history_mut().set_mode(NavigationMode::Normal);
                        if let Some(data) = entry.data {
                            navigated |= item.navigate(data, cx);
                        }
                    })?;
                }

                if !navigated {
                    workspace
                        .update(&mut cx, |workspace, cx| {
                            Self::navigate_history(workspace, pane, mode, cx)
                        })?
                        .await?;
                }

                Ok(())
            })
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn go_back(
        &mut self,
        pane: WeakView<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<()>> {
        self.navigate_history(pane, NavigationMode::GoingBack, cx)
    }

    pub fn go_forward(
        &mut self,
        pane: WeakView<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<()>> {
        self.navigate_history(pane, NavigationMode::GoingForward, cx)
    }

    pub fn reopen_closed_item(&mut self, cx: &mut ViewContext<Workspace>) -> Task<Result<()>> {
        self.navigate_history(
            self.active_pane().downgrade(),
            NavigationMode::ReopeningClosedItem,
            cx,
        )
    }

    pub fn client(&self) -> &Arc<Client> {
        &self.app_state.client
    }

    pub fn set_titlebar_item(&mut self, item: AnyView, cx: &mut ViewContext<Self>) {
        self.titlebar_item = Some(item);
        cx.notify();
    }

    pub fn set_prompt_for_new_path(&mut self, prompt: PromptForNewPath) {
        self.on_prompt_for_new_path = Some(prompt)
    }

    pub fn prompt_for_new_path(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> oneshot::Receiver<Option<ProjectPath>> {
        if let Some(prompt) = self.on_prompt_for_new_path.take() {
            let rx = prompt(self, cx);
            self.on_prompt_for_new_path = Some(prompt);
            rx
        } else {
            let start_abs_path = self
                .project
                .update(cx, |project, cx| {
                    let worktree = project.visible_worktrees(cx).next()?;
                    Some(worktree.read(cx).as_local()?.abs_path().to_path_buf())
                })
                .unwrap_or_else(|| Path::new("").into());

            let (tx, rx) = oneshot::channel();
            let abs_path = cx.prompt_for_new_path(&start_abs_path);
            cx.spawn(|this, mut cx| async move {
                let abs_path = abs_path.await?;
                let project_path = abs_path.and_then(|abs_path| {
                    this.update(&mut cx, |this, cx| {
                        this.project.update(cx, |project, cx| {
                            project.find_or_create_local_worktree(abs_path, true, cx)
                        })
                    })
                    .ok()
                });

                if let Some(project_path) = project_path {
                    let (worktree, path) = project_path.await?;
                    let worktree_id = worktree.read_with(&cx, |worktree, _| worktree.id())?;
                    tx.send(Some(ProjectPath {
                        worktree_id,
                        path: path.into(),
                    }))
                    .ok();
                } else {
                    tx.send(None).ok();
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            rx
        }
    }

    pub fn titlebar_item(&self) -> Option<AnyView> {
        self.titlebar_item.clone()
    }

    /// Call the given callback with a workspace whose project is local.
    ///
    /// If the given workspace has a local project, then it will be passed
    /// to the callback. Otherwise, a new empty window will be created.
    pub fn with_local_workspace<T, F>(
        &mut self,
        cx: &mut ViewContext<Self>,
        callback: F,
    ) -> Task<Result<T>>
    where
        T: 'static,
        F: 'static + FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
    {
        if self.project.read(cx).is_local() {
            Task::Ready(Some(Ok(callback(self, cx))))
        } else {
            let task = Self::new_local(Vec::new(), self.app_state.clone(), None, cx);
            cx.spawn(|_vh, mut cx| async move {
                let (workspace, _) = task.await?;
                workspace.update(&mut cx, callback)
            })
        }
    }

    pub fn worktrees<'a>(&self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Model<Worktree>> {
        self.project.read(cx).worktrees()
    }

    pub fn visible_worktrees<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = Model<Worktree>> {
        self.project.read(cx).visible_worktrees(cx)
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

    pub fn close_global(_: &CloseWindow, cx: &mut AppContext) {
        cx.defer(|cx| {
            cx.windows().iter().find(|window| {
                window
                    .update(cx, |_, window| {
                        if window.is_window_active() {
                            //This can only get called when the window's project connection has been lost
                            //so we don't need to prompt the user for anything and instead just close the window
                            window.remove_window();
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            });
        });
    }

    pub fn close_window(&mut self, _: &CloseWindow, cx: &mut ViewContext<Self>) {
        let window = cx.window_handle();
        let prepare = self.prepare_to_close(false, cx);
        cx.spawn(|_, mut cx| async move {
            if prepare.await? {
                window.update(&mut cx, |_, cx| {
                    cx.remove_window();
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
    }

    pub fn prepare_to_close(
        &mut self,
        quitting: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        let active_call = self.active_call().cloned();
        let window = cx.window_handle();

        cx.spawn(|this, mut cx| async move {
            let workspace_count = (*cx).update(|cx| {
                cx.windows()
                    .iter()
                    .filter(|window| window.downcast::<Workspace>().is_some())
                    .count()
            })?;

            if let Some(active_call) = active_call {
                if !quitting
                    && workspace_count == 1
                    && active_call.read_with(&cx, |call, _| call.room().is_some())?
                {
                    let answer = window.update(&mut cx, |_, cx| {
                        cx.prompt(
                            PromptLevel::Warning,
                            "Do you want to leave the current call?",
                            None,
                            &["Close window and hang up", "Cancel"],
                        )
                    })?;

                    if answer.await.log_err() == Some(1) {
                        return anyhow::Ok(false);
                    } else {
                        active_call
                            .update(&mut cx, |call, cx| call.hang_up(cx))?
                            .await
                            .log_err();
                    }
                }
            }

            this.update(&mut cx, |this, cx| {
                this.save_all_internal(SaveIntent::Close, cx)
            })?
            .await
        })
    }

    fn save_all(&mut self, action: &SaveAll, cx: &mut ViewContext<Self>) {
        self.save_all_internal(action.save_intent.unwrap_or(SaveIntent::SaveAll), cx)
            .detach_and_log_err(cx);
    }

    fn send_keystrokes(&mut self, action: &SendKeystrokes, cx: &mut ViewContext<Self>) {
        let mut keystrokes: Vec<Keystroke> = action
            .0
            .split(' ')
            .flat_map(|k| Keystroke::parse(k).log_err())
            .collect();
        keystrokes.reverse();

        self.dispatching_keystrokes
            .borrow_mut()
            .append(&mut keystrokes);

        let keystrokes = self.dispatching_keystrokes.clone();
        cx.window_context()
            .spawn(|mut cx| async move {
                // limit to 100 keystrokes to avoid infinite recursion.
                for _ in 0..100 {
                    let Some(keystroke) = keystrokes.borrow_mut().pop() else {
                        return Ok(());
                    };
                    cx.update(|cx| {
                        let focused = cx.focused();
                        cx.dispatch_keystroke(keystroke.clone());
                        if cx.focused() != focused {
                            // dispatch_keystroke may cause the focus to change.
                            // draw's side effect is to schedule the FocusChanged events in the current flush effect cycle
                            // And we need that to happen before the next keystroke to keep vim mode happy...
                            // (Note that the tests always do this implicitly, so you must manually test with something like:
                            //   "bindings": { "g z": ["workspace::SendKeystrokes", ": j <enter> u"]}
                            // )
                            cx.draw();
                        }
                    })?;
                }
                keystrokes.borrow_mut().clear();
                Err(anyhow!("over 100 keystrokes passed to send_keystrokes"))
            })
            .detach_and_log_err(cx);
    }

    fn save_all_internal(
        &mut self,
        mut save_intent: SaveIntent,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        if self.project.read(cx).is_disconnected() {
            return Task::ready(Ok(true));
        }
        let dirty_items = self
            .panes
            .iter()
            .flat_map(|pane| {
                pane.read(cx).items().filter_map(|item| {
                    if item.is_dirty(cx) {
                        Some((pane.downgrade(), item.boxed_clone()))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let project = self.project.clone();
        cx.spawn(|workspace, mut cx| async move {
            // Override save mode and display "Save all files" prompt
            if save_intent == SaveIntent::Close && dirty_items.len() > 1 {
                let answer = workspace.update(&mut cx, |_, cx| {
                    let (prompt, detail) = Pane::file_names_for_prompt(
                        &mut dirty_items.iter().map(|(_, handle)| handle),
                        dirty_items.len(),
                        cx,
                    );
                    cx.prompt(
                        PromptLevel::Warning,
                        &prompt,
                        Some(&detail),
                        &["Save all", "Discard all", "Cancel"],
                    )
                })?;
                match answer.await.log_err() {
                    Some(0) => save_intent = SaveIntent::SaveAll,
                    Some(1) => save_intent = SaveIntent::Skip,
                    _ => {}
                }
            }
            for (pane, item) in dirty_items {
                let (singleton, project_entry_ids) =
                    cx.update(|cx| (item.is_singleton(cx), item.project_entry_ids(cx)))?;
                if singleton || !project_entry_ids.is_empty() {
                    if let Some(ix) =
                        pane.update(&mut cx, |pane, _| pane.index_for_item(item.as_ref()))?
                    {
                        if !Pane::save_item(
                            project.clone(),
                            &pane,
                            ix,
                            &*item,
                            save_intent,
                            &mut cx,
                        )
                        .await?
                        {
                            return Ok(false);
                        }
                    }
                }
            }
            Ok(true)
        })
    }

    pub fn open(&mut self, _: &Open, cx: &mut ViewContext<Self>) {
        self.client()
            .telemetry()
            .report_app_event("open project".to_string());
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        });

        cx.spawn(|this, mut cx| async move {
            let Some(paths) = paths.await.log_err().flatten() else {
                return;
            };

            if let Some(task) = this
                .update(&mut cx, |this, cx| {
                    this.open_workspace_for_paths(false, paths, cx)
                })
                .log_err()
            {
                task.await.log_err();
            }
        })
        .detach()
    }

    pub fn open_workspace_for_paths(
        &mut self,
        replace_current_window: bool,
        paths: Vec<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let window = cx.window_handle().downcast::<Self>();
        let is_remote = self.project.read(cx).is_remote();
        let has_worktree = self.project.read(cx).worktrees().next().is_some();
        let has_dirty_items = self.items(cx).any(|item| item.is_dirty(cx));

        let window_to_replace = if replace_current_window {
            window
        } else if is_remote || has_worktree || has_dirty_items {
            None
        } else {
            window
        };
        let app_state = self.app_state.clone();

        cx.spawn(|_, mut cx| async move {
            cx.update(|cx| {
                open_paths(
                    &paths,
                    app_state,
                    OpenOptions {
                        replace_window: window_to_replace,
                        ..Default::default()
                    },
                    cx,
                )
            })?
            .await?;
            Ok(())
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn open_paths(
        &mut self,
        mut abs_paths: Vec<PathBuf>,
        visible: OpenVisible,
        pane: Option<WeakView<Pane>>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>> {
        log::info!("open paths {abs_paths:?}");

        let fs = self.app_state.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn(move |this, mut cx| async move {
            let mut tasks = Vec::with_capacity(abs_paths.len());

            for abs_path in &abs_paths {
                let visible = match visible {
                    OpenVisible::All => Some(true),
                    OpenVisible::None => Some(false),
                    OpenVisible::OnlyFiles => match fs.metadata(abs_path).await.log_err() {
                        Some(Some(metadata)) => Some(!metadata.is_dir),
                        Some(None) => Some(true),
                        None => None,
                    },
                    OpenVisible::OnlyDirectories => match fs.metadata(abs_path).await.log_err() {
                        Some(Some(metadata)) => Some(metadata.is_dir),
                        Some(None) => Some(false),
                        None => None,
                    },
                };
                let project_path = match visible {
                    Some(visible) => match this
                        .update(&mut cx, |this, cx| {
                            Workspace::project_path_for_path(
                                this.project.clone(),
                                abs_path,
                                visible,
                                cx,
                            )
                        })
                        .log_err()
                    {
                        Some(project_path) => project_path.await.log_err(),
                        None => None,
                    },
                    None => None,
                };

                let this = this.clone();
                let abs_path = abs_path.clone();
                let fs = fs.clone();
                let pane = pane.clone();
                let task = cx.spawn(move |mut cx| async move {
                    let (worktree, project_path) = project_path?;
                    if fs.is_dir(&abs_path).await {
                        this.update(&mut cx, |workspace, cx| {
                            let worktree = worktree.read(cx);
                            let worktree_abs_path = worktree.abs_path();
                            let entry_id = if abs_path == worktree_abs_path.as_ref() {
                                worktree.root_entry()
                            } else {
                                abs_path
                                    .strip_prefix(worktree_abs_path.as_ref())
                                    .ok()
                                    .and_then(|relative_path| {
                                        worktree.entry_for_path(relative_path)
                                    })
                            }
                            .map(|entry| entry.id);
                            if let Some(entry_id) = entry_id {
                                workspace.project.update(cx, |_, cx| {
                                    cx.emit(project::Event::ActiveEntryChanged(Some(entry_id)));
                                })
                            }
                        })
                        .log_err()?;
                        None
                    } else {
                        Some(
                            this.update(&mut cx, |this, cx| {
                                this.open_path(project_path, pane, true, cx)
                            })
                            .log_err()?
                            .await,
                        )
                    }
                });
                tasks.push(task);
            }

            futures::future::join_all(tasks).await
        })
    }

    fn add_folder_to_project(&mut self, _: &AddFolderToProject, cx: &mut ViewContext<Self>) {
        if self.project.read(cx).is_remote() {
            self.show_error(
                &anyhow!("Folders cannot yet be added to remote projects"),
                cx,
            );
            return;
        }
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: true,
        });
        cx.spawn(|this, mut cx| async move {
            if let Some(paths) = paths.await.log_err().flatten() {
                let results = this
                    .update(&mut cx, |this, cx| {
                        this.open_paths(paths, OpenVisible::All, None, cx)
                    })?
                    .await;
                for result in results.into_iter().flatten() {
                    result.log_err();
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn project_path_for_path(
        project: Model<Project>,
        abs_path: &Path,
        visible: bool,
        cx: &mut AppContext,
    ) -> Task<Result<(Model<Worktree>, ProjectPath)>> {
        let entry = project.update(cx, |project, cx| {
            project.find_or_create_local_worktree(abs_path, visible, cx)
        });
        cx.spawn(|mut cx| async move {
            let (worktree, path) = entry.await?;
            let worktree_id = worktree.update(&mut cx, |t, _| t.id())?;
            Ok((
                worktree,
                ProjectPath {
                    worktree_id,
                    path: path.into(),
                },
            ))
        })
    }

    pub fn items<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = &Box<dyn ItemHandle>> {
        self.panes.iter().flat_map(|pane| pane.read(cx).items())
    }

    pub fn item_of_type<T: Item>(&self, cx: &AppContext) -> Option<View<T>> {
        self.items_of_type(cx).max_by_key(|item| item.item_id())
    }

    pub fn items_of_type<'a, T: Item>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = View<T>> {
        self.panes
            .iter()
            .flat_map(|pane| pane.read(cx).items_of_type())
    }

    pub fn active_item(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.active_pane().read(cx).active_item()
    }

    pub fn active_item_as<I: 'static>(&self, cx: &AppContext) -> Option<View<I>> {
        let item = self.active_item(cx)?;
        item.to_any().downcast::<I>().ok()
    }

    fn active_project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.active_item(cx).and_then(|item| item.project_path(cx))
    }

    pub fn save_active_item(
        &mut self,
        save_intent: SaveIntent,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        let project = self.project.clone();
        let pane = self.active_pane();
        let item_ix = pane.read(cx).active_item_index();
        let item = pane.read(cx).active_item();
        let pane = pane.downgrade();

        cx.spawn(|mut cx| async move {
            if let Some(item) = item {
                Pane::save_item(project, &pane, item_ix, item.as_ref(), save_intent, &mut cx)
                    .await
                    .map(|_| ())
            } else {
                Ok(())
            }
        })
    }

    pub fn close_inactive_items_and_panes(
        &mut self,
        action: &CloseInactiveTabsAndPanes,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(task) =
            self.close_all_internal(true, action.save_intent.unwrap_or(SaveIntent::Close), cx)
        {
            task.detach_and_log_err(cx)
        }
    }

    pub fn close_all_items_and_panes(
        &mut self,
        action: &CloseAllItemsAndPanes,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(task) =
            self.close_all_internal(false, action.save_intent.unwrap_or(SaveIntent::Close), cx)
        {
            task.detach_and_log_err(cx)
        }
    }

    fn close_all_internal(
        &mut self,
        retain_active_pane: bool,
        save_intent: SaveIntent,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let current_pane = self.active_pane();

        let mut tasks = Vec::new();

        if retain_active_pane {
            if let Some(current_pane_close) = current_pane.update(cx, |pane, cx| {
                pane.close_inactive_items(&CloseInactiveItems { save_intent: None }, cx)
            }) {
                tasks.push(current_pane_close);
            };
        }

        for pane in self.panes() {
            if retain_active_pane && pane.entity_id() == current_pane.entity_id() {
                continue;
            }

            if let Some(close_pane_items) = pane.update(cx, |pane: &mut Pane, cx| {
                pane.close_all_items(
                    &CloseAllItems {
                        save_intent: Some(save_intent),
                    },
                    cx,
                )
            }) {
                tasks.push(close_pane_items)
            }
        }

        if tasks.is_empty() {
            None
        } else {
            Some(cx.spawn(|_, _| async move {
                for task in tasks {
                    task.await?
                }
                Ok(())
            }))
        }
    }

    pub fn toggle_dock(&mut self, dock_side: DockPosition, cx: &mut ViewContext<Self>) {
        let dock = match dock_side {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        };
        let mut focus_center = false;
        let mut reveal_dock = false;
        dock.update(cx, |dock, cx| {
            let other_is_zoomed = self.zoomed.is_some() && self.zoomed_position != Some(dock_side);
            let was_visible = dock.is_open() && !other_is_zoomed;
            dock.set_open(!was_visible, cx);

            if let Some(active_panel) = dock.active_panel() {
                if was_visible {
                    if active_panel.focus_handle(cx).contains_focused(cx) {
                        focus_center = true;
                    }
                } else {
                    let focus_handle = &active_panel.focus_handle(cx);
                    cx.focus(focus_handle);
                    reveal_dock = true;
                }
            }
        });

        if reveal_dock {
            self.dismiss_zoomed_items_to_reveal(Some(dock_side), cx);
        }

        if focus_center {
            self.active_pane.update(cx, |pane, cx| pane.focus(cx))
        }

        cx.notify();
        self.serialize_workspace(cx);
    }

    pub fn close_all_docks(&mut self, cx: &mut ViewContext<Self>) {
        let docks = [&self.left_dock, &self.bottom_dock, &self.right_dock];

        for dock in docks {
            dock.update(cx, |dock, cx| {
                dock.set_open(false, cx);
            });
        }

        cx.focus_self();
        cx.notify();
        self.serialize_workspace(cx);
    }

    /// Transfer focus to the panel of the given type.
    pub fn focus_panel<T: Panel>(&mut self, cx: &mut ViewContext<Self>) -> Option<View<T>> {
        let panel = self.focus_or_unfocus_panel::<T>(cx, |_, _| true)?;
        panel.to_any().downcast().ok()
    }

    /// Focus the panel of the given type if it isn't already focused. If it is
    /// already focused, then transfer focus back to the workspace center.
    pub fn toggle_panel_focus<T: Panel>(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_or_unfocus_panel::<T>(cx, |panel, cx| {
            !panel.focus_handle(cx).contains_focused(cx)
        });
    }

    /// Focus or unfocus the given panel type, depending on the given callback.
    fn focus_or_unfocus_panel<T: Panel>(
        &mut self,
        cx: &mut ViewContext<Self>,
        should_focus: impl Fn(&dyn PanelHandle, &mut ViewContext<Dock>) -> bool,
    ) -> Option<Arc<dyn PanelHandle>> {
        let mut result_panel = None;
        let mut serialize = false;
        for dock in [&self.left_dock, &self.bottom_dock, &self.right_dock] {
            if let Some(panel_index) = dock.read(cx).panel_index_for_type::<T>() {
                let mut focus_center = false;
                let panel = dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, cx);

                    let panel = dock.active_panel().cloned();
                    if let Some(panel) = panel.as_ref() {
                        if should_focus(&**panel, cx) {
                            dock.set_open(true, cx);
                            panel.focus_handle(cx).focus(cx);
                        } else {
                            focus_center = true;
                        }
                    }
                    panel
                });

                if focus_center {
                    self.active_pane.update(cx, |pane, cx| pane.focus(cx))
                }

                result_panel = panel;
                serialize = true;
                break;
            }
        }

        if serialize {
            self.serialize_workspace(cx);
        }

        cx.notify();
        result_panel
    }

    /// Open the panel of the given type
    pub fn open_panel<T: Panel>(&mut self, cx: &mut ViewContext<Self>) {
        for dock in [&self.left_dock, &self.bottom_dock, &self.right_dock] {
            if let Some(panel_index) = dock.read(cx).panel_index_for_type::<T>() {
                dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, cx);
                    dock.set_open(true, cx);
                });
            }
        }
    }

    pub fn panel<T: Panel>(&self, cx: &WindowContext) -> Option<View<T>> {
        for dock in [&self.left_dock, &self.bottom_dock, &self.right_dock] {
            let dock = dock.read(cx);
            if let Some(panel) = dock.panel::<T>() {
                return Some(panel);
            }
        }
        None
    }

    fn dismiss_zoomed_items_to_reveal(
        &mut self,
        dock_to_reveal: Option<DockPosition>,
        cx: &mut ViewContext<Self>,
    ) {
        // If a center pane is zoomed, unzoom it.
        for pane in &self.panes {
            if pane != &self.active_pane || dock_to_reveal.is_some() {
                pane.update(cx, |pane, cx| pane.set_zoomed(false, cx));
            }
        }

        // If another dock is zoomed, hide it.
        let mut focus_center = false;
        for dock in [&self.left_dock, &self.right_dock, &self.bottom_dock] {
            dock.update(cx, |dock, cx| {
                if Some(dock.position()) != dock_to_reveal {
                    if let Some(panel) = dock.active_panel() {
                        if panel.is_zoomed(cx) {
                            focus_center |= panel.focus_handle(cx).contains_focused(cx);
                            dock.set_open(false, cx);
                        }
                    }
                }
            });
        }

        if focus_center {
            self.active_pane.update(cx, |pane, cx| pane.focus(cx))
        }

        if self.zoomed_position != dock_to_reveal {
            self.zoomed = None;
            self.zoomed_position = None;
            cx.emit(Event::ZoomChanged);
        }

        cx.notify();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> View<Pane> {
        let pane = cx.new_view(|cx| {
            Pane::new(
                self.weak_handle(),
                self.project.clone(),
                self.pane_history_timestamp.clone(),
                None,
                NewFile.boxed_clone(),
                cx,
            )
        });
        cx.subscribe(&pane, Self::handle_pane_event).detach();
        self.panes.push(pane.clone());
        cx.focus_view(&pane);
        cx.emit(Event::PaneAdded(pane.clone()));
        pane
    }

    pub fn add_item_to_center(
        &mut self,
        item: Box<dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        if let Some(center_pane) = self.last_active_center_pane.clone() {
            if let Some(center_pane) = center_pane.upgrade() {
                center_pane.update(cx, |pane, cx| pane.add_item(item, true, true, None, cx));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn add_item_to_active_pane(
        &mut self,
        item: Box<dyn ItemHandle>,
        destination_index: Option<usize>,
        cx: &mut WindowContext,
    ) {
        self.add_item(self.active_pane.clone(), item, destination_index, cx)
    }

    pub fn add_item(
        &mut self,
        pane: View<Pane>,
        item: Box<dyn ItemHandle>,
        destination_index: Option<usize>,
        cx: &mut WindowContext,
    ) {
        if let Some(text) = item.telemetry_event_text(cx) {
            self.client()
                .telemetry()
                .report_app_event(format!("{}: open", text));
        }

        pane.update(cx, |pane, cx| {
            pane.add_item(item, true, true, destination_index, cx)
        });
    }

    pub fn split_item(
        &mut self,
        split_direction: SplitDirection,
        item: Box<dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        let new_pane = self.split_pane(self.active_pane.clone(), split_direction, cx);
        self.add_item(new_pane, item, None, cx);
    }

    pub fn open_abs_path(
        &mut self,
        abs_path: PathBuf,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        cx.spawn(|workspace, mut cx| async move {
            let open_paths_task_result = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_paths(
                        vec![abs_path.clone()],
                        if visible {
                            OpenVisible::All
                        } else {
                            OpenVisible::None
                        },
                        None,
                        cx,
                    )
                })
                .with_context(|| format!("open abs path {abs_path:?} task spawn"))?
                .await;
            anyhow::ensure!(
                open_paths_task_result.len() == 1,
                "open abs path {abs_path:?} task returned incorrect number of results"
            );
            match open_paths_task_result
                .into_iter()
                .next()
                .expect("ensured single task result")
            {
                Some(open_result) => {
                    open_result.with_context(|| format!("open abs path {abs_path:?} task join"))
                }
                None => anyhow::bail!("open abs path {abs_path:?} task returned None"),
            }
        })
    }

    pub fn split_abs_path(
        &mut self,
        abs_path: PathBuf,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        let project_path_task =
            Workspace::project_path_for_path(self.project.clone(), &abs_path, visible, cx);
        cx.spawn(|this, mut cx| async move {
            let (_, path) = project_path_task.await?;
            this.update(&mut cx, |this, cx| this.split_path(path, cx))?
                .await
        })
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakView<Pane>>,
        focus_item: bool,
        cx: &mut WindowContext,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        self.open_path_preview(path, pane, focus_item, false, cx)
    }

    pub fn open_path_preview(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakView<Pane>>,
        focus_item: bool,
        allow_preview: bool,
        cx: &mut WindowContext,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        let pane = pane.unwrap_or_else(|| {
            self.last_active_center_pane.clone().unwrap_or_else(|| {
                self.panes
                    .first()
                    .expect("There must be an active pane")
                    .downgrade()
            })
        });

        let task = self.load_path(path.into(), cx);
        cx.spawn(move |mut cx| async move {
            let (project_entry_id, build_item) = task.await?;
            pane.update(&mut cx, |pane, cx| {
                pane.open_item(project_entry_id, focus_item, allow_preview, cx, build_item)
            })
        })
    }

    pub fn split_path(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        self.split_path_preview(path, false, cx)
    }

    pub fn split_path_preview(
        &mut self,
        path: impl Into<ProjectPath>,
        allow_preview: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        let pane = self.last_active_center_pane.clone().unwrap_or_else(|| {
            self.panes
                .first()
                .expect("There must be an active pane")
                .downgrade()
        });

        if let Member::Pane(center_pane) = &self.center.root {
            if center_pane.read(cx).items_len() == 0 {
                return self.open_path(path, Some(pane), true, cx);
            }
        }

        let task = self.load_path(path.into(), cx);
        cx.spawn(|this, mut cx| async move {
            let (project_entry_id, build_item) = task.await?;
            this.update(&mut cx, move |this, cx| -> Option<_> {
                let pane = pane.upgrade()?;
                let new_pane = this.split_pane(pane, SplitDirection::Right, cx);
                new_pane.update(cx, |new_pane, cx| {
                    Some(new_pane.open_item(project_entry_id, true, allow_preview, cx, build_item))
                })
            })
            .map(|option| option.ok_or_else(|| anyhow!("pane was dropped")))?
        })
    }

    fn load_path(
        &mut self,
        path: ProjectPath,
        cx: &mut WindowContext,
    ) -> Task<Result<(Option<ProjectEntryId>, WorkspaceItemBuilder)>> {
        let project = self.project().clone();
        let project_item_builders = cx.default_global::<ProjectItemOpeners>().clone();
        let Some(open_project_item) = project_item_builders
            .iter()
            .rev()
            .find_map(|open_project_item| open_project_item(&project, &path, cx))
        else {
            return Task::ready(Err(anyhow!("cannot open file {:?}", path.path)));
        };
        open_project_item
    }

    pub fn open_project_item<T>(
        &mut self,
        pane: View<Pane>,
        project_item: Model<T::Item>,
        cx: &mut ViewContext<Self>,
    ) -> View<T>
    where
        T: ProjectItem,
    {
        use project::Item as _;

        let entry_id = project_item.read(cx).entry_id(cx);
        if let Some(item) = entry_id
            .and_then(|entry_id| pane.read(cx).item_for_entry(entry_id, cx))
            .and_then(|item| item.downcast())
        {
            self.activate_item(&item, cx);
            return item;
        }

        let item = cx.new_view(|cx| T::for_project_item(self.project().clone(), project_item, cx));

        let item_id = item.item_id();
        let mut destination_index = None;
        pane.update(cx, |pane, cx| {
            if PreviewTabsSettings::get_global(cx).enable_preview_from_code_navigation {
                if let Some(preview_item_id) = pane.preview_item_id() {
                    if preview_item_id != item_id {
                        destination_index = pane.close_current_preview_item(cx);
                    }
                }
            }
            pane.set_preview_item_id(Some(item.item_id()), cx)
        });

        self.add_item(pane, Box::new(item.clone()), destination_index, cx);
        item
    }

    pub fn open_shared_screen(&mut self, peer_id: PeerId, cx: &mut ViewContext<Self>) {
        if let Some(shared_screen) = self.shared_screen_for_peer(peer_id, &self.active_pane, cx) {
            self.active_pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(shared_screen), false, true, None, cx)
            });
        }
    }

    pub fn activate_item(&mut self, item: &dyn ItemHandle, cx: &mut WindowContext) -> bool {
        let result = self.panes.iter().find_map(|pane| {
            pane.read(cx)
                .index_for_item(item)
                .map(|ix| (pane.clone(), ix))
        });
        if let Some((pane, ix)) = result {
            pane.update(cx, |pane, cx| pane.activate_item(ix, true, true, cx));
            true
        } else {
            false
        }
    }

    fn activate_pane_at_index(&mut self, action: &ActivatePane, cx: &mut ViewContext<Self>) {
        let panes = self.center.panes();
        if let Some(pane) = panes.get(action.0).map(|p| (*p).clone()) {
            cx.focus_view(&pane);
        } else {
            self.split_and_clone(self.active_pane.clone(), SplitDirection::Right, cx);
        }
    }

    pub fn activate_next_pane(&mut self, cx: &mut WindowContext) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let next_ix = (ix + 1) % panes.len();
            let next_pane = panes[next_ix].clone();
            cx.focus_view(&next_pane);
        }
    }

    pub fn activate_previous_pane(&mut self, cx: &mut WindowContext) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let prev_ix = cmp::min(ix.wrapping_sub(1), panes.len() - 1);
            let prev_pane = panes[prev_ix].clone();
            cx.focus_view(&prev_pane);
        }
    }

    pub fn activate_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        cx: &mut WindowContext,
    ) {
        use ActivateInDirectionTarget as Target;
        enum Origin {
            LeftDock,
            RightDock,
            BottomDock,
            Center,
        }

        let origin: Origin = [
            (&self.left_dock, Origin::LeftDock),
            (&self.right_dock, Origin::RightDock),
            (&self.bottom_dock, Origin::BottomDock),
        ]
        .into_iter()
        .find_map(|(dock, origin)| {
            if dock.focus_handle(cx).contains_focused(cx) && dock.read(cx).is_open() {
                Some(origin)
            } else {
                None
            }
        })
        .unwrap_or(Origin::Center);

        let get_last_active_pane = || {
            self.last_active_center_pane.as_ref().and_then(|p| {
                let p = p.upgrade()?;
                (p.read(cx).items_len() != 0).then_some(p)
            })
        };

        let try_dock =
            |dock: &View<Dock>| dock.read(cx).is_open().then(|| Target::Dock(dock.clone()));

        let target = match (origin, direction) {
            // We're in the center, so we first try to go to a different pane,
            // otherwise try to go to a dock.
            (Origin::Center, direction) => {
                if let Some(pane) = self.find_pane_in_direction(direction, cx) {
                    Some(Target::Pane(pane))
                } else {
                    match direction {
                        SplitDirection::Up => None,
                        SplitDirection::Down => try_dock(&self.bottom_dock),
                        SplitDirection::Left => try_dock(&self.left_dock),
                        SplitDirection::Right => try_dock(&self.right_dock),
                    }
                }
            }

            (Origin::LeftDock, SplitDirection::Right) => {
                if let Some(last_active_pane) = get_last_active_pane() {
                    Some(Target::Pane(last_active_pane))
                } else {
                    try_dock(&self.bottom_dock).or_else(|| try_dock(&self.right_dock))
                }
            }

            (Origin::LeftDock, SplitDirection::Down)
            | (Origin::RightDock, SplitDirection::Down) => try_dock(&self.bottom_dock),

            (Origin::BottomDock, SplitDirection::Up) => get_last_active_pane().map(Target::Pane),
            (Origin::BottomDock, SplitDirection::Left) => try_dock(&self.left_dock),
            (Origin::BottomDock, SplitDirection::Right) => try_dock(&self.right_dock),

            (Origin::RightDock, SplitDirection::Left) => {
                if let Some(last_active_pane) = get_last_active_pane() {
                    Some(Target::Pane(last_active_pane))
                } else {
                    try_dock(&self.bottom_dock).or_else(|| try_dock(&self.left_dock))
                }
            }

            _ => None,
        };

        match target {
            Some(ActivateInDirectionTarget::Pane(pane)) => cx.focus_view(&pane),
            Some(ActivateInDirectionTarget::Dock(dock)) => {
                if let Some(panel) = dock.read(cx).active_panel() {
                    panel.focus_handle(cx).focus(cx);
                } else {
                    log::error!("Could not find a focus target when in switching focus in {direction} direction for a {:?} dock", dock.read(cx).position());
                }
            }
            None => {}
        }
    }

    pub fn find_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        cx: &WindowContext,
    ) -> Option<View<Pane>> {
        let Some(bounding_box) = self.center.bounding_box_for_pane(&self.active_pane) else {
            return None;
        };
        let cursor = self.active_pane.read(cx).pixel_position_of_cursor(cx);
        let center = match cursor {
            Some(cursor) if bounding_box.contains(&cursor) => cursor,
            _ => bounding_box.center(),
        };

        let distance_to_next = pane_group::HANDLE_HITBOX_SIZE;

        let target = match direction {
            SplitDirection::Left => {
                Point::new(bounding_box.left() - distance_to_next.into(), center.y)
            }
            SplitDirection::Right => {
                Point::new(bounding_box.right() + distance_to_next.into(), center.y)
            }
            SplitDirection::Up => {
                Point::new(center.x, bounding_box.top() - distance_to_next.into())
            }
            SplitDirection::Down => {
                Point::new(center.x, bounding_box.bottom() + distance_to_next.into())
            }
        };
        self.center.pane_at_pixel_position(target).cloned()
    }

    pub fn swap_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(to) = self
            .find_pane_in_direction(direction, cx)
            .map(|pane| pane.clone())
        {
            self.center.swap(&self.active_pane.clone(), &to);
            cx.notify();
        }
    }

    fn handle_pane_focused(&mut self, pane: View<Pane>, cx: &mut ViewContext<Self>) {
        // This is explicitly hoisted out of the following check for pane identity as
        // terminal panel panes are not registered as a center panes.
        self.status_bar.update(cx, |status_bar, cx| {
            status_bar.set_active_pane(&pane, cx);
        });
        if self.active_pane != pane {
            self.active_pane = pane.clone();
            self.active_item_path_changed(cx);
            self.last_active_center_pane = Some(pane.downgrade());
        }

        self.dismiss_zoomed_items_to_reveal(None, cx);
        if pane.read(cx).is_zoomed() {
            self.zoomed = Some(pane.downgrade().into());
        } else {
            self.zoomed = None;
        }
        self.zoomed_position = None;
        cx.emit(Event::ZoomChanged);
        self.update_active_view_for_followers(cx);
        pane.model.update(cx, |pane, _| {
            pane.track_alternate_file_items();
        });

        cx.notify();
    }

    fn handle_pane_event(
        &mut self,
        pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::AddItem { item } => {
                item.added_to_pane(self, pane, cx);
                cx.emit(Event::ItemAdded);
            }
            pane::Event::Split(direction) => {
                self.split_and_clone(pane, *direction, cx);
            }
            pane::Event::Remove => self.remove_pane(pane, cx),
            pane::Event::ActivateItem { local } => {
                pane.model.update(cx, |pane, _| {
                    pane.track_alternate_file_items();
                });
                if *local {
                    self.unfollow(&pane, cx);
                }
                if &pane == self.active_pane() {
                    self.active_item_path_changed(cx);
                    self.update_active_view_for_followers(cx);
                }
            }
            pane::Event::ChangeItemTitle => {
                if pane == self.active_pane {
                    self.active_item_path_changed(cx);
                }
                self.update_window_edited(cx);
            }
            pane::Event::RemoveItem { item_id } => {
                cx.emit(Event::ActiveItemChanged);
                self.update_window_edited(cx);
                if let hash_map::Entry::Occupied(entry) = self.panes_by_item.entry(*item_id) {
                    if entry.get().entity_id() == pane.entity_id() {
                        entry.remove();
                    }
                }
            }
            pane::Event::Focus => {
                self.handle_pane_focused(pane.clone(), cx);
            }
            pane::Event::ZoomIn => {
                if pane == self.active_pane {
                    pane.update(cx, |pane, cx| pane.set_zoomed(true, cx));
                    if pane.read(cx).has_focus(cx) {
                        self.zoomed = Some(pane.downgrade().into());
                        self.zoomed_position = None;
                        cx.emit(Event::ZoomChanged);
                    }
                    cx.notify();
                }
            }
            pane::Event::ZoomOut => {
                pane.update(cx, |pane, cx| pane.set_zoomed(false, cx));
                if self.zoomed_position.is_none() {
                    self.zoomed = None;
                    cx.emit(Event::ZoomChanged);
                }
                cx.notify();
            }
        }

        self.serialize_workspace(cx);
    }

    pub fn split_pane(
        &mut self,
        pane_to_split: View<Pane>,
        split_direction: SplitDirection,
        cx: &mut ViewContext<Self>,
    ) -> View<Pane> {
        let new_pane = self.add_pane(cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();
        cx.notify();
        new_pane
    }

    pub fn split_and_clone(
        &mut self,
        pane: View<Pane>,
        direction: SplitDirection,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Pane>> {
        let item = pane.read(cx).active_item()?;
        let maybe_pane_handle = if let Some(clone) = item.clone_on_split(self.database_id(), cx) {
            let new_pane = self.add_pane(cx);
            new_pane.update(cx, |pane, cx| pane.add_item(clone, true, true, None, cx));
            self.center.split(&pane, &new_pane, direction).unwrap();
            Some(new_pane)
        } else {
            None
        };
        cx.notify();
        maybe_pane_handle
    }

    pub fn split_pane_with_item(
        &mut self,
        pane_to_split: WeakView<Pane>,
        split_direction: SplitDirection,
        from: WeakView<Pane>,
        item_id_to_move: EntityId,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(pane_to_split) = pane_to_split.upgrade() else {
            return;
        };
        let Some(from) = from.upgrade() else {
            return;
        };

        let new_pane = self.add_pane(cx);
        self.move_item(from.clone(), new_pane.clone(), item_id_to_move, 0, cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();
        cx.notify();
    }

    pub fn split_pane_with_project_entry(
        &mut self,
        pane_to_split: WeakView<Pane>,
        split_direction: SplitDirection,
        project_entry: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let pane_to_split = pane_to_split.upgrade()?;
        let new_pane = self.add_pane(cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();

        let path = self.project.read(cx).path_for_entry(project_entry, cx)?;
        let task = self.open_path(path, Some(new_pane.downgrade()), true, cx);
        Some(cx.foreground_executor().spawn(async move {
            task.await?;
            Ok(())
        }))
    }

    pub fn move_item(
        &mut self,
        source: View<Pane>,
        destination: View<Pane>,
        item_id_to_move: EntityId,
        destination_index: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let Some((item_ix, item_handle)) = source
            .read(cx)
            .items()
            .enumerate()
            .find(|(_, item_handle)| item_handle.item_id() == item_id_to_move)
        else {
            // Tab was closed during drag
            return;
        };

        let item_handle = item_handle.clone();

        if source != destination {
            // Close item from previous pane
            source.update(cx, |source, cx| {
                source.remove_item(item_ix, false, true, cx);
            });
        }

        // This automatically removes duplicate items in the pane
        destination.update(cx, |destination, cx| {
            destination.add_item(item_handle, true, true, Some(destination_index), cx);
            destination.focus(cx)
        });
    }

    fn remove_pane(&mut self, pane: View<Pane>, cx: &mut ViewContext<Self>) {
        if self.center.remove(&pane).unwrap() {
            self.force_remove_pane(&pane, cx);
            self.unfollow(&pane, cx);
            self.last_leaders_by_pane.remove(&pane.downgrade());
            for removed_item in pane.read(cx).items() {
                self.panes_by_item.remove(&removed_item.item_id());
            }

            cx.notify();
        } else {
            self.active_item_path_changed(cx);
        }
        cx.emit(Event::PaneRemoved);
    }

    pub fn panes(&self) -> &[View<Pane>] {
        &self.panes
    }

    pub fn active_pane(&self) -> &View<Pane> {
        &self.active_pane
    }

    pub fn adjacent_pane(&mut self, cx: &mut ViewContext<Self>) -> View<Pane> {
        self.find_pane_in_direction(SplitDirection::Right, cx)
            .or_else(|| self.find_pane_in_direction(SplitDirection::Left, cx))
            .unwrap_or_else(|| self.split_pane(self.active_pane.clone(), SplitDirection::Right, cx))
            .clone()
    }

    pub fn pane_for(&self, handle: &dyn ItemHandle) -> Option<View<Pane>> {
        let weak_pane = self.panes_by_item.get(&handle.item_id())?;
        weak_pane.upgrade()
    }

    fn collaborator_left(&mut self, peer_id: PeerId, cx: &mut ViewContext<Self>) {
        self.follower_states.retain(|_, state| {
            if state.leader_id == peer_id {
                for item in state.items_by_leader_view_id.values() {
                    item.set_leader_peer_id(None, cx);
                }
                false
            } else {
                true
            }
        });
        cx.notify();
    }

    pub fn start_following(
        &mut self,
        leader_id: PeerId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let pane = self.active_pane().clone();

        self.last_leaders_by_pane
            .insert(pane.downgrade(), leader_id);
        self.unfollow(&pane, cx);
        self.follower_states.insert(
            pane.clone(),
            FollowerState {
                leader_id,
                active_view_id: None,
                items_by_leader_view_id: Default::default(),
            },
        );
        cx.notify();

        let room_id = self.active_call()?.read(cx).room()?.read(cx).id();
        let project_id = self.project.read(cx).remote_id();
        let request = self.app_state.client.request(proto::Follow {
            room_id,
            project_id,
            leader_id: Some(leader_id),
        });

        Some(cx.spawn(|this, mut cx| async move {
            let response = request.await?;
            this.update(&mut cx, |this, _| {
                let state = this
                    .follower_states
                    .get_mut(&pane)
                    .ok_or_else(|| anyhow!("following interrupted"))?;
                state.active_view_id = if let Some(active_view_id) = response.active_view_id {
                    Some(ViewId::from_proto(active_view_id)?)
                } else {
                    None
                };
                Ok::<_, anyhow::Error>(())
            })??;
            if let Some(view) = response.active_view {
                Self::add_view_from_leader(this.clone(), leader_id, pane.clone(), &view, &mut cx)
                    .await?;
            }
            Self::add_views_from_leader(
                this.clone(),
                leader_id,
                vec![pane],
                response.views,
                &mut cx,
            )
            .await?;
            this.update(&mut cx, |this, cx| this.leader_updated(leader_id, cx))?;
            Ok(())
        }))
    }

    pub fn follow_next_collaborator(
        &mut self,
        _: &FollowNextCollaborator,
        cx: &mut ViewContext<Self>,
    ) {
        let collaborators = self.project.read(cx).collaborators();
        let next_leader_id = if let Some(leader_id) = self.leader_for_pane(&self.active_pane) {
            let mut collaborators = collaborators.keys().copied();
            for peer_id in collaborators.by_ref() {
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

        let pane = self.active_pane.clone();
        let Some(leader_id) = next_leader_id.or_else(|| collaborators.keys().copied().next())
        else {
            return;
        };
        if Some(leader_id) == self.unfollow(&pane, cx) {
            return;
        }
        if let Some(task) = self.start_following(leader_id, cx) {
            task.detach_and_log_err(cx)
        }
    }

    pub fn follow(&mut self, leader_id: PeerId, cx: &mut ViewContext<Self>) {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return;
        };
        let room = room.read(cx);
        let Some(remote_participant) = room.remote_participant_for_peer_id(leader_id) else {
            return;
        };

        let project = self.project.read(cx);

        let other_project_id = match remote_participant.location {
            call::ParticipantLocation::External => None,
            call::ParticipantLocation::UnsharedProject => None,
            call::ParticipantLocation::SharedProject { project_id } => {
                if Some(project_id) == project.remote_id() {
                    None
                } else {
                    Some(project_id)
                }
            }
        };

        // if they are active in another project, follow there.
        if let Some(project_id) = other_project_id {
            let app_state = self.app_state.clone();
            crate::join_in_room_project(project_id, remote_participant.user.id, app_state, cx)
                .detach_and_log_err(cx);
        }

        // if you're already following, find the right pane and focus it.
        for (pane, state) in &self.follower_states {
            if leader_id == state.leader_id {
                cx.focus_view(pane);
                return;
            }
        }

        // Otherwise, follow.
        if let Some(task) = self.start_following(leader_id, cx) {
            task.detach_and_log_err(cx)
        }
    }

    pub fn unfollow(&mut self, pane: &View<Pane>, cx: &mut ViewContext<Self>) -> Option<PeerId> {
        let state = self.follower_states.remove(pane)?;
        let leader_id = state.leader_id;
        for (_, item) in state.items_by_leader_view_id {
            item.set_leader_peer_id(None, cx);
        }

        if self
            .follower_states
            .values()
            .all(|state| state.leader_id != leader_id)
        {
            let project_id = self.project.read(cx).remote_id();
            let room_id = self.active_call()?.read(cx).room()?.read(cx).id();
            self.app_state
                .client
                .send(proto::Unfollow {
                    room_id,
                    project_id,
                    leader_id: Some(leader_id),
                })
                .log_err();
        }

        cx.notify();
        Some(leader_id)
    }

    pub fn is_being_followed(&self, peer_id: PeerId) -> bool {
        self.follower_states
            .values()
            .any(|state| state.leader_id == peer_id)
    }

    fn active_item_path_changed(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::ActiveItemChanged);
        let active_entry = self.active_project_path(cx);
        self.project
            .update(cx, |project, cx| project.set_active_path(active_entry, cx));

        self.update_window_title(cx);
    }

    fn update_window_title(&mut self, cx: &mut WindowContext) {
        let project = self.project().read(cx);
        let mut title = String::new();

        if let Some(path) = self.active_item(cx).and_then(|item| item.project_path(cx)) {
            let filename = path
                .path
                .file_name()
                .map(|s| s.to_string_lossy())
                .or_else(|| {
                    Some(Cow::Borrowed(
                        project
                            .worktree_for_id(path.worktree_id, cx)?
                            .read(cx)
                            .root_name(),
                    ))
                });

            if let Some(filename) = filename {
                title.push_str(filename.as_ref());
                title.push_str(" — ");
            }
        }

        for (i, name) in project.worktree_root_names(cx).enumerate() {
            if i > 0 {
                title.push_str(", ");
            }
            title.push_str(name);
        }

        if title.is_empty() {
            title = "empty project".to_string();
        }

        if project.is_remote() {
            title.push_str(" ↙");
        } else if project.is_shared() {
            title.push_str(" ↗");
        }

        cx.set_window_title(&title);
    }

    fn update_window_edited(&mut self, cx: &mut WindowContext) {
        let is_edited = !self.project.read(cx).is_disconnected()
            && self
                .items(cx)
                .any(|item| item.has_conflict(cx) || item.is_dirty(cx));
        if is_edited != self.window_edited {
            self.window_edited = is_edited;
            cx.set_window_edited(self.window_edited)
        }
    }

    fn render_notifications(&self, _cx: &ViewContext<Self>) -> Option<Div> {
        if self.notifications.is_empty() {
            None
        } else {
            Some(
                div()
                    .absolute()
                    .right_3()
                    .bottom_3()
                    .w_112()
                    .h_full()
                    .flex()
                    .flex_col()
                    .justify_end()
                    .gap_2()
                    .children(
                        self.notifications
                            .iter()
                            .map(|(_, notification)| notification.to_any()),
                    ),
            )
        }
    }

    // RPC handlers

    fn active_view_for_follower(
        &self,
        follower_project_id: Option<u64>,
        cx: &mut ViewContext<Self>,
    ) -> Option<proto::View> {
        let item = self.active_item(cx)?;
        let leader_id = self
            .pane_for(&*item)
            .and_then(|pane| self.leader_for_pane(&pane));

        let item_handle = item.to_followable_item_handle(cx)?;
        let id = item_handle.remote_id(&self.app_state.client, cx)?;
        let variant = item_handle.to_state_proto(cx)?;

        if item_handle.is_project_item(cx)
            && (follower_project_id.is_none()
                || follower_project_id != self.project.read(cx).remote_id())
        {
            return None;
        }

        Some(proto::View {
            id: Some(id.to_proto()),
            leader_id,
            variant: Some(variant),
        })
    }

    fn handle_follow(
        &mut self,
        follower_project_id: Option<u64>,
        cx: &mut ViewContext<Self>,
    ) -> proto::FollowResponse {
        let client = &self.app_state.client;
        let project_id = self.project.read(cx).remote_id();

        let active_view = self.active_view_for_follower(follower_project_id, cx);
        let active_view_id = active_view.as_ref().and_then(|view| view.id.clone());

        cx.notify();

        proto::FollowResponse {
            active_view,
            // TODO: once v0.124.0 is retired we can stop sending these
            active_view_id,
            views: self
                .panes()
                .iter()
                .flat_map(|pane| {
                    let leader_id = self.leader_for_pane(pane);
                    pane.read(cx).items().filter_map({
                        let cx = &cx;
                        move |item| {
                            let item = item.to_followable_item_handle(cx)?;

                            // If the item belongs to a particular project, then it should
                            // only be included if this project is shared, and the follower
                            // is in the project.
                            //
                            // Some items, like channel notes, do not belong to a particular
                            // project, so they should be included regardless of whether the
                            // current project is shared, or what project the follower is in.
                            if item.is_project_item(cx)
                                && (project_id.is_none() || project_id != follower_project_id)
                            {
                                return None;
                            }

                            let id = item.remote_id(client, cx)?.to_proto();
                            let variant = item.to_state_proto(cx)?;
                            Some(proto::View {
                                id: Some(id),
                                leader_id,
                                variant: Some(variant),
                            })
                        }
                    })
                })
                .collect(),
        }
    }

    fn handle_update_followers(
        &mut self,
        leader_id: PeerId,
        message: proto::UpdateFollowers,
        _cx: &mut ViewContext<Self>,
    ) {
        self.leader_updates_tx
            .unbounded_send((leader_id, message))
            .ok();
    }

    async fn process_leader_update(
        this: &WeakView<Self>,
        leader_id: PeerId,
        update: proto::UpdateFollowers,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        match update.variant.ok_or_else(|| anyhow!("invalid update"))? {
            proto::update_followers::Variant::UpdateActiveView(update_active_view) => {
                let panes_missing_view = this.update(cx, |this, _| {
                    let mut panes = vec![];
                    for (pane, state) in &mut this.follower_states {
                        if state.leader_id != leader_id {
                            continue;
                        }

                        state.active_view_id =
                            if let Some(active_view_id) = update_active_view.id.clone() {
                                Some(ViewId::from_proto(active_view_id)?)
                            } else {
                                None
                            };

                        if state.active_view_id.is_some_and(|view_id| {
                            !state.items_by_leader_view_id.contains_key(&view_id)
                        }) {
                            panes.push(pane.clone())
                        }
                    }
                    anyhow::Ok(panes)
                })??;

                if let Some(view) = update_active_view.view {
                    for pane in panes_missing_view {
                        Self::add_view_from_leader(this.clone(), leader_id, pane.clone(), &view, cx)
                            .await?
                    }
                }
            }
            proto::update_followers::Variant::UpdateView(update_view) => {
                let variant = update_view
                    .variant
                    .ok_or_else(|| anyhow!("missing update view variant"))?;
                let id = update_view
                    .id
                    .ok_or_else(|| anyhow!("missing update view id"))?;
                let mut tasks = Vec::new();
                this.update(cx, |this, cx| {
                    let project = this.project.clone();
                    for (_, state) in &mut this.follower_states {
                        if state.leader_id == leader_id {
                            let view_id = ViewId::from_proto(id.clone())?;
                            if let Some(item) = state.items_by_leader_view_id.get(&view_id) {
                                tasks.push(item.apply_update_proto(&project, variant.clone(), cx));
                            }
                        }
                    }
                    anyhow::Ok(())
                })??;
                try_join_all(tasks).await.log_err();
            }
            proto::update_followers::Variant::CreateView(view) => {
                let panes = this.update(cx, |this, _| {
                    this.follower_states
                        .iter()
                        .filter_map(|(pane, state)| (state.leader_id == leader_id).then_some(pane))
                        .cloned()
                        .collect()
                })?;
                Self::add_views_from_leader(this.clone(), leader_id, panes, vec![view], cx).await?;
            }
        }
        this.update(cx, |this, cx| this.leader_updated(leader_id, cx))?;
        Ok(())
    }

    async fn add_view_from_leader(
        this: WeakView<Self>,
        leader_id: PeerId,
        pane: View<Pane>,
        view: &proto::View,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let this = this.upgrade().context("workspace dropped")?;

        let item_builders = cx.update(|cx| {
            cx.default_global::<FollowableItemBuilders>()
                .values()
                .map(|b| b.0)
                .collect::<Vec<_>>()
        })?;

        let Some(id) = view.id.clone() else {
            return Err(anyhow!("no id for view"));
        };
        let id = ViewId::from_proto(id)?;

        let mut variant = view.variant.clone();
        if variant.is_none() {
            Err(anyhow!("missing view variant"))?;
        }

        let task = item_builders.iter().find_map(|build_item| {
            cx.update(|cx| build_item(pane.clone(), this.clone(), id, &mut variant, cx))
                .log_err()
                .flatten()
        });
        let Some(task) = task else {
            return Err(anyhow!(
                "failed to construct view from leader (maybe from a different version of zed?)"
            ));
        };

        let item = task.await?;

        this.update(cx, |this, cx| {
            let state = this.follower_states.get_mut(&pane)?;
            item.set_leader_peer_id(Some(leader_id), cx);
            state.items_by_leader_view_id.insert(id, item);

            Some(())
        })?;

        Ok(())
    }

    async fn add_views_from_leader(
        this: WeakView<Self>,
        leader_id: PeerId,
        panes: Vec<View<Pane>>,
        views: Vec<proto::View>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let this = this.upgrade().context("workspace dropped")?;

        let item_builders = cx.update(|cx| {
            cx.default_global::<FollowableItemBuilders>()
                .values()
                .map(|b| b.0)
                .collect::<Vec<_>>()
        })?;

        let mut item_tasks_by_pane = HashMap::default();
        for pane in panes {
            let mut item_tasks = Vec::new();
            let mut leader_view_ids = Vec::new();
            for view in &views {
                let Some(id) = &view.id else {
                    continue;
                };
                let id = ViewId::from_proto(id.clone())?;
                let mut variant = view.variant.clone();
                if variant.is_none() {
                    Err(anyhow!("missing view variant"))?;
                }
                for build_item in &item_builders {
                    let task = cx.update(|cx| {
                        build_item(pane.clone(), this.clone(), id, &mut variant, cx)
                    })?;
                    if let Some(task) = task {
                        item_tasks.push(task);
                        leader_view_ids.push(id);
                        break;
                    } else if variant.is_none() {
                        Err(anyhow!(
                            "failed to construct view from leader (maybe from a different version of zed?)"
                        ))?;
                    }
                }
            }

            item_tasks_by_pane.insert(pane, (item_tasks, leader_view_ids));
        }

        for (pane, (item_tasks, leader_view_ids)) in item_tasks_by_pane {
            let items = futures::future::try_join_all(item_tasks).await?;
            this.update(cx, |this, cx| {
                let state = this.follower_states.get_mut(&pane)?;
                for (id, item) in leader_view_ids.into_iter().zip(items) {
                    item.set_leader_peer_id(Some(leader_id), cx);
                    state.items_by_leader_view_id.insert(id, item);
                }

                Some(())
            })?;
        }
        Ok(())
    }

    pub fn update_active_view_for_followers(&mut self, cx: &mut WindowContext) {
        let mut is_project_item = true;
        let mut update = proto::UpdateActiveView::default();
        if cx.is_window_active() {
            if let Some(item) = self.active_item(cx) {
                if item.focus_handle(cx).contains_focused(cx) {
                    let leader_id = self
                        .pane_for(&*item)
                        .and_then(|pane| self.leader_for_pane(&pane));

                    if let Some(item) = item.to_followable_item_handle(cx) {
                        let id = item
                            .remote_id(&self.app_state.client, cx)
                            .map(|id| id.to_proto());

                        if let Some(id) = id.clone() {
                            if let Some(variant) = item.to_state_proto(cx) {
                                let view = Some(proto::View {
                                    id: Some(id.clone()),
                                    leader_id,
                                    variant: Some(variant),
                                });

                                is_project_item = item.is_project_item(cx);
                                update = proto::UpdateActiveView {
                                    view,
                                    // TODO: once v0.124.0 is retired we can stop sending these
                                    id: Some(id),
                                    leader_id,
                                };
                            }
                        };
                    }
                }
            }
        }

        if &update.id != &self.last_active_view_id {
            self.last_active_view_id.clone_from(&update.id);
            self.update_followers(
                is_project_item,
                proto::update_followers::Variant::UpdateActiveView(update),
                cx,
            );
        }
    }

    fn update_followers(
        &self,
        project_only: bool,
        update: proto::update_followers::Variant,
        cx: &mut WindowContext,
    ) -> Option<()> {
        // If this update only applies to for followers in the current project,
        // then skip it unless this project is shared. If it applies to all
        // followers, regardless of project, then set `project_id` to none,
        // indicating that it goes to all followers.
        let project_id = if project_only {
            Some(self.project.read(cx).remote_id()?)
        } else {
            None
        };
        self.app_state().workspace_store.update(cx, |store, cx| {
            store.update_followers(project_id, update, cx)
        })
    }

    pub fn leader_for_pane(&self, pane: &View<Pane>) -> Option<PeerId> {
        self.follower_states.get(pane).map(|state| state.leader_id)
    }

    fn leader_updated(&mut self, leader_id: PeerId, cx: &mut ViewContext<Self>) -> Option<()> {
        cx.notify();

        let call = self.active_call()?;
        let room = call.read(cx).room()?.read(cx);
        let participant = room.remote_participant_for_peer_id(leader_id)?;
        let mut items_to_activate = Vec::new();

        let leader_in_this_app;
        let leader_in_this_project;
        match participant.location {
            call::ParticipantLocation::SharedProject { project_id } => {
                leader_in_this_app = true;
                leader_in_this_project = Some(project_id) == self.project.read(cx).remote_id();
            }
            call::ParticipantLocation::UnsharedProject => {
                leader_in_this_app = true;
                leader_in_this_project = false;
            }
            call::ParticipantLocation::External => {
                leader_in_this_app = false;
                leader_in_this_project = false;
            }
        };

        for (pane, state) in &self.follower_states {
            if state.leader_id != leader_id {
                continue;
            }
            if let (Some(active_view_id), true) = (state.active_view_id, leader_in_this_app) {
                if let Some(item) = state.items_by_leader_view_id.get(&active_view_id) {
                    if leader_in_this_project || !item.is_project_item(cx) {
                        items_to_activate.push((pane.clone(), item.boxed_clone()));
                    }
                }
                continue;
            }

            if let Some(shared_screen) = self.shared_screen_for_peer(leader_id, pane, cx) {
                items_to_activate.push((pane.clone(), Box::new(shared_screen)));
            }
        }

        for (pane, item) in items_to_activate {
            let pane_was_focused = pane.read(cx).has_focus(cx);
            if let Some(index) = pane.update(cx, |pane, _| pane.index_for_item(item.as_ref())) {
                pane.update(cx, |pane, cx| pane.activate_item(index, false, false, cx));
            } else {
                pane.update(cx, |pane, cx| {
                    pane.add_item(item.boxed_clone(), false, false, None, cx)
                });
            }

            if pane_was_focused {
                pane.update(cx, |pane, cx| pane.focus_active_item(cx));
            }
        }

        None
    }

    fn shared_screen_for_peer(
        &self,
        peer_id: PeerId,
        pane: &View<Pane>,
        cx: &mut WindowContext,
    ) -> Option<View<SharedScreen>> {
        let call = self.active_call()?;
        let room = call.read(cx).room()?.read(cx);
        let participant = room.remote_participant_for_peer_id(peer_id)?;
        let track = participant.video_tracks.values().next()?.clone();
        let user = participant.user.clone();

        for item in pane.read(cx).items_of_type::<SharedScreen>() {
            if item.read(cx).peer_id == peer_id {
                return Some(item);
            }
        }

        Some(cx.new_view(|cx| SharedScreen::new(&track, peer_id, user.clone(), cx)))
    }

    pub fn on_window_activation_changed(&mut self, cx: &mut ViewContext<Self>) {
        if cx.is_window_active() {
            self.update_active_view_for_followers(cx);

            if let Some(database_id) = self.database_id {
                cx.background_executor()
                    .spawn(persistence::DB.update_timestamp(database_id))
                    .detach();
            }
        } else {
            for pane in &self.panes {
                pane.update(cx, |pane, cx| {
                    if let Some(item) = pane.active_item() {
                        item.workspace_deactivated(cx);
                    }
                    if matches!(
                        WorkspaceSettings::get_global(cx).autosave,
                        AutosaveSetting::OnWindowChange | AutosaveSetting::OnFocusChange
                    ) {
                        for item in pane.items() {
                            Pane::autosave_item(item.as_ref(), self.project.clone(), cx)
                                .detach_and_log_err(cx);
                        }
                    }
                });
            }
        }
    }

    fn active_call(&self) -> Option<&Model<ActiveCall>> {
        self.active_call.as_ref().map(|(call, _)| call)
    }

    fn on_active_call_event(
        &mut self,
        _: Model<ActiveCall>,
        event: &call::room::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            call::room::Event::ParticipantLocationChanged { participant_id }
            | call::room::Event::RemoteVideoTracksChanged { participant_id } => {
                self.leader_updated(*participant_id, cx);
            }
            _ => {}
        }
    }

    pub fn database_id(&self) -> Option<WorkspaceId> {
        self.database_id
    }

    fn local_paths(&self, cx: &AppContext) -> Option<Vec<Arc<Path>>> {
        let project = self.project().read(cx);

        if project.is_local() {
            Some(
                project
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }

    fn remove_panes(&mut self, member: Member, cx: &mut ViewContext<Workspace>) {
        match member {
            Member::Axis(PaneAxis { members, .. }) => {
                for child in members.iter() {
                    self.remove_panes(child.clone(), cx)
                }
            }
            Member::Pane(pane) => {
                self.force_remove_pane(&pane, cx);
            }
        }
    }

    fn force_remove_pane(&mut self, pane: &View<Pane>, cx: &mut ViewContext<Workspace>) {
        self.panes.retain(|p| p != pane);
        self.panes
            .last()
            .unwrap()
            .update(cx, |pane, cx| pane.focus(cx));
        if self.last_active_center_pane == Some(pane.downgrade()) {
            self.last_active_center_pane = None;
        }
        cx.notify();
    }

    fn serialize_workspace(&mut self, cx: &mut ViewContext<Self>) {
        if self._schedule_serialize.is_none() {
            self._schedule_serialize = Some(cx.spawn(|this, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
                this.update(&mut cx, |this, cx| {
                    this.serialize_workspace_internal(cx).detach();
                    this._schedule_serialize.take();
                })
                .log_err();
            }));
        }
    }

    fn serialize_workspace_internal(&self, cx: &mut WindowContext) -> Task<()> {
        let Some(database_id) = self.database_id() else {
            return Task::ready(());
        };

        fn serialize_pane_handle(pane_handle: &View<Pane>, cx: &WindowContext) -> SerializedPane {
            let (items, active) = {
                let pane = pane_handle.read(cx);
                let active_item_id = pane.active_item().map(|item| item.item_id());
                (
                    pane.items()
                        .filter_map(|item_handle| {
                            Some(SerializedItem {
                                kind: Arc::from(item_handle.serialized_item_kind()?),
                                item_id: item_handle.item_id().as_u64(),
                                active: Some(item_handle.item_id()) == active_item_id,
                                preview: pane.is_active_preview_item(item_handle.item_id()),
                            })
                        })
                        .collect::<Vec<_>>(),
                    pane.has_focus(cx),
                )
            };

            SerializedPane::new(items, active)
        }

        fn build_serialized_pane_group(
            pane_group: &Member,
            cx: &WindowContext,
        ) -> SerializedPaneGroup {
            match pane_group {
                Member::Axis(PaneAxis {
                    axis,
                    members,
                    flexes,
                    bounding_boxes: _,
                }) => SerializedPaneGroup::Group {
                    axis: SerializedAxis(*axis),
                    children: members
                        .iter()
                        .map(|member| build_serialized_pane_group(member, cx))
                        .collect::<Vec<_>>(),
                    flexes: Some(flexes.lock().clone()),
                },
                Member::Pane(pane_handle) => {
                    SerializedPaneGroup::Pane(serialize_pane_handle(pane_handle, cx))
                }
            }
        }

        fn build_serialized_docks(this: &Workspace, cx: &mut WindowContext) -> DockStructure {
            let left_dock = this.left_dock.read(cx);
            let left_visible = left_dock.is_open();
            let left_active_panel = left_dock
                .visible_panel()
                .map(|panel| panel.persistent_name().to_string());
            let left_dock_zoom = left_dock
                .visible_panel()
                .map(|panel| panel.is_zoomed(cx))
                .unwrap_or(false);

            let right_dock = this.right_dock.read(cx);
            let right_visible = right_dock.is_open();
            let right_active_panel = right_dock
                .visible_panel()
                .map(|panel| panel.persistent_name().to_string());
            let right_dock_zoom = right_dock
                .visible_panel()
                .map(|panel| panel.is_zoomed(cx))
                .unwrap_or(false);

            let bottom_dock = this.bottom_dock.read(cx);
            let bottom_visible = bottom_dock.is_open();
            let bottom_active_panel = bottom_dock
                .visible_panel()
                .map(|panel| panel.persistent_name().to_string());
            let bottom_dock_zoom = bottom_dock
                .visible_panel()
                .map(|panel| panel.is_zoomed(cx))
                .unwrap_or(false);

            DockStructure {
                left: DockData {
                    visible: left_visible,
                    active_panel: left_active_panel,
                    zoom: left_dock_zoom,
                },
                right: DockData {
                    visible: right_visible,
                    active_panel: right_active_panel,
                    zoom: right_dock_zoom,
                },
                bottom: DockData {
                    visible: bottom_visible,
                    active_panel: bottom_active_panel,
                    zoom: bottom_dock_zoom,
                },
            }
        }

        let location = if let Some(local_paths) = self.local_paths(cx) {
            if !local_paths.is_empty() {
                let (order, paths): (Vec<_>, Vec<_>) = local_paths
                    .iter()
                    .enumerate()
                    .sorted_by(|a, b| a.1.cmp(b.1))
                    .unzip();

                Some(SerializedWorkspaceLocation::Local(
                    LocalPaths::new(paths),
                    LocalPathsOrder::new(order),
                ))
            } else {
                None
            }
        } else if let Some(dev_server_project_id) = self.project().read(cx).dev_server_project_id()
        {
            let store = dev_server_projects::Store::global(cx).read(cx);
            maybe!({
                let project = store.dev_server_project(dev_server_project_id)?;
                let dev_server = store.dev_server(project.dev_server_id)?;

                let dev_server_project = SerializedDevServerProject {
                    id: dev_server_project_id,
                    dev_server_name: dev_server.name.to_string(),
                    path: project.path.to_string(),
                };
                Some(SerializedWorkspaceLocation::DevServer(dev_server_project))
            })
        } else {
            None
        };

        // don't save workspace state for the empty workspace.
        if let Some(location) = location {
            let center_group = build_serialized_pane_group(&self.center.root, cx);
            let docks = build_serialized_docks(self, cx);
            let window_bounds = Some(SerializedWindowBounds(cx.window_bounds()));
            let serialized_workspace = SerializedWorkspace {
                id: database_id,
                location,
                center_group,
                window_bounds,
                display: Default::default(),
                docks,
                centered_layout: self.centered_layout,
            };
            return cx.spawn(|_| persistence::DB.save_workspace(serialized_workspace));
        }
        Task::ready(())
    }

    pub(crate) fn load_workspace(
        serialized_workspace: SerializedWorkspace,
        paths_to_open: Vec<Option<ProjectPath>>,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<Vec<Option<Box<dyn ItemHandle>>>>> {
        cx.spawn(|workspace, mut cx| async move {
            let project = workspace.update(&mut cx, |workspace, _| workspace.project().clone())?;

            let mut center_group = None;
            let mut center_items = None;

            // Traverse the splits tree and add to things
            if let Some((group, active_pane, items)) = serialized_workspace
                .center_group
                .deserialize(
                    &project,
                    serialized_workspace.id,
                    workspace.clone(),
                    &mut cx,
                )
                .await
            {
                center_items = Some(items);
                center_group = Some((group, active_pane))
            }

            let mut items_by_project_path = cx.update(|cx| {
                center_items
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|item| {
                        let item = item?;
                        let project_path = item.project_path(cx)?;
                        Some((project_path, item))
                    })
                    .collect::<HashMap<_, _>>()
            })?;

            let opened_items = paths_to_open
                .into_iter()
                .map(|path_to_open| {
                    path_to_open
                        .and_then(|path_to_open| items_by_project_path.remove(&path_to_open))
                })
                .collect::<Vec<_>>();

            // Remove old panes from workspace panes list
            workspace.update(&mut cx, |workspace, cx| {
                if let Some((center_group, active_pane)) = center_group {
                    workspace.remove_panes(workspace.center.root.clone(), cx);

                    // Swap workspace center group
                    workspace.center = PaneGroup::with_root(center_group);
                    workspace.last_active_center_pane = active_pane.as_ref().map(|p| p.downgrade());
                    if let Some(active_pane) = active_pane {
                        workspace.active_pane = active_pane;
                        cx.focus_self();
                    } else {
                        workspace.active_pane = workspace.center.first_pane().clone();
                    }
                }

                let docks = serialized_workspace.docks;

                let right = docks.right.clone();
                workspace
                    .right_dock
                    .update(cx, |dock, _| dock.serialized_dock = Some(right));
                let left = docks.left.clone();
                workspace
                    .left_dock
                    .update(cx, |dock, _| dock.serialized_dock = Some(left));
                let bottom = docks.bottom.clone();
                workspace
                    .bottom_dock
                    .update(cx, |dock, _| dock.serialized_dock = Some(bottom));

                cx.notify();
            })?;

            // Serialize ourself to make sure our timestamps and any pane / item changes are replicated
            workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.serialize_workspace_internal(cx).detach();
                })
                .ok();

            Ok(opened_items)
        })
    }

    fn actions(&self, div: Div, cx: &mut ViewContext<Self>) -> Div {
        self.add_workspace_actions_listeners(div, cx)
            .on_action(cx.listener(Self::close_inactive_items_and_panes))
            .on_action(cx.listener(Self::close_all_items_and_panes))
            .on_action(cx.listener(Self::save_all))
            .on_action(cx.listener(Self::send_keystrokes))
            .on_action(cx.listener(Self::add_folder_to_project))
            .on_action(cx.listener(Self::follow_next_collaborator))
            .on_action(cx.listener(|workspace, _: &Unfollow, cx| {
                let pane = workspace.active_pane().clone();
                workspace.unfollow(&pane, cx);
            }))
            .on_action(cx.listener(|workspace, action: &Save, cx| {
                workspace
                    .save_active_item(action.save_intent.unwrap_or(SaveIntent::Save), cx)
                    .detach_and_log_err(cx);
            }))
            .on_action(cx.listener(|workspace, _: &SaveWithoutFormat, cx| {
                workspace
                    .save_active_item(SaveIntent::SaveWithoutFormat, cx)
                    .detach_and_log_err(cx);
            }))
            .on_action(cx.listener(|workspace, _: &SaveAs, cx| {
                workspace
                    .save_active_item(SaveIntent::SaveAs, cx)
                    .detach_and_log_err(cx);
            }))
            .on_action(cx.listener(|workspace, _: &ActivatePreviousPane, cx| {
                workspace.activate_previous_pane(cx)
            }))
            .on_action(
                cx.listener(|workspace, _: &ActivateNextPane, cx| workspace.activate_next_pane(cx)),
            )
            .on_action(
                cx.listener(|workspace, action: &ActivatePaneInDirection, cx| {
                    workspace.activate_pane_in_direction(action.0, cx)
                }),
            )
            .on_action(cx.listener(|workspace, action: &SwapPaneInDirection, cx| {
                workspace.swap_pane_in_direction(action.0, cx)
            }))
            .on_action(cx.listener(|this, _: &ToggleLeftDock, cx| {
                this.toggle_dock(DockPosition::Left, cx);
            }))
            .on_action(
                cx.listener(|workspace: &mut Workspace, _: &ToggleRightDock, cx| {
                    workspace.toggle_dock(DockPosition::Right, cx);
                }),
            )
            .on_action(
                cx.listener(|workspace: &mut Workspace, _: &ToggleBottomDock, cx| {
                    workspace.toggle_dock(DockPosition::Bottom, cx);
                }),
            )
            .on_action(
                cx.listener(|workspace: &mut Workspace, _: &CloseAllDocks, cx| {
                    workspace.close_all_docks(cx);
                }),
            )
            .on_action(cx.listener(Workspace::open))
            .on_action(cx.listener(Workspace::close_window))
            .on_action(cx.listener(Workspace::activate_pane_at_index))
            .on_action(
                cx.listener(|workspace: &mut Workspace, _: &ReopenClosedItem, cx| {
                    workspace.reopen_closed_item(cx).detach();
                }),
            )
            .on_action(cx.listener(Workspace::toggle_centered_layout))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: Model<Project>, cx: &mut ViewContext<Self>) -> Self {
        use node_runtime::FakeNodeRuntime;

        let client = project.read(cx).client();
        let user_store = project.read(cx).user_store();

        let workspace_store = cx.new_model(|cx| WorkspaceStore::new(client.clone(), cx));
        cx.activate_window();
        let app_state = Arc::new(AppState {
            languages: project.read(cx).languages().clone(),
            workspace_store,
            client,
            user_store,
            fs: project.read(cx).fs().clone(),
            build_window_options: |_, _| Default::default(),
            node_runtime: FakeNodeRuntime::new(),
        });
        let workspace = Self::new(Default::default(), project, app_state, cx);
        workspace.active_pane.update(cx, |pane, cx| pane.focus(cx));
        workspace
    }

    pub fn register_action<A: Action>(
        &mut self,
        callback: impl Fn(&mut Self, &A, &mut ViewContext<Self>) + 'static,
    ) -> &mut Self {
        let callback = Arc::new(callback);

        self.workspace_actions.push(Box::new(move |div, cx| {
            let callback = callback.clone();
            div.on_action(
                cx.listener(move |workspace, event, cx| (callback.clone())(workspace, event, cx)),
            )
        }));
        self
    }

    fn add_workspace_actions_listeners(&self, div: Div, cx: &mut ViewContext<Self>) -> Div {
        let mut div = div
            .on_action(cx.listener(Self::close_inactive_items_and_panes))
            .on_action(cx.listener(Self::close_all_items_and_panes))
            .on_action(cx.listener(Self::add_folder_to_project))
            .on_action(cx.listener(Self::save_all))
            .on_action(cx.listener(Self::open));
        for action in self.workspace_actions.iter() {
            div = (action)(div, cx)
        }
        div
    }

    pub fn has_active_modal(&self, cx: &WindowContext<'_>) -> bool {
        self.modal_layer.read(cx).has_active_modal()
    }

    pub fn active_modal<V: ManagedView + 'static>(&mut self, cx: &AppContext) -> Option<View<V>> {
        self.modal_layer.read(cx).active_modal()
    }

    pub fn toggle_modal<V: ModalView, B>(&mut self, cx: &mut WindowContext, build: B)
    where
        B: FnOnce(&mut ViewContext<V>) -> V,
    {
        self.modal_layer
            .update(cx, |modal_layer, cx| modal_layer.toggle_modal(cx, build))
    }

    pub fn toggle_centered_layout(&mut self, _: &ToggleCenteredLayout, cx: &mut ViewContext<Self>) {
        self.centered_layout = !self.centered_layout;
        if let Some(database_id) = self.database_id() {
            cx.background_executor()
                .spawn(DB.set_centered_layout(database_id, self.centered_layout))
                .detach_and_log_err(cx);
        }
        cx.notify();
    }

    fn adjust_padding(padding: Option<f32>) -> f32 {
        padding
            .unwrap_or(Self::DEFAULT_PADDING)
            .min(Self::MAX_PADDING)
            .max(0.0)
    }
}

fn window_bounds_env_override() -> Option<Bounds<DevicePixels>> {
    ZED_WINDOW_POSITION
        .zip(*ZED_WINDOW_SIZE)
        .map(|(position, size)| Bounds {
            origin: position,
            size,
        })
}

fn open_items(
    serialized_workspace: Option<SerializedWorkspace>,
    mut project_paths_to_open: Vec<(PathBuf, Option<ProjectPath>)>,
    app_state: Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) -> impl 'static + Future<Output = Result<Vec<Option<Result<Box<dyn ItemHandle>>>>>> {
    let restored_items = serialized_workspace.map(|serialized_workspace| {
        Workspace::load_workspace(
            serialized_workspace,
            project_paths_to_open
                .iter()
                .map(|(_, project_path)| project_path)
                .cloned()
                .collect(),
            cx,
        )
    });

    cx.spawn(|workspace, mut cx| async move {
        let mut opened_items = Vec::with_capacity(project_paths_to_open.len());

        if let Some(restored_items) = restored_items {
            let restored_items = restored_items.await?;

            let restored_project_paths = restored_items
                .iter()
                .filter_map(|item| {
                    cx.update(|cx| item.as_ref()?.project_path(cx))
                        .ok()
                        .flatten()
                })
                .collect::<HashSet<_>>();

            for restored_item in restored_items {
                opened_items.push(restored_item.map(Ok));
            }

            project_paths_to_open
                .iter_mut()
                .for_each(|(_, project_path)| {
                    if let Some(project_path_to_open) = project_path {
                        if restored_project_paths.contains(project_path_to_open) {
                            *project_path = None;
                        }
                    }
                });
        } else {
            for _ in 0..project_paths_to_open.len() {
                opened_items.push(None);
            }
        }
        assert!(opened_items.len() == project_paths_to_open.len());

        let tasks =
            project_paths_to_open
                .into_iter()
                .enumerate()
                .map(|(ix, (abs_path, project_path))| {
                    let workspace = workspace.clone();
                    cx.spawn(|mut cx| {
                        let fs = app_state.fs.clone();
                        async move {
                            let file_project_path = project_path?;
                            if fs.is_dir(&abs_path).await {
                                None
                            } else {
                                Some((
                                    ix,
                                    workspace
                                        .update(&mut cx, |workspace, cx| {
                                            workspace.open_path(file_project_path, None, true, cx)
                                        })
                                        .log_err()?
                                        .await,
                                ))
                            }
                        }
                    })
                });

        let tasks = tasks.collect::<Vec<_>>();

        let tasks = futures::future::join_all(tasks);
        for (ix, path_open_result) in tasks.await.into_iter().flatten() {
            opened_items[ix] = Some(path_open_result);
        }

        Ok(opened_items)
    })
}

enum ActivateInDirectionTarget {
    Pane(View<Pane>),
    Dock(View<Dock>),
}

fn notify_if_database_failed(workspace: WindowHandle<Workspace>, cx: &mut AsyncAppContext) {
    const REPORT_ISSUE_URL: &str = "https://github.com/zed-industries/zed/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml";

    workspace
        .update(cx, |workspace, cx| {
            if (*db::ALL_FILE_DB_FAILED).load(std::sync::atomic::Ordering::Acquire) {
                struct DatabaseFailedNotification;

                workspace.show_notification_once(
                    NotificationId::unique::<DatabaseFailedNotification>(),
                    cx,
                    |cx| {
                        cx.new_view(|_| {
                            MessageNotification::new("Failed to load the database file.")
                                .with_click_message("Click to let us know about this error")
                                .on_click(|cx| cx.open_url(REPORT_ISSUE_URL))
                        })
                    },
                );
            }
        })
        .log_err();
}

impl FocusableView for Workspace {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

#[derive(Clone, Render)]
struct DraggedDock(DockPosition);

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut context = KeyContext::new_with_defaults();
        context.add("Workspace");
        let centered_layout = self.centered_layout
            && self.center.panes().len() == 1
            && self.active_item(cx).is_some();
        let render_padding = |size| {
            (size > 0.0).then(|| {
                div()
                    .h_full()
                    .w(relative(size))
                    .bg(cx.theme().colors().editor_background)
                    .border_color(cx.theme().colors().pane_group_border)
            })
        };
        let paddings = if centered_layout {
            let settings = WorkspaceSettings::get_global(cx).centered_layout;
            (
                render_padding(Self::adjust_padding(settings.left_padding)),
                render_padding(Self::adjust_padding(settings.right_padding)),
            )
        } else {
            (None, None)
        };
        let (ui_font, ui_font_size) = {
            let theme_settings = ThemeSettings::get_global(cx);
            (theme_settings.ui_font.clone(), theme_settings.ui_font_size)
        };

        let theme = cx.theme().clone();
        let colors = theme.colors();
        cx.set_rem_size(ui_font_size);

        self.actions(div(), cx)
            .key_context(context)
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .font(ui_font)
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(colors.text)
            .bg(colors.background)
            .children(self.titlebar_item.clone())
            .child(
                div()
                    .id("workspace")
                    .relative()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .border_t_1()
                    .border_b_1()
                    .border_color(colors.border)
                    .child({
                        let this = cx.view().clone();
                        canvas(
                            move |bounds, cx| this.update(cx, |this, _cx| this.bounds = bounds),
                            |_, _, _| {},
                        )
                        .absolute()
                        .size_full()
                    })
                    .when(self.zoomed.is_none(), |this| {
                        this.on_drag_move(cx.listener(
                            |workspace, e: &DragMoveEvent<DraggedDock>, cx| match e.drag(cx).0 {
                                DockPosition::Left => {
                                    let size = workspace.bounds.left() + e.event.position.x;
                                    workspace.left_dock.update(cx, |left_dock, cx| {
                                        left_dock.resize_active_panel(Some(size), cx);
                                    });
                                }
                                DockPosition::Right => {
                                    let size = workspace.bounds.right() - e.event.position.x;
                                    workspace.right_dock.update(cx, |right_dock, cx| {
                                        right_dock.resize_active_panel(Some(size), cx);
                                    });
                                }
                                DockPosition::Bottom => {
                                    let size = workspace.bounds.bottom() - e.event.position.y;
                                    workspace.bottom_dock.update(cx, |bottom_dock, cx| {
                                        bottom_dock.resize_active_panel(Some(size), cx);
                                    });
                                }
                            },
                        ))
                    })
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .h_full()
                            // Left Dock
                            .children(self.zoomed_position.ne(&Some(DockPosition::Left)).then(
                                || {
                                    div()
                                        .flex()
                                        .flex_none()
                                        .overflow_hidden()
                                        .child(self.left_dock.clone())
                                },
                            ))
                            // Panes
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(
                                        h_flex()
                                            .flex_1()
                                            .when_some(paddings.0, |this, p| {
                                                this.child(p.border_r_1())
                                            })
                                            .child(self.center.render(
                                                &self.project,
                                                &self.follower_states,
                                                self.active_call(),
                                                &self.active_pane,
                                                self.zoomed.as_ref(),
                                                &self.app_state,
                                                cx,
                                            ))
                                            .when_some(paddings.1, |this, p| {
                                                this.child(p.border_l_1())
                                            }),
                                    )
                                    .children(
                                        self.zoomed_position
                                            .ne(&Some(DockPosition::Bottom))
                                            .then(|| self.bottom_dock.clone()),
                                    ),
                            )
                            // Right Dock
                            .children(self.zoomed_position.ne(&Some(DockPosition::Right)).then(
                                || {
                                    div()
                                        .flex()
                                        .flex_none()
                                        .overflow_hidden()
                                        .child(self.right_dock.clone())
                                },
                            )),
                    )
                    .children(self.zoomed.as_ref().and_then(|view| {
                        let zoomed_view = view.upgrade()?;
                        let div = div()
                            .occlude()
                            .absolute()
                            .overflow_hidden()
                            .border_color(colors.border)
                            .bg(colors.background)
                            .child(zoomed_view)
                            .inset_0()
                            .shadow_lg();

                        Some(match self.zoomed_position {
                            Some(DockPosition::Left) => div.right_2().border_r_1(),
                            Some(DockPosition::Right) => div.left_2().border_l_1(),
                            Some(DockPosition::Bottom) => div.top_2().border_t_1(),
                            None => div.top_2().bottom_2().left_2().right_2().border_1(),
                        })
                    }))
                    .child(self.modal_layer.clone())
                    .children(self.render_notifications(cx)),
            )
            .child(self.status_bar.clone())
            .children(if self.project.read(cx).is_disconnected() {
                Some(DisconnectedOverlay)
            } else {
                None
            })
    }
}

impl WorkspaceStore {
    pub fn new(client: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        Self {
            workspaces: Default::default(),
            _subscriptions: vec![
                client.add_request_handler(cx.weak_model(), Self::handle_follow),
                client.add_message_handler(cx.weak_model(), Self::handle_update_followers),
            ],
            client,
        }
    }

    pub fn update_followers(
        &self,
        project_id: Option<u64>,
        update: proto::update_followers::Variant,
        cx: &AppContext,
    ) -> Option<()> {
        let active_call = ActiveCall::try_global(cx)?;
        let room_id = active_call.read(cx).room()?.read(cx).id();
        self.client
            .send(proto::UpdateFollowers {
                room_id,
                project_id,
                variant: Some(update),
            })
            .log_err()
    }

    pub async fn handle_follow(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::Follow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FollowResponse> {
        this.update(&mut cx, |this, cx| {
            let follower = Follower {
                project_id: envelope.payload.project_id,
                peer_id: envelope.original_sender_id()?,
            };

            let mut response = proto::FollowResponse::default();
            this.workspaces.retain(|workspace| {
                workspace
                    .update(cx, |workspace, cx| {
                        let handler_response = workspace.handle_follow(follower.project_id, cx);
                        if response.views.is_empty() {
                            response.views = handler_response.views;
                        } else {
                            response.views.extend_from_slice(&handler_response.views);
                        }

                        if let Some(active_view_id) = handler_response.active_view_id.clone() {
                            if response.active_view_id.is_none()
                                || workspace.project.read(cx).remote_id() == follower.project_id
                            {
                                response.active_view_id = Some(active_view_id);
                            }
                        }

                        if let Some(active_view) = handler_response.active_view.clone() {
                            if response.active_view_id.is_none()
                                || workspace.project.read(cx).remote_id() == follower.project_id
                            {
                                response.active_view = Some(active_view)
                            }
                        }
                    })
                    .is_ok()
            });

            Ok(response)
        })?
    }

    async fn handle_update_followers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        let update = envelope.payload;

        this.update(&mut cx, |this, cx| {
            this.workspaces.retain(|workspace| {
                workspace
                    .update(cx, |workspace, cx| {
                        let project_id = workspace.project.read(cx).remote_id();
                        if update.project_id != project_id && update.project_id.is_some() {
                            return;
                        }
                        workspace.handle_update_followers(leader_id, update.clone(), cx);
                    })
                    .is_ok()
            });
            Ok(())
        })?
    }
}

impl ViewId {
    pub(crate) fn from_proto(message: proto::ViewId) -> Result<Self> {
        Ok(Self {
            creator: message
                .creator
                .ok_or_else(|| anyhow!("creator is missing"))?,
            id: message.id,
        })
    }

    pub(crate) fn to_proto(&self) -> proto::ViewId {
        proto::ViewId {
            creator: Some(self.creator),
            id: self.id,
        }
    }
}

pub trait WorkspaceHandle {
    fn file_project_paths(&self, cx: &AppContext) -> Vec<ProjectPath>;
}

impl WorkspaceHandle for View<Workspace> {
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

impl std::fmt::Debug for OpenPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenPaths")
            .field("paths", &self.paths)
            .finish()
    }
}

pub fn activate_workspace_for_project(
    cx: &mut AppContext,
    predicate: impl Fn(&Project, &AppContext) -> bool + Send + 'static,
) -> Option<WindowHandle<Workspace>> {
    for window in cx.windows() {
        let Some(workspace) = window.downcast::<Workspace>() else {
            continue;
        };

        let predicate = workspace
            .update(cx, |workspace, cx| {
                let project = workspace.project.read(cx);
                if predicate(project, cx) {
                    cx.activate_window();
                    true
                } else {
                    false
                }
            })
            .log_err()
            .unwrap_or(false);

        if predicate {
            return Some(workspace);
        }
    }

    None
}

pub async fn last_opened_workspace_paths() -> Option<LocalPaths> {
    DB.last_workspace().await.log_err().flatten()
}

actions!(collab, [OpenChannelNotes]);
actions!(zed, [OpenLog]);

async fn join_channel_internal(
    channel_id: ChannelId,
    app_state: &Arc<AppState>,
    requesting_window: Option<WindowHandle<Workspace>>,
    active_call: &Model<ActiveCall>,
    cx: &mut AsyncAppContext,
) -> Result<bool> {
    let (should_prompt, open_room) = active_call.update(cx, |active_call, cx| {
        let Some(room) = active_call.room().map(|room| room.read(cx)) else {
            return (false, None);
        };

        let already_in_channel = room.channel_id() == Some(channel_id);
        let should_prompt = room.is_sharing_project()
            && room.remote_participants().len() > 0
            && !already_in_channel;
        let open_room = if already_in_channel {
            active_call.room().cloned()
        } else {
            None
        };
        (should_prompt, open_room)
    })?;

    if let Some(room) = open_room {
        let task = room.update(cx, |room, cx| {
            if let Some((project, host)) = room.most_active_project(cx) {
                return Some(join_in_room_project(project, host, app_state.clone(), cx));
            }

            None
        })?;
        if let Some(task) = task {
            task.await?;
        }
        return anyhow::Ok(true);
    }

    if should_prompt {
        if let Some(workspace) = requesting_window {
            let answer = workspace
                .update(cx, |_, cx| {
                    cx.prompt(
                        PromptLevel::Warning,
                        "Do you want to switch channels?",
                        Some("Leaving this call will unshare your current project."),
                        &["Yes, Join Channel", "Cancel"],
                    )
                })?
                .await;

            if answer == Ok(1) {
                return Ok(false);
            }
        } else {
            return Ok(false); // unreachable!() hopefully
        }
    }

    let client = cx.update(|cx| active_call.read(cx).client())?;

    let mut client_status = client.status();

    // this loop will terminate within client::CONNECTION_TIMEOUT seconds.
    'outer: loop {
        let Some(status) = client_status.recv().await else {
            return Err(anyhow!("error connecting"));
        };

        match status {
            Status::Connecting
            | Status::Authenticating
            | Status::Reconnecting
            | Status::Reauthenticating => continue,
            Status::Connected { .. } => break 'outer,
            Status::SignedOut => return Err(ErrorCode::SignedOut.into()),
            Status::UpgradeRequired => return Err(ErrorCode::UpgradeRequired.into()),
            Status::ConnectionError | Status::ConnectionLost | Status::ReconnectionError { .. } => {
                return Err(ErrorCode::Disconnected.into());
            }
        }
    }

    let room = active_call
        .update(cx, |active_call, cx| {
            active_call.join_channel(channel_id, cx)
        })?
        .await?;

    let Some(room) = room else {
        return anyhow::Ok(true);
    };

    room.update(cx, |room, _| room.room_update_completed())?
        .await;

    let task = room.update(cx, |room, cx| {
        if let Some((project, host)) = room.most_active_project(cx) {
            return Some(join_in_room_project(project, host, app_state.clone(), cx));
        }

        // If you are the first to join a channel, see if you should share your project.
        if room.remote_participants().is_empty() && !room.local_participant_is_guest() {
            if let Some(workspace) = requesting_window {
                let project = workspace.update(cx, |workspace, cx| {
                    let project = workspace.project.read(cx);
                    let is_dev_server = project.dev_server_project_id().is_some();

                    if !is_dev_server && !CallSettings::get_global(cx).share_on_join {
                        return None;
                    }

                    if (project.is_local() || is_dev_server)
                        && project.visible_worktrees(cx).any(|tree| {
                            tree.read(cx)
                                .root_entry()
                                .map_or(false, |entry| entry.is_dir())
                        })
                    {
                        Some(workspace.project.clone())
                    } else {
                        None
                    }
                });
                if let Ok(Some(project)) = project {
                    return Some(cx.spawn(|room, mut cx| async move {
                        room.update(&mut cx, |room, cx| room.share_project(project, cx))?
                            .await?;
                        Ok(())
                    }));
                }
            }
        }

        None
    })?;
    if let Some(task) = task {
        task.await?;
        return anyhow::Ok(true);
    }
    anyhow::Ok(false)
}

pub fn join_channel(
    channel_id: ChannelId,
    app_state: Arc<AppState>,
    requesting_window: Option<WindowHandle<Workspace>>,
    cx: &mut AppContext,
) -> Task<Result<()>> {
    let active_call = ActiveCall::global(cx);
    cx.spawn(|mut cx| async move {
        let result = join_channel_internal(
            channel_id,
            &app_state,
            requesting_window,
            &active_call,
            &mut cx,
        )
            .await;

        // join channel succeeded, and opened a window
        if matches!(result, Ok(true)) {
            return anyhow::Ok(());
        }

        // find an existing workspace to focus and show call controls
        let mut active_window =
            requesting_window.or_else(|| activate_any_workspace_window(&mut cx));
        if active_window.is_none() {
            // no open workspaces, make one to show the error in (blergh)
            let (window_handle, _) = cx
                .update(|cx| {
                    Workspace::new_local(vec![], app_state.clone(), requesting_window, cx)
                })?
                .await?;

            if result.is_ok() {
                cx.update(|cx| {
                    cx.dispatch_action(&OpenChannelNotes);
                }).log_err();
            }

            active_window = Some(window_handle);
        }

        if let Err(err) = result {
            log::error!("failed to join channel: {}", err);
            if let Some(active_window) = active_window {
                active_window
                    .update(&mut cx, |_, cx| {
                        let detail: SharedString = match err.error_code() {
                            ErrorCode::SignedOut => {
                                "Please sign in to continue.".into()
                            }
                            ErrorCode::UpgradeRequired => {
                                "Your are running an unsupported version of Zed. Please update to continue.".into()
                            }
                            ErrorCode::NoSuchChannel => {
                                "No matching channel was found. Please check the link and try again.".into()
                            }
                            ErrorCode::Forbidden => {
                                "This channel is private, and you do not have access. Please ask someone to add you and try again.".into()
                            }
                            ErrorCode::Disconnected => "Please check your internet connection and try again.".into(),
                            _ => format!("{}\n\nPlease try again.", err).into(),
                        };
                        cx.prompt(
                            PromptLevel::Critical,
                            "Failed to join channel",
                            Some(&detail),
                            &["Ok"],
                        )
                    })?
                    .await
                    .ok();
            }
        }

        // return ok, we showed the error to the user.
        return anyhow::Ok(());
    })
}

pub async fn get_any_active_workspace(
    app_state: Arc<AppState>,
    mut cx: AsyncAppContext,
) -> anyhow::Result<WindowHandle<Workspace>> {
    // find an existing workspace to focus and show call controls
    let active_window = activate_any_workspace_window(&mut cx);
    if active_window.is_none() {
        cx.update(|cx| Workspace::new_local(vec![], app_state.clone(), None, cx))?
            .await?;
    }
    activate_any_workspace_window(&mut cx).context("could not open zed")
}

fn activate_any_workspace_window(cx: &mut AsyncAppContext) -> Option<WindowHandle<Workspace>> {
    cx.update(|cx| {
        if let Some(workspace_window) = cx
            .active_window()
            .and_then(|window| window.downcast::<Workspace>())
        {
            return Some(workspace_window);
        }

        for window in cx.windows() {
            if let Some(workspace_window) = window.downcast::<Workspace>() {
                workspace_window
                    .update(cx, |_, cx| cx.activate_window())
                    .ok();
                return Some(workspace_window);
            }
        }
        None
    })
    .ok()
    .flatten()
}

fn local_workspace_windows(cx: &AppContext) -> Vec<WindowHandle<Workspace>> {
    cx.windows()
        .into_iter()
        .filter_map(|window| window.downcast::<Workspace>())
        .filter(|workspace| {
            workspace
                .read(cx)
                .is_ok_and(|workspace| workspace.project.read(cx).is_local())
        })
        .collect()
}

#[derive(Default)]
pub struct OpenOptions {
    pub open_new_workspace: Option<bool>,
    pub replace_window: Option<WindowHandle<Workspace>>,
}

#[allow(clippy::type_complexity)]
pub fn open_paths(
    abs_paths: &[PathBuf],
    app_state: Arc<AppState>,
    open_options: OpenOptions,
    cx: &mut AppContext,
) -> Task<
    anyhow::Result<(
        WindowHandle<Workspace>,
        Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>,
    )>,
> {
    let abs_paths = abs_paths.to_vec();
    let mut existing = None;
    let mut best_match = None;
    let mut open_visible = OpenVisible::All;

    if open_options.open_new_workspace != Some(true) {
        for window in local_workspace_windows(cx) {
            if let Ok(workspace) = window.read(cx) {
                let m = workspace
                    .project
                    .read(cx)
                    .visibility_for_paths(&abs_paths, cx);
                if m > best_match {
                    existing = Some(window);
                    best_match = m;
                } else if best_match.is_none() && open_options.open_new_workspace == Some(false) {
                    existing = Some(window)
                }
            }
        }
    }

    cx.spawn(move |mut cx| async move {
        if open_options.open_new_workspace.is_none() && existing.is_none() {
            let all_files = abs_paths.iter().map(|path| app_state.fs.metadata(path));
            if futures::future::join_all(all_files)
                .await
                .into_iter()
                .filter_map(|result| result.ok().flatten())
                .all(|file| !file.is_dir)
            {
                cx.update(|cx| {
                    for window in local_workspace_windows(cx) {
                        if let Ok(workspace) = window.read(cx) {
                            let project = workspace.project().read(cx);
                            if project.is_remote() {
                                continue;
                            }
                            existing = Some(window);
                            open_visible = OpenVisible::None;
                            break;
                        }
                    }
                })?;
            }
        }

        if let Some(existing) = existing {
            Ok((
                existing,
                existing
                    .update(&mut cx, |workspace, cx| {
                        cx.activate_window();
                        workspace.open_paths(abs_paths, open_visible, None, cx)
                    })?
                    .await,
            ))
        } else {
            cx.update(move |cx| {
                Workspace::new_local(
                    abs_paths,
                    app_state.clone(),
                    open_options.replace_window,
                    cx,
                )
            })?
            .await
        }
    })
}

pub fn open_new(
    app_state: Arc<AppState>,
    cx: &mut AppContext,
    init: impl FnOnce(&mut Workspace, &mut ViewContext<Workspace>) + 'static + Send,
) -> Task<()> {
    let task = Workspace::new_local(Vec::new(), app_state, None, cx);
    cx.spawn(|mut cx| async move {
        if let Some((workspace, opened_paths)) = task.await.log_err() {
            workspace
                .update(&mut cx, |workspace, cx| {
                    if opened_paths.is_empty() {
                        init(workspace, cx)
                    }
                })
                .log_err();
        }
    })
}

pub fn create_and_open_local_file(
    path: &'static Path,
    cx: &mut ViewContext<Workspace>,
    default_content: impl 'static + Send + FnOnce() -> Rope,
) -> Task<Result<Box<dyn ItemHandle>>> {
    cx.spawn(|workspace, mut cx| async move {
        let fs = workspace.update(&mut cx, |workspace, _| workspace.app_state().fs.clone())?;
        if !fs.is_file(path).await {
            fs.create_file(path, Default::default()).await?;
            fs.save(path, &default_content(), Default::default())
                .await?;
        }

        let mut items = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.with_local_workspace(cx, |workspace, cx| {
                    workspace.open_paths(vec![path.to_path_buf()], OpenVisible::None, None, cx)
                })
            })?
            .await?
            .await;

        let item = items.pop().flatten();
        item.ok_or_else(|| anyhow!("path {path:?} is not a file"))?
    })
}

pub fn join_hosted_project(
    hosted_project_id: ProjectId,
    app_state: Arc<AppState>,
    cx: &mut AppContext,
) -> Task<Result<()>> {
    cx.spawn(|mut cx| async move {
        let existing_window = cx.update(|cx| {
            cx.windows().into_iter().find_map(|window| {
                let workspace = window.downcast::<Workspace>()?;
                workspace
                    .read(cx)
                    .is_ok_and(|workspace| {
                        workspace.project().read(cx).hosted_project_id() == Some(hosted_project_id)
                    })
                    .then(|| workspace)
            })
        })?;

        let workspace = if let Some(existing_window) = existing_window {
            existing_window
        } else {
            let project = Project::hosted(
                hosted_project_id,
                app_state.user_store.clone(),
                app_state.client.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx.clone(),
            )
            .await?;

            let window_bounds_override = window_bounds_env_override();
            cx.update(|cx| {
                let mut options = (app_state.build_window_options)(None, cx);
                options.window_bounds =
                    window_bounds_override.map(|bounds| WindowBounds::Windowed(bounds));
                cx.open_window(options, |cx| {
                    cx.new_view(|cx| {
                        Workspace::new(Default::default(), project, app_state.clone(), cx)
                    })
                })
            })?
        };

        workspace.update(&mut cx, |_, cx| {
            cx.activate(true);
            cx.activate_window();
        })?;

        Ok(())
    })
}

pub fn join_dev_server_project(
    project_id: ProjectId,
    app_state: Arc<AppState>,
    window_to_replace: Option<WindowHandle<Workspace>>,
    cx: &mut AppContext,
) -> Task<Result<WindowHandle<Workspace>>> {
    let windows = cx.windows();
    cx.spawn(|mut cx| async move {
        let existing_workspace = windows.into_iter().find_map(|window| {
            window.downcast::<Workspace>().and_then(|window| {
                window
                    .update(&mut cx, |workspace, cx| {
                        if workspace.project().read(cx).remote_id() == Some(project_id.0) {
                            Some(window)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(None)
            })
        });

        let workspace = if let Some(existing_workspace) = existing_workspace {
            existing_workspace
        } else {
            let project = Project::remote(
                project_id.0,
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx.clone(),
            )
            .await?;

            if let Some(window_to_replace) = window_to_replace {
                cx.update_window(window_to_replace.into(), |_, cx| {
                    cx.replace_root_view(|cx| {
                        Workspace::new(Default::default(), project, app_state.clone(), cx)
                    });
                })?;
                window_to_replace
            } else {
                let window_bounds_override = window_bounds_env_override();
                cx.update(|cx| {
                    let mut options = (app_state.build_window_options)(None, cx);
                    options.window_bounds =
                        window_bounds_override.map(|bounds| WindowBounds::Windowed(bounds));
                    cx.open_window(options, |cx| {
                        cx.new_view(|cx| {
                            Workspace::new(Default::default(), project, app_state.clone(), cx)
                        })
                    })
                })?
            }
        };

        workspace.update(&mut cx, |_, cx| {
            cx.activate(true);
            cx.activate_window();
        })?;

        anyhow::Ok(workspace)
    })
}

pub fn join_in_room_project(
    project_id: u64,
    follow_user_id: u64,
    app_state: Arc<AppState>,
    cx: &mut AppContext,
) -> Task<Result<()>> {
    let windows = cx.windows();
    cx.spawn(|mut cx| async move {
        let existing_workspace = windows.into_iter().find_map(|window| {
            window.downcast::<Workspace>().and_then(|window| {
                window
                    .update(&mut cx, |workspace, cx| {
                        if workspace.project().read(cx).remote_id() == Some(project_id) {
                            Some(window)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(None)
            })
        });

        let workspace = if let Some(existing_workspace) = existing_workspace {
            existing_workspace
        } else {
            let active_call = cx.update(|cx| ActiveCall::global(cx))?;
            let room = active_call
                .read_with(&cx, |call, _| call.room().cloned())?
                .ok_or_else(|| anyhow!("not in a call"))?;
            let project = room
                .update(&mut cx, |room, cx| {
                    room.join_project(
                        project_id,
                        app_state.languages.clone(),
                        app_state.fs.clone(),
                        cx,
                    )
                })?
                .await?;

            let window_bounds_override = window_bounds_env_override();
            cx.update(|cx| {
                let mut options = (app_state.build_window_options)(None, cx);
                options.window_bounds =
                    window_bounds_override.map(|bounds| WindowBounds::Windowed(bounds));
                cx.open_window(options, |cx| {
                    cx.new_view(|cx| {
                        Workspace::new(Default::default(), project, app_state.clone(), cx)
                    })
                })
            })?
        };

        workspace.update(&mut cx, |workspace, cx| {
            cx.activate(true);
            cx.activate_window();

            if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
                let follow_peer_id = room
                    .read(cx)
                    .remote_participants()
                    .iter()
                    .find(|(_, participant)| participant.user.id == follow_user_id)
                    .map(|(_, p)| p.peer_id)
                    .or_else(|| {
                        // If we couldn't follow the given user, follow the host instead.
                        let collaborator = workspace
                            .project()
                            .read(cx)
                            .collaborators()
                            .values()
                            .find(|collaborator| collaborator.replica_id == 0)?;
                        Some(collaborator.peer_id)
                    });

                if let Some(follow_peer_id) = follow_peer_id {
                    workspace.follow(follow_peer_id, cx);
                }
            }
        })?;

        anyhow::Ok(())
    })
}

pub fn restart(restart: &Restart, cx: &mut AppContext) {
    let should_confirm = WorkspaceSettings::get_global(cx).confirm_quit;
    let mut workspace_windows = cx
        .windows()
        .into_iter()
        .filter_map(|window| window.downcast::<Workspace>())
        .collect::<Vec<_>>();

    // If multiple windows have unsaved changes, and need a save prompt,
    // prompt in the active window before switching to a different window.
    workspace_windows.sort_by_key(|window| window.is_active(cx) == Some(false));

    let mut prompt = None;
    if let (true, Some(window)) = (should_confirm, workspace_windows.first()) {
        prompt = window
            .update(cx, |_, cx| {
                cx.prompt(
                    PromptLevel::Info,
                    "Are you sure you want to restart?",
                    None,
                    &["Restart", "Cancel"],
                )
            })
            .ok();
    }

    let binary_path = restart.binary_path.clone();
    cx.spawn(|mut cx| async move {
        if let Some(prompt) = prompt {
            let answer = prompt.await?;
            if answer != 0 {
                return Ok(());
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for window in workspace_windows {
            if let Ok(should_close) = window.update(&mut cx, |workspace, cx| {
                workspace.prepare_to_close(true, cx)
            }) {
                if !should_close.await? {
                    return Ok(());
                }
            }
        }

        cx.update(|cx| cx.restart(binary_path))
    })
    .detach_and_log_err(cx);
}

fn parse_pixel_position_env_var(value: &str) -> Option<Point<DevicePixels>> {
    let mut parts = value.split(',');
    let x: usize = parts.next()?.parse().ok()?;
    let y: usize = parts.next()?.parse().ok()?;
    Some(point((x as i32).into(), (y as i32).into()))
}

fn parse_pixel_size_env_var(value: &str) -> Option<Size<DevicePixels>> {
    let mut parts = value.split(',');
    let width: usize = parts.next()?.parse().ok()?;
    let height: usize = parts.next()?.parse().ok()?;
    Some(size((width as i32).into(), (height as i32).into()))
}

struct DisconnectedOverlay;

impl Element for DisconnectedOverlay {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut background = cx.theme().colors().elevated_surface_background;
        background.fade_out(0.2);
        let mut overlay = div()
            .bg(background)
            .absolute()
            .left_0()
            .top(ui::TitleBar::height(cx))
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .capture_any_mouse_down(|_, cx| cx.stop_propagation())
            .capture_any_mouse_up(|_, cx| cx.stop_propagation())
            .child(Label::new(
                "Your connection to the remote project has been lost.",
            ))
            .into_any();
        (overlay.request_layout(cx), overlay)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        overlay: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) {
        cx.insert_hitbox(bounds, true);
        overlay.prepaint(cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        overlay: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        overlay.paint(cx)
    }
}

impl IntoElement for DisconnectedOverlay {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::{
        dock::{test::TestPanel, PanelEvent},
        item::{
            test::{TestItem, TestProjectItem},
            ItemEvent,
        },
    };
    use fs::FakeFs;
    use gpui::{
        px, DismissEvent, Empty, EventEmitter, FocusHandle, FocusableView, Render, TestAppContext,
        UpdateGlobal, VisualTestContext,
    };
    use project::{Project, ProjectEntryId};
    use serde_json::json;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_tab_disambiguation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        // Adding an item with no ambiguity renders the tab without detail.
        let item1 = cx.new_view(|cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b1/c", "a/b1/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item1.clone()), None, cx);
        });
        item1.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(0)));

        // Adding an item that creates ambiguity increases the level of detail on
        // both tabs.
        let item2 = cx.new_view(|cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item2.clone()), None, cx);
        });
        item1.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));
        item2.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));

        // Adding an item that creates ambiguity increases the level of detail only
        // on the ambiguous tabs. In this case, the ambiguity can't be resolved so
        // we stop at the highest detail available.
        let item3 = cx.new_view(|cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item3.clone()), None, cx);
        });
        item1.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));
        item2.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(3)));
        item3.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(3)));
    }

    #[gpui::test]
    async fn test_tracking_active_path(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root1",
            json!({
                "one.txt": "",
                "two.txt": "",
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "three.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs, ["root1".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees().next().unwrap().read(cx).id()
        });

        let item1 = cx.new_view(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
        });
        let item2 = cx.new_view(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(2, "two.txt", cx)])
        });

        // Add an item to an empty pane
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item1), None, cx)
        });
        project.update(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "one.txt").into(), cx)
                    .map(|e| e.id)
            );
        });
        assert_eq!(cx.window_title().as_deref(), Some("one.txt — root1"));

        // Add a second item to a non-empty pane
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item2), None, cx)
        });
        assert_eq!(cx.window_title().as_deref(), Some("two.txt — root1"));
        project.update(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "two.txt").into(), cx)
                    .map(|e| e.id)
            );
        });

        // Close the active item
        pane.update(cx, |pane, cx| {
            pane.close_active_item(&Default::default(), cx).unwrap()
        })
        .await
        .unwrap();
        assert_eq!(cx.window_title().as_deref(), Some("one.txt — root1"));
        project.update(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "one.txt").into(), cx)
                    .map(|e| e.id)
            );
        });

        // Add a project folder
        project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("root2", true, cx)
            })
            .await
            .unwrap();
        assert_eq!(cx.window_title().as_deref(), Some("one.txt — root1, root2"));

        // Remove a project folder
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        assert_eq!(cx.window_title().as_deref(), Some("one.txt — root2"));
    }

    #[gpui::test]
    async fn test_close_window(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "one": "" })).await;

        let project = Project::test(fs, ["root".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        // When there are no dirty items, there's nothing to do.
        let item1 = cx.new_view(|cx| TestItem::new(cx));
        workspace.update(cx, |w, cx| {
            w.add_item_to_active_pane(Box::new(item1.clone()), None, cx)
        });
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(false, cx));
        assert!(task.await.unwrap());

        // When there are dirty untitled items, prompt to save each one. If the user
        // cancels any prompt, then abort.
        let item2 = cx.new_view(|cx| TestItem::new(cx).with_dirty(true));
        let item3 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        workspace.update(cx, |w, cx| {
            w.add_item_to_active_pane(Box::new(item2.clone()), None, cx);
            w.add_item_to_active_pane(Box::new(item3.clone()), None, cx);
        });
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(false, cx));
        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2); // cancel save all
        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2); // cancel save all
        cx.executor().run_until_parked();
        assert!(!cx.has_pending_prompt());
        assert!(!task.await.unwrap());
    }

    #[gpui::test]
    async fn test_close_pane_items(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let item1 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item2 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)])
        });
        let item3 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(3, "3.txt", cx)])
        });
        let item4 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new_untitled(cx)])
        });
        let pane = workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item1.clone()), None, cx);
            workspace.add_item_to_active_pane(Box::new(item2.clone()), None, cx);
            workspace.add_item_to_active_pane(Box::new(item3.clone()), None, cx);
            workspace.add_item_to_active_pane(Box::new(item4.clone()), None, cx);
            workspace.active_pane().clone()
        });

        let close_items = pane.update(cx, |pane, cx| {
            pane.activate_item(1, true, true, cx);
            assert_eq!(pane.active_item().unwrap().item_id(), item2.item_id());
            let item1_id = item1.item_id();
            let item3_id = item3.item_id();
            let item4_id = item4.item_id();
            pane.close_items(cx, SaveIntent::Close, move |id| {
                [item1_id, item3_id, item4_id].contains(&id)
            })
        });
        cx.executor().run_until_parked();

        assert!(cx.has_pending_prompt());
        // Ignore "Save all" prompt
        cx.simulate_prompt_answer(2);
        cx.executor().run_until_parked();
        // There's a prompt to save item 1.
        pane.update(cx, |pane, _| {
            assert_eq!(pane.items_len(), 4);
            assert_eq!(pane.active_item().unwrap().item_id(), item1.item_id());
        });
        // Confirm saving item 1.
        cx.simulate_prompt_answer(0);
        cx.executor().run_until_parked();

        // Item 1 is saved. There's a prompt to save item 3.
        pane.update(cx, |pane, cx| {
            assert_eq!(item1.read(cx).save_count, 1);
            assert_eq!(item1.read(cx).save_as_count, 0);
            assert_eq!(item1.read(cx).reload_count, 0);
            assert_eq!(pane.items_len(), 3);
            assert_eq!(pane.active_item().unwrap().item_id(), item3.item_id());
        });
        assert!(cx.has_pending_prompt());

        // Cancel saving item 3.
        cx.simulate_prompt_answer(1);
        cx.executor().run_until_parked();

        // Item 3 is reloaded. There's a prompt to save item 4.
        pane.update(cx, |pane, cx| {
            assert_eq!(item3.read(cx).save_count, 0);
            assert_eq!(item3.read(cx).save_as_count, 0);
            assert_eq!(item3.read(cx).reload_count, 1);
            assert_eq!(pane.items_len(), 2);
            assert_eq!(pane.active_item().unwrap().item_id(), item4.item_id());
        });
        assert!(cx.has_pending_prompt());

        // Confirm saving item 4.
        cx.simulate_prompt_answer(0);
        cx.executor().run_until_parked();

        // There's a prompt for a path for item 4.
        cx.simulate_new_path_selection(|_| Some(Default::default()));
        close_items.await.unwrap();

        // The requested items are closed.
        pane.update(cx, |pane, cx| {
            assert_eq!(item4.read(cx).save_count, 0);
            assert_eq!(item4.read(cx).save_as_count, 1);
            assert_eq!(item4.read(cx).reload_count, 0);
            assert_eq!(pane.items_len(), 1);
            assert_eq!(pane.active_item().unwrap().item_id(), item2.item_id());
        });
    }

    #[gpui::test]
    async fn test_prompting_to_save_only_on_last_item_for_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        // Create several workspace items with single project entries, and two
        // workspace items with multiple project entries.
        let single_entry_items = (0..=4)
            .map(|project_entry_id| {
                cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_dirty(true)
                        .with_project_items(&[TestProjectItem::new(
                            project_entry_id,
                            &format!("{project_entry_id}.txt"),
                            cx,
                        )])
                })
            })
            .collect::<Vec<_>>();
        let item_2_3 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_project_items(&[
                    single_entry_items[2].read(cx).project_items[0].clone(),
                    single_entry_items[3].read(cx).project_items[0].clone(),
                ])
        });
        let item_3_4 = cx.new_view(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_project_items(&[
                    single_entry_items[3].read(cx).project_items[0].clone(),
                    single_entry_items[4].read(cx).project_items[0].clone(),
                ])
        });

        // Create two panes that contain the following project entries:
        //   left pane:
        //     multi-entry items:   (2, 3)
        //     single-entry items:  0, 1, 2, 3, 4
        //   right pane:
        //     single-entry items:  1
        //     multi-entry items:   (3, 4)
        let left_pane = workspace.update(cx, |workspace, cx| {
            let left_pane = workspace.active_pane().clone();
            workspace.add_item_to_active_pane(Box::new(item_2_3.clone()), None, cx);
            for item in single_entry_items {
                workspace.add_item_to_active_pane(Box::new(item), None, cx);
            }
            left_pane.update(cx, |pane, cx| {
                pane.activate_item(2, true, true, cx);
            });

            let right_pane = workspace
                .split_and_clone(left_pane.clone(), SplitDirection::Right, cx)
                .unwrap();

            right_pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(item_3_4.clone()), true, true, None, cx);
            });

            left_pane
        });

        cx.focus_view(&left_pane);

        // When closing all of the items in the left pane, we should be prompted twice:
        // once for project entry 0, and once for project entry 2. Project entries 1,
        // 3, and 4 are all still open in the other paten. After those two
        // prompts, the task should complete.

        let close = left_pane.update(cx, |pane, cx| {
            pane.close_all_items(&CloseAllItems::default(), cx).unwrap()
        });
        cx.executor().run_until_parked();

        // Discard "Save all" prompt
        cx.simulate_prompt_answer(2);

        cx.executor().run_until_parked();
        left_pane.update(cx, |pane, cx| {
            assert_eq!(
                pane.active_item().unwrap().project_entry_ids(cx).as_slice(),
                &[ProjectEntryId::from_proto(0)]
            );
        });
        cx.simulate_prompt_answer(0);

        cx.executor().run_until_parked();
        left_pane.update(cx, |pane, cx| {
            assert_eq!(
                pane.active_item().unwrap().project_entry_ids(cx).as_slice(),
                &[ProjectEntryId::from_proto(2)]
            );
        });
        cx.simulate_prompt_answer(0);

        cx.executor().run_until_parked();
        close.await.unwrap();
        left_pane.update(cx, |pane, _| {
            assert_eq!(pane.items_len(), 0);
        });
    }

    #[gpui::test]
    async fn test_autosave(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        let item = cx.new_view(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item_id = item.entity_id();
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, cx);
        });

        // Autosave on window change.
        item.update(cx, |item, cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnWindowChange);
                })
            });
            item.is_dirty = true;
        });

        // Deactivating the window saves the file.
        cx.deactivate_window();
        item.update(cx, |item, _| assert_eq!(item.save_count, 1));

        // Re-activating the window doesn't save the file.
        cx.update(|cx| cx.activate_window());
        cx.executor().run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 1));

        // Autosave on focus change.
        item.update(cx, |item, cx| {
            cx.focus_self();
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnFocusChange);
                })
            });
            item.is_dirty = true;
        });

        // Blurring the item saves the file.
        item.update(cx, |_, cx| cx.blur());
        cx.executor().run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 2));

        // Deactivating the window still saves the file.
        item.update(cx, |item, cx| {
            cx.focus_self();
            item.is_dirty = true;
        });
        cx.deactivate_window();
        item.update(cx, |item, _| assert_eq!(item.save_count, 3));

        // Autosave after delay.
        item.update(cx, |item, cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::AfterDelay { milliseconds: 500 });
                })
            });
            item.is_dirty = true;
            cx.emit(ItemEvent::Edit);
        });

        // Delay hasn't fully expired, so the file is still dirty and unsaved.
        cx.executor().advance_clock(Duration::from_millis(250));
        item.update(cx, |item, _| assert_eq!(item.save_count, 3));

        // After delay expires, the file is saved.
        cx.executor().advance_clock(Duration::from_millis(250));
        item.update(cx, |item, _| assert_eq!(item.save_count, 4));

        // Autosave on focus change, ensuring closing the tab counts as such.
        item.update(cx, |item, cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnFocusChange);
                })
            });
            item.is_dirty = true;
        });

        pane.update(cx, |pane, cx| {
            pane.close_items(cx, SaveIntent::Close, move |id| id == item_id)
        })
        .await
        .unwrap();
        assert!(!cx.has_pending_prompt());
        item.update(cx, |item, _| assert_eq!(item.save_count, 5));

        // Add the item again, ensuring autosave is prevented if the underlying file has been deleted.
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, cx);
        });
        item.update(cx, |item, cx| {
            item.project_items[0].update(cx, |item, _| {
                item.entry_id = None;
            });
            item.is_dirty = true;
            cx.blur();
        });
        cx.run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 5));

        // Ensure autosave is prevented for deleted files also when closing the buffer.
        let _close_items = pane.update(cx, |pane, cx| {
            pane.close_items(cx, SaveIntent::Close, move |id| id == item_id)
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        item.update(cx, |item, _| assert_eq!(item.save_count, 5));
    }

    #[gpui::test]
    async fn test_pane_navigation(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let item = cx.new_view(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        let toolbar = pane.update(cx, |pane, _| pane.toolbar().clone());
        let toolbar_notify_count = Rc::new(RefCell::new(0));

        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, cx);
            let toolbar_notification_count = toolbar_notify_count.clone();
            cx.observe(&toolbar, move |_, _, _| {
                *toolbar_notification_count.borrow_mut() += 1
            })
            .detach();
        });

        pane.update(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        item.update(cx, |item, cx| {
            item.set_state("one".to_string(), cx);
        });

        // Toolbar must be notified to re-render the navigation buttons
        assert_eq!(*toolbar_notify_count.borrow(), 1);

        pane.update(cx, |pane, _| {
            assert!(pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        workspace
            .update(cx, |workspace, cx| workspace.go_back(pane.downgrade(), cx))
            .await
            .unwrap();

        assert_eq!(*toolbar_notify_count.borrow(), 2);
        pane.update(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(pane.can_navigate_forward());
        });
    }

    #[gpui::test]
    async fn test_toggle_docks_and_panels(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let panel = workspace.update(cx, |workspace, cx| {
            let panel = cx.new_view(|cx| TestPanel::new(DockPosition::Right, cx));
            workspace.add_panel(panel.clone(), cx);

            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, cx));

            panel
        });

        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        pane.update(cx, |pane, cx| {
            let item = cx.new_view(|cx| TestItem::new(cx));
            pane.add_item(Box::new(item), true, true, None, cx);
        });

        // Transfer focus from center to panel
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Transfer focus from panel to center
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Close the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Open the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Focus and zoom panel
        panel.update(cx, |panel, cx| {
            cx.focus_self();
            panel.set_zoomed(true, cx)
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Transfer focus to the center closes the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Transferring focus back to the panel keeps it zoomed
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Close the dock while it is zoomed
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });

        workspace.update(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(workspace.zoomed.is_none());
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Opening the dock, when it's zoomed, retains focus
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(workspace.zoomed.is_some());
            assert!(panel.read(cx).focus_handle(cx).contains_focused(cx));
        });

        // Unzoom and close the panel, zoom the active pane.
        panel.update(cx, |panel, cx| panel.set_zoomed(false, cx));
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });
        pane.update(cx, |pane, cx| pane.toggle_zoom(&Default::default(), cx));

        // Opening a dock unzooms the pane.
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });
        workspace.update(cx, |workspace, cx| {
            let pane = pane.read(cx);
            assert!(!pane.is_zoomed());
            assert!(!pane.focus_handle(cx).is_focused(cx));
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(workspace.zoomed.is_none());
        });
    }

    struct TestModal(FocusHandle);

    impl TestModal {
        fn new(cx: &mut ViewContext<Self>) -> Self {
            Self(cx.focus_handle())
        }
    }

    impl EventEmitter<DismissEvent> for TestModal {}

    impl FocusableView for TestModal {
        fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
            self.0.clone()
        }
    }

    impl ModalView for TestModal {}

    impl Render for TestModal {
        fn render(&mut self, _cx: &mut ViewContext<TestModal>) -> impl IntoElement {
            div().track_focus(&self.0)
        }
    }

    #[gpui::test]
    async fn test_panels(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let (panel_1, panel_2) = workspace.update(cx, |workspace, cx| {
            let panel_1 = cx.new_view(|cx| TestPanel::new(DockPosition::Left, cx));
            workspace.add_panel(panel_1.clone(), cx);
            workspace
                .left_dock()
                .update(cx, |left_dock, cx| left_dock.set_open(true, cx));
            let panel_2 = cx.new_view(|cx| TestPanel::new(DockPosition::Right, cx));
            workspace.add_panel(panel_2.clone(), cx);
            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, cx));

            let left_dock = workspace.left_dock();
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id()
            );
            assert_eq!(
                left_dock.read(cx).active_panel_size(cx).unwrap(),
                panel_1.size(cx)
            );

            left_dock.update(cx, |left_dock, cx| {
                left_dock.resize_active_panel(Some(px(1337.)), cx)
            });
            assert_eq!(
                workspace
                    .right_dock()
                    .read(cx)
                    .visible_panel()
                    .unwrap()
                    .panel_id(),
                panel_2.panel_id(),
            );

            (panel_1, panel_2)
        });

        // Move panel_1 to the right
        panel_1.update(cx, |panel_1, cx| {
            panel_1.set_position(DockPosition::Right, cx)
        });

        workspace.update(cx, |workspace, cx| {
            // Since panel_1 was visible on the left, it should now be visible now that it's been moved to the right.
            // Since it was the only panel on the left, the left dock should now be closed.
            assert!(!workspace.left_dock().read(cx).is_open());
            assert!(workspace.left_dock().read(cx).visible_panel().is_none());
            let right_dock = workspace.right_dock();
            assert_eq!(
                right_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id()
            );
            assert_eq!(
                right_dock.read(cx).active_panel_size(cx).unwrap(),
                px(1337.)
            );

            // Now we move panel_2 to the left
            panel_2.set_position(DockPosition::Left, cx);
        });

        workspace.update(cx, |workspace, cx| {
            // Since panel_2 was not visible on the right, we don't open the left dock.
            assert!(!workspace.left_dock().read(cx).is_open());
            // And the right dock is unaffected in its displaying of panel_1
            assert!(workspace.right_dock().read(cx).is_open());
            assert_eq!(
                workspace
                    .right_dock()
                    .read(cx)
                    .visible_panel()
                    .unwrap()
                    .panel_id(),
                panel_1.panel_id(),
            );
        });

        // Move panel_1 back to the left
        panel_1.update(cx, |panel_1, cx| {
            panel_1.set_position(DockPosition::Left, cx)
        });

        workspace.update(cx, |workspace, cx| {
            // Since panel_1 was visible on the right, we open the left dock and make panel_1 active.
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id()
            );
            assert_eq!(left_dock.read(cx).active_panel_size(cx).unwrap(), px(1337.));
            // And the right dock should be closed as it no longer has any panels.
            assert!(!workspace.right_dock().read(cx).is_open());

            // Now we move panel_1 to the bottom
            panel_1.set_position(DockPosition::Bottom, cx);
        });

        workspace.update(cx, |workspace, cx| {
            // Since panel_1 was visible on the left, we close the left dock.
            assert!(!workspace.left_dock().read(cx).is_open());
            // The bottom dock is sized based on the panel's default size,
            // since the panel orientation changed from vertical to horizontal.
            let bottom_dock = workspace.bottom_dock();
            assert_eq!(
                bottom_dock.read(cx).active_panel_size(cx).unwrap(),
                panel_1.size(cx),
            );
            // Close bottom dock and move panel_1 back to the left.
            bottom_dock.update(cx, |bottom_dock, cx| bottom_dock.set_open(false, cx));
            panel_1.set_position(DockPosition::Left, cx);
        });

        // Emit activated event on panel 1
        panel_1.update(cx, |_, cx| cx.emit(PanelEvent::Activate));

        // Now the left dock is open and panel_1 is active and focused.
        workspace.update(cx, |workspace, cx| {
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id(),
            );
            assert!(panel_1.focus_handle(cx).is_focused(cx));
        });

        // Emit closed event on panel 2, which is not active
        panel_2.update(cx, |_, cx| cx.emit(PanelEvent::Close));

        // Wo don't close the left dock, because panel_2 wasn't the active panel
        workspace.update(cx, |workspace, cx| {
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id(),
            );
        });

        // Emitting a ZoomIn event shows the panel as zoomed.
        panel_1.update(cx, |_, cx| cx.emit(PanelEvent::ZoomIn));
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.to_any().downgrade()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Left));
        });

        // Move panel to another dock while it is zoomed
        panel_1.update(cx, |panel, cx| panel.set_position(DockPosition::Right, cx));
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.to_any().downgrade()));

            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // This is a helper for getting a:
        // - valid focus on an element,
        // - that isn't a part of the panes and panels system of the Workspace,
        // - and doesn't trigger the 'on_focus_lost' API.
        let focus_other_view = {
            let workspace = workspace.clone();
            move |cx: &mut VisualTestContext| {
                workspace.update(cx, |workspace, cx| {
                    if let Some(_) = workspace.active_modal::<TestModal>(cx) {
                        workspace.toggle_modal(cx, TestModal::new);
                        workspace.toggle_modal(cx, TestModal::new);
                    } else {
                        workspace.toggle_modal(cx, TestModal::new);
                    }
                })
            }
        };

        // If focus is transferred to another view that's not a panel or another pane, we still show
        // the panel as zoomed.
        focus_other_view(cx);
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.to_any().downgrade()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // If focus is transferred elsewhere in the workspace, the panel is no longer zoomed.
        workspace.update(cx, |_, cx| cx.focus_self());
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // If focus is transferred again to another view that's not a panel or a pane, we won't
        // show the panel as zoomed because it wasn't zoomed before.
        focus_other_view(cx);
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // When the panel is activated, it is zoomed again.
        cx.dispatch_action(ToggleRightDock);
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.to_any().downgrade()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // Emitting a ZoomOut event unzooms the panel.
        panel_1.update(cx, |_, cx| cx.emit(PanelEvent::ZoomOut));
        workspace.update(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // Emit closed event on panel 1, which is active
        panel_1.update(cx, |_, cx| cx.emit(PanelEvent::Close));

        // Now the left dock is closed, because panel_1 was the active panel
        workspace.update(cx, |workspace, cx| {
            let right_dock = workspace.right_dock();
            assert!(!right_dock.read(cx).is_open());
        });
    }

    mod register_project_item_tests {
        use ui::Context as _;

        use super::*;

        const TEST_PNG_KIND: &str = "TestPngItemView";
        // View
        struct TestPngItemView {
            focus_handle: FocusHandle,
        }
        // Model
        struct TestPngItem {}

        impl project::Item for TestPngItem {
            fn try_open(
                _project: &Model<Project>,
                path: &ProjectPath,
                cx: &mut AppContext,
            ) -> Option<Task<gpui::Result<Model<Self>>>> {
                if path.path.extension().unwrap() == "png" {
                    Some(cx.spawn(|mut cx| async move { cx.new_model(|_| TestPngItem {}) }))
                } else {
                    None
                }
            }

            fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
                None
            }

            fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
                None
            }
        }

        impl Item for TestPngItemView {
            type Event = ();

            fn serialized_item_kind() -> Option<&'static str> {
                Some(TEST_PNG_KIND)
            }
        }
        impl EventEmitter<()> for TestPngItemView {}
        impl FocusableView for TestPngItemView {
            fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestPngItemView {
            fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestPngItemView {
            type Item = TestPngItem;

            fn for_project_item(
                _project: Model<Project>,
                _item: Model<Self::Item>,
                cx: &mut ViewContext<Self>,
            ) -> Self
            where
                Self: Sized,
            {
                Self {
                    focus_handle: cx.focus_handle(),
                }
            }
        }

        const TEST_IPYNB_KIND: &str = "TestIpynbItemView";
        // View
        struct TestIpynbItemView {
            focus_handle: FocusHandle,
        }
        // Model
        struct TestIpynbItem {}

        impl project::Item for TestIpynbItem {
            fn try_open(
                _project: &Model<Project>,
                path: &ProjectPath,
                cx: &mut AppContext,
            ) -> Option<Task<gpui::Result<Model<Self>>>> {
                if path.path.extension().unwrap() == "ipynb" {
                    Some(cx.spawn(|mut cx| async move { cx.new_model(|_| TestIpynbItem {}) }))
                } else {
                    None
                }
            }

            fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
                None
            }

            fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
                None
            }
        }

        impl Item for TestIpynbItemView {
            type Event = ();

            fn serialized_item_kind() -> Option<&'static str> {
                Some(TEST_IPYNB_KIND)
            }
        }
        impl EventEmitter<()> for TestIpynbItemView {}
        impl FocusableView for TestIpynbItemView {
            fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestIpynbItemView {
            fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestIpynbItemView {
            type Item = TestIpynbItem;

            fn for_project_item(
                _project: Model<Project>,
                _item: Model<Self::Item>,
                cx: &mut ViewContext<Self>,
            ) -> Self
            where
                Self: Sized,
            {
                Self {
                    focus_handle: cx.focus_handle(),
                }
            }
        }

        struct TestAlternatePngItemView {
            focus_handle: FocusHandle,
        }

        const TEST_ALTERNATE_PNG_KIND: &str = "TestAlternatePngItemView";
        impl Item for TestAlternatePngItemView {
            type Event = ();

            fn serialized_item_kind() -> Option<&'static str> {
                Some(TEST_ALTERNATE_PNG_KIND)
            }
        }
        impl EventEmitter<()> for TestAlternatePngItemView {}
        impl FocusableView for TestAlternatePngItemView {
            fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestAlternatePngItemView {
            fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestAlternatePngItemView {
            type Item = TestPngItem;

            fn for_project_item(
                _project: Model<Project>,
                _item: Model<Self::Item>,
                cx: &mut ViewContext<Self>,
            ) -> Self
            where
                Self: Sized,
            {
                Self {
                    focus_handle: cx.focus_handle(),
                }
            }
        }

        #[gpui::test]
        async fn test_register_project_item(cx: &mut TestAppContext) {
            init_test(cx);

            cx.update(|cx| {
                register_project_item::<TestPngItemView>(cx);
                register_project_item::<TestIpynbItemView>(cx);
            });

            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                "/root1",
                json!({
                    "one.png": "BINARYDATAHERE",
                    "two.ipynb": "{ totally a notebook }",
                    "three.txt": "editing text, sure why not?"
                }),
            )
            .await;

            let project = Project::test(fs, ["root1".as_ref()], cx).await;
            let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

            let worktree_id = project.update(cx, |project, cx| {
                project.worktrees().next().unwrap().read(cx).id()
            });

            let handle = workspace
                .update(cx, |workspace, cx| {
                    let project_path = (worktree_id, "one.png");
                    workspace.open_path(project_path, None, true, cx)
                })
                .await
                .unwrap();

            // Now we can check if the handle we got back errored or not
            assert_eq!(handle.serialized_item_kind().unwrap(), TEST_PNG_KIND);

            let handle = workspace
                .update(cx, |workspace, cx| {
                    let project_path = (worktree_id, "two.ipynb");
                    workspace.open_path(project_path, None, true, cx)
                })
                .await
                .unwrap();

            assert_eq!(handle.serialized_item_kind().unwrap(), TEST_IPYNB_KIND);

            let handle = workspace
                .update(cx, |workspace, cx| {
                    let project_path = (worktree_id, "three.txt");
                    workspace.open_path(project_path, None, true, cx)
                })
                .await;
            assert!(handle.is_err());
        }

        #[gpui::test]
        async fn test_register_project_item_two_enter_one_leaves(cx: &mut TestAppContext) {
            init_test(cx);

            cx.update(|cx| {
                register_project_item::<TestPngItemView>(cx);
                register_project_item::<TestAlternatePngItemView>(cx);
            });

            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                "/root1",
                json!({
                    "one.png": "BINARYDATAHERE",
                    "two.ipynb": "{ totally a notebook }",
                    "three.txt": "editing text, sure why not?"
                }),
            )
            .await;

            let project = Project::test(fs, ["root1".as_ref()], cx).await;
            let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

            let worktree_id = project.update(cx, |project, cx| {
                project.worktrees().next().unwrap().read(cx).id()
            });

            let handle = workspace
                .update(cx, |workspace, cx| {
                    let project_path = (worktree_id, "one.png");
                    workspace.open_path(project_path, None, true, cx)
                })
                .await
                .unwrap();

            // This _must_ be the second item registered
            assert_eq!(
                handle.serialized_item_kind().unwrap(),
                TEST_ALTERNATE_PNG_KIND
            );

            let handle = workspace
                .update(cx, |workspace, cx| {
                    let project_path = (worktree_id, "three.txt");
                    workspace.open_path(project_path, None, true, cx)
                })
                .await;
            assert!(handle.is_err());
        }
    }

    pub fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            crate::init_settings(cx);
            Project::init_settings(cx);
        });
    }
}
