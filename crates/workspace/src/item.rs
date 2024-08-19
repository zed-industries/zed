use crate::{
    pane::{self, Pane},
    persistence::model::ItemId,
    searchable::SearchableItemHandle,
    workspace_settings::{AutosaveSetting, WorkspaceSettings},
    DelayedDebouncedEditAction, FollowableViewRegistry, ItemNavHistory, SerializableItemRegistry,
    ToolbarItemLocation, ViewId, Workspace, WorkspaceId,
};
use anyhow::Result;
use client::{
    proto::{self, PeerId},
    Client,
};
use futures::{channel::mpsc, StreamExt};
use gpui::{
    AnyElement, AnyView, AppContext, Entity, EntityId, EventEmitter, FocusHandle, FocusableView,
    Font, HighlightStyle, Model, Pixels, Point, SharedString, Task, View, ViewContext, WeakView,
    WindowContext,
};
use project::{Project, ProjectEntryId, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsLocation, SettingsSources};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::Range,
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use theme::Theme;
use ui::{Color, Element as _, Icon, IntoElement, Label, LabelCommon};
use util::ResultExt;

pub const LEADER_UPDATE_THROTTLE: Duration = Duration::from_millis(200);

#[derive(Deserialize)]
pub struct ItemSettings {
    pub git_status: bool,
    pub close_position: ClosePosition,
    pub file_icons: bool,
}

#[derive(Deserialize)]
pub struct PreviewTabsSettings {
    pub enabled: bool,
    pub enable_preview_from_file_finder: bool,
    pub enable_preview_from_code_navigation: bool,
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
    /// Whether to show the Git file status on a tab item.
    ///
    /// Default: false
    git_status: Option<bool>,
    /// Position of the close button in a tab.
    ///
    /// Default: right
    close_position: Option<ClosePosition>,
    /// Whether to show the file icon for a tab.
    ///
    /// Default: true
    file_icons: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct PreviewTabsSettingsContent {
    /// Whether to show opened editors as preview tabs.
    /// Preview tabs do not stay open, are reused until explicitly set to be kept open opened (via double-click or editing) and show file names in italic.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to open tabs in preview mode when selected from the file finder.
    ///
    /// Default: false
    enable_preview_from_file_finder: Option<bool>,
    /// Whether a preview tab gets replaced when code navigation is used to navigate away from the tab.
    ///
    /// Default: false
    enable_preview_from_code_navigation: Option<bool>,
}

impl Settings for ItemSettings {
    const KEY: Option<&'static str> = Some("tabs");

    type FileContent = ItemSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

impl Settings for PreviewTabsSettings {
    const KEY: Option<&'static str> = Some("preview_tabs");

    type FileContent = PreviewTabsSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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
    pub font: Option<Font>,
}

#[derive(Debug, Clone, Copy)]
pub struct TabContentParams {
    pub detail: Option<usize>,
    pub selected: bool,
    pub preview: bool,
}

impl TabContentParams {
    /// Returns the text color to be used for the tab content.
    pub fn text_color(&self) -> Color {
        if self.selected {
            Color::Default
        } else {
            Color::Muted
        }
    }
}

pub trait Item: FocusableView + EventEmitter<Self::Event> {
    type Event;

    /// Returns the tab contents.
    ///
    /// By default this returns a [`Label`] that displays that text from
    /// `tab_content_text`.
    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let Some(text) = self.tab_content_text(cx) else {
            return gpui::Empty.into_any();
        };

        Label::new(text)
            .color(params.text_color())
            .into_any_element()
    }

    /// Returns the textual contents of the tab.
    ///
    /// Use this if you don't need to customize the tab contents.
    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        None
    }

    fn tab_icon(&self, _cx: &WindowContext) -> Option<Icon> {
        None
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(ItemEvent)) {}

    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn workspace_deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        None
    }
    fn tab_description(&self, _: usize, _: &AppContext) -> Option<SharedString> {
        None
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    /// (model id, Item)
    fn for_each_project_item(
        &self,
        _: &AppContext,
        _: &mut dyn FnMut(EntityId, &dyn project::Item),
    ) {
    }
    fn is_singleton(&self, _cx: &AppContext) -> bool {
        false
    }
    fn set_nav_history(&mut self, _: ItemNavHistory, _: &mut ViewContext<Self>) {}
    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
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
        _format: bool,
        _project: Model<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save() must be implemented if can_save() returns true")
    }
    fn save_as(
        &mut self,
        _project: Model<Project>,
        _path: ProjectPath,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save_as() must be implemented if can_save() returns true")
    }
    fn reload(
        &mut self,
        _project: Model<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("reload() must be implemented if can_save() returns true")
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::Hidden
    }

    fn breadcrumbs(&self, _theme: &Theme, _cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        None
    }

    fn added_to_workspace(&mut self, _workspace: &mut Workspace, _cx: &mut ViewContext<Self>) {}

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &AppContext) -> Option<Point<Pixels>> {
        None
    }

    fn preserve_preview(&self, _cx: &AppContext) -> bool {
        false
    }
}

pub trait SerializableItem: Item {
    fn serialized_item_kind() -> &'static str;

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;

    fn deserialize(
        _project: Model<Project>,
        _workspace: WeakView<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        _cx: &mut ViewContext<Pane>,
    ) -> Task<Result<View<Self>>>;

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        closing: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>>;

    fn should_serialize(&self, event: &Self::Event) -> bool;
}

pub trait SerializableItemHandle: ItemHandle {
    fn serialized_item_kind(&self) -> &'static str;
    fn serialize(
        &self,
        workspace: &mut Workspace,
        closing: bool,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<()>>>;
    fn should_serialize(&self, event: &dyn Any, cx: &AppContext) -> bool;
}

impl<T> SerializableItemHandle for View<T>
where
    T: SerializableItem,
{
    fn serialized_item_kind(&self) -> &'static str {
        T::serialized_item_kind()
    }

    fn serialize(
        &self,
        workspace: &mut Workspace,
        closing: bool,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<()>>> {
        self.update(cx, |this, cx| {
            this.serialize(workspace, cx.entity_id().as_u64(), closing, cx)
        })
    }

    fn should_serialize(&self, event: &dyn Any, cx: &AppContext) -> bool {
        event
            .downcast_ref::<T::Event>()
            .map_or(false, |event| self.read(cx).should_serialize(event))
    }
}

pub trait ItemHandle: 'static + Send {
    fn subscribe_to_item_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(ItemEvent, &mut WindowContext)>,
    ) -> gpui::Subscription;
    fn focus_handle(&self, cx: &WindowContext) -> FocusHandle;
    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString>;
    fn tab_description(&self, detail: usize, cx: &AppContext) -> Option<SharedString>;
    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement;
    fn tab_icon(&self, cx: &WindowContext) -> Option<Icon>;
    fn telemetry_event_text(&self, cx: &WindowContext) -> Option<&'static str>;
    fn dragged_tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn project_paths(&self, cx: &AppContext) -> SmallVec<[ProjectPath; 3]>;
    fn project_item_model_ids(&self, cx: &AppContext) -> SmallVec<[EntityId; 3]>;
    fn for_each_project_item(
        &self,
        _: &AppContext,
        _: &mut dyn FnMut(EntityId, &dyn project::Item),
    );
    fn is_singleton(&self, cx: &AppContext) -> bool;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        cx: &mut WindowContext,
    ) -> Option<Box<dyn ItemHandle>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: View<Pane>,
        cx: &mut ViewContext<Workspace>,
    );
    fn deactivated(&self, cx: &mut WindowContext);
    fn workspace_deactivated(&self, cx: &mut WindowContext);
    fn navigate(&self, data: Box<dyn Any>, cx: &mut WindowContext) -> bool;
    fn item_id(&self) -> EntityId;
    fn to_any(&self) -> AnyView;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
    fn save(
        &self,
        format: bool,
        project: Model<Project>,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: Model<Project>,
        path: ProjectPath,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;
    fn reload(&self, project: Model<Project>, cx: &mut WindowContext) -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyView>;
    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>>;
    fn to_serializable_item_handle(
        &self,
        cx: &AppContext,
    ) -> Option<Box<dyn SerializableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut AppContext,
        callback: Box<dyn FnOnce(&mut AppContext) + Send>,
    ) -> gpui::Subscription;
    fn to_searchable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>>;
    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation;
    fn breadcrumbs(&self, theme: &Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>>;
    fn show_toolbar(&self, cx: &AppContext) -> bool;
    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Point<Pixels>>;
    fn downgrade_item(&self) -> Box<dyn WeakItemHandle>;
    fn workspace_settings<'a>(&self, cx: &'a AppContext) -> &'a WorkspaceSettings;
    fn preserve_preview(&self, cx: &AppContext) -> bool;
}

pub trait WeakItemHandle: Send + Sync {
    fn id(&self) -> EntityId;
    fn boxed_clone(&self) -> Box<dyn WeakItemHandle>;
    fn upgrade(&self) -> Option<Box<dyn ItemHandle>>;
}

impl dyn ItemHandle {
    pub fn downcast<V: 'static>(&self) -> Option<View<V>> {
        self.to_any().downcast().ok()
    }

    pub fn act_as<V: 'static>(&self, cx: &AppContext) -> Option<View<V>> {
        self.act_as_type(TypeId::of::<V>(), cx)
            .and_then(|t| t.downcast().ok())
    }
}

impl<T: Item> ItemHandle for View<T> {
    fn subscribe_to_item_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(ItemEvent, &mut WindowContext)>,
    ) -> gpui::Subscription {
        cx.subscribe(self, move |_, event, cx| {
            T::to_item_events(event, |item_event| handler(item_event, cx));
        })
    }

    fn focus_handle(&self, cx: &WindowContext) -> FocusHandle {
        self.focus_handle(cx)
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        self.read(cx).tab_tooltip_text(cx)
    }

    fn telemetry_event_text(&self, cx: &WindowContext) -> Option<&'static str> {
        self.read(cx).telemetry_event_text()
    }

    fn tab_description(&self, detail: usize, cx: &AppContext) -> Option<SharedString> {
        self.read(cx).tab_description(detail, cx)
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        self.read(cx).tab_content(params, cx)
    }

    fn tab_icon(&self, cx: &WindowContext) -> Option<Icon> {
        self.read(cx).tab_icon(cx)
    }

    fn dragged_tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        self.read(cx).tab_content(
            TabContentParams {
                selected: true,
                ..params
            },
            cx,
        )
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

    fn workspace_settings<'a>(&self, cx: &'a AppContext) -> &'a WorkspaceSettings {
        if let Some(project_path) = self.project_path(cx) {
            WorkspaceSettings::get(
                Some(SettingsLocation {
                    worktree_id: project_path.worktree_id.into(),
                    path: &project_path.path,
                }),
                cx,
            )
        } else {
            WorkspaceSettings::get_global(cx)
        }
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

    fn project_paths(&self, cx: &AppContext) -> SmallVec<[ProjectPath; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |_, item| {
            if let Some(id) = item.project_path(cx) {
                result.push(id);
            }
        });
        result
    }

    fn project_item_model_ids(&self, cx: &AppContext) -> SmallVec<[EntityId; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |id, _| {
            result.push(id);
        });
        result
    }

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(EntityId, &dyn project::Item),
    ) {
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
        workspace_id: Option<WorkspaceId>,
        cx: &mut WindowContext,
    ) -> Option<Box<dyn ItemHandle>> {
        self.update(cx, |item, cx| item.clone_on_split(workspace_id, cx))
            .map(|handle| Box::new(handle) as Box<dyn ItemHandle>)
    }

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: View<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let weak_item = self.downgrade();
        let history = pane.read(cx).nav_history_for_item(self);
        self.update(cx, |this, cx| {
            this.set_nav_history(history, cx);
            this.added_to_workspace(workspace, cx);
        });

        if let Some(serializable_item) = self.to_serializable_item_handle(cx) {
            workspace
                .enqueue_item_serialization(serializable_item)
                .log_err();
        }

        if workspace
            .panes_by_item
            .insert(self.item_id(), pane.downgrade())
            .is_none()
        {
            let mut pending_autosave = DelayedDebouncedEditAction::new();
            let (pending_update_tx, mut pending_update_rx) = mpsc::unbounded();
            let pending_update = Rc::new(RefCell::new(None));

            let mut send_follower_updates = None;
            if let Some(item) = self.to_followable_item_handle(cx) {
                let is_project_item = item.is_project_item(cx);
                let item = item.downgrade();

                send_follower_updates = Some(cx.spawn({
                    let pending_update = pending_update.clone();
                    |workspace, mut cx| async move {
                        while let Some(mut leader_id) = pending_update_rx.next().await {
                            while let Ok(Some(id)) = pending_update_rx.try_next() {
                                leader_id = id;
                            }

                            workspace.update(&mut cx, |workspace, cx| {
                                let Some(item) = item.upgrade() else { return };
                                workspace.update_followers(
                                    is_project_item,
                                    proto::update_followers::Variant::UpdateView(
                                        proto::UpdateView {
                                            id: item
                                                .remote_id(workspace.client(), cx)
                                                .map(|id| id.to_proto()),
                                            variant: pending_update.borrow_mut().take(),
                                            leader_id,
                                        },
                                    ),
                                    cx,
                                );
                            })?;
                            cx.background_executor().timer(LEADER_UPDATE_THROTTLE).await;
                        }
                        anyhow::Ok(())
                    }
                }));
            }

            let mut event_subscription = Some(cx.subscribe(
                self,
                move |workspace, item: View<T>, event, cx| {
                    let pane = if let Some(pane) = workspace
                        .panes_by_item
                        .get(&item.item_id())
                        .and_then(|pane| pane.upgrade())
                    {
                        pane
                    } else {
                        return;
                    };

                    if let Some(item) = item.to_followable_item_handle(cx) {
                        let leader_id = workspace.leader_for_pane(&pane);

                        if let Some(leader_id) = leader_id {
                            if let Some(FollowEvent::Unfollow) = item.to_follow_event(event) {
                                workspace.unfollow(leader_id, cx);
                            }
                        }

                        if item.focus_handle(cx).contains_focused(cx) {
                            item.add_event_to_update_proto(
                                event,
                                &mut pending_update.borrow_mut(),
                                cx,
                            );
                            pending_update_tx.unbounded_send(leader_id).ok();
                        }
                    }

                    if let Some(item) = item.to_serializable_item_handle(cx) {
                        if item.should_serialize(event, cx) {
                            workspace.enqueue_item_serialization(item).ok();
                        }
                    }

                    T::to_item_events(event, |event| match event {
                        ItemEvent::CloseItem => {
                            pane.update(cx, |pane, cx| {
                                pane.close_item_by_id(item.item_id(), crate::SaveIntent::Close, cx)
                            })
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
                            let autosave = item.workspace_settings(cx).autosave;

                            if let AutosaveSetting::AfterDelay { milliseconds } = autosave {
                                let delay = Duration::from_millis(milliseconds);
                                let item = item.clone();
                                pending_autosave.fire_new(delay, cx, move |workspace, cx| {
                                    Pane::autosave_item(&item, workspace.project().clone(), cx)
                                });
                            }
                            pane.update(cx, |pane, cx| pane.handle_item_edit(item.item_id(), cx));
                        }

                        _ => {}
                    });
                },
            ));

            cx.on_blur(&self.focus_handle(cx), move |workspace, cx| {
                if let Some(item) = weak_item.upgrade() {
                    if item.workspace_settings(cx).autosave == AutosaveSetting::OnFocusChange {
                        Pane::autosave_item(&item, workspace.project.clone(), cx)
                            .detach_and_log_err(cx);
                    }
                }
            })
            .detach();

            let item_id = self.item_id();
            cx.observe_release(self, move |workspace, _, _| {
                workspace.panes_by_item.remove(&item_id);
                event_subscription.take();
                send_follower_updates.take();
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

    fn item_id(&self) -> EntityId {
        self.entity_id()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
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

    fn save(
        &self,
        format: bool,
        project: Model<Project>,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.save(format, project, cx))
    }

    fn save_as(
        &self,
        project: Model<Project>,
        path: ProjectPath,
        cx: &mut WindowContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(project, path, cx))
    }

    fn reload(&self, project: Model<Project>, cx: &mut WindowContext) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.reload(project, cx))
    }

    fn act_as_type<'a>(&'a self, type_id: TypeId, cx: &'a AppContext) -> Option<AnyView> {
        self.read(cx).act_as_type(type_id, self, cx)
    }

    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>> {
        FollowableViewRegistry::to_followable_view(self.clone(), cx)
    }

    fn on_release(
        &self,
        cx: &mut AppContext,
        callback: Box<dyn FnOnce(&mut AppContext) + Send>,
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

    fn show_toolbar(&self, cx: &AppContext) -> bool {
        self.read(cx).show_toolbar()
    }

    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Point<Pixels>> {
        self.read(cx).pixel_position_of_cursor(cx)
    }

    fn downgrade_item(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.downgrade())
    }

    fn to_serializable_item_handle(
        &self,
        cx: &AppContext,
    ) -> Option<Box<dyn SerializableItemHandle>> {
        SerializableItemRegistry::view_to_serializable_item_handle(self.to_any(), cx)
    }

    fn preserve_preview(&self, cx: &AppContext) -> bool {
        self.read(cx).preserve_preview(cx)
    }
}

impl From<Box<dyn ItemHandle>> for AnyView {
    fn from(val: Box<dyn ItemHandle>) -> Self {
        val.to_any()
    }
}

impl From<&Box<dyn ItemHandle>> for AnyView {
    fn from(val: &Box<dyn ItemHandle>) -> Self {
        val.to_any()
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}

impl<T: Item> WeakItemHandle for WeakView<T> {
    fn id(&self) -> EntityId {
        self.entity_id()
    }

    fn boxed_clone(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.clone())
    }

    fn upgrade(&self) -> Option<Box<dyn ItemHandle>> {
        self.upgrade().map(|v| Box::new(v) as Box<dyn ItemHandle>)
    }
}

pub trait ProjectItem: Item {
    type Item: project::Item;

    fn for_project_item(
        project: Model<Project>,
        item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum FollowEvent {
    Unfollow,
}

pub enum Dedup {
    KeepExisting,
    ReplaceExisting,
}

pub trait FollowableItem: Item {
    fn remote_id(&self) -> Option<ViewId>;
    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant>;
    fn from_state_proto(
        project: View<Workspace>,
        id: ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<View<Self>>>>;
    fn to_follow_event(event: &Self::Event) -> Option<FollowEvent>;
    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool;
    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn is_project_item(&self, cx: &WindowContext) -> bool;
    fn set_leader_peer_id(&mut self, leader_peer_id: Option<PeerId>, cx: &mut ViewContext<Self>);
    fn dedup(&self, existing: &Self, cx: &WindowContext) -> Option<Dedup>;
}

pub trait FollowableItemHandle: ItemHandle {
    fn remote_id(&self, client: &Arc<Client>, cx: &WindowContext) -> Option<ViewId>;
    fn downgrade(&self) -> Box<dyn WeakFollowableItemHandle>;
    fn set_leader_peer_id(&self, leader_peer_id: Option<PeerId>, cx: &mut WindowContext);
    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool;
    fn to_follow_event(&self, event: &dyn Any) -> Option<FollowEvent>;
    fn apply_update_proto(
        &self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut WindowContext,
    ) -> Task<Result<()>>;
    fn is_project_item(&self, cx: &WindowContext) -> bool;
    fn dedup(&self, existing: &dyn FollowableItemHandle, cx: &WindowContext) -> Option<Dedup>;
}

impl<T: FollowableItem> FollowableItemHandle for View<T> {
    fn remote_id(&self, client: &Arc<Client>, cx: &WindowContext) -> Option<ViewId> {
        self.read(cx).remote_id().or_else(|| {
            client.peer_id().map(|creator| ViewId {
                creator,
                id: self.item_id().as_u64(),
            })
        })
    }

    fn downgrade(&self) -> Box<dyn WeakFollowableItemHandle> {
        Box::new(self.downgrade())
    }

    fn set_leader_peer_id(&self, leader_peer_id: Option<PeerId>, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_leader_peer_id(leader_peer_id, cx))
    }

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        self.read(cx).to_state_proto(cx)
    }

    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool {
        if let Some(event) = event.downcast_ref() {
            self.read(cx).add_event_to_update_proto(event, update, cx)
        } else {
            false
        }
    }

    fn to_follow_event(&self, event: &dyn Any) -> Option<FollowEvent> {
        T::to_follow_event(event.downcast_ref()?)
    }

    fn apply_update_proto(
        &self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| this.apply_update_proto(project, message, cx))
    }

    fn is_project_item(&self, cx: &WindowContext) -> bool {
        self.read(cx).is_project_item(cx)
    }

    fn dedup(&self, existing: &dyn FollowableItemHandle, cx: &WindowContext) -> Option<Dedup> {
        let existing = existing.to_any().downcast::<T>().ok()?;
        self.read(cx).dedup(existing.read(cx), cx)
    }
}

pub trait WeakFollowableItemHandle: Send + Sync {
    fn upgrade(&self) -> Option<Box<dyn FollowableItemHandle>>;
}

impl<T: FollowableItem> WeakFollowableItemHandle for WeakView<T> {
    fn upgrade(&self) -> Option<Box<dyn FollowableItemHandle>> {
        Some(Box::new(self.upgrade()?))
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test {
    use super::{Item, ItemEvent, SerializableItem, TabContentParams};
    use crate::{ItemId, ItemNavHistory, Pane, Workspace, WorkspaceId};
    use gpui::{
        AnyElement, AppContext, Context as _, EntityId, EventEmitter, FocusableView,
        InteractiveElement, IntoElement, Model, Render, SharedString, Task, View, ViewContext,
        VisualContext, WeakView,
    };
    use project::{Project, ProjectEntryId, ProjectPath, WorktreeId};
    use std::{any::Any, cell::Cell, path::Path};

    pub struct TestProjectItem {
        pub entry_id: Option<ProjectEntryId>,
        pub project_path: Option<ProjectPath>,
    }

    pub struct TestItem {
        pub workspace_id: Option<WorkspaceId>,
        pub state: String,
        pub label: String,
        pub save_count: usize,
        pub save_as_count: usize,
        pub reload_count: usize,
        pub is_dirty: bool,
        pub is_singleton: bool,
        pub has_conflict: bool,
        pub project_items: Vec<Model<TestProjectItem>>,
        pub nav_history: Option<ItemNavHistory>,
        pub tab_descriptions: Option<Vec<&'static str>>,
        pub tab_detail: Cell<Option<usize>>,
        serialize: Option<Box<dyn Fn() -> Option<Task<anyhow::Result<()>>>>>,
        focus_handle: gpui::FocusHandle,
    }

    impl project::Item for TestProjectItem {
        fn try_open(
            _project: &Model<Project>,
            _path: &ProjectPath,
            _cx: &mut AppContext,
        ) -> Option<Task<gpui::Result<Model<Self>>>> {
            None
        }

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

    impl TestProjectItem {
        pub fn new(id: u64, path: &str, cx: &mut AppContext) -> Model<Self> {
            let entry_id = Some(ProjectEntryId::from_proto(id));
            let project_path = Some(ProjectPath {
                worktree_id: WorktreeId::from_usize(0),
                path: Path::new(path).into(),
            });
            cx.new_model(|_| Self {
                entry_id,
                project_path,
            })
        }

        pub fn new_untitled(cx: &mut AppContext) -> Model<Self> {
            cx.new_model(|_| Self {
                project_path: None,
                entry_id: None,
            })
        }
    }

    impl TestItem {
        pub fn new(cx: &mut ViewContext<Self>) -> Self {
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
                workspace_id: Default::default(),
                focus_handle: cx.focus_handle(),
                serialize: None,
            }
        }

        pub fn new_deserialized(id: WorkspaceId, cx: &mut ViewContext<Self>) -> Self {
            let mut this = Self::new(cx);
            this.workspace_id = Some(id);
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

        pub fn with_project_items(mut self, items: &[Model<TestProjectItem>]) -> Self {
            self.project_items.clear();
            self.project_items.extend(items.iter().cloned());
            self
        }

        pub fn with_serialize(
            mut self,
            serialize: impl Fn() -> Option<Task<anyhow::Result<()>>> + 'static,
        ) -> Self {
            self.serialize = Some(Box::new(serialize));
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

    impl Render for TestItem {
        fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
            gpui::div().track_focus(&self.focus_handle)
        }
    }

    impl EventEmitter<ItemEvent> for TestItem {}

    impl FocusableView for TestItem {
        fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl Item for TestItem {
        type Event = ItemEvent;

        fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
            f(*event)
        }

        fn tab_description(&self, detail: usize, _: &AppContext) -> Option<SharedString> {
            self.tab_descriptions.as_ref().and_then(|descriptions| {
                let description = *descriptions.get(detail).or_else(|| descriptions.last())?;
                Some(description.into())
            })
        }

        fn telemetry_event_text(&self) -> Option<&'static str> {
            None
        }

        fn tab_content(
            &self,
            params: TabContentParams,
            _cx: &ui::prelude::WindowContext,
        ) -> AnyElement {
            self.tab_detail.set(params.detail);
            gpui::div().into_any_element()
        }

        fn for_each_project_item(
            &self,
            cx: &AppContext,
            f: &mut dyn FnMut(EntityId, &dyn project::Item),
        ) {
            self.project_items
                .iter()
                .for_each(|item| f(item.entity_id(), item.read(cx)))
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
            _workspace_id: Option<WorkspaceId>,
            cx: &mut ViewContext<Self>,
        ) -> Option<View<Self>>
        where
            Self: Sized,
        {
            Some(cx.new_view(|cx| Self {
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
                focus_handle: cx.focus_handle(),
                serialize: None,
            }))
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
            _: bool,
            _: Model<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn save_as(
            &mut self,
            _: Model<Project>,
            _: ProjectPath,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_as_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn reload(
            &mut self,
            _: Model<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.reload_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }
    }

    impl SerializableItem for TestItem {
        fn serialized_item_kind() -> &'static str {
            "TestItem"
        }

        fn deserialize(
            _project: Model<Project>,
            _workspace: WeakView<Workspace>,
            workspace_id: WorkspaceId,
            _item_id: ItemId,
            cx: &mut ViewContext<Pane>,
        ) -> Task<anyhow::Result<View<Self>>> {
            let view = cx.new_view(|cx| Self::new_deserialized(workspace_id, cx));
            Task::ready(Ok(view))
        }

        fn cleanup(
            _workspace_id: WorkspaceId,
            _alive_items: Vec<ItemId>,
            _cx: &mut ui::WindowContext,
        ) -> Task<anyhow::Result<()>> {
            Task::ready(Ok(()))
        }

        fn serialize(
            &mut self,
            _workspace: &mut Workspace,
            _item_id: ItemId,
            _closing: bool,
            _cx: &mut ViewContext<Self>,
        ) -> Option<Task<anyhow::Result<()>>> {
            if let Some(serialize) = self.serialize.take() {
                let result = serialize();
                self.serialize = Some(serialize);
                result
            } else {
                None
            }
        }

        fn should_serialize(&self, _event: &Self::Event) -> bool {
            false
        }
    }
}
