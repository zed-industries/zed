use crate::{
    CollaboratorId, DelayedDebouncedEditAction, FollowableViewRegistry, ItemNavHistory,
    SerializableItemRegistry, ToolbarItemLocation, ViewId, Workspace, WorkspaceId,
    invalid_item_view::InvalidItemView,
    pane::{self, Pane},
    persistence::model::ItemId,
    searchable::SearchableItemHandle,
    workspace_settings::{AutosaveSetting, WorkspaceSettings},
};
use anyhow::Result;
use client::{Client, proto};
use futures::{StreamExt, channel::mpsc};
use gpui::{
    Action, AnyElement, AnyEntity, AnyView, App, AppContext, Context, Entity, EntityId,
    EventEmitter, FocusHandle, Focusable, Font, HighlightStyle, Pixels, Point, Render,
    SharedString, Task, WeakEntity, Window,
};
use language::Capability;
use project::{Project, ProjectEntryId, ProjectPath};
pub use settings::{
    ActivateOnClose, ClosePosition, RegisterSetting, Settings, SettingsLocation, ShowCloseButton,
    ShowDiagnostics,
};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::Range,
    path::Path,
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use theme::Theme;
use ui::{Color, Icon, IntoElement, Label, LabelCommon};
use util::ResultExt;

pub const LEADER_UPDATE_THROTTLE: Duration = Duration::from_millis(200);

#[derive(Clone, Copy, Debug)]
pub struct SaveOptions {
    pub format: bool,
    pub autosave: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            format: true,
            autosave: false,
        }
    }
}

#[derive(RegisterSetting)]
pub struct ItemSettings {
    pub git_status: bool,
    pub close_position: ClosePosition,
    pub activate_on_close: ActivateOnClose,
    pub file_icons: bool,
    pub show_diagnostics: ShowDiagnostics,
    pub show_close_button: ShowCloseButton,
}

#[derive(RegisterSetting)]
pub struct PreviewTabsSettings {
    pub enabled: bool,
    pub enable_preview_from_project_panel: bool,
    pub enable_preview_from_file_finder: bool,
    pub enable_preview_from_multibuffer: bool,
    pub enable_preview_multibuffer_from_code_navigation: bool,
    pub enable_preview_file_from_code_navigation: bool,
    pub enable_keep_preview_on_code_navigation: bool,
}

impl Settings for ItemSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let tabs = content.tabs.as_ref().unwrap();
        Self {
            git_status: tabs.git_status.unwrap()
                && content
                    .git
                    .unwrap()
                    .enabled
                    .unwrap()
                    .is_git_status_enabled(),
            close_position: tabs.close_position.unwrap(),
            activate_on_close: tabs.activate_on_close.unwrap(),
            file_icons: tabs.file_icons.unwrap(),
            show_diagnostics: tabs.show_diagnostics.unwrap(),
            show_close_button: tabs.show_close_button.unwrap(),
        }
    }
}

impl Settings for PreviewTabsSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let preview_tabs = content.preview_tabs.as_ref().unwrap();
        Self {
            enabled: preview_tabs.enabled.unwrap(),
            enable_preview_from_project_panel: preview_tabs
                .enable_preview_from_project_panel
                .unwrap(),
            enable_preview_from_file_finder: preview_tabs.enable_preview_from_file_finder.unwrap(),
            enable_preview_from_multibuffer: preview_tabs.enable_preview_from_multibuffer.unwrap(),
            enable_preview_multibuffer_from_code_navigation: preview_tabs
                .enable_preview_multibuffer_from_code_navigation
                .unwrap(),
            enable_preview_file_from_code_navigation: preview_tabs
                .enable_preview_file_from_code_navigation
                .unwrap(),
            enable_keep_preview_on_code_navigation: preview_tabs
                .enable_keep_preview_on_code_navigation
                .unwrap(),
        }
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
#[derive(Debug)]
pub struct BreadcrumbText {
    pub text: String,
    pub highlights: Option<Vec<(Range<usize>, HighlightStyle)>>,
    pub font: Option<Font>,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct TabContentParams {
    pub detail: Option<usize>,
    pub selected: bool,
    pub preview: bool,
    /// Tab content should be deemphasized when active pane does not have focus.
    pub deemphasized: bool,
}

impl TabContentParams {
    /// Returns the text color to be used for the tab content.
    pub fn text_color(&self) -> Color {
        if self.deemphasized {
            if self.selected {
                Color::Muted
            } else {
                Color::Hidden
            }
        } else if self.selected {
            Color::Default
        } else {
            Color::Muted
        }
    }
}

pub enum TabTooltipContent {
    Text(SharedString),
    Custom(Box<dyn Fn(&mut Window, &mut App) -> AnyView>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemBufferKind {
    Multibuffer,
    Singleton,
    None,
}

pub trait Item: Focusable + EventEmitter<Self::Event> + Render + Sized {
    type Event;

    /// Returns the tab contents.
    ///
    /// By default this returns a [`Label`] that displays that text from
    /// `tab_content_text`.
    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let text = self.tab_content_text(params.detail.unwrap_or_default(), cx);

        Label::new(text)
            .color(params.text_color())
            .into_any_element()
    }

    /// Returns the textual contents of the tab.
    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString;

    /// Returns the suggested filename for saving this item.
    /// By default, returns the tab content text.
    fn suggested_filename(&self, cx: &App) -> SharedString {
        self.tab_content_text(0, cx)
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        None
    }

    /// Returns the tab tooltip text.
    ///
    /// Use this if you don't need to customize the tab tooltip content.
    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        None
    }

    /// Returns the tab tooltip content.
    ///
    /// By default this returns a Tooltip text from
    /// `tab_tooltip_text`.
    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent> {
        self.tab_tooltip_text(cx).map(TabTooltipContent::Text)
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(ItemEvent)) {}

    fn deactivated(&mut self, _window: &mut Window, _: &mut Context<Self>) {}
    fn discarded(&self, _project: Entity<Project>, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn on_removed(&self, _cx: &mut Context<Self>) {}
    fn workspace_deactivated(&mut self, _window: &mut Window, _: &mut Context<Self>) {}
    fn navigate(
        &mut self,
        _: Arc<dyn Any + Send>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> bool {
        false
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    /// (model id, Item)
    fn for_each_project_item(
        &self,
        _: &App,
        _: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
    }
    fn buffer_kind(&self, _cx: &App) -> ItemBufferKind {
        ItemBufferKind::None
    }
    fn set_nav_history(&mut self, _: ItemNavHistory, _window: &mut Window, _: &mut Context<Self>) {}

    fn can_split(&self) -> bool {
        false
    }
    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        _ = (workspace_id, window, cx);
        unimplemented!("clone_on_split() must be implemented if can_split() returns true")
    }
    fn is_dirty(&self, _: &App) -> bool {
        false
    }
    fn capability(&self, _: &App) -> Capability {
        Capability::ReadWrite
    }

    fn toggle_read_only(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn has_deleted_file(&self, _: &App) -> bool {
        false
    }
    fn has_conflict(&self, _: &App) -> bool {
        false
    }
    fn can_save(&self, _cx: &App) -> bool {
        false
    }
    fn can_save_as(&self, _: &App) -> bool {
        false
    }
    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save() must be implemented if can_save() returns true")
    }
    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: ProjectPath,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save_as() must be implemented if can_save() returns true")
    }
    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("reload() must be implemented if can_save() returns true")
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyEntity> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::Hidden
    }

    fn breadcrumbs(&self, _theme: &Theme, _cx: &App) -> Option<Vec<BreadcrumbText>> {
        None
    }

    /// Returns optional elements to render to the left of the breadcrumb.
    fn breadcrumb_prefix(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<gpui::AnyElement> {
        None
    }

    fn added_to_workspace(
        &mut self,
        _workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &App) -> Option<Point<Pixels>> {
        None
    }

    fn preserve_preview(&self, _cx: &App) -> bool {
        false
    }

    fn include_in_nav_history() -> bool {
        true
    }

    /// Returns additional actions to add to the tab's context menu.
    /// Each entry is a label and an action to dispatch.
    fn tab_extra_context_menu_actions(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Vec<(SharedString, Box<dyn Action>)> {
        Vec::new()
    }
}

pub trait SerializableItem: Item {
    fn serialized_item_kind() -> &'static str;

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;

    fn deserialize(
        _project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Entity<Self>>>;

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        closing: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>>;

    fn should_serialize(&self, event: &Self::Event) -> bool;
}

pub trait SerializableItemHandle: ItemHandle {
    fn serialized_item_kind(&self) -> &'static str;
    fn serialize(
        &self,
        workspace: &mut Workspace,
        closing: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<()>>>;
    fn should_serialize(&self, event: &dyn Any, cx: &App) -> bool;
}

impl<T> SerializableItemHandle for Entity<T>
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
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<()>>> {
        self.update(cx, |this, cx| {
            this.serialize(workspace, cx.entity_id().as_u64(), closing, window, cx)
        })
    }

    fn should_serialize(&self, event: &dyn Any, cx: &App) -> bool {
        event
            .downcast_ref::<T::Event>()
            .is_some_and(|event| self.read(cx).should_serialize(event))
    }
}

pub trait ItemHandle: 'static + Send {
    fn item_focus_handle(&self, cx: &App) -> FocusHandle;
    fn subscribe_to_item_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(ItemEvent, &mut Window, &mut App)>,
    ) -> gpui::Subscription;
    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement;
    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString;
    fn suggested_filename(&self, cx: &App) -> SharedString;
    fn tab_icon(&self, window: &Window, cx: &App) -> Option<Icon>;
    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString>;
    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent>;
    fn telemetry_event_text(&self, cx: &App) -> Option<&'static str>;
    fn dragged_tab_content(
        &self,
        params: TabContentParams,
        window: &Window,
        cx: &App,
    ) -> AnyElement;
    fn project_path(&self, cx: &App) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &App) -> SmallVec<[ProjectEntryId; 3]>;
    fn project_paths(&self, cx: &App) -> SmallVec<[ProjectPath; 3]>;
    fn project_item_model_ids(&self, cx: &App) -> SmallVec<[EntityId; 3]>;
    fn for_each_project_item(
        &self,
        _: &App,
        _: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    );
    fn buffer_kind(&self, cx: &App) -> ItemBufferKind;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn can_split(&self, cx: &App) -> bool;
    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<Box<dyn ItemHandle>>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    );
    fn deactivated(&self, window: &mut Window, cx: &mut App);
    fn on_removed(&self, cx: &mut App);
    fn workspace_deactivated(&self, window: &mut Window, cx: &mut App);
    fn navigate(&self, data: Arc<dyn Any + Send>, window: &mut Window, cx: &mut App) -> bool;
    fn item_id(&self) -> EntityId;
    fn to_any_view(&self) -> AnyView;
    fn is_dirty(&self, cx: &App) -> bool;
    fn capability(&self, cx: &App) -> Capability;
    fn toggle_read_only(&self, window: &mut Window, cx: &mut App);
    fn has_deleted_file(&self, cx: &App) -> bool;
    fn has_conflict(&self, cx: &App) -> bool;
    fn can_save(&self, cx: &App) -> bool;
    fn can_save_as(&self, cx: &App) -> bool;
    fn save(
        &self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: Entity<Project>,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn reload(
        &self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &App) -> Option<AnyEntity>;
    fn to_followable_item_handle(&self, cx: &App) -> Option<Box<dyn FollowableItemHandle>>;
    fn to_serializable_item_handle(&self, cx: &App) -> Option<Box<dyn SerializableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut App,
        callback: Box<dyn FnOnce(&mut App) + Send>,
    ) -> gpui::Subscription;
    fn to_searchable_item_handle(&self, cx: &App) -> Option<Box<dyn SearchableItemHandle>>;
    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation;
    fn breadcrumbs(&self, theme: &Theme, cx: &App) -> Option<Vec<BreadcrumbText>>;
    fn breadcrumb_prefix(&self, window: &mut Window, cx: &mut App) -> Option<gpui::AnyElement>;
    fn show_toolbar(&self, cx: &App) -> bool;
    fn pixel_position_of_cursor(&self, cx: &App) -> Option<Point<Pixels>>;
    fn downgrade_item(&self) -> Box<dyn WeakItemHandle>;
    fn workspace_settings<'a>(&self, cx: &'a App) -> &'a WorkspaceSettings;
    fn preserve_preview(&self, cx: &App) -> bool;
    fn include_in_nav_history(&self) -> bool;
    fn relay_action(&self, action: Box<dyn Action>, window: &mut Window, cx: &mut App);
    fn tab_extra_context_menu_actions(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<(SharedString, Box<dyn Action>)>;
    fn can_autosave(&self, cx: &App) -> bool {
        let is_deleted = self.project_entry_ids(cx).is_empty();
        self.is_dirty(cx) && !self.has_conflict(cx) && self.can_save(cx) && !is_deleted
    }
}

pub trait WeakItemHandle: Send + Sync {
    fn id(&self) -> EntityId;
    fn boxed_clone(&self) -> Box<dyn WeakItemHandle>;
    fn upgrade(&self) -> Option<Box<dyn ItemHandle>>;
}

impl dyn ItemHandle {
    pub fn downcast<V: 'static>(&self) -> Option<Entity<V>> {
        self.to_any_view().downcast().ok()
    }

    pub fn act_as<V: 'static>(&self, cx: &App) -> Option<Entity<V>> {
        self.act_as_type(TypeId::of::<V>(), cx)
            .and_then(|t| t.downcast().ok())
    }
}

impl<T: Item> ItemHandle for Entity<T> {
    fn subscribe_to_item_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(ItemEvent, &mut Window, &mut App)>,
    ) -> gpui::Subscription {
        window.subscribe(self, cx, move |_, event, window, cx| {
            T::to_item_events(event, |item_event| handler(item_event, window, cx));
        })
    }

    fn item_focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }

    fn telemetry_event_text(&self, cx: &App) -> Option<&'static str> {
        self.read(cx).telemetry_event_text()
    }

    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement {
        self.read(cx).tab_content(params, window, cx)
    }
    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString {
        self.read(cx).tab_content_text(detail, cx)
    }

    fn suggested_filename(&self, cx: &App) -> SharedString {
        self.read(cx).suggested_filename(cx)
    }

    fn tab_icon(&self, window: &Window, cx: &App) -> Option<Icon> {
        self.read(cx).tab_icon(window, cx)
    }

    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent> {
        self.read(cx).tab_tooltip_content(cx)
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        self.read(cx).tab_tooltip_text(cx)
    }

    fn dragged_tab_content(
        &self,
        params: TabContentParams,
        window: &Window,
        cx: &App,
    ) -> AnyElement {
        self.read(cx).tab_content(
            TabContentParams {
                selected: true,
                ..params
            },
            window,
            cx,
        )
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        let this = self.read(cx);
        let mut result = None;
        if this.buffer_kind(cx) == ItemBufferKind::Singleton {
            this.for_each_project_item(cx, &mut |_, item| {
                result = item.project_path(cx);
            });
        }
        result
    }

    fn workspace_settings<'a>(&self, cx: &'a App) -> &'a WorkspaceSettings {
        if let Some(project_path) = self.project_path(cx) {
            WorkspaceSettings::get(
                Some(SettingsLocation {
                    worktree_id: project_path.worktree_id,
                    path: &project_path.path,
                }),
                cx,
            )
        } else {
            WorkspaceSettings::get_global(cx)
        }
    }

    fn project_entry_ids(&self, cx: &App) -> SmallVec<[ProjectEntryId; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |_, item| {
            if let Some(id) = item.entry_id(cx) {
                result.push(id);
            }
        });
        result
    }

    fn project_paths(&self, cx: &App) -> SmallVec<[ProjectPath; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |_, item| {
            if let Some(id) = item.project_path(cx) {
                result.push(id);
            }
        });
        result
    }

    fn project_item_model_ids(&self, cx: &App) -> SmallVec<[EntityId; 3]> {
        let mut result = SmallVec::new();
        self.read(cx).for_each_project_item(cx, &mut |id, _| {
            result.push(id);
        });
        result
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
        self.read(cx).for_each_project_item(cx, f)
    }

    fn buffer_kind(&self, cx: &App) -> ItemBufferKind {
        self.read(cx).buffer_kind(cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn can_split(&self, cx: &App) -> bool {
        self.read(cx).can_split()
    }

    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<Box<dyn ItemHandle>>> {
        let task = self.update(cx, |item, cx| item.clone_on_split(workspace_id, window, cx));
        cx.background_spawn(async move {
            task.await
                .map(|handle| Box::new(handle) as Box<dyn ItemHandle>)
        })
    }

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak_item = self.downgrade();
        let history = pane.read(cx).nav_history_for_item(self);
        self.update(cx, |this, cx| {
            this.set_nav_history(history, window, cx);
            this.added_to_workspace(workspace, window, cx);
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
                let is_project_item = item.is_project_item(window, cx);
                let item = item.downgrade();

                send_follower_updates = Some(cx.spawn_in(window, {
                    let pending_update = pending_update.clone();
                    async move |workspace, cx| {
                        while let Some(mut leader_id) = pending_update_rx.next().await {
                            while let Ok(Some(id)) = pending_update_rx.try_next() {
                                leader_id = id;
                            }

                            workspace.update_in(cx, |workspace, window, cx| {
                                let Some(item) = item.upgrade() else { return };
                                workspace.update_followers(
                                    is_project_item,
                                    proto::update_followers::Variant::UpdateView(
                                        proto::UpdateView {
                                            id: item
                                                .remote_id(workspace.client(), window, cx)
                                                .and_then(|id| id.to_proto()),
                                            variant: pending_update.borrow_mut().take(),
                                            leader_id,
                                        },
                                    ),
                                    window,
                                    cx,
                                );
                            })?;
                            cx.background_executor().timer(LEADER_UPDATE_THROTTLE).await;
                        }
                        anyhow::Ok(())
                    }
                }));
            }

            let mut event_subscription = Some(cx.subscribe_in(
                self,
                window,
                move |workspace, item: &Entity<T>, event, window, cx| {
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

                        if let Some(leader_id) = leader_id
                            && let Some(FollowEvent::Unfollow) = item.to_follow_event(event)
                        {
                            workspace.unfollow(leader_id, window, cx);
                        }

                        if item.item_focus_handle(cx).contains_focused(window, cx) {
                            match leader_id {
                                Some(CollaboratorId::Agent) => {}
                                Some(CollaboratorId::PeerId(leader_peer_id)) => {
                                    item.add_event_to_update_proto(
                                        event,
                                        &mut pending_update.borrow_mut(),
                                        window,
                                        cx,
                                    );
                                    pending_update_tx.unbounded_send(Some(leader_peer_id)).ok();
                                }
                                None => {
                                    item.add_event_to_update_proto(
                                        event,
                                        &mut pending_update.borrow_mut(),
                                        window,
                                        cx,
                                    );
                                    pending_update_tx.unbounded_send(None).ok();
                                }
                            }
                        }
                    }

                    if let Some(item) = item.to_serializable_item_handle(cx)
                        && item.should_serialize(event, cx)
                    {
                        workspace.enqueue_item_serialization(item).ok();
                    }

                    T::to_item_events(event, |event| match event {
                        ItemEvent::CloseItem => {
                            pane.update(cx, |pane, cx| {
                                pane.close_item_by_id(
                                    item.item_id(),
                                    crate::SaveIntent::Close,
                                    window,
                                    cx,
                                )
                            })
                            .detach_and_log_err(cx);
                        }

                        ItemEvent::UpdateTab => {
                            workspace.update_item_dirty_state(item, window, cx);

                            if item.has_deleted_file(cx)
                                && !item.is_dirty(cx)
                                && item.workspace_settings(cx).close_on_file_delete
                            {
                                let item_id = item.item_id();
                                let close_item_task = pane.update(cx, |pane, cx| {
                                    pane.close_item_by_id(
                                        item_id,
                                        crate::SaveIntent::Close,
                                        window,
                                        cx,
                                    )
                                });
                                cx.spawn_in(window, {
                                    let pane = pane.clone();
                                    async move |_workspace, cx| {
                                        close_item_task.await?;
                                        pane.update(cx, |pane, _cx| {
                                            pane.nav_history_mut().remove_item(item_id);
                                        });
                                        anyhow::Ok(())
                                    }
                                })
                                .detach_and_log_err(cx);
                            } else {
                                pane.update(cx, |_, cx| {
                                    cx.emit(pane::Event::ChangeItemTitle);
                                    cx.notify();
                                });
                            }
                        }

                        ItemEvent::Edit => {
                            let autosave = item.workspace_settings(cx).autosave;

                            if let AutosaveSetting::AfterDelay { milliseconds } = autosave {
                                let delay = Duration::from_millis(milliseconds.0);
                                let item = item.clone();
                                pending_autosave.fire_new(
                                    delay,
                                    window,
                                    cx,
                                    move |workspace, window, cx| {
                                        Pane::autosave_item(
                                            &item,
                                            workspace.project().clone(),
                                            window,
                                            cx,
                                        )
                                    },
                                );
                            }
                            pane.update(cx, |pane, cx| pane.handle_item_edit(item.item_id(), cx));
                        }

                        _ => {}
                    });
                },
            ));

            cx.on_blur(
                &self.read(cx).focus_handle(cx),
                window,
                move |workspace, window, cx| {
                    if let Some(item) = weak_item.upgrade()
                        && item.workspace_settings(cx).autosave == AutosaveSetting::OnFocusChange
                    {
                        // Only trigger autosave if focus has truly left the item.
                        // If focus is still within the item's hierarchy (e.g., moved to a context menu),
                        // don't trigger autosave to avoid unwanted formatting and cursor jumps.
                        // Also skip autosave if focus moved to a modal (e.g., command palette),
                        // since the user is still interacting with the workspace.
                        let focus_handle = item.item_focus_handle(cx);
                        if !focus_handle.contains_focused(window, cx)
                            && !workspace.has_active_modal(window, cx)
                        {
                            Pane::autosave_item(&item, workspace.project.clone(), window, cx)
                                .detach_and_log_err(cx);
                        }
                    }
                },
            )
            .detach();

            let item_id = self.item_id();
            workspace.update_item_dirty_state(self, window, cx);
            cx.observe_release_in(self, window, move |workspace, _, _, _| {
                workspace.panes_by_item.remove(&item_id);
                event_subscription.take();
                send_follower_updates.take();
            })
            .detach();
        }

        cx.defer_in(window, |workspace, window, cx| {
            workspace.serialize_workspace(window, cx);
        });
    }

    fn deactivated(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.deactivated(window, cx));
    }

    fn on_removed(&self, cx: &mut App) {
        self.update(cx, |item, cx| item.on_removed(cx));
    }

    fn workspace_deactivated(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.workspace_deactivated(window, cx));
    }

    fn navigate(&self, data: Arc<dyn Any + Send>, window: &mut Window, cx: &mut App) -> bool {
        self.update(cx, |this, cx| this.navigate(data, window, cx))
    }

    fn item_id(&self) -> EntityId {
        self.entity_id()
    }

    fn to_any_view(&self) -> AnyView {
        self.clone().into()
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.read(cx).is_dirty(cx)
    }

    fn capability(&self, cx: &App) -> Capability {
        self.read(cx).capability(cx)
    }

    fn toggle_read_only(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.toggle_read_only(window, cx);
        })
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.read(cx).has_deleted_file(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.read(cx).has_conflict(cx)
    }

    fn can_save(&self, cx: &App) -> bool {
        self.read(cx).can_save(cx)
    }

    fn can_save_as(&self, cx: &App) -> bool {
        self.read(cx).can_save_as(cx)
    }

    fn save(
        &self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.save(options, project, window, cx))
    }

    fn save_as(
        &self,
        project: Entity<Project>,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(project, path, window, cx))
    }

    fn reload(
        &self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.reload(project, window, cx))
    }

    fn act_as_type<'a>(&'a self, type_id: TypeId, cx: &'a App) -> Option<AnyEntity> {
        self.read(cx).act_as_type(type_id, self, cx)
    }

    fn to_followable_item_handle(&self, cx: &App) -> Option<Box<dyn FollowableItemHandle>> {
        FollowableViewRegistry::to_followable_view(self.clone(), cx)
    }

    fn on_release(
        &self,
        cx: &mut App,
        callback: Box<dyn FnOnce(&mut App) + Send>,
    ) -> gpui::Subscription {
        cx.observe_release(self, move |_, cx| callback(cx))
    }

    fn to_searchable_item_handle(&self, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        self.read(cx).as_searchable(self, cx)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        self.read(cx).breadcrumb_location(cx)
    }

    fn breadcrumbs(&self, theme: &Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.read(cx).breadcrumbs(theme, cx)
    }

    fn breadcrumb_prefix(&self, window: &mut Window, cx: &mut App) -> Option<gpui::AnyElement> {
        self.update(cx, |item, cx| item.breadcrumb_prefix(window, cx))
    }

    fn show_toolbar(&self, cx: &App) -> bool {
        self.read(cx).show_toolbar()
    }

    fn pixel_position_of_cursor(&self, cx: &App) -> Option<Point<Pixels>> {
        self.read(cx).pixel_position_of_cursor(cx)
    }

    fn downgrade_item(&self) -> Box<dyn WeakItemHandle> {
        Box::new(self.downgrade())
    }

    fn to_serializable_item_handle(&self, cx: &App) -> Option<Box<dyn SerializableItemHandle>> {
        SerializableItemRegistry::view_to_serializable_item_handle(self.to_any_view(), cx)
    }

    fn preserve_preview(&self, cx: &App) -> bool {
        self.read(cx).preserve_preview(cx)
    }

    fn include_in_nav_history(&self) -> bool {
        T::include_in_nav_history()
    }

    fn relay_action(&self, action: Box<dyn Action>, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.focus_handle(cx).focus(window, cx);
            window.dispatch_action(action, cx);
        })
    }

    fn tab_extra_context_menu_actions(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<(SharedString, Box<dyn Action>)> {
        self.update(cx, |this, cx| {
            this.tab_extra_context_menu_actions(window, cx)
        })
    }
}

impl From<Box<dyn ItemHandle>> for AnyView {
    fn from(val: Box<dyn ItemHandle>) -> Self {
        val.to_any_view()
    }
}

impl From<&Box<dyn ItemHandle>> for AnyView {
    fn from(val: &Box<dyn ItemHandle>) -> Self {
        val.to_any_view()
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}

impl<T: Item> WeakItemHandle for WeakEntity<T> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectItemKind(pub &'static str);

pub trait ProjectItem: Item {
    type Item: project::ProjectItem;

    fn project_item_kind() -> Option<ProjectItemKind> {
        None
    }

    fn for_project_item(
        project: Entity<Project>,
        pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized;

    /// A fallback handler, which will be called after [`project::ProjectItem::try_open`] fails,
    /// with the error from that failure as an argument.
    /// Allows to open an item that can gracefully display and handle errors.
    fn for_broken_project_item(
        _abs_path: &Path,
        _is_local: bool,
        _e: &anyhow::Error,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<InvalidItemView>
    where
        Self: Sized,
    {
        None
    }
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
    fn to_state_proto(&self, window: &mut Window, cx: &mut App) -> Option<proto::view::Variant>;
    fn from_state_proto(
        project: Entity<Workspace>,
        id: ViewId,
        state: &mut Option<proto::view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>>;
    fn to_follow_event(event: &Self::Event) -> Option<FollowEvent>;
    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool;
    fn apply_update_proto(
        &mut self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>>;
    fn is_project_item(&self, window: &Window, cx: &App) -> bool;
    fn set_leader_id(
        &mut self,
        leader_peer_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    );
    fn dedup(&self, existing: &Self, window: &Window, cx: &App) -> Option<Dedup>;
    fn update_agent_location(
        &mut self,
        _location: language::Anchor,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

pub trait FollowableItemHandle: ItemHandle {
    fn remote_id(&self, client: &Arc<Client>, window: &mut Window, cx: &mut App) -> Option<ViewId>;
    fn downgrade(&self) -> Box<dyn WeakFollowableItemHandle>;
    fn set_leader_id(
        &self,
        leader_peer_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut App,
    );
    fn to_state_proto(&self, window: &mut Window, cx: &mut App) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool;
    fn to_follow_event(&self, event: &dyn Any) -> Option<FollowEvent>;
    fn apply_update_proto(
        &self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn is_project_item(&self, window: &mut Window, cx: &mut App) -> bool;
    fn dedup(
        &self,
        existing: &dyn FollowableItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Dedup>;
    fn update_agent_location(&self, location: language::Anchor, window: &mut Window, cx: &mut App);
}

impl<T: FollowableItem> FollowableItemHandle for Entity<T> {
    fn remote_id(&self, client: &Arc<Client>, _: &mut Window, cx: &mut App) -> Option<ViewId> {
        self.read(cx).remote_id().or_else(|| {
            client.peer_id().map(|creator| ViewId {
                creator: CollaboratorId::PeerId(creator),
                id: self.item_id().as_u64(),
            })
        })
    }

    fn downgrade(&self) -> Box<dyn WeakFollowableItemHandle> {
        Box::new(self.downgrade())
    }

    fn set_leader_id(&self, leader_id: Option<CollaboratorId>, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_leader_id(leader_id, window, cx))
    }

    fn to_state_proto(&self, window: &mut Window, cx: &mut App) -> Option<proto::view::Variant> {
        self.update(cx, |this, cx| this.to_state_proto(window, cx))
    }

    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if let Some(event) = event.downcast_ref() {
            self.update(cx, |this, cx| {
                this.add_event_to_update_proto(event, update, window, cx)
            })
        } else {
            false
        }
    }

    fn to_follow_event(&self, event: &dyn Any) -> Option<FollowEvent> {
        T::to_follow_event(event.downcast_ref()?)
    }

    fn apply_update_proto(
        &self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| {
            this.apply_update_proto(project, message, window, cx)
        })
    }

    fn is_project_item(&self, window: &mut Window, cx: &mut App) -> bool {
        self.read(cx).is_project_item(window, cx)
    }

    fn dedup(
        &self,
        existing: &dyn FollowableItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Dedup> {
        let existing = existing.to_any_view().downcast::<T>().ok()?;
        self.read(cx).dedup(existing.read(cx), window, cx)
    }

    fn update_agent_location(&self, location: language::Anchor, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.update_agent_location(location, window, cx)
        })
    }
}

pub trait WeakFollowableItemHandle: Send + Sync {
    fn upgrade(&self) -> Option<Box<dyn FollowableItemHandle>>;
}

impl<T: FollowableItem> WeakFollowableItemHandle for WeakEntity<T> {
    fn upgrade(&self) -> Option<Box<dyn FollowableItemHandle>> {
        Some(Box::new(self.upgrade()?))
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test {
    use super::{Item, ItemEvent, SerializableItem, TabContentParams};
    use crate::{
        ItemId, ItemNavHistory, Workspace, WorkspaceId,
        item::{ItemBufferKind, SaveOptions},
    };
    use gpui::{
        AnyElement, App, AppContext as _, Context, Entity, EntityId, EventEmitter, Focusable,
        InteractiveElement, IntoElement, Render, SharedString, Task, WeakEntity, Window,
    };
    use project::{Project, ProjectEntryId, ProjectPath, WorktreeId};
    use std::{any::Any, cell::Cell, sync::Arc};
    use util::rel_path::rel_path;

    pub struct TestProjectItem {
        pub entry_id: Option<ProjectEntryId>,
        pub project_path: Option<ProjectPath>,
        pub is_dirty: bool,
    }

    pub struct TestItem {
        pub workspace_id: Option<WorkspaceId>,
        pub state: String,
        pub label: String,
        pub save_count: usize,
        pub save_as_count: usize,
        pub reload_count: usize,
        pub is_dirty: bool,
        pub buffer_kind: ItemBufferKind,
        pub has_conflict: bool,
        pub has_deleted_file: bool,
        pub project_items: Vec<Entity<TestProjectItem>>,
        pub nav_history: Option<ItemNavHistory>,
        pub tab_descriptions: Option<Vec<&'static str>>,
        pub tab_detail: Cell<Option<usize>>,
        serialize: Option<Box<dyn Fn() -> Option<Task<anyhow::Result<()>>>>>,
        focus_handle: gpui::FocusHandle,
    }

    impl project::ProjectItem for TestProjectItem {
        fn try_open(
            _project: &Entity<Project>,
            _path: &ProjectPath,
            _cx: &mut App,
        ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
            None
        }
        fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
            self.entry_id
        }

        fn project_path(&self, _: &App) -> Option<ProjectPath> {
            self.project_path.clone()
        }

        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
    }

    pub enum TestItemEvent {
        Edit,
    }

    impl TestProjectItem {
        pub fn new(id: u64, path: &str, cx: &mut App) -> Entity<Self> {
            let entry_id = Some(ProjectEntryId::from_proto(id));
            let project_path = Some(ProjectPath {
                worktree_id: WorktreeId::from_usize(0),
                path: rel_path(path).into(),
            });
            cx.new(|_| Self {
                entry_id,
                project_path,
                is_dirty: false,
            })
        }

        pub fn new_untitled(cx: &mut App) -> Entity<Self> {
            cx.new(|_| Self {
                project_path: None,
                entry_id: None,
                is_dirty: false,
            })
        }

        pub fn new_dirty(id: u64, path: &str, cx: &mut App) -> Entity<Self> {
            let entry_id = Some(ProjectEntryId::from_proto(id));
            let project_path = Some(ProjectPath {
                worktree_id: WorktreeId::from_usize(0),
                path: rel_path(path).into(),
            });
            cx.new(|_| Self {
                entry_id,
                project_path,
                is_dirty: true,
            })
        }
    }

    impl TestItem {
        pub fn new(cx: &mut Context<Self>) -> Self {
            Self {
                state: String::new(),
                label: String::new(),
                save_count: 0,
                save_as_count: 0,
                reload_count: 0,
                is_dirty: false,
                has_conflict: false,
                has_deleted_file: false,
                project_items: Vec::new(),
                buffer_kind: ItemBufferKind::Singleton,
                nav_history: None,
                tab_descriptions: None,
                tab_detail: Default::default(),
                workspace_id: Default::default(),
                focus_handle: cx.focus_handle(),
                serialize: None,
            }
        }

        pub fn new_deserialized(id: WorkspaceId, cx: &mut Context<Self>) -> Self {
            let mut this = Self::new(cx);
            this.workspace_id = Some(id);
            this
        }

        pub fn with_label(mut self, state: &str) -> Self {
            self.label = state.to_string();
            self
        }

        pub fn with_buffer_kind(mut self, buffer_kind: ItemBufferKind) -> Self {
            self.buffer_kind = buffer_kind;
            self
        }

        pub fn set_has_deleted_file(&mut self, deleted: bool) {
            self.has_deleted_file = deleted;
        }

        pub fn with_dirty(mut self, dirty: bool) -> Self {
            self.is_dirty = dirty;
            self
        }

        pub fn with_conflict(mut self, has_conflict: bool) -> Self {
            self.has_conflict = has_conflict;
            self
        }

        pub fn with_project_items(mut self, items: &[Entity<TestProjectItem>]) -> Self {
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

        pub fn set_state(&mut self, state: String, cx: &mut Context<Self>) {
            self.push_to_nav_history(cx);
            self.state = state;
        }

        fn push_to_nav_history(&mut self, cx: &mut Context<Self>) {
            if let Some(history) = &mut self.nav_history {
                history.push(Some(Box::new(self.state.clone())), cx);
            }
        }
    }

    impl Render for TestItem {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            gpui::div().track_focus(&self.focus_handle(cx))
        }
    }

    impl EventEmitter<ItemEvent> for TestItem {}

    impl Focusable for TestItem {
        fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl Item for TestItem {
        type Event = ItemEvent;

        fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
            f(*event)
        }

        fn tab_content_text(&self, detail: usize, _cx: &App) -> SharedString {
            self.tab_descriptions
                .as_ref()
                .and_then(|descriptions| {
                    let description = *descriptions.get(detail).or_else(|| descriptions.last())?;
                    description.into()
                })
                .unwrap_or_default()
                .into()
        }

        fn telemetry_event_text(&self) -> Option<&'static str> {
            None
        }

        fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
            self.tab_detail.set(params.detail);
            gpui::div().into_any_element()
        }

        fn for_each_project_item(
            &self,
            cx: &App,
            f: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
        ) {
            self.project_items
                .iter()
                .for_each(|item| f(item.entity_id(), item.read(cx)))
        }

        fn buffer_kind(&self, _: &App) -> ItemBufferKind {
            self.buffer_kind
        }

        fn set_nav_history(
            &mut self,
            history: ItemNavHistory,
            _window: &mut Window,
            _: &mut Context<Self>,
        ) {
            self.nav_history = Some(history);
        }

        fn navigate(
            &mut self,
            state: Arc<dyn Any + Send>,
            _window: &mut Window,
            _: &mut Context<Self>,
        ) -> bool {
            if let Some(state) = state.downcast_ref::<Box<String>>() {
                let state = *state.clone();
                if state != self.state {
                    false
                } else {
                    self.state = state;
                    true
                }
            } else {
                false
            }
        }

        fn deactivated(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
            self.push_to_nav_history(cx);
        }

        fn can_split(&self) -> bool {
            true
        }

        fn clone_on_split(
            &self,
            _workspace_id: Option<WorkspaceId>,
            _: &mut Window,
            cx: &mut Context<Self>,
        ) -> Task<Option<Entity<Self>>>
        where
            Self: Sized,
        {
            Task::ready(Some(cx.new(|cx| Self {
                state: self.state.clone(),
                label: self.label.clone(),
                save_count: self.save_count,
                save_as_count: self.save_as_count,
                reload_count: self.reload_count,
                is_dirty: self.is_dirty,
                buffer_kind: self.buffer_kind,
                has_conflict: self.has_conflict,
                has_deleted_file: self.has_deleted_file,
                project_items: self.project_items.clone(),
                nav_history: None,
                tab_descriptions: None,
                tab_detail: Default::default(),
                workspace_id: self.workspace_id,
                focus_handle: cx.focus_handle(),
                serialize: None,
            })))
        }

        fn is_dirty(&self, _: &App) -> bool {
            self.is_dirty
        }

        fn has_conflict(&self, _: &App) -> bool {
            self.has_conflict
        }

        fn has_deleted_file(&self, _: &App) -> bool {
            self.has_deleted_file
        }

        fn can_save(&self, cx: &App) -> bool {
            !self.project_items.is_empty()
                && self
                    .project_items
                    .iter()
                    .all(|item| item.read(cx).entry_id.is_some())
        }

        fn can_save_as(&self, _cx: &App) -> bool {
            self.buffer_kind == ItemBufferKind::Singleton
        }

        fn save(
            &mut self,
            _: SaveOptions,
            _: Entity<Project>,
            _window: &mut Window,
            cx: &mut Context<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_count += 1;
            self.is_dirty = false;
            for item in &self.project_items {
                item.update(cx, |item, _| {
                    if item.is_dirty {
                        item.is_dirty = false;
                    }
                })
            }
            Task::ready(Ok(()))
        }

        fn save_as(
            &mut self,
            _: Entity<Project>,
            _: ProjectPath,
            _window: &mut Window,
            _: &mut Context<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_as_count += 1;
            self.is_dirty = false;
            Task::ready(Ok(()))
        }

        fn reload(
            &mut self,
            _: Entity<Project>,
            _window: &mut Window,
            _: &mut Context<Self>,
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
            _project: Entity<Project>,
            _workspace: WeakEntity<Workspace>,
            workspace_id: WorkspaceId,
            _item_id: ItemId,
            _window: &mut Window,
            cx: &mut App,
        ) -> Task<anyhow::Result<Entity<Self>>> {
            let entity = cx.new(|cx| Self::new_deserialized(workspace_id, cx));
            Task::ready(Ok(entity))
        }

        fn cleanup(
            _workspace_id: WorkspaceId,
            _alive_items: Vec<ItemId>,
            _window: &mut Window,
            _cx: &mut App,
        ) -> Task<anyhow::Result<()>> {
            Task::ready(Ok(()))
        }

        fn serialize(
            &mut self,
            _workspace: &mut Workspace,
            _item_id: ItemId,
            _closing: bool,
            _window: &mut Window,
            _cx: &mut Context<Self>,
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
