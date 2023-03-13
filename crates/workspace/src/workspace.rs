/// NOTE: Focus only 'takes' after an update has flushed_effects.
///
/// This may cause issues when you're trying to write tests that use workspace focus to add items at
/// specific locations.
pub mod dock;
pub mod item;
pub mod notifications;
pub mod pane;
pub mod pane_group;
mod persistence;
pub mod searchable;
pub mod shared_screen;
pub mod sidebar;
mod status_bar;
pub mod terminal_button;
mod toolbar;

pub use smallvec;

use anyhow::{anyhow, Context, Result};
use call::ActiveCall;
use client::{
    proto::{self, PeerId},
    Client, TypedEnvelope, UserStore,
};
use collections::{hash_map, HashMap, HashSet};
use dock::{Dock, DockDefaultItemFactory, ToggleDockButton};
use drag_and_drop::DragAndDrop;
use fs::{self, Fs};
use futures::{
    channel::{mpsc, oneshot},
    future::try_join_all,
    FutureExt, StreamExt,
};
use gpui::{
    actions,
    elements::*,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    impl_actions, impl_internal_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, WindowOptions},
    AnyModelHandle, AnyViewHandle, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle,
    MouseButton, MutableAppContext, PathPromptOptions, Platform, PromptLevel, RenderContext,
    SizeConstraint, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
    WindowBounds,
};
use item::{FollowableItem, FollowableItemHandle, Item, ItemHandle, ProjectItem};
use language::LanguageRegistry;
use std::{
    any::TypeId,
    borrow::Cow,
    cmp, env,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use terminal_button::TerminalButton;

use crate::{
    notifications::simple_message_notification::{MessageNotification, OsOpen},
    persistence::model::{SerializedPane, SerializedPaneGroup, SerializedWorkspace},
};
use lazy_static::lazy_static;
use log::{error, warn};
use notifications::{NotificationHandle, NotifyResultExt};
pub use pane::*;
pub use pane_group::*;
use persistence::{model::SerializedItem, DB};
pub use persistence::{
    model::{ItemId, WorkspaceLocation},
    WorkspaceDb, DB as WORKSPACE_DB,
};
use postage::prelude::Stream;
use project::{Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId};
use serde::Deserialize;
use settings::{Autosave, DockAnchor, Settings};
use shared_screen::SharedScreen;
use sidebar::{Sidebar, SidebarButtons, SidebarSide, ToggleSidebarItem};
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use theme::{Theme, ThemeRegistry};
pub use toolbar::{ToolbarItemLocation, ToolbarItemView};
use util::ResultExt;

lazy_static! {
    static ref ZED_WINDOW_SIZE: Option<Vector2F> = env::var("ZED_WINDOW_SIZE")
        .ok()
        .as_deref()
        .and_then(parse_pixel_position_env_var);
    static ref ZED_WINDOW_POSITION: Option<Vector2F> = env::var("ZED_WINDOW_POSITION")
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
        NewFile,
        NewWindow,
        CloseWindow,
        AddFolderToProject,
        Unfollow,
        Save,
        SaveAs,
        SaveAll,
        ActivatePreviousPane,
        ActivateNextPane,
        FollowNextCollaborator,
        ToggleLeftSidebar,
        NewTerminal,
        NewSearch,
        Feedback,
        Restart,
        Welcome
    ]
);

#[derive(Clone, PartialEq)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePane(pub usize);

#[derive(Clone, PartialEq)]
pub struct ToggleFollow(pub PeerId);

#[derive(Clone, PartialEq)]
pub struct JoinProject {
    pub project_id: u64,
    pub follow_user_id: u64,
}

#[derive(Clone, PartialEq)]
pub struct OpenSharedScreen {
    pub peer_id: PeerId,
}

#[derive(Clone, PartialEq)]
pub struct SplitWithItem {
    pane_to_split: WeakViewHandle<Pane>,
    split_direction: SplitDirection,
    from: WeakViewHandle<Pane>,
    item_id_to_move: usize,
}

#[derive(Clone, PartialEq)]
pub struct SplitWithProjectEntry {
    pane_to_split: WeakViewHandle<Pane>,
    split_direction: SplitDirection,
    project_entry: ProjectEntryId,
}

#[derive(Clone, PartialEq)]
pub struct OpenProjectEntryInPane {
    pane: WeakViewHandle<Pane>,
    project_entry: ProjectEntryId,
}

pub type WorkspaceId = i64;

impl_internal_actions!(
    workspace,
    [
        OpenPaths,
        ToggleFollow,
        JoinProject,
        OpenSharedScreen,
        RemoveWorktreeFromProject,
        SplitWithItem,
        SplitWithProjectEntry,
        OpenProjectEntryInPane,
    ]
);
impl_actions!(workspace, [ActivatePane]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    pane::init(cx);
    dock::init(cx);
    notifications::init(cx);

    cx.add_global_action(|_: &Open, cx: &mut MutableAppContext| {
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
    });
    cx.add_action(|_, _: &Open, cx: &mut ViewContext<Workspace>| {
        let mut paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        });

        let handle = cx.handle().downgrade();
        cx.spawn(|_, mut cx| async move {
            if let Some(paths) = paths.recv().await.flatten() {
                cx.update(|cx| {
                    cx.dispatch_action_at(handle.window_id(), handle.id(), OpenPaths { paths })
                })
            }
        })
        .detach();
    });
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |action: &OpenPaths, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_paths(&action.paths, &app_state, None, cx).detach();
            }
        }
    });
    cx.add_async_action({
        let app_state = Arc::downgrade(&app_state);
        move |workspace, action: &OpenPaths, cx: &mut ViewContext<Workspace>| {
            if !workspace.project().read(cx).is_local() {
                cx.propagate_action();
                return None;
            }

            let app_state = app_state.upgrade()?;
            let window_id = cx.window_id();
            let action = action.clone();
            let close = workspace.prepare_to_close(false, cx);

            Some(cx.spawn_weak(|_, mut cx| async move {
                let can_close = close.await?;
                if can_close {
                    cx.update(|cx| open_paths(&action.paths, &app_state, Some(window_id), cx))
                        .await;
                }
                Ok(())
            }))
        }
    });

    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &NewWindow, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_new(&app_state, cx, |_, cx| cx.dispatch_action(NewFile)).detach();
            }
        }
    });

    cx.add_async_action(Workspace::toggle_follow);
    cx.add_async_action(Workspace::follow_next_collaborator);
    cx.add_async_action(Workspace::close);
    cx.add_global_action(Workspace::close_global);
    cx.add_async_action(Workspace::save_all);
    cx.add_action(Workspace::open_shared_screen);
    cx.add_action(Workspace::add_folder_to_project);
    cx.add_action(Workspace::remove_folder_from_project);
    cx.add_action(
        |workspace: &mut Workspace, _: &Unfollow, cx: &mut ViewContext<Workspace>| {
            let pane = workspace.active_pane().clone();
            workspace.unfollow(&pane, cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &Save, cx: &mut ViewContext<Workspace>| {
            workspace.save_active_item(false, cx).detach_and_log_err(cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &SaveAs, cx: &mut ViewContext<Workspace>| {
            workspace.save_active_item(true, cx).detach_and_log_err(cx);
        },
    );
    cx.add_action(Workspace::toggle_sidebar_item);
    cx.add_action(Workspace::focus_center);
    cx.add_action(|workspace: &mut Workspace, _: &ActivatePreviousPane, cx| {
        workspace.activate_previous_pane(cx)
    });
    cx.add_action(|workspace: &mut Workspace, _: &ActivateNextPane, cx| {
        workspace.activate_next_pane(cx)
    });
    cx.add_action(|workspace: &mut Workspace, _: &ToggleLeftSidebar, cx| {
        workspace.toggle_sidebar(SidebarSide::Left, cx);
    });
    cx.add_action(Workspace::activate_pane_at_index);

    cx.add_action(Workspace::split_pane_with_item);
    cx.add_action(Workspace::split_pane_with_project_entry);

    cx.add_async_action(
        |workspace: &mut Workspace,
         OpenProjectEntryInPane {
             pane,
             project_entry,
         }: &_,
         cx| {
            workspace
                .project
                .read(cx)
                .path_for_entry(*project_entry, cx)
                .map(|path| {
                    let task = workspace.open_path(path, Some(pane.clone()), true, cx);
                    cx.foreground().spawn(async move {
                        task.await?;
                        Ok(())
                    })
                })
        },
    );

    cx.add_action(|_: &mut Workspace, _: &install_cli::Install, cx| {
        cx.spawn(|workspace, mut cx| async move {
            let err = install_cli::install_cli(&cx)
                .await
                .context("Failed to create CLI symlink");

            cx.update(|cx| {
                workspace.update(cx, |workspace, cx| {
                    if matches!(err, Err(_)) {
                        err.notify_err(workspace, cx);
                    } else {
                        workspace.show_notification(1, cx, |cx| {
                            cx.add_view(|_| {
                                MessageNotification::new_message(
                                    "Successfully installed the `zed` binary",
                                )
                            })
                        });
                    }
                })
            })
        })
        .detach();
    });

    let client = &app_state.client;
    client.add_view_request_handler(Workspace::handle_follow);
    client.add_view_message_handler(Workspace::handle_unfollow);
    client.add_view_message_handler(Workspace::handle_update_followers);
}

type ProjectItemBuilders = HashMap<
    TypeId,
    fn(ModelHandle<Project>, AnyModelHandle, &mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
>;
pub fn register_project_item<I: ProjectItem>(cx: &mut MutableAppContext) {
    cx.update_default_global(|builders: &mut ProjectItemBuilders, _| {
        builders.insert(TypeId::of::<I::Item>(), |project, model, cx| {
            let item = model.downcast::<I::Item>().unwrap();
            Box::new(cx.add_view(|cx| I::for_project_item(project, item, cx)))
        });
    });
}

type FollowableItemBuilder = fn(
    ViewHandle<Pane>,
    ModelHandle<Project>,
    ViewId,
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
pub fn register_followable_item<I: FollowableItem>(cx: &mut MutableAppContext) {
    cx.update_default_global(|builders: &mut FollowableItemBuilders, _| {
        builders.insert(
            TypeId::of::<I>(),
            (
                |pane, project, id, state, cx| {
                    I::from_state_proto(pane, project, id, state, cx).map(|task| {
                        cx.foreground()
                            .spawn(async move { Ok(Box::new(task.await?) as Box<_>) })
                    })
                },
                |this| Box::new(this.downcast::<I>().unwrap()),
            ),
        );
    });
}

type ItemDeserializers = HashMap<
    Arc<str>,
    fn(
        ModelHandle<Project>,
        WeakViewHandle<Workspace>,
        WorkspaceId,
        ItemId,
        &mut ViewContext<Pane>,
    ) -> Task<Result<Box<dyn ItemHandle>>>,
>;
pub fn register_deserializable_item<I: Item>(cx: &mut MutableAppContext) {
    cx.update_default_global(|deserializers: &mut ItemDeserializers, _cx| {
        if let Some(serialized_item_kind) = I::serialized_item_kind() {
            deserializers.insert(
                Arc::from(serialized_item_kind),
                |project, workspace, workspace_id, item_id, cx| {
                    let task = I::deserialize(project, workspace, workspace_id, item_id, cx);
                    cx.foreground()
                        .spawn(async { Ok(Box::new(task.await?) as Box<_>) })
                },
            );
        }
    });
}

pub struct AppState {
    pub languages: Arc<LanguageRegistry>,
    pub themes: Arc<ThemeRegistry>,
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub build_window_options:
        fn(Option<WindowBounds>, Option<uuid::Uuid>, &dyn Platform) -> WindowOptions<'static>,
    pub initialize_workspace: fn(&mut Workspace, &Arc<AppState>, &mut ViewContext<Workspace>),
    pub dock_default_item_factory: DockDefaultItemFactory,
    pub background_actions: BackgroundActions,
}

impl AppState {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut MutableAppContext) -> Arc<Self> {
        let settings = Settings::test(cx);
        cx.set_global(settings);

        let fs = fs::FakeFs::new(cx.background().clone());
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone(), cx);
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let themes = ThemeRegistry::new((), cx.font_cache().clone());
        Arc::new(Self {
            client,
            themes,
            fs,
            languages,
            user_store,
            initialize_workspace: |_, _, _| {},
            build_window_options: |_, _, _| Default::default(),
            dock_default_item_factory: |_, _| None,
            background_actions: || &[],
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

    fn fire_new<F, Fut>(
        &mut self,
        delay: Duration,
        workspace: &Workspace,
        cx: &mut ViewContext<Workspace>,
        f: F,
    ) where
        F: FnOnce(ModelHandle<Project>, AsyncAppContext) -> Fut + 'static,
        Fut: 'static + Future<Output = ()>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            _ = channel.send(());
        }

        let project = workspace.project().downgrade();

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        let previous_task = self.task.take();
        self.task = Some(cx.spawn_weak(|_, cx| async move {
            let mut timer = cx.background().timer(delay).fuse();
            if let Some(previous_task) = previous_task {
                previous_task.await;
            }

            futures::select_biased! {
                _ = receiver => return,
                    _ = timer => {}
            }

            if let Some(project) = project.upgrade(&cx) {
                (f)(project, cx).await;
            }
        }));
    }
}

pub enum Event {
    DockAnchorChanged,
    PaneAdded(ViewHandle<Pane>),
    ContactRequestedJoin(u64),
}

pub struct Workspace {
    weak_self: WeakViewHandle<Self>,
    client: Arc<Client>,
    user_store: ModelHandle<client::UserStore>,
    remote_entity_subscription: Option<client::Subscription>,
    fs: Arc<dyn Fs>,
    modal: Option<AnyViewHandle>,
    center: PaneGroup,
    left_sidebar: ViewHandle<Sidebar>,
    right_sidebar: ViewHandle<Sidebar>,
    panes: Vec<ViewHandle<Pane>>,
    panes_by_item: HashMap<usize, WeakViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    last_active_center_pane: Option<WeakViewHandle<Pane>>,
    status_bar: ViewHandle<StatusBar>,
    titlebar_item: Option<AnyViewHandle>,
    dock: Dock,
    notifications: Vec<(TypeId, usize, Box<dyn NotificationHandle>)>,
    project: ModelHandle<Project>,
    leader_state: LeaderState,
    follower_states_by_leader: FollowerStatesByLeader,
    last_leaders_by_pane: HashMap<WeakViewHandle<Pane>, PeerId>,
    window_edited: bool,
    active_call: Option<(ModelHandle<ActiveCall>, Vec<gpui::Subscription>)>,
    leader_updates_tx: mpsc::UnboundedSender<(PeerId, proto::UpdateFollowers)>,
    database_id: WorkspaceId,
    background_actions: BackgroundActions,
    _window_subscriptions: [Subscription; 3],
    _apply_leader_updates: Task<Result<()>>,
    _observe_current_user: Task<()>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewId {
    pub creator: PeerId,
    pub id: u64,
}

#[derive(Default)]
struct LeaderState {
    followers: HashSet<PeerId>,
}

type FollowerStatesByLeader = HashMap<PeerId, HashMap<ViewHandle<Pane>, FollowerState>>;

#[derive(Default)]
struct FollowerState {
    active_view_id: Option<ViewId>,
    items_by_leader_view_id: HashMap<ViewId, Box<dyn FollowableItemHandle>>,
}

impl Workspace {
    pub fn new(
        serialized_workspace: Option<SerializedWorkspace>,
        workspace_id: WorkspaceId,
        project: ModelHandle<Project>,
        dock_default_factory: DockDefaultItemFactory,
        background_actions: BackgroundActions,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&project, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&project, move |this, _, event, cx| {
            match event {
                project::Event::RemoteIdChanged(remote_id) => {
                    this.update_window_title(cx);
                    this.project_remote_id_changed(*remote_id, cx);
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
                    cx.blur();
                }

                _ => {}
            }
            cx.notify()
        })
        .detach();

        let weak_handle = cx.weak_handle();

        let center_pane =
            cx.add_view(|cx| Pane::new(weak_handle.id(), None, background_actions, cx));
        let pane_id = center_pane.id();
        cx.subscribe(&center_pane, move |this, _, event, cx| {
            this.handle_pane_event(pane_id, event, cx)
        })
        .detach();
        cx.focus(&center_pane);
        cx.emit(Event::PaneAdded(center_pane.clone()));
        let dock = Dock::new(
            weak_handle.id(),
            dock_default_factory,
            background_actions,
            cx,
        );
        let dock_pane = dock.pane().clone();

        let fs = project.read(cx).fs().clone();
        let user_store = project.read(cx).user_store();
        let client = project.read(cx).client();
        let mut current_user = user_store.read(cx).watch_current_user();
        let mut connection_status = client.status();
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
        let handle = cx.handle();

        // All leader updates are enqueued and then processed in a single task, so
        // that each asynchronous operation can be run in order.
        let (leader_updates_tx, mut leader_updates_rx) =
            mpsc::unbounded::<(PeerId, proto::UpdateFollowers)>();
        let _apply_leader_updates = cx.spawn_weak(|this, mut cx| async move {
            while let Some((leader_id, update)) = leader_updates_rx.next().await {
                let Some(this) = this.upgrade(&cx) else { break };
                Self::process_leader_update(this, leader_id, update, &mut cx)
                    .await
                    .log_err();
            }

            Ok(())
        });

        cx.emit_global(WorkspaceCreated(weak_handle.clone()));

        let left_sidebar = cx.add_view(|_| Sidebar::new(SidebarSide::Left));
        let right_sidebar = cx.add_view(|_| Sidebar::new(SidebarSide::Right));
        let left_sidebar_buttons = cx.add_view(|cx| SidebarButtons::new(left_sidebar.clone(), cx));
        let toggle_terminal = cx.add_view(|cx| TerminalButton::new(handle.clone(), cx));
        let toggle_dock = cx.add_view(|cx| ToggleDockButton::new(handle, cx));
        let right_sidebar_buttons =
            cx.add_view(|cx| SidebarButtons::new(right_sidebar.clone(), cx));
        let status_bar = cx.add_view(|cx| {
            let mut status_bar = StatusBar::new(&center_pane.clone(), cx);
            status_bar.add_left_item(left_sidebar_buttons, cx);
            status_bar.add_right_item(right_sidebar_buttons, cx);
            status_bar.add_right_item(toggle_dock, cx);
            status_bar.add_right_item(toggle_terminal, cx);
            status_bar
        });

        cx.update_default_global::<DragAndDrop<Workspace>, _, _>(|drag_and_drop, _| {
            drag_and_drop.register_container(weak_handle.clone());
        });

        let mut active_call = None;
        if cx.has_global::<ModelHandle<ActiveCall>>() {
            let call = cx.global::<ModelHandle<ActiveCall>>().clone();
            let mut subscriptions = Vec::new();
            subscriptions.push(cx.subscribe(&call, Self::on_active_call_event));
            active_call = Some((call, subscriptions));
        }

        let subscriptions = [
            cx.observe_fullscreen(|_, _, cx| cx.notify()),
            cx.observe_window_activation(Self::on_window_activation_changed),
            cx.observe_window_bounds(move |_, mut bounds, display, cx| {
                // Transform fixed bounds to be stored in terms of the containing display
                if let WindowBounds::Fixed(mut window_bounds) = bounds {
                    if let Some(screen) = cx.platform().screen_by_id(display) {
                        let screen_bounds = screen.bounds();
                        window_bounds
                            .set_origin_x(window_bounds.origin_x() - screen_bounds.origin_x());
                        window_bounds
                            .set_origin_y(window_bounds.origin_y() - screen_bounds.origin_y());
                        bounds = WindowBounds::Fixed(window_bounds);
                    }
                }

                cx.background()
                    .spawn(DB.set_window_bounds(workspace_id, bounds, display))
                    .detach_and_log_err(cx);
            }),
        ];

        let mut this = Workspace {
            modal: None,
            weak_self: weak_handle.clone(),
            center: PaneGroup::new(center_pane.clone()),
            dock,
            // When removing an item, the last element remaining in this array
            // is used to find where focus should fallback to. As such, the order
            // of these two variables is important.
            panes: vec![dock_pane.clone(), center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane.clone(),
            last_active_center_pane: Some(center_pane.downgrade()),
            status_bar,
            titlebar_item: None,
            notifications: Default::default(),
            client,
            remote_entity_subscription: None,
            user_store,
            fs,
            left_sidebar,
            right_sidebar,
            project: project.clone(),
            leader_state: Default::default(),
            follower_states_by_leader: Default::default(),
            last_leaders_by_pane: Default::default(),
            window_edited: false,
            active_call,
            database_id: workspace_id,
            background_actions,
            _observe_current_user,
            _apply_leader_updates,
            leader_updates_tx,
            _window_subscriptions: subscriptions,
        };
        this.project_remote_id_changed(project.read(cx).remote_id(), cx);
        cx.defer(|this, cx| this.update_window_title(cx));

        if let Some(serialized_workspace) = serialized_workspace {
            cx.defer(move |_, cx| {
                Self::load_from_serialized_workspace(weak_handle, serialized_workspace, cx)
            });
        } else if project.read(cx).is_local() {
            if cx.global::<Settings>().default_dock_anchor != DockAnchor::Expanded {
                Dock::show(&mut this, false, cx);
            }
        }

        this
    }

    fn new_local(
        abs_paths: Vec<PathBuf>,
        app_state: Arc<AppState>,
        requesting_window_id: Option<usize>,
        cx: &mut MutableAppContext,
    ) -> Task<(
        ViewHandle<Workspace>,
        Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>,
    )> {
        let project_handle = Project::local(
            app_state.client.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            cx,
        );

        cx.spawn(|mut cx| async move {
            let serialized_workspace = persistence::DB.workspace_for_roots(&abs_paths.as_slice());

            let paths_to_open = serialized_workspace
                .as_ref()
                .map(|workspace| workspace.location.paths())
                .unwrap_or(Arc::new(abs_paths));

            // Get project paths for all of the abs_paths
            let mut worktree_roots: HashSet<Arc<Path>> = Default::default();
            let mut project_paths = Vec::new();
            for path in paths_to_open.iter() {
                if let Some((worktree, project_entry)) = cx
                    .update(|cx| {
                        Workspace::project_path_for_path(project_handle.clone(), &path, true, cx)
                    })
                    .await
                    .log_err()
                {
                    worktree_roots.insert(worktree.read_with(&mut cx, |tree, _| tree.abs_path()));
                    project_paths.push(Some(project_entry));
                } else {
                    project_paths.push(None);
                }
            }

            let workspace_id = if let Some(serialized_workspace) = serialized_workspace.as_ref() {
                serialized_workspace.id
            } else {
                DB.next_id().await.unwrap_or(0)
            };

            let window_bounds_override =
                ZED_WINDOW_POSITION
                    .zip(*ZED_WINDOW_SIZE)
                    .map(|(position, size)| {
                        WindowBounds::Fixed(RectF::new(
                            cx.platform().screens()[0].bounds().origin() + position,
                            size,
                        ))
                    });

            let build_workspace =
                |cx: &mut ViewContext<Workspace>,
                 serialized_workspace: Option<SerializedWorkspace>| {
                    let mut workspace = Workspace::new(
                        serialized_workspace,
                        workspace_id,
                        project_handle,
                        app_state.dock_default_item_factory,
                        app_state.background_actions,
                        cx,
                    );
                    (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                    workspace
                };

            let workspace = if let Some(window_id) = requesting_window_id {
                cx.update(|cx| {
                    cx.replace_root_view(window_id, |cx| build_workspace(cx, serialized_workspace))
                })
            } else {
                let (bounds, display) = if let Some(bounds) = window_bounds_override {
                    (Some(bounds), None)
                } else {
                    serialized_workspace
                        .as_ref()
                        .and_then(|serialized_workspace| {
                            let display = serialized_workspace.display?;
                            let mut bounds = serialized_workspace.bounds?;

                            // Stored bounds are relative to the containing display.
                            // So convert back to global coordinates if that screen still exists
                            if let WindowBounds::Fixed(mut window_bounds) = bounds {
                                if let Some(screen) = cx.platform().screen_by_id(display) {
                                    let screen_bounds = screen.bounds();
                                    window_bounds.set_origin_x(
                                        window_bounds.origin_x() + screen_bounds.origin_x(),
                                    );
                                    window_bounds.set_origin_y(
                                        window_bounds.origin_y() + screen_bounds.origin_y(),
                                    );
                                    bounds = WindowBounds::Fixed(window_bounds);
                                } else {
                                    // Screen no longer exists. Return none here.
                                    return None;
                                }
                            }

                            Some((bounds, display))
                        })
                        .unzip()
                };

                // Use the serialized workspace to construct the new window
                cx.add_window(
                    (app_state.build_window_options)(bounds, display, cx.platform().as_ref()),
                    |cx| build_workspace(cx, serialized_workspace),
                )
                .1
            };

            notify_if_database_failed(&workspace, &mut cx);

            // Call open path for each of the project paths
            // (this will bring them to the front if they were in the serialized workspace)
            debug_assert!(paths_to_open.len() == project_paths.len());
            let tasks = paths_to_open
                .iter()
                .cloned()
                .zip(project_paths.into_iter())
                .map(|(abs_path, project_path)| {
                    let workspace = workspace.clone();
                    cx.spawn(|mut cx| {
                        let fs = app_state.fs.clone();
                        async move {
                            let project_path = project_path?;
                            if fs.is_file(&abs_path).await {
                                Some(
                                    workspace
                                        .update(&mut cx, |workspace, cx| {
                                            workspace.open_path(project_path, None, true, cx)
                                        })
                                        .await,
                                )
                            } else {
                                None
                            }
                        }
                    })
                });

            let opened_items = futures::future::join_all(tasks.into_iter()).await;

            (workspace, opened_items)
        })
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

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn set_titlebar_item(
        &mut self,
        item: impl Into<AnyViewHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        self.titlebar_item = Some(item.into());
        cx.notify();
    }

    pub fn titlebar_item(&self) -> Option<AnyViewHandle> {
        self.titlebar_item.clone()
    }

    /// Call the given callback with a workspace whose project is local.
    ///
    /// If the given workspace has a local project, then it will be passed
    /// to the callback. Otherwise, a new empty window will be created.
    pub fn with_local_workspace<T, F>(
        &mut self,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Self>,
        callback: F,
    ) -> Task<T>
    where
        T: 'static,
        F: 'static + FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
    {
        if self.project.read(cx).is_local() {
            Task::Ready(Some(callback(self, cx)))
        } else {
            let task = Self::new_local(Vec::new(), app_state.clone(), None, cx);
            cx.spawn(|_vh, mut cx| async move {
                let (workspace, _) = task.await;
                workspace.update(&mut cx, callback)
            })
        }
    }

    pub fn worktrees<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
        self.project.read(cx).worktrees(cx)
    }

    pub fn visible_worktrees<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
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

    pub fn close_global(_: &CloseWindow, cx: &mut MutableAppContext) {
        let id = cx.window_ids().find(|&id| cx.window_is_active(id));
        if let Some(id) = id {
            //This can only get called when the window's project connection has been lost
            //so we don't need to prompt the user for anything and instead just close the window
            cx.remove_window(id);
        }
    }

    pub fn close(
        &mut self,
        _: &CloseWindow,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let prepare = self.prepare_to_close(false, cx);
        Some(cx.spawn(|this, mut cx| async move {
            if prepare.await? {
                this.update(&mut cx, |_, cx| {
                    let window_id = cx.window_id();
                    cx.remove_window(window_id);
                });
            }
            Ok(())
        }))
    }

    pub fn prepare_to_close(
        &mut self,
        quitting: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        let active_call = self.active_call().cloned();
        let window_id = cx.window_id();
        let workspace_count = cx
            .window_ids()
            .flat_map(|window_id| cx.root_view::<Workspace>(window_id))
            .count();

        cx.spawn(|this, mut cx| async move {
            if let Some(active_call) = active_call {
                if !quitting
                    && workspace_count == 1
                    && active_call.read_with(&cx, |call, _| call.room().is_some())
                {
                    let answer = cx
                        .prompt(
                            window_id,
                            PromptLevel::Warning,
                            "Do you want to leave the current call?",
                            &["Close window and hang up", "Cancel"],
                        )
                        .next()
                        .await;

                    if answer == Some(1) {
                        return anyhow::Ok(false);
                    } else {
                        active_call
                            .update(&mut cx, |call, cx| call.hang_up(cx))
                            .await
                            .log_err();
                    }
                }
            }

            Ok(this
                .update(&mut cx, |this, cx| this.save_all_internal(true, cx))
                .await?)
        })
    }

    fn save_all(&mut self, _: &SaveAll, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let save_all = self.save_all_internal(false, cx);
        Some(cx.foreground().spawn(async move {
            save_all.await?;
            Ok(())
        }))
    }

    fn save_all_internal(
        &mut self,
        should_prompt_to_save: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<bool>> {
        if self.project.read(cx).is_read_only() {
            return Task::ready(Ok(true));
        }

        let dirty_items = self
            .panes
            .iter()
            .flat_map(|pane| {
                pane.read(cx).items().filter_map(|item| {
                    if item.is_dirty(cx) {
                        Some((pane.clone(), item.boxed_clone()))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let project = self.project.clone();
        cx.spawn_weak(|_, mut cx| async move {
            for (pane, item) in dirty_items {
                let (singleton, project_entry_ids) =
                    cx.read(|cx| (item.is_singleton(cx), item.project_entry_ids(cx)));
                if singleton || !project_entry_ids.is_empty() {
                    if let Some(ix) =
                        pane.read_with(&cx, |pane, _| pane.index_for_item(item.as_ref()))
                    {
                        if !Pane::save_item(
                            project.clone(),
                            &pane,
                            ix,
                            &*item,
                            should_prompt_to_save,
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

    #[allow(clippy::type_complexity)]
    pub fn open_paths(
        &mut self,
        mut abs_paths: Vec<PathBuf>,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>> {
        let fs = self.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn(|this, mut cx| async move {
            let mut project_paths = Vec::new();
            for path in &abs_paths {
                project_paths.push(
                    this.update(&mut cx, |this, cx| {
                        Workspace::project_path_for_path(this.project.clone(), path, visible, cx)
                    })
                    .await
                    .log_err(),
                );
            }

            let tasks = abs_paths
                .iter()
                .cloned()
                .zip(project_paths.into_iter())
                .map(|(abs_path, project_path)| {
                    let this = this.clone();
                    cx.spawn(|mut cx| {
                        let fs = fs.clone();
                        async move {
                            let (_worktree, project_path) = project_path?;
                            if fs.is_file(&abs_path).await {
                                Some(
                                    this.update(&mut cx, |this, cx| {
                                        this.open_path(project_path, None, true, cx)
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

    fn add_folder_to_project(&mut self, _: &AddFolderToProject, cx: &mut ViewContext<Self>) {
        let mut paths = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: true,
        });
        cx.spawn(|this, mut cx| async move {
            if let Some(paths) = paths.recv().await.flatten() {
                let results = this
                    .update(&mut cx, |this, cx| this.open_paths(paths, true, cx))
                    .await;
                for result in results.into_iter().flatten() {
                    result.log_err();
                }
            }
        })
        .detach();
    }

    fn remove_folder_from_project(
        &mut self,
        RemoveWorktreeFromProject(worktree_id): &RemoveWorktreeFromProject,
        cx: &mut ViewContext<Self>,
    ) {
        let future = self
            .project
            .update(cx, |project, cx| project.remove_worktree(*worktree_id, cx));
        cx.foreground().spawn(future).detach();
    }

    fn project_path_for_path(
        project: ModelHandle<Project>,
        abs_path: &Path,
        visible: bool,
        cx: &mut MutableAppContext,
    ) -> Task<Result<(ModelHandle<Worktree>, ProjectPath)>> {
        let entry = project.update(cx, |project, cx| {
            project.find_or_create_local_worktree(abs_path, visible, cx)
        });
        cx.spawn(|cx| async move {
            let (worktree, path) = entry.await?;
            let worktree_id = worktree.read_with(&cx, |t, _| t.id());
            Ok((
                worktree,
                ProjectPath {
                    worktree_id,
                    path: path.into(),
                },
            ))
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

    pub fn modal<V: 'static + View>(&self) -> Option<ViewHandle<V>> {
        self.modal
            .as_ref()
            .and_then(|modal| modal.clone().downcast::<V>())
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

    pub fn save_active_item(
        &mut self,
        force_name_change: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let project = self.project.clone();
        if let Some(item) = self.active_item(cx) {
            if !force_name_change && item.can_save(cx) {
                if item.has_conflict(cx.as_ref()) {
                    const CONFLICT_MESSAGE: &str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";

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
            } else if item.is_singleton(cx) {
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

    pub fn toggle_sidebar(&mut self, sidebar_side: SidebarSide, cx: &mut ViewContext<Self>) {
        let sidebar = match sidebar_side {
            SidebarSide::Left => &mut self.left_sidebar,
            SidebarSide::Right => &mut self.right_sidebar,
        };
        let open = sidebar.update(cx, |sidebar, cx| {
            let open = !sidebar.is_open();
            sidebar.set_open(open, cx);
            open
        });

        if open {
            Dock::hide_on_sidebar_shown(self, sidebar_side, cx);
        }

        self.serialize_workspace(cx);

        cx.focus_self();
        cx.notify();
    }

    pub fn toggle_sidebar_item(&mut self, action: &ToggleSidebarItem, cx: &mut ViewContext<Self>) {
        let sidebar = match action.sidebar_side {
            SidebarSide::Left => &mut self.left_sidebar,
            SidebarSide::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, move |sidebar, cx| {
            if sidebar.is_open() && sidebar.active_item_ix() == action.item_index {
                sidebar.set_open(false, cx);
                None
            } else {
                sidebar.set_open(true, cx);
                sidebar.activate_item(action.item_index, cx);
                sidebar.active_item().cloned()
            }
        });

        if let Some(active_item) = active_item {
            Dock::hide_on_sidebar_shown(self, action.sidebar_side, cx);

            if active_item.is_focused(cx) {
                cx.focus_self();
            } else {
                cx.focus(active_item.to_any());
            }
        } else {
            cx.focus_self();
        }

        self.serialize_workspace(cx);

        cx.notify();
    }

    pub fn toggle_sidebar_item_focus(
        &mut self,
        sidebar_side: SidebarSide,
        item_index: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let sidebar = match sidebar_side {
            SidebarSide::Left => &mut self.left_sidebar,
            SidebarSide::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, |sidebar, cx| {
            sidebar.set_open(true, cx);
            sidebar.activate_item(item_index, cx);
            sidebar.active_item().cloned()
        });
        if let Some(active_item) = active_item {
            Dock::hide_on_sidebar_shown(self, sidebar_side, cx);

            if active_item.is_focused(cx) {
                cx.focus_self();
            } else {
                cx.focus(active_item.to_any());
            }
        }

        self.serialize_workspace(cx);

        cx.notify();
    }

    pub fn focus_center(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.focus_self();
        cx.notify();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane =
            cx.add_view(|cx| Pane::new(self.weak_handle().id(), None, self.background_actions, cx));
        let pane_id = pane.id();
        cx.subscribe(&pane, move |this, _, event, cx| {
            this.handle_pane_event(pane_id, event, cx)
        })
        .detach();
        self.panes.push(pane.clone());
        cx.focus(pane.clone());
        cx.emit(Event::PaneAdded(pane.clone()));
        pane
    }

    pub fn add_item_to_center(
        &mut self,
        item: Box<dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        if let Some(center_pane) = self.last_active_center_pane.clone() {
            if let Some(center_pane) = center_pane.upgrade(cx) {
                Pane::add_item(self, &center_pane, item, true, true, None, cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn add_item(&mut self, item: Box<dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        let active_pane = self.active_pane().clone();
        Pane::add_item(self, &active_pane, item, true, true, None, cx);
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakViewHandle<Pane>>,
        focus_item: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        let pane = pane.unwrap_or_else(|| {
            if !self.dock_active() {
                self.active_pane().downgrade()
            } else {
                self.last_active_center_pane.clone().unwrap_or_else(|| {
                    self.panes
                        .first()
                        .expect("There must be an active pane")
                        .downgrade()
                })
            }
        });

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
            impl 'static + FnOnce(&mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
        )>,
    > {
        let project = self.project().clone();
        let project_item = project.update(cx, |project, cx| project.open_path(path, cx));
        cx.as_mut().spawn(|mut cx| async move {
            let (project_entry_id, project_item) = project_item.await?;
            let build_item = cx.update(|cx| {
                cx.default_global::<ProjectItemBuilders>()
                    .get(&project_item.model_type())
                    .ok_or_else(|| anyhow!("no item builder for project item"))
                    .cloned()
            })?;
            let build_item =
                move |cx: &mut ViewContext<Pane>| build_item(project, project_item, cx);
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

    pub fn open_shared_screen(&mut self, action: &OpenSharedScreen, cx: &mut ViewContext<Self>) {
        if let Some(shared_screen) =
            self.shared_screen_for_peer(action.peer_id, &self.active_pane, cx)
        {
            let pane = self.active_pane.clone();
            Pane::add_item(self, &pane, Box::new(shared_screen), false, true, None, cx);
        }
    }

    pub fn activate_item(&mut self, item: &dyn ItemHandle, cx: &mut ViewContext<Self>) -> bool {
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
            cx.focus(pane);
        } else {
            self.split_pane(self.active_pane.clone(), SplitDirection::Right, cx);
        }
    }

    pub fn activate_next_pane(&mut self, cx: &mut ViewContext<Self>) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let next_ix = (ix + 1) % panes.len();
            let next_pane = panes[next_ix].clone();
            cx.focus(next_pane);
        }
    }

    pub fn activate_previous_pane(&mut self, cx: &mut ViewContext<Self>) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let prev_ix = cmp::min(ix.wrapping_sub(1), panes.len() - 1);
            let prev_pane = panes[prev_ix].clone();
            cx.focus(prev_pane);
        }
    }

    fn handle_pane_focused(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.active_pane != pane {
            self.active_pane
                .update(cx, |pane, cx| pane.set_active(false, cx));
            self.active_pane = pane.clone();
            self.active_pane
                .update(cx, |pane, cx| pane.set_active(true, cx));
            self.status_bar.update(cx, |status_bar, cx| {
                status_bar.set_active_pane(&self.active_pane, cx);
            });
            self.active_item_path_changed(cx);

            if &pane == self.dock_pane() {
                Dock::show(self, true, cx);
            } else {
                self.last_active_center_pane = Some(pane.downgrade());
                if self.dock.is_anchored_at(DockAnchor::Expanded) {
                    Dock::hide(self, cx);
                }
            }
            cx.notify();
        }

        self.update_followers(
            proto::update_followers::Variant::UpdateActiveView(proto::UpdateActiveView {
                id: self.active_item(cx).and_then(|item| {
                    item.to_followable_item_handle(cx)?
                        .remote_id(&self.client, cx)
                        .map(|id| id.to_proto())
                }),
                leader_id: self.leader_for_pane(&pane),
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
            let is_dock = &pane == self.dock.pane();
            match event {
                pane::Event::Split(direction) if !is_dock => {
                    self.split_pane(pane, *direction, cx);
                }
                pane::Event::Remove if !is_dock => self.remove_pane(pane, cx),
                pane::Event::Remove if is_dock => Dock::hide(self, cx),
                pane::Event::ActivateItem { local } => {
                    if *local {
                        self.unfollow(&pane, cx);
                    }
                    if &pane == self.active_pane() {
                        self.active_item_path_changed(cx);
                    }
                }
                pane::Event::ChangeItemTitle => {
                    if pane == self.active_pane {
                        self.active_item_path_changed(cx);
                    }
                    self.update_window_edited(cx);
                }
                pane::Event::RemoveItem { item_id } => {
                    self.update_window_edited(cx);
                    if let hash_map::Entry::Occupied(entry) = self.panes_by_item.entry(*item_id) {
                        if entry.get().id() == pane.id() {
                            entry.remove();
                        }
                    }
                }
                _ => {}
            }

            self.serialize_workspace(cx);
        } else if self.dock.visible_pane().is_none() {
            error!("pane {} not found", pane_id);
        }
    }

    pub fn split_pane(
        &mut self,
        pane: ViewHandle<Pane>,
        direction: SplitDirection,
        cx: &mut ViewContext<Self>,
    ) -> Option<ViewHandle<Pane>> {
        if &pane == self.dock_pane() {
            warn!("Can't split dock pane.");
            return None;
        }

        let item = pane.read(cx).active_item()?;
        let maybe_pane_handle =
            if let Some(clone) = item.clone_on_split(self.database_id(), cx.as_mut()) {
                let new_pane = self.add_pane(cx);
                Pane::add_item(self, &new_pane, clone, true, true, None, cx);
                self.center.split(&pane, &new_pane, direction).unwrap();
                Some(new_pane)
            } else {
                None
            };
        cx.notify();
        maybe_pane_handle
    }

    pub fn split_pane_with_item(&mut self, action: &SplitWithItem, cx: &mut ViewContext<Self>) {
        let Some(pane_to_split) = action.pane_to_split.upgrade(cx) else { return; };
        let Some(from) = action.from.upgrade(cx) else { return; };
        if &pane_to_split == self.dock_pane() {
            warn!("Can't split dock pane.");
            return;
        }

        let new_pane = self.add_pane(cx);
        Pane::move_item(
            self,
            from.clone(),
            new_pane.clone(),
            action.item_id_to_move,
            0,
            cx,
        );
        self.center
            .split(&pane_to_split, &new_pane, action.split_direction)
            .unwrap();
        cx.notify();
    }

    pub fn split_pane_with_project_entry(
        &mut self,
        action: &SplitWithProjectEntry,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let pane_to_split = action.pane_to_split.upgrade(cx)?;
        if &pane_to_split == self.dock_pane() {
            warn!("Can't split dock pane.");
            return None;
        }

        let new_pane = self.add_pane(cx);
        self.center
            .split(&pane_to_split, &new_pane, action.split_direction)
            .unwrap();

        let path = self
            .project
            .read(cx)
            .path_for_entry(action.project_entry, cx)?;
        let task = self.open_path(path, Some(new_pane.downgrade()), true, cx);
        Some(cx.foreground().spawn(async move {
            task.await?;
            Ok(())
        }))
    }

    fn remove_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.center.remove(&pane).unwrap() {
            self.panes.retain(|p| p != &pane);
            cx.focus(self.panes.last().unwrap().clone());
            self.unfollow(&pane, cx);
            self.last_leaders_by_pane.remove(&pane.downgrade());
            for removed_item in pane.read(cx).items() {
                self.panes_by_item.remove(&removed_item.id());
            }
            if self.last_active_center_pane == Some(pane.downgrade()) {
                self.last_active_center_pane = None;
            }

            cx.notify();
        } else {
            self.active_item_path_changed(cx);
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

    pub fn dock_pane(&self) -> &ViewHandle<Pane> {
        self.dock.pane()
    }

    fn dock_active(&self) -> bool {
        &self.active_pane == self.dock.pane()
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
                    item.set_leader_replica_id(None, cx);
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
            leader_id: Some(leader_id),
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
                    state.active_view_id = if let Some(active_view_id) = response.active_view_id {
                        Some(ViewId::from_proto(active_view_id)?)
                    } else {
                        None
                    };
                    Ok::<_, anyhow::Error>(())
                })?;
                Self::add_views_from_leader(
                    this.clone(),
                    leader_id,
                    vec![pane],
                    response.views,
                    &mut cx,
                )
                .await?;
                this.update(&mut cx, |this, cx| this.leader_updated(leader_id, cx));
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
            if let Some(state) = states_by_pane.remove(pane) {
                for (_, item) in state.items_by_leader_view_id {
                    item.set_leader_replica_id(None, cx);
                }

                if states_by_pane.is_empty() {
                    self.follower_states_by_leader.remove(&leader_id);
                    if let Some(project_id) = self.project.read(cx).remote_id() {
                        self.client
                            .send(proto::Unfollow {
                                project_id,
                                leader_id: Some(leader_id),
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

    pub fn is_being_followed(&self, peer_id: PeerId) -> bool {
        self.follower_states_by_leader.contains_key(&peer_id)
    }

    pub fn is_followed_by(&self, peer_id: PeerId) -> bool {
        self.leader_state.followers.contains(&peer_id)
    }

    fn render_titlebar(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        // TODO: There should be a better system in place for this
        // (https://github.com/zed-industries/zed/issues/1290)
        let is_fullscreen = cx.window_is_fullscreen(cx.window_id());
        let container_theme = if is_fullscreen {
            let mut container_theme = theme.workspace.titlebar.container;
            container_theme.padding.left = container_theme.padding.right;
            container_theme
        } else {
            theme.workspace.titlebar.container
        };

        enum TitleBar {}
        ConstrainedBox::new(
            MouseEventHandler::<TitleBar>::new(0, cx, |_, cx| {
                Container::new(
                    Stack::new()
                        .with_children(
                            self.titlebar_item
                                .as_ref()
                                .map(|item| ChildView::new(item, cx).boxed()),
                        )
                        .boxed(),
                )
                .with_style(container_theme)
                .boxed()
            })
            .on_click(MouseButton::Left, |event, cx| {
                if event.click_count == 2 {
                    cx.zoom_window(cx.window_id());
                }
            })
            .boxed(),
        )
        .with_height(theme.workspace.titlebar.height)
        .named("titlebar")
    }

    fn active_item_path_changed(&mut self, cx: &mut ViewContext<Self>) {
        let active_entry = self.active_project_path(cx);
        self.project
            .update(cx, |project, cx| project.set_active_path(active_entry, cx));
        self.update_window_title(cx);
    }

    fn update_window_title(&mut self, cx: &mut ViewContext<Self>) {
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

    fn update_window_edited(&mut self, cx: &mut ViewContext<Self>) {
        let is_edited = !self.project.read(cx).is_read_only()
            && self
                .items(cx)
                .any(|item| item.has_conflict(cx) || item.is_dirty(cx));
        if is_edited != self.window_edited {
            self.window_edited = is_edited;
            cx.set_window_edited(self.window_edited)
        }
    }

    fn render_disconnected_overlay(&self, cx: &mut RenderContext<Workspace>) -> Option<ElementBox> {
        if self.project.read(cx).is_read_only() {
            enum DisconnectedOverlay {}
            Some(
                MouseEventHandler::<DisconnectedOverlay>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    Label::new(
                        "Your connection to the remote project has been lost.",
                        theme.workspace.disconnected_overlay.text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(theme.workspace.disconnected_overlay.container)
                    .boxed()
                })
                .with_cursor_style(CursorStyle::Arrow)
                .capture_all()
                .boxed(),
            )
        } else {
            None
        }
    }

    fn render_notifications(
        &self,
        theme: &theme::Workspace,
        cx: &AppContext,
    ) -> Option<ElementBox> {
        if self.notifications.is_empty() {
            None
        } else {
            Some(
                Flex::column()
                    .with_children(self.notifications.iter().map(|(_, _, notification)| {
                        ChildView::new(notification.as_ref(), cx)
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
            let client = &this.client;
            this.leader_state
                .followers
                .insert(envelope.original_sender_id()?);

            let active_view_id = this.active_item(cx).and_then(|i| {
                Some(
                    i.to_followable_item_handle(cx)?
                        .remote_id(client, cx)?
                        .to_proto(),
                )
            });

            cx.notify();

            Ok(proto::FollowResponse {
                active_view_id,
                views: this
                    .panes()
                    .iter()
                    .flat_map(|pane| {
                        let leader_id = this.leader_for_pane(pane);
                        pane.read(cx).items().filter_map({
                            let cx = &cx;
                            move |item| {
                                let item = item.to_followable_item_handle(cx)?;
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
            })
        })
    }

    async fn handle_unfollow(
        this: ViewHandle<Self>,
        envelope: TypedEnvelope<proto::Unfollow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.leader_state
                .followers
                .remove(&envelope.original_sender_id()?);
            cx.notify();
            Ok(())
        })
    }

    async fn handle_update_followers(
        this: ViewHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        this.read_with(&cx, |this, _| {
            this.leader_updates_tx
                .unbounded_send((leader_id, envelope.payload))
        })?;
        Ok(())
    }

    async fn process_leader_update(
        this: ViewHandle<Self>,
        leader_id: PeerId,
        update: proto::UpdateFollowers,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        match update.variant.ok_or_else(|| anyhow!("invalid update"))? {
            proto::update_followers::Variant::UpdateActiveView(update_active_view) => {
                this.update(cx, |this, _| {
                    if let Some(state) = this.follower_states_by_leader.get_mut(&leader_id) {
                        for state in state.values_mut() {
                            state.active_view_id =
                                if let Some(active_view_id) = update_active_view.id.clone() {
                                    Some(ViewId::from_proto(active_view_id)?)
                                } else {
                                    None
                                };
                        }
                    }
                    anyhow::Ok(())
                })?;
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
                    if let Some(state) = this.follower_states_by_leader.get_mut(&leader_id) {
                        for state in state.values_mut() {
                            let view_id = ViewId::from_proto(id.clone())?;
                            if let Some(item) = state.items_by_leader_view_id.get(&view_id) {
                                tasks.push(item.apply_update_proto(&project, variant.clone(), cx));
                            }
                        }
                    }
                    anyhow::Ok(())
                })?;
                try_join_all(tasks).await.log_err();
            }
            proto::update_followers::Variant::CreateView(view) => {
                let panes = this.read_with(cx, |this, _| {
                    this.follower_states_by_leader
                        .get(&leader_id)
                        .into_iter()
                        .flat_map(|states_by_pane| states_by_pane.keys())
                        .cloned()
                        .collect()
                });
                Self::add_views_from_leader(this.clone(), leader_id, panes, vec![view], cx).await?;
            }
        }
        this.update(cx, |this, cx| this.leader_updated(leader_id, cx));
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
        });

        let mut item_tasks_by_pane = HashMap::default();
        for pane in panes {
            let mut item_tasks = Vec::new();
            let mut leader_view_ids = Vec::new();
            for view in &views {
                let Some(id) = &view.id else { continue };
                let id = ViewId::from_proto(id.clone())?;
                let mut variant = view.variant.clone();
                if variant.is_none() {
                    Err(anyhow!("missing variant"))?;
                }
                for build_item in &item_builders {
                    let task = cx.update(|cx| {
                        build_item(pane.clone(), project.clone(), id, &mut variant, cx)
                    });
                    if let Some(task) = task {
                        item_tasks.push(task);
                        leader_view_ids.push(id);
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
                    state.items_by_leader_view_id.insert(id, item);
                }

                Some(())
            });
        }
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
                    follower_ids: self.leader_state.followers.iter().copied().collect(),
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

    fn leader_updated(&mut self, leader_id: PeerId, cx: &mut ViewContext<Self>) -> Option<()> {
        cx.notify();

        let call = self.active_call()?;
        let room = call.read(cx).room()?.read(cx);
        let participant = room.remote_participant_for_peer_id(leader_id)?;
        let mut items_to_activate = Vec::new();
        match participant.location {
            call::ParticipantLocation::SharedProject { project_id } => {
                if Some(project_id) == self.project.read(cx).remote_id() {
                    for (pane, state) in self.follower_states_by_leader.get(&leader_id)? {
                        if let Some(item) = state
                            .active_view_id
                            .and_then(|id| state.items_by_leader_view_id.get(&id))
                        {
                            items_to_activate.push((pane.clone(), item.boxed_clone()));
                        } else {
                            if let Some(shared_screen) =
                                self.shared_screen_for_peer(leader_id, pane, cx)
                            {
                                items_to_activate.push((pane.clone(), Box::new(shared_screen)));
                            }
                        }
                    }
                }
            }
            call::ParticipantLocation::UnsharedProject => {}
            call::ParticipantLocation::External => {
                for (pane, _) in self.follower_states_by_leader.get(&leader_id)? {
                    if let Some(shared_screen) = self.shared_screen_for_peer(leader_id, pane, cx) {
                        items_to_activate.push((pane.clone(), Box::new(shared_screen)));
                    }
                }
            }
        }

        for (pane, item) in items_to_activate {
            let active_item_was_focused = pane
                .read(cx)
                .active_item()
                .map(|active_item| cx.is_child_focused(active_item.to_any()))
                .unwrap_or_default();

            if let Some(index) = pane.update(cx, |pane, _| pane.index_for_item(item.as_ref())) {
                pane.update(cx, |pane, cx| pane.activate_item(index, false, false, cx));
            } else {
                Pane::add_item(self, &pane, item.boxed_clone(), false, false, None, cx);
            }

            if active_item_was_focused {
                pane.update(cx, |pane, cx| pane.focus_active_item(cx));
            }
        }

        None
    }

    fn shared_screen_for_peer(
        &self,
        peer_id: PeerId,
        pane: &ViewHandle<Pane>,
        cx: &mut ViewContext<Self>,
    ) -> Option<ViewHandle<SharedScreen>> {
        let call = self.active_call()?;
        let room = call.read(cx).room()?.read(cx);
        let participant = room.remote_participant_for_peer_id(peer_id)?;
        let track = participant.tracks.values().next()?.clone();
        let user = participant.user.clone();

        for item in pane.read(cx).items_of_type::<SharedScreen>() {
            if item.read(cx).peer_id == peer_id {
                return Some(item);
            }
        }

        Some(cx.add_view(|cx| SharedScreen::new(&track, peer_id, user.clone(), cx)))
    }

    pub fn on_window_activation_changed(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            cx.background()
                .spawn(persistence::DB.update_timestamp(self.database_id()))
                .detach();
        } else {
            for pane in &self.panes {
                pane.update(cx, |pane, cx| {
                    if let Some(item) = pane.active_item() {
                        item.workspace_deactivated(cx);
                    }
                    if matches!(
                        cx.global::<Settings>().autosave,
                        Autosave::OnWindowChange | Autosave::OnFocusChange
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

    fn active_call(&self) -> Option<&ModelHandle<ActiveCall>> {
        self.active_call.as_ref().map(|(call, _)| call)
    }

    fn on_active_call_event(
        &mut self,
        _: ModelHandle<ActiveCall>,
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

    pub fn database_id(&self) -> WorkspaceId {
        self.database_id
    }

    fn location(&self, cx: &AppContext) -> Option<WorkspaceLocation> {
        let project = self.project().read(cx);

        if project.is_local() {
            Some(
                project
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path())
                    .collect::<Vec<_>>()
                    .into(),
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
            Member::Pane(pane) => self.remove_pane(pane.clone(), cx),
        }
    }

    fn serialize_workspace(&self, cx: &AppContext) {
        fn serialize_pane_handle(
            pane_handle: &ViewHandle<Pane>,
            cx: &AppContext,
        ) -> SerializedPane {
            let (items, active) = {
                let pane = pane_handle.read(cx);
                let active_item_id = pane.active_item().map(|item| item.id());
                (
                    pane.items()
                        .filter_map(|item_handle| {
                            Some(SerializedItem {
                                kind: Arc::from(item_handle.serialized_item_kind()?),
                                item_id: item_handle.id(),
                                active: Some(item_handle.id()) == active_item_id,
                            })
                        })
                        .collect::<Vec<_>>(),
                    pane.is_active(),
                )
            };

            SerializedPane::new(items, active)
        }

        fn build_serialized_pane_group(
            pane_group: &Member,
            cx: &AppContext,
        ) -> SerializedPaneGroup {
            match pane_group {
                Member::Axis(PaneAxis { axis, members }) => SerializedPaneGroup::Group {
                    axis: *axis,
                    children: members
                        .iter()
                        .map(|member| build_serialized_pane_group(member, cx))
                        .collect::<Vec<_>>(),
                },
                Member::Pane(pane_handle) => {
                    SerializedPaneGroup::Pane(serialize_pane_handle(&pane_handle, cx))
                }
            }
        }

        if let Some(location) = self.location(cx) {
            // Load bearing special case:
            //  - with_local_workspace() relies on this to not have other stuff open
            //    when you open your log
            if !location.paths().is_empty() {
                let dock_pane = serialize_pane_handle(self.dock.pane(), cx);
                let center_group = build_serialized_pane_group(&self.center.root, cx);

                let serialized_workspace = SerializedWorkspace {
                    id: self.database_id,
                    location,
                    dock_position: self.dock.position(),
                    dock_pane,
                    center_group,
                    left_sidebar_open: self.left_sidebar.read(cx).is_open(),
                    bounds: Default::default(),
                    display: Default::default(),
                };

                cx.background()
                    .spawn(persistence::DB.save_workspace(serialized_workspace))
                    .detach();
            }
        }
    }

    fn load_from_serialized_workspace(
        workspace: WeakViewHandle<Workspace>,
        serialized_workspace: SerializedWorkspace,
        cx: &mut MutableAppContext,
    ) {
        cx.spawn(|mut cx| async move {
            if let Some(workspace) = workspace.upgrade(&cx) {
                let (project, dock_pane_handle, old_center_pane) =
                    workspace.read_with(&cx, |workspace, _| {
                        (
                            workspace.project().clone(),
                            workspace.dock_pane().clone(),
                            workspace.last_active_center_pane.clone(),
                        )
                    });

                serialized_workspace
                    .dock_pane
                    .deserialize_to(
                        &project,
                        &dock_pane_handle,
                        serialized_workspace.id,
                        &workspace,
                        &mut cx,
                    )
                    .await;

                // Traverse the splits tree and add to things
                let center_group = serialized_workspace
                    .center_group
                    .deserialize(&project, serialized_workspace.id, &workspace, &mut cx)
                    .await;

                // Remove old panes from workspace panes list
                workspace.update(&mut cx, |workspace, cx| {
                    if let Some((center_group, active_pane)) = center_group {
                        workspace.remove_panes(workspace.center.root.clone(), cx);

                        // Swap workspace center group
                        workspace.center = PaneGroup::with_root(center_group);

                        // Change the focus to the workspace first so that we retrigger focus in on the pane.
                        cx.focus_self();

                        if let Some(active_pane) = active_pane {
                            cx.focus(active_pane);
                        } else {
                            cx.focus(workspace.panes.last().unwrap().clone());
                        }
                    } else {
                        let old_center_handle = old_center_pane.and_then(|weak| weak.upgrade(cx));
                        if let Some(old_center_handle) = old_center_handle {
                            cx.focus(old_center_handle)
                        } else {
                            cx.focus_self()
                        }
                    }

                    if workspace.left_sidebar().read(cx).is_open()
                        != serialized_workspace.left_sidebar_open
                    {
                        workspace.toggle_sidebar(SidebarSide::Left, cx);
                    }

                    // Note that without after_window, the focus_self() and
                    // the focus the dock generates start generating alternating
                    // focus due to the deferred execution each triggering each other
                    cx.after_window_update(move |workspace, cx| {
                        Dock::set_dock_position(
                            workspace,
                            serialized_workspace.dock_position,
                            true,
                            cx,
                        );
                    });

                    cx.notify();
                });

                // Serialize ourself to make sure our timestamps and any pane / item changes are replicated
                workspace.read_with(&cx, |workspace, cx| workspace.serialize_workspace(cx))
            }
        })
        .detach();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        Self::new(None, 0, project, |_, _| None, || &[], cx)
    }
}

fn notify_if_database_failed(workspace: &ViewHandle<Workspace>, cx: &mut AsyncAppContext) {
    if (*db::ALL_FILE_DB_FAILED).load(std::sync::atomic::Ordering::Acquire) {
        workspace.update(cx, |workspace, cx| {
            workspace.show_notification_once(0, cx, |cx| {
                cx.add_view(|_| {
                    MessageNotification::new(
                        indoc::indoc! {"
                            Failed to load any database file :(
                        "},
                        OsOpen("https://github.com/zed-industries/community/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml".to_string()),
                        "Click to let us know about this error"
                    )
                })
            });
        });
    } else {
        let backup_path = (*db::BACKUP_DB_PATH).read();
        if let Some(backup_path) = &*backup_path {
            workspace.update(cx, |workspace, cx| {
                workspace.show_notification_once(0, cx, |cx| {
                    cx.add_view(|_| {
                        let backup_path = backup_path.to_string_lossy();
                        MessageNotification::new(
                            format!(
                                indoc::indoc! {"
                                Database file was corrupted :(
                                Old database backed up to:
                                {}
                                "},
                                backup_path
                            ),
                            OsOpen(backup_path.to_string()),
                            "Click to show old database in finder",
                        )
                    })
                });
            });
        }
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
                                let project = self.project.clone();
                                Flex::row()
                                    .with_children(
                                        if self.left_sidebar.read(cx).active_item().is_some() {
                                            Some(
                                                ChildView::new(&self.left_sidebar, cx)
                                                    .constrained()
                                                    .dynamically(|constraint, cx| {
                                                        SizeConstraint::new(
                                                            Vector2F::new(20., constraint.min.y()),
                                                            Vector2F::new(
                                                                cx.window_size.x() * 0.8,
                                                                constraint.max.y(),
                                                            ),
                                                        )
                                                    })
                                                    .boxed(),
                                            )
                                        } else {
                                            None
                                        },
                                    )
                                    .with_child(
                                        FlexItem::new(
                                            Flex::column()
                                                .with_child(
                                                    FlexItem::new(self.center.render(
                                                        &project,
                                                        &theme,
                                                        &self.follower_states_by_leader,
                                                        self.active_call(),
                                                        self.active_pane(),
                                                        cx,
                                                    ))
                                                    .flex(1., true)
                                                    .boxed(),
                                                )
                                                .with_children(self.dock.render(
                                                    &theme,
                                                    DockAnchor::Bottom,
                                                    cx,
                                                ))
                                                .boxed(),
                                        )
                                        .flex(1., true)
                                        .boxed(),
                                    )
                                    .with_children(self.dock.render(&theme, DockAnchor::Right, cx))
                                    .with_children(
                                        if self.right_sidebar.read(cx).active_item().is_some() {
                                            Some(
                                                ChildView::new(&self.right_sidebar, cx)
                                                    .constrained()
                                                    .dynamically(|constraint, cx| {
                                                        SizeConstraint::new(
                                                            Vector2F::new(20., constraint.min.y()),
                                                            Vector2F::new(
                                                                cx.window_size.x() * 0.8,
                                                                constraint.max.y(),
                                                            ),
                                                        )
                                                    })
                                                    .boxed(),
                                            )
                                        } else {
                                            None
                                        },
                                    )
                                    .boxed()
                            })
                            .with_child(
                                Overlay::new(
                                    Stack::new()
                                        .with_children(self.dock.render(
                                            &theme,
                                            DockAnchor::Expanded,
                                            cx,
                                        ))
                                        .with_children(self.modal.as_ref().map(|modal| {
                                            ChildView::new(modal, cx)
                                                .contained()
                                                .with_style(theme.workspace.modal)
                                                .aligned()
                                                .top()
                                                .boxed()
                                        }))
                                        .with_children(
                                            self.render_notifications(&theme.workspace, cx),
                                        )
                                        .boxed(),
                                )
                                .boxed(),
                            )
                            .flex(1.0, true)
                            .boxed(),
                    )
                    .with_child(ChildView::new(&self.status_bar, cx).boxed())
                    .contained()
                    .with_background_color(theme.workspace.background)
                    .boxed(),
            )
            .with_children(DragAndDrop::render(cx))
            .with_children(self.render_disconnected_overlay(cx))
            .named("workspace")
    }

    fn focus_in(&mut self, view: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.active_pane);
        } else {
            for pane in self.panes() {
                let view = view.clone();
                if pane.update(cx, |_, cx| view.id() == cx.view_id() || cx.is_child(view)) {
                    self.handle_pane_focused(pane.clone(), cx);
                    break;
                }
            }
        }
    }

    fn keymap_context(&self, _: &AppContext) -> KeymapContext {
        Self::default_keymap_context()
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

impl std::fmt::Debug for OpenPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenPaths")
            .field("paths", &self.paths)
            .finish()
    }
}

pub struct WorkspaceCreated(WeakViewHandle<Workspace>);

pub fn activate_workspace_for_project(
    cx: &mut MutableAppContext,
    predicate: impl Fn(&mut Project, &mut ModelContext<Project>) -> bool,
) -> Option<ViewHandle<Workspace>> {
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(workspace_handle) = cx.root_view::<Workspace>(window_id) {
            let project = workspace_handle.read(cx).project.clone();
            if project.update(cx, &predicate) {
                cx.activate_window(window_id);
                return Some(workspace_handle);
            }
        }
    }
    None
}

pub async fn last_opened_workspace_paths() -> Option<WorkspaceLocation> {
    DB.last_workspace().await.log_err().flatten()
}

#[allow(clippy::type_complexity)]
pub fn open_paths(
    abs_paths: &[PathBuf],
    app_state: &Arc<AppState>,
    requesting_window_id: Option<usize>,
    cx: &mut MutableAppContext,
) -> Task<(
    ViewHandle<Workspace>,
    Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>,
)> {
    log::info!("open paths {:?}", abs_paths);

    // Open paths in existing workspace if possible
    let existing =
        activate_workspace_for_project(cx, |project, cx| project.contains_paths(abs_paths, cx));

    let app_state = app_state.clone();
    let abs_paths = abs_paths.to_vec();
    cx.spawn(|mut cx| async move {
        if let Some(existing) = existing {
            (
                existing.clone(),
                existing
                    .update(&mut cx, |workspace, cx| {
                        workspace.open_paths(abs_paths, true, cx)
                    })
                    .await,
            )
        } else {
            let contains_directory =
                futures::future::join_all(abs_paths.iter().map(|path| app_state.fs.is_file(path)))
                    .await
                    .contains(&false);

            cx.update(|cx| {
                let task =
                    Workspace::new_local(abs_paths, app_state.clone(), requesting_window_id, cx);

                cx.spawn(|mut cx| async move {
                    let (workspace, items) = task.await;

                    workspace.update(&mut cx, |workspace, cx| {
                        if contains_directory {
                            workspace.toggle_sidebar(SidebarSide::Left, cx);
                        }
                    });

                    (workspace, items)
                })
            })
            .await
        }
    })
}

pub fn open_new(
    app_state: &Arc<AppState>,
    cx: &mut MutableAppContext,
    init: impl FnOnce(&mut Workspace, &mut ViewContext<Workspace>) + 'static,
) -> Task<()> {
    let task = Workspace::new_local(Vec::new(), app_state.clone(), None, cx);
    cx.spawn(|mut cx| async move {
        let (workspace, opened_paths) = task.await;

        workspace.update(&mut cx, |workspace, cx| {
            if opened_paths.is_empty() {
                init(workspace, cx)
            }
        })
    })
}

fn parse_pixel_position_env_var(value: &str) -> Option<Vector2F> {
    let mut parts = value.split(',');
    let width: usize = parts.next()?.parse().ok()?;
    let height: usize = parts.next()?.parse().ok()?;
    Some(vec2f(width as f32, height as f32))
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::item::test::{TestItem, TestItemEvent, TestProjectItem};

    use super::*;
    use fs::FakeFs;
    use gpui::{executor::Deterministic, TestAppContext};
    use project::{Project, ProjectEntryId};
    use serde_json::json;

    #[gpui::test]
    async fn test_tab_disambiguation(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);

        let fs = FakeFs::new(cx.background());
        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| None,
                || &[],
                cx,
            )
        });

        // Adding an item with no ambiguity renders the tab without detail.
        let item1 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.tab_descriptions = Some(vec!["c", "b1/c", "a/b1/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item1.clone()), cx);
        });
        item1.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), None));

        // Adding an item that creates ambiguity increases the level of detail on
        // both tabs.
        let item2 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item2.clone()), cx);
        });
        item1.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));
        item2.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));

        // Adding an item that creates ambiguity increases the level of detail only
        // on the ambiguous tabs. In this case, the ambiguity can't be resolved so
        // we stop at the highest detail available.
        let item3 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.tab_descriptions = Some(vec!["c", "b2/c", "a/b2/c"]);
            item
        });
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item3.clone()), cx);
        });
        item1.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(1)));
        item2.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(3)));
        item3.read_with(cx, |item, _| assert_eq!(item.tab_detail.get(), Some(3)));
    }

    #[gpui::test]
    async fn test_tracking_active_path(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());
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
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| None,
                || &[],
                cx,
            )
        });
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let item1 = cx.add_view(&workspace, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
        });
        let item2 = cx.add_view(&workspace, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(2, "two.txt", cx)])
        });

        // Add an item to an empty pane
        workspace.update(cx, |workspace, cx| workspace.add_item(Box::new(item1), cx));
        project.read_with(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "one.txt").into(), cx)
                    .map(|e| e.id)
            );
        });
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("one.txt — root1")
        );

        // Add a second item to a non-empty pane
        workspace.update(cx, |workspace, cx| workspace.add_item(Box::new(item2), cx));
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("two.txt — root1")
        );
        project.read_with(cx, |project, cx| {
            assert_eq!(
                project.active_entry(),
                project
                    .entry_for_path(&(worktree_id, "two.txt").into(), cx)
                    .map(|e| e.id)
            );
        });

        // Close the active item
        workspace
            .update(cx, |workspace, cx| {
                Pane::close_active_item(workspace, &Default::default(), cx).unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("one.txt — root1")
        );
        project.read_with(cx, |project, cx| {
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
                project.find_or_create_local_worktree("/root2", true, cx)
            })
            .await
            .unwrap();
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("one.txt — root1, root2")
        );

        // Remove a project folder
        project
            .update(cx, |project, cx| project.remove_worktree(worktree_id, cx))
            .await;
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("one.txt — root2")
        );
    }

    #[gpui::test]
    async fn test_close_window(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/root", json!({ "one": "" })).await;

        let project = Project::test(fs, ["root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| None,
                || &[],
                cx,
            )
        });

        // When there are no dirty items, there's nothing to do.
        let item1 = cx.add_view(&workspace, |_| TestItem::new());
        workspace.update(cx, |w, cx| w.add_item(Box::new(item1.clone()), cx));
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(false, cx));
        assert!(task.await.unwrap());

        // When there are dirty untitled items, prompt to save each one. If the user
        // cancels any prompt, then abort.
        let item2 = cx.add_view(&workspace, |_| TestItem::new().with_dirty(true));
        let item3 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        workspace.update(cx, |w, cx| {
            w.add_item(Box::new(item2.clone()), cx);
            w.add_item(Box::new(item3.clone()), cx);
        });
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(false, cx));
        cx.foreground().run_until_parked();
        cx.simulate_prompt_answer(window_id, 2 /* cancel */);
        cx.foreground().run_until_parked();
        assert!(!cx.has_pending_prompt(window_id));
        assert!(!task.await.unwrap());
    }

    #[gpui::test]
    async fn test_close_pane_items(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| None, || &[], cx)
        });

        let item1 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item2 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)])
        });
        let item3 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(3, "3.txt", cx)])
        });
        let item4 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new_untitled(cx)])
        });
        let pane = workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item1.clone()), cx);
            workspace.add_item(Box::new(item2.clone()), cx);
            workspace.add_item(Box::new(item3.clone()), cx);
            workspace.add_item(Box::new(item4.clone()), cx);
            workspace.active_pane().clone()
        });

        let close_items = workspace.update(cx, |workspace, cx| {
            pane.update(cx, |pane, cx| {
                pane.activate_item(1, true, true, cx);
                assert_eq!(pane.active_item().unwrap().id(), item2.id());
            });

            let item1_id = item1.id();
            let item3_id = item3.id();
            let item4_id = item4.id();
            Pane::close_items(workspace, pane.clone(), cx, move |id| {
                [item1_id, item3_id, item4_id].contains(&id)
            })
        });
        cx.foreground().run_until_parked();

        // There's a prompt to save item 1.
        pane.read_with(cx, |pane, _| {
            assert_eq!(pane.items_len(), 4);
            assert_eq!(pane.active_item().unwrap().id(), item1.id());
        });
        assert!(cx.has_pending_prompt(window_id));

        // Confirm saving item 1.
        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();

        // Item 1 is saved. There's a prompt to save item 3.
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item1.read(cx).save_count, 1);
            assert_eq!(item1.read(cx).save_as_count, 0);
            assert_eq!(item1.read(cx).reload_count, 0);
            assert_eq!(pane.items_len(), 3);
            assert_eq!(pane.active_item().unwrap().id(), item3.id());
        });
        assert!(cx.has_pending_prompt(window_id));

        // Cancel saving item 3.
        cx.simulate_prompt_answer(window_id, 1);
        cx.foreground().run_until_parked();

        // Item 3 is reloaded. There's a prompt to save item 4.
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item3.read(cx).save_count, 0);
            assert_eq!(item3.read(cx).save_as_count, 0);
            assert_eq!(item3.read(cx).reload_count, 1);
            assert_eq!(pane.items_len(), 2);
            assert_eq!(pane.active_item().unwrap().id(), item4.id());
        });
        assert!(cx.has_pending_prompt(window_id));

        // Confirm saving item 4.
        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();

        // There's a prompt for a path for item 4.
        cx.simulate_new_path_selection(|_| Some(Default::default()));
        close_items.await.unwrap();

        // The requested items are closed.
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item4.read(cx).save_count, 0);
            assert_eq!(item4.read(cx).save_as_count, 1);
            assert_eq!(item4.read(cx).reload_count, 0);
            assert_eq!(pane.items_len(), 1);
            assert_eq!(pane.active_item().unwrap().id(), item2.id());
        });
    }

    #[gpui::test]
    async fn test_prompting_to_save_only_on_last_item_for_entry(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| None, || &[], cx)
        });

        // Create several workspace items with single project entries, and two
        // workspace items with multiple project entries.
        let single_entry_items = (0..=4)
            .map(|project_entry_id| {
                cx.add_view(&workspace, |cx| {
                    TestItem::new()
                        .with_dirty(true)
                        .with_project_items(&[TestProjectItem::new(
                            project_entry_id,
                            &format!("{project_entry_id}.txt"),
                            cx,
                        )])
                })
            })
            .collect::<Vec<_>>();
        let item_2_3 = cx.add_view(&workspace, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_singleton(false)
                .with_project_items(&[
                    single_entry_items[2].read(cx).project_items[0].clone(),
                    single_entry_items[3].read(cx).project_items[0].clone(),
                ])
        });
        let item_3_4 = cx.add_view(&workspace, |cx| {
            TestItem::new()
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
            workspace.add_item(Box::new(item_2_3.clone()), cx);
            for item in single_entry_items {
                workspace.add_item(Box::new(item), cx);
            }
            left_pane.update(cx, |pane, cx| {
                pane.activate_item(2, true, true, cx);
            });

            workspace
                .split_pane(left_pane.clone(), SplitDirection::Right, cx)
                .unwrap();

            left_pane
        });

        //Need to cause an effect flush in order to respect new focus
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item_3_4.clone()), cx);
            cx.focus(left_pane.clone());
        });

        // When closing all of the items in the left pane, we should be prompted twice:
        // once for project entry 0, and once for project entry 2. After those two
        // prompts, the task should complete.

        let close = workspace.update(cx, |workspace, cx| {
            Pane::close_items(workspace, left_pane.clone(), cx, |_| true)
        });

        cx.foreground().run_until_parked();
        left_pane.read_with(cx, |pane, cx| {
            assert_eq!(
                pane.active_item().unwrap().project_entry_ids(cx).as_slice(),
                &[ProjectEntryId::from_proto(0)]
            );
        });
        cx.simulate_prompt_answer(window_id, 0);

        cx.foreground().run_until_parked();
        left_pane.read_with(cx, |pane, cx| {
            assert_eq!(
                pane.active_item().unwrap().project_entry_ids(cx).as_slice(),
                &[ProjectEntryId::from_proto(2)]
            );
        });
        cx.simulate_prompt_answer(window_id, 0);

        cx.foreground().run_until_parked();
        close.await.unwrap();
        left_pane.read_with(cx, |pane, _| {
            assert_eq!(pane.items_len(), 0);
        });
    }

    #[gpui::test]
    async fn test_autosave(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
        deterministic.forbid_parking();

        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| None, || &[], cx)
        });

        let item = cx.add_view(&workspace, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item_id = item.id();
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item.clone()), cx);
        });

        // Autosave on window change.
        item.update(cx, |item, cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.autosave = Autosave::OnWindowChange;
            });
            item.is_dirty = true;
        });

        // Deactivating the window saves the file.
        cx.simulate_window_activation(None);
        deterministic.run_until_parked();
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 1));

        // Autosave on focus change.
        item.update(cx, |item, cx| {
            cx.focus_self();
            cx.update_global(|settings: &mut Settings, _| {
                settings.autosave = Autosave::OnFocusChange;
            });
            item.is_dirty = true;
        });

        // Blurring the item saves the file.
        item.update(cx, |_, cx| cx.blur());
        deterministic.run_until_parked();
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 2));

        // Deactivating the window still saves the file.
        cx.simulate_window_activation(Some(window_id));
        item.update(cx, |item, cx| {
            cx.focus_self();
            item.is_dirty = true;
        });
        cx.simulate_window_activation(None);

        deterministic.run_until_parked();
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 3));

        // Autosave after delay.
        item.update(cx, |item, cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.autosave = Autosave::AfterDelay { milliseconds: 500 };
            });
            item.is_dirty = true;
            cx.emit(TestItemEvent::Edit);
        });

        // Delay hasn't fully expired, so the file is still dirty and unsaved.
        deterministic.advance_clock(Duration::from_millis(250));
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 3));

        // After delay expires, the file is saved.
        deterministic.advance_clock(Duration::from_millis(250));
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 4));

        // Autosave on focus change, ensuring closing the tab counts as such.
        item.update(cx, |item, cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.autosave = Autosave::OnFocusChange;
            });
            item.is_dirty = true;
        });

        workspace
            .update(cx, |workspace, cx| {
                let pane = workspace.active_pane().clone();
                Pane::close_items(workspace, pane, cx, move |id| id == item_id)
            })
            .await
            .unwrap();
        assert!(!cx.has_pending_prompt(window_id));
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 5));

        // Add the item again, ensuring autosave is prevented if the underlying file has been deleted.
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item.clone()), cx);
        });
        item.update(cx, |item, cx| {
            item.project_items[0].update(cx, |item, _| {
                item.entry_id = None;
            });
            item.is_dirty = true;
            cx.blur();
        });
        deterministic.run_until_parked();
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 5));

        // Ensure autosave is prevented for deleted files also when closing the buffer.
        let _close_items = workspace.update(cx, |workspace, cx| {
            let pane = workspace.active_pane().clone();
            Pane::close_items(workspace, pane, cx, move |id| id == item_id)
        });
        deterministic.run_until_parked();
        assert!(cx.has_pending_prompt(window_id));
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 5));
    }

    #[gpui::test]
    async fn test_pane_navigation(
        deterministic: Arc<Deterministic>,
        cx: &mut gpui::TestAppContext,
    ) {
        deterministic.forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| None, || &[], cx)
        });

        let item = cx.add_view(&workspace, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        let toolbar = pane.read_with(cx, |pane, _| pane.toolbar().clone());
        let toolbar_notify_count = Rc::new(RefCell::new(0));

        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item.clone()), cx);
            let toolbar_notification_count = toolbar_notify_count.clone();
            cx.observe(&toolbar, move |_, _, _| {
                *toolbar_notification_count.borrow_mut() += 1
            })
            .detach();
        });

        pane.read_with(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        item.update(cx, |item, cx| {
            item.set_state("one".to_string(), cx);
        });

        // Toolbar must be notified to re-render the navigation buttons
        assert_eq!(*toolbar_notify_count.borrow(), 1);

        pane.read_with(cx, |pane, _| {
            assert!(pane.can_navigate_backward());
            assert!(!pane.can_navigate_forward());
        });

        workspace
            .update(cx, |workspace, cx| {
                Pane::go_back(workspace, Some(pane.clone()), cx)
            })
            .await;

        assert_eq!(*toolbar_notify_count.borrow(), 3);
        pane.read_with(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(pane.can_navigate_forward());
        });
    }
}
