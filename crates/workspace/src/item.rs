use crate::{
    pane, persistence::model::ItemId, searchable::SearchableItemHandle, FollowableItemBuilders,
    ItemNavHistory, Pane, ToolbarItemLocation, ViewId, Workspace, WorkspaceId,
};
use crate::{AutosaveSetting, DelayedDebouncedEditAction, WorkspaceSettings};
use anyhow::Result;
use client::{proto, Client};
use gpui::geometry::vector::Vector2F;
use gpui::{
    fonts::HighlightStyle, AnyElement, AnyViewHandle, AppContext, ModelHandle, Task, View,
    ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use project::{Project, ProjectEntryId, ProjectPath};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cell::RefCell,
    fmt,
    ops::Range,
    path::PathBuf,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use theme::Theme;

#[derive(Deserialize)]
pub struct ItemSettings {
    pub git_status: bool,
    pub close_position: ClosePosition,
    pub visible: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ClosePosition {
    Left,
    #[default]
    Right,
}

impl ClosePosition {
    pub fn right(&self) -> bool {
        match self {
            ClosePosition::Left => false,
            ClosePosition::Right => true,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ItemSettingsContent {
    git_status: Option<bool>,
    close_position: Option<ClosePosition>,
    visible: Option<bool>,
}

impl Setting for ItemSettings {
    const KEY: Option<&'static str> = Some("tabs");

    type FileContent = ItemSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ItemEvent {
    CloseItem,
    UpdateTab,
    UpdateBreadcrumbs,
    Edit,
}

// TODO: Combine this with existing HighlightedText struct?
pub struct BreadcrumbText {
    pub text: String,
    pub highlights: Option<Vec<(Range<usize>, HighlightStyle)>>,
}

pub trait Item: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn workspace_deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<Cow<str>> {
        None
    }
    fn tab_description<'a>(&'a self, _: usize, _: &'a AppContext) -> Option<Cow<str>> {
        None
    }
    fn tab_content<V: View>(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<V>;
    fn for_each_project_item(&self, _: &AppContext, _: &mut dyn FnMut(usize, &dyn project::Item)) {} // (model id, Item)
    fn is_singleton(&self, _cx: &AppContext) -> bool {
        false
    }
    fn set_nav_history(&mut self, _: ItemNavHistory, _: &mut ViewContext<Self>) {}
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
    fn can_save(&self, _cx: &AppContext) -> bool {
        false
    }
    fn save(
        &mut self,
        _project: ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save() must be implemented if can_save() returns true")
    }
    fn save_as(
        &mut self,
        _project: ModelHandle<Project>,
        _abs_path: PathBuf,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save_as() must be implemented if can_save() returns true")
    }
    fn reload(
        &mut self,
        _project: ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("reload() must be implemented if can_save() returns true")
    }
    fn to_item_events(_event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        SmallVec::new()
    }
    fn should_close_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
    fn is_edit_event(_: &Self::Event) -> bool {
        false
    }
    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a ViewHandle<Self>,
        _: &'a AppContext,
    ) -> Option<&AnyViewHandle> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle)
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

    fn breadcrumbs(&self, _theme: &Theme, _cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        None
    }

    fn added_to_workspace(&mut self, _workspace: &mut Workspace, _cx: &mut ViewContext<Self>) {}

    fn serialized_item_kind() -> Option<&'static str> {
        None
    }

    fn deserialize(
        _project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        _cx: &mut ViewContext<Pane>,
    ) -> Task<Result<ViewHandle<Self>>> {
        unimplemented!(
            "deserialize() must be implemented if serialized_item_kind() returns Some(_)"
        )
    }
    fn show_toolbar(&self) -> bool {
        true
    }
    fn pixel_position_of_cursor(&self) -> Option<Vector2F> {
        None
    }
}

pub trait ItemHandle: 'static + fmt::Debug {
    fn subscribe_to_item_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(ItemEvent, &mut WindowContext)>,
    ) -> gpui::Subscription;
    fn tab_tooltip_text<'a>(&self, cx: &'a AppContext) -> Option<Cow<'a, str>>;
    fn tab_description<'a>(&'a self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>>;
    fn tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<Pane>;
    fn dragged_tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<Workspace>;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn project_item_model_ids(&self, cx: &AppContext) -> SmallVec<[usize; 3]>;
    fn for_each_project_item(&self, _: &AppContext, _: &mut dyn FnMut(usize, &dyn project::Item));
    fn is_singleton(&self, cx: &AppContext) -> bool;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn clone_on_split(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut WindowContext,
    ) -> Option<Box<dyn ItemHandle>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    );
    fn deactivated(&self, cx: &mut WindowContext);
    fn workspace_deactivated(&self, cx: &mut WindowContext);
    fn navigate(&self, data: Box<dyn Any>, cx: &mut WindowContext) -> bool;
    fn id(&self) -> usize;
    fn window_id(&self) -> usize;
    fn as_any(&self) -> &AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
    fn save(&self, project: ModelHandle<Project>, cx: &mut WindowContext) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;
    fn reload(&self, project: ModelHandle<Project>, cx: &mut WindowContext) -> Task<Result<()>>;
    fn act_as_type<'a>(&'a self, type_id: TypeId, cx: &'a AppContext) -> Option<&'a AnyViewHandle>;
    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut AppContext,
        callback: Box<dyn FnOnce(&mut AppContext)>,
    ) -> gpui::Subscription;
    fn to_searchable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>>;
    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation;
    fn breadcrumbs(&self, theme: &Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>>;
    fn serialized_item_kind(&self) -> Option<&'static str>;
    fn show_toolbar(&self, cx: &AppContext) -> bool;
    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Vector2F>;
}

pub trait WeakItemHandle {
    fn id(&self) -> usize;
    fn window_id(&self) -> usize;
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>>;
}

impl dyn ItemHandle {
    pub fn downcast<T: View>(&self) -> Option<ViewHandle<T>> {
        self.as_any().clone().downcast()
    }

    pub fn act_as<T: View>(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        self.act_as_type(TypeId::of::<T>(), cx)
            .and_then(|t| t.clone().downcast())
    }
}

impl<T: Item> ItemHandle for ViewHandle<T> {
    fn subscribe_to_item_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(ItemEvent, &mut WindowContext)>,
    ) -> gpui::Subscription {
        cx.subscribe(self, move |_, event, cx| {
            for item_event in T::to_item_events(event) {
                handler(item_event, cx)
            }
        })
    }

    fn tab_tooltip_text<'a>(&self, cx: &'a AppContext) -> Option<Cow<'a, str>> {
        self.read(cx).tab_tooltip_text(cx)
    }

    fn tab_description<'a>(&'a self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>> {
        self.read(cx).tab_description(detail, cx)
    }

    fn tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<Pane> {
        self.read(cx).tab_content(detail, style, cx)
    }

    fn dragged_tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<Workspace> {
        self.read(cx).tab_content(detail, style, cx)
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        let this = self.read(cx);
        let mut result = None;
        if this.is_singleton(cx) {
            this.for_each_project_item(cx, &mut |_, item| {
                result = item.project_path(cx);
            });
        }
        result
    }

    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |_, item| {
            if let Some(id) = item.entry_id(cx) {
                result.push(id);
            }
        });
        result
    }

    fn project_item_model_ids(&self, cx: &AppContext) -> SmallVec<[usize; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |id, _| {
            result.push(id);
        });
        result
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
        self.read(cx).for_each_project_item(cx, f)
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
        cx: &mut WindowContext,
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
                        id: followed_item
                            .remote_id(&workspace.app_state.client, cx)
                            .map(|id| id.to_proto()),
                        variant: Some(message),
                        leader_id: workspace.leader_for_pane(&pane),
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
                                                id: item
                                                    .remote_id(&this.app_state.client, cx)
                                                    .map(|id| id.to_proto()),
                                                variant: pending_update.borrow_mut().take(),
                                                leader_id,
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
                                pane.update(cx, |pane, cx| pane.close_item_by_id(item.id(), cx))
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
                                let autosave = settings::get::<WorkspaceSettings>(cx).autosave;
                                if let AutosaveSetting::AfterDelay { milliseconds } = autosave {
                                    let delay = Duration::from_millis(milliseconds);
                                    let item = item.clone();
                                    pending_autosave.fire_new(delay, cx, move |workspace, cx| {
                                        Pane::autosave_item(&item, workspace.project().clone(), cx)
                                    });
                                }
                            }

                            _ => {}
                        }
                    }
                }));

            cx.observe_focus(self, move |workspace, item, focused, cx| {
                if !focused
                    && settings::get::<WorkspaceSettings>(cx).autosave
                        == AutosaveSetting::OnFocusChange
                {
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

    fn deactivated(&self, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.deactivated(cx));
    }

    fn workspace_deactivated(&self, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.workspace_deactivated(cx));
    }

    fn navigate(&self, data: Box<dyn Any>, cx: &mut WindowContext) -> bool {
        self.update(cx, |this, cx| this.navigate(data, cx))
    }

    fn id(&self) -> usize {
        self.id()
    }

    fn window_id(&self) -> usize {
        self.window_id()
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
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

    fn save(&self, project: ModelHandle<Project>, cx: &mut WindowContext) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.save(project, cx))
    }

    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut WindowContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(project, abs_path, cx))
    }

    fn reload(&self, project: ModelHandle<Project>, cx: &mut WindowContext) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.reload(project, cx))
    }

    fn act_as_type<'a>(&'a self, type_id: TypeId, cx: &'a AppContext) -> Option<&'a AnyViewHandle> {
        self.read(cx).act_as_type(type_id, self, cx)
    }

    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>> {
        if cx.has_global::<FollowableItemBuilders>() {
            let builders = cx.global::<FollowableItemBuilders>();
            let item = self.as_any();
            Some(builders.get(&item.view_type())?.1(item))
        } else {
            None
        }
    }

    fn on_release(
        &self,
        cx: &mut AppContext,
        callback: Box<dyn FnOnce(&mut AppContext)>,
    ) -> gpui::Subscription {
        cx.observe_release(self, move |_, cx| callback(cx))
    }

    fn to_searchable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>> {
        self.read(cx).as_searchable(self)
    }

    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation {
        self.read(cx).breadcrumb_location()
    }

    fn breadcrumbs(&self, theme: &Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        self.read(cx).breadcrumbs(theme, cx)
    }

    fn serialized_item_kind(&self) -> Option<&'static str> {
        T::serialized_item_kind()
    }

    fn show_toolbar(&self, cx: &AppContext) -> bool {
        self.read(cx).show_toolbar()
    }

    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Vector2F> {
        self.read(cx).pixel_position_of_cursor()
    }
}

impl From<Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: Box<dyn ItemHandle>) -> Self {
        val.as_any().clone()
    }
}

impl From<&Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: &Box<dyn ItemHandle>) -> Self {
        val.as_any().clone()
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
    type Item: project::Item + gpui::Entity;

    fn for_project_item(
        project: ModelHandle<Project>,
        item: ModelHandle<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self;
}

pub trait FollowableItem: Item {
    fn remote_id(&self) -> Option<ViewId>;
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn from_state_proto(
        pane: ViewHandle<Pane>,
        project: ModelHandle<Project>,
        id: ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut AppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>>;
    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &mut self,
        project: &ModelHandle<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;

    fn set_leader_replica_id(&mut self, leader_replica_id: Option<u16>, cx: &mut ViewContext<Self>);
    fn should_unfollow_on_event(event: &Self::Event, cx: &AppContext) -> bool;
}

pub trait FollowableItemHandle: ItemHandle {
    fn remote_id(&self, client: &Arc<Client>, cx: &AppContext) -> Option<ViewId>;
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut WindowContext);
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &self,
        project: &ModelHandle<Project>,
        message: proto::update_view::Variant,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;
    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool;
}

impl<T: FollowableItem> FollowableItemHandle for ViewHandle<T> {
    fn remote_id(&self, client: &Arc<Client>, cx: &AppContext) -> Option<ViewId> {
        self.read(cx).remote_id().or_else(|| {
            client.peer_id().map(|creator| ViewId {
                creator,
                id: self.id() as u64,
            })
        })
    }

    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut WindowContext) {
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
        project: &ModelHandle<Project>,
        message: proto::update_view::Variant,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| this.apply_update_proto(project, message, cx))
    }

    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool {
        if let Some(event) = event.downcast_ref() {
            T::should_unfollow_on_event(event, cx)
        } else {
            false
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test {
    use super::{Item, ItemEvent};
    use crate::{ItemId, ItemNavHistory, Pane, Workspace, WorkspaceId};
    use gpui::{
        elements::Empty, AnyElement, AppContext, Element, Entity, ModelHandle, Task, View,
        ViewContext, ViewHandle, WeakViewHandle,
    };
    use project::{Project, ProjectEntryId, ProjectPath, WorktreeId};
    use smallvec::SmallVec;
    use std::{any::Any, borrow::Cow, cell::Cell, path::Path};

    pub struct TestProjectItem {
        pub entry_id: Option<ProjectEntryId>,
        pub project_path: Option<ProjectPath>,
    }

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
        pub project_items: Vec<ModelHandle<TestProjectItem>>,
        pub nav_history: Option<ItemNavHistory>,
        pub tab_descriptions: Option<Vec<&'static str>>,
        pub tab_detail: Cell<Option<usize>>,
    }

    impl Entity for TestProjectItem {
        type Event = ();
    }

    impl project::Item for TestProjectItem {
        fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
            self.entry_id
        }

        fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
            self.project_path.clone()
        }
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
                project_items: self.project_items.clone(),
                nav_history: None,
                tab_descriptions: None,
                tab_detail: Default::default(),
                workspace_id: self.workspace_id,
            }
        }
    }

    impl TestProjectItem {
        pub fn new(id: u64, path: &str, cx: &mut AppContext) -> ModelHandle<Self> {
            let entry_id = Some(ProjectEntryId::from_proto(id));
            let project_path = Some(ProjectPath {
                worktree_id: WorktreeId::from_usize(0),
                path: Path::new(path).into(),
            });
            cx.add_model(|_| Self {
                entry_id,
                project_path,
            })
        }

        pub fn new_untitled(cx: &mut AppContext) -> ModelHandle<Self> {
            cx.add_model(|_| Self {
                project_path: None,
                entry_id: None,
            })
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
                project_items: Vec::new(),
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

        pub fn with_dirty(mut self, dirty: bool) -> Self {
            self.is_dirty = dirty;
            self
        }

        pub fn with_conflict(mut self, has_conflict: bool) -> Self {
            self.has_conflict = has_conflict;
            self
        }

        pub fn with_project_items(mut self, items: &[ModelHandle<TestProjectItem>]) -> Self {
            self.project_items.clear();
            self.project_items.extend(items.iter().cloned());
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

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }

    impl Item for TestItem {
        fn tab_description(&self, detail: usize, _: &AppContext) -> Option<Cow<str>> {
            self.tab_descriptions.as_ref().and_then(|descriptions| {
                let description = *descriptions.get(detail).or_else(|| descriptions.last())?;
                Some(description.into())
            })
        }

        fn tab_content<V: View>(
            &self,
            detail: Option<usize>,
            _: &theme::Tab,
            _: &AppContext,
        ) -> AnyElement<V> {
            self.tab_detail.set(detail);
            Empty::new().into_any()
        }

        fn for_each_project_item(
            &self,
            cx: &AppContext,
            f: &mut dyn FnMut(usize, &dyn project::Item),
        ) {
            self.project_items
                .iter()
                .for_each(|item| f(item.id(), item.read(cx)))
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

        fn can_save(&self, cx: &AppContext) -> bool {
            !self.project_items.is_empty()
                && self
                    .project_items
                    .iter()
                    .all(|item| item.read(cx).entry_id.is_some())
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

        fn to_item_events(_: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
            [ItemEvent::UpdateTab, ItemEvent::Edit].into()
        }

        fn serialized_item_kind() -> Option<&'static str> {
            Some("TestItem")
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
}
