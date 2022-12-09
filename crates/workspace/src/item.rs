use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cell::RefCell,
    fmt,
    path::PathBuf,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use anyhow::Result;
use client::proto;
use gpui::{
    AnyViewHandle, AppContext, ElementBox, ModelHandle, MutableAppContext, Task, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use project::{Project, ProjectEntryId, ProjectPath};
use settings::{Autosave, Settings};
use smallvec::SmallVec;
use theme::Theme;
use util::ResultExt;

use crate::{
    pane, persistence::model::ItemId, searchable::SearchableItemHandle, DelayedDebouncedEditAction,
    FollowableItemBuilders, ItemNavHistory, Pane, ToolbarItemLocation, Workspace, WorkspaceId,
};

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
    fn clone_on_split(&self, _workspace_id: WorkspaceId, _: &mut ViewContext<Self>) -> Option<Self>
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
    fn git_diff_recalc(
        &mut self,
        _project: ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
    fn to_item_events(event: &Self::Event) -> Vec<ItemEvent>;
    fn should_close_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
    fn is_edit_event(_: &Self::Event) -> bool {
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
    fn as_searchable(&self, _: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::Hidden
    }

    fn breadcrumbs(&self, _theme: &Theme, _cx: &AppContext) -> Option<Vec<ElementBox>> {
        None
    }

    fn added_to_workspace(&mut self, _workspace: &mut Workspace, _cx: &mut ViewContext<Self>) {}

    fn serialized_item_kind() -> Option<&'static str>;

    fn deserialize(
        project: ModelHandle<Project>,
        workspace: WeakViewHandle<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<Result<ViewHandle<Self>>>;
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
    fn clone_on_split(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemHandle>>;
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
    fn git_diff_recalc(
        &self,
        project: ModelHandle<Project>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>>;
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
    fn serialized_item_kind(&self) -> Option<&'static str>;
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

    fn clone_on_split(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut MutableAppContext,
    ) -> Option<Box<dyn ItemHandle>> {
        self.update(cx, |item, cx| {
            cx.add_option_view(|cx| item.clone_on_split(workspace_id, cx))
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
        self.update(cx, |this, cx| {
            this.set_nav_history(history, cx);
            this.added_to_workspace(workspace, cx);
        });

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
            let mut pending_autosave = DelayedDebouncedEditAction::new();
            let mut pending_git_update = DelayedDebouncedEditAction::new();
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
                        ) && !pending_update_scheduled.load(Ordering::SeqCst)
                        {
                            pending_update_scheduled.store(true, Ordering::SeqCst);
                            cx.after_window_update({
                                let pending_update = pending_update.clone();
                                let pending_update_scheduled = pending_update_scheduled.clone();
                                move |this, cx| {
                                    pending_update_scheduled.store(false, Ordering::SeqCst);
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
                                    let delay = Duration::from_millis(milliseconds);
                                    let item = item.clone();
                                    pending_autosave.fire_new(
                                        delay,
                                        workspace,
                                        cx,
                                        |project, mut cx| async move {
                                            cx.update(|cx| Pane::autosave_item(&item, project, cx))
                                                .await
                                                .log_err();
                                        },
                                    );
                                }

                                let settings = cx.global::<Settings>();
                                let debounce_delay = settings.git_overrides.gutter_debounce;

                                let item = item.clone();

                                if let Some(delay) = debounce_delay {
                                    const MIN_GIT_DELAY: u64 = 50;

                                    let delay = delay.max(MIN_GIT_DELAY);
                                    let duration = Duration::from_millis(delay);

                                    pending_git_update.fire_new(
                                        duration,
                                        workspace,
                                        cx,
                                        |project, mut cx| async move {
                                            cx.update(|cx| item.git_diff_recalc(project, cx))
                                                .await
                                                .log_err();
                                        },
                                    );
                                } else {
                                    let project = workspace.project().downgrade();
                                    cx.spawn_weak(|_, mut cx| async move {
                                        if let Some(project) = project.upgrade(&cx) {
                                            cx.update(|cx| item.git_diff_recalc(project, cx))
                                                .await
                                                .log_err();
                                        }
                                    })
                                    .detach();
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

        cx.defer(|workspace, cx| {
            workspace.serialize_workspace(cx);
        });
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

    fn git_diff_recalc(
        &self,
        project: ModelHandle<Project>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.git_diff_recalc(project, cx))
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

    fn serialized_item_kind(&self) -> Option<&'static str> {
        T::serialized_item_kind()
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

#[cfg(test)]
pub(crate) mod test {
    use std::{any::Any, borrow::Cow, cell::Cell};

    use gpui::{
        elements::Empty, AppContext, Element, ElementBox, Entity, ModelHandle, RenderContext, Task,
        View, ViewContext, ViewHandle, WeakViewHandle,
    };
    use project::{Project, ProjectEntryId, ProjectPath};
    use smallvec::SmallVec;

    use crate::{sidebar::SidebarItem, ItemId, ItemNavHistory, Pane, Workspace, WorkspaceId};

    use super::{Item, ItemEvent};

    pub struct TestItem {
        pub workspace_id: WorkspaceId,
        pub state: String,
        pub label: String,
        pub save_count: usize,
        pub save_as_count: usize,
        pub reload_count: usize,
        pub is_dirty: bool,
        pub is_singleton: bool,
        pub has_conflict: bool,
        pub project_entry_ids: Vec<ProjectEntryId>,
        pub project_path: Option<ProjectPath>,
        pub nav_history: Option<ItemNavHistory>,
        pub tab_descriptions: Option<Vec<&'static str>>,
        pub tab_detail: Cell<Option<usize>>,
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
                workspace_id: self.workspace_id,
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
                workspace_id: 0,
            }
        }

        pub fn new_deserialized(id: WorkspaceId) -> Self {
            let mut this = Self::new();
            this.workspace_id = id;
            this
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

        pub fn set_state(&mut self, state: String, cx: &mut ViewContext<Self>) {
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

        fn clone_on_split(
            &self,
            _workspace_id: WorkspaceId,
            _: &mut ViewContext<Self>,
        ) -> Option<Self>
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

        fn serialized_item_kind() -> Option<&'static str> {
            None
        }

        fn deserialize(
            _project: ModelHandle<Project>,
            _workspace: WeakViewHandle<Workspace>,
            workspace_id: WorkspaceId,
            _item_id: ItemId,
            cx: &mut ViewContext<Pane>,
        ) -> Task<anyhow::Result<ViewHandle<Self>>> {
            let view = cx.add_view(|_cx| Self::new_deserialized(workspace_id));
            Task::Ready(Some(anyhow::Ok(view)))
        }
    }

    impl SidebarItem for TestItem {}
}
