pub mod dock;
pub mod history_manager;
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
mod theme_preview;
mod toast_layer;
mod toolbar;
mod workspace_settings;

pub use toast_layer::{RunAction, ToastAction, ToastLayer, ToastView};

use anyhow::{Context as _, Result, anyhow};
use call::{ActiveCall, call_settings::CallSettings};
use client::{
    ChannelId, Client, ErrorExt, Status, TypedEnvelope, UserStore,
    proto::{self, ErrorCode, PanelId, PeerId},
};
use collections::{HashMap, HashSet, hash_map};
pub use dock::Panel;
use dock::{Dock, DockPosition, PanelButtons, PanelHandle, RESIZE_HANDLE_SIZE};
use futures::{
    Future, FutureExt, StreamExt,
    channel::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::try_join_all,
};
use gpui::{
    Action, AnyEntity, AnyView, AnyWeakView, App, AsyncApp, AsyncWindowContext, Bounds, Context,
    CursorStyle, Decorations, DragMoveEvent, Entity, EntityId, EventEmitter, FocusHandle,
    Focusable, Global, Hsla, KeyContext, Keystroke, ManagedView, MouseButton, PathPromptOptions,
    Point, PromptLevel, Render, ResizeEdge, Size, Stateful, Subscription, Task, Tiling, WeakEntity,
    WindowBounds, WindowHandle, WindowId, WindowOptions, action_as, actions, canvas,
    impl_action_as, impl_actions, point, relative, size, transparent_black,
};
pub use history_manager::*;
pub use item::{
    FollowableItem, FollowableItemHandle, Item, ItemHandle, ItemSettings, PreviewTabsSettings,
    ProjectItem, SerializableItem, SerializableItemHandle, WeakItemHandle,
};
use itertools::Itertools;
use language::{Buffer, LanguageRegistry, Rope};
pub use modal_layer::*;
use node_runtime::NodeRuntime;
use notifications::{
    DetachAndPromptErr, Notifications, dismiss_app_notification,
    simple_message_notification::MessageNotification,
};
pub use pane::*;
pub use pane_group::*;
use persistence::{
    DB, SerializedWindowBounds,
    model::{SerializedSshProject, SerializedWorkspace},
};
pub use persistence::{
    DB as WORKSPACE_DB, WorkspaceDb, delete_unloaded_items,
    model::{ItemId, LocalPaths, SerializedWorkspaceLocation},
};
use postage::stream::Stream;
use project::{
    DirectoryLister, Project, ProjectEntryId, ProjectPath, ResolvedPath, Worktree, WorktreeId,
    debugger::breakpoint_store::BreakpointStoreEvent,
};
use remote::{SshClientDelegate, SshConnectionOptions, ssh_session::ConnectionIdentifier};
use schemars::JsonSchema;
use serde::Deserialize;
use session::AppSession;
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
    process::ExitStatus,
    rc::Rc,
    sync::{Arc, LazyLock, Weak, atomic::AtomicUsize},
    time::Duration,
};
use task::{DebugScenario, SpawnInTerminal, TaskContext};
use theme::{ActiveTheme, SystemAppearance, ThemeSettings};
pub use toolbar::{Toolbar, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};
pub use ui;
use ui::prelude::*;
use util::{ResultExt, TryFutureExt, paths::SanitizedPath, serde::default_true};
use uuid::Uuid;
pub use workspace_settings::{
    AutosaveSetting, BottomDockLayout, RestoreOnStartupBehavior, TabBarSettings, WorkspaceSettings,
};
use zed_actions::feedback::FileBugReport;

use crate::notifications::NotificationId;
use crate::persistence::{
    SerializedAxis,
    model::{DockData, DockStructure, SerializedItem, SerializedPane, SerializedPaneGroup},
};

pub const SERIALIZATION_THROTTLE_TIME: Duration = Duration::from_millis(200);

static ZED_WINDOW_SIZE: LazyLock<Option<Size<Pixels>>> = LazyLock::new(|| {
    env::var("ZED_WINDOW_SIZE")
        .ok()
        .as_deref()
        .and_then(parse_pixel_size_env_var)
});

static ZED_WINDOW_POSITION: LazyLock<Option<Point<Pixels>>> = LazyLock::new(|| {
    env::var("ZED_WINDOW_POSITION")
        .ok()
        .as_deref()
        .and_then(parse_pixel_position_env_var)
});

pub trait TerminalProvider {
    fn spawn(
        &self,
        task: SpawnInTerminal,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<Result<ExitStatus>>>;
}

pub trait DebuggerProvider {
    // `active_buffer` is used to resolve build task's name against language-specific tasks.
    fn start_session(
        &self,
        definition: DebugScenario,
        task_context: TaskContext,
        active_buffer: Option<Entity<Buffer>>,
        window: &mut Window,
        cx: &mut App,
    );
}

actions!(
    workspace,
    [
        ActivateNextPane,
        ActivatePreviousPane,
        ActivateNextWindow,
        ActivatePreviousWindow,
        AddFolderToProject,
        ClearAllNotifications,
        CloseActiveDock,
        CloseAllDocks,
        CloseWindow,
        Feedback,
        FollowNextCollaborator,
        MoveFocusedPanelToNextPosition,
        NewCenterTerminal,
        NewFile,
        NewFileSplitVertical,
        NewFileSplitHorizontal,
        NewSearch,
        NewTerminal,
        NewWindow,
        Open,
        OpenFiles,
        OpenInTerminal,
        OpenComponentPreview,
        ReloadActiveItem,
        SaveAs,
        SaveWithoutFormat,
        ShutdownDebugAdapters,
        SuppressNotification,
        ToggleBottomDock,
        ToggleCenteredLayout,
        ToggleLeftDock,
        ToggleRightDock,
        ToggleZoom,
        Unfollow,
        Welcome,
        // todo!("remove")
        Walkthrough,
        OnboardingWalkthrough,
        RestoreBanner,
    ]
);

#[derive(Clone, PartialEq)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Deserialize, PartialEq, JsonSchema)]
pub struct ActivatePane(pub usize);

#[derive(Clone, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveItemToPane {
    pub destination: usize,
    #[serde(default = "default_true")]
    pub focus: bool,
}

#[derive(Clone, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MoveItemToPaneInDirection {
    pub direction: SplitDirection,
    #[serde(default = "default_true")]
    pub focus: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaveAll {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Save {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CloseAllItemsAndPanes {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CloseInactiveTabsAndPanes {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, Deserialize, PartialEq, JsonSchema)]
pub struct SendKeystrokes(pub String);

#[derive(Clone, Deserialize, PartialEq, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Reload {
    pub binary_path: Option<PathBuf>,
}

action_as!(project_symbols, ToggleProjectSymbols as Toggle);

#[derive(Default, PartialEq, Eq, Clone, Deserialize, JsonSchema)]
pub struct ToggleFileFinder {
    #[serde(default)]
    pub separate_history: bool,
}

impl_action_as!(file_finder, ToggleFileFinder as Toggle);

impl_actions!(
    workspace,
    [
        ActivatePane,
        CloseAllItemsAndPanes,
        CloseInactiveTabsAndPanes,
        MoveItemToPane,
        MoveItemToPaneInDirection,
        OpenTerminal,
        Reload,
        Save,
        SaveAll,
        SendKeystrokes,
    ]
);

actions!(
    workspace,
    [
        ActivatePaneLeft,
        ActivatePaneRight,
        ActivatePaneUp,
        ActivatePaneDown,
        SwapPaneLeft,
        SwapPaneRight,
        SwapPaneUp,
        SwapPaneDown,
    ]
);

#[derive(PartialEq, Eq, Debug)]
pub enum CloseIntent {
    /// Quit the program entirely.
    Quit,
    /// Close a window.
    CloseWindow,
    /// Replace the workspace in an existing window.
    ReplaceWindow,
}

#[derive(Clone)]
pub struct Toast {
    id: NotificationId,
    msg: Cow<'static, str>,
    autohide: bool,
    on_click: Option<(Cow<'static, str>, Arc<dyn Fn(&mut Window, &mut App)>)>,
}

impl Toast {
    pub fn new<I: Into<Cow<'static, str>>>(id: NotificationId, msg: I) -> Self {
        Toast {
            id,
            msg: msg.into(),
            on_click: None,
            autohide: false,
        }
    }

    pub fn on_click<F, M>(mut self, message: M, on_click: F) -> Self
    where
        M: Into<Cow<'static, str>>,
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some((message.into(), Arc::new(on_click)));
        self
    }

    pub fn autohide(mut self) -> Self {
        self.autohide = true;
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

#[derive(Debug, Default, Clone, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields)]
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
impl From<WorkspaceId> for i64 {
    fn from(val: WorkspaceId) -> Self {
        val.0
    }
}

pub fn init_settings(cx: &mut App) {
    WorkspaceSettings::register(cx);
    ItemSettings::register(cx);
    PreviewTabsSettings::register(cx);
    TabBarSettings::register(cx);
}

fn prompt_and_open_paths(app_state: Arc<AppState>, options: PathPromptOptions, cx: &mut App) {
    let paths = cx.prompt_for_paths(options);
    cx.spawn(
        async move |cx| match paths.await.anyhow().and_then(|res| res) {
            Ok(Some(paths)) => {
                cx.update(|cx| {
                    open_paths(&paths, app_state, OpenOptions::default(), cx).detach_and_log_err(cx)
                })
                .ok();
            }
            Ok(None) => {}
            Err(err) => {
                util::log_err(&err);
                cx.update(|cx| {
                    if let Some(workspace_window) = cx
                        .active_window()
                        .and_then(|window| window.downcast::<Workspace>())
                    {
                        workspace_window
                            .update(cx, |workspace, _, cx| {
                                workspace.show_portal_error(err.to_string(), cx);
                            })
                            .ok();
                    }
                })
                .ok();
            }
        },
    )
    .detach();
}

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    init_settings(cx);
    component::init();
    theme_preview::init(cx);
    toast_layer::init(cx);
    history_manager::init(cx);

    cx.on_action(Workspace::close_global);
    cx.on_action(reload);

    cx.on_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &Open, cx: &mut App| {
            if let Some(app_state) = app_state.upgrade() {
                prompt_and_open_paths(
                    app_state,
                    PathPromptOptions {
                        files: true,
                        directories: true,
                        multiple: true,
                    },
                    cx,
                );
            }
        }
    });
    cx.on_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &OpenFiles, cx: &mut App| {
            let directories = cx.can_select_mixed_files_and_dirs();
            if let Some(app_state) = app_state.upgrade() {
                prompt_and_open_paths(
                    app_state,
                    PathPromptOptions {
                        files: true,
                        directories,
                        multiple: true,
                    },
                    cx,
                );
            }
        }
    });
}

type BuildProjectItemFn =
    fn(AnyEntity, Entity<Project>, Option<&Pane>, &mut Window, &mut App) -> Box<dyn ItemHandle>;

type BuildProjectItemForPathFn =
    fn(
        &Entity<Project>,
        &ProjectPath,
        &mut Window,
        &mut App,
    ) -> Option<Task<Result<(Option<ProjectEntryId>, WorkspaceItemBuilder)>>>;

#[derive(Clone, Default)]
struct ProjectItemRegistry {
    build_project_item_fns_by_type: HashMap<TypeId, BuildProjectItemFn>,
    build_project_item_for_path_fns: Vec<BuildProjectItemForPathFn>,
}

impl ProjectItemRegistry {
    fn register<T: ProjectItem>(&mut self) {
        self.build_project_item_fns_by_type.insert(
            TypeId::of::<T::Item>(),
            |item, project, pane, window, cx| {
                let item = item.downcast().unwrap();
                Box::new(cx.new(|cx| T::for_project_item(project, pane, item, window, cx)))
                    as Box<dyn ItemHandle>
            },
        );
        self.build_project_item_for_path_fns
            .push(|project, project_path, window, cx| {
                let project_item =
                    <T::Item as project::ProjectItem>::try_open(project, project_path, cx)?;
                let project = project.clone();
                Some(window.spawn(cx, async move |cx| {
                    let project_item = project_item.await?;
                    let project_entry_id: Option<ProjectEntryId> =
                        project_item.read_with(cx, project::ProjectItem::entry_id)?;
                    let build_workspace_item = Box::new(
                        |pane: &mut Pane, window: &mut Window, cx: &mut Context<Pane>| {
                            Box::new(cx.new(|cx| {
                                T::for_project_item(project, Some(pane), project_item, window, cx)
                            })) as Box<dyn ItemHandle>
                        },
                    ) as Box<_>;
                    Ok((project_entry_id, build_workspace_item))
                }))
            });
    }

    fn open_path(
        &self,
        project: &Entity<Project>,
        path: &ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<(Option<ProjectEntryId>, WorkspaceItemBuilder)>> {
        let Some(open_project_item) = self
            .build_project_item_for_path_fns
            .iter()
            .rev()
            .find_map(|open_project_item| open_project_item(&project, &path, window, cx))
        else {
            return Task::ready(Err(anyhow!("cannot open file {:?}", path.path)));
        };
        open_project_item
    }

    fn build_item<T: project::ProjectItem>(
        &self,
        item: Entity<T>,
        project: Entity<Project>,
        pane: Option<&Pane>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Box<dyn ItemHandle>> {
        let build = self
            .build_project_item_fns_by_type
            .get(&TypeId::of::<T>())?;
        Some(build(item.into_any(), project, pane, window, cx))
    }
}

type WorkspaceItemBuilder =
    Box<dyn FnOnce(&mut Pane, &mut Window, &mut Context<Pane>) -> Box<dyn ItemHandle>>;

impl Global for ProjectItemRegistry {}

/// Registers a [ProjectItem] for the app. When opening a file, all the registered
/// items will get a chance to open the file, starting from the project item that
/// was added last.
pub fn register_project_item<I: ProjectItem>(cx: &mut App) {
    cx.default_global::<ProjectItemRegistry>().register::<I>();
}

#[derive(Default)]
pub struct FollowableViewRegistry(HashMap<TypeId, FollowableViewDescriptor>);

struct FollowableViewDescriptor {
    from_state_proto: fn(
        Entity<Workspace>,
        ViewId,
        &mut Option<proto::view::Variant>,
        &mut Window,
        &mut App,
    ) -> Option<Task<Result<Box<dyn FollowableItemHandle>>>>,
    to_followable_view: fn(&AnyView) -> Box<dyn FollowableItemHandle>,
}

impl Global for FollowableViewRegistry {}

impl FollowableViewRegistry {
    pub fn register<I: FollowableItem>(cx: &mut App) {
        cx.default_global::<Self>().0.insert(
            TypeId::of::<I>(),
            FollowableViewDescriptor {
                from_state_proto: |workspace, id, state, window, cx| {
                    I::from_state_proto(workspace, id, state, window, cx).map(|task| {
                        cx.foreground_executor()
                            .spawn(async move { Ok(Box::new(task.await?) as Box<_>) })
                    })
                },
                to_followable_view: |view| Box::new(view.clone().downcast::<I>().unwrap()),
            },
        );
    }

    pub fn from_state_proto(
        workspace: Entity<Workspace>,
        view_id: ViewId,
        mut state: Option<proto::view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Box<dyn FollowableItemHandle>>>> {
        cx.update_default_global(|this: &mut Self, cx| {
            this.0.values().find_map(|descriptor| {
                (descriptor.from_state_proto)(workspace.clone(), view_id, &mut state, window, cx)
            })
        })
    }

    pub fn to_followable_view(
        view: impl Into<AnyView>,
        cx: &App,
    ) -> Option<Box<dyn FollowableItemHandle>> {
        let this = cx.try_global::<Self>()?;
        let view = view.into();
        let descriptor = this.0.get(&view.entity_type())?;
        Some((descriptor.to_followable_view)(&view))
    }
}

#[derive(Copy, Clone)]
struct SerializableItemDescriptor {
    deserialize: fn(
        Entity<Project>,
        WeakEntity<Workspace>,
        WorkspaceId,
        ItemId,
        &mut Window,
        &mut Context<Pane>,
    ) -> Task<Result<Box<dyn ItemHandle>>>,
    cleanup: fn(WorkspaceId, Vec<ItemId>, &mut Window, &mut App) -> Task<Result<()>>,
    view_to_serializable_item: fn(AnyView) -> Box<dyn SerializableItemHandle>,
}

#[derive(Default)]
struct SerializableItemRegistry {
    descriptors_by_kind: HashMap<Arc<str>, SerializableItemDescriptor>,
    descriptors_by_type: HashMap<TypeId, SerializableItemDescriptor>,
}

impl Global for SerializableItemRegistry {}

impl SerializableItemRegistry {
    fn deserialize(
        item_kind: &str,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_item: ItemId,
        window: &mut Window,
        cx: &mut Context<Pane>,
    ) -> Task<Result<Box<dyn ItemHandle>>> {
        let Some(descriptor) = Self::descriptor(item_kind, cx) else {
            return Task::ready(Err(anyhow!(
                "cannot deserialize {}, descriptor not found",
                item_kind
            )));
        };

        (descriptor.deserialize)(project, workspace, workspace_id, item_item, window, cx)
    }

    fn cleanup(
        item_kind: &str,
        workspace_id: WorkspaceId,
        loaded_items: Vec<ItemId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(descriptor) = Self::descriptor(item_kind, cx) else {
            return Task::ready(Err(anyhow!(
                "cannot cleanup {}, descriptor not found",
                item_kind
            )));
        };

        (descriptor.cleanup)(workspace_id, loaded_items, window, cx)
    }

    fn view_to_serializable_item_handle(
        view: AnyView,
        cx: &App,
    ) -> Option<Box<dyn SerializableItemHandle>> {
        let this = cx.try_global::<Self>()?;
        let descriptor = this.descriptors_by_type.get(&view.entity_type())?;
        Some((descriptor.view_to_serializable_item)(view))
    }

    fn descriptor(item_kind: &str, cx: &App) -> Option<SerializableItemDescriptor> {
        let this = cx.try_global::<Self>()?;
        this.descriptors_by_kind.get(item_kind).copied()
    }
}

pub fn register_serializable_item<I: SerializableItem>(cx: &mut App) {
    let serialized_item_kind = I::serialized_item_kind();

    let registry = cx.default_global::<SerializableItemRegistry>();
    let descriptor = SerializableItemDescriptor {
        deserialize: |project, workspace, workspace_id, item_id, window, cx| {
            let task = I::deserialize(project, workspace, workspace_id, item_id, window, cx);
            cx.foreground_executor()
                .spawn(async { Ok(Box::new(task.await?) as Box<_>) })
        },
        cleanup: |workspace_id, loaded_items, window, cx| {
            I::cleanup(workspace_id, loaded_items, window, cx)
        },
        view_to_serializable_item: |view| Box::new(view.downcast::<I>().unwrap()),
    };
    registry
        .descriptors_by_kind
        .insert(Arc::from(serialized_item_kind), descriptor);
    registry
        .descriptors_by_type
        .insert(TypeId::of::<I>(), descriptor);
}

pub struct AppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Entity<UserStore>,
    pub workspace_store: Entity<WorkspaceStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub build_window_options: fn(Option<Uuid>, &mut App) -> WindowOptions,
    pub node_runtime: NodeRuntime,
    pub session: Entity<AppSession>,
}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

pub struct WorkspaceStore {
    workspaces: HashSet<WindowHandle<Workspace>>,
    client: Arc<Client>,
    _subscriptions: Vec<client::Subscription>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum CollaboratorId {
    PeerId(PeerId),
    Agent,
}

impl From<PeerId> for CollaboratorId {
    fn from(peer_id: PeerId) -> Self {
        CollaboratorId::PeerId(peer_id)
    }
}

impl From<&PeerId> for CollaboratorId {
    fn from(peer_id: &PeerId) -> Self {
        CollaboratorId::PeerId(*peer_id)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Follower {
    project_id: Option<u64>,
    peer_id: PeerId,
}

impl AppState {
    #[track_caller]
    pub fn global(cx: &App) -> Weak<Self> {
        cx.global::<GlobalAppState>().0.clone()
    }
    pub fn try_global(cx: &App) -> Option<Weak<Self>> {
        cx.try_global::<GlobalAppState>()
            .map(|state| state.0.clone())
    }
    pub fn set_global(state: Weak<AppState>, cx: &mut App) {
        cx.set_global(GlobalAppState(state));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut App) -> Arc<Self> {
        use node_runtime::NodeRuntime;
        use session::Session;
        use settings::SettingsStore;

        if !cx.has_global::<SettingsStore>() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }

        let fs = fs::FakeFs::new(cx.background_executor().clone());
        let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let clock = Arc::new(clock::FakeSystemClock::new());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = Client::new(clock, http_client.clone(), cx);
        let session = cx.new(|cx| AppSession::new(Session::test(), cx));
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));

        theme::init(theme::LoadThemes::JustBase, cx);
        client::init(&client, cx);
        crate::init_settings(cx);

        Arc::new(Self {
            client,
            fs,
            languages,
            user_store,
            workspace_store,
            node_runtime: NodeRuntime::unavailable(),
            build_window_options: |_, _| Default::default(),
            session,
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

    fn fire_new<F>(
        &mut self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<Workspace>,
        func: F,
    ) where
        F: 'static
            + Send
            + FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) -> Task<Result<()>>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            _ = channel.send(());
        }

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        let previous_task = self.task.take();
        self.task = Some(cx.spawn_in(window, async move |workspace, cx| {
            let mut timer = cx.background_executor().timer(delay).fuse();
            if let Some(previous_task) = previous_task {
                previous_task.await;
            }

            futures::select_biased! {
                _ = receiver => return,
                    _ = timer => {}
            }

            if let Some(result) = workspace
                .update_in(cx, |workspace, window, cx| (func)(workspace, window, cx))
                .log_err()
            {
                result.await.log_err();
            }
        }));
    }
}

pub enum Event {
    PaneAdded(Entity<Pane>),
    PaneRemoved,
    ItemAdded {
        item: Box<dyn ItemHandle>,
    },
    ItemRemoved,
    ActiveItemChanged,
    UserSavedItem {
        pane: WeakEntity<Pane>,
        item: Box<dyn WeakItemHandle>,
        save_intent: SaveIntent,
    },
    ContactRequestedJoin(u64),
    WorkspaceCreated(WeakEntity<Workspace>),
    OpenBundledFile {
        text: Cow<'static, str>,
        title: &'static str,
        language: &'static str,
    },
    ZoomChanged,
    ModalOpened,
    ClearActivityIndicator,
}

#[derive(Debug)]
pub enum OpenVisible {
    All,
    None,
    OnlyFiles,
    OnlyDirectories,
}

type PromptForNewPath = Box<
    dyn Fn(
        &mut Workspace,
        &mut Window,
        &mut Context<Workspace>,
    ) -> oneshot::Receiver<Option<ProjectPath>>,
>;

type PromptForOpenPath = Box<
    dyn Fn(
        &mut Workspace,
        DirectoryLister,
        &mut Window,
        &mut Context<Workspace>,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>>,
>;

/// Collects everything project-related for a certain window opened.
/// In some way, is a counterpart of a window, as the [`WindowHandle`] could be downcast into `Workspace`.
///
/// A `Workspace` usually consists of 1 or more projects, a central pane group, 3 docks and a status bar.
/// The `Workspace` owns everybody's state and serves as a default, "global context",
/// that can be used to register a global action to be triggered from any place in the window.
pub struct Workspace {
    weak_self: WeakEntity<Self>,
    workspace_actions: Vec<Box<dyn Fn(Div, &mut Window, &mut Context<Self>) -> Div>>,
    zoomed: Option<AnyWeakView>,
    previous_dock_drag_coordinates: Option<Point<Pixels>>,
    zoomed_position: Option<DockPosition>,
    center: PaneGroup,
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    bottom_dock_layout: BottomDockLayout,
    right_dock: Entity<Dock>,
    panes: Vec<Entity<Pane>>,
    panes_by_item: HashMap<EntityId, WeakEntity<Pane>>,
    active_pane: Entity<Pane>,
    last_active_center_pane: Option<WeakEntity<Pane>>,
    last_active_view_id: Option<proto::ViewId>,
    status_bar: Entity<StatusBar>,
    modal_layer: Entity<ModalLayer>,
    toast_layer: Entity<ToastLayer>,
    titlebar_item: Option<AnyView>,
    notifications: Notifications,
    suppressed_notifications: HashSet<NotificationId>,
    project: Entity<Project>,
    follower_states: HashMap<CollaboratorId, FollowerState>,
    last_leaders_by_pane: HashMap<WeakEntity<Pane>, CollaboratorId>,
    window_edited: bool,
    dirty_items: HashMap<EntityId, Subscription>,
    active_call: Option<(Entity<ActiveCall>, Vec<Subscription>)>,
    leader_updates_tx: mpsc::UnboundedSender<(PeerId, proto::UpdateFollowers)>,
    database_id: Option<WorkspaceId>,
    app_state: Arc<AppState>,
    dispatching_keystrokes: Rc<RefCell<(HashSet<String>, Vec<Keystroke>)>>,
    _subscriptions: Vec<Subscription>,
    _apply_leader_updates: Task<Result<()>>,
    _observe_current_user: Task<Result<()>>,
    _schedule_serialize: Option<Task<()>>,
    pane_history_timestamp: Arc<AtomicUsize>,
    bounds: Bounds<Pixels>,
    pub centered_layout: bool,
    bounds_save_task_queued: Option<Task<()>>,
    on_prompt_for_new_path: Option<PromptForNewPath>,
    on_prompt_for_open_path: Option<PromptForOpenPath>,
    terminal_provider: Option<Box<dyn TerminalProvider>>,
    debugger_provider: Option<Box<dyn DebuggerProvider>>,
    serializable_items_tx: UnboundedSender<Box<dyn SerializableItemHandle>>,
    serialized_ssh_project: Option<SerializedSshProject>,
    _items_serializer: Task<Result<()>>,
    session_id: Option<String>,
}

impl EventEmitter<Event> for Workspace {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewId {
    pub creator: CollaboratorId,
    pub id: u64,
}

pub struct FollowerState {
    center_pane: Entity<Pane>,
    dock_pane: Option<Entity<Pane>>,
    active_view_id: Option<ViewId>,
    items_by_leader_view_id: HashMap<ViewId, FollowerView>,
}

struct FollowerView {
    view: Box<dyn FollowableItemHandle>,
    location: Option<proto::PanelId>,
}

impl Workspace {
    const DEFAULT_PADDING: f32 = 0.2;
    const MAX_PADDING: f32 = 0.4;

    pub fn new(
        workspace_id: Option<WorkspaceId>,
        project: Entity<Project>,
        app_state: Arc<AppState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe_in(&project, window, move |this, _, event, window, cx| {
            match event {
                project::Event::RemoteIdChanged(_) => {
                    this.update_window_title(window, cx);
                }

                project::Event::CollaboratorLeft(peer_id) => {
                    this.collaborator_left(*peer_id, window, cx);
                }

                project::Event::WorktreeRemoved(_) | project::Event::WorktreeAdded(_) => {
                    this.update_window_title(window, cx);
                    this.serialize_workspace(window, cx);
                    // This event could be triggered by `AddFolderToProject` or `RemoveFromProject`.
                    // So we need to update the history.
                    this.update_history(cx);
                }

                project::Event::DisconnectedFromHost => {
                    this.update_window_edited(window, cx);
                    let leaders_to_unfollow =
                        this.follower_states.keys().copied().collect::<Vec<_>>();
                    for leader_id in leaders_to_unfollow {
                        this.unfollow(leader_id, window, cx);
                    }
                }

                project::Event::DisconnectedFromSshRemote => {
                    this.update_window_edited(window, cx);
                }

                project::Event::Closed => {
                    window.remove_window();
                }

                project::Event::DeletedEntry(_, entry_id) => {
                    for pane in this.panes.iter() {
                        pane.update(cx, |pane, cx| {
                            pane.handle_deleted_project_item(*entry_id, window, cx)
                        });
                    }
                }

                project::Event::Toast {
                    notification_id,
                    message,
                } => this.show_notification(
                    NotificationId::named(notification_id.clone()),
                    cx,
                    |cx| cx.new(|cx| MessageNotification::new(message.clone(), cx)),
                ),

                project::Event::HideToast { notification_id } => {
                    this.dismiss_notification(&NotificationId::named(notification_id.clone()), cx)
                }

                project::Event::LanguageServerPrompt(request) => {
                    struct LanguageServerPrompt;

                    let mut hasher = DefaultHasher::new();
                    request.lsp_name.as_str().hash(&mut hasher);
                    let id = hasher.finish();

                    this.show_notification(
                        NotificationId::composite::<LanguageServerPrompt>(id as usize),
                        cx,
                        |cx| {
                            cx.new(|cx| {
                                notifications::LanguageServerPrompt::new(request.clone(), cx)
                            })
                        },
                    );
                }

                project::Event::AgentLocationChanged => {
                    this.handle_agent_location_changed(window, cx)
                }

                _ => {}
            }
            cx.notify()
        })
        .detach();

        cx.subscribe_in(
            &project.read(cx).breakpoint_store(),
            window,
            |workspace, _, event, window, cx| match event {
                BreakpointStoreEvent::BreakpointsUpdated(_, _)
                | BreakpointStoreEvent::BreakpointsCleared(_) => {
                    workspace.serialize_workspace(window, cx);
                }
                BreakpointStoreEvent::SetDebugLine | BreakpointStoreEvent::ClearDebugLines => {}
            },
        )
        .detach();

        cx.on_focus_lost(window, |this, window, cx| {
            let focus_handle = this.focus_handle(cx);
            window.focus(&focus_handle);
        })
        .detach();

        let weak_handle = cx.entity().downgrade();
        let pane_history_timestamp = Arc::new(AtomicUsize::new(0));

        let center_pane = cx.new(|cx| {
            let mut center_pane = Pane::new(
                weak_handle.clone(),
                project.clone(),
                pane_history_timestamp.clone(),
                None,
                NewFile.boxed_clone(),
                window,
                cx,
            );
            center_pane.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            center_pane
        });
        cx.subscribe_in(&center_pane, window, Self::handle_pane_event)
            .detach();

        window.focus(&center_pane.focus_handle(cx));

        cx.emit(Event::PaneAdded(center_pane.clone()));

        let window_handle = window.window_handle().downcast::<Workspace>().unwrap();
        app_state.workspace_store.update(cx, |store, _| {
            store.workspaces.insert(window_handle);
        });

        let mut current_user = app_state.user_store.read(cx).watch_current_user();
        let mut connection_status = app_state.client.status();
        let _observe_current_user = cx.spawn_in(window, async move |this, cx| {
            current_user.next().await;
            connection_status.next().await;
            let mut stream =
                Stream::map(current_user, drop).merge(Stream::map(connection_status, drop));

            while stream.recv().await.is_some() {
                this.update(cx, |_, cx| cx.notify())?;
            }
            anyhow::Ok(())
        });

        // All leader updates are enqueued and then processed in a single task, so
        // that each asynchronous operation can be run in order.
        let (leader_updates_tx, mut leader_updates_rx) =
            mpsc::unbounded::<(PeerId, proto::UpdateFollowers)>();
        let _apply_leader_updates = cx.spawn_in(window, async move |this, cx| {
            while let Some((leader_id, update)) = leader_updates_rx.next().await {
                Self::process_leader_update(&this, leader_id, update, cx)
                    .await
                    .log_err();
            }

            Ok(())
        });

        cx.emit(Event::WorkspaceCreated(weak_handle.clone()));
        let modal_layer = cx.new(|_| ModalLayer::new());
        let toast_layer = cx.new(|_| ToastLayer::new());
        cx.subscribe(
            &modal_layer,
            |_, _, _: &modal_layer::ModalOpenedEvent, cx| {
                cx.emit(Event::ModalOpened);
            },
        )
        .detach();

        let bottom_dock_layout = WorkspaceSettings::get_global(cx).bottom_dock_layout;
        let left_dock = Dock::new(DockPosition::Left, modal_layer.clone(), window, cx);
        let bottom_dock = Dock::new(DockPosition::Bottom, modal_layer.clone(), window, cx);
        let right_dock = Dock::new(DockPosition::Right, modal_layer.clone(), window, cx);
        let left_dock_buttons = cx.new(|cx| PanelButtons::new(left_dock.clone(), cx));
        let bottom_dock_buttons = cx.new(|cx| PanelButtons::new(bottom_dock.clone(), cx));
        let right_dock_buttons = cx.new(|cx| PanelButtons::new(right_dock.clone(), cx));
        let status_bar = cx.new(|cx| {
            let mut status_bar = StatusBar::new(&center_pane.clone(), window, cx);
            status_bar.add_left_item(left_dock_buttons, window, cx);
            status_bar.add_right_item(right_dock_buttons, window, cx);
            status_bar.add_right_item(bottom_dock_buttons, window, cx);
            status_bar
        });

        let session_id = app_state.session.read(cx).id().to_owned();

        let mut active_call = None;
        if let Some(call) = ActiveCall::try_global(cx) {
            let call = call.clone();
            let subscriptions = vec![cx.subscribe_in(&call, window, Self::on_active_call_event)];
            active_call = Some((call, subscriptions));
        }

        let (serializable_items_tx, serializable_items_rx) =
            mpsc::unbounded::<Box<dyn SerializableItemHandle>>();
        let _items_serializer = cx.spawn_in(window, async move |this, cx| {
            Self::serialize_items(&this, serializable_items_rx, cx).await
        });

        let subscriptions = vec![
            cx.observe_window_activation(window, Self::on_window_activation_changed),
            cx.observe_window_bounds(window, move |this, window, cx| {
                if this.bounds_save_task_queued.is_some() {
                    return;
                }
                this.bounds_save_task_queued = Some(cx.spawn_in(window, async move |this, cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(100))
                        .await;
                    this.update_in(cx, |this, window, cx| {
                        if let Some(display) = window.display(cx) {
                            if let Ok(display_uuid) = display.uuid() {
                                let window_bounds = window.inner_window_bounds();
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
            cx.observe_window_appearance(window, |_, window, cx| {
                let window_appearance = window.appearance();

                *SystemAppearance::global_mut(cx) = SystemAppearance(window_appearance.into());

                ThemeSettings::reload_current_theme(cx);
                ThemeSettings::reload_current_icon_theme(cx);
            }),
            cx.on_release(move |this, cx| {
                this.app_state.workspace_store.update(cx, move |store, _| {
                    store.workspaces.remove(&window_handle.clone());
                })
            }),
        ];

        cx.defer_in(window, |this, window, cx| {
            this.update_window_title(window, cx);
            this.show_initial_notifications(cx);
        });
        Workspace {
            weak_self: weak_handle.clone(),
            zoomed: None,
            zoomed_position: None,
            previous_dock_drag_coordinates: None,
            center: PaneGroup::new(center_pane.clone()),
            panes: vec![center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane.clone(),
            last_active_center_pane: Some(center_pane.downgrade()),
            last_active_view_id: None,
            status_bar,
            modal_layer,
            toast_layer,
            titlebar_item: None,
            notifications: Notifications::default(),
            suppressed_notifications: HashSet::default(),
            left_dock,
            bottom_dock,
            bottom_dock_layout,
            right_dock,
            project: project.clone(),
            follower_states: Default::default(),
            last_leaders_by_pane: Default::default(),
            dispatching_keystrokes: Default::default(),
            window_edited: false,
            dirty_items: Default::default(),
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
            on_prompt_for_open_path: None,
            terminal_provider: None,
            debugger_provider: None,
            serializable_items_tx,
            _items_serializer,
            session_id: Some(session_id),
            serialized_ssh_project: None,
        }
    }

    pub fn new_local(
        abs_paths: Vec<PathBuf>,
        app_state: Arc<AppState>,
        requesting_window: Option<WindowHandle<Workspace>>,
        env: Option<HashMap<String, String>>,
        cx: &mut App,
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
            env,
            cx,
        );

        cx.spawn(async move |cx| {
            let mut paths_to_open = Vec::with_capacity(abs_paths.len());
            for path in abs_paths.into_iter() {
                if let Some(canonical) = app_state.fs.canonicalize(&path).await.ok() {
                    paths_to_open.push(canonical)
                } else {
                    paths_to_open.push(path)
                }
            }

            let serialized_workspace =
                persistence::DB.workspace_for_roots(paths_to_open.as_slice());

            let workspace_location = serialized_workspace
                .as_ref()
                .map(|ws| &ws.location)
                .and_then(|loc| match loc {
                    SerializedWorkspaceLocation::Local(_, order) => {
                        Some((loc.sorted_paths(), order.order()))
                    }
                    _ => None,
                });

            if let Some((paths, order)) = workspace_location {
                paths_to_open = paths.iter().cloned().collect();

                if order.iter().enumerate().any(|(i, &j)| i != j) {
                    project_handle
                        .update(cx, |project, cx| {
                            project.set_worktrees_reordered(true, cx);
                        })
                        .log_err();
                }
            }

            // Get project paths for all of the abs_paths
            let mut project_paths: Vec<(PathBuf, Option<ProjectPath>)> =
                Vec::with_capacity(paths_to_open.len());

            for path in paths_to_open.into_iter() {
                if let Some((_, project_entry)) = cx
                    .update(|cx| {
                        Workspace::project_path_for_path(project_handle.clone(), &path, true, cx)
                    })?
                    .await
                    .log_err()
                {
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

            let toolchains = DB.toolchains(workspace_id).await?;
            for (toolchain, worktree_id, path) in toolchains {
                project_handle
                    .update(cx, |this, cx| {
                        this.activate_toolchain(ProjectPath { worktree_id, path }, toolchain, cx)
                    })?
                    .await;
            }
            let window = if let Some(window) = requesting_window {
                cx.update_window(window.into(), |_, window, cx| {
                    window.replace_root(cx, |window, cx| {
                        Workspace::new(
                            Some(workspace_id),
                            project_handle.clone(),
                            app_state.clone(),
                            window,
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
                    move |window, cx| {
                        cx.new(|cx| {
                            let mut workspace = Workspace::new(
                                Some(workspace_id),
                                project_handle,
                                app_state,
                                window,
                                cx,
                            );
                            workspace.centered_layout = centered_layout;
                            workspace
                        })
                    }
                })?
            };

            notify_if_database_failed(window, cx);
            let opened_items = window
                .update(cx, |_workspace, window, cx| {
                    open_items(serialized_workspace, project_paths, window, cx)
                })?
                .await
                .unwrap_or_default();

            window
                .update(cx, |workspace, window, cx| {
                    window.activate_window();
                    workspace.update_history(cx);
                })
                .log_err();
            Ok((window, opened_items))
        })
    }

    pub fn weak_handle(&self) -> WeakEntity<Self> {
        self.weak_self.clone()
    }

    pub fn left_dock(&self) -> &Entity<Dock> {
        &self.left_dock
    }

    pub fn bottom_dock(&self) -> &Entity<Dock> {
        &self.bottom_dock
    }

    pub fn bottom_dock_layout(&self) -> BottomDockLayout {
        self.bottom_dock_layout
    }

    pub fn set_bottom_dock_layout(
        &mut self,
        layout: BottomDockLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = self.project().read(cx).fs();
        settings::update_settings_file::<WorkspaceSettings>(fs.clone(), cx, move |content, _cx| {
            content.bottom_dock_layout = Some(layout);
        });

        self.bottom_dock_layout = layout;
        cx.notify();
        self.serialize_workspace(window, cx);
    }

    pub fn right_dock(&self) -> &Entity<Dock> {
        &self.right_dock
    }

    pub fn all_docks(&self) -> [&Entity<Dock>; 3] {
        [&self.left_dock, &self.bottom_dock, &self.right_dock]
    }

    pub fn dock_at_position(&self, position: DockPosition) -> &Entity<Dock> {
        match position {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        }
    }

    pub fn is_edited(&self) -> bool {
        self.window_edited
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focus_handle = panel.panel_focus_handle(cx);
        cx.on_focus_in(&focus_handle, window, Self::handle_panel_focused)
            .detach();

        let dock_position = panel.position(window, cx);
        let dock = self.dock_at_position(dock_position);

        dock.update(cx, |dock, cx| {
            dock.add_panel(panel, self.weak_self.clone(), window, cx)
        });
    }

    pub fn status_bar(&self) -> &Entity<StatusBar> {
        &self.status_bar
    }

    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    pub fn user_store(&self) -> &Entity<UserStore> {
        &self.app_state.user_store
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn recently_activated_items(&self, cx: &App) -> HashMap<EntityId, usize> {
        let mut history: HashMap<EntityId, usize> = HashMap::default();

        for pane_handle in &self.panes {
            let pane = pane_handle.read(cx);

            for entry in pane.activation_history() {
                history.insert(
                    entry.entity_id,
                    history
                        .get(&entry.entity_id)
                        .cloned()
                        .unwrap_or(0)
                        .max(entry.timestamp),
                );
            }
        }

        history
    }

    pub fn recent_navigation_history_iter(
        &self,
        cx: &App,
    ) -> impl Iterator<Item = (ProjectPath, Option<PathBuf>)> {
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

            if let Some(item) = pane.active_item() {
                if let Some(project_path) = item.project_path(cx) {
                    let fs_path = self.project.read(cx).absolute_path(&project_path, cx);

                    if let Some(fs_path) = &fs_path {
                        abs_paths_opened
                            .entry(fs_path.clone())
                            .or_default()
                            .insert(project_path.clone());
                    }

                    history.insert(project_path, (fs_path, std::usize::MAX));
                }
            }
        }

        history
            .into_iter()
            .sorted_by_key(|(_, (_, order))| *order)
            .map(|(project_path, (fs_path, _))| (project_path, fs_path))
            .rev()
            .filter(move |(history_path, abs_path)| {
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
    }

    pub fn recent_navigation_history(
        &self,
        limit: Option<usize>,
        cx: &App,
    ) -> Vec<(ProjectPath, Option<PathBuf>)> {
        self.recent_navigation_history_iter(cx)
            .take(limit.unwrap_or(usize::MAX))
            .collect()
    }

    fn navigate_history(
        &mut self,
        pane: WeakEntity<Pane>,
        mode: NavigationMode,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        let to_load = if let Some(pane) = pane.upgrade() {
            pane.update(cx, |pane, cx| {
                window.focus(&pane.focus_handle(cx));
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
                        pane.activate_item(index, true, true, window, cx);
                        pane.nav_history_mut().set_mode(NavigationMode::Normal);

                        let mut navigated = prev_active_item_index != pane.active_item_index();
                        if let Some(data) = entry.data {
                            navigated |= pane.active_item()?.navigate(data, window, cx);
                        }

                        if navigated {
                            break None;
                        }
                    } else {
                        // If the item is no longer present in this pane, then retrieve its
                        // path info in order to reopen it.
                        break pane
                            .nav_history()
                            .path_for_item(entry.item.id())
                            .map(|(project_path, abs_path)| (project_path, abs_path, entry));
                    }
                }
            })
        } else {
            None
        };

        if let Some((project_path, abs_path, entry)) = to_load {
            // If the item was no longer present, then load it again from its previous path, first try the local path
            let open_by_project_path = self.load_path(project_path.clone(), window, cx);

            cx.spawn_in(window, async move  |workspace, cx| {
                let open_by_project_path = open_by_project_path.await;
                let mut navigated = false;
                match open_by_project_path
                    .with_context(|| format!("Navigating to {project_path:?}"))
                {
                    Ok((project_entry_id, build_item)) => {
                        let prev_active_item_id = pane.update(cx, |pane, _| {
                            pane.nav_history_mut().set_mode(mode);
                            pane.active_item().map(|p| p.item_id())
                        })?;

                        pane.update_in(cx, |pane, window, cx| {
                            let item = pane.open_item(
                                project_entry_id,
                                true,
                                entry.is_preview,
                                true,
                                None,
                                window, cx,
                                build_item,
                            );
                            navigated |= Some(item.item_id()) != prev_active_item_id;
                            pane.nav_history_mut().set_mode(NavigationMode::Normal);
                            if let Some(data) = entry.data {
                                navigated |= item.navigate(data, window, cx);
                            }
                        })?;
                    }
                    Err(open_by_project_path_e) => {
                        // Fall back to opening by abs path, in case an external file was opened and closed,
                        // and its worktree is now dropped
                        if let Some(abs_path) = abs_path {
                            let prev_active_item_id = pane.update(cx, |pane, _| {
                                pane.nav_history_mut().set_mode(mode);
                                pane.active_item().map(|p| p.item_id())
                            })?;
                            let open_by_abs_path = workspace.update_in(cx, |workspace, window, cx| {
                                workspace.open_abs_path(abs_path.clone(), OpenOptions { visible: Some(OpenVisible::None), ..Default::default() }, window, cx)
                            })?;
                            match open_by_abs_path
                                .await
                                .with_context(|| format!("Navigating to {abs_path:?}"))
                            {
                                Ok(item) => {
                                    pane.update_in(cx, |pane, window, cx| {
                                        navigated |= Some(item.item_id()) != prev_active_item_id;
                                        pane.nav_history_mut().set_mode(NavigationMode::Normal);
                                        if let Some(data) = entry.data {
                                            navigated |= item.navigate(data, window, cx);
                                        }
                                    })?;
                                }
                                Err(open_by_abs_path_e) => {
                                    log::error!("Failed to navigate history: {open_by_project_path_e:#} and {open_by_abs_path_e:#}");
                                }
                            }
                        }
                    }
                }

                if !navigated {
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            Self::navigate_history(workspace, pane, mode, window, cx)
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
        pane: WeakEntity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        self.navigate_history(pane, NavigationMode::GoingBack, window, cx)
    }

    pub fn go_forward(
        &mut self,
        pane: WeakEntity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        self.navigate_history(pane, NavigationMode::GoingForward, window, cx)
    }

    pub fn reopen_closed_item(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        self.navigate_history(
            self.active_pane().downgrade(),
            NavigationMode::ReopeningClosedItem,
            window,
            cx,
        )
    }

    pub fn client(&self) -> &Arc<Client> {
        &self.app_state.client
    }

    pub fn set_titlebar_item(&mut self, item: AnyView, _: &mut Window, cx: &mut Context<Self>) {
        self.titlebar_item = Some(item);
        cx.notify();
    }

    pub fn set_prompt_for_new_path(&mut self, prompt: PromptForNewPath) {
        self.on_prompt_for_new_path = Some(prompt)
    }

    pub fn set_prompt_for_open_path(&mut self, prompt: PromptForOpenPath) {
        self.on_prompt_for_open_path = Some(prompt)
    }

    pub fn set_terminal_provider(&mut self, provider: impl TerminalProvider + 'static) {
        self.terminal_provider = Some(Box::new(provider));
    }

    pub fn set_debugger_provider(&mut self, provider: impl DebuggerProvider + 'static) {
        self.debugger_provider = Some(Box::new(provider));
    }

    pub fn serialized_ssh_project(&self) -> Option<SerializedSshProject> {
        self.serialized_ssh_project.clone()
    }

    pub fn set_serialized_ssh_project(&mut self, serialized_ssh_project: SerializedSshProject) {
        self.serialized_ssh_project = Some(serialized_ssh_project);
    }

    pub fn prompt_for_open_path(
        &mut self,
        path_prompt_options: PathPromptOptions,
        lister: DirectoryLister,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        if !lister.is_local(cx) || !WorkspaceSettings::get_global(cx).use_system_path_prompts {
            let prompt = self.on_prompt_for_open_path.take().unwrap();
            let rx = prompt(self, lister, window, cx);
            self.on_prompt_for_open_path = Some(prompt);
            rx
        } else {
            let (tx, rx) = oneshot::channel();
            let abs_path = cx.prompt_for_paths(path_prompt_options);

            cx.spawn_in(window, async move |this, cx| {
                let Ok(result) = abs_path.await else {
                    return Ok(());
                };

                match result {
                    Ok(result) => {
                        tx.send(result).log_err();
                    }
                    Err(err) => {
                        let rx = this.update_in(cx, |this, window, cx| {
                            this.show_portal_error(err.to_string(), cx);
                            let prompt = this.on_prompt_for_open_path.take().unwrap();
                            let rx = prompt(this, lister, window, cx);
                            this.on_prompt_for_open_path = Some(prompt);
                            rx
                        })?;
                        if let Ok(path) = rx.await {
                            tx.send(path).log_err();
                        }
                    }
                };
                anyhow::Ok(())
            })
            .detach();

            rx
        }
    }

    pub fn prompt_for_new_path(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<Option<ProjectPath>> {
        if (self.project.read(cx).is_via_collab() || self.project.read(cx).is_via_ssh())
            || !WorkspaceSettings::get_global(cx).use_system_path_prompts
        {
            let prompt = self.on_prompt_for_new_path.take().unwrap();
            let rx = prompt(self, window, cx);
            self.on_prompt_for_new_path = Some(prompt);
            return rx;
        }

        let (tx, rx) = oneshot::channel();
        cx.spawn_in(window, async move |this, cx| {
            let abs_path = this.update(cx, |this, cx| {
                let mut relative_to = this
                    .most_recent_active_path(cx)
                    .and_then(|p| p.parent().map(|p| p.to_path_buf()));
                if relative_to.is_none() {
                    let project = this.project.read(cx);
                    relative_to = project
                        .visible_worktrees(cx)
                        .filter_map(|worktree| {
                            Some(worktree.read(cx).as_local()?.abs_path().to_path_buf())
                        })
                        .next()
                };

                cx.prompt_for_new_path(&relative_to.unwrap_or_else(|| PathBuf::from("")))
            })?;
            let abs_path = match abs_path.await? {
                Ok(path) => path,
                Err(err) => {
                    let rx = this.update_in(cx, |this, window, cx| {
                        this.show_portal_error(err.to_string(), cx);

                        let prompt = this.on_prompt_for_new_path.take().unwrap();
                        let rx = prompt(this, window, cx);
                        this.on_prompt_for_new_path = Some(prompt);
                        rx
                    })?;
                    if let Ok(path) = rx.await {
                        tx.send(path).log_err();
                    }
                    return anyhow::Ok(());
                }
            };

            let project_path = abs_path.and_then(|abs_path| {
                this.update(cx, |this, cx| {
                    this.project.update(cx, |project, cx| {
                        project.find_or_create_worktree(abs_path, true, cx)
                    })
                })
                .ok()
            });

            if let Some(project_path) = project_path {
                let (worktree, path) = project_path.await?;
                let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id())?;
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

    pub fn titlebar_item(&self) -> Option<AnyView> {
        self.titlebar_item.clone()
    }

    /// Call the given callback with a workspace whose project is local.
    ///
    /// If the given workspace has a local project, then it will be passed
    /// to the callback. Otherwise, a new empty window will be created.
    pub fn with_local_workspace<T, F>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        callback: F,
    ) -> Task<Result<T>>
    where
        T: 'static,
        F: 'static + FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) -> T,
    {
        if self.project.read(cx).is_local() {
            Task::ready(Ok(callback(self, window, cx)))
        } else {
            let env = self.project.read(cx).cli_environment(cx);
            let task = Self::new_local(Vec::new(), self.app_state.clone(), None, env, cx);
            cx.spawn_in(window, async move |_vh, cx| {
                let (workspace, _) = task.await?;
                workspace.update(cx, callback)
            })
        }
    }

    pub fn worktrees<'a>(&self, cx: &'a App) -> impl 'a + Iterator<Item = Entity<Worktree>> {
        self.project.read(cx).worktrees(cx)
    }

    pub fn visible_worktrees<'a>(
        &self,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = Entity<Worktree>> {
        self.project.read(cx).visible_worktrees(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn worktree_scans_complete(&self, cx: &App) -> impl Future<Output = ()> + 'static + use<> {
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

    pub fn close_global(_: &CloseWindow, cx: &mut App) {
        cx.defer(|cx| {
            cx.windows().iter().find(|window| {
                window
                    .update(cx, |_, window, _| {
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

    pub fn close_window(&mut self, _: &CloseWindow, window: &mut Window, cx: &mut Context<Self>) {
        let prepare = self.prepare_to_close(CloseIntent::CloseWindow, window, cx);
        cx.spawn_in(window, async move |_, cx| {
            if prepare.await? {
                cx.update(|window, _cx| window.remove_window())?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
    }

    pub fn move_focused_panel_to_next_position(
        &mut self,
        _: &MoveFocusedPanelToNextPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let docks = self.all_docks();
        let active_dock = docks
            .into_iter()
            .find(|dock| dock.focus_handle(cx).contains_focused(window, cx));

        if let Some(dock) = active_dock {
            dock.update(cx, |dock, cx| {
                let active_panel = dock
                    .active_panel()
                    .filter(|panel| panel.panel_focus_handle(cx).contains_focused(window, cx));

                if let Some(panel) = active_panel {
                    panel.move_to_next_position(window, cx);
                }
            })
        }
    }

    pub fn prepare_to_close(
        &mut self,
        close_intent: CloseIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let active_call = self.active_call().cloned();

        // On Linux and Windows, closing the last window should restore the last workspace.
        let save_last_workspace = cfg!(not(target_os = "macos"))
            && close_intent != CloseIntent::ReplaceWindow
            && cx.windows().len() == 1;

        cx.spawn_in(window, async move |this, cx| {
            let workspace_count = cx.update(|_window, cx| {
                cx.windows()
                    .iter()
                    .filter(|window| window.downcast::<Workspace>().is_some())
                    .count()
            })?;

            if let Some(active_call) = active_call {
                if close_intent != CloseIntent::Quit
                    && workspace_count == 1
                    && active_call.read_with(cx, |call, _| call.room().is_some())?
                {
                    let answer = cx.update(|window, cx| {
                        window.prompt(
                            PromptLevel::Warning,
                            "Do you want to leave the current call?",
                            None,
                            &["Close window and hang up", "Cancel"],
                            cx,
                        )
                    })?;

                    if answer.await.log_err() == Some(1) {
                        return anyhow::Ok(false);
                    } else {
                        active_call
                            .update(cx, |call, cx| call.hang_up(cx))?
                            .await
                            .log_err();
                    }
                }
            }

            let save_result = this
                .update_in(cx, |this, window, cx| {
                    this.save_all_internal(SaveIntent::Close, window, cx)
                })?
                .await;

            // If we're not quitting, but closing, we remove the workspace from
            // the current session.
            if close_intent != CloseIntent::Quit
                && !save_last_workspace
                && save_result.as_ref().map_or(false, |&res| res)
            {
                this.update_in(cx, |this, window, cx| this.remove_from_session(window, cx))?
                    .await;
            }

            save_result
        })
    }

    fn save_all(&mut self, action: &SaveAll, window: &mut Window, cx: &mut Context<Self>) {
        self.save_all_internal(
            action.save_intent.unwrap_or(SaveIntent::SaveAll),
            window,
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn send_keystrokes(
        &mut self,
        action: &SendKeystrokes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut state = self.dispatching_keystrokes.borrow_mut();
        if !state.0.insert(action.0.clone()) {
            cx.propagate();
            return;
        }
        let mut keystrokes: Vec<Keystroke> = action
            .0
            .split(' ')
            .flat_map(|k| Keystroke::parse(k).log_err())
            .collect();
        keystrokes.reverse();

        state.1.append(&mut keystrokes);
        drop(state);

        let keystrokes = self.dispatching_keystrokes.clone();
        window
            .spawn(cx, async move |cx| {
                // limit to 100 keystrokes to avoid infinite recursion.
                for _ in 0..100 {
                    let Some(keystroke) = keystrokes.borrow_mut().1.pop() else {
                        keystrokes.borrow_mut().0.clear();
                        return Ok(());
                    };
                    cx.update(|window, cx| {
                        let focused = window.focused(cx);
                        window.dispatch_keystroke(keystroke.clone(), cx);
                        if window.focused(cx) != focused {
                            // dispatch_keystroke may cause the focus to change.
                            // draw's side effect is to schedule the FocusChanged events in the current flush effect cycle
                            // And we need that to happen before the next keystroke to keep vim mode happy...
                            // (Note that the tests always do this implicitly, so you must manually test with something like:
                            //   "bindings": { "g z": ["workspace::SendKeystrokes", ": j <enter> u"]}
                            // )
                            window.draw(cx);
                        }
                    })?;
                }

                *keystrokes.borrow_mut() = Default::default();
                Err(anyhow!("over 100 keystrokes passed to send_keystrokes"))
            })
            .detach_and_log_err(cx);
    }

    fn save_all_internal(
        &mut self,
        mut save_intent: SaveIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        if self.project.read(cx).is_disconnected(cx) {
            return Task::ready(Ok(true));
        }
        let dirty_items = self
            .panes
            .iter()
            .flat_map(|pane| {
                pane.read(cx).items().filter_map(|item| {
                    if item.is_dirty(cx) {
                        item.tab_content_text(0, cx);
                        Some((pane.downgrade(), item.boxed_clone()))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let project = self.project.clone();
        cx.spawn_in(window, async move |workspace, cx| {
            let dirty_items = if save_intent == SaveIntent::Close && !dirty_items.is_empty() {
                let (serialize_tasks, remaining_dirty_items) =
                    workspace.update_in(cx, |workspace, window, cx| {
                        let mut remaining_dirty_items = Vec::new();
                        let mut serialize_tasks = Vec::new();
                        for (pane, item) in dirty_items {
                            if let Some(task) = item
                                .to_serializable_item_handle(cx)
                                .and_then(|handle| handle.serialize(workspace, true, window, cx))
                            {
                                serialize_tasks.push(task);
                            } else {
                                remaining_dirty_items.push((pane, item));
                            }
                        }
                        (serialize_tasks, remaining_dirty_items)
                    })?;

                futures::future::try_join_all(serialize_tasks).await?;

                if remaining_dirty_items.len() > 1 {
                    let answer = workspace.update_in(cx, |_, window, cx| {
                        let detail = Pane::file_names_for_prompt(
                            &mut remaining_dirty_items.iter().map(|(_, handle)| handle),
                            cx,
                        );
                        window.prompt(
                            PromptLevel::Warning,
                            &"Do you want to save all changes in the following files?",
                            Some(&detail),
                            &["Save all", "Discard all", "Cancel"],
                            cx,
                        )
                    })?;
                    match answer.await.log_err() {
                        Some(0) => save_intent = SaveIntent::SaveAll,
                        Some(1) => save_intent = SaveIntent::Skip,
                        Some(2) => return Ok(false),
                        _ => {}
                    }
                }

                remaining_dirty_items
            } else {
                dirty_items
            };

            for (pane, item) in dirty_items {
                let (singleton, project_entry_ids) =
                    cx.update(|_, cx| (item.is_singleton(cx), item.project_entry_ids(cx)))?;
                if singleton || !project_entry_ids.is_empty() {
                    if !Pane::save_item(project.clone(), &pane, &*item, save_intent, cx).await? {
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        })
    }

    pub fn open_workspace_for_paths(
        &mut self,
        replace_current_window: bool,
        paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let window_handle = window.window_handle().downcast::<Self>();
        let is_remote = self.project.read(cx).is_via_collab();
        let has_worktree = self.project.read(cx).worktrees(cx).next().is_some();
        let has_dirty_items = self.items(cx).any(|item| item.is_dirty(cx));

        let window_to_replace = if replace_current_window {
            window_handle
        } else if is_remote || has_worktree || has_dirty_items {
            None
        } else {
            window_handle
        };
        let app_state = self.app_state.clone();

        cx.spawn(async move |_, cx| {
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
        options: OpenOptions,
        pane: Option<WeakEntity<Pane>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>> {
        log::info!("open paths {abs_paths:?}");

        let fs = self.app_state.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn_in(window, async move |this, cx| {
            let mut tasks = Vec::with_capacity(abs_paths.len());

            for abs_path in &abs_paths {
                let visible = match options.visible.as_ref().unwrap_or(&OpenVisible::None) {
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
                        .update(cx, |this, cx| {
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
                let abs_path: Arc<Path> = SanitizedPath::from(abs_path.clone()).into();
                let fs = fs.clone();
                let pane = pane.clone();
                let task = cx.spawn(async move |cx| {
                    let (worktree, project_path) = project_path?;
                    if fs.is_dir(&abs_path).await {
                        this.update(cx, |workspace, cx| {
                            let worktree = worktree.read(cx);
                            let worktree_abs_path = worktree.abs_path();
                            let entry_id = if abs_path.as_ref() == worktree_abs_path.as_ref() {
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
                            this.update_in(cx, |this, window, cx| {
                                this.open_path(
                                    project_path,
                                    pane,
                                    options.focus.unwrap_or(true),
                                    window,
                                    cx,
                                )
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

    pub fn open_resolved_path(
        &mut self,
        path: ResolvedPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        match path {
            ResolvedPath::ProjectPath { project_path, .. } => {
                self.open_path(project_path, None, true, window, cx)
            }
            ResolvedPath::AbsPath { path, .. } => self.open_abs_path(
                path,
                OpenOptions {
                    visible: Some(OpenVisible::None),
                    ..Default::default()
                },
                window,
                cx,
            ),
        }
    }

    pub fn absolute_path_of_worktree(
        &self,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) -> Option<PathBuf> {
        self.project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            // TODO: use `abs_path` or `root_dir`
            .map(|wt| wt.read(cx).abs_path().as_ref().to_path_buf())
    }

    fn add_folder_to_project(
        &mut self,
        _: &AddFolderToProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = self.project.read(cx);
        if project.is_via_collab() {
            self.show_error(
                &anyhow!("You cannot add folders to someone else's project"),
                cx,
            );
            return;
        }
        let paths = self.prompt_for_open_path(
            PathPromptOptions {
                files: false,
                directories: true,
                multiple: true,
            },
            DirectoryLister::Project(self.project.clone()),
            window,
            cx,
        );
        cx.spawn_in(window, async move |this, cx| {
            if let Some(paths) = paths.await.log_err().flatten() {
                let results = this
                    .update_in(cx, |this, window, cx| {
                        this.open_paths(
                            paths,
                            OpenOptions {
                                visible: Some(OpenVisible::All),
                                ..Default::default()
                            },
                            None,
                            window,
                            cx,
                        )
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

    pub fn project_path_for_path(
        project: Entity<Project>,
        abs_path: &Path,
        visible: bool,
        cx: &mut App,
    ) -> Task<Result<(Entity<Worktree>, ProjectPath)>> {
        let entry = project.update(cx, |project, cx| {
            project.find_or_create_worktree(abs_path, visible, cx)
        });
        cx.spawn(async move |cx| {
            let (worktree, path) = entry.await?;
            let worktree_id = worktree.update(cx, |t, _| t.id())?;
            Ok((
                worktree,
                ProjectPath {
                    worktree_id,
                    path: path.into(),
                },
            ))
        })
    }

    pub fn items<'a>(&'a self, cx: &'a App) -> impl 'a + Iterator<Item = &'a Box<dyn ItemHandle>> {
        self.panes.iter().flat_map(|pane| pane.read(cx).items())
    }

    pub fn item_of_type<T: Item>(&self, cx: &App) -> Option<Entity<T>> {
        self.items_of_type(cx).max_by_key(|item| item.item_id())
    }

    pub fn items_of_type<'a, T: Item>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = Entity<T>> {
        self.panes
            .iter()
            .flat_map(|pane| pane.read(cx).items_of_type())
    }

    pub fn active_item(&self, cx: &App) -> Option<Box<dyn ItemHandle>> {
        self.active_pane().read(cx).active_item()
    }

    pub fn active_item_as<I: 'static>(&self, cx: &App) -> Option<Entity<I>> {
        let item = self.active_item(cx)?;
        item.to_any().downcast::<I>().ok()
    }

    fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.active_item(cx).and_then(|item| item.project_path(cx))
    }

    pub fn most_recent_active_path(&self, cx: &App) -> Option<PathBuf> {
        self.recent_navigation_history_iter(cx)
            .filter_map(|(path, abs_path)| {
                let worktree = self
                    .project
                    .read(cx)
                    .worktree_for_id(path.worktree_id, cx)?;
                if worktree.read(cx).is_visible() {
                    abs_path
                } else {
                    None
                }
            })
            .next()
    }

    pub fn save_active_item(
        &mut self,
        save_intent: SaveIntent,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let project = self.project.clone();
        let pane = self.active_pane();
        let item = pane.read(cx).active_item();
        let pane = pane.downgrade();

        window.spawn(cx, async move |mut cx| {
            if let Some(item) = item {
                Pane::save_item(project, &pane, item.as_ref(), save_intent, &mut cx)
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(task) = self.close_all_internal(
            true,
            action.save_intent.unwrap_or(SaveIntent::Close),
            window,
            cx,
        ) {
            task.detach_and_log_err(cx)
        }
    }

    pub fn close_all_items_and_panes(
        &mut self,
        action: &CloseAllItemsAndPanes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(task) = self.close_all_internal(
            false,
            action.save_intent.unwrap_or(SaveIntent::Close),
            window,
            cx,
        ) {
            task.detach_and_log_err(cx)
        }
    }

    fn close_all_internal(
        &mut self,
        retain_active_pane: bool,
        save_intent: SaveIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let current_pane = self.active_pane();

        let mut tasks = Vec::new();

        if retain_active_pane {
            if let Some(current_pane_close) = current_pane.update(cx, |pane, cx| {
                pane.close_inactive_items(
                    &CloseInactiveItems {
                        save_intent: None,
                        close_pinned: false,
                    },
                    window,
                    cx,
                )
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
                        close_pinned: false,
                    },
                    window,
                    cx,
                )
            }) {
                tasks.push(close_pane_items)
            }
        }

        if tasks.is_empty() {
            None
        } else {
            Some(cx.spawn_in(window, async move |_, _| {
                for task in tasks {
                    task.await?
                }
                Ok(())
            }))
        }
    }

    pub fn is_dock_at_position_open(&self, position: DockPosition, cx: &mut Context<Self>) -> bool {
        self.dock_at_position(position).read(cx).is_open()
    }

    pub fn toggle_dock(
        &mut self,
        dock_side: DockPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dock = self.dock_at_position(dock_side);
        let mut focus_center = false;
        let mut reveal_dock = false;
        dock.update(cx, |dock, cx| {
            let other_is_zoomed = self.zoomed.is_some() && self.zoomed_position != Some(dock_side);
            let was_visible = dock.is_open() && !other_is_zoomed;
            dock.set_open(!was_visible, window, cx);

            if dock.active_panel().is_none() {
                let Some(panel_ix) = dock
                    .first_enabled_panel_idx(cx)
                    .log_with_level(log::Level::Info)
                else {
                    return;
                };
                dock.activate_panel(panel_ix, window, cx);
            }

            if let Some(active_panel) = dock.active_panel() {
                if was_visible {
                    if active_panel
                        .panel_focus_handle(cx)
                        .contains_focused(window, cx)
                    {
                        focus_center = true;
                    }
                } else {
                    let focus_handle = &active_panel.panel_focus_handle(cx);
                    window.focus(focus_handle);
                    reveal_dock = true;
                }
            }
        });

        if reveal_dock {
            self.dismiss_zoomed_items_to_reveal(Some(dock_side), window, cx);
        }

        if focus_center {
            self.active_pane
                .update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)))
        }

        cx.notify();
        self.serialize_workspace(window, cx);
    }

    fn active_dock(&self, window: &Window, cx: &Context<Self>) -> Option<&Entity<Dock>> {
        self.all_docks().into_iter().find(|&dock| {
            dock.read(cx).is_open() && dock.focus_handle(cx).contains_focused(window, cx)
        })
    }

    fn close_active_dock(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(dock) = self.active_dock(window, cx) {
            dock.update(cx, |dock, cx| {
                dock.set_open(false, window, cx);
            });
        }
    }

    pub fn close_all_docks(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for dock in self.all_docks() {
            dock.update(cx, |dock, cx| {
                dock.set_open(false, window, cx);
            });
        }

        cx.focus_self(window);
        cx.notify();
        self.serialize_workspace(window, cx);
    }

    /// Transfer focus to the panel of the given type.
    pub fn focus_panel<T: Panel>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<T>> {
        let panel = self.focus_or_unfocus_panel::<T>(window, cx, |_, _, _| true)?;
        panel.to_any().downcast().ok()
    }

    /// Focus the panel of the given type if it isn't already focused. If it is
    /// already focused, then transfer focus back to the workspace center.
    pub fn toggle_panel_focus<T: Panel>(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus_or_unfocus_panel::<T>(window, cx, |panel, window, cx| {
            !panel.panel_focus_handle(cx).contains_focused(window, cx)
        });
    }

    pub fn activate_panel_for_proto_id(
        &mut self,
        panel_id: PanelId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Arc<dyn PanelHandle>> {
        let mut panel = None;
        for dock in self.all_docks() {
            if let Some(panel_index) = dock.read(cx).panel_index_for_proto_id(panel_id) {
                panel = dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, window, cx);
                    dock.set_open(true, window, cx);
                    dock.active_panel().cloned()
                });
                break;
            }
        }

        if panel.is_some() {
            cx.notify();
            self.serialize_workspace(window, cx);
        }

        panel
    }

    /// Focus or unfocus the given panel type, depending on the given callback.
    fn focus_or_unfocus_panel<T: Panel>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        should_focus: impl Fn(&dyn PanelHandle, &mut Window, &mut Context<Dock>) -> bool,
    ) -> Option<Arc<dyn PanelHandle>> {
        let mut result_panel = None;
        let mut serialize = false;
        for dock in self.all_docks() {
            if let Some(panel_index) = dock.read(cx).panel_index_for_type::<T>() {
                let mut focus_center = false;
                let panel = dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, window, cx);

                    let panel = dock.active_panel().cloned();
                    if let Some(panel) = panel.as_ref() {
                        if should_focus(&**panel, window, cx) {
                            dock.set_open(true, window, cx);
                            panel.panel_focus_handle(cx).focus(window);
                        } else {
                            focus_center = true;
                        }
                    }
                    panel
                });

                if focus_center {
                    self.active_pane
                        .update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)))
                }

                result_panel = panel;
                serialize = true;
                break;
            }
        }

        if serialize {
            self.serialize_workspace(window, cx);
        }

        cx.notify();
        result_panel
    }

    /// Open the panel of the given type
    pub fn open_panel<T: Panel>(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for dock in self.all_docks() {
            if let Some(panel_index) = dock.read(cx).panel_index_for_type::<T>() {
                dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, window, cx);
                    dock.set_open(true, window, cx);
                });
            }
        }
    }

    pub fn panel<T: Panel>(&self, cx: &App) -> Option<Entity<T>> {
        self.all_docks()
            .iter()
            .find_map(|dock| dock.read(cx).panel::<T>())
    }

    fn dismiss_zoomed_items_to_reveal(
        &mut self,
        dock_to_reveal: Option<DockPosition>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If a center pane is zoomed, unzoom it.
        for pane in &self.panes {
            if pane != &self.active_pane || dock_to_reveal.is_some() {
                pane.update(cx, |pane, cx| pane.set_zoomed(false, cx));
            }
        }

        // If another dock is zoomed, hide it.
        let mut focus_center = false;
        for dock in self.all_docks() {
            dock.update(cx, |dock, cx| {
                if Some(dock.position()) != dock_to_reveal {
                    if let Some(panel) = dock.active_panel() {
                        if panel.is_zoomed(window, cx) {
                            focus_center |=
                                panel.panel_focus_handle(cx).contains_focused(window, cx);
                            dock.set_open(false, window, cx);
                        }
                    }
                }
            });
        }

        if focus_center {
            self.active_pane
                .update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)))
        }

        if self.zoomed_position != dock_to_reveal {
            self.zoomed = None;
            self.zoomed_position = None;
            cx.emit(Event::ZoomChanged);
        }

        cx.notify();
    }

    fn add_pane(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<Pane> {
        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                self.weak_handle(),
                self.project.clone(),
                self.pane_history_timestamp.clone(),
                None,
                NewFile.boxed_clone(),
                window,
                cx,
            );
            pane.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            pane
        });
        cx.subscribe_in(&pane, window, Self::handle_pane_event)
            .detach();
        self.panes.push(pane.clone());

        window.focus(&pane.focus_handle(cx));

        cx.emit(Event::PaneAdded(pane.clone()));
        pane
    }

    pub fn add_item_to_center(
        &mut self,
        item: Box<dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if let Some(center_pane) = self.last_active_center_pane.clone() {
            if let Some(center_pane) = center_pane.upgrade() {
                center_pane.update(cx, |pane, cx| {
                    pane.add_item(item, true, true, None, window, cx)
                });
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
        focus_item: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.add_item(
            self.active_pane.clone(),
            item,
            destination_index,
            false,
            focus_item,
            window,
            cx,
        )
    }

    pub fn add_item(
        &mut self,
        pane: Entity<Pane>,
        item: Box<dyn ItemHandle>,
        destination_index: Option<usize>,
        activate_pane: bool,
        focus_item: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(text) = item.telemetry_event_text(cx) {
            telemetry::event!(text);
        }

        pane.update(cx, |pane, cx| {
            pane.add_item(
                item,
                activate_pane,
                focus_item,
                destination_index,
                window,
                cx,
            )
        });
    }

    pub fn split_item(
        &mut self,
        split_direction: SplitDirection,
        item: Box<dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_pane = self.split_pane(self.active_pane.clone(), split_direction, window, cx);
        self.add_item(new_pane, item, None, true, true, window, cx);
    }

    pub fn open_abs_path(
        &mut self,
        abs_path: PathBuf,
        options: OpenOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        cx.spawn_in(window, async move |workspace, cx| {
            let open_paths_task_result = workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_paths(vec![abs_path.clone()], options, None, window, cx)
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        let project_path_task =
            Workspace::project_path_for_path(self.project.clone(), &abs_path, visible, cx);
        cx.spawn_in(window, async move |this, cx| {
            let (_, path) = project_path_task.await?;
            this.update_in(cx, |this, window, cx| this.split_path(path, window, cx))?
                .await
        })
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakEntity<Pane>>,
        focus_item: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        self.open_path_preview(path, pane, focus_item, false, true, window, cx)
    }

    pub fn open_path_preview(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakEntity<Pane>>,
        focus_item: bool,
        allow_preview: bool,
        activate: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        let pane = pane.unwrap_or_else(|| {
            self.last_active_center_pane.clone().unwrap_or_else(|| {
                self.panes
                    .first()
                    .expect("There must be an active pane")
                    .downgrade()
            })
        });

        let task = self.load_path(path.into(), window, cx);
        window.spawn(cx, async move |cx| {
            let (project_entry_id, build_item) = task.await?;
            let result = pane.update_in(cx, |pane, window, cx| {
                let result = pane.open_item(
                    project_entry_id,
                    focus_item,
                    allow_preview,
                    activate,
                    None,
                    window,
                    cx,
                    build_item,
                );

                result
            });
            result
        })
    }

    pub fn split_path(
        &mut self,
        path: impl Into<ProjectPath>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        self.split_path_preview(path, false, None, window, cx)
    }

    pub fn split_path_preview(
        &mut self,
        path: impl Into<ProjectPath>,
        allow_preview: bool,
        split_direction: Option<SplitDirection>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        let pane = self.last_active_center_pane.clone().unwrap_or_else(|| {
            self.panes
                .first()
                .expect("There must be an active pane")
                .downgrade()
        });

        if let Member::Pane(center_pane) = &self.center.root {
            if center_pane.read(cx).items_len() == 0 {
                return self.open_path(path, Some(pane), true, window, cx);
            }
        }

        let task = self.load_path(path.into(), window, cx);
        cx.spawn_in(window, async move |this, cx| {
            let (project_entry_id, build_item) = task.await?;
            this.update_in(cx, move |this, window, cx| -> Option<_> {
                let pane = pane.upgrade()?;
                let new_pane = this.split_pane(
                    pane,
                    split_direction.unwrap_or(SplitDirection::Right),
                    window,
                    cx,
                );
                new_pane.update(cx, |new_pane, cx| {
                    Some(new_pane.open_item(
                        project_entry_id,
                        true,
                        allow_preview,
                        true,
                        None,
                        window,
                        cx,
                        build_item,
                    ))
                })
            })
            .map(|option| option.ok_or_else(|| anyhow!("pane was dropped")))?
        })
    }

    fn load_path(
        &mut self,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<(Option<ProjectEntryId>, WorkspaceItemBuilder)>> {
        let project = self.project().clone();
        let registry = cx.default_global::<ProjectItemRegistry>().clone();
        registry.open_path(&project, &path, window, cx)
    }

    pub fn find_project_item<T>(
        &self,
        pane: &Entity<Pane>,
        project_item: &Entity<T::Item>,
        cx: &App,
    ) -> Option<Entity<T>>
    where
        T: ProjectItem,
    {
        use project::ProjectItem as _;
        let project_item = project_item.read(cx);
        let entry_id = project_item.entry_id(cx);
        let project_path = project_item.project_path(cx);

        let mut item = None;
        if let Some(entry_id) = entry_id {
            item = pane.read(cx).item_for_entry(entry_id, cx);
        }
        if item.is_none() {
            if let Some(project_path) = project_path {
                item = pane.read(cx).item_for_path(project_path, cx);
            }
        }

        item.and_then(|item| item.downcast::<T>())
    }

    pub fn is_project_item_open<T>(
        &self,
        pane: &Entity<Pane>,
        project_item: &Entity<T::Item>,
        cx: &App,
    ) -> bool
    where
        T: ProjectItem,
    {
        self.find_project_item::<T>(pane, project_item, cx)
            .is_some()
    }

    pub fn open_project_item<T>(
        &mut self,
        pane: Entity<Pane>,
        project_item: Entity<T::Item>,
        activate_pane: bool,
        focus_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<T>
    where
        T: ProjectItem,
    {
        if let Some(item) = self.find_project_item(&pane, &project_item, cx) {
            self.activate_item(&item, activate_pane, focus_item, window, cx);
            return item;
        }

        let item = pane.update(cx, |pane, cx| {
            cx.new(|cx| {
                T::for_project_item(self.project().clone(), Some(pane), project_item, window, cx)
            })
        });
        let item_id = item.item_id();
        let mut destination_index = None;
        pane.update(cx, |pane, cx| {
            if PreviewTabsSettings::get_global(cx).enable_preview_from_code_navigation {
                if let Some(preview_item_id) = pane.preview_item_id() {
                    if preview_item_id != item_id {
                        destination_index = pane.close_current_preview_item(window, cx);
                    }
                }
            }
            pane.set_preview_item_id(Some(item.item_id()), cx)
        });

        self.add_item(
            pane,
            Box::new(item.clone()),
            destination_index,
            activate_pane,
            focus_item,
            window,
            cx,
        );
        item
    }

    pub fn open_shared_screen(
        &mut self,
        peer_id: PeerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(shared_screen) =
            self.shared_screen_for_peer(peer_id, &self.active_pane, window, cx)
        {
            self.active_pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(shared_screen), false, true, None, window, cx)
            });
        }
    }

    pub fn activate_item(
        &mut self,
        item: &dyn ItemHandle,
        activate_pane: bool,
        focus_item: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let result = self.panes.iter().find_map(|pane| {
            pane.read(cx)
                .index_for_item(item)
                .map(|ix| (pane.clone(), ix))
        });
        if let Some((pane, ix)) = result {
            pane.update(cx, |pane, cx| {
                pane.activate_item(ix, activate_pane, focus_item, window, cx)
            });
            true
        } else {
            false
        }
    }

    fn activate_pane_at_index(
        &mut self,
        action: &ActivatePane,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let panes = self.center.panes();
        if let Some(pane) = panes.get(action.0).map(|p| (*p).clone()) {
            window.focus(&pane.focus_handle(cx));
        } else {
            self.split_and_clone(self.active_pane.clone(), SplitDirection::Right, window, cx);
        }
    }

    fn move_item_to_pane_at_index(
        &mut self,
        action: &MoveItemToPane,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(&target_pane) = self.center.panes().get(action.destination) else {
            return;
        };
        move_active_item(
            &self.active_pane,
            target_pane,
            action.focus,
            true,
            window,
            cx,
        );
    }

    pub fn activate_next_pane(&mut self, window: &mut Window, cx: &mut App) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let next_ix = (ix + 1) % panes.len();
            let next_pane = panes[next_ix].clone();
            window.focus(&next_pane.focus_handle(cx));
        }
    }

    pub fn activate_previous_pane(&mut self, window: &mut Window, cx: &mut App) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let prev_ix = cmp::min(ix.wrapping_sub(1), panes.len() - 1);
            let prev_pane = panes[prev_ix].clone();
            window.focus(&prev_pane.focus_handle(cx));
        }
    }

    pub fn activate_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut App,
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
            if dock.focus_handle(cx).contains_focused(window, cx) && dock.read(cx).is_open() {
                Some(origin)
            } else {
                None
            }
        })
        .unwrap_or(Origin::Center);

        let get_last_active_pane = || {
            let pane = self
                .last_active_center_pane
                .clone()
                .unwrap_or_else(|| {
                    self.panes
                        .first()
                        .expect("There must be an active pane")
                        .downgrade()
                })
                .upgrade()?;
            (pane.read(cx).items_len() != 0).then_some(pane)
        };

        let try_dock =
            |dock: &Entity<Dock>| dock.read(cx).is_open().then(|| Target::Dock(dock.clone()));

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
            Some(ActivateInDirectionTarget::Pane(pane)) => {
                window.focus(&pane.focus_handle(cx));
            }
            Some(ActivateInDirectionTarget::Dock(dock)) => {
                // Defer this to avoid a panic when the dock's active panel is already on the stack.
                window.defer(cx, move |window, cx| {
                    let dock = dock.read(cx);
                    if let Some(panel) = dock.active_panel() {
                        panel.panel_focus_handle(cx).focus(window);
                    } else {
                        log::error!("Could not find a focus target when in switching focus in {direction} direction for a {:?} dock", dock.position());
                    }
                })
            }
            None => {}
        }
    }

    pub fn move_item_to_pane_in_direction(
        &mut self,
        action: &MoveItemToPaneInDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(destination) = self.find_pane_in_direction(action.direction, cx) {
            move_active_item(
                &self.active_pane,
                &destination,
                action.focus,
                true,
                window,
                cx,
            );
        }
    }

    pub fn bounding_box_for_pane(&self, pane: &Entity<Pane>) -> Option<Bounds<Pixels>> {
        self.center.bounding_box_for_pane(pane)
    }

    pub fn find_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        cx: &App,
    ) -> Option<Entity<Pane>> {
        self.center
            .find_pane_in_direction(&self.active_pane, direction, cx)
            .cloned()
    }

    pub fn swap_pane_in_direction(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if let Some(to) = self.find_pane_in_direction(direction, cx) {
            self.center.swap(&self.active_pane, &to);
            cx.notify();
        }
    }

    pub fn resize_pane(
        &mut self,
        axis: gpui::Axis,
        amount: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let docks = self.all_docks();
        let active_dock = docks
            .into_iter()
            .find(|dock| dock.focus_handle(cx).contains_focused(window, cx));

        if let Some(dock) = active_dock {
            let Some(panel_size) = dock.read(cx).active_panel_size(window, cx) else {
                return;
            };
            match dock.read(cx).position() {
                DockPosition::Left => resize_left_dock(panel_size + amount, self, window, cx),
                DockPosition::Bottom => resize_bottom_dock(panel_size + amount, self, window, cx),
                DockPosition::Right => resize_right_dock(panel_size + amount, self, window, cx),
            }
        } else {
            self.center
                .resize(&self.active_pane, axis, amount, &self.bounds);
        }
        cx.notify();
    }

    pub fn reset_pane_sizes(&mut self, cx: &mut Context<Self>) {
        self.center.reset_pane_sizes();
        cx.notify();
    }

    fn handle_pane_focused(
        &mut self,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // This is explicitly hoisted out of the following check for pane identity as
        // terminal panel panes are not registered as a center panes.
        self.status_bar.update(cx, |status_bar, cx| {
            status_bar.set_active_pane(&pane, window, cx);
        });
        if self.active_pane != pane {
            self.set_active_pane(&pane, window, cx);
        }

        if self.last_active_center_pane.is_none() {
            self.last_active_center_pane = Some(pane.downgrade());
        }

        self.dismiss_zoomed_items_to_reveal(None, window, cx);
        if pane.read(cx).is_zoomed() {
            self.zoomed = Some(pane.downgrade().into());
        } else {
            self.zoomed = None;
        }
        self.zoomed_position = None;
        cx.emit(Event::ZoomChanged);
        self.update_active_view_for_followers(window, cx);
        pane.update(cx, |pane, _| {
            pane.track_alternate_file_items();
        });

        cx.notify();
    }

    fn set_active_pane(
        &mut self,
        pane: &Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_pane = pane.clone();
        self.active_item_path_changed(window, cx);
        self.last_active_center_pane = Some(pane.downgrade());
    }

    fn handle_panel_focused(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_active_view_for_followers(window, cx);
    }

    fn handle_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut serialize_workspace = true;
        match event {
            pane::Event::AddItem { item } => {
                item.added_to_pane(self, pane.clone(), window, cx);
                cx.emit(Event::ItemAdded {
                    item: item.boxed_clone(),
                });
            }
            pane::Event::Split(direction) => {
                self.split_and_clone(pane.clone(), *direction, window, cx);
            }
            pane::Event::JoinIntoNext => {
                self.join_pane_into_next(pane.clone(), window, cx);
            }
            pane::Event::JoinAll => {
                self.join_all_panes(window, cx);
            }
            pane::Event::Remove { focus_on_pane } => {
                self.remove_pane(pane.clone(), focus_on_pane.clone(), window, cx);
            }
            pane::Event::ActivateItem {
                local,
                focus_changed,
            } => {
                cx.on_next_frame(window, |_, window, _| {
                    window.invalidate_character_coordinates();
                });

                pane.update(cx, |pane, _| {
                    pane.track_alternate_file_items();
                });
                if *local {
                    self.unfollow_in_pane(&pane, window, cx);
                }
                if pane == self.active_pane() {
                    self.active_item_path_changed(window, cx);
                    self.update_active_view_for_followers(window, cx);
                }
                serialize_workspace = *focus_changed || pane != self.active_pane();
            }
            pane::Event::UserSavedItem { item, save_intent } => {
                cx.emit(Event::UserSavedItem {
                    pane: pane.downgrade(),
                    item: item.boxed_clone(),
                    save_intent: *save_intent,
                });
                serialize_workspace = false;
            }
            pane::Event::ChangeItemTitle => {
                if *pane == self.active_pane {
                    self.active_item_path_changed(window, cx);
                }
                serialize_workspace = false;
            }
            pane::Event::RemoveItem { .. } => {}
            pane::Event::RemovedItem { item } => {
                cx.emit(Event::ActiveItemChanged);
                self.update_window_edited(window, cx);
                if let hash_map::Entry::Occupied(entry) = self.panes_by_item.entry(item.item_id()) {
                    if entry.get().entity_id() == pane.entity_id() {
                        entry.remove();
                    }
                }
            }
            pane::Event::Focus => {
                cx.on_next_frame(window, |_, window, _| {
                    window.invalidate_character_coordinates();
                });
                self.handle_pane_focused(pane.clone(), window, cx);
            }
            pane::Event::ZoomIn => {
                if *pane == self.active_pane {
                    pane.update(cx, |pane, cx| pane.set_zoomed(true, cx));
                    if pane.read(cx).has_focus(window, cx) {
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

        if serialize_workspace {
            self.serialize_workspace(window, cx);
        }
    }

    pub fn unfollow_in_pane(
        &mut self,
        pane: &Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<CollaboratorId> {
        let leader_id = self.leader_for_pane(pane)?;
        self.unfollow(leader_id, window, cx);
        Some(leader_id)
    }

    pub fn split_pane(
        &mut self,
        pane_to_split: Entity<Pane>,
        split_direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Pane> {
        let new_pane = self.add_pane(window, cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();
        cx.notify();
        new_pane
    }

    pub fn split_and_clone(
        &mut self,
        pane: Entity<Pane>,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Pane>> {
        let item = pane.read(cx).active_item()?;
        let maybe_pane_handle =
            if let Some(clone) = item.clone_on_split(self.database_id(), window, cx) {
                let new_pane = self.add_pane(window, cx);
                new_pane.update(cx, |pane, cx| {
                    pane.add_item(clone, true, true, None, window, cx)
                });
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
        pane_to_split: WeakEntity<Pane>,
        split_direction: SplitDirection,
        from: WeakEntity<Pane>,
        item_id_to_move: EntityId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(pane_to_split) = pane_to_split.upgrade() else {
            return;
        };
        let Some(from) = from.upgrade() else {
            return;
        };

        let new_pane = self.add_pane(window, cx);
        move_item(&from, &new_pane, item_id_to_move, 0, window, cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();
        cx.notify();
    }

    pub fn split_pane_with_project_entry(
        &mut self,
        pane_to_split: WeakEntity<Pane>,
        split_direction: SplitDirection,
        project_entry: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let pane_to_split = pane_to_split.upgrade()?;
        let new_pane = self.add_pane(window, cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();

        let path = self.project.read(cx).path_for_entry(project_entry, cx)?;
        let task = self.open_path(path, Some(new_pane.downgrade()), true, window, cx);
        Some(cx.foreground_executor().spawn(async move {
            task.await?;
            Ok(())
        }))
    }

    pub fn join_all_panes(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let active_item = self.active_pane.read(cx).active_item();
        for pane in &self.panes {
            join_pane_into_active(&self.active_pane, pane, window, cx);
        }
        if let Some(active_item) = active_item {
            self.activate_item(active_item.as_ref(), true, true, window, cx);
        }
        cx.notify();
    }

    pub fn join_pane_into_next(
        &mut self,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let next_pane = self
            .find_pane_in_direction(SplitDirection::Right, cx)
            .or_else(|| self.find_pane_in_direction(SplitDirection::Down, cx))
            .or_else(|| self.find_pane_in_direction(SplitDirection::Left, cx))
            .or_else(|| self.find_pane_in_direction(SplitDirection::Up, cx));
        let Some(next_pane) = next_pane else {
            return;
        };
        move_all_items(&pane, &next_pane, window, cx);
        cx.notify();
    }

    fn remove_pane(
        &mut self,
        pane: Entity<Pane>,
        focus_on: Option<Entity<Pane>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.center.remove(&pane).unwrap() {
            self.force_remove_pane(&pane, &focus_on, window, cx);
            self.unfollow_in_pane(&pane, window, cx);
            self.last_leaders_by_pane.remove(&pane.downgrade());
            for removed_item in pane.read(cx).items() {
                self.panes_by_item.remove(&removed_item.item_id());
            }

            cx.notify();
        } else {
            self.active_item_path_changed(window, cx);
        }
        cx.emit(Event::PaneRemoved);
    }

    pub fn panes(&self) -> &[Entity<Pane>] {
        &self.panes
    }

    pub fn active_pane(&self) -> &Entity<Pane> {
        &self.active_pane
    }

    pub fn focused_pane(&self, window: &Window, cx: &App) -> Entity<Pane> {
        for dock in self.all_docks() {
            if dock.focus_handle(cx).contains_focused(window, cx) {
                if let Some(pane) = dock
                    .read(cx)
                    .active_panel()
                    .and_then(|panel| panel.pane(cx))
                {
                    return pane;
                }
            }
        }
        self.active_pane().clone()
    }

    pub fn adjacent_pane(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<Pane> {
        self.find_pane_in_direction(SplitDirection::Right, cx)
            .or_else(|| self.find_pane_in_direction(SplitDirection::Left, cx))
            .unwrap_or_else(|| {
                self.split_pane(self.active_pane.clone(), SplitDirection::Right, window, cx)
            })
            .clone()
    }

    pub fn pane_for(&self, handle: &dyn ItemHandle) -> Option<Entity<Pane>> {
        let weak_pane = self.panes_by_item.get(&handle.item_id())?;
        weak_pane.upgrade()
    }

    fn collaborator_left(&mut self, peer_id: PeerId, window: &mut Window, cx: &mut Context<Self>) {
        self.follower_states.retain(|leader_id, state| {
            if *leader_id == CollaboratorId::PeerId(peer_id) {
                for item in state.items_by_leader_view_id.values() {
                    item.view.set_leader_id(None, window, cx);
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
        leader_id: impl Into<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let leader_id = leader_id.into();
        let pane = self.active_pane().clone();

        self.last_leaders_by_pane
            .insert(pane.downgrade(), leader_id);
        self.unfollow(leader_id, window, cx);
        self.unfollow_in_pane(&pane, window, cx);
        self.follower_states.insert(
            leader_id,
            FollowerState {
                center_pane: pane.clone(),
                dock_pane: None,
                active_view_id: None,
                items_by_leader_view_id: Default::default(),
            },
        );
        cx.notify();

        match leader_id {
            CollaboratorId::PeerId(leader_peer_id) => {
                let room_id = self.active_call()?.read(cx).room()?.read(cx).id();
                let project_id = self.project.read(cx).remote_id();
                let request = self.app_state.client.request(proto::Follow {
                    room_id,
                    project_id,
                    leader_id: Some(leader_peer_id),
                });

                Some(cx.spawn_in(window, async move |this, cx| {
                    let response = request.await?;
                    this.update(cx, |this, _| {
                        let state = this
                            .follower_states
                            .get_mut(&leader_id)
                            .ok_or_else(|| anyhow!("following interrupted"))?;
                        state.active_view_id = response
                            .active_view
                            .as_ref()
                            .and_then(|view| ViewId::from_proto(view.id.clone()?).ok());
                        Ok::<_, anyhow::Error>(())
                    })??;
                    if let Some(view) = response.active_view {
                        Self::add_view_from_leader(this.clone(), leader_peer_id, &view, cx).await?;
                    }
                    this.update_in(cx, |this, window, cx| {
                        this.leader_updated(leader_id, window, cx)
                    })?;
                    Ok(())
                }))
            }
            CollaboratorId::Agent => {
                self.leader_updated(leader_id, window, cx)?;
                Some(Task::ready(Ok(())))
            }
        }
    }

    pub fn follow_next_collaborator(
        &mut self,
        _: &FollowNextCollaborator,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let collaborators = self.project.read(cx).collaborators();
        let next_leader_id = if let Some(leader_id) = self.leader_for_pane(&self.active_pane) {
            let mut collaborators = collaborators.keys().copied();
            for peer_id in collaborators.by_ref() {
                if CollaboratorId::PeerId(peer_id) == leader_id {
                    break;
                }
            }
            collaborators.next().map(CollaboratorId::PeerId)
        } else if let Some(last_leader_id) =
            self.last_leaders_by_pane.get(&self.active_pane.downgrade())
        {
            match last_leader_id {
                CollaboratorId::PeerId(peer_id) => {
                    if collaborators.contains_key(peer_id) {
                        Some(*last_leader_id)
                    } else {
                        None
                    }
                }
                CollaboratorId::Agent => Some(CollaboratorId::Agent),
            }
        } else {
            None
        };

        let pane = self.active_pane.clone();
        let Some(leader_id) = next_leader_id.or_else(|| {
            Some(CollaboratorId::PeerId(
                collaborators.keys().copied().next()?,
            ))
        }) else {
            return;
        };
        if self.unfollow_in_pane(&pane, window, cx) == Some(leader_id) {
            return;
        }
        if let Some(task) = self.start_following(leader_id, window, cx) {
            task.detach_and_log_err(cx)
        }
    }

    pub fn follow(
        &mut self,
        leader_id: impl Into<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let leader_id = leader_id.into();

        if let CollaboratorId::PeerId(peer_id) = leader_id {
            let Some(room) = ActiveCall::global(cx).read(cx).room() else {
                return;
            };
            let room = room.read(cx);
            let Some(remote_participant) = room.remote_participant_for_peer_id(peer_id) else {
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
        }

        // if you're already following, find the right pane and focus it.
        if let Some(follower_state) = self.follower_states.get(&leader_id) {
            window.focus(&follower_state.pane().focus_handle(cx));

            return;
        }

        // Otherwise, follow.
        if let Some(task) = self.start_following(leader_id, window, cx) {
            task.detach_and_log_err(cx)
        }
    }

    pub fn unfollow(
        &mut self,
        leader_id: impl Into<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        cx.notify();

        let leader_id = leader_id.into();
        let state = self.follower_states.remove(&leader_id)?;
        for (_, item) in state.items_by_leader_view_id {
            item.view.set_leader_id(None, window, cx);
        }

        if let CollaboratorId::PeerId(leader_peer_id) = leader_id {
            let project_id = self.project.read(cx).remote_id();
            let room_id = self.active_call()?.read(cx).room()?.read(cx).id();
            self.app_state
                .client
                .send(proto::Unfollow {
                    room_id,
                    project_id,
                    leader_id: Some(leader_peer_id),
                })
                .log_err();
        }

        Some(())
    }

    pub fn is_being_followed(&self, id: impl Into<CollaboratorId>) -> bool {
        self.follower_states.contains_key(&id.into())
    }

    fn active_item_path_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(Event::ActiveItemChanged);
        let active_entry = self.active_project_path(cx);
        self.project
            .update(cx, |project, cx| project.set_active_path(active_entry, cx));

        self.update_window_title(window, cx);
    }

    fn update_window_title(&mut self, window: &mut Window, cx: &mut App) {
        let project = self.project().read(cx);
        let mut title = String::new();

        for (i, name) in project.worktree_root_names(cx).enumerate() {
            if i > 0 {
                title.push_str(", ");
            }
            title.push_str(name);
        }

        if title.is_empty() {
            title = "empty project".to_string();
        }

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
                title.push_str(" — ");
                title.push_str(filename.as_ref());
            }
        }

        if project.is_via_collab() {
            title.push_str(" ↙");
        } else if project.is_shared() {
            title.push_str(" ↗");
        }

        window.set_window_title(&title);
    }

    fn update_window_edited(&mut self, window: &mut Window, cx: &mut App) {
        let is_edited = !self.project.read(cx).is_disconnected(cx) && !self.dirty_items.is_empty();
        if is_edited != self.window_edited {
            self.window_edited = is_edited;
            window.set_window_edited(self.window_edited)
        }
    }

    fn update_item_dirty_state(
        &mut self,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) {
        let is_dirty = item.is_dirty(cx);
        let item_id = item.item_id();
        let was_dirty = self.dirty_items.contains_key(&item_id);
        if is_dirty == was_dirty {
            return;
        }
        if was_dirty {
            self.dirty_items.remove(&item_id);
            self.update_window_edited(window, cx);
            return;
        }
        if let Some(window_handle) = window.window_handle().downcast::<Self>() {
            let s = item.on_release(
                cx,
                Box::new(move |cx| {
                    window_handle
                        .update(cx, |this, window, cx| {
                            this.dirty_items.remove(&item_id);
                            this.update_window_edited(window, cx)
                        })
                        .ok();
                }),
            );
            self.dirty_items.insert(item_id, s);
            self.update_window_edited(window, cx);
        }
    }

    fn render_notifications(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Option<Div> {
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
                            .map(|(_, notification)| notification.clone().into_any()),
                    ),
            )
        }
    }

    // RPC handlers

    fn active_view_for_follower(
        &self,
        follower_project_id: Option<u64>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<proto::View> {
        let (item, panel_id) = self.active_item_for_followers(window, cx);
        let item = item?;
        let leader_id = self
            .pane_for(&*item)
            .and_then(|pane| self.leader_for_pane(&pane));
        let leader_peer_id = match leader_id {
            Some(CollaboratorId::PeerId(peer_id)) => Some(peer_id),
            Some(CollaboratorId::Agent) | None => None,
        };

        let item_handle = item.to_followable_item_handle(cx)?;
        let id = item_handle.remote_id(&self.app_state.client, window, cx)?;
        let variant = item_handle.to_state_proto(window, cx)?;

        if item_handle.is_project_item(window, cx)
            && (follower_project_id.is_none()
                || follower_project_id != self.project.read(cx).remote_id())
        {
            return None;
        }

        Some(proto::View {
            id: id.to_proto(),
            leader_id: leader_peer_id,
            variant: Some(variant),
            panel_id: panel_id.map(|id| id as i32),
        })
    }

    fn handle_follow(
        &mut self,
        follower_project_id: Option<u64>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> proto::FollowResponse {
        let active_view = self.active_view_for_follower(follower_project_id, window, cx);

        cx.notify();
        proto::FollowResponse {
            // TODO: Remove after version 0.145.x stabilizes.
            active_view_id: active_view.as_ref().and_then(|view| view.id.clone()),
            views: active_view.iter().cloned().collect(),
            active_view,
        }
    }

    fn handle_update_followers(
        &mut self,
        leader_id: PeerId,
        message: proto::UpdateFollowers,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.leader_updates_tx
            .unbounded_send((leader_id, message))
            .ok();
    }

    async fn process_leader_update(
        this: &WeakEntity<Self>,
        leader_id: PeerId,
        update: proto::UpdateFollowers,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        match update.variant.ok_or_else(|| anyhow!("invalid update"))? {
            proto::update_followers::Variant::CreateView(view) => {
                let view_id = ViewId::from_proto(view.id.clone().context("invalid view id")?)?;
                let should_add_view = this.update(cx, |this, _| {
                    if let Some(state) = this.follower_states.get_mut(&leader_id.into()) {
                        anyhow::Ok(!state.items_by_leader_view_id.contains_key(&view_id))
                    } else {
                        anyhow::Ok(false)
                    }
                })??;

                if should_add_view {
                    Self::add_view_from_leader(this.clone(), leader_id, &view, cx).await?
                }
            }
            proto::update_followers::Variant::UpdateActiveView(update_active_view) => {
                let should_add_view = this.update(cx, |this, _| {
                    if let Some(state) = this.follower_states.get_mut(&leader_id.into()) {
                        state.active_view_id = update_active_view
                            .view
                            .as_ref()
                            .and_then(|view| ViewId::from_proto(view.id.clone()?).ok());

                        if state.active_view_id.is_some_and(|view_id| {
                            !state.items_by_leader_view_id.contains_key(&view_id)
                        }) {
                            anyhow::Ok(true)
                        } else {
                            anyhow::Ok(false)
                        }
                    } else {
                        anyhow::Ok(false)
                    }
                })??;

                if should_add_view {
                    if let Some(view) = update_active_view.view {
                        Self::add_view_from_leader(this.clone(), leader_id, &view, cx).await?
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
                this.update_in(cx, |this, window, cx| {
                    let project = this.project.clone();
                    if let Some(state) = this.follower_states.get(&leader_id.into()) {
                        let view_id = ViewId::from_proto(id.clone())?;
                        if let Some(item) = state.items_by_leader_view_id.get(&view_id) {
                            tasks.push(item.view.apply_update_proto(
                                &project,
                                variant.clone(),
                                window,
                                cx,
                            ));
                        }
                    }
                    anyhow::Ok(())
                })??;
                try_join_all(tasks).await.log_err();
            }
        }
        this.update_in(cx, |this, window, cx| {
            this.leader_updated(leader_id, window, cx)
        })?;
        Ok(())
    }

    async fn add_view_from_leader(
        this: WeakEntity<Self>,
        leader_id: PeerId,
        view: &proto::View,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let this = this.upgrade().context("workspace dropped")?;

        let Some(id) = view.id.clone() else {
            return Err(anyhow!("no id for view"));
        };
        let id = ViewId::from_proto(id)?;
        let panel_id = view.panel_id.and_then(proto::PanelId::from_i32);

        let pane = this.update(cx, |this, _cx| {
            let state = this
                .follower_states
                .get(&leader_id.into())
                .context("stopped following")?;
            anyhow::Ok(state.pane().clone())
        })??;
        let existing_item = pane.update_in(cx, |pane, window, cx| {
            let client = this.read(cx).client().clone();
            pane.items().find_map(|item| {
                let item = item.to_followable_item_handle(cx)?;
                if item.remote_id(&client, window, cx) == Some(id) {
                    Some(item)
                } else {
                    None
                }
            })
        })?;
        let item = if let Some(existing_item) = existing_item {
            existing_item
        } else {
            let variant = view.variant.clone();
            if variant.is_none() {
                Err(anyhow!("missing view variant"))?;
            }

            let task = cx.update(|window, cx| {
                FollowableViewRegistry::from_state_proto(this.clone(), id, variant, window, cx)
            })?;

            let Some(task) = task else {
                return Err(anyhow!(
                    "failed to construct view from leader (maybe from a different version of zed?)"
                ));
            };

            let mut new_item = task.await?;
            pane.update_in(cx, |pane, window, cx| {
                let mut item_to_remove = None;
                for (ix, item) in pane.items().enumerate() {
                    if let Some(item) = item.to_followable_item_handle(cx) {
                        match new_item.dedup(item.as_ref(), window, cx) {
                            Some(item::Dedup::KeepExisting) => {
                                new_item =
                                    item.boxed_clone().to_followable_item_handle(cx).unwrap();
                                break;
                            }
                            Some(item::Dedup::ReplaceExisting) => {
                                item_to_remove = Some((ix, item.item_id()));
                                break;
                            }
                            None => {}
                        }
                    }
                }

                if let Some((ix, id)) = item_to_remove {
                    pane.remove_item(id, false, false, window, cx);
                    pane.add_item(new_item.boxed_clone(), false, false, Some(ix), window, cx);
                }
            })?;

            new_item
        };

        this.update_in(cx, |this, window, cx| {
            let state = this.follower_states.get_mut(&leader_id.into())?;
            item.set_leader_id(Some(leader_id.into()), window, cx);
            state.items_by_leader_view_id.insert(
                id,
                FollowerView {
                    view: item,
                    location: panel_id,
                },
            );

            Some(())
        })?;

        Ok(())
    }

    fn handle_agent_location_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(follower_state) = self.follower_states.get_mut(&CollaboratorId::Agent) else {
            return;
        };

        if let Some(agent_location) = self.project.read(cx).agent_location() {
            let buffer_entity_id = agent_location.buffer.entity_id();
            let view_id = ViewId {
                creator: CollaboratorId::Agent,
                id: buffer_entity_id.as_u64(),
            };
            follower_state.active_view_id = Some(view_id);

            let item = match follower_state.items_by_leader_view_id.entry(view_id) {
                hash_map::Entry::Occupied(entry) => Some(entry.into_mut()),
                hash_map::Entry::Vacant(entry) => {
                    let existing_view =
                        follower_state
                            .center_pane
                            .read(cx)
                            .items()
                            .find_map(|item| {
                                let item = item.to_followable_item_handle(cx)?;
                                if item.is_singleton(cx)
                                    && item.project_item_model_ids(cx).as_slice()
                                        == [buffer_entity_id]
                                {
                                    Some(item)
                                } else {
                                    None
                                }
                            });
                    let view = existing_view.or_else(|| {
                        agent_location.buffer.upgrade().and_then(|buffer| {
                            cx.update_default_global(|registry: &mut ProjectItemRegistry, cx| {
                                registry.build_item(buffer, self.project.clone(), None, window, cx)
                            })?
                            .to_followable_item_handle(cx)
                        })
                    });

                    if let Some(view) = view {
                        Some(entry.insert(FollowerView {
                            view,
                            location: None,
                        }))
                    } else {
                        None
                    }
                }
            };

            if let Some(item) = item {
                item.view
                    .set_leader_id(Some(CollaboratorId::Agent), window, cx);
                item.view
                    .update_agent_location(agent_location.position, window, cx);
            }
        } else {
            follower_state.active_view_id = None;
        }

        self.leader_updated(CollaboratorId::Agent, window, cx);
    }

    pub fn update_active_view_for_followers(&mut self, window: &mut Window, cx: &mut App) {
        let mut is_project_item = true;
        let mut update = proto::UpdateActiveView::default();
        if window.is_window_active() {
            let (active_item, panel_id) = self.active_item_for_followers(window, cx);

            if let Some(item) = active_item {
                if item.item_focus_handle(cx).contains_focused(window, cx) {
                    let leader_id = self
                        .pane_for(&*item)
                        .and_then(|pane| self.leader_for_pane(&pane));
                    let leader_peer_id = match leader_id {
                        Some(CollaboratorId::PeerId(peer_id)) => Some(peer_id),
                        Some(CollaboratorId::Agent) | None => None,
                    };

                    if let Some(item) = item.to_followable_item_handle(cx) {
                        let id = item
                            .remote_id(&self.app_state.client, window, cx)
                            .map(|id| id.to_proto());

                        if let Some(id) = id.clone() {
                            if let Some(variant) = item.to_state_proto(window, cx) {
                                let view = Some(proto::View {
                                    id: id.clone(),
                                    leader_id: leader_peer_id,
                                    variant: Some(variant),
                                    panel_id: panel_id.map(|id| id as i32),
                                });

                                is_project_item = item.is_project_item(window, cx);
                                update = proto::UpdateActiveView {
                                    view,
                                    // TODO: Remove after version 0.145.x stabilizes.
                                    id: id.clone(),
                                    leader_id: leader_peer_id,
                                };
                            }
                        };
                    }
                }
            }
        }

        let active_view_id = update.view.as_ref().and_then(|view| view.id.as_ref());
        if active_view_id != self.last_active_view_id.as_ref() {
            self.last_active_view_id = active_view_id.cloned();
            self.update_followers(
                is_project_item,
                proto::update_followers::Variant::UpdateActiveView(update),
                window,
                cx,
            );
        }
    }

    fn active_item_for_followers(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> (Option<Box<dyn ItemHandle>>, Option<proto::PanelId>) {
        let mut active_item = None;
        let mut panel_id = None;
        for dock in self.all_docks() {
            if dock.focus_handle(cx).contains_focused(window, cx) {
                if let Some(panel) = dock.read(cx).active_panel() {
                    if let Some(pane) = panel.pane(cx) {
                        if let Some(item) = pane.read(cx).active_item() {
                            active_item = Some(item);
                            panel_id = panel.remote_id();
                            break;
                        }
                    }
                }
            }
        }

        if active_item.is_none() {
            active_item = self.active_pane().read(cx).active_item();
        }
        (active_item, panel_id)
    }

    fn update_followers(
        &self,
        project_only: bool,
        update: proto::update_followers::Variant,
        _: &mut Window,
        cx: &mut App,
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

    pub fn leader_for_pane(&self, pane: &Entity<Pane>) -> Option<CollaboratorId> {
        self.follower_states.iter().find_map(|(leader_id, state)| {
            if state.center_pane == *pane || state.dock_pane.as_ref() == Some(pane) {
                Some(*leader_id)
            } else {
                None
            }
        })
    }

    fn leader_updated(
        &mut self,
        leader_id: impl Into<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Box<dyn ItemHandle>> {
        cx.notify();

        let leader_id = leader_id.into();
        let (panel_id, item) = match leader_id {
            CollaboratorId::PeerId(peer_id) => self.active_item_for_peer(peer_id, window, cx)?,
            CollaboratorId::Agent => (None, self.active_item_for_agent()?),
        };

        let state = self.follower_states.get(&leader_id)?;
        let mut transfer_focus = state.center_pane.read(cx).has_focus(window, cx);
        let pane;
        if let Some(panel_id) = panel_id {
            pane = self
                .activate_panel_for_proto_id(panel_id, window, cx)?
                .pane(cx)?;
            let state = self.follower_states.get_mut(&leader_id)?;
            state.dock_pane = Some(pane.clone());
        } else {
            pane = state.center_pane.clone();
            let state = self.follower_states.get_mut(&leader_id)?;
            if let Some(dock_pane) = state.dock_pane.take() {
                transfer_focus |= dock_pane.focus_handle(cx).contains_focused(window, cx);
            }
        }

        pane.update(cx, |pane, cx| {
            let focus_active_item = pane.has_focus(window, cx) || transfer_focus;
            if let Some(index) = pane.index_for_item(item.as_ref()) {
                pane.activate_item(index, false, false, window, cx);
            } else {
                pane.add_item(item.boxed_clone(), false, false, None, window, cx)
            }

            if focus_active_item {
                pane.focus_active_item(window, cx)
            }
        });

        Some(item)
    }

    fn active_item_for_agent(&self) -> Option<Box<dyn ItemHandle>> {
        let state = self.follower_states.get(&CollaboratorId::Agent)?;
        let active_view_id = state.active_view_id?;
        Some(
            state
                .items_by_leader_view_id
                .get(&active_view_id)?
                .view
                .boxed_clone(),
        )
    }

    fn active_item_for_peer(
        &self,
        peer_id: PeerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<(Option<PanelId>, Box<dyn ItemHandle>)> {
        let call = self.active_call()?;
        let room = call.read(cx).room()?.read(cx);
        let participant = room.remote_participant_for_peer_id(peer_id)?;
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
        let state = self.follower_states.get(&peer_id.into())?;
        let mut item_to_activate = None;
        if let (Some(active_view_id), true) = (state.active_view_id, leader_in_this_app) {
            if let Some(item) = state.items_by_leader_view_id.get(&active_view_id) {
                if leader_in_this_project || !item.view.is_project_item(window, cx) {
                    item_to_activate = Some((item.location, item.view.boxed_clone()));
                }
            }
        } else if let Some(shared_screen) =
            self.shared_screen_for_peer(peer_id, &state.center_pane, window, cx)
        {
            item_to_activate = Some((None, Box::new(shared_screen)));
        }
        item_to_activate
    }

    fn shared_screen_for_peer(
        &self,
        peer_id: PeerId,
        pane: &Entity<Pane>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Entity<SharedScreen>> {
        let call = self.active_call()?;
        let room = call.read(cx).room()?.clone();
        let participant = room.read(cx).remote_participant_for_peer_id(peer_id)?;
        let track = participant.video_tracks.values().next()?.clone();
        let user = participant.user.clone();

        for item in pane.read(cx).items_of_type::<SharedScreen>() {
            if item.read(cx).peer_id == peer_id {
                return Some(item);
            }
        }

        Some(cx.new(|cx| SharedScreen::new(track, peer_id, user.clone(), room.clone(), window, cx)))
    }

    pub fn on_window_activation_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if window.is_window_active() {
            self.update_active_view_for_followers(window, cx);

            if let Some(database_id) = self.database_id {
                cx.background_spawn(persistence::DB.update_timestamp(database_id))
                    .detach();
            }
        } else {
            for pane in &self.panes {
                pane.update(cx, |pane, cx| {
                    if let Some(item) = pane.active_item() {
                        item.workspace_deactivated(window, cx);
                    }
                    for item in pane.items() {
                        if matches!(
                            item.workspace_settings(cx).autosave,
                            AutosaveSetting::OnWindowChange | AutosaveSetting::OnFocusChange
                        ) {
                            Pane::autosave_item(item.as_ref(), self.project.clone(), window, cx)
                                .detach_and_log_err(cx);
                        }
                    }
                });
            }
        }
    }

    pub fn active_call(&self) -> Option<&Entity<ActiveCall>> {
        self.active_call.as_ref().map(|(call, _)| call)
    }

    fn on_active_call_event(
        &mut self,
        _: &Entity<ActiveCall>,
        event: &call::room::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            call::room::Event::ParticipantLocationChanged { participant_id }
            | call::room::Event::RemoteVideoTracksChanged { participant_id } => {
                self.leader_updated(participant_id, window, cx);
            }
            _ => {}
        }
    }

    pub fn database_id(&self) -> Option<WorkspaceId> {
        self.database_id
    }

    pub fn session_id(&self) -> Option<String> {
        self.session_id.clone()
    }

    fn local_paths(&self, cx: &App) -> Option<Vec<Arc<Path>>> {
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

    fn remove_panes(&mut self, member: Member, window: &mut Window, cx: &mut Context<Workspace>) {
        match member {
            Member::Axis(PaneAxis { members, .. }) => {
                for child in members.iter() {
                    self.remove_panes(child.clone(), window, cx)
                }
            }
            Member::Pane(pane) => {
                self.force_remove_pane(&pane, &None, window, cx);
            }
        }
    }

    fn remove_from_session(&mut self, window: &mut Window, cx: &mut App) -> Task<()> {
        self.session_id.take();
        self.serialize_workspace_internal(window, cx)
    }

    fn force_remove_pane(
        &mut self,
        pane: &Entity<Pane>,
        focus_on: &Option<Entity<Pane>>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        self.panes.retain(|p| p != pane);
        if let Some(focus_on) = focus_on {
            focus_on.update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)));
        } else {
            if self.active_pane() == pane {
                self.panes
                    .last()
                    .unwrap()
                    .update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)));
            }
        }
        if self.last_active_center_pane == Some(pane.downgrade()) {
            self.last_active_center_pane = None;
        }
        cx.notify();
    }

    fn serialize_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._schedule_serialize.is_none() {
            self._schedule_serialize = Some(cx.spawn_in(window, async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
                this.update_in(cx, |this, window, cx| {
                    this.serialize_workspace_internal(window, cx).detach();
                    this._schedule_serialize.take();
                })
                .log_err();
            }));
        }
    }

    fn serialize_workspace_internal(&self, window: &mut Window, cx: &mut App) -> Task<()> {
        let Some(database_id) = self.database_id() else {
            return Task::ready(());
        };

        fn serialize_pane_handle(
            pane_handle: &Entity<Pane>,
            window: &mut Window,
            cx: &mut App,
        ) -> SerializedPane {
            let (items, active, pinned_count) = {
                let pane = pane_handle.read(cx);
                let active_item_id = pane.active_item().map(|item| item.item_id());
                (
                    pane.items()
                        .filter_map(|handle| {
                            let handle = handle.to_serializable_item_handle(cx)?;

                            Some(SerializedItem {
                                kind: Arc::from(handle.serialized_item_kind()),
                                item_id: handle.item_id().as_u64(),
                                active: Some(handle.item_id()) == active_item_id,
                                preview: pane.is_active_preview_item(handle.item_id()),
                            })
                        })
                        .collect::<Vec<_>>(),
                    pane.has_focus(window, cx),
                    pane.pinned_count(),
                )
            };

            SerializedPane::new(items, active, pinned_count)
        }

        fn build_serialized_pane_group(
            pane_group: &Member,
            window: &mut Window,
            cx: &mut App,
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
                        .map(|member| build_serialized_pane_group(member, window, cx))
                        .collect::<Vec<_>>(),
                    flexes: Some(flexes.lock().clone()),
                },
                Member::Pane(pane_handle) => {
                    SerializedPaneGroup::Pane(serialize_pane_handle(pane_handle, window, cx))
                }
            }
        }

        fn build_serialized_docks(
            this: &Workspace,
            window: &mut Window,
            cx: &mut App,
        ) -> DockStructure {
            let left_dock = this.left_dock.read(cx);
            let left_visible = left_dock.is_open();
            let left_active_panel = left_dock
                .active_panel()
                .map(|panel| panel.persistent_name().to_string());
            let left_dock_zoom = left_dock
                .active_panel()
                .map(|panel| panel.is_zoomed(window, cx))
                .unwrap_or(false);

            let right_dock = this.right_dock.read(cx);
            let right_visible = right_dock.is_open();
            let right_active_panel = right_dock
                .active_panel()
                .map(|panel| panel.persistent_name().to_string());
            let right_dock_zoom = right_dock
                .active_panel()
                .map(|panel| panel.is_zoomed(window, cx))
                .unwrap_or(false);

            let bottom_dock = this.bottom_dock.read(cx);
            let bottom_visible = bottom_dock.is_open();
            let bottom_active_panel = bottom_dock
                .active_panel()
                .map(|panel| panel.persistent_name().to_string());
            let bottom_dock_zoom = bottom_dock
                .active_panel()
                .map(|panel| panel.is_zoomed(window, cx))
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

        if let Some(location) = self.serialize_workspace_location(cx) {
            let breakpoints = self.project.update(cx, |project, cx| {
                project.breakpoint_store().read(cx).all_breakpoints(cx)
            });

            let center_group = build_serialized_pane_group(&self.center.root, window, cx);
            let docks = build_serialized_docks(self, window, cx);
            let window_bounds = Some(SerializedWindowBounds(window.window_bounds()));
            let serialized_workspace = SerializedWorkspace {
                id: database_id,
                location,
                center_group,
                window_bounds,
                display: Default::default(),
                docks,
                centered_layout: self.centered_layout,
                session_id: self.session_id.clone(),
                breakpoints,
                window_id: Some(window.window_handle().window_id().as_u64()),
            };

            return window.spawn(cx, async move |_| {
                persistence::DB.save_workspace(serialized_workspace).await;
            });
        }
        Task::ready(())
    }

    fn serialize_workspace_location(&self, cx: &App) -> Option<SerializedWorkspaceLocation> {
        if let Some(ssh_project) = &self.serialized_ssh_project {
            Some(SerializedWorkspaceLocation::Ssh(ssh_project.clone()))
        } else if let Some(local_paths) = self.local_paths(cx) {
            if !local_paths.is_empty() {
                Some(SerializedWorkspaceLocation::from_local_paths(local_paths))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn update_history(&self, cx: &mut App) {
        let Some(id) = self.database_id() else {
            return;
        };
        let Some(location) = self.serialize_workspace_location(cx) else {
            return;
        };
        if let Some(manager) = HistoryManager::global(cx) {
            manager.update(cx, |this, cx| {
                this.update_history(id, HistoryManagerEntry::new(id, &location), cx);
            });
        }
    }

    async fn serialize_items(
        this: &WeakEntity<Self>,
        items_rx: UnboundedReceiver<Box<dyn SerializableItemHandle>>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        const CHUNK_SIZE: usize = 200;

        let mut serializable_items = items_rx.ready_chunks(CHUNK_SIZE);

        while let Some(items_received) = serializable_items.next().await {
            let unique_items =
                items_received
                    .into_iter()
                    .fold(HashMap::default(), |mut acc, item| {
                        acc.entry(item.item_id()).or_insert(item);
                        acc
                    });

            // We use into_iter() here so that the references to the items are moved into
            // the tasks and not kept alive while we're sleeping.
            for (_, item) in unique_items.into_iter() {
                if let Ok(Some(task)) = this.update_in(cx, |workspace, window, cx| {
                    item.serialize(workspace, false, window, cx)
                }) {
                    cx.background_spawn(async move { task.await.log_err() })
                        .detach();
                }
            }

            cx.background_executor()
                .timer(SERIALIZATION_THROTTLE_TIME)
                .await;
        }

        Ok(())
    }

    pub(crate) fn enqueue_item_serialization(
        &mut self,
        item: Box<dyn SerializableItemHandle>,
    ) -> Result<()> {
        self.serializable_items_tx
            .unbounded_send(item)
            .map_err(|err| anyhow!("failed to send serializable item over channel: {}", err))
    }

    pub(crate) fn load_workspace(
        serialized_workspace: SerializedWorkspace,
        paths_to_open: Vec<Option<ProjectPath>>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<Vec<Option<Box<dyn ItemHandle>>>>> {
        cx.spawn_in(window, async move |workspace, cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;

            let mut center_group = None;
            let mut center_items = None;

            // Traverse the splits tree and add to things
            if let Some((group, active_pane, items)) = serialized_workspace
                .center_group
                .deserialize(&project, serialized_workspace.id, workspace.clone(), cx)
                .await
            {
                center_items = Some(items);
                center_group = Some((group, active_pane))
            }

            let mut items_by_project_path = HashMap::default();
            let mut item_ids_by_kind = HashMap::default();
            let mut all_deserialized_items = Vec::default();
            cx.update(|_, cx| {
                for item in center_items.unwrap_or_default().into_iter().flatten() {
                    if let Some(serializable_item_handle) = item.to_serializable_item_handle(cx) {
                        item_ids_by_kind
                            .entry(serializable_item_handle.serialized_item_kind())
                            .or_insert(Vec::new())
                            .push(item.item_id().as_u64() as ItemId);
                    }

                    if let Some(project_path) = item.project_path(cx) {
                        items_by_project_path.insert(project_path, item.clone());
                    }
                    all_deserialized_items.push(item);
                }
            })?;

            let opened_items = paths_to_open
                .into_iter()
                .map(|path_to_open| {
                    path_to_open
                        .and_then(|path_to_open| items_by_project_path.remove(&path_to_open))
                })
                .collect::<Vec<_>>();

            // Remove old panes from workspace panes list
            workspace.update_in(cx, |workspace, window, cx| {
                if let Some((center_group, active_pane)) = center_group {
                    workspace.remove_panes(workspace.center.root.clone(), window, cx);

                    // Swap workspace center group
                    workspace.center = PaneGroup::with_root(center_group);
                    if let Some(active_pane) = active_pane {
                        workspace.set_active_pane(&active_pane, window, cx);
                        cx.focus_self(window);
                    } else {
                        workspace.set_active_pane(&workspace.center.first_pane(), window, cx);
                    }
                }

                let docks = serialized_workspace.docks;

                for (dock, serialized_dock) in [
                    (&mut workspace.right_dock, docks.right),
                    (&mut workspace.left_dock, docks.left),
                    (&mut workspace.bottom_dock, docks.bottom),
                ]
                .iter_mut()
                {
                    dock.update(cx, |dock, cx| {
                        dock.serialized_dock = Some(serialized_dock.clone());
                        dock.restore_state(window, cx);
                    });
                }

                cx.notify();
            })?;

            let _ = project
                .update(cx, |project, cx| {
                    project
                        .breakpoint_store()
                        .update(cx, |breakpoint_store, cx| {
                            breakpoint_store
                                .with_serialized_breakpoints(serialized_workspace.breakpoints, cx)
                        })
                })?
                .await;

            // Clean up all the items that have _not_ been loaded. Our ItemIds aren't stable. That means
            // after loading the items, we might have different items and in order to avoid
            // the database filling up, we delete items that haven't been loaded now.
            //
            // The items that have been loaded, have been saved after they've been added to the workspace.
            let clean_up_tasks = workspace.update_in(cx, |_, window, cx| {
                item_ids_by_kind
                    .into_iter()
                    .map(|(item_kind, loaded_items)| {
                        SerializableItemRegistry::cleanup(
                            item_kind,
                            serialized_workspace.id,
                            loaded_items,
                            window,
                            cx,
                        )
                        .log_err()
                    })
                    .collect::<Vec<_>>()
            })?;

            futures::future::join_all(clean_up_tasks).await;

            workspace
                .update_in(cx, |workspace, window, cx| {
                    // Serialize ourself to make sure our timestamps and any pane / item changes are replicated
                    workspace.serialize_workspace_internal(window, cx).detach();

                    // Ensure that we mark the window as edited if we did load dirty items
                    workspace.update_window_edited(window, cx);
                })
                .ok();

            Ok(opened_items)
        })
    }

    fn actions(&self, div: Div, window: &mut Window, cx: &mut Context<Self>) -> Div {
        self.add_workspace_actions_listeners(div, window, cx)
            .on_action(cx.listener(Self::close_inactive_items_and_panes))
            .on_action(cx.listener(Self::close_all_items_and_panes))
            .on_action(cx.listener(Self::save_all))
            .on_action(cx.listener(Self::send_keystrokes))
            .on_action(cx.listener(Self::add_folder_to_project))
            .on_action(cx.listener(Self::follow_next_collaborator))
            .on_action(cx.listener(Self::close_window))
            .on_action(cx.listener(Self::activate_pane_at_index))
            .on_action(cx.listener(Self::move_item_to_pane_at_index))
            .on_action(cx.listener(Self::move_focused_panel_to_next_position))
            .on_action(cx.listener(|workspace, _: &Unfollow, window, cx| {
                let pane = workspace.active_pane().clone();
                workspace.unfollow_in_pane(&pane, window, cx);
            }))
            .on_action(cx.listener(|workspace, action: &Save, window, cx| {
                workspace
                    .save_active_item(action.save_intent.unwrap_or(SaveIntent::Save), window, cx)
                    .detach_and_prompt_err("Failed to save", window, cx, |_, _, _| None);
            }))
            .on_action(cx.listener(|workspace, _: &SaveWithoutFormat, window, cx| {
                workspace
                    .save_active_item(SaveIntent::SaveWithoutFormat, window, cx)
                    .detach_and_prompt_err("Failed to save", window, cx, |_, _, _| None);
            }))
            .on_action(cx.listener(|workspace, _: &SaveAs, window, cx| {
                workspace
                    .save_active_item(SaveIntent::SaveAs, window, cx)
                    .detach_and_prompt_err("Failed to save", window, cx, |_, _, _| None);
            }))
            .on_action(
                cx.listener(|workspace, _: &ActivatePreviousPane, window, cx| {
                    workspace.activate_previous_pane(window, cx)
                }),
            )
            .on_action(cx.listener(|workspace, _: &ActivateNextPane, window, cx| {
                workspace.activate_next_pane(window, cx)
            }))
            .on_action(
                cx.listener(|workspace, _: &ActivateNextWindow, _window, cx| {
                    workspace.activate_next_window(cx)
                }),
            )
            .on_action(
                cx.listener(|workspace, _: &ActivatePreviousWindow, _window, cx| {
                    workspace.activate_previous_window(cx)
                }),
            )
            .on_action(cx.listener(|workspace, _: &ActivatePaneLeft, window, cx| {
                workspace.activate_pane_in_direction(SplitDirection::Left, window, cx)
            }))
            .on_action(cx.listener(|workspace, _: &ActivatePaneRight, window, cx| {
                workspace.activate_pane_in_direction(SplitDirection::Right, window, cx)
            }))
            .on_action(cx.listener(|workspace, _: &ActivatePaneUp, window, cx| {
                workspace.activate_pane_in_direction(SplitDirection::Up, window, cx)
            }))
            .on_action(cx.listener(|workspace, _: &ActivatePaneDown, window, cx| {
                workspace.activate_pane_in_direction(SplitDirection::Down, window, cx)
            }))
            .on_action(cx.listener(|workspace, _: &ActivateNextPane, window, cx| {
                workspace.activate_next_pane(window, cx)
            }))
            .on_action(cx.listener(
                |workspace, action: &MoveItemToPaneInDirection, window, cx| {
                    workspace.move_item_to_pane_in_direction(action, window, cx)
                },
            ))
            .on_action(cx.listener(|workspace, _: &SwapPaneLeft, _, cx| {
                workspace.swap_pane_in_direction(SplitDirection::Left, cx)
            }))
            .on_action(cx.listener(|workspace, _: &SwapPaneRight, _, cx| {
                workspace.swap_pane_in_direction(SplitDirection::Right, cx)
            }))
            .on_action(cx.listener(|workspace, _: &SwapPaneUp, _, cx| {
                workspace.swap_pane_in_direction(SplitDirection::Up, cx)
            }))
            .on_action(cx.listener(|workspace, _: &SwapPaneDown, _, cx| {
                workspace.swap_pane_in_direction(SplitDirection::Down, cx)
            }))
            .on_action(cx.listener(|this, _: &ToggleLeftDock, window, cx| {
                this.toggle_dock(DockPosition::Left, window, cx);
            }))
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &ToggleRightDock, window, cx| {
                    workspace.toggle_dock(DockPosition::Right, window, cx);
                },
            ))
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &ToggleBottomDock, window, cx| {
                    workspace.toggle_dock(DockPosition::Bottom, window, cx);
                },
            ))
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &CloseActiveDock, window, cx| {
                    workspace.close_active_dock(window, cx);
                },
            ))
            .on_action(
                cx.listener(|workspace: &mut Workspace, _: &CloseAllDocks, window, cx| {
                    workspace.close_all_docks(window, cx);
                }),
            )
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &ClearAllNotifications, _, cx| {
                    workspace.clear_all_notifications(cx);
                },
            ))
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &SuppressNotification, _, cx| {
                    if let Some((notification_id, _)) = workspace.notifications.pop() {
                        workspace.suppress_notification(&notification_id, cx);
                    }
                },
            ))
            .on_action(cx.listener(
                |workspace: &mut Workspace, _: &ReopenClosedItem, window, cx| {
                    workspace.reopen_closed_item(window, cx).detach();
                },
            ))
            .on_action(cx.listener(Workspace::toggle_centered_layout))
            .on_action(cx.listener(Workspace::cancel))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        use node_runtime::NodeRuntime;
        use session::Session;

        let client = project.read(cx).client();
        let user_store = project.read(cx).user_store();

        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
        let session = cx.new(|cx| AppSession::new(Session::test(), cx));
        window.activate_window();
        let app_state = Arc::new(AppState {
            languages: project.read(cx).languages().clone(),
            workspace_store,
            client,
            user_store,
            fs: project.read(cx).fs().clone(),
            build_window_options: |_, _| Default::default(),
            node_runtime: NodeRuntime::unavailable(),
            session,
        });
        let workspace = Self::new(Default::default(), project, app_state, window, cx);
        workspace
            .active_pane
            .update(cx, |pane, cx| window.focus(&pane.focus_handle(cx)));
        workspace
    }

    pub fn register_action<A: Action>(
        &mut self,
        callback: impl Fn(&mut Self, &A, &mut Window, &mut Context<Self>) + 'static,
    ) -> &mut Self {
        let callback = Arc::new(callback);

        self.workspace_actions.push(Box::new(move |div, _, cx| {
            let callback = callback.clone();
            div.on_action(cx.listener(move |workspace, event, window, cx| {
                (callback)(workspace, event, window, cx)
            }))
        }));
        self
    }

    fn add_workspace_actions_listeners(
        &self,
        mut div: Div,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        for action in self.workspace_actions.iter() {
            div = (action)(div, window, cx)
        }
        div
    }

    pub fn has_active_modal(&self, _: &mut Window, cx: &mut App) -> bool {
        self.modal_layer.read(cx).has_active_modal()
    }

    pub fn active_modal<V: ManagedView + 'static>(&self, cx: &App) -> Option<Entity<V>> {
        self.modal_layer.read(cx).active_modal()
    }

    pub fn toggle_modal<V: ModalView, B>(&mut self, window: &mut Window, cx: &mut App, build: B)
    where
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        self.modal_layer.update(cx, |modal_layer, cx| {
            modal_layer.toggle_modal(window, cx, build)
        })
    }

    pub fn toggle_status_toast<V: ToastView>(&mut self, entity: Entity<V>, cx: &mut App) {
        self.toast_layer
            .update(cx, |toast_layer, cx| toast_layer.toggle_toast(cx, entity))
    }

    pub fn toggle_centered_layout(
        &mut self,
        _: &ToggleCenteredLayout,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.centered_layout = !self.centered_layout;
        if let Some(database_id) = self.database_id() {
            cx.background_spawn(DB.set_centered_layout(database_id, self.centered_layout))
                .detach_and_log_err(cx);
        }
        cx.notify();
    }

    fn adjust_padding(padding: Option<f32>) -> f32 {
        padding
            .unwrap_or(Self::DEFAULT_PADDING)
            .clamp(0.0, Self::MAX_PADDING)
    }

    fn render_dock(
        &self,
        position: DockPosition,
        dock: &Entity<Dock>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Div> {
        if self.zoomed_position == Some(position) {
            return None;
        }

        let leader_border = dock.read(cx).active_panel().and_then(|panel| {
            let pane = panel.pane(cx)?;
            let follower_states = &self.follower_states;
            leader_border_for_pane(follower_states, &pane, window, cx)
        });

        Some(
            div()
                .flex()
                .flex_none()
                .overflow_hidden()
                .child(dock.clone())
                .children(leader_border),
        )
    }

    pub fn for_window(window: &mut Window, _: &mut App) -> Option<Entity<Workspace>> {
        window.root().flatten()
    }

    pub fn zoomed_item(&self) -> Option<&AnyWeakView> {
        self.zoomed.as_ref()
    }

    pub fn activate_next_window(&mut self, cx: &mut Context<Self>) {
        let Some(current_window_id) = cx.active_window().map(|a| a.window_id()) else {
            return;
        };
        let windows = cx.windows();
        let Some(next_window) = windows
            .iter()
            .cycle()
            .skip_while(|window| window.window_id() != current_window_id)
            .nth(1)
        else {
            return;
        };
        next_window
            .update(cx, |_, window, _| window.activate_window())
            .ok();
    }

    pub fn activate_previous_window(&mut self, cx: &mut Context<Self>) {
        let Some(current_window_id) = cx.active_window().map(|a| a.window_id()) else {
            return;
        };
        let windows = cx.windows();
        let Some(prev_window) = windows
            .iter()
            .rev()
            .cycle()
            .skip_while(|window| window.window_id() != current_window_id)
            .nth(1)
        else {
            return;
        };
        prev_window
            .update(cx, |_, window, _| window.activate_window())
            .ok();
    }

    pub fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if cx.stop_active_drag(window) {
            return;
        } else if let Some((notification_id, _)) = self.notifications.pop() {
            dismiss_app_notification(&notification_id, cx);
        } else {
            cx.emit(Event::ClearActivityIndicator);
            cx.propagate();
        }
    }
}

fn leader_border_for_pane(
    follower_states: &HashMap<CollaboratorId, FollowerState>,
    pane: &Entity<Pane>,
    _: &Window,
    cx: &App,
) -> Option<Div> {
    let (leader_id, _follower_state) = follower_states.iter().find_map(|(leader_id, state)| {
        if state.pane() == pane {
            Some((*leader_id, state))
        } else {
            None
        }
    })?;

    let mut leader_color = match leader_id {
        CollaboratorId::PeerId(leader_peer_id) => {
            let room = ActiveCall::try_global(cx)?.read(cx).room()?.read(cx);
            let leader = room.remote_participant_for_peer_id(leader_peer_id)?;

            cx.theme()
                .players()
                .color_for_participant(leader.participant_index.0)
                .cursor
        }
        CollaboratorId::Agent => cx.theme().players().agent().cursor,
    };
    leader_color.fade_out(0.3);
    Some(
        div()
            .absolute()
            .size_full()
            .left_0()
            .top_0()
            .border_2()
            .border_color(leader_color),
    )
}

fn window_bounds_env_override() -> Option<Bounds<Pixels>> {
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
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> impl 'static + Future<Output = Result<Vec<Option<Result<Box<dyn ItemHandle>>>>>> + use<> {
    let restored_items = serialized_workspace.map(|serialized_workspace| {
        Workspace::load_workspace(
            serialized_workspace,
            project_paths_to_open
                .iter()
                .map(|(_, project_path)| project_path)
                .cloned()
                .collect(),
            window,
            cx,
        )
    });

    cx.spawn_in(window, async move |workspace, cx| {
        let mut opened_items = Vec::with_capacity(project_paths_to_open.len());

        if let Some(restored_items) = restored_items {
            let restored_items = restored_items.await?;

            let restored_project_paths = restored_items
                .iter()
                .filter_map(|item| {
                    cx.update(|_, cx| item.as_ref()?.project_path(cx))
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
                    cx.spawn(async move |cx| {
                        let file_project_path = project_path?;
                        let abs_path_task = workspace.update(cx, |workspace, cx| {
                            workspace.project().update(cx, |project, cx| {
                                project.resolve_abs_path(abs_path.to_string_lossy().as_ref(), cx)
                            })
                        });

                        // We only want to open file paths here. If one of the items
                        // here is a directory, it was already opened further above
                        // with a `find_or_create_worktree`.
                        if let Ok(task) = abs_path_task {
                            if task.await.map_or(true, |p| p.is_file()) {
                                return Some((
                                    ix,
                                    workspace
                                        .update_in(cx, |workspace, window, cx| {
                                            workspace.open_path(
                                                file_project_path,
                                                None,
                                                true,
                                                window,
                                                cx,
                                            )
                                        })
                                        .log_err()?
                                        .await,
                                ));
                            }
                        }
                        None
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
    Pane(Entity<Pane>),
    Dock(Entity<Dock>),
}

fn notify_if_database_failed(workspace: WindowHandle<Workspace>, cx: &mut AsyncApp) {
    workspace
        .update(cx, |workspace, _, cx| {
            if (*db::ALL_FILE_DB_FAILED).load(std::sync::atomic::Ordering::Acquire) {
                struct DatabaseFailedNotification;

                workspace.show_notification(
                    NotificationId::unique::<DatabaseFailedNotification>(),
                    cx,
                    |cx| {
                        cx.new(|cx| {
                            MessageNotification::new("Failed to load the database file.", cx)
                                .primary_message("File an Issue")
                                .primary_icon(IconName::Plus)
                                .primary_on_click(|window, cx| {
                                    window.dispatch_action(Box::new(FileBugReport), cx)
                                })
                        })
                    },
                );
            }
        })
        .log_err();
}

impl Focusable for Workspace {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

#[derive(Clone)]
struct DraggedDock(DockPosition);

impl Render for DraggedDock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut context = KeyContext::new_with_defaults();
        context.add("Workspace");
        context.set("keyboard_layout", cx.keyboard_layout().name().to_string());
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
        let ui_font = theme::setup_ui_font(window, cx);

        let theme = cx.theme().clone();
        let colors = theme.colors();
        let notification_entities = self
            .notifications
            .iter()
            .map(|(_, notification)| notification.entity_id())
            .collect::<Vec<_>>();

        client_side_decorations(
            self.actions(div(), window, cx)
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
                .overflow_hidden()
                .children(self.titlebar_item.clone())
                .on_modifiers_changed(move |_, _, cx| {
                    for &id in &notification_entities {
                        cx.notify(id);
                    }
                })
                .child(
                    div()
                        .size_full()
                        .relative()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .id("workspace")
                                .bg(colors.background)
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
                                    let this = cx.entity().clone();
                                    canvas(
                                        move |bounds, window, cx| {
                                            this.update(cx, |this, cx| {
                                                let bounds_changed = this.bounds != bounds;
                                                this.bounds = bounds;

                                                if bounds_changed {
                                                    this.left_dock.update(cx, |dock, cx| {
                                                        dock.clamp_panel_size(
                                                            bounds.size.width,
                                                            window,
                                                            cx,
                                                        )
                                                    });

                                                    this.right_dock.update(cx, |dock, cx| {
                                                        dock.clamp_panel_size(
                                                            bounds.size.width,
                                                            window,
                                                            cx,
                                                        )
                                                    });

                                                    this.bottom_dock.update(cx, |dock, cx| {
                                                        dock.clamp_panel_size(
                                                            bounds.size.height,
                                                            window,
                                                            cx,
                                                        )
                                                    });
                                                }
                                            })
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .absolute()
                                    .size_full()
                                })
                                .when(self.zoomed.is_none(), |this| {
                                    this.on_drag_move(cx.listener(
                                        move |workspace,
                                              e: &DragMoveEvent<DraggedDock>,
                                              window,
                                              cx| {
                                            if workspace.previous_dock_drag_coordinates
                                                != Some(e.event.position)
                                            {
                                                workspace.previous_dock_drag_coordinates =
                                                    Some(e.event.position);
                                                match e.drag(cx).0 {
                                                    DockPosition::Left => {
                                                        resize_left_dock(
                                                            e.event.position.x
                                                                - workspace.bounds.left(),
                                                            workspace,
                                                            window,
                                                            cx,
                                                        );
                                                    }
                                                    DockPosition::Right => {
                                                        resize_right_dock(
                                                            workspace.bounds.right()
                                                                - e.event.position.x,
                                                            workspace,
                                                            window,
                                                            cx,
                                                        );
                                                    }
                                                    DockPosition::Bottom => {
                                                        resize_bottom_dock(
                                                            workspace.bounds.bottom()
                                                                - e.event.position.y,
                                                            workspace,
                                                            window,
                                                            cx,
                                                        );
                                                    }
                                                };
                                                workspace.serialize_workspace(window, cx);
                                            }
                                        },
                                    ))
                                })
                                .child({
                                    match self.bottom_dock_layout {
                                        BottomDockLayout::Full => div()
                                            .flex()
                                            .flex_col()
                                            .h_full()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .flex_1()
                                                    .overflow_hidden()
                                                    .children(self.render_dock(
                                                        DockPosition::Left,
                                                        &self.left_dock,
                                                        window,
                                                        cx,
                                                    ))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .flex_1()
                                                            .overflow_hidden()
                                                            .child(
                                                                h_flex()
                                                                    .flex_1()
                                                                    .when_some(
                                                                        paddings.0,
                                                                        |this, p| {
                                                                            this.child(
                                                                                p.border_r_1(),
                                                                            )
                                                                        },
                                                                    )
                                                                    .child(self.center.render(
                                                                        self.zoomed.as_ref(),
                                                                        &PaneRenderContext {
                                                                            follower_states:
                                                                                &self.follower_states,
                                                                            active_call: self.active_call(),
                                                                            active_pane: &self.active_pane,
                                                                            app_state: &self.app_state,
                                                                            project: &self.project,
                                                                            workspace: &self.weak_self,
                                                                        },
                                                                        window,
                                                                        cx,
                                                                    ))
                                                                    .when_some(
                                                                        paddings.1,
                                                                        |this, p| {
                                                                            this.child(
                                                                                p.border_l_1(),
                                                                            )
                                                                        },
                                                                    ),
                                                            ),
                                                    )
                                                    .children(self.render_dock(
                                                        DockPosition::Right,
                                                        &self.right_dock,
                                                        window,
                                                        cx,
                                                    )),
                                            )
                                            .child(div().w_full().children(self.render_dock(
                                                DockPosition::Bottom,
                                                &self.bottom_dock,
                                                window,
                                                cx
                                            ))),

                                        BottomDockLayout::LeftAligned => div()
                                            .flex()
                                            .flex_row()
                                            .h_full()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .h_full()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .flex_1()
                                                            .children(self.render_dock(DockPosition::Left, &self.left_dock, window, cx))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .flex_1()
                                                                    .overflow_hidden()
                                                                    .child(
                                                                        h_flex()
                                                                            .flex_1()
                                                                            .when_some(paddings.0, |this, p| this.child(p.border_r_1()))
                                                                            .child(self.center.render(
                                                                                self.zoomed.as_ref(),
                                                                                &PaneRenderContext {
                                                                                    follower_states:
                                                                                        &self.follower_states,
                                                                                    active_call: self.active_call(),
                                                                                    active_pane: &self.active_pane,
                                                                                    app_state: &self.app_state,
                                                                                    project: &self.project,
                                                                                    workspace: &self.weak_self,
                                                                                },
                                                                                window,
                                                                                cx,
                                                                            ))
                                                                            .when_some(paddings.1, |this, p| this.child(p.border_l_1())),
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .w_full()
                                                            .children(self.render_dock(DockPosition::Bottom, &self.bottom_dock, window, cx))
                                                    ),
                                            )
                                            .children(self.render_dock(
                                                DockPosition::Right,
                                                &self.right_dock,
                                                window,
                                                cx,
                                            )),

                                        BottomDockLayout::RightAligned => div()
                                            .flex()
                                            .flex_row()
                                            .h_full()
                                            .children(self.render_dock(
                                                DockPosition::Left,
                                                &self.left_dock,
                                                window,
                                                cx,
                                            ))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .h_full()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .flex_1()
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .flex_1()
                                                                    .overflow_hidden()
                                                                    .child(
                                                                        h_flex()
                                                                            .flex_1()
                                                                            .when_some(paddings.0, |this, p| this.child(p.border_r_1()))
                                                                            .child(self.center.render(
                                                                                self.zoomed.as_ref(),
                                                                                &PaneRenderContext {
                                                                                    follower_states:
                                                                                        &self.follower_states,
                                                                                    active_call: self.active_call(),
                                                                                    active_pane: &self.active_pane,
                                                                                    app_state: &self.app_state,
                                                                                    project: &self.project,
                                                                                    workspace: &self.weak_self,
                                                                                },
                                                                                window,
                                                                                cx,
                                                                            ))
                                                                            .when_some(paddings.1, |this, p| this.child(p.border_l_1())),
                                                                    )
                                                            )
                                                            .children(self.render_dock(DockPosition::Right, &self.right_dock, window, cx))
                                                    )
                                                    .child(
                                                        div()
                                                            .w_full()
                                                            .children(self.render_dock(DockPosition::Bottom, &self.bottom_dock, window, cx))
                                                    ),
                                            ),

                                        BottomDockLayout::Contained => div()
                                            .flex()
                                            .flex_row()
                                            .h_full()
                                            .children(self.render_dock(
                                                DockPosition::Left,
                                                &self.left_dock,
                                                window,
                                                cx,
                                            ))
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
                                                                self.zoomed.as_ref(),
                                                                &PaneRenderContext {
                                                                    follower_states:
                                                                        &self.follower_states,
                                                                    active_call: self.active_call(),
                                                                    active_pane: &self.active_pane,
                                                                    app_state: &self.app_state,
                                                                    project: &self.project,
                                                                    workspace: &self.weak_self,
                                                                },
                                                                window,
                                                                cx,
                                                            ))
                                                            .when_some(paddings.1, |this, p| {
                                                                this.child(p.border_l_1())
                                                            }),
                                                    )
                                                    .children(self.render_dock(
                                                        DockPosition::Bottom,
                                                        &self.bottom_dock,
                                                        window,
                                                        cx,
                                                    )),
                                            )
                                            .children(self.render_dock(
                                                DockPosition::Right,
                                                &self.right_dock,
                                                window,
                                                cx,
                                            )),
                                    }
                                })
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
                                        None => {
                                            div.top_2().bottom_2().left_2().right_2().border_1()
                                        }
                                    })
                                }))
                                .children(self.render_notifications(window, cx)),
                        )
                        .child(self.status_bar.clone())
                        .child(self.modal_layer.clone())
                        .child(self.toast_layer.clone()),
                ),
            window,
            cx,
        )
    }
}

fn resize_bottom_dock(
    new_size: Pixels,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut App,
) {
    let size =
        new_size.min(workspace.bounds.bottom() - RESIZE_HANDLE_SIZE - workspace.bounds.top());
    workspace.bottom_dock.update(cx, |bottom_dock, cx| {
        bottom_dock.resize_active_panel(Some(size), window, cx);
    });
}

fn resize_right_dock(
    new_size: Pixels,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut App,
) {
    let size = new_size.max(workspace.bounds.left() - RESIZE_HANDLE_SIZE);
    workspace.right_dock.update(cx, |right_dock, cx| {
        right_dock.resize_active_panel(Some(size), window, cx);
    });
}

fn resize_left_dock(
    new_size: Pixels,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut App,
) {
    let size = new_size.min(workspace.bounds.right() - RESIZE_HANDLE_SIZE);

    workspace.left_dock.update(cx, |left_dock, cx| {
        left_dock.resize_active_panel(Some(size), window, cx);
    });
}

impl WorkspaceStore {
    pub fn new(client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        Self {
            workspaces: Default::default(),
            _subscriptions: vec![
                client.add_request_handler(cx.weak_entity(), Self::handle_follow),
                client.add_message_handler(cx.weak_entity(), Self::handle_update_followers),
            ],
            client,
        }
    }

    pub fn update_followers(
        &self,
        project_id: Option<u64>,
        update: proto::update_followers::Variant,
        cx: &App,
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
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Follow>,
        mut cx: AsyncApp,
    ) -> Result<proto::FollowResponse> {
        this.update(&mut cx, |this, cx| {
            let follower = Follower {
                project_id: envelope.payload.project_id,
                peer_id: envelope.original_sender_id()?,
            };

            let mut response = proto::FollowResponse::default();
            this.workspaces.retain(|workspace| {
                workspace
                    .update(cx, |workspace, window, cx| {
                        let handler_response =
                            workspace.handle_follow(follower.project_id, window, cx);
                        if let Some(active_view) = handler_response.active_view.clone() {
                            if workspace.project.read(cx).remote_id() == follower.project_id {
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
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        let update = envelope.payload;

        this.update(&mut cx, |this, cx| {
            this.workspaces.retain(|workspace| {
                workspace
                    .update(cx, |workspace, window, cx| {
                        let project_id = workspace.project.read(cx).remote_id();
                        if update.project_id != project_id && update.project_id.is_some() {
                            return;
                        }
                        workspace.handle_update_followers(leader_id, update.clone(), window, cx);
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
                .map(CollaboratorId::PeerId)
                .ok_or_else(|| anyhow!("creator is missing"))?,
            id: message.id,
        })
    }

    pub(crate) fn to_proto(self) -> Option<proto::ViewId> {
        if let CollaboratorId::PeerId(peer_id) = self.creator {
            Some(proto::ViewId {
                creator: Some(peer_id),
                id: self.id,
            })
        } else {
            None
        }
    }
}

impl FollowerState {
    fn pane(&self) -> &Entity<Pane> {
        self.dock_pane.as_ref().unwrap_or(&self.center_pane)
    }
}

pub trait WorkspaceHandle {
    fn file_project_paths(&self, cx: &App) -> Vec<ProjectPath>;
}

impl WorkspaceHandle for Entity<Workspace> {
    fn file_project_paths(&self, cx: &App) -> Vec<ProjectPath> {
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

pub async fn last_opened_workspace_location() -> Option<SerializedWorkspaceLocation> {
    DB.last_workspace().await.log_err().flatten()
}

pub fn last_session_workspace_locations(
    last_session_id: &str,
    last_session_window_stack: Option<Vec<WindowId>>,
) -> Option<Vec<SerializedWorkspaceLocation>> {
    DB.last_session_workspace_locations(last_session_id, last_session_window_stack)
        .log_err()
}

actions!(
    collab,
    [
        OpenChannelNotes,
        Mute,
        Deafen,
        LeaveCall,
        ShareProject,
        ScreenShare
    ]
);
actions!(zed, [OpenLog]);

async fn join_channel_internal(
    channel_id: ChannelId,
    app_state: &Arc<AppState>,
    requesting_window: Option<WindowHandle<Workspace>>,
    active_call: &Entity<ActiveCall>,
    cx: &mut AsyncApp,
) -> Result<bool> {
    let (should_prompt, open_room) = active_call.update(cx, |active_call, cx| {
        let Some(room) = active_call.room().map(|room| room.read(cx)) else {
            return (false, None);
        };

        let already_in_channel = room.channel_id() == Some(channel_id);
        let should_prompt = room.is_sharing_project()
            && !room.remote_participants().is_empty()
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
                .update(cx, |_, window, cx| {
                    window.prompt(
                        PromptLevel::Warning,
                        "Do you want to switch channels?",
                        Some("Leaving this call will unshare your current project."),
                        &["Yes, Join Channel", "Cancel"],
                        cx,
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
                let project = workspace.update(cx, |workspace, _, cx| {
                    let project = workspace.project.read(cx);

                    if !CallSettings::get_global(cx).share_on_join {
                        return None;
                    }

                    if (project.is_local() || project.is_via_ssh())
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
                    return Some(cx.spawn(async move |room, cx| {
                        room.update(cx, |room, cx| room.share_project(project, cx))?
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
    cx: &mut App,
) -> Task<Result<()>> {
    let active_call = ActiveCall::global(cx);
    cx.spawn(async move |cx| {
        let result = join_channel_internal(
            channel_id,
            &app_state,
            requesting_window,
            &active_call,
             cx,
        )
            .await;

        // join channel succeeded, and opened a window
        if matches!(result, Ok(true)) {
            return anyhow::Ok(());
        }

        // find an existing workspace to focus and show call controls
        let mut active_window =
            requesting_window.or_else(|| activate_any_workspace_window( cx));
        if active_window.is_none() {
            // no open workspaces, make one to show the error in (blergh)
            let (window_handle, _) = cx
                .update(|cx| {
                    Workspace::new_local(vec![], app_state.clone(), requesting_window, None, cx)
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
                    .update(cx, |_, window, cx| {
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
                        window.prompt(
                            PromptLevel::Critical,
                            "Failed to join channel",
                            Some(&detail),
                            &["Ok"],
                        cx)
                    })?
                    .await
                    .ok();
            }
        }

        // return ok, we showed the error to the user.
        anyhow::Ok(())
    })
}

pub async fn get_any_active_workspace(
    app_state: Arc<AppState>,
    mut cx: AsyncApp,
) -> anyhow::Result<WindowHandle<Workspace>> {
    // find an existing workspace to focus and show call controls
    let active_window = activate_any_workspace_window(&mut cx);
    if active_window.is_none() {
        cx.update(|cx| Workspace::new_local(vec![], app_state.clone(), None, None, cx))?
            .await?;
    }
    activate_any_workspace_window(&mut cx).context("could not open zed")
}

fn activate_any_workspace_window(cx: &mut AsyncApp) -> Option<WindowHandle<Workspace>> {
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
                    .update(cx, |_, window, _| window.activate_window())
                    .ok();
                return Some(workspace_window);
            }
        }
        None
    })
    .ok()
    .flatten()
}

pub fn local_workspace_windows(cx: &App) -> Vec<WindowHandle<Workspace>> {
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
    pub visible: Option<OpenVisible>,
    pub focus: Option<bool>,
    pub open_new_workspace: Option<bool>,
    pub replace_window: Option<WindowHandle<Workspace>>,
    pub env: Option<HashMap<String, String>>,
}

#[allow(clippy::type_complexity)]
pub fn open_paths(
    abs_paths: &[PathBuf],
    app_state: Arc<AppState>,
    open_options: OpenOptions,
    cx: &mut App,
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

    cx.spawn(async move |cx| {
        if open_options.open_new_workspace != Some(true) {
            let all_paths = abs_paths.iter().map(|path| app_state.fs.metadata(path));
            let all_metadatas = futures::future::join_all(all_paths)
                .await
                .into_iter()
                .filter_map(|result| result.ok().flatten())
                .collect::<Vec<_>>();

            cx.update(|cx| {
                for window in local_workspace_windows(&cx) {
                    if let Ok(workspace) = window.read(&cx) {
                        let m = workspace.project.read(&cx).visibility_for_paths(
                            &abs_paths,
                            &all_metadatas,
                            open_options.open_new_workspace == None,
                            cx,
                        );
                        if m > best_match {
                            existing = Some(window);
                            best_match = m;
                        } else if best_match.is_none()
                            && open_options.open_new_workspace == Some(false)
                        {
                            existing = Some(window)
                        }
                    }
                }
            })?;

            if open_options.open_new_workspace.is_none() && existing.is_none() {
                if all_metadatas.iter().all(|file| !file.is_dir) {
                    cx.update(|cx| {
                        if let Some(window) = cx
                            .active_window()
                            .and_then(|window| window.downcast::<Workspace>())
                        {
                            if let Ok(workspace) = window.read(cx) {
                                let project = workspace.project().read(cx);
                                if project.is_local() && !project.is_via_collab() {
                                    existing = Some(window);
                                    open_visible = OpenVisible::None;
                                    return;
                                }
                            }
                        }
                        for window in local_workspace_windows(cx) {
                            if let Ok(workspace) = window.read(cx) {
                                let project = workspace.project().read(cx);
                                if project.is_via_collab() {
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
        }

        if let Some(existing) = existing {
            let open_task = existing
                .update(cx, |workspace, window, cx| {
                    window.activate_window();
                    workspace.open_paths(
                        abs_paths,
                        OpenOptions {
                            visible: Some(open_visible),
                            ..Default::default()
                        },
                        None,
                        window,
                        cx,
                    )
                })?
                .await;

            _ = existing.update(cx, |workspace, _, cx| {
                for item in open_task.iter().flatten() {
                    if let Err(e) = item {
                        workspace.show_error(&e, cx);
                    }
                }
            });

            Ok((existing, open_task))
        } else {
            cx.update(move |cx| {
                Workspace::new_local(
                    abs_paths,
                    app_state.clone(),
                    open_options.replace_window,
                    open_options.env,
                    cx,
                )
            })?
            .await
        }
    })
}

pub fn open_new(
    open_options: OpenOptions,
    app_state: Arc<AppState>,
    cx: &mut App,
    init: impl FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) + 'static + Send,
) -> Task<anyhow::Result<()>> {
    let task = Workspace::new_local(Vec::new(), app_state, None, open_options.env, cx);
    cx.spawn(async move |cx| {
        let (workspace, opened_paths) = task.await?;
        workspace.update(cx, |workspace, window, cx| {
            if opened_paths.is_empty() {
                init(workspace, window, cx)
            }
        })?;
        Ok(())
    })
}

pub fn create_and_open_local_file(
    path: &'static Path,
    window: &mut Window,
    cx: &mut Context<Workspace>,
    default_content: impl 'static + Send + FnOnce() -> Rope,
) -> Task<Result<Box<dyn ItemHandle>>> {
    cx.spawn_in(window, async move |workspace, cx| {
        let fs = workspace.update(cx, |workspace, _| workspace.app_state().fs.clone())?;
        if !fs.is_file(path).await {
            fs.create_file(path, Default::default()).await?;
            fs.save(path, &default_content(), Default::default())
                .await?;
        }

        let mut items = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.with_local_workspace(window, cx, |workspace, window, cx| {
                    workspace.open_paths(
                        vec![path.to_path_buf()],
                        OpenOptions {
                            visible: Some(OpenVisible::None),
                            ..Default::default()
                        },
                        None,
                        window,
                        cx,
                    )
                })
            })?
            .await?
            .await;

        let item = items.pop().flatten();
        item.ok_or_else(|| anyhow!("path {path:?} is not a file"))?
    })
}

pub fn open_ssh_project_with_new_connection(
    window: WindowHandle<Workspace>,
    connection_options: SshConnectionOptions,
    cancel_rx: oneshot::Receiver<()>,
    delegate: Arc<dyn SshClientDelegate>,
    app_state: Arc<AppState>,
    paths: Vec<PathBuf>,
    cx: &mut App,
) -> Task<Result<()>> {
    cx.spawn(async move |cx| {
        let (serialized_ssh_project, workspace_id, serialized_workspace) =
            serialize_ssh_project(connection_options.clone(), paths.clone(), &cx).await?;

        let session = match cx
            .update(|cx| {
                remote::SshRemoteClient::new(
                    ConnectionIdentifier::Workspace(workspace_id.0),
                    connection_options,
                    cancel_rx,
                    delegate,
                    cx,
                )
            })?
            .await?
        {
            Some(result) => result,
            None => return Ok(()),
        };

        let project = cx.update(|cx| {
            project::Project::ssh(
                session,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx,
            )
        })?;

        open_ssh_project_inner(
            project,
            paths,
            serialized_ssh_project,
            workspace_id,
            serialized_workspace,
            app_state,
            window,
            cx,
        )
        .await
    })
}

pub fn open_ssh_project_with_existing_connection(
    connection_options: SshConnectionOptions,
    project: Entity<Project>,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    window: WindowHandle<Workspace>,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    cx.spawn(async move |cx| {
        let (serialized_ssh_project, workspace_id, serialized_workspace) =
            serialize_ssh_project(connection_options.clone(), paths.clone(), &cx).await?;

        open_ssh_project_inner(
            project,
            paths,
            serialized_ssh_project,
            workspace_id,
            serialized_workspace,
            app_state,
            window,
            cx,
        )
        .await
    })
}

async fn open_ssh_project_inner(
    project: Entity<Project>,
    paths: Vec<PathBuf>,
    serialized_ssh_project: SerializedSshProject,
    workspace_id: WorkspaceId,
    serialized_workspace: Option<SerializedWorkspace>,
    app_state: Arc<AppState>,
    window: WindowHandle<Workspace>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let toolchains = DB.toolchains(workspace_id).await?;
    for (toolchain, worktree_id, path) in toolchains {
        project
            .update(cx, |this, cx| {
                this.activate_toolchain(ProjectPath { worktree_id, path }, toolchain, cx)
            })?
            .await;
    }
    let mut project_paths_to_open = vec![];
    let mut project_path_errors = vec![];

    for path in paths {
        let result = cx
            .update(|cx| Workspace::project_path_for_path(project.clone(), &path, true, cx))?
            .await;
        match result {
            Ok((_, project_path)) => {
                project_paths_to_open.push((path.clone(), Some(project_path)));
            }
            Err(error) => {
                project_path_errors.push(error);
            }
        };
    }

    if project_paths_to_open.is_empty() {
        return Err(project_path_errors
            .pop()
            .unwrap_or_else(|| anyhow!("no paths given")));
    }

    cx.update_window(window.into(), |_, window, cx| {
        window.replace_root(cx, |window, cx| {
            telemetry::event!("SSH Project Opened");

            let mut workspace =
                Workspace::new(Some(workspace_id), project, app_state.clone(), window, cx);
            workspace.set_serialized_ssh_project(serialized_ssh_project);
            workspace.update_history(cx);
            workspace
        });
    })?;

    window
        .update(cx, |_, window, cx| {
            window.activate_window();
            open_items(serialized_workspace, project_paths_to_open, window, cx)
        })?
        .await?;

    window.update(cx, |workspace, _, cx| {
        for error in project_path_errors {
            if error.error_code() == proto::ErrorCode::DevServerProjectPathDoesNotExist {
                if let Some(path) = error.error_tag("path") {
                    workspace.show_error(&anyhow!("'{path}' does not exist"), cx)
                }
            } else {
                workspace.show_error(&error, cx)
            }
        }
    })?;

    Ok(())
}

fn serialize_ssh_project(
    connection_options: SshConnectionOptions,
    paths: Vec<PathBuf>,
    cx: &AsyncApp,
) -> Task<
    Result<(
        SerializedSshProject,
        WorkspaceId,
        Option<SerializedWorkspace>,
    )>,
> {
    cx.background_spawn(async move {
        let serialized_ssh_project = persistence::DB
            .get_or_create_ssh_project(
                connection_options.host.clone(),
                connection_options.port,
                paths
                    .iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect::<Vec<_>>(),
                connection_options.username.clone(),
            )
            .await?;

        let serialized_workspace =
            persistence::DB.workspace_for_ssh_project(&serialized_ssh_project);

        let workspace_id = if let Some(workspace_id) =
            serialized_workspace.as_ref().map(|workspace| workspace.id)
        {
            workspace_id
        } else {
            persistence::DB.next_id().await?
        };

        Ok((serialized_ssh_project, workspace_id, serialized_workspace))
    })
}

pub fn join_in_room_project(
    project_id: u64,
    follow_user_id: u64,
    app_state: Arc<AppState>,
    cx: &mut App,
) -> Task<Result<()>> {
    let windows = cx.windows();
    cx.spawn(async move |cx| {
        let existing_workspace = windows.into_iter().find_map(|window_handle| {
            window_handle
                .downcast::<Workspace>()
                .and_then(|window_handle| {
                    window_handle
                        .update(cx, |workspace, _window, cx| {
                            if workspace.project().read(cx).remote_id() == Some(project_id) {
                                Some(window_handle)
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
                .read_with(cx, |call, _| call.room().cloned())?
                .ok_or_else(|| anyhow!("not in a call"))?;
            let project = room
                .update(cx, |room, cx| {
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
                options.window_bounds = window_bounds_override.map(WindowBounds::Windowed);
                cx.open_window(options, |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(Default::default(), project, app_state.clone(), window, cx)
                    })
                })
            })??
        };

        workspace.update(cx, |workspace, window, cx| {
            cx.activate(true);
            window.activate_window();

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
                            .find(|collaborator| collaborator.is_host)?;
                        Some(collaborator.peer_id)
                    });

                if let Some(follow_peer_id) = follow_peer_id {
                    workspace.follow(follow_peer_id, window, cx);
                }
            }
        })?;

        anyhow::Ok(())
    })
}

pub fn reload(reload: &Reload, cx: &mut App) {
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
            .update(cx, |_, window, cx| {
                window.prompt(
                    PromptLevel::Info,
                    "Are you sure you want to restart?",
                    None,
                    &["Restart", "Cancel"],
                    cx,
                )
            })
            .ok();
    }

    let binary_path = reload.binary_path.clone();
    cx.spawn(async move |cx| {
        if let Some(prompt) = prompt {
            let answer = prompt.await?;
            if answer != 0 {
                return Ok(());
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for window in workspace_windows {
            if let Ok(should_close) = window.update(cx, |workspace, window, cx| {
                workspace.prepare_to_close(CloseIntent::Quit, window, cx)
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

fn parse_pixel_position_env_var(value: &str) -> Option<Point<Pixels>> {
    let mut parts = value.split(',');
    let x: usize = parts.next()?.parse().ok()?;
    let y: usize = parts.next()?.parse().ok()?;
    Some(point(px(x as f32), px(y as f32)))
}

fn parse_pixel_size_env_var(value: &str) -> Option<Size<Pixels>> {
    let mut parts = value.split(',');
    let width: usize = parts.next()?.parse().ok()?;
    let height: usize = parts.next()?.parse().ok()?;
    Some(size(px(width as f32), px(height as f32)))
}

pub fn client_side_decorations(
    element: impl IntoElement,
    window: &mut Window,
    cx: &mut App,
) -> Stateful<Div> {
    const BORDER_SIZE: Pixels = px(1.0);
    let decorations = window.window_decorations();

    if matches!(decorations, Decorations::Client { .. }) {
        window.set_client_inset(theme::CLIENT_SIDE_DECORATION_SHADOW);
    }

    struct GlobalResizeEdge(ResizeEdge);
    impl Global for GlobalResizeEdge {}

    div()
        .id("window-backdrop")
        .bg(transparent_black())
        .map(|div| match decorations {
            Decorations::Server => div,
            Decorations::Client { tiling, .. } => div
                .when(!(tiling.top || tiling.right), |div| {
                    div.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.top || tiling.left), |div| {
                    div.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.bottom || tiling.right), |div| {
                    div.rounded_br(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.bottom || tiling.left), |div| {
                    div.rounded_bl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!tiling.top, |div| {
                    div.pt(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.bottom, |div| {
                    div.pb(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.left, |div| {
                    div.pl(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.right, |div| {
                    div.pr(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .on_mouse_move(move |e, window, cx| {
                    let size = window.window_bounds().get_bounds().size;
                    let pos = e.position;

                    let new_edge =
                        resize_edge(pos, theme::CLIENT_SIDE_DECORATION_SHADOW, size, tiling);

                    let edge = cx.try_global::<GlobalResizeEdge>();
                    if new_edge != edge.map(|edge| edge.0) {
                        window
                            .window_handle()
                            .update(cx, |workspace, _, cx| {
                                cx.notify(workspace.entity_id());
                            })
                            .ok();
                    }
                })
                .on_mouse_down(MouseButton::Left, move |e, window, _| {
                    let size = window.window_bounds().get_bounds().size;
                    let pos = e.position;

                    let edge = match resize_edge(
                        pos,
                        theme::CLIENT_SIDE_DECORATION_SHADOW,
                        size,
                        tiling,
                    ) {
                        Some(value) => value,
                        None => return,
                    };

                    window.start_window_resize(edge);
                }),
        })
        .size_full()
        .child(
            div()
                .cursor(CursorStyle::Arrow)
                .map(|div| match decorations {
                    Decorations::Server => div,
                    Decorations::Client { tiling } => div
                        .border_color(cx.theme().colors().border)
                        .when(!(tiling.top || tiling.right), |div| {
                            div.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.top || tiling.left), |div| {
                            div.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.bottom || tiling.right), |div| {
                            div.rounded_br(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.bottom || tiling.left), |div| {
                            div.rounded_bl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!tiling.top, |div| div.border_t(BORDER_SIZE))
                        .when(!tiling.bottom, |div| div.border_b(BORDER_SIZE))
                        .when(!tiling.left, |div| div.border_l(BORDER_SIZE))
                        .when(!tiling.right, |div| div.border_r(BORDER_SIZE))
                        .when(!tiling.is_tiled(), |div| {
                            div.shadow(smallvec::smallvec![gpui::BoxShadow {
                                color: Hsla {
                                    h: 0.,
                                    s: 0.,
                                    l: 0.,
                                    a: 0.4,
                                },
                                blur_radius: theme::CLIENT_SIDE_DECORATION_SHADOW / 2.,
                                spread_radius: px(0.),
                                offset: point(px(0.0), px(0.0)),
                            }])
                        }),
                })
                .on_mouse_move(|_e, _, cx| {
                    cx.stop_propagation();
                })
                .size_full()
                .child(element),
        )
        .map(|div| match decorations {
            Decorations::Server => div,
            Decorations::Client { tiling, .. } => div.child(
                canvas(
                    |_bounds, window, _| {
                        window.insert_hitbox(
                            Bounds::new(
                                point(px(0.0), px(0.0)),
                                window.window_bounds().get_bounds().size,
                            ),
                            false,
                        )
                    },
                    move |_bounds, hitbox, window, cx| {
                        let mouse = window.mouse_position();
                        let size = window.window_bounds().get_bounds().size;
                        let Some(edge) =
                            resize_edge(mouse, theme::CLIENT_SIDE_DECORATION_SHADOW, size, tiling)
                        else {
                            return;
                        };
                        cx.set_global(GlobalResizeEdge(edge));
                        window.set_cursor_style(
                            match edge {
                                ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
                                ResizeEdge::Left | ResizeEdge::Right => {
                                    CursorStyle::ResizeLeftRight
                                }
                                ResizeEdge::TopLeft | ResizeEdge::BottomRight => {
                                    CursorStyle::ResizeUpLeftDownRight
                                }
                                ResizeEdge::TopRight | ResizeEdge::BottomLeft => {
                                    CursorStyle::ResizeUpRightDownLeft
                                }
                            },
                            Some(&hitbox),
                        );
                    },
                )
                .size_full()
                .absolute(),
            ),
        })
}

fn resize_edge(
    pos: Point<Pixels>,
    shadow_size: Pixels,
    window_size: Size<Pixels>,
    tiling: Tiling,
) -> Option<ResizeEdge> {
    let bounds = Bounds::new(Point::default(), window_size).inset(shadow_size * 1.5);
    if bounds.contains(&pos) {
        return None;
    }

    let corner_size = size(shadow_size * 1.5, shadow_size * 1.5);
    let top_left_bounds = Bounds::new(Point::new(px(0.), px(0.)), corner_size);
    if !tiling.top && top_left_bounds.contains(&pos) {
        return Some(ResizeEdge::TopLeft);
    }

    let top_right_bounds = Bounds::new(
        Point::new(window_size.width - corner_size.width, px(0.)),
        corner_size,
    );
    if !tiling.top && top_right_bounds.contains(&pos) {
        return Some(ResizeEdge::TopRight);
    }

    let bottom_left_bounds = Bounds::new(
        Point::new(px(0.), window_size.height - corner_size.height),
        corner_size,
    );
    if !tiling.bottom && bottom_left_bounds.contains(&pos) {
        return Some(ResizeEdge::BottomLeft);
    }

    let bottom_right_bounds = Bounds::new(
        Point::new(
            window_size.width - corner_size.width,
            window_size.height - corner_size.height,
        ),
        corner_size,
    );
    if !tiling.bottom && bottom_right_bounds.contains(&pos) {
        return Some(ResizeEdge::BottomRight);
    }

    if !tiling.top && pos.y < shadow_size {
        Some(ResizeEdge::Top)
    } else if !tiling.bottom && pos.y > window_size.height - shadow_size {
        Some(ResizeEdge::Bottom)
    } else if !tiling.left && pos.x < shadow_size {
        Some(ResizeEdge::Left)
    } else if !tiling.right && pos.x > window_size.width - shadow_size {
        Some(ResizeEdge::Right)
    } else {
        None
    }
}

fn join_pane_into_active(
    active_pane: &Entity<Pane>,
    pane: &Entity<Pane>,
    window: &mut Window,
    cx: &mut App,
) {
    if pane == active_pane {
        return;
    } else if pane.read(cx).items_len() == 0 {
        pane.update(cx, |_, cx| {
            cx.emit(pane::Event::Remove {
                focus_on_pane: None,
            });
        })
    } else {
        move_all_items(pane, active_pane, window, cx);
    }
}

fn move_all_items(
    from_pane: &Entity<Pane>,
    to_pane: &Entity<Pane>,
    window: &mut Window,
    cx: &mut App,
) {
    let destination_is_different = from_pane != to_pane;
    let mut moved_items = 0;
    for (item_ix, item_handle) in from_pane
        .read(cx)
        .items()
        .enumerate()
        .map(|(ix, item)| (ix, item.clone()))
        .collect::<Vec<_>>()
    {
        let ix = item_ix - moved_items;
        if destination_is_different {
            // Close item from previous pane
            from_pane.update(cx, |source, cx| {
                source.remove_item_and_focus_on_pane(ix, false, to_pane.clone(), window, cx);
            });
            moved_items += 1;
        }

        // This automatically removes duplicate items in the pane
        to_pane.update(cx, |destination, cx| {
            destination.add_item(item_handle, true, true, None, window, cx);
            window.focus(&destination.focus_handle(cx))
        });
    }
}

pub fn move_item(
    source: &Entity<Pane>,
    destination: &Entity<Pane>,
    item_id_to_move: EntityId,
    destination_index: usize,
    window: &mut Window,
    cx: &mut App,
) {
    let Some((item_ix, item_handle)) = source
        .read(cx)
        .items()
        .enumerate()
        .find(|(_, item_handle)| item_handle.item_id() == item_id_to_move)
        .map(|(ix, item)| (ix, item.clone()))
    else {
        // Tab was closed during drag
        return;
    };

    if source != destination {
        // Close item from previous pane
        source.update(cx, |source, cx| {
            source.remove_item_and_focus_on_pane(item_ix, false, destination.clone(), window, cx);
        });
    }

    // This automatically removes duplicate items in the pane
    destination.update(cx, |destination, cx| {
        destination.add_item(item_handle, true, true, Some(destination_index), window, cx);
        window.focus(&destination.focus_handle(cx))
    });
}

pub fn move_active_item(
    source: &Entity<Pane>,
    destination: &Entity<Pane>,
    focus_destination: bool,
    close_if_empty: bool,
    window: &mut Window,
    cx: &mut App,
) {
    if source == destination {
        return;
    }
    let Some(active_item) = source.read(cx).active_item() else {
        return;
    };
    source.update(cx, |source_pane, cx| {
        let item_id = active_item.item_id();
        source_pane.remove_item(item_id, false, close_if_empty, window, cx);
        destination.update(cx, |target_pane, cx| {
            target_pane.add_item(
                active_item,
                focus_destination,
                focus_destination,
                Some(target_pane.items_len()),
                window,
                cx,
            );
        });
    });
}

#[derive(Debug)]
pub struct WorkspacePosition {
    pub window_bounds: Option<WindowBounds>,
    pub display: Option<Uuid>,
    pub centered_layout: bool,
}

pub fn ssh_workspace_position_from_db(
    host: String,
    port: Option<u16>,
    user: Option<String>,
    paths_to_open: &[PathBuf],
    cx: &App,
) -> Task<Result<WorkspacePosition>> {
    let paths = paths_to_open
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    cx.background_spawn(async move {
        let serialized_ssh_project = persistence::DB
            .get_or_create_ssh_project(host, port, paths, user)
            .await
            .context("fetching serialized ssh project")?;
        let serialized_workspace =
            persistence::DB.workspace_for_ssh_project(&serialized_ssh_project);

        let (window_bounds, display) = if let Some(bounds) = window_bounds_env_override() {
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

        let centered_layout = serialized_workspace
            .as_ref()
            .map(|w| w.centered_layout)
            .unwrap_or(false);

        Ok(WorkspacePosition {
            window_bounds,
            display,
            centered_layout,
        })
    })
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::{
        dock::{PanelEvent, test::TestPanel},
        item::{
            ItemEvent,
            test::{TestItem, TestProjectItem},
        },
    };
    use fs::FakeFs;
    use gpui::{
        DismissEvent, Empty, EventEmitter, FocusHandle, Focusable, Render, TestAppContext,
        UpdateGlobal, VisualTestContext, px,
    };
    use project::{Project, ProjectEntryId};
    use serde_json::json;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_tab_disambiguation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Adding an item with no ambiguity renders the tab without detail.
        let item1 = cx.new(|cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b1/c", "a/b1/c"]);
            item
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item1.clone()), None, true, window, cx);
        });
        item1.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(0)));

        // Adding an item that creates ambiguity increases the level of detail on
        // both tabs.
        let item2 = cx.new_window_entity(|_window, cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item2.clone()), None, true, window, cx);
        });
        item1.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));
        item2.update(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));

        // Adding an item that creates ambiguity increases the level of detail only
        // on the ambiguous tabs. In this case, the ambiguity can't be resolved so
        // we stop at the highest detail available.
        let item3 = cx.new(|cx| {
            let mut item = TestItem::new(cx);
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item3.clone()), None, true, window, cx);
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let item1 = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
        });
        let item2 = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(2, "two.txt", cx)])
        });

        // Add an item to an empty pane
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item1), None, true, window, cx)
        });
        project.update(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "one.txt").into(), cx)
                    .map(|e| e.id)
            );
        });
        assert_eq!(cx.window_title().as_deref(), Some("root1 — one.txt"));

        // Add a second item to a non-empty pane
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item2), None, true, window, cx)
        });
        assert_eq!(cx.window_title().as_deref(), Some("root1 — two.txt"));
        project.update(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "two.txt").into(), cx)
                    .map(|e| e.id)
            );
        });

        // Close the active item
        pane.update_in(cx, |pane, window, cx| {
            pane.close_active_item(&Default::default(), window, cx)
                .unwrap()
        })
        .await
        .unwrap();
        assert_eq!(cx.window_title().as_deref(), Some("root1 — one.txt"));
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
                project.find_or_create_worktree("root2", true, cx)
            })
            .await
            .unwrap();
        assert_eq!(cx.window_title().as_deref(), Some("root1, root2 — one.txt"));

        // Remove a project folder
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        assert_eq!(cx.window_title().as_deref(), Some("root2 — one.txt"));
    }

    #[gpui::test]
    async fn test_close_window(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "one": "" })).await;

        let project = Project::test(fs, ["root".as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // When there are no dirty items, there's nothing to do.
        let item1 = cx.new(TestItem::new);
        workspace.update_in(cx, |w, window, cx| {
            w.add_item_to_active_pane(Box::new(item1.clone()), None, true, window, cx)
        });
        let task = workspace.update_in(cx, |w, window, cx| {
            w.prepare_to_close(CloseIntent::CloseWindow, window, cx)
        });
        assert!(task.await.unwrap());

        // When there are dirty untitled items, prompt to save each one. If the user
        // cancels any prompt, then abort.
        let item2 = cx.new(|cx| TestItem::new(cx).with_dirty(true));
        let item3 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        workspace.update_in(cx, |w, window, cx| {
            w.add_item_to_active_pane(Box::new(item2.clone()), None, true, window, cx);
            w.add_item_to_active_pane(Box::new(item3.clone()), None, true, window, cx);
        });
        let task = workspace.update_in(cx, |w, window, cx| {
            w.prepare_to_close(CloseIntent::CloseWindow, window, cx)
        });
        cx.executor().run_until_parked();
        cx.simulate_prompt_answer("Cancel"); // cancel save all
        cx.executor().run_until_parked();
        assert!(!cx.has_pending_prompt());
        assert!(!task.await.unwrap());
    }

    #[gpui::test]
    async fn test_close_window_with_serializable_items(cx: &mut TestAppContext) {
        init_test(cx);

        // Register TestItem as a serializable item
        cx.update(|cx| {
            register_serializable_item::<TestItem>(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "one": "" })).await;

        let project = Project::test(fs, ["root".as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // When there are dirty untitled items, but they can serialize, then there is no prompt.
        let item1 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_serialize(|| Some(Task::ready(Ok(()))))
        });
        let item2 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                .with_serialize(|| Some(Task::ready(Ok(()))))
        });
        workspace.update_in(cx, |w, window, cx| {
            w.add_item_to_active_pane(Box::new(item1.clone()), None, true, window, cx);
            w.add_item_to_active_pane(Box::new(item2.clone()), None, true, window, cx);
        });
        let task = workspace.update_in(cx, |w, window, cx| {
            w.prepare_to_close(CloseIntent::CloseWindow, window, cx)
        });
        assert!(task.await.unwrap());
    }

    #[gpui::test]
    async fn test_close_pane_items(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        let item1 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[dirty_project_item(1, "1.txt", cx)])
        });
        let item2 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[dirty_project_item(2, "2.txt", cx)])
        });
        let item3 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[dirty_project_item(3, "3.txt", cx)])
        });
        let item4 = cx.new(|cx| {
            TestItem::new(cx).with_dirty(true).with_project_items(&[{
                let project_item = TestProjectItem::new_untitled(cx);
                project_item.update(cx, |project_item, _| project_item.is_dirty = true);
                project_item
            }])
        });
        let pane = workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item1.clone()), None, true, window, cx);
            workspace.add_item_to_active_pane(Box::new(item2.clone()), None, true, window, cx);
            workspace.add_item_to_active_pane(Box::new(item3.clone()), None, true, window, cx);
            workspace.add_item_to_active_pane(Box::new(item4.clone()), None, true, window, cx);
            workspace.active_pane().clone()
        });

        let close_items = pane.update_in(cx, |pane, window, cx| {
            pane.activate_item(1, true, true, window, cx);
            assert_eq!(pane.active_item().unwrap().item_id(), item2.item_id());
            let item1_id = item1.item_id();
            let item3_id = item3.item_id();
            let item4_id = item4.item_id();
            pane.close_items(window, cx, SaveIntent::Close, move |id| {
                [item1_id, item3_id, item4_id].contains(&id)
            })
        });
        cx.executor().run_until_parked();

        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Save all");

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
        cx.simulate_prompt_answer("Discard");
        cx.executor().run_until_parked();

        // Item 3 is reloaded. There's a prompt to save item 4.
        pane.update(cx, |pane, cx| {
            assert_eq!(item3.read(cx).save_count, 0);
            assert_eq!(item3.read(cx).save_as_count, 0);
            assert_eq!(item3.read(cx).reload_count, 1);
            assert_eq!(pane.items_len(), 2);
            assert_eq!(pane.active_item().unwrap().item_id(), item4.item_id());
        });

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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        // Create several workspace items with single project entries, and two
        // workspace items with multiple project entries.
        let single_entry_items = (0..=4)
            .map(|project_entry_id| {
                cx.new(|cx| {
                    TestItem::new(cx)
                        .with_dirty(true)
                        .with_project_items(&[dirty_project_item(
                            project_entry_id,
                            &format!("{project_entry_id}.txt"),
                            cx,
                        )])
                })
            })
            .collect::<Vec<_>>();
        let item_2_3 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_project_items(&[
                    single_entry_items[2].read(cx).project_items[0].clone(),
                    single_entry_items[3].read(cx).project_items[0].clone(),
                ])
        });
        let item_3_4 = cx.new(|cx| {
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
        //     single-entry items:  0, 2, 3, 4
        //   right pane:
        //     single-entry items:  4, 1
        //     multi-entry items:   (3, 4)
        let (left_pane, right_pane) = workspace.update_in(cx, |workspace, window, cx| {
            let left_pane = workspace.active_pane().clone();
            workspace.add_item_to_active_pane(Box::new(item_2_3.clone()), None, true, window, cx);
            workspace.add_item_to_active_pane(
                single_entry_items[0].boxed_clone(),
                None,
                true,
                window,
                cx,
            );
            workspace.add_item_to_active_pane(
                single_entry_items[2].boxed_clone(),
                None,
                true,
                window,
                cx,
            );
            workspace.add_item_to_active_pane(
                single_entry_items[3].boxed_clone(),
                None,
                true,
                window,
                cx,
            );
            workspace.add_item_to_active_pane(
                single_entry_items[4].boxed_clone(),
                None,
                true,
                window,
                cx,
            );

            let right_pane = workspace
                .split_and_clone(left_pane.clone(), SplitDirection::Right, window, cx)
                .unwrap();

            right_pane.update(cx, |pane, cx| {
                pane.add_item(
                    single_entry_items[1].boxed_clone(),
                    true,
                    true,
                    None,
                    window,
                    cx,
                );
                pane.add_item(Box::new(item_3_4.clone()), true, true, None, window, cx);
            });

            (left_pane, right_pane)
        });

        cx.focus(&right_pane);

        let mut close = right_pane.update_in(cx, |pane, window, cx| {
            pane.close_all_items(&CloseAllItems::default(), window, cx)
                .unwrap()
        });
        cx.executor().run_until_parked();

        let msg = cx.pending_prompt().unwrap().0;
        assert!(msg.contains("1.txt"));
        assert!(!msg.contains("2.txt"));
        assert!(!msg.contains("3.txt"));
        assert!(!msg.contains("4.txt"));

        cx.simulate_prompt_answer("Cancel");
        close.await.unwrap();

        left_pane
            .update_in(cx, |left_pane, window, cx| {
                left_pane.close_item_by_id(
                    single_entry_items[3].entity_id(),
                    SaveIntent::Skip,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        close = right_pane.update_in(cx, |pane, window, cx| {
            pane.close_all_items(&CloseAllItems::default(), window, cx)
                .unwrap()
        });
        cx.executor().run_until_parked();

        let details = cx.pending_prompt().unwrap().1;
        assert!(details.contains("1.txt"));
        assert!(!details.contains("2.txt"));
        assert!(details.contains("3.txt"));
        // ideally this assertion could be made, but today we can only
        // save whole items not project items, so the orphaned item 3 causes
        // 4 to be saved too.
        // assert!(!details.contains("4.txt"));

        cx.simulate_prompt_answer("Save all");

        cx.executor().run_until_parked();
        close.await.unwrap();
        right_pane.update(cx, |pane, _| {
            assert_eq!(pane.items_len(), 0);
        });
    }

    #[gpui::test]
    async fn test_autosave(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        let item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item_id = item.entity_id();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
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
        cx.update(|window, _| window.activate_window());
        cx.executor().run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 1));

        // Autosave on focus change.
        item.update_in(cx, |item, window, cx| {
            cx.focus_self(window);
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnFocusChange);
                })
            });
            item.is_dirty = true;
        });

        // Blurring the item saves the file.
        item.update_in(cx, |_, window, _| window.blur());
        cx.executor().run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 2));

        // Deactivating the window still saves the file.
        item.update_in(cx, |item, window, cx| {
            cx.focus_self(window);
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
            for project_item in &mut item.project_items {
                project_item.update(cx, |project_item, _| project_item.is_dirty = true);
            }
        });

        pane.update_in(cx, |pane, window, cx| {
            pane.close_items(window, cx, SaveIntent::Close, move |id| id == item_id)
        })
        .await
        .unwrap();
        assert!(!cx.has_pending_prompt());
        item.update(cx, |item, _| assert_eq!(item.save_count, 5));

        // Add the item again, ensuring autosave is prevented if the underlying file has been deleted.
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
        });
        item.update_in(cx, |item, window, cx| {
            item.project_items[0].update(cx, |item, _| {
                item.entry_id = None;
            });
            item.is_dirty = true;
            window.blur();
        });
        cx.run_until_parked();
        item.update(cx, |item, _| assert_eq!(item.save_count, 5));

        // Ensure autosave is prevented for deleted files also when closing the buffer.
        let _close_items = pane.update_in(cx, |pane, window, cx| {
            pane.close_items(window, cx, SaveIntent::Close, move |id| id == item_id)
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        let item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        let toolbar = pane.update(cx, |pane, _| pane.toolbar().clone());
        let toolbar_notify_count = Rc::new(RefCell::new(0));

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
            let toolbar_notification_count = toolbar_notify_count.clone();
            cx.observe_in(&toolbar, window, move |_, _, _, _| {
                *toolbar_notification_count.borrow_mut() += 1
            })
            .detach();
        });

        pane.update(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        item.update_in(cx, |item, _, cx| {
            item.set_state("one".to_string(), cx);
        });

        // Toolbar must be notified to re-render the navigation buttons
        assert_eq!(*toolbar_notify_count.borrow(), 1);

        pane.update(cx, |pane, _| {
            assert!(pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.go_back(pane.downgrade(), window, cx)
            })
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TestPanel::new(DockPosition::Right, cx));
            workspace.add_panel(panel.clone(), window, cx);

            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, window, cx));

            panel
        });

        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
        pane.update_in(cx, |pane, window, cx| {
            let item = cx.new(TestItem::new);
            pane.add_item(Box::new(item), true, true, None, window, cx);
        });

        // Transfer focus from center to panel
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(window, cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Transfer focus from panel to center
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(window, cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Close the dock
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(window, cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Open the dock
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(window, cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Focus and zoom panel
        panel.update_in(cx, |panel, window, cx| {
            cx.focus_self(window);
            panel.set_zoomed(true, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(window, cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Transfer focus to the center closes the dock
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(window, cx));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Transferring focus back to the panel keeps it zoomed
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(window, cx));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Close the dock while it is zoomed
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(window, cx));
            assert!(workspace.zoomed.is_none());
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Opening the dock, when it's zoomed, retains focus
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(window, cx));
            assert!(workspace.zoomed.is_some());
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        // Unzoom and close the panel, zoom the active pane.
        panel.update_in(cx, |panel, window, cx| panel.set_zoomed(false, window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx)
        });
        pane.update_in(cx, |pane, window, cx| {
            pane.toggle_zoom(&Default::default(), window, cx)
        });

        // Opening a dock unzooms the pane.
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx)
        });
        workspace.update_in(cx, |workspace, window, cx| {
            let pane = pane.read(cx);
            assert!(!pane.is_zoomed());
            assert!(!pane.focus_handle(cx).is_focused(window));
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(workspace.zoomed.is_none());
        });
    }

    #[gpui::test]
    async fn test_join_pane_into_next(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        // Let's arrange the panes like this:
        //
        // +-----------------------+
        // |         top           |
        // +------+--------+-------+
        // | left | center | right |
        // +------+--------+-------+
        // |        bottom         |
        // +-----------------------+

        let top_item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(1, "top.txt", cx)])
        });
        let bottom_item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(2, "bottom.txt", cx)])
        });
        let left_item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(3, "left.txt", cx)])
        });
        let right_item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(4, "right.txt", cx)])
        });
        let center_item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(5, "center.txt", cx)])
        });

        let top_pane_id = workspace.update_in(cx, |workspace, window, cx| {
            let top_pane_id = workspace.active_pane().entity_id();
            workspace.add_item_to_active_pane(Box::new(top_item.clone()), None, false, window, cx);
            workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Down,
                window,
                cx,
            );
            top_pane_id
        });
        let bottom_pane_id = workspace.update_in(cx, |workspace, window, cx| {
            let bottom_pane_id = workspace.active_pane().entity_id();
            workspace.add_item_to_active_pane(
                Box::new(bottom_item.clone()),
                None,
                false,
                window,
                cx,
            );
            workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Up,
                window,
                cx,
            );
            bottom_pane_id
        });
        let left_pane_id = workspace.update_in(cx, |workspace, window, cx| {
            let left_pane_id = workspace.active_pane().entity_id();
            workspace.add_item_to_active_pane(Box::new(left_item.clone()), None, false, window, cx);
            workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Right,
                window,
                cx,
            );
            left_pane_id
        });
        let right_pane_id = workspace.update_in(cx, |workspace, window, cx| {
            let right_pane_id = workspace.active_pane().entity_id();
            workspace.add_item_to_active_pane(
                Box::new(right_item.clone()),
                None,
                false,
                window,
                cx,
            );
            workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Left,
                window,
                cx,
            );
            right_pane_id
        });
        let center_pane_id = workspace.update_in(cx, |workspace, window, cx| {
            let center_pane_id = workspace.active_pane().entity_id();
            workspace.add_item_to_active_pane(
                Box::new(center_item.clone()),
                None,
                false,
                window,
                cx,
            );
            center_pane_id
        });
        cx.executor().run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            assert_eq!(center_pane_id, workspace.active_pane().entity_id());

            // Join into next from center pane into right
            workspace.join_pane_into_next(workspace.active_pane().clone(), window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let active_pane = workspace.active_pane();
            assert_eq!(right_pane_id, active_pane.entity_id());
            assert_eq!(2, active_pane.read(cx).items_len());
            let item_ids_in_pane =
                HashSet::from_iter(active_pane.read(cx).items().map(|item| item.item_id()));
            assert!(item_ids_in_pane.contains(&center_item.item_id()));
            assert!(item_ids_in_pane.contains(&right_item.item_id()));

            // Join into next from right pane into bottom
            workspace.join_pane_into_next(workspace.active_pane().clone(), window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let active_pane = workspace.active_pane();
            assert_eq!(bottom_pane_id, active_pane.entity_id());
            assert_eq!(3, active_pane.read(cx).items_len());
            let item_ids_in_pane =
                HashSet::from_iter(active_pane.read(cx).items().map(|item| item.item_id()));
            assert!(item_ids_in_pane.contains(&center_item.item_id()));
            assert!(item_ids_in_pane.contains(&right_item.item_id()));
            assert!(item_ids_in_pane.contains(&bottom_item.item_id()));

            // Join into next from bottom pane into left
            workspace.join_pane_into_next(workspace.active_pane().clone(), window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let active_pane = workspace.active_pane();
            assert_eq!(left_pane_id, active_pane.entity_id());
            assert_eq!(4, active_pane.read(cx).items_len());
            let item_ids_in_pane =
                HashSet::from_iter(active_pane.read(cx).items().map(|item| item.item_id()));
            assert!(item_ids_in_pane.contains(&center_item.item_id()));
            assert!(item_ids_in_pane.contains(&right_item.item_id()));
            assert!(item_ids_in_pane.contains(&bottom_item.item_id()));
            assert!(item_ids_in_pane.contains(&left_item.item_id()));

            // Join into next from left pane into top
            workspace.join_pane_into_next(workspace.active_pane().clone(), window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let active_pane = workspace.active_pane();
            assert_eq!(top_pane_id, active_pane.entity_id());
            assert_eq!(5, active_pane.read(cx).items_len());
            let item_ids_in_pane =
                HashSet::from_iter(active_pane.read(cx).items().map(|item| item.item_id()));
            assert!(item_ids_in_pane.contains(&center_item.item_id()));
            assert!(item_ids_in_pane.contains(&right_item.item_id()));
            assert!(item_ids_in_pane.contains(&bottom_item.item_id()));
            assert!(item_ids_in_pane.contains(&left_item.item_id()));
            assert!(item_ids_in_pane.contains(&top_item.item_id()));

            // Single pane left: no-op
            workspace.join_pane_into_next(workspace.active_pane().clone(), window, cx)
        });

        workspace.update(cx, |workspace, _cx| {
            let active_pane = workspace.active_pane();
            assert_eq!(top_pane_id, active_pane.entity_id());
        });
    }

    fn add_an_item_to_active_pane(
        cx: &mut VisualTestContext,
        workspace: &Entity<Workspace>,
        item_id: u64,
    ) -> Entity<TestItem> {
        let item = cx.new(|cx| {
            TestItem::new(cx).with_project_items(&[TestProjectItem::new(
                item_id,
                "item{item_id}.txt",
                cx,
            )])
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, false, window, cx);
        });
        return item;
    }

    fn split_pane(cx: &mut VisualTestContext, workspace: &Entity<Workspace>) -> Entity<Pane> {
        return workspace.update_in(cx, |workspace, window, cx| {
            let new_pane = workspace.split_pane(
                workspace.active_pane().clone(),
                SplitDirection::Right,
                window,
                cx,
            );
            new_pane
        });
    }

    #[gpui::test]
    async fn test_join_all_panes(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        add_an_item_to_active_pane(cx, &workspace, 1);
        split_pane(cx, &workspace);
        add_an_item_to_active_pane(cx, &workspace, 2);
        split_pane(cx, &workspace); // empty pane
        split_pane(cx, &workspace);
        let last_item = add_an_item_to_active_pane(cx, &workspace, 3);

        cx.executor().run_until_parked();

        workspace.update(cx, |workspace, cx| {
            let num_panes = workspace.panes().len();
            let num_items_in_current_pane = workspace.active_pane().read(cx).items().count();
            let active_item = workspace
                .active_pane()
                .read(cx)
                .active_item()
                .expect("item is in focus");

            assert_eq!(num_panes, 4);
            assert_eq!(num_items_in_current_pane, 1);
            assert_eq!(active_item.item_id(), last_item.item_id());
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.join_all_panes(window, cx);
        });

        workspace.update(cx, |workspace, cx| {
            let num_panes = workspace.panes().len();
            let num_items_in_current_pane = workspace.active_pane().read(cx).items().count();
            let active_item = workspace
                .active_pane()
                .read(cx)
                .active_item()
                .expect("item is in focus");

            assert_eq!(num_panes, 1);
            assert_eq!(num_items_in_current_pane, 3);
            assert_eq!(active_item.item_id(), last_item.item_id());
        });
    }
    struct TestModal(FocusHandle);

    impl TestModal {
        fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
            Self(cx.focus_handle())
        }
    }

    impl EventEmitter<DismissEvent> for TestModal {}

    impl Focusable for TestModal {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.0.clone()
        }
    }

    impl ModalView for TestModal {}

    impl Render for TestModal {
        fn render(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<TestModal>,
        ) -> impl IntoElement {
            div().track_focus(&self.0)
        }
    }

    #[gpui::test]
    async fn test_panels(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        let (panel_1, panel_2) = workspace.update_in(cx, |workspace, window, cx| {
            let panel_1 = cx.new(|cx| TestPanel::new(DockPosition::Left, cx));
            workspace.add_panel(panel_1.clone(), window, cx);
            workspace.toggle_dock(DockPosition::Left, window, cx);
            let panel_2 = cx.new(|cx| TestPanel::new(DockPosition::Right, cx));
            workspace.add_panel(panel_2.clone(), window, cx);
            workspace.toggle_dock(DockPosition::Right, window, cx);

            let left_dock = workspace.left_dock();
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id()
            );
            assert_eq!(
                left_dock.read(cx).active_panel_size(window, cx).unwrap(),
                panel_1.size(window, cx)
            );

            left_dock.update(cx, |left_dock, cx| {
                left_dock.resize_active_panel(Some(px(1337.)), window, cx)
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
        panel_1.update_in(cx, |panel_1, window, cx| {
            panel_1.set_position(DockPosition::Right, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
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
                right_dock.read(cx).active_panel_size(window, cx).unwrap(),
                px(1337.)
            );

            // Now we move panel_2 to the left
            panel_2.set_position(DockPosition::Left, window, cx);
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
        panel_1.update_in(cx, |panel_1, window, cx| {
            panel_1.set_position(DockPosition::Left, window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            // Since panel_1 was visible on the right, we open the left dock and make panel_1 active.
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id()
            );
            assert_eq!(
                left_dock.read(cx).active_panel_size(window, cx).unwrap(),
                px(1337.)
            );
            // And the right dock should be closed as it no longer has any panels.
            assert!(!workspace.right_dock().read(cx).is_open());

            // Now we move panel_1 to the bottom
            panel_1.set_position(DockPosition::Bottom, window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            // Since panel_1 was visible on the left, we close the left dock.
            assert!(!workspace.left_dock().read(cx).is_open());
            // The bottom dock is sized based on the panel's default size,
            // since the panel orientation changed from vertical to horizontal.
            let bottom_dock = workspace.bottom_dock();
            assert_eq!(
                bottom_dock.read(cx).active_panel_size(window, cx).unwrap(),
                panel_1.size(window, cx),
            );
            // Close bottom dock and move panel_1 back to the left.
            bottom_dock.update(cx, |bottom_dock, cx| {
                bottom_dock.set_open(false, window, cx)
            });
            panel_1.set_position(DockPosition::Left, window, cx);
        });

        // Emit activated event on panel 1
        panel_1.update(cx, |_, cx| cx.emit(PanelEvent::Activate));

        // Now the left dock is open and panel_1 is active and focused.
        workspace.update_in(cx, |workspace, window, cx| {
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().panel_id(),
                panel_1.panel_id(),
            );
            assert!(panel_1.focus_handle(cx).is_focused(window));
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
        panel_1.update_in(cx, |panel, window, cx| {
            panel.set_position(DockPosition::Right, window, cx)
        });
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
                workspace.update_in(cx, |workspace, window, cx| {
                    if let Some(_) = workspace.active_modal::<TestModal>(cx) {
                        workspace.toggle_modal(window, cx, TestModal::new);
                        workspace.toggle_modal(window, cx, TestModal::new);
                    } else {
                        workspace.toggle_modal(window, cx, TestModal::new);
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
        workspace.update_in(cx, |_workspace, window, cx| {
            cx.focus_self(window);
        });
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

    #[gpui::test]
    async fn test_no_save_prompt_when_multi_buffer_dirty_items_closed(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        let dirty_regular_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("1.txt")
                .with_project_items(&[dirty_project_item(1, "1.txt", cx)])
        });
        let dirty_regular_buffer_2 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("2.txt")
                .with_project_items(&[dirty_project_item(2, "2.txt", cx)])
        });
        let dirty_multi_buffer_with_both = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_label("Fake Project Search")
                .with_project_items(&[
                    dirty_regular_buffer.read(cx).project_items[0].clone(),
                    dirty_regular_buffer_2.read(cx).project_items[0].clone(),
                ])
        });
        let multi_buffer_with_both_files_id = dirty_multi_buffer_with_both.item_id();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_regular_buffer.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_regular_buffer_2.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_multi_buffer_with_both.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
        });

        pane.update_in(cx, |pane, window, cx| {
            pane.activate_item(2, true, true, window, cx);
            assert_eq!(
                pane.active_item().unwrap().item_id(),
                multi_buffer_with_both_files_id,
                "Should select the multi buffer in the pane"
            );
        });
        let close_all_but_multi_buffer_task = pane
            .update_in(cx, |pane, window, cx| {
                pane.close_inactive_items(
                    &CloseInactiveItems {
                        save_intent: Some(SaveIntent::Save),
                        close_pinned: true,
                    },
                    window,
                    cx,
                )
            })
            .expect("should have inactive files to close");
        cx.background_executor.run_until_parked();
        assert!(!cx.has_pending_prompt());
        close_all_but_multi_buffer_task
            .await
            .expect("Closing all buffers but the multi buffer failed");
        pane.update(cx, |pane, cx| {
            assert_eq!(dirty_regular_buffer.read(cx).save_count, 1);
            assert_eq!(dirty_multi_buffer_with_both.read(cx).save_count, 0);
            assert_eq!(dirty_regular_buffer_2.read(cx).save_count, 1);
            assert_eq!(pane.items_len(), 1);
            assert_eq!(
                pane.active_item().unwrap().item_id(),
                multi_buffer_with_both_files_id,
                "Should have only the multi buffer left in the pane"
            );
            assert!(
                dirty_multi_buffer_with_both.read(cx).is_dirty,
                "The multi buffer containing the unsaved buffer should still be dirty"
            );
        });

        dirty_regular_buffer.update(cx, |buffer, cx| {
            buffer.project_items[0].update(cx, |pi, _| pi.is_dirty = true)
        });

        let close_multi_buffer_task = pane
            .update_in(cx, |pane, window, cx| {
                pane.close_active_item(
                    &CloseActiveItem {
                        save_intent: Some(SaveIntent::Close),
                        close_pinned: false,
                    },
                    window,
                    cx,
                )
            })
            .expect("should have the multi buffer to close");
        cx.background_executor.run_until_parked();
        assert!(
            cx.has_pending_prompt(),
            "Dirty multi buffer should prompt a save dialog"
        );
        cx.simulate_prompt_answer("Save");
        cx.background_executor.run_until_parked();
        close_multi_buffer_task
            .await
            .expect("Closing the multi buffer failed");
        pane.update(cx, |pane, cx| {
            assert_eq!(
                dirty_multi_buffer_with_both.read(cx).save_count,
                1,
                "Multi buffer item should get be saved"
            );
            // Test impl does not save inner items, so we do not assert them
            assert_eq!(
                pane.items_len(),
                0,
                "No more items should be left in the pane"
            );
            assert!(pane.active_item().is_none());
        });
    }

    #[gpui::test]
    async fn test_save_prompt_when_dirty_multi_buffer_closed_with_some_of_its_dirty_items_not_present_in_the_pane(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        let dirty_regular_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("1.txt")
                .with_project_items(&[dirty_project_item(1, "1.txt", cx)])
        });
        let dirty_regular_buffer_2 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("2.txt")
                .with_project_items(&[dirty_project_item(2, "2.txt", cx)])
        });
        let clear_regular_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_label("3.txt")
                .with_project_items(&[TestProjectItem::new(3, "3.txt", cx)])
        });

        let dirty_multi_buffer_with_both = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_label("Fake Project Search")
                .with_project_items(&[
                    dirty_regular_buffer.read(cx).project_items[0].clone(),
                    dirty_regular_buffer_2.read(cx).project_items[0].clone(),
                    clear_regular_buffer.read(cx).project_items[0].clone(),
                ])
        });
        let multi_buffer_with_both_files_id = dirty_multi_buffer_with_both.item_id();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_regular_buffer.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_multi_buffer_with_both.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
        });

        pane.update_in(cx, |pane, window, cx| {
            pane.activate_item(1, true, true, window, cx);
            assert_eq!(
                pane.active_item().unwrap().item_id(),
                multi_buffer_with_both_files_id,
                "Should select the multi buffer in the pane"
            );
        });
        let _close_multi_buffer_task = pane
            .update_in(cx, |pane, window, cx| {
                pane.close_active_item(
                    &CloseActiveItem {
                        save_intent: None,
                        close_pinned: false,
                    },
                    window,
                    cx,
                )
            })
            .expect("should have active multi buffer to close");
        cx.background_executor.run_until_parked();
        assert!(
            cx.has_pending_prompt(),
            "With one dirty item from the multi buffer not being in the pane, a save prompt should be shown"
        );
    }

    #[gpui::test]
    async fn test_no_save_prompt_when_dirty_multi_buffer_closed_with_all_of_its_dirty_items_present_in_the_pane(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        let dirty_regular_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("1.txt")
                .with_project_items(&[dirty_project_item(1, "1.txt", cx)])
        });
        let dirty_regular_buffer_2 = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_label("2.txt")
                .with_project_items(&[dirty_project_item(2, "2.txt", cx)])
        });
        let clear_regular_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_label("3.txt")
                .with_project_items(&[TestProjectItem::new(3, "3.txt", cx)])
        });

        let dirty_multi_buffer = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_singleton(false)
                .with_label("Fake Project Search")
                .with_project_items(&[
                    dirty_regular_buffer.read(cx).project_items[0].clone(),
                    dirty_regular_buffer_2.read(cx).project_items[0].clone(),
                    clear_regular_buffer.read(cx).project_items[0].clone(),
                ])
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_regular_buffer.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_regular_buffer_2.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
            workspace.add_item(
                pane.clone(),
                Box::new(dirty_multi_buffer.clone()),
                None,
                false,
                false,
                window,
                cx,
            );
        });

        pane.update_in(cx, |pane, window, cx| {
            pane.activate_item(2, true, true, window, cx);
            assert_eq!(
                pane.active_item().unwrap().item_id(),
                dirty_multi_buffer.item_id(),
                "Should select the multi buffer in the pane"
            );
        });
        let close_multi_buffer_task = pane
            .update_in(cx, |pane, window, cx| {
                pane.close_active_item(
                    &CloseActiveItem {
                        save_intent: None,
                        close_pinned: false,
                    },
                    window,
                    cx,
                )
            })
            .expect("should have active multi buffer to close");
        cx.background_executor.run_until_parked();
        assert!(
            !cx.has_pending_prompt(),
            "All dirty items from the multi buffer are in the pane still, no save prompts should be shown"
        );
        close_multi_buffer_task
            .await
            .expect("Closing multi buffer failed");
        pane.update(cx, |pane, cx| {
            assert_eq!(dirty_regular_buffer.read(cx).save_count, 0);
            assert_eq!(dirty_multi_buffer.read(cx).save_count, 0);
            assert_eq!(dirty_regular_buffer_2.read(cx).save_count, 0);
            assert_eq!(
                pane.items()
                    .map(|item| item.item_id())
                    .sorted()
                    .collect::<Vec<_>>(),
                vec![
                    dirty_regular_buffer.item_id(),
                    dirty_regular_buffer_2.item_id(),
                ],
                "Should have no multi buffer left in the pane"
            );
            assert!(dirty_regular_buffer.read(cx).is_dirty);
            assert!(dirty_regular_buffer_2.read(cx).is_dirty);
        });
    }

    #[gpui::test]
    async fn test_move_focused_panel_to_next_position(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        // Add a new panel to the right dock, opening the dock and setting the
        // focus to the new panel.
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TestPanel::new(DockPosition::Right, cx));
            workspace.add_panel(panel.clone(), window, cx);

            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, window, cx));

            workspace.toggle_panel_focus::<TestPanel>(window, cx);

            panel
        });

        // Dispatch the `MoveFocusedPanelToNextPosition` action, moving the
        // panel to the next valid position which, in this case, is the left
        // dock.
        cx.dispatch_action(MoveFocusedPanelToNextPosition);
        workspace.update(cx, |workspace, cx| {
            assert!(workspace.left_dock().read(cx).is_open());
            assert_eq!(panel.read(cx).position, DockPosition::Left);
        });

        // Dispatch the `MoveFocusedPanelToNextPosition` action, moving the
        // panel to the next valid position which, in this case, is the bottom
        // dock.
        cx.dispatch_action(MoveFocusedPanelToNextPosition);
        workspace.update(cx, |workspace, cx| {
            assert!(workspace.bottom_dock().read(cx).is_open());
            assert_eq!(panel.read(cx).position, DockPosition::Bottom);
        });

        // Dispatch the `MoveFocusedPanelToNextPosition` action again, this time
        // around moving the panel to its initial position, the right dock.
        cx.dispatch_action(MoveFocusedPanelToNextPosition);
        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert_eq!(panel.read(cx).position, DockPosition::Right);
        });

        // Remove focus from the panel, ensuring that, if the panel is not
        // focused, the `MoveFocusedPanelToNextPosition` action does not update
        // the panel's position, so the panel is still in the right dock.
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });

        cx.dispatch_action(MoveFocusedPanelToNextPosition);
        workspace.update(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert_eq!(panel.read(cx).position, DockPosition::Right);
        });
    }

    mod register_project_item_tests {

        use super::*;

        // View
        struct TestPngItemView {
            focus_handle: FocusHandle,
        }
        // Model
        struct TestPngItem {}

        impl project::ProjectItem for TestPngItem {
            fn try_open(
                _project: &Entity<Project>,
                path: &ProjectPath,
                cx: &mut App,
            ) -> Option<Task<gpui::Result<Entity<Self>>>> {
                if path.path.extension().unwrap() == "png" {
                    Some(cx.spawn(async move |cx| cx.new(|_| TestPngItem {})))
                } else {
                    None
                }
            }

            fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
                None
            }

            fn project_path(&self, _: &App) -> Option<ProjectPath> {
                None
            }

            fn is_dirty(&self) -> bool {
                false
            }
        }

        impl Item for TestPngItemView {
            type Event = ();
            fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
                "".into()
            }
        }
        impl EventEmitter<()> for TestPngItemView {}
        impl Focusable for TestPngItemView {
            fn focus_handle(&self, _cx: &App) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestPngItemView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestPngItemView {
            type Item = TestPngItem;

            fn for_project_item(
                _project: Entity<Project>,
                _pane: Option<&Pane>,
                _item: Entity<Self::Item>,
                _: &mut Window,
                cx: &mut Context<Self>,
            ) -> Self
            where
                Self: Sized,
            {
                Self {
                    focus_handle: cx.focus_handle(),
                }
            }
        }

        // View
        struct TestIpynbItemView {
            focus_handle: FocusHandle,
        }
        // Model
        struct TestIpynbItem {}

        impl project::ProjectItem for TestIpynbItem {
            fn try_open(
                _project: &Entity<Project>,
                path: &ProjectPath,
                cx: &mut App,
            ) -> Option<Task<gpui::Result<Entity<Self>>>> {
                if path.path.extension().unwrap() == "ipynb" {
                    Some(cx.spawn(async move |cx| cx.new(|_| TestIpynbItem {})))
                } else {
                    None
                }
            }

            fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
                None
            }

            fn project_path(&self, _: &App) -> Option<ProjectPath> {
                None
            }

            fn is_dirty(&self) -> bool {
                false
            }
        }

        impl Item for TestIpynbItemView {
            type Event = ();
            fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
                "".into()
            }
        }
        impl EventEmitter<()> for TestIpynbItemView {}
        impl Focusable for TestIpynbItemView {
            fn focus_handle(&self, _cx: &App) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestIpynbItemView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestIpynbItemView {
            type Item = TestIpynbItem;

            fn for_project_item(
                _project: Entity<Project>,
                _pane: Option<&Pane>,
                _item: Entity<Self::Item>,
                _: &mut Window,
                cx: &mut Context<Self>,
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

        impl Item for TestAlternatePngItemView {
            type Event = ();
            fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
                "".into()
            }
        }

        impl EventEmitter<()> for TestAlternatePngItemView {}
        impl Focusable for TestAlternatePngItemView {
            fn focus_handle(&self, _cx: &App) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl Render for TestAlternatePngItemView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                Empty
            }
        }

        impl ProjectItem for TestAlternatePngItemView {
            type Item = TestPngItem;

            fn for_project_item(
                _project: Entity<Project>,
                _pane: Option<&Pane>,
                _item: Entity<Self::Item>,
                _: &mut Window,
                cx: &mut Context<Self>,
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
            let (workspace, cx) =
                cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

            let worktree_id = project.update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            });

            let handle = workspace
                .update_in(cx, |workspace, window, cx| {
                    let project_path = (worktree_id, "one.png");
                    workspace.open_path(project_path, None, true, window, cx)
                })
                .await
                .unwrap();

            // Now we can check if the handle we got back errored or not
            assert_eq!(
                handle.to_any().entity_type(),
                TypeId::of::<TestPngItemView>()
            );

            let handle = workspace
                .update_in(cx, |workspace, window, cx| {
                    let project_path = (worktree_id, "two.ipynb");
                    workspace.open_path(project_path, None, true, window, cx)
                })
                .await
                .unwrap();

            assert_eq!(
                handle.to_any().entity_type(),
                TypeId::of::<TestIpynbItemView>()
            );

            let handle = workspace
                .update_in(cx, |workspace, window, cx| {
                    let project_path = (worktree_id, "three.txt");
                    workspace.open_path(project_path, None, true, window, cx)
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
            let (workspace, cx) =
                cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
            let worktree_id = project.update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            });

            let handle = workspace
                .update_in(cx, |workspace, window, cx| {
                    let project_path = (worktree_id, "one.png");
                    workspace.open_path(project_path, None, true, window, cx)
                })
                .await
                .unwrap();

            // This _must_ be the second item registered
            assert_eq!(
                handle.to_any().entity_type(),
                TypeId::of::<TestAlternatePngItemView>()
            );

            let handle = workspace
                .update_in(cx, |workspace, window, cx| {
                    let project_path = (worktree_id, "three.txt");
                    workspace.open_path(project_path, None, true, window, cx)
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

    fn dirty_project_item(id: u64, path: &str, cx: &mut App) -> Entity<TestProjectItem> {
        let item = TestProjectItem::new(id, path, cx);
        item.update(cx, |item, _| {
            item.is_dirty = true;
        });
        item
    }
}
