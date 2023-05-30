pub mod dock;
/// NOTE: Focus only 'takes' after an update has flushed_effects.
///
/// This may cause issues when you're trying to write tests that use workspace focus to add items at
/// specific locations.
pub mod item;
pub mod notifications;
pub mod pane;
pub mod pane_group;
mod persistence;
pub mod searchable;
pub mod shared_screen;
mod status_bar;
mod toolbar;
mod workspace_settings;

use anyhow::{anyhow, Context, Result};
use assets::Assets;
use call::ActiveCall;
use client::{
    proto::{self, PeerId},
    Client, TypedEnvelope, UserStore,
};
use collections::{hash_map, HashMap, HashSet};
use drag_and_drop::DragAndDrop;
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
    impl_actions,
    platform::{
        CursorStyle, MouseButton, PathPromptOptions, Platform, PromptLevel, WindowBounds,
        WindowOptions,
    },
    AnyModelHandle, AnyViewHandle, AnyWeakViewHandle, AppContext, AsyncAppContext, Entity,
    ModelContext, ModelHandle, SizeConstraint, Subscription, Task, View, ViewContext, ViewHandle,
    WeakViewHandle, WindowContext,
};
use item::{FollowableItem, FollowableItemHandle, Item, ItemHandle, ProjectItem};
use itertools::Itertools;
use language::{LanguageRegistry, Rope};
use std::{
    any::TypeId,
    borrow::Cow,
    cmp, env,
    future::Future,
    path::{Path, PathBuf},
    rc::Rc,
    str,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

use crate::{
    notifications::simple_message_notification::MessageNotification,
    persistence::model::{
        DockData, DockStructure, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
    },
};
use dock::{Dock, DockPosition, Panel, PanelButtons, PanelHandle};
use lazy_static::lazy_static;
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
use shared_screen::SharedScreen;
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use theme::Theme;
pub use toolbar::{ToolbarItemLocation, ToolbarItemView};
use util::{async_iife, paths, ResultExt};
pub use workspace_settings::{AutosaveSetting, GitGutterSetting, WorkspaceSettings};

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

pub trait Modal: View {
    fn dismiss_on_event(event: &Self::Event) -> bool;
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
        NewTerminal,
        NewCenterTerminal,
        ToggleTerminalFocus,
        NewSearch,
        Feedback,
        Restart,
        Welcome,
        ToggleZoom,
        ToggleLeftDock,
        ToggleRightDock,
        ToggleBottomDock,
    ]
);

actions!(zed, [OpenSettings]);

#[derive(Clone, PartialEq)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePane(pub usize);

pub struct Toast {
    id: usize,
    msg: Cow<'static, str>,
    on_click: Option<(Cow<'static, str>, Arc<dyn Fn(&mut WindowContext)>)>,
}

impl Toast {
    pub fn new<I: Into<Cow<'static, str>>>(id: usize, msg: I) -> Self {
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
            id: self.id,
            msg: self.msg.to_owned(),
            on_click: self.on_click.clone(),
        }
    }
}

pub type WorkspaceId = i64;

impl_actions!(workspace, [ActivatePane]);

pub fn init_settings(cx: &mut AppContext) {
    settings::register::<WorkspaceSettings>(cx);
}

pub fn init(app_state: Arc<AppState>, cx: &mut AppContext) {
    init_settings(cx);
    pane::init(cx);
    notifications::init(cx);

    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &Open, cx: &mut AppContext| {
            let mut paths = cx.prompt_for_paths(PathPromptOptions {
                files: true,
                directories: true,
                multiple: true,
            });

            if let Some(app_state) = app_state.upgrade() {
                cx.spawn(move |mut cx| async move {
                    if let Some(paths) = paths.recv().await.flatten() {
                        cx.update(|cx| {
                            open_paths(&paths, &app_state, None, cx).detach_and_log_err(cx)
                        });
                    }
                })
                .detach();
            }
        }
    });
    cx.add_async_action(Workspace::open);

    cx.add_async_action(Workspace::follow_next_collaborator);
    cx.add_async_action(Workspace::close);
    cx.add_global_action(Workspace::close_global);
    cx.add_global_action(restart);
    cx.add_async_action(Workspace::save_all);
    cx.add_action(Workspace::add_folder_to_project);
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
    cx.add_action(|workspace: &mut Workspace, _: &ActivatePreviousPane, cx| {
        workspace.activate_previous_pane(cx)
    });
    cx.add_action(|workspace: &mut Workspace, _: &ActivateNextPane, cx| {
        workspace.activate_next_pane(cx)
    });
    cx.add_action(|workspace: &mut Workspace, _: &ToggleLeftDock, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
    });
    cx.add_action(|workspace: &mut Workspace, _: &ToggleRightDock, cx| {
        workspace.toggle_dock(DockPosition::Right, cx);
    });
    cx.add_action(|workspace: &mut Workspace, _: &ToggleBottomDock, cx| {
        workspace.toggle_dock(DockPosition::Bottom, cx);
    });
    cx.add_action(Workspace::activate_pane_at_index);

    cx.add_action(|_: &mut Workspace, _: &install_cli::Install, cx| {
        cx.spawn(|workspace, mut cx| async move {
            let err = install_cli::install_cli(&cx)
                .await
                .context("Failed to create CLI symlink");

            workspace.update(&mut cx, |workspace, cx| {
                if matches!(err, Err(_)) {
                    err.notify_err(workspace, cx);
                } else {
                    workspace.show_notification(1, cx, |cx| {
                        cx.add_view(|_| {
                            MessageNotification::new("Successfully installed the `zed` binary")
                        })
                    });
                }
            })
        })
        .detach();
    });

    cx.add_action(
        move |_: &mut Workspace, _: &OpenSettings, cx: &mut ViewContext<Workspace>| {
            create_and_open_local_file(&paths::SETTINGS, cx, || {
                settings::initial_user_settings_content(&Assets)
                    .as_ref()
                    .into()
            })
            .detach_and_log_err(cx);
        },
    );

    let client = &app_state.client;
    client.add_view_request_handler(Workspace::handle_follow);
    client.add_view_message_handler(Workspace::handle_unfollow);
    client.add_view_message_handler(Workspace::handle_update_followers);
}

type ProjectItemBuilders = HashMap<
    TypeId,
    fn(ModelHandle<Project>, AnyModelHandle, &mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
>;
pub fn register_project_item<I: ProjectItem>(cx: &mut AppContext) {
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
    &mut AppContext,
) -> Option<Task<Result<Box<dyn FollowableItemHandle>>>>;
type FollowableItemBuilders = HashMap<
    TypeId,
    (
        FollowableItemBuilder,
        fn(&AnyViewHandle) -> Box<dyn FollowableItemHandle>,
    ),
>;
pub fn register_followable_item<I: FollowableItem>(cx: &mut AppContext) {
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
                |this| Box::new(this.clone().downcast::<I>().unwrap()),
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
pub fn register_deserializable_item<I: Item>(cx: &mut AppContext) {
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
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub build_window_options:
        fn(Option<WindowBounds>, Option<uuid::Uuid>, &dyn Platform) -> WindowOptions<'static>,
    pub initialize_workspace:
        fn(WeakViewHandle<Workspace>, bool, Arc<AppState>, AsyncAppContext) -> Task<Result<()>>,
    pub background_actions: BackgroundActions,
}

impl AppState {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut AppContext) -> Arc<Self> {
        use settings::SettingsStore;

        if !cx.has_global::<SettingsStore>() {
            cx.set_global(SettingsStore::test(cx));
        }

        let fs = fs::FakeFs::new(cx.background().clone());
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = util::http::FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone(), cx);
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        theme::init((), cx);
        client::init(&client, cx);
        crate::init_settings(cx);

        Arc::new(Self {
            client,
            fs,
            languages,
            user_store,
            initialize_workspace: |_, _, _, _| Task::ready(Ok(())),
            build_window_options: |_, _, _| Default::default(),
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

    fn fire_new<F>(&mut self, delay: Duration, cx: &mut ViewContext<Workspace>, f: F)
    where
        F: 'static + FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> Task<Result<()>>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            _ = channel.send(());
        }

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        let previous_task = self.task.take();
        self.task = Some(cx.spawn(|workspace, mut cx| async move {
            let mut timer = cx.background().timer(delay).fuse();
            if let Some(previous_task) = previous_task {
                previous_task.await;
            }

            futures::select_biased! {
                _ = receiver => return,
                    _ = timer => {}
            }

            if let Some(result) = workspace
                .update(&mut cx, |workspace, cx| (f)(workspace, cx))
                .log_err()
            {
                result.await.log_err();
            }
        }));
    }
}

pub enum Event {
    PaneAdded(ViewHandle<Pane>),
    ContactRequestedJoin(u64),
}

pub struct Workspace {
    weak_self: WeakViewHandle<Self>,
    remote_entity_subscription: Option<client::Subscription>,
    modal: Option<AnyViewHandle>,
    zoomed: Option<AnyWeakViewHandle>,
    zoomed_position: Option<DockPosition>,
    center: PaneGroup,
    left_dock: ViewHandle<Dock>,
    bottom_dock: ViewHandle<Dock>,
    right_dock: ViewHandle<Dock>,
    panes: Vec<ViewHandle<Pane>>,
    panes_by_item: HashMap<usize, WeakViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    last_active_center_pane: Option<WeakViewHandle<Pane>>,
    status_bar: ViewHandle<StatusBar>,
    titlebar_item: Option<AnyViewHandle>,
    notifications: Vec<(TypeId, usize, Box<dyn NotificationHandle>)>,
    project: ModelHandle<Project>,
    leader_state: LeaderState,
    follower_states_by_leader: FollowerStatesByLeader,
    last_leaders_by_pane: HashMap<WeakViewHandle<Pane>, PeerId>,
    window_edited: bool,
    active_call: Option<(ModelHandle<ActiveCall>, Vec<gpui::Subscription>)>,
    leader_updates_tx: mpsc::UnboundedSender<(PeerId, proto::UpdateFollowers)>,
    database_id: WorkspaceId,
    app_state: Arc<AppState>,
    subscriptions: Vec<Subscription>,
    _apply_leader_updates: Task<Result<()>>,
    _observe_current_user: Task<Result<()>>,
    pane_history_timestamp: Arc<AtomicUsize>,
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
        workspace_id: WorkspaceId,
        project: ModelHandle<Project>,
        app_state: Arc<AppState>,
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

                _ => {}
            }
            cx.notify()
        })
        .detach();

        let weak_handle = cx.weak_handle();
        let pane_history_timestamp = Arc::new(AtomicUsize::new(0));

        let center_pane = cx.add_view(|cx| {
            Pane::new(
                weak_handle.clone(),
                app_state.background_actions,
                pane_history_timestamp.clone(),
                cx,
            )
        });
        cx.subscribe(&center_pane, Self::handle_pane_event).detach();
        cx.focus(&center_pane);
        cx.emit(Event::PaneAdded(center_pane.clone()));

        let mut current_user = app_state.user_store.read(cx).watch_current_user();
        let mut connection_status = app_state.client.status();
        let _observe_current_user = cx.spawn(|this, mut cx| async move {
            current_user.recv().await;
            connection_status.recv().await;
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

        cx.emit_global(WorkspaceCreated(weak_handle.clone()));

        let left_dock = cx.add_view(|_| Dock::new(DockPosition::Left));
        let bottom_dock = cx.add_view(|_| Dock::new(DockPosition::Bottom));
        let right_dock = cx.add_view(|_| Dock::new(DockPosition::Right));
        let left_dock_buttons =
            cx.add_view(|cx| PanelButtons::new(left_dock.clone(), weak_handle.clone(), cx));
        let bottom_dock_buttons =
            cx.add_view(|cx| PanelButtons::new(bottom_dock.clone(), weak_handle.clone(), cx));
        let right_dock_buttons =
            cx.add_view(|cx| PanelButtons::new(right_dock.clone(), weak_handle.clone(), cx));
        let status_bar = cx.add_view(|cx| {
            let mut status_bar = StatusBar::new(&center_pane.clone(), cx);
            status_bar.add_left_item(left_dock_buttons, cx);
            status_bar.add_right_item(right_dock_buttons, cx);
            status_bar.add_right_item(bottom_dock_buttons, cx);
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

        let subscriptions = vec![
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
        ];

        let mut this = Workspace {
            weak_self: weak_handle.clone(),
            modal: None,
            zoomed: None,
            zoomed_position: None,
            center: PaneGroup::new(center_pane.clone()),
            panes: vec![center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane.clone(),
            last_active_center_pane: Some(center_pane.downgrade()),
            status_bar,
            titlebar_item: None,
            notifications: Default::default(),
            remote_entity_subscription: None,
            left_dock,
            bottom_dock,
            right_dock,
            project: project.clone(),
            leader_state: Default::default(),
            follower_states_by_leader: Default::default(),
            last_leaders_by_pane: Default::default(),
            window_edited: false,
            active_call,
            database_id: workspace_id,
            app_state,
            _observe_current_user,
            _apply_leader_updates,
            leader_updates_tx,
            subscriptions,
            pane_history_timestamp,
        };
        this.project_remote_id_changed(project.read(cx).remote_id(), cx);
        cx.defer(|this, cx| this.update_window_title(cx));
        this
    }

    fn new_local(
        abs_paths: Vec<PathBuf>,
        app_state: Arc<AppState>,
        requesting_window_id: Option<usize>,
        cx: &mut AppContext,
    ) -> Task<(
        WeakViewHandle<Workspace>,
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

            let paths_to_open = Arc::new(abs_paths);

            // Get project paths for all of the abs_paths
            let mut worktree_roots: HashSet<Arc<Path>> = Default::default();
            let mut project_paths: Vec<(PathBuf, Option<ProjectPath>)> =
                Vec::with_capacity(paths_to_open.len());
            for path in paths_to_open.iter().cloned() {
                if let Some((worktree, project_entry)) = cx
                    .update(|cx| {
                        Workspace::project_path_for_path(project_handle.clone(), &path, true, cx)
                    })
                    .await
                    .log_err()
                {
                    worktree_roots.insert(worktree.read_with(&mut cx, |tree, _| tree.abs_path()));
                    project_paths.push((path, Some(project_entry)));
                } else {
                    project_paths.push((path, None));
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

            let build_workspace = |cx: &mut ViewContext<Workspace>| {
                Workspace::new(workspace_id, project_handle.clone(), app_state.clone(), cx)
            };

            let workspace = requesting_window_id
                .and_then(|window_id| {
                    cx.update(|cx| cx.replace_root_view(window_id, |cx| build_workspace(cx)))
                })
                .unwrap_or_else(|| {
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
                        |cx| build_workspace(cx),
                    )
                    .1
                });

            (app_state.initialize_workspace)(
                workspace.downgrade(),
                serialized_workspace.is_some(),
                app_state.clone(),
                cx.clone(),
            )
            .await
            .log_err();

            cx.update_window(workspace.window_id(), |cx| cx.activate_window());

            let workspace = workspace.downgrade();
            notify_if_database_failed(&workspace, &mut cx);
            let opened_items = open_items(
                serialized_workspace,
                &workspace,
                project_paths,
                app_state,
                cx,
            )
            .await;

            (workspace, opened_items)
        })
    }

    pub fn weak_handle(&self) -> WeakViewHandle<Self> {
        self.weak_self.clone()
    }

    pub fn left_dock(&self) -> &ViewHandle<Dock> {
        &self.left_dock
    }

    pub fn bottom_dock(&self) -> &ViewHandle<Dock> {
        &self.bottom_dock
    }

    pub fn right_dock(&self) -> &ViewHandle<Dock> {
        &self.right_dock
    }

    pub fn add_panel<T: Panel>(&mut self, panel: ViewHandle<T>, cx: &mut ViewContext<Self>) {
        let dock = match panel.position(cx) {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        };

        self.subscriptions.push(cx.subscribe(&panel, {
            let mut dock = dock.clone();
            let mut prev_position = panel.position(cx);
            move |this, panel, event, cx| {
                if T::should_change_position_on_event(event) {
                    let new_position = panel.read(cx).position(cx);
                    let mut was_visible = false;
                    dock.update(cx, |dock, cx| {
                        prev_position = new_position;

                        was_visible = dock.is_open()
                            && dock
                                .visible_panel()
                                .map_or(false, |active_panel| active_panel.id() == panel.id());
                        dock.remove_panel(&panel, cx);
                    });

                    if panel.is_zoomed(cx) {
                        this.zoomed_position = Some(new_position);
                    }

                    dock = match panel.read(cx).position(cx) {
                        DockPosition::Left => &this.left_dock,
                        DockPosition::Bottom => &this.bottom_dock,
                        DockPosition::Right => &this.right_dock,
                    }
                    .clone();
                    dock.update(cx, |dock, cx| {
                        dock.add_panel(panel.clone(), cx);
                        if was_visible {
                            dock.set_open(true, cx);
                            dock.activate_panel(dock.panels_len() - 1, cx);
                        }
                    });
                } else if T::should_zoom_in_on_event(event) {
                    this.zoom_out(cx);
                    dock.update(cx, |dock, cx| dock.set_panel_zoomed(&panel, true, cx));
                    if panel.has_focus(cx) {
                        this.zoomed = Some(panel.downgrade().into_any());
                        this.zoomed_position = Some(panel.read(cx).position(cx));
                    }
                } else if T::should_zoom_out_on_event(event) {
                    this.zoom_out(cx);
                } else if T::is_focus_event(event) {
                    if panel.is_zoomed(cx) {
                        this.zoomed = Some(panel.downgrade().into_any());
                        this.zoomed_position = Some(panel.read(cx).position(cx));
                    } else {
                        this.zoomed = None;
                        this.zoomed_position = None;
                    }
                    cx.notify();
                }
            }
        }));

        dock.update(cx, |dock, cx| dock.add_panel(panel, cx));
    }

    pub fn status_bar(&self) -> &ViewHandle<StatusBar> {
        &self.status_bar
    }

    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    pub fn user_store(&self) -> &ModelHandle<UserStore> {
        &self.app_state.user_store
    }

    pub fn project(&self) -> &ModelHandle<Project> {
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
                            let (old_fs_path, old_timestamp) = entry.get();
                            if &timestamp > old_timestamp {
                                assert_eq!(&fs_path, old_fs_path, "Inconsistent nav history");
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

    pub fn client(&self) -> &Client {
        &self.app_state.client
    }

    pub fn set_titlebar_item(&mut self, item: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.titlebar_item = Some(item);
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

    pub fn close_global(_: &CloseWindow, cx: &mut AppContext) {
        cx.spawn(|mut cx| async move {
            let id = cx
                .window_ids()
                .into_iter()
                .find(|&id| cx.window_is_active(id));
            if let Some(id) = id {
                //This can only get called when the window's project connection has been lost
                //so we don't need to prompt the user for anything and instead just close the window
                cx.remove_window(id);
            }
        })
        .detach();
    }

    pub fn close(
        &mut self,
        _: &CloseWindow,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let window_id = cx.window_id();
        let prepare = self.prepare_to_close(false, cx);
        Some(cx.spawn(|_, mut cx| async move {
            if prepare.await? {
                cx.remove_window(window_id);
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

        cx.spawn(|this, mut cx| async move {
            let workspace_count = cx
                .window_ids()
                .into_iter()
                .filter_map(|window_id| cx.root_view(window_id)?.clone().downcast::<Workspace>())
                .count();

            if let Some(active_call) = active_call {
                if !quitting
                    && workspace_count == 1
                    && active_call.read_with(&cx, |call, _| call.room().is_some())
                {
                    let answer = cx.prompt(
                        window_id,
                        PromptLevel::Warning,
                        "Do you want to leave the current call?",
                        &["Close window and hang up", "Cancel"],
                    );

                    if let Some(mut answer) = answer {
                        if answer.next().await == Some(1) {
                            return anyhow::Ok(false);
                        } else {
                            active_call
                                .update(&mut cx, |call, cx| call.hang_up(cx))
                                .await
                                .log_err();
                        }
                    }
                }
            }

            Ok(this
                .update(&mut cx, |this, cx| this.save_all_internal(true, cx))?
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
                        Some((pane.downgrade(), item.boxed_clone()))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let project = self.project.clone();
        cx.spawn(|_, mut cx| async move {
            for (pane, item) in dirty_items {
                let (singleton, project_entry_ids) =
                    cx.read(|cx| (item.is_singleton(cx), item.project_entry_ids(cx)));
                if singleton || !project_entry_ids.is_empty() {
                    if let Some(ix) =
                        pane.read_with(&cx, |pane, _| pane.index_for_item(item.as_ref()))?
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

    pub fn open(&mut self, _: &Open, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let mut paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        });

        Some(cx.spawn(|this, mut cx| async move {
            if let Some(paths) = paths.recv().await.flatten() {
                if let Some(task) = this
                    .update(&mut cx, |this, cx| this.open_workspace_for_paths(paths, cx))
                    .log_err()
                {
                    task.await?
                }
            }
            Ok(())
        }))
    }

    pub fn open_workspace_for_paths(
        &mut self,
        paths: Vec<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let window_id = cx.window_id();
        let is_remote = self.project.read(cx).is_remote();
        let has_worktree = self.project.read(cx).worktrees(cx).next().is_some();
        let has_dirty_items = self.items(cx).any(|item| item.is_dirty(cx));
        let close_task = if is_remote || has_worktree || has_dirty_items {
            None
        } else {
            Some(self.prepare_to_close(false, cx))
        };
        let app_state = self.app_state.clone();

        cx.spawn(|_, mut cx| async move {
            let window_id_to_replace = if let Some(close_task) = close_task {
                if !close_task.await? {
                    return Ok(());
                }
                Some(window_id)
            } else {
                None
            };
            cx.update(|cx| open_paths(&paths, &app_state, window_id_to_replace, cx))
                .await?;
            Ok(())
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn open_paths(
        &mut self,
        mut abs_paths: Vec<PathBuf>,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>> {
        log::info!("open paths {:?}", abs_paths);

        let fs = self.app_state.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn(|this, mut cx| async move {
            let mut project_paths = Vec::new();
            for path in &abs_paths {
                if let Some(project_path) = this
                    .update(&mut cx, |this, cx| {
                        Workspace::project_path_for_path(this.project.clone(), path, visible, cx)
                    })
                    .log_err()
                {
                    project_paths.push(project_path.await.log_err());
                } else {
                    project_paths.push(None);
                }
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
                                    .log_err()?
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

    pub fn absolute_path(&self, project_path: &ProjectPath, cx: &AppContext) -> Option<PathBuf> {
        let workspace_root = self
            .project()
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)?
            .read(cx)
            .abs_path();
        let project_path = project_path.path.as_ref();

        Some(if project_path == Path::new("") {
            workspace_root.to_path_buf()
        } else {
            workspace_root.join(project_path)
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
                    .update(&mut cx, |this, cx| this.open_paths(paths, true, cx))?
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
        project: ModelHandle<Project>,
        abs_path: &Path,
        visible: bool,
        cx: &mut AppContext,
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
        V: 'static + Modal,
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
            cx.subscribe(&modal, |this, _, event, cx| {
                if V::dismiss_on_event(event) {
                    this.dismiss_modal(cx);
                }
            })
            .detach();
            cx.focus(&modal);
            self.modal = Some(modal.into_any());
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
                if item.has_conflict(cx) {
                    const CONFLICT_MESSAGE: &str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";

                    let mut answer = cx.prompt(
                        PromptLevel::Warning,
                        CONFLICT_MESSAGE,
                        &["Overwrite", "Cancel"],
                    );
                    cx.spawn(|this, mut cx| async move {
                        let answer = answer.recv().await;
                        if answer == Some(0) {
                            this.update(&mut cx, |this, cx| item.save(this.project.clone(), cx))?
                                .await?;
                        }
                        Ok(())
                    })
                } else {
                    item.save(self.project.clone(), cx)
                }
            } else if item.is_singleton(cx) {
                let worktree = self.worktrees(cx).next();
                let start_abs_path = worktree
                    .and_then(|w| w.read(cx).as_local())
                    .map_or(Path::new(""), |w| w.abs_path())
                    .to_path_buf();
                let mut abs_path = cx.prompt_for_new_path(&start_abs_path);
                cx.spawn(|this, mut cx| async move {
                    if let Some(abs_path) = abs_path.recv().await.flatten() {
                        this.update(&mut cx, |_, cx| item.save_as(project, abs_path, cx))?
                            .await?;
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

    pub fn toggle_dock(&mut self, dock_side: DockPosition, cx: &mut ViewContext<Self>) {
        let dock = match dock_side {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        };
        let mut focus_center = false;
        let mut zoom_out = false;
        dock.update(cx, |dock, cx| {
            let other_is_zoomed = self.zoomed.is_some() && self.zoomed_position != Some(dock_side);
            let was_visible = dock.is_open() && !other_is_zoomed;
            dock.set_open(!was_visible, cx);

            if let Some(active_panel) = dock.active_panel() {
                if was_visible {
                    if active_panel.has_focus(cx) {
                        focus_center = true;
                    }
                } else {
                    if active_panel.is_zoomed(cx) {
                        cx.focus(active_panel.as_any());
                    }
                    zoom_out = true;
                }
            }
        });

        if zoom_out {
            self.zoom_out_everything_except(dock_side, cx);
        }
        if focus_center {
            cx.focus_self();
        }

        cx.notify();
        self.serialize_workspace(cx);
    }

    pub fn focus_panel<T: Panel>(&mut self, cx: &mut ViewContext<Self>) -> Option<ViewHandle<T>> {
        self.show_or_hide_panel::<T>(cx, |_, _| true)?
            .as_any()
            .clone()
            .downcast()
    }

    pub fn toggle_panel_focus<T: Panel>(&mut self, cx: &mut ViewContext<Self>) {
        self.show_or_hide_panel::<T>(cx, |panel, cx| !panel.has_focus(cx));
    }

    fn show_or_hide_panel<T: Panel>(
        &mut self,
        cx: &mut ViewContext<Self>,
        show: impl Fn(&dyn PanelHandle, &mut ViewContext<Dock>) -> bool,
    ) -> Option<Rc<dyn PanelHandle>> {
        for (dock, position) in [
            self.left_dock.clone(),
            self.bottom_dock.clone(),
            self.right_dock.clone(),
        ]
        .into_iter()
        .zip(
            [
                DockPosition::Left,
                DockPosition::Bottom,
                DockPosition::Right,
            ]
            .into_iter(),
        ) {
            if let Some(panel_index) = dock.read(cx).panel_index_for_type::<T>() {
                let mut focus_center = false;
                let mut zoom_out = false;
                let panel = dock.update(cx, |dock, cx| {
                    dock.activate_panel(panel_index, cx);

                    let panel = dock.active_panel().cloned();
                    if let Some(panel) = panel.as_ref() {
                        let should_show = show(&**panel, cx);
                        if should_show {
                            dock.set_open(true, cx);
                            cx.focus(panel.as_any());
                            zoom_out = true;
                        } else {
                            if panel.is_zoomed(cx) {
                                dock.set_open(false, cx);
                            }
                            focus_center = true;
                        }
                    }
                    panel
                });

                if zoom_out {
                    self.zoom_out_everything_except(position, cx);
                }
                if focus_center {
                    cx.focus_self();
                }

                self.serialize_workspace(cx);
                cx.notify();
                return panel;
            }
        }
        None
    }

    fn zoom_out(&mut self, cx: &mut ViewContext<Self>) {
        for pane in &self.panes {
            pane.update(cx, |pane, cx| pane.set_zoomed(false, cx));
        }

        self.left_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        self.bottom_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        self.right_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        self.zoomed = None;
        self.zoomed_position = None;

        cx.notify();
    }

    fn zoom_out_everything_except(
        &mut self,
        except_position: DockPosition,
        cx: &mut ViewContext<Self>,
    ) {
        for pane in &self.panes {
            pane.update(cx, |pane, cx| pane.set_zoomed(false, cx));
        }

        if except_position != DockPosition::Left {
            self.left_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        }

        if except_position != DockPosition::Bottom {
            self.bottom_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        }

        if except_position != DockPosition::Right {
            self.right_dock.update(cx, |dock, cx| dock.zoom_out(cx));
        }

        if self.zoomed_position != Some(except_position) {
            self.zoomed = None;
            self.zoomed_position = None;
        }

        cx.notify();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = cx.add_view(|cx| {
            Pane::new(
                self.weak_handle(),
                self.app_state.background_actions,
                self.pane_history_timestamp.clone(),
                cx,
            )
        });
        cx.subscribe(&pane, Self::handle_pane_event).detach();
        self.panes.push(pane.clone());
        cx.focus(&pane);
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

    pub fn open_abs_path(
        &mut self,
        abs_path: PathBuf,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<Box<dyn ItemHandle>>> {
        cx.spawn(|workspace, mut cx| async move {
            let open_paths_task_result = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_paths(vec![abs_path.clone()], visible, cx)
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

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        pane: Option<WeakViewHandle<Pane>>,
        focus_item: bool,
        cx: &mut ViewContext<Self>,
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
        cx.spawn(|this, mut cx| async move {
            let (project_entry_id, build_item) = task.await?;
            let pane = pane
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("pane was closed"))?;
            this.update(&mut cx, |this, cx| {
                Pane::open_item(this, pane, project_entry_id, focus_item, cx, build_item)
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
        cx.spawn(|_, mut cx| async move {
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

    pub fn open_shared_screen(&mut self, peer_id: PeerId, cx: &mut ViewContext<Self>) {
        if let Some(shared_screen) = self.shared_screen_for_peer(peer_id, &self.active_pane, cx) {
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
            cx.focus(&pane);
        } else {
            self.split_pane(self.active_pane.clone(), SplitDirection::Right, cx);
        }
    }

    pub fn activate_next_pane(&mut self, cx: &mut ViewContext<Self>) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let next_ix = (ix + 1) % panes.len();
            let next_pane = panes[next_ix].clone();
            cx.focus(&next_pane);
        }
    }

    pub fn activate_previous_pane(&mut self, cx: &mut ViewContext<Self>) {
        let panes = self.center.panes();
        if let Some(ix) = panes.iter().position(|pane| **pane == self.active_pane) {
            let prev_ix = cmp::min(ix.wrapping_sub(1), panes.len() - 1);
            let prev_pane = panes[prev_ix].clone();
            cx.focus(&prev_pane);
        }
    }

    fn handle_pane_focused(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.active_pane != pane {
            self.active_pane = pane.clone();
            self.status_bar.update(cx, |status_bar, cx| {
                status_bar.set_active_pane(&self.active_pane, cx);
            });
            self.active_item_path_changed(cx);
            self.last_active_center_pane = Some(pane.downgrade());
        }

        if pane.read(cx).is_zoomed() {
            self.zoomed = Some(pane.downgrade().into_any());
        } else {
            self.zoomed = None;
        }
        self.zoomed_position = None;

        self.update_followers(
            proto::update_followers::Variant::UpdateActiveView(proto::UpdateActiveView {
                id: self.active_item(cx).and_then(|item| {
                    item.to_followable_item_handle(cx)?
                        .remote_id(&self.app_state.client, cx)
                        .map(|id| id.to_proto())
                }),
                leader_id: self.leader_for_pane(&pane),
            }),
            cx,
        );

        cx.notify();
    }

    fn handle_pane_event(
        &mut self,
        pane: ViewHandle<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::Split(direction) => {
                self.split_pane(pane, *direction, cx);
            }
            pane::Event::Remove => self.remove_pane(pane, cx),
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
            pane::Event::Focus => {
                self.handle_pane_focused(pane.clone(), cx);
            }
            pane::Event::ZoomIn => {
                if pane == self.active_pane {
                    self.zoom_out(cx);
                    pane.update(cx, |pane, cx| pane.set_zoomed(true, cx));
                    if pane.read(cx).has_focus() {
                        self.zoomed = Some(pane.downgrade().into_any());
                        self.zoomed_position = None;
                    }
                    cx.notify();
                }
            }
            pane::Event::ZoomOut => self.zoom_out(cx),
        }

        self.serialize_workspace(cx);
    }

    pub fn split_pane(
        &mut self,
        pane: ViewHandle<Pane>,
        direction: SplitDirection,
        cx: &mut ViewContext<Self>,
    ) -> Option<ViewHandle<Pane>> {
        let item = pane.read(cx).active_item()?;
        let maybe_pane_handle = if let Some(clone) = item.clone_on_split(self.database_id(), cx) {
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

    pub fn split_pane_with_item(
        &mut self,
        pane_to_split: WeakViewHandle<Pane>,
        split_direction: SplitDirection,
        from: WeakViewHandle<Pane>,
        item_id_to_move: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(pane_to_split) = pane_to_split.upgrade(cx) else { return; };
        let Some(from) = from.upgrade(cx) else { return; };

        let new_pane = self.add_pane(cx);
        Pane::move_item(self, from.clone(), new_pane.clone(), item_id_to_move, 0, cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();
        cx.notify();
    }

    pub fn split_pane_with_project_entry(
        &mut self,
        pane_to_split: WeakViewHandle<Pane>,
        split_direction: SplitDirection,
        project_entry: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let pane_to_split = pane_to_split.upgrade(cx)?;
        let new_pane = self.add_pane(cx);
        self.center
            .split(&pane_to_split, &new_pane, split_direction)
            .unwrap();

        let path = self.project.read(cx).path_for_entry(project_entry, cx)?;
        let task = self.open_path(path, Some(new_pane.downgrade()), true, cx);
        Some(cx.foreground().spawn(async move {
            task.await?;
            Ok(())
        }))
    }

    fn remove_pane(&mut self, pane: ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        if self.center.remove(&pane).unwrap() {
            self.force_remove_pane(&pane, cx);
            self.unfollow(&pane, cx);
            self.last_leaders_by_pane.remove(&pane.downgrade());
            for removed_item in pane.read(cx).items() {
                self.panes_by_item.remove(&removed_item.id());
            }

            cx.notify();
        } else {
            self.active_item_path_changed(cx);
        }
    }

    pub fn panes(&self) -> &[ViewHandle<Pane>] {
        &self.panes
    }

    pub fn active_pane(&self) -> &ViewHandle<Pane> {
        &self.active_pane
    }

    fn project_remote_id_changed(&mut self, remote_id: Option<u64>, cx: &mut ViewContext<Self>) {
        if let Some(remote_id) = remote_id {
            self.remote_entity_subscription = Some(
                self.app_state
                    .client
                    .add_view_for_remote_entity(remote_id, cx),
            );
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
        leader_id: PeerId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
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
        let request = self.app_state.client.request(proto::Follow {
            project_id,
            leader_id: Some(leader_id),
        });

        Some(cx.spawn(|this, mut cx| async move {
            let response = request.await?;
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
            })??;
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
            .and_then(|leader_id| self.toggle_follow(leader_id, cx))
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
                        self.app_state
                            .client
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

    fn render_titlebar(&self, theme: &Theme, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        // TODO: There should be a better system in place for this
        // (https://github.com/zed-industries/zed/issues/1290)
        let is_fullscreen = cx.window_is_fullscreen();
        let container_theme = if is_fullscreen {
            let mut container_theme = theme.workspace.titlebar.container;
            container_theme.padding.left = container_theme.padding.right;
            container_theme
        } else {
            theme.workspace.titlebar.container
        };

        enum TitleBar {}
        MouseEventHandler::<TitleBar, _>::new(0, cx, |_, cx| {
            Stack::new()
                .with_children(
                    self.titlebar_item
                        .as_ref()
                        .map(|item| ChildView::new(item, cx)),
                )
                .contained()
                .with_style(container_theme)
        })
        .on_click(MouseButton::Left, |event, _, cx| {
            if event.click_count == 2 {
                cx.zoom_window();
            }
        })
        .constrained()
        .with_height(theme.workspace.titlebar.height)
        .into_any_named("titlebar")
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

    fn render_disconnected_overlay(
        &self,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<AnyElement<Workspace>> {
        if self.project.read(cx).is_read_only() {
            enum DisconnectedOverlay {}
            Some(
                MouseEventHandler::<DisconnectedOverlay, _>::new(0, cx, |_, cx| {
                    let theme = &theme::current(cx);
                    Label::new(
                        "Your connection to the remote project has been lost.",
                        theme.workspace.disconnected_overlay.text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(theme.workspace.disconnected_overlay.container)
                })
                .with_cursor_style(CursorStyle::Arrow)
                .capture_all()
                .into_any_named("disconnected overlay"),
            )
        } else {
            None
        }
    }

    fn render_notifications(
        &self,
        theme: &theme::Workspace,
        cx: &AppContext,
    ) -> Option<AnyElement<Workspace>> {
        if self.notifications.is_empty() {
            None
        } else {
            Some(
                Flex::column()
                    .with_children(self.notifications.iter().map(|(_, _, notification)| {
                        ChildView::new(notification.as_any(), cx)
                            .contained()
                            .with_style(theme.notification)
                    }))
                    .constrained()
                    .with_width(theme.notifications.width)
                    .contained()
                    .with_style(theme.notifications.container)
                    .aligned()
                    .bottom()
                    .right()
                    .into_any(),
            )
        }
    }

    // RPC handlers

    async fn handle_follow(
        this: WeakViewHandle<Self>,
        envelope: TypedEnvelope<proto::Follow>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FollowResponse> {
        this.update(&mut cx, |this, cx| {
            let client = &this.app_state.client;
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
        })?
    }

    async fn handle_unfollow(
        this: WeakViewHandle<Self>,
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
        })?
    }

    async fn handle_update_followers(
        this: WeakViewHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateFollowers>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<()> {
        let leader_id = envelope.original_sender_id()?;
        this.read_with(&cx, |this, _| {
            this.leader_updates_tx
                .unbounded_send((leader_id, envelope.payload))
        })??;
        Ok(())
    }

    async fn process_leader_update(
        this: &WeakViewHandle<Self>,
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
                })??;
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
                })??;
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
                })?;
                Self::add_views_from_leader(this.clone(), leader_id, panes, vec![view], cx).await?;
            }
        }
        this.update(cx, |this, cx| this.leader_updated(leader_id, cx))?;
        Ok(())
    }

    async fn add_views_from_leader(
        this: WeakViewHandle<Self>,
        leader_id: PeerId,
        panes: Vec<ViewHandle<Pane>>,
        views: Vec<proto::View>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let project = this.read_with(cx, |this, _| this.project.clone())?;
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
            })?;
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
            self.app_state
                .client
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
            let pane_was_focused = pane.read(cx).has_focus();
            if let Some(index) = pane.update(cx, |pane, _| pane.index_for_item(item.as_ref())) {
                pane.update(cx, |pane, cx| pane.activate_item(index, false, false, cx));
            } else {
                Pane::add_item(self, &pane, item.boxed_clone(), false, false, None, cx);
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
                        settings::get::<WorkspaceSettings>(cx).autosave,
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
            Member::Pane(pane) => {
                self.force_remove_pane(&pane, cx);
            }
        }
    }

    fn force_remove_pane(&mut self, pane: &ViewHandle<Pane>, cx: &mut ViewContext<Workspace>) {
        self.panes.retain(|p| p != pane);
        cx.focus(self.panes.last().unwrap());
        if self.last_active_center_pane == Some(pane.downgrade()) {
            self.last_active_center_pane = None;
        }
        cx.notify();
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
                    pane.has_focus(),
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

        fn build_serialized_docks(this: &Workspace, cx: &AppContext) -> DockStructure {
            let left_dock = this.left_dock.read(cx);
            let left_visible = left_dock.is_open();
            let left_active_panel = left_dock.visible_panel().and_then(|panel| {
                Some(
                    cx.view_ui_name(panel.as_any().window_id(), panel.id())?
                        .to_string(),
                )
            });

            let right_dock = this.right_dock.read(cx);
            let right_visible = right_dock.is_open();
            let right_active_panel = right_dock.visible_panel().and_then(|panel| {
                Some(
                    cx.view_ui_name(panel.as_any().window_id(), panel.id())?
                        .to_string(),
                )
            });

            let bottom_dock = this.bottom_dock.read(cx);
            let bottom_visible = bottom_dock.is_open();
            let bottom_active_panel = bottom_dock.visible_panel().and_then(|panel| {
                Some(
                    cx.view_ui_name(panel.as_any().window_id(), panel.id())?
                        .to_string(),
                )
            });

            DockStructure {
                left: DockData {
                    visible: left_visible,
                    active_panel: left_active_panel,
                },
                right: DockData {
                    visible: right_visible,
                    active_panel: right_active_panel,
                },
                bottom: DockData {
                    visible: bottom_visible,
                    active_panel: bottom_active_panel,
                },
            }
        }

        if let Some(location) = self.location(cx) {
            // Load bearing special case:
            //  - with_local_workspace() relies on this to not have other stuff open
            //    when you open your log
            if !location.paths().is_empty() {
                let center_group = build_serialized_pane_group(&self.center.root, cx);
                let docks = build_serialized_docks(self, cx);

                let serialized_workspace = SerializedWorkspace {
                    id: self.database_id,
                    location,
                    center_group,
                    bounds: Default::default(),
                    display: Default::default(),
                    docks,
                };

                cx.background()
                    .spawn(persistence::DB.save_workspace(serialized_workspace))
                    .detach();
            }
        }
    }

    pub(crate) fn load_workspace(
        workspace: WeakViewHandle<Workspace>,
        serialized_workspace: SerializedWorkspace,
        paths_to_open: Vec<Option<ProjectPath>>,
        cx: &mut AppContext,
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>> {
        cx.spawn(|mut cx| async move {
            let result = async_iife! {{
                let (project, old_center_pane) =
                workspace.read_with(&cx, |workspace, _| {
                    (
                        workspace.project().clone(),
                        workspace.last_active_center_pane.clone(),
                    )
                })?;

                let mut center_items = None;
                let mut center_group = None;
                // Traverse the splits tree and add to things
                if let Some((group, active_pane, items)) = serialized_workspace
                        .center_group
                        .deserialize(&project, serialized_workspace.id, &workspace, &mut cx)
                        .await {
                    center_items = Some(items);
                    center_group = Some((group, active_pane))
                }

                let resulting_list = cx.read(|cx| {
                    let mut opened_items = center_items
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|item| {
                            let item = item?;
                            let project_path = item.project_path(cx)?;
                            Some((project_path, item))
                        })
                        .collect::<HashMap<_, _>>();

                    paths_to_open
                        .into_iter()
                        .map(|path_to_open| {
                            path_to_open.map(|path_to_open| {
                                Ok(opened_items.remove(&path_to_open))
                            })
                            .transpose()
                            .map(|item| item.flatten())
                            .transpose()
                        })
                        .collect::<Vec<_>>()
                });

                // Remove old panes from workspace panes list
                workspace.update(&mut cx, |workspace, cx| {
                    if let Some((center_group, active_pane)) = center_group {
                        workspace.remove_panes(workspace.center.root.clone(), cx);

                        // Swap workspace center group
                        workspace.center = PaneGroup::with_root(center_group);

                        // Change the focus to the workspace first so that we retrigger focus in on the pane.
                        cx.focus_self();

                        if let Some(active_pane) = active_pane {
                            cx.focus(&active_pane);
                        } else {
                            cx.focus(workspace.panes.last().unwrap());
                        }
                    } else {
                        let old_center_handle = old_center_pane.and_then(|weak| weak.upgrade(cx));
                        if let Some(old_center_handle) = old_center_handle {
                            cx.focus(&old_center_handle)
                        } else {
                            cx.focus_self()
                        }
                    }

                    let docks = serialized_workspace.docks;
                    workspace.left_dock.update(cx, |dock, cx| {
                        dock.set_open(docks.left.visible, cx);
                        if let Some(active_panel) = docks.left.active_panel {
                            if let Some(ix) = dock.panel_index_for_ui_name(&active_panel, cx) {
                                dock.activate_panel(ix, cx);
                            }
                        }
                    });
                    workspace.right_dock.update(cx, |dock, cx| {
                        dock.set_open(docks.right.visible, cx);
                        if let Some(active_panel) = docks.right.active_panel {
                            if let Some(ix) = dock.panel_index_for_ui_name(&active_panel, cx) {
                                dock.activate_panel(ix, cx);
                            }
                        }
                    });
                    workspace.bottom_dock.update(cx, |dock, cx| {
                        dock.set_open(docks.bottom.visible, cx);
                        if let Some(active_panel) = docks.bottom.active_panel {
                            if let Some(ix) = dock.panel_index_for_ui_name(&active_panel, cx) {
                                dock.activate_panel(ix, cx);
                            }
                        }
                    });

                    cx.notify();
                })?;

                // Serialize ourself to make sure our timestamps and any pane / item changes are replicated
                workspace.read_with(&cx, |workspace, cx| workspace.serialize_workspace(cx))?;

                Ok::<_, anyhow::Error>(resulting_list)
            }};

            result.await.unwrap_or_default()
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        let app_state = Arc::new(AppState {
            languages: project.read(cx).languages().clone(),
            client: project.read(cx).client(),
            user_store: project.read(cx).user_store(),
            fs: project.read(cx).fs().clone(),
            build_window_options: |_, _, _| Default::default(),
            initialize_workspace: |_, _, _, _| Task::ready(Ok(())),
            background_actions: || &[],
        });
        Self::new(0, project, app_state, cx)
    }

    fn render_dock(&self, position: DockPosition, cx: &WindowContext) -> Option<AnyElement<Self>> {
        let dock = match position {
            DockPosition::Left => &self.left_dock,
            DockPosition::Right => &self.right_dock,
            DockPosition::Bottom => &self.bottom_dock,
        };
        let active_panel = dock.read(cx).visible_panel()?;
        let element = if Some(active_panel.id()) == self.zoomed.as_ref().map(|zoomed| zoomed.id()) {
            dock.read(cx).render_placeholder(cx)
        } else {
            ChildView::new(dock, cx).into_any()
        };

        Some(
            element
                .constrained()
                .dynamically(move |constraint, _, cx| match position {
                    DockPosition::Left | DockPosition::Right => SizeConstraint::new(
                        Vector2F::new(20., constraint.min.y()),
                        Vector2F::new(cx.window_size().x() * 0.8, constraint.max.y()),
                    ),
                    DockPosition::Bottom => SizeConstraint::new(
                        Vector2F::new(constraint.min.x(), 20.),
                        Vector2F::new(constraint.max.x(), cx.window_size().y() * 0.8),
                    ),
                })
                .into_any(),
        )
    }
}

async fn open_items(
    serialized_workspace: Option<SerializedWorkspace>,
    workspace: &WeakViewHandle<Workspace>,
    mut project_paths_to_open: Vec<(PathBuf, Option<ProjectPath>)>,
    app_state: Arc<AppState>,
    mut cx: AsyncAppContext,
) -> Vec<Option<anyhow::Result<Box<dyn ItemHandle>>>> {
    let mut opened_items = Vec::with_capacity(project_paths_to_open.len());

    if let Some(serialized_workspace) = serialized_workspace {
        let workspace = workspace.clone();
        let restored_items = cx
            .update(|cx| {
                Workspace::load_workspace(
                    workspace,
                    serialized_workspace,
                    project_paths_to_open
                        .iter()
                        .map(|(_, project_path)| project_path)
                        .cloned()
                        .collect(),
                    cx,
                )
            })
            .await;

        let restored_project_paths = cx.read(|cx| {
            restored_items
                .iter()
                .filter_map(|item| item.as_ref()?.as_ref().ok()?.project_path(cx))
                .collect::<HashSet<_>>()
        });

        opened_items = restored_items;
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
            .map(|(i, (abs_path, project_path))| {
                let workspace = workspace.clone();
                cx.spawn(|mut cx| {
                    let fs = app_state.fs.clone();
                    async move {
                        let file_project_path = project_path?;
                        if fs.is_file(&abs_path).await {
                            Some((
                                i,
                                workspace
                                    .update(&mut cx, |workspace, cx| {
                                        workspace.open_path(file_project_path, None, true, cx)
                                    })
                                    .log_err()?
                                    .await,
                            ))
                        } else {
                            None
                        }
                    }
                })
            });

    for maybe_opened_path in futures::future::join_all(tasks.into_iter())
        .await
        .into_iter()
    {
        if let Some((i, path_open_result)) = maybe_opened_path {
            opened_items[i] = Some(path_open_result);
        }
    }

    opened_items
}

fn notify_if_database_failed(workspace: &WeakViewHandle<Workspace>, cx: &mut AsyncAppContext) {
    const REPORT_ISSUE_URL: &str ="https://github.com/zed-industries/community/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml";

    workspace
        .update(cx, |workspace, cx| {
            if (*db::ALL_FILE_DB_FAILED).load(std::sync::atomic::Ordering::Acquire) {
                workspace.show_notification_once(0, cx, |cx| {
                    cx.add_view(|_| {
                        MessageNotification::new("Failed to load any database file.")
                            .with_click_message("Click to let us know about this error")
                            .on_click(|cx| cx.platform().open_url(REPORT_ISSUE_URL))
                    })
                });
            } else {
                let backup_path = (*db::BACKUP_DB_PATH).read();
                if let Some(backup_path) = backup_path.clone() {
                    workspace.show_notification_once(0, cx, move |cx| {
                        cx.add_view(move |_| {
                            MessageNotification::new(format!(
                                "Database file was corrupted. Old database backed up to {}",
                                backup_path.display()
                            ))
                            .with_click_message("Click to show old database in finder")
                            .on_click(move |cx| {
                                cx.platform().open_url(&backup_path.to_string_lossy())
                            })
                        })
                    });
                }
            }
        })
        .log_err();
}

impl Entity for Workspace {
    type Event = Event;
}

impl View for Workspace {
    fn ui_name() -> &'static str {
        "Workspace"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        Stack::new()
            .with_child(
                Flex::column()
                    .with_child(self.render_titlebar(&theme, cx))
                    .with_child(
                        Stack::new()
                            .with_child({
                                let project = self.project.clone();
                                Flex::row()
                                    .with_children(self.render_dock(DockPosition::Left, cx))
                                    .with_child(
                                        Flex::column()
                                            .with_child(
                                                FlexItem::new(
                                                    self.center.render(
                                                        &project,
                                                        &theme,
                                                        &self.follower_states_by_leader,
                                                        self.active_call(),
                                                        self.active_pane(),
                                                        self.zoomed
                                                            .as_ref()
                                                            .and_then(|zoomed| zoomed.upgrade(cx))
                                                            .as_ref(),
                                                        &self.app_state,
                                                        cx,
                                                    ),
                                                )
                                                .flex(1., true),
                                            )
                                            .with_children(
                                                self.render_dock(DockPosition::Bottom, cx),
                                            )
                                            .flex(1., true),
                                    )
                                    .with_children(self.render_dock(DockPosition::Right, cx))
                            })
                            .with_child(Overlay::new(
                                Stack::new()
                                    .with_children(self.zoomed.as_ref().and_then(|zoomed| {
                                        enum ZoomBackground {}
                                        let zoomed = zoomed.upgrade(cx)?;

                                        let mut foreground_style;
                                        match self.zoomed_position {
                                            Some(DockPosition::Left) => {
                                                foreground_style =
                                                    theme.workspace.zoomed_panel_foreground;
                                                foreground_style.margin.left = 0.;
                                                foreground_style.margin.top = 0.;
                                                foreground_style.margin.bottom = 0.;
                                            }
                                            Some(DockPosition::Right) => {
                                                foreground_style =
                                                    theme.workspace.zoomed_panel_foreground;
                                                foreground_style.margin.right = 0.;
                                                foreground_style.margin.top = 0.;
                                                foreground_style.margin.bottom = 0.;
                                            }
                                            Some(DockPosition::Bottom) => {
                                                foreground_style =
                                                    theme.workspace.zoomed_panel_foreground;
                                                foreground_style.margin.left = 0.;
                                                foreground_style.margin.right = 0.;
                                                foreground_style.margin.bottom = 0.;
                                            }
                                            None => {
                                                foreground_style =
                                                    theme.workspace.zoomed_pane_foreground;
                                            }
                                        }

                                        Some(
                                            ChildView::new(&zoomed, cx)
                                                .contained()
                                                .with_style(foreground_style)
                                                .aligned()
                                                .contained()
                                                .with_style(theme.workspace.zoomed_background)
                                                .mouse::<ZoomBackground>(0)
                                                .capture_all()
                                                .on_down(
                                                    MouseButton::Left,
                                                    |_, this: &mut Self, cx| {
                                                        this.zoom_out(cx);
                                                    },
                                                ),
                                        )
                                    }))
                                    .with_children(self.modal.as_ref().map(|modal| {
                                        ChildView::new(modal, cx)
                                            .contained()
                                            .with_style(theme.workspace.modal)
                                            .aligned()
                                            .top()
                                    }))
                                    .with_children(self.render_notifications(&theme.workspace, cx)),
                            ))
                            .flex(1.0, true),
                    )
                    .with_child(ChildView::new(&self.status_bar, cx))
                    .contained()
                    .with_background_color(theme.workspace.background),
            )
            .with_children(DragAndDrop::render(cx))
            .with_children(self.render_disconnected_overlay(cx))
            .into_any_named("workspace")
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.active_pane);
        }
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
    cx: &mut AsyncAppContext,
    predicate: impl Fn(&mut Project, &mut ModelContext<Project>) -> bool,
) -> Option<WeakViewHandle<Workspace>> {
    for window_id in cx.window_ids() {
        let handle = cx
            .update_window(window_id, |cx| {
                if let Some(workspace_handle) = cx.root_view().clone().downcast::<Workspace>() {
                    let project = workspace_handle.read(cx).project.clone();
                    if project.update(cx, &predicate) {
                        cx.activate_window();
                        return Some(workspace_handle.clone());
                    }
                }
                None
            })
            .flatten();

        if let Some(handle) = handle {
            return Some(handle.downgrade());
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
    cx: &mut AppContext,
) -> Task<
    Result<(
        WeakViewHandle<Workspace>,
        Vec<Option<Result<Box<dyn ItemHandle>, anyhow::Error>>>,
    )>,
> {
    let app_state = app_state.clone();
    let abs_paths = abs_paths.to_vec();
    cx.spawn(|mut cx| async move {
        // Open paths in existing workspace if possible
        let existing = activate_workspace_for_project(&mut cx, |project, cx| {
            project.contains_paths(&abs_paths, cx)
        });

        if let Some(existing) = existing {
            Ok((
                existing.clone(),
                existing
                    .update(&mut cx, |workspace, cx| {
                        workspace.open_paths(abs_paths, true, cx)
                    })?
                    .await,
            ))
        } else {
            Ok(cx
                .update(|cx| {
                    Workspace::new_local(abs_paths, app_state.clone(), requesting_window_id, cx)
                })
                .await)
        }
    })
}

pub fn open_new(
    app_state: &Arc<AppState>,
    cx: &mut AppContext,
    init: impl FnOnce(&mut Workspace, &mut ViewContext<Workspace>) + 'static,
) -> Task<()> {
    let task = Workspace::new_local(Vec::new(), app_state.clone(), None, cx);
    cx.spawn(|mut cx| async move {
        let (workspace, opened_paths) = task.await;

        workspace
            .update(&mut cx, |workspace, cx| {
                if opened_paths.is_empty() {
                    init(workspace, cx)
                }
            })
            .log_err();
    })
}

pub fn create_and_open_local_file(
    path: &'static Path,
    cx: &mut ViewContext<Workspace>,
    default_content: impl 'static + Send + FnOnce() -> Rope,
) -> Task<Result<Box<dyn ItemHandle>>> {
    cx.spawn(|workspace, mut cx| async move {
        let fs = workspace.read_with(&cx, |workspace, _| workspace.app_state().fs.clone())?;
        if !fs.is_file(path).await {
            fs.create_file(path, Default::default()).await?;
            fs.save(path, &default_content(), Default::default())
                .await?;
        }

        let mut items = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.with_local_workspace(cx, |workspace, cx| {
                    workspace.open_paths(vec![path.to_path_buf()], false, cx)
                })
            })?
            .await?
            .await;

        let item = items.pop().flatten();
        item.ok_or_else(|| anyhow!("path {path:?} is not a file"))?
    })
}

pub fn join_remote_project(
    project_id: u64,
    follow_user_id: u64,
    app_state: Arc<AppState>,
    cx: &mut AppContext,
) -> Task<Result<()>> {
    cx.spawn(|mut cx| async move {
        let existing_workspace = cx
            .window_ids()
            .into_iter()
            .filter_map(|window_id| cx.root_view(window_id)?.clone().downcast::<Workspace>())
            .find(|workspace| {
                cx.read_window(workspace.window_id(), |cx| {
                    workspace.read(cx).project().read(cx).remote_id() == Some(project_id)
                })
                .unwrap_or(false)
            });

        let workspace = if let Some(existing_workspace) = existing_workspace {
            existing_workspace.downgrade()
        } else {
            let active_call = cx.read(ActiveCall::global);
            let room = active_call
                .read_with(&cx, |call, _| call.room().cloned())
                .ok_or_else(|| anyhow!("not in a call"))?;
            let project = room
                .update(&mut cx, |room, cx| {
                    room.join_project(
                        project_id,
                        app_state.languages.clone(),
                        app_state.fs.clone(),
                        cx,
                    )
                })
                .await?;

            let (_, workspace) = cx.add_window(
                (app_state.build_window_options)(None, None, cx.platform().as_ref()),
                |cx| Workspace::new(0, project, app_state.clone(), cx),
            );
            (app_state.initialize_workspace)(
                workspace.downgrade(),
                false,
                app_state.clone(),
                cx.clone(),
            )
            .await
            .log_err();

            workspace.downgrade()
        };

        cx.activate_window(workspace.window_id());
        cx.platform().activate(true);

        workspace.update(&mut cx, |workspace, cx| {
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
                    if !workspace.is_being_followed(follow_peer_id) {
                        workspace
                            .toggle_follow(follow_peer_id, cx)
                            .map(|follow| follow.detach_and_log_err(cx));
                    }
                }
            }
        })?;

        anyhow::Ok(())
    })
}

pub fn restart(_: &Restart, cx: &mut AppContext) {
    let should_confirm = settings::get::<WorkspaceSettings>(cx).confirm_quit;
    cx.spawn(|mut cx| async move {
        let mut workspaces = cx
            .window_ids()
            .into_iter()
            .filter_map(|window_id| {
                Some(
                    cx.root_view(window_id)?
                        .clone()
                        .downcast::<Workspace>()?
                        .downgrade(),
                )
            })
            .collect::<Vec<_>>();

        // If multiple windows have unsaved changes, and need a save prompt,
        // prompt in the active window before switching to a different window.
        workspaces.sort_by_key(|workspace| !cx.window_is_active(workspace.window_id()));

        if let (true, Some(workspace)) = (should_confirm, workspaces.first()) {
            let answer = cx.prompt(
                workspace.window_id(),
                PromptLevel::Info,
                "Are you sure you want to restart?",
                &["Restart", "Cancel"],
            );

            if let Some(mut answer) = answer {
                let answer = answer.next().await;
                if answer != Some(0) {
                    return Ok(());
                }
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for workspace in workspaces {
            if !workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.prepare_to_close(true, cx)
                })?
                .await?
            {
                return Ok(());
            }
        }
        cx.platform().restart();
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn parse_pixel_position_env_var(value: &str) -> Option<Vector2F> {
    let mut parts = value.split(',');
    let width: usize = parts.next()?.parse().ok()?;
    let height: usize = parts.next()?.parse().ok()?;
    Some(vec2f(width as f32, height as f32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dock::test::{TestPanel, TestPanelEvent},
        item::test::{TestItem, TestItemEvent, TestProjectItem},
    };
    use fs::FakeFs;
    use gpui::{executor::Deterministic, test::EmptyView, TestAppContext};
    use project::{Project, ProjectEntryId};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cell::RefCell, rc::Rc};

    #[gpui::test]
    async fn test_tab_disambiguation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

        // Adding an item with no ambiguity renders the tab without detail.
        let item1 = cx.add_view(window_id, |_| {
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
        let item2 = cx.add_view(window_id, |_| {
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
        let item3 = cx.add_view(window_id, |_| {
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
        init_test(cx);

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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let item1 = cx.add_view(window_id, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
        });
        let item2 = cx.add_view(window_id, |cx| {
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
        pane.update(cx, |pane, cx| {
            pane.close_active_item(&Default::default(), cx).unwrap()
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
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        assert_eq!(
            cx.current_window_title(window_id).as_deref(),
            Some("one.txt — root2")
        );
    }

    #[gpui::test]
    async fn test_close_window(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/root", json!({ "one": "" })).await;

        let project = Project::test(fs, ["root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

        // When there are no dirty items, there's nothing to do.
        let item1 = cx.add_view(window_id, |_| TestItem::new());
        workspace.update(cx, |w, cx| w.add_item(Box::new(item1.clone()), cx));
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(false, cx));
        assert!(task.await.unwrap());

        // When there are dirty untitled items, prompt to save each one. If the user
        // cancels any prompt, then abort.
        let item2 = cx.add_view(window_id, |_| TestItem::new().with_dirty(true));
        let item3 = cx.add_view(window_id, |cx| {
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
        init_test(cx);

        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let item1 = cx.add_view(window_id, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item2 = cx.add_view(window_id, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)])
        });
        let item3 = cx.add_view(window_id, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_conflict(true)
                .with_project_items(&[TestProjectItem::new(3, "3.txt", cx)])
        });
        let item4 = cx.add_view(window_id, |cx| {
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

        let close_items = pane.update(cx, |pane, cx| {
            pane.activate_item(1, true, true, cx);
            assert_eq!(pane.active_item().unwrap().id(), item2.id());
            let item1_id = item1.id();
            let item3_id = item3.id();
            let item4_id = item4.id();
            pane.close_items(cx, move |id| [item1_id, item3_id, item4_id].contains(&id))
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
        init_test(cx);

        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        // Create several workspace items with single project entries, and two
        // workspace items with multiple project entries.
        let single_entry_items = (0..=4)
            .map(|project_entry_id| {
                cx.add_view(window_id, |cx| {
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
        let item_2_3 = cx.add_view(window_id, |cx| {
            TestItem::new()
                .with_dirty(true)
                .with_singleton(false)
                .with_project_items(&[
                    single_entry_items[2].read(cx).project_items[0].clone(),
                    single_entry_items[3].read(cx).project_items[0].clone(),
                ])
        });
        let item_3_4 = cx.add_view(window_id, |cx| {
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
            cx.focus(&left_pane);
        });

        // When closing all of the items in the left pane, we should be prompted twice:
        // once for project entry 0, and once for project entry 2. After those two
        // prompts, the task should complete.

        let close = left_pane.update(cx, |pane, cx| pane.close_items(cx, |_| true));
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
        init_test(cx);

        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        let item = cx.add_view(window_id, |cx| {
            TestItem::new().with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
        });
        let item_id = item.id();
        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item.clone()), cx);
        });

        // Autosave on window change.
        item.update(cx, |item, cx| {
            cx.update_global(|settings: &mut SettingsStore, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnWindowChange);
                })
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
            cx.update_global(|settings: &mut SettingsStore, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnFocusChange);
                })
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
            cx.update_global(|settings: &mut SettingsStore, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::AfterDelay { milliseconds: 500 });
                })
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
            cx.update_global(|settings: &mut SettingsStore, cx| {
                settings.update_user_settings::<WorkspaceSettings>(cx, |settings| {
                    settings.autosave = Some(AutosaveSetting::OnFocusChange);
                })
            });
            item.is_dirty = true;
        });

        pane.update(cx, |pane, cx| pane.close_items(cx, move |id| id == item_id))
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
        let _close_items =
            pane.update(cx, |pane, cx| pane.close_items(cx, move |id| id == item_id));
        deterministic.run_until_parked();
        assert!(cx.has_pending_prompt(window_id));
        item.read_with(cx, |item, _| assert_eq!(item.save_count, 5));
    }

    #[gpui::test]
    async fn test_pane_navigation(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let item = cx.add_view(window_id, |cx| {
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
                Pane::go_back(workspace, Some(pane.downgrade()), cx)
            })
            .await
            .unwrap();

        assert_eq!(*toolbar_notify_count.borrow(), 3);
        pane.read_with(cx, |pane, _| {
            assert!(!pane.can_navigate_backward());
            assert!(pane.can_navigate_forward());
        });
    }

    #[gpui::test]
    async fn test_toggle_docks_and_panels(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let panel = workspace.update(cx, |workspace, cx| {
            let panel = cx.add_view(|_| TestPanel::new(DockPosition::Right));
            workspace.add_panel(panel.clone(), cx);

            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, cx));

            panel
        });

        // Transfer focus from center to panel
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(panel.has_focus(cx));
        });

        // Transfer focus from panel to center
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(!panel.has_focus(cx));
        });

        // Close the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(!panel.has_focus(cx));
        });

        // Open the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(!panel.is_zoomed(cx));
            assert!(!panel.has_focus(cx));
        });

        // Focus and zoom panel
        panel.update(cx, |panel, cx| {
            cx.focus_self();
            panel.set_zoomed(true, cx)
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(panel.has_focus(cx));
        });

        // Transfer focus to the center closes the dock
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(!panel.has_focus(cx));
        });

        // Transfering focus back to the panel keeps it zoomed
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_panel_focus::<TestPanel>(cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(panel.has_focus(cx));
        });

        // Close the dock while it is zoomed
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(!workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(workspace.zoomed.is_none());
            assert!(!panel.has_focus(cx));
        });

        // Opening the dock, when it's zoomed, retains focus
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_dock(DockPosition::Right, cx)
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.right_dock().read(cx).is_open());
            assert!(panel.is_zoomed(cx));
            assert!(workspace.zoomed.is_some());
            assert!(panel.has_focus(cx));
        });
    }

    #[gpui::test]
    async fn test_panels(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let (panel_1, panel_2) = workspace.update(cx, |workspace, cx| {
            // Add panel_1 on the left, panel_2 on the right.
            let panel_1 = cx.add_view(|_| TestPanel::new(DockPosition::Left));
            workspace.add_panel(panel_1.clone(), cx);
            workspace
                .left_dock()
                .update(cx, |left_dock, cx| left_dock.set_open(true, cx));
            let panel_2 = cx.add_view(|_| TestPanel::new(DockPosition::Right));
            workspace.add_panel(panel_2.clone(), cx);
            workspace
                .right_dock()
                .update(cx, |right_dock, cx| right_dock.set_open(true, cx));

            let left_dock = workspace.left_dock();
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().id(),
                panel_1.id()
            );
            assert_eq!(
                left_dock.read(cx).active_panel_size(cx).unwrap(),
                panel_1.size(cx)
            );

            left_dock.update(cx, |left_dock, cx| left_dock.resize_active_panel(1337., cx));
            assert_eq!(
                workspace
                    .right_dock()
                    .read(cx)
                    .visible_panel()
                    .unwrap()
                    .id(),
                panel_2.id()
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
                right_dock.read(cx).visible_panel().unwrap().id(),
                panel_1.id()
            );
            assert_eq!(right_dock.read(cx).active_panel_size(cx).unwrap(), 1337.);

            // Now we move panel_2 to the left
            panel_2.set_position(DockPosition::Left, cx);
        });

        workspace.update(cx, |workspace, cx| {
            // Since panel_2 was not visible on the right, we don't open the left dock.
            assert!(!workspace.left_dock().read(cx).is_open());
            // And the right dock is unaffected in it's displaying of panel_1
            assert!(workspace.right_dock().read(cx).is_open());
            assert_eq!(
                workspace
                    .right_dock()
                    .read(cx)
                    .visible_panel()
                    .unwrap()
                    .id(),
                panel_1.id()
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
                left_dock.read(cx).visible_panel().unwrap().id(),
                panel_1.id()
            );
            assert_eq!(left_dock.read(cx).active_panel_size(cx).unwrap(), 1337.);
            // And right the dock should be closed as it no longer has any panels.
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
        panel_1.update(cx, |_, cx| cx.emit(TestPanelEvent::Activated));

        // Now the left dock is open and panel_1 is active and focused.
        workspace.read_with(cx, |workspace, cx| {
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().id(),
                panel_1.id()
            );
            assert!(panel_1.is_focused(cx));
        });

        // Emit closed event on panel 2, which is not active
        panel_2.update(cx, |_, cx| cx.emit(TestPanelEvent::Closed));

        // Wo don't close the left dock, because panel_2 wasn't the active panel
        workspace.read_with(cx, |workspace, cx| {
            let left_dock = workspace.left_dock();
            assert!(left_dock.read(cx).is_open());
            assert_eq!(
                left_dock.read(cx).visible_panel().unwrap().id(),
                panel_1.id()
            );
        });

        // Emitting a ZoomIn event shows the panel as zoomed.
        panel_1.update(cx, |_, cx| cx.emit(TestPanelEvent::ZoomIn));
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.downgrade().into_any()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Left));
        });

        // Move panel to another dock while it is zoomed
        panel_1.update(cx, |panel, cx| panel.set_position(DockPosition::Right, cx));
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.downgrade().into_any()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // If focus is transferred to another view that's not a panel or another pane, we still show
        // the panel as zoomed.
        let focus_receiver = cx.add_view(window_id, |_| EmptyView);
        focus_receiver.update(cx, |_, cx| cx.focus_self());
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.downgrade().into_any()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // If focus is transferred elsewhere in the workspace, the panel is no longer zoomed.
        workspace.update(cx, |_, cx| cx.focus_self());
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // If focus is transferred again to another view that's not a panel or a pane, we won't
        // show the panel as zoomed because it wasn't zoomed before.
        focus_receiver.update(cx, |_, cx| cx.focus_self());
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // When focus is transferred back to the panel, it is zoomed again.
        panel_1.update(cx, |_, cx| cx.focus_self());
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, Some(panel_1.downgrade().into_any()));
            assert_eq!(workspace.zoomed_position, Some(DockPosition::Right));
        });

        // Emitting a ZoomOut event unzooms the panel.
        panel_1.update(cx, |_, cx| cx.emit(TestPanelEvent::ZoomOut));
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.zoomed, None);
            assert_eq!(workspace.zoomed_position, None);
        });

        // Emit closed event on panel 1, which is active
        panel_1.update(cx, |_, cx| cx.emit(TestPanelEvent::Closed));

        // Now the left dock is closed, because panel_1 was the active panel
        workspace.read_with(cx, |workspace, cx| {
            let right_dock = workspace.right_dock();
            assert!(!right_dock.read(cx).is_open());
        });
    }

    pub fn init_test(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            theme::init((), cx);
            language::init(cx);
            crate::init_settings(cx);
        });
    }
}
