/// NOTE: Focus only 'takes' after an update has flushed_effects. Pane sends an event in on_focus_in
/// which the workspace uses to change the activated pane.
///
/// This may cause issues when you're trying to write tests that use workspace focus to add items at
/// specific locations.
pub mod pane;
pub mod pane_group;
pub mod programs;
pub mod searchable;
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
use drag_and_drop::DragAndDrop;
use futures::{channel::oneshot, FutureExt};
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    impl_actions, impl_internal_actions,
    json::{self, ToJson},
    platform::{CursorStyle, WindowOptions},
    AnyModelHandle, AnyViewHandle, AppContext, AsyncAppContext, Border, Entity, ImageData,
    ModelContext, ModelHandle, MouseButton, MutableAppContext, PathPromptOptions, PromptLevel,
    RenderContext, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::LanguageRegistry;
use log::error;
pub use pane::*;
pub use pane_group::*;
use postage::prelude::Stream;
use programs::ProgramManager;
use project::{fs, Fs, Project, ProjectEntryId, ProjectPath, ProjectStore, Worktree, WorktreeId};
use searchable::SearchableItemHandle;
use serde::Deserialize;
use settings::{Autosave, Settings};
use sidebar::{Side, Sidebar, SidebarButtons, ToggleSidebarItem};
use smallvec::SmallVec;
use status_bar::StatusBar;
pub use status_bar::StatusItemView;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cell::RefCell,
    fmt,
    future::Future,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};
use theme::{Theme, ThemeRegistry};
pub use toolbar::{ToolbarItemLocation, ToolbarItemView};
use util::ResultExt;
use waiting_room::WaitingRoom;

type ProjectItemBuilders = HashMap<
    TypeId,
    fn(ModelHandle<Project>, AnyModelHandle, &mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
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
        ToggleRightSidebar,
        NewTerminal,
        NewSearch
    ]
);

#[derive(Clone, PartialEq)]
pub struct OpenPaths {
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ToggleProjectOnline {
    #[serde(skip_deserializing)]
    pub project: Option<ModelHandle<Project>>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePane(pub usize);

#[derive(Clone, PartialEq)]
pub struct ToggleFollow(pub PeerId);

#[derive(Clone, PartialEq)]
pub struct JoinProject {
    pub contact: Arc<Contact>,
    pub project_index: usize,
}

impl_internal_actions!(
    workspace,
    [
        OpenPaths,
        ToggleFollow,
        JoinProject,
        RemoveWorktreeFromProject
    ]
);
impl_actions!(workspace, [ToggleProjectOnline, ActivatePane]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    // Initialize the program manager immediately
    cx.set_global(ProgramManager::new());

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
        move |_: &NewFile, cx: &mut MutableAppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_new(&app_state, cx)
            }
        }
    });
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &NewWindow, cx: &mut MutableAppContext| {
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
    cx.add_async_action(Workspace::close);
    cx.add_async_action(Workspace::save_all);
    cx.add_action(Workspace::add_folder_to_project);
    cx.add_action(Workspace::remove_folder_from_project);
    cx.add_action(Workspace::toggle_project_online);
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
        workspace.toggle_sidebar(Side::Left, cx);
    });
    cx.add_action(|workspace: &mut Workspace, _: &ToggleRightSidebar, cx| {
        workspace.toggle_sidebar(Side::Right, cx);
    });
    cx.add_action(Workspace::activate_pane_at_index);

    let client = &app_state.client;
    client.add_view_request_handler(Workspace::handle_follow);
    client.add_view_message_handler(Workspace::handle_unfollow);
    client.add_view_message_handler(Workspace::handle_update_followers);
}

pub fn register_project_item<I: ProjectItem>(cx: &mut MutableAppContext) {
    cx.update_default_global(|builders: &mut ProjectItemBuilders, _| {
        builders.insert(TypeId::of::<I::Item>(), |project, model, cx| {
            let item = model.downcast::<I::Item>().unwrap();
            Box::new(cx.add_view(|cx| I::for_project_item(project, item, cx)))
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
    pub project_store: ModelHandle<ProjectStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub build_window_options: fn() -> WindowOptions<'static>,
    pub initialize_workspace: fn(&mut Workspace, &Arc<AppState>, &mut ViewContext<Workspace>),
}

#[derive(Eq, PartialEq, Hash)]
pub enum ItemEvent {
    CloseItem,
    UpdateTab,
    UpdateBreadcrumbs,
    Edit,
}

pub trait Item: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn workspace_deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn tab_description<'a>(&'a self, _: usize, _: &'a AppContext) -> Option<Cow<'a, str>> {
        None
    }
    fn tab_content(&self, detail: Option<usize>, style: &theme::Tab, cx: &AppContext)
        -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn is_singleton(&self, cx: &AppContext) -> bool;
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
    fn to_item_events(event: &Self::Event) -> Vec<ItemEvent>;
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
    fn as_searchable(&self, _: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::Hidden
    }
    fn breadcrumbs(&self, _theme: &Theme, _cx: &AppContext) -> Option<Vec<ElementBox>> {
        None
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
    fn subscribe_to_item_events(
        &self,
        cx: &mut MutableAppContext,
        handler: Box<dyn Fn(ItemEvent, &mut MutableAppContext)>,
    ) -> gpui::Subscription;
    fn tab_description<'a>(&self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>>;
    fn tab_content(&self, detail: Option<usize>, style: &theme::Tab, cx: &AppContext)
        -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn is_singleton(&self, cx: &AppContext) -> bool;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    );
    fn deactivated(&self, cx: &mut MutableAppContext);
    fn workspace_deactivated(&self, cx: &mut MutableAppContext);
    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool;
    fn id(&self) -> usize;
    fn window_id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
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
    fn to_searchable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>>;
    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation;
    fn breadcrumbs(&self, theme: &Theme, cx: &AppContext) -> Option<Vec<ElementBox>>;
}

pub trait WeakItemHandle {
    fn id(&self) -> usize;
    fn window_id(&self) -> usize;
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
    fn subscribe_to_item_events(
        &self,
        cx: &mut MutableAppContext,
        handler: Box<dyn Fn(ItemEvent, &mut MutableAppContext)>,
    ) -> gpui::Subscription {
        cx.subscribe(self, move |_, event, cx| {
            for item_event in T::to_item_events(event) {
                handler(item_event, cx)
            }
        })
    }

    fn tab_description<'a>(&self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>> {
        self.read(cx).tab_description(detail, cx)
    }

    fn tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> ElementBox {
        self.read(cx).tab_content(detail, style, cx)
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path(cx)
    }

    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]> {
        self.read(cx).project_entry_ids(cx)
    }

    fn is_singleton(&self, cx: &AppContext) -> bool {
        self.read(cx).is_singleton(cx)
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

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let history = pane.read(cx).nav_history_for_item(self);
        self.update(cx, |this, cx| this.set_nav_history(history, cx));

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

        if workspace
            .panes_by_item
            .insert(self.id(), pane.downgrade())
            .is_none()
        {
            let mut pending_autosave = None;
            let mut cancel_pending_autosave = oneshot::channel::<()>().0;
            let pending_update = Rc::new(RefCell::new(None));
            let pending_update_scheduled = Rc::new(AtomicBool::new(false));

            let mut event_subscription =
                Some(cx.subscribe(self, move |workspace, item, event, cx| {
                    let pane = if let Some(pane) = workspace
                        .panes_by_item
                        .get(&item.id())
                        .and_then(|pane| pane.upgrade(cx))
                    {
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

                        if item.add_event_to_update_proto(
                            event,
                            &mut *pending_update.borrow_mut(),
                            cx,
                        ) && !pending_update_scheduled.load(SeqCst)
                        {
                            pending_update_scheduled.store(true, SeqCst);
                            cx.after_window_update({
                                let pending_update = pending_update.clone();
                                let pending_update_scheduled = pending_update_scheduled.clone();
                                move |this, cx| {
                                    pending_update_scheduled.store(false, SeqCst);
                                    this.update_followers(
                                        proto::update_followers::Variant::UpdateView(
                                            proto::UpdateView {
                                                id: item.id() as u64,
                                                variant: pending_update.borrow_mut().take(),
                                                leader_id: leader_id.map(|id| id.0),
                                            },
                                        ),
                                        cx,
                                    );
                                }
                            });
                        }
                    }

                    for item_event in T::to_item_events(event).into_iter() {
                        match item_event {
                            ItemEvent::CloseItem => {
                                Pane::close_item(workspace, pane, item.id(), cx)
                                    .detach_and_log_err(cx);
                                return;
                            }
                            ItemEvent::UpdateTab => {
                                pane.update(cx, |_, cx| {
                                    cx.emit(pane::Event::ChangeItemTitle);
                                    cx.notify();
                                });
                            }
                            ItemEvent::Edit => {
                                if let Autosave::AfterDelay { milliseconds } =
                                    cx.global::<Settings>().autosave
                                {
                                    let prev_autosave = pending_autosave
                                        .take()
                                        .unwrap_or_else(|| Task::ready(Some(())));
                                    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
                                    let prev_cancel_tx =
                                        mem::replace(&mut cancel_pending_autosave, cancel_tx);
                                    let project = workspace.project.downgrade();
                                    let _ = prev_cancel_tx.send(());
                                    let item = item.clone();
                                    pending_autosave =
                                        Some(cx.spawn_weak(|_, mut cx| async move {
                                            let mut timer = cx
                                                .background()
                                                .timer(Duration::from_millis(milliseconds))
                                                .fuse();
                                            prev_autosave.await;
                                            futures::select_biased! {
                                                _ = cancel_rx => return None,
                                                    _ = timer => {}
                                            }

                                            let project = project.upgrade(&cx)?;
                                            cx.update(|cx| Pane::autosave_item(&item, project, cx))
                                                .await
                                                .log_err();
                                            None
                                        }));
                                }
                            }
                            _ => {}
                        }
                    }
                }));

            cx.observe_focus(self, move |workspace, item, focused, cx| {
                if !focused && cx.global::<Settings>().autosave == Autosave::OnFocusChange {
                    Pane::autosave_item(&item, workspace.project.clone(), cx)
                        .detach_and_log_err(cx);
                }
            })
            .detach();

            let item_id = self.id();
            cx.observe_release(self, move |workspace, _, _| {
                workspace.panes_by_item.remove(&item_id);
                event_subscription.take();
            })
            .detach();
        }
    }

    fn deactivated(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.deactivated(cx));
    }

    fn workspace_deactivated(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.workspace_deactivated(cx));
    }

    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool {
        self.update(cx, |this, cx| this.navigate(data, cx))
    }

    fn id(&self) -> usize {
        self.id()
    }

    fn window_id(&self) -> usize {
        self.window_id()
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

    fn to_searchable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>> {
        self.read(cx).as_searchable(self)
    }

    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation {
        self.read(cx).breadcrumb_location()
    }

    fn breadcrumbs(&self, theme: &Theme, cx: &AppContext) -> Option<Vec<ElementBox>> {
        self.read(cx).breadcrumbs(theme, cx)
    }
}

impl From<Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: Box<dyn ItemHandle>) -> Self {
        val.to_any()
    }
}

impl From<&Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: &Box<dyn ItemHandle>) -> Self {
        val.to_any()
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

    fn window_id(&self) -> usize {
        self.window_id()
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

impl From<&dyn NotificationHandle> for AnyViewHandle {
    fn from(val: &dyn NotificationHandle) -> Self {
        val.to_any()
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
        let project_store = cx.add_model(|_| ProjectStore::new(project::Db::open_fake()));
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let themes = ThemeRegistry::new((), cx.font_cache().clone(), false);
        Arc::new(Self {
            client,
            themes,
            fs,
            languages,
            user_store,
            project_store,
            initialize_workspace: |_, _, _| {},
            build_window_options: Default::default,
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
    panes_by_item: HashMap<usize, WeakViewHandle<Pane>>,
    active_pane: ViewHandle<Pane>,
    status_bar: ViewHandle<StatusBar>,
    notifications: Vec<(TypeId, usize, Box<dyn NotificationHandle>)>,
    project: ModelHandle<Project>,
    leader_state: LeaderState,
    follower_states_by_leader: FollowerStatesByLeader,
    last_leaders_by_pane: HashMap<WeakViewHandle<Pane>, PeerId>,
    window_edited: bool,
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
        cx.observe_fullscreen(|_, _, cx| cx.notify()).detach();

        cx.observe_window_activation(Self::on_window_activation_changed)
            .detach();
        cx.observe(&project, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&project, move |this, _, event, cx| {
            match event {
                project::Event::RemoteIdChanged(remote_id) => {
                    this.project_remote_id_changed(*remote_id, cx);
                }
                project::Event::CollaboratorLeft(peer_id) => {
                    this.collaborator_left(*peer_id, cx);
                }
                project::Event::WorktreeRemoved(_) | project::Event::WorktreeAdded => {
                    this.update_window_title(cx);
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

        let pane = cx.add_view(Pane::new);
        let pane_id = pane.id();
        cx.subscribe(&pane, move |this, _, event, cx| {
            this.handle_pane_event(pane_id, event, cx)
        })
        .detach();
        cx.focus(&pane);
        cx.emit(Event::PaneAdded(pane.clone()));

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

        cx.update_default_global::<DragAndDrop<Workspace>, _, _>(|drag_and_drop, _| {
            drag_and_drop.register_container(weak_self.clone());
        });

        let mut this = Workspace {
            modal: None,
            weak_self,
            center: PaneGroup::new(pane.clone()),
            panes: vec![pane.clone()],
            panes_by_item: Default::default(),
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
            window_edited: false,
            _observe_current_user,
        };
        this.project_remote_id_changed(this.project.read(cx).remote_id(), cx);
        cx.defer(|this, cx| this.update_window_title(cx));

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

    /// Call the given callback with a workspace whose project is local.
    ///
    /// If the given workspace has a local project, then it will be passed
    /// to the callback. Otherwise, a new empty window will be created.
    pub fn with_local_workspace<T, F>(
        &mut self,
        cx: &mut ViewContext<Self>,
        app_state: Arc<AppState>,
        callback: F,
    ) -> T
    where
        T: 'static,
        F: FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
    {
        if self.project.read(cx).is_local() {
            callback(self, cx)
        } else {
            let (_, workspace) = cx.add_window((app_state.build_window_options)(), |cx| {
                let mut workspace = Workspace::new(
                    Project::local(
                        false,
                        app_state.client.clone(),
                        app_state.user_store.clone(),
                        app_state.project_store.clone(),
                        app_state.languages.clone(),
                        app_state.fs.clone(),
                        cx,
                    ),
                    cx,
                );
                (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                workspace
            });
            workspace.update(cx, callback)
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

    pub fn close(
        &mut self,
        _: &CloseWindow,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let prepare = self.prepare_to_close(cx);
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

    pub fn prepare_to_close(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<bool>> {
        self.save_all_internal(true, cx)
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
    ) -> Task<Vec<Option<Result<Box<dyn ItemHandle>, Arc<anyhow::Error>>>>> {
        let fs = self.fs.clone();

        // Sort the paths to ensure we add worktrees for parents before their children.
        abs_paths.sort_unstable();
        cx.spawn(|this, mut cx| async move {
            let mut project_paths = Vec::new();
            for path in &abs_paths {
                project_paths.push(
                    this.update(&mut cx, |this, cx| {
                        this.project_path_for_path(path, visible, cx)
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
        self.project
            .update(cx, |project, cx| project.remove_worktree(*worktree_id, cx));
    }

    fn toggle_project_online(&mut self, action: &ToggleProjectOnline, cx: &mut ViewContext<Self>) {
        let project = action
            .project
            .clone()
            .unwrap_or_else(|| self.project.clone());
        project.update(cx, |project, cx| {
            let public = !project.is_online();
            project.set_online(public, cx);
        });
    }

    fn project_path_for_path(
        &self,
        abs_path: &Path,
        visible: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, ProjectPath)>> {
        let entry = self.project().update(cx, |project, cx| {
            project.find_or_create_local_worktree(abs_path, visible, cx)
        });
        cx.spawn(|_, cx| async move {
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

    pub fn toggle_sidebar(&mut self, side: Side, cx: &mut ViewContext<Self>) {
        let sidebar = match side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        sidebar.update(cx, |sidebar, cx| {
            sidebar.set_open(!sidebar.is_open(), cx);
        });
        cx.focus_self();
        cx.notify();
    }

    pub fn toggle_sidebar_item(&mut self, action: &ToggleSidebarItem, cx: &mut ViewContext<Self>) {
        let sidebar = match action.side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, |sidebar, cx| {
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
            if active_item.is_focused(cx) {
                cx.focus_self();
            } else {
                cx.focus(active_item.to_any());
            }
        } else {
            cx.focus_self();
        }
        cx.notify();
    }

    pub fn toggle_sidebar_item_focus(
        &mut self,
        side: Side,
        item_index: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let sidebar = match side {
            Side::Left => &mut self.left_sidebar,
            Side::Right => &mut self.right_sidebar,
        };
        let active_item = sidebar.update(cx, |sidebar, cx| {
            sidebar.set_open(true, cx);
            sidebar.activate_item(item_index, cx);
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

    pub fn focus_center(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.focus_self();
        cx.notify();
    }

    fn add_pane(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<Pane> {
        let pane = cx.add_view(Pane::new);
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

    pub fn add_item(&mut self, item: Box<dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        let active_pane = self.active_pane().clone();
        Pane::add_item(self, &active_pane, item, true, true, None, cx);
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
        let next_pane = {
            let panes = self.center.panes();
            let ix = panes
                .iter()
                .position(|pane| **pane == self.active_pane)
                .unwrap();
            let next_ix = (ix + 1) % panes.len();
            panes[next_ix].clone()
        };
        cx.focus(next_pane);
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
        cx.focus(prev_pane);
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
                pane::Event::Focused => {
                    self.handle_pane_focused(pane, cx);
                }
                pane::Event::ActivateItem { local } => {
                    if *local {
                        self.unfollow(&pane, cx);
                    }
                    if pane == self.active_pane {
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
    ) -> Option<ViewHandle<Pane>> {
        pane.read(cx).active_item().map(|item| {
            let new_pane = self.add_pane(cx);
            if let Some(clone) = item.clone_on_split(cx.as_mut()) {
                Pane::add_item(self, &new_pane, clone, true, true, None, cx);
            }
            self.center.split(&pane, &new_pane, direction).unwrap();
            cx.notify();
            new_pane
        })
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
            | client::Status::Reauthenticating { .. }
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                Container::new(
                    Align::new(
                        ConstrainedBox::new(
                            Svg::new("icons/cloud_slash_12.svg")
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
        let project = &self.project.read(cx);
        let replica_id = project.replica_id();
        let mut worktree_root_names = String::new();
        for (i, name) in project.worktree_root_names(cx).enumerate() {
            if i > 0 {
                worktree_root_names.push_str(", ");
            }
            worktree_root_names.push_str(name);
        }

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

        ConstrainedBox::new(
            MouseEventHandler::new::<Self, _, _>(0, cx, |_, cx| {
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
                                        replica_id,
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
        let mut title = String::new();
        let project = self.project().read(cx);
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
                    Some((collaborator.peer_id, &collaborator.user.github_login)),
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
                .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(Authenticate))
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
        peer: Option<(PeerId, &str)>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let replica_color = theme.editor.replica_selection_style(replica_id).cursor;
        let is_followed = peer.map_or(false, |(peer_id, _)| {
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

        if let Some((peer_id, peer_github_login)) = peer {
            MouseEventHandler::new::<ToggleFollow, _, _>(replica_id.into(), cx, move |_, _| content)
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(ToggleFollow(peer_id))
                })
                .with_tooltip::<ToggleFollow, _>(
                    peer_id.0 as usize,
                    if is_followed {
                        format!("Unfollow {}", peer_github_login)
                    } else {
                        format!("Follow {}", peer_github_login)
                    },
                    Some(Box::new(FollowNextCollaborator)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed()
        } else {
            content
        }
    }

    fn render_disconnected_overlay(&self, cx: &mut RenderContext<Workspace>) -> Option<ElementBox> {
        if self.project.read(cx).is_read_only() {
            Some(
                MouseEventHandler::new::<Workspace, _, _>(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    Label::new(
                        "Your connection to the remote project has been lost.".to_string(),
                        theme.workspace.disconnected_overlay.text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(theme.workspace.disconnected_overlay.container)
                    .boxed()
                })
                .capture_all()
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
            if let Some(FollowerItem::Loaded(item)) = state
                .active_view_id
                .and_then(|id| state.items_by_leader_view_id.get(&id))
            {
                items_to_add.push((pane.clone(), item.boxed_clone()));
            }
        }

        for (pane, item) in items_to_add {
            Pane::add_item(self, &pane, item.boxed_clone(), false, false, None, cx);
            if pane == self.active_pane {
                pane.update(cx, |pane, cx| pane.focus_active_item(cx));
            }
            cx.notify();
        }
        None
    }

    pub fn on_window_activation_changed(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if !active {
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
            .with_children(DragAndDrop::render(cx))
            .with_children(self.render_disconnected_overlay(cx))
            .named("workspace")
    }

    fn on_focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.active_pane);
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

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::MeasurementContext,
    ) -> Option<RectF> {
        None
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

#[allow(clippy::type_complexity)]
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
    let existing =
        activate_workspace_for_project(cx, |project, cx| project.contains_paths(abs_paths, cx));

    let app_state = app_state.clone();
    let abs_paths = abs_paths.to_vec();
    cx.spawn(|mut cx| async move {
        let mut new_project = None;
        let workspace = if let Some(existing) = existing {
            existing
        } else {
            let contains_directory =
                futures::future::join_all(abs_paths.iter().map(|path| app_state.fs.is_file(path)))
                    .await
                    .contains(&false);

            cx.add_window((app_state.build_window_options)(), |cx| {
                let project = Project::local(
                    false,
                    app_state.client.clone(),
                    app_state.user_store.clone(),
                    app_state.project_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    cx,
                );
                new_project = Some(project.clone());
                let mut workspace = Workspace::new(project, cx);
                (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                if contains_directory {
                    workspace.toggle_sidebar(Side::Left, cx);
                }
                workspace
            })
            .1
        };

        let items = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.open_paths(abs_paths, true, cx)
            })
            .await;

        if let Some(project) = new_project {
            project
                .update(&mut cx, |project, cx| project.restore_state(cx))
                .await
                .log_err();
        }

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
                false,
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.project_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx,
            ),
            cx,
        );
        (app_state.initialize_workspace)(&mut workspace, app_state, cx);
        workspace
    });
    cx.dispatch_action_at(window_id, workspace.id(), NewFile);
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use gpui::{executor::Deterministic, ModelHandle, TestAppContext, ViewContext};
    use project::{FakeFs, Project, ProjectEntryId};
    use serde_json::json;

    #[gpui::test]
    async fn test_tab_disambiguation(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);

        let fs = FakeFs::new(cx.background());
        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));

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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let item1 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.project_path = Some((worktree_id, "one.txt").into());
            item
        });
        let item2 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.project_path = Some((worktree_id, "two.txt").into());
            item
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
        project.update(cx, |project, cx| {
            project.remove_worktree(worktree_id, cx);
        });
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));

        // When there are no dirty items, there's nothing to do.
        let item1 = cx.add_view(&workspace, |_| TestItem::new());
        workspace.update(cx, |w, cx| w.add_item(Box::new(item1.clone()), cx));
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(cx));
        assert!(task.await.unwrap());

        // When there are dirty untitled items, prompt to save each one. If the user
        // cancels any prompt, then abort.
        let item2 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item
        });
        let item3 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.project_entry_ids = vec![ProjectEntryId::from_proto(1)];
            item
        });
        workspace.update(cx, |w, cx| {
            w.add_item(Box::new(item2.clone()), cx);
            w.add_item(Box::new(item3.clone()), cx);
        });
        let task = workspace.update(cx, |w, cx| w.prepare_to_close(cx));
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        let item1 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.project_entry_ids = vec![ProjectEntryId::from_proto(1)];
            item
        });
        let item2 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.has_conflict = true;
            item.project_entry_ids = vec![ProjectEntryId::from_proto(2)];
            item
        });
        let item3 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.has_conflict = true;
            item.project_entry_ids = vec![ProjectEntryId::from_proto(3)];
            item
        });
        let item4 = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item
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
        pane.read_with(cx, |pane, _| {
            assert_eq!(pane.items().count(), 4);
            assert_eq!(pane.active_item().unwrap().id(), item1.id());
        });

        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item1.read(cx).save_count, 1);
            assert_eq!(item1.read(cx).save_as_count, 0);
            assert_eq!(item1.read(cx).reload_count, 0);
            assert_eq!(pane.items().count(), 3);
            assert_eq!(pane.active_item().unwrap().id(), item3.id());
        });

        cx.simulate_prompt_answer(window_id, 1);
        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item3.read(cx).save_count, 0);
            assert_eq!(item3.read(cx).save_as_count, 0);
            assert_eq!(item3.read(cx).reload_count, 1);
            assert_eq!(pane.items().count(), 2);
            assert_eq!(pane.active_item().unwrap().id(), item4.id());
        });

        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();
        cx.simulate_new_path_selection(|_| Some(Default::default()));
        close_items.await.unwrap();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item4.read(cx).save_count, 0);
            assert_eq!(item4.read(cx).save_as_count, 1);
            assert_eq!(item4.read(cx).reload_count, 0);
            assert_eq!(pane.items().count(), 1);
            assert_eq!(pane.active_item().unwrap().id(), item2.id());
        });
    }

    #[gpui::test]
    async fn test_prompting_to_save_only_on_last_item_for_entry(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        // Create several workspace items with single project entries, and two
        // workspace items with multiple project entries.
        let single_entry_items = (0..=4)
            .map(|project_entry_id| {
                let mut item = TestItem::new();
                item.is_dirty = true;
                item.project_entry_ids = vec![ProjectEntryId::from_proto(project_entry_id)];
                item.is_singleton = true;
                item
            })
            .collect::<Vec<_>>();
        let item_2_3 = {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.is_singleton = false;
            item.project_entry_ids =
                vec![ProjectEntryId::from_proto(2), ProjectEntryId::from_proto(3)];
            item
        };
        let item_3_4 = {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.is_singleton = false;
            item.project_entry_ids =
                vec![ProjectEntryId::from_proto(3), ProjectEntryId::from_proto(4)];
            item
        };

        // Create two panes that contain the following project entries:
        //   left pane:
        //     multi-entry items:   (2, 3)
        //     single-entry items:  0, 1, 2, 3, 4
        //   right pane:
        //     single-entry items:  1
        //     multi-entry items:   (3, 4)
        let left_pane = workspace.update(cx, |workspace, cx| {
            let left_pane = workspace.active_pane().clone();
            workspace.add_item(Box::new(cx.add_view(|_| item_2_3.clone())), cx);
            for item in &single_entry_items {
                workspace.add_item(Box::new(cx.add_view(|_| item.clone())), cx);
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
            workspace.add_item(Box::new(cx.add_view(|_| item_3_4.clone())), cx);
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
            assert_eq!(pane.items().count(), 0);
        });
    }

    #[gpui::test]
    async fn test_autosave(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
        deterministic.forbid_parking();

        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        let item = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.project_entry_ids = vec![ProjectEntryId::from_proto(1)];
            item
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
            item.project_entry_ids = Default::default();
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
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        let item = cx.add_view(&workspace, |_| {
            let mut item = TestItem::new();
            item.project_entry_ids = vec![ProjectEntryId::from_proto(1)];
            item
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

    pub struct TestItem {
        state: String,
        pub label: String,
        save_count: usize,
        save_as_count: usize,
        reload_count: usize,
        is_dirty: bool,
        is_singleton: bool,
        has_conflict: bool,
        project_entry_ids: Vec<ProjectEntryId>,
        project_path: Option<ProjectPath>,
        nav_history: Option<ItemNavHistory>,
        tab_descriptions: Option<Vec<&'static str>>,
        tab_detail: Cell<Option<usize>>,
    }

    pub enum TestItemEvent {
        Edit,
    }

    impl Clone for TestItem {
        fn clone(&self) -> Self {
            Self {
                state: self.state.clone(),
                label: self.label.clone(),
                save_count: self.save_count,
                save_as_count: self.save_as_count,
                reload_count: self.reload_count,
                is_dirty: self.is_dirty,
                is_singleton: self.is_singleton,
                has_conflict: self.has_conflict,
                project_entry_ids: self.project_entry_ids.clone(),
                project_path: self.project_path.clone(),
                nav_history: None,
                tab_descriptions: None,
                tab_detail: Default::default(),
            }
        }
    }

    impl TestItem {
        pub fn new() -> Self {
            Self {
                state: String::new(),
                label: String::new(),
                save_count: 0,
                save_as_count: 0,
                reload_count: 0,
                is_dirty: false,
                has_conflict: false,
                project_entry_ids: Vec::new(),
                project_path: None,
                is_singleton: true,
                nav_history: None,
                tab_descriptions: None,
                tab_detail: Default::default(),
            }
        }

        pub fn with_label(mut self, state: &str) -> Self {
            self.label = state.to_string();
            self
        }

        pub fn with_singleton(mut self, singleton: bool) -> Self {
            self.is_singleton = singleton;
            self
        }

        pub fn with_project_entry_ids(mut self, project_entry_ids: &[u64]) -> Self {
            self.project_entry_ids.extend(
                project_entry_ids
                    .iter()
                    .copied()
                    .map(ProjectEntryId::from_proto),
            );
            self
        }

        fn set_state(&mut self, state: String, cx: &mut ViewContext<Self>) {
            self.push_to_nav_history(cx);
            self.state = state;
        }

        fn push_to_nav_history(&mut self, cx: &mut ViewContext<Self>) {
            if let Some(history) = &mut self.nav_history {
                history.push(Some(Box::new(self.state.clone())), cx);
            }
        }
    }

    impl Entity for TestItem {
        type Event = TestItemEvent;
    }

    impl View for TestItem {
        fn ui_name() -> &'static str {
            "TestItem"
        }

        fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
            Empty::new().boxed()
        }
    }

    impl Item for TestItem {
        fn tab_description<'a>(&'a self, detail: usize, _: &'a AppContext) -> Option<Cow<'a, str>> {
            self.tab_descriptions.as_ref().and_then(|descriptions| {
                let description = *descriptions.get(detail).or_else(|| descriptions.last())?;
                Some(description.into())
            })
        }

        fn tab_content(&self, detail: Option<usize>, _: &theme::Tab, _: &AppContext) -> ElementBox {
            self.tab_detail.set(detail);
            Empty::new().boxed()
        }

        fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
            self.project_path.clone()
        }

        fn project_entry_ids(&self, _: &AppContext) -> SmallVec<[ProjectEntryId; 3]> {
            self.project_entry_ids.iter().copied().collect()
        }

        fn is_singleton(&self, _: &AppContext) -> bool {
            self.is_singleton
        }

        fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
            self.nav_history = Some(history);
        }

        fn navigate(&mut self, state: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
            let state = *state.downcast::<String>().unwrap_or_default();
            if state != self.state {
                self.state = state;
                true
            } else {
                false
            }
        }

        fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
            self.push_to_nav_history(cx);
        }

        fn clone_on_split(&self, _: &mut ViewContext<Self>) -> Option<Self>
        where
            Self: Sized,
        {
            Some(self.clone())
        }

        fn is_dirty(&self, _: &AppContext) -> bool {
            self.is_dirty
        }

        fn has_conflict(&self, _: &AppContext) -> bool {
            self.has_conflict
        }

        fn can_save(&self, _: &AppContext) -> bool {
            !self.project_entry_ids.is_empty()
        }

        fn save(
            &mut self,
            _: ModelHandle<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn save_as(
            &mut self,
            _: ModelHandle<Project>,
            _: std::path::PathBuf,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_as_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn reload(
            &mut self,
            _: ModelHandle<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.reload_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn to_item_events(_: &Self::Event) -> Vec<ItemEvent> {
            vec![ItemEvent::UpdateTab, ItemEvent::Edit]
        }
    }
}
