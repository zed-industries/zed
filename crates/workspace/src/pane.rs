use crate::{
    item::{
        ClosePosition, Item, ItemHandle, ItemSettings, PreviewTabsSettings, TabContentParams,
        WeakItemHandle,
    },
    toolbar::Toolbar,
    workspace_settings::{AutosaveSetting, TabBarSettings, WorkspaceSettings},
    CloseWindow, NewCenterTerminal, NewFile, NewSearch, OpenInTerminal, OpenTerminal, OpenVisible,
    SplitDirection, ToggleZoom, Workspace,
};
use anyhow::Result;
use collections::{HashMap, HashSet, VecDeque};
use futures::{stream::FuturesUnordered, StreamExt};
use gpui::{
    actions, anchored, deferred, impl_actions, prelude::*, Action, AnchorCorner, AnyElement,
    AppContext, AsyncWindowContext, ClickEvent, DismissEvent, Div, DragMoveEvent, EntityId,
    EventEmitter, ExternalPaths, FocusHandle, FocusableView, KeyContext, Model, MouseButton,
    MouseDownEvent, NavigationDirection, Pixels, Point, PromptLevel, Render, ScrollHandle,
    Subscription, Task, View, ViewContext, VisualContext, WeakFocusHandle, WeakView, WindowContext,
};
use itertools::Itertools;
use parking_lot::Mutex;
use project::{Project, ProjectEntryId, ProjectPath};
use serde::Deserialize;
use settings::{Settings, SettingsStore};
use std::{
    any::Any,
    cmp, fmt, mem,
    ops::ControlFlow,
    path::PathBuf,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use theme::ThemeSettings;

use ui::{
    prelude::*, right_click_menu, ButtonSize, Color, IconButton, IconButtonShape, IconName,
    IconSize, Indicator, Label, Tab, TabBar, TabPosition, Tooltip,
};
use ui::{v_flex, ContextMenu};
use util::{debug_panic, maybe, truncate_and_remove_front, ResultExt};

#[derive(PartialEq, Clone, Copy, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SaveIntent {
    /// write all files (even if unchanged)
    /// prompt before overwriting on-disk changes
    Save,
    /// same as Save, but without auto formatting
    SaveWithoutFormat,
    /// write any files that have local changes
    /// prompt before overwriting on-disk changes
    SaveAll,
    /// always prompt for a new path
    SaveAs,
    /// prompt "you have unsaved changes" before writing
    Close,
    /// write all dirty files, don't prompt on conflict
    Overwrite,
    /// skip all save-related behavior
    Skip,
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct ActivateItem(pub usize);

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseActiveItem {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseInactiveItems {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseAllItems {
    pub save_intent: Option<SaveIntent>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RevealInProjectPanel {
    pub entry_id: Option<u64>,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct DeploySearch {
    #[serde(default)]
    pub replace_enabled: bool,
}

impl_actions!(
    pane,
    [
        CloseAllItems,
        CloseActiveItem,
        CloseInactiveItems,
        ActivateItem,
        RevealInProjectPanel,
        DeploySearch,
    ]
);

actions!(
    pane,
    [
        ActivatePrevItem,
        ActivateNextItem,
        ActivateLastItem,
        AlternateFile,
        CloseCleanItems,
        CloseItemsToTheLeft,
        CloseItemsToTheRight,
        GoBack,
        GoForward,
        ReopenClosedItem,
        SplitLeft,
        SplitUp,
        SplitRight,
        SplitDown,
        TogglePreviewTab,
    ]
);

impl DeploySearch {
    pub fn find() -> Self {
        Self {
            replace_enabled: false,
        }
    }
}

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub enum Event {
    AddItem { item: Box<dyn ItemHandle> },
    ActivateItem { local: bool },
    Remove,
    RemoveItem { item_id: EntityId },
    Split(SplitDirection),
    ChangeItemTitle,
    Focus,
    ZoomIn,
    ZoomOut,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::AddItem { item } => f
                .debug_struct("AddItem")
                .field("item", &item.item_id())
                .finish(),
            Event::ActivateItem { local } => f
                .debug_struct("ActivateItem")
                .field("local", local)
                .finish(),
            Event::Remove => f.write_str("Remove"),
            Event::RemoveItem { item_id } => f
                .debug_struct("RemoveItem")
                .field("item_id", item_id)
                .finish(),
            Event::Split(direction) => f
                .debug_struct("Split")
                .field("direction", direction)
                .finish(),
            Event::ChangeItemTitle => f.write_str("ChangeItemTitle"),
            Event::Focus => f.write_str("Focus"),
            Event::ZoomIn => f.write_str("ZoomIn"),
            Event::ZoomOut => f.write_str("ZoomOut"),
        }
    }
}

/// A container for 0 to many items that are open in the workspace.
/// Treats all items uniformly via the [`ItemHandle`] trait, whether it's an editor, search results multibuffer, terminal or something else,
/// responsible for managing item tabs, focus and zoom states and drag and drop features.
/// Can be split, see `PaneGroup` for more details.
pub struct Pane {
    alternate_file_items: (
        Option<Box<dyn WeakItemHandle>>,
        Option<Box<dyn WeakItemHandle>>,
    ),
    focus_handle: FocusHandle,
    items: Vec<Box<dyn ItemHandle>>,
    activation_history: Vec<ActivationHistoryEntry>,
    next_activation_timestamp: Arc<AtomicUsize>,
    zoomed: bool,
    was_focused: bool,
    active_item_index: usize,
    preview_item_id: Option<EntityId>,
    last_focus_handle_by_item: HashMap<EntityId, WeakFocusHandle>,
    nav_history: NavHistory,
    toolbar: View<Toolbar>,
    pub new_item_menu: Option<View<ContextMenu>>,
    split_item_menu: Option<View<ContextMenu>>,
    //     tab_context_menu: View<ContextMenu>,
    pub(crate) workspace: WeakView<Workspace>,
    project: Model<Project>,
    drag_split_direction: Option<SplitDirection>,
    can_drop_predicate: Option<Arc<dyn Fn(&dyn Any, &mut WindowContext) -> bool>>,
    custom_drop_handle:
        Option<Arc<dyn Fn(&mut Pane, &dyn Any, &mut ViewContext<Pane>) -> ControlFlow<(), ()>>>,
    can_split: bool,
    should_display_tab_bar: Rc<dyn Fn(&ViewContext<Pane>) -> bool>,
    render_tab_bar_buttons: Rc<dyn Fn(&mut Pane, &mut ViewContext<Pane>) -> AnyElement>,
    _subscriptions: Vec<Subscription>,
    tab_bar_scroll_handle: ScrollHandle,
    /// Is None if navigation buttons are permanently turned off (and should not react to setting changes).
    /// Otherwise, when `display_nav_history_buttons` is Some, it determines whether nav buttons should be displayed.
    display_nav_history_buttons: Option<bool>,
    double_click_dispatch_action: Box<dyn Action>,
    save_modals_spawned: HashSet<EntityId>,
}

pub struct ActivationHistoryEntry {
    pub entity_id: EntityId,
    pub timestamp: usize,
}

pub struct ItemNavHistory {
    history: NavHistory,
    item: Arc<dyn WeakItemHandle>,
    is_preview: bool,
}

#[derive(Clone)]
pub struct NavHistory(Arc<Mutex<NavHistoryState>>);

struct NavHistoryState {
    mode: NavigationMode,
    backward_stack: VecDeque<NavigationEntry>,
    forward_stack: VecDeque<NavigationEntry>,
    closed_stack: VecDeque<NavigationEntry>,
    paths_by_item: HashMap<EntityId, (ProjectPath, Option<PathBuf>)>,
    pane: WeakView<Pane>,
    next_timestamp: Arc<AtomicUsize>,
}

#[derive(Debug, Copy, Clone)]
pub enum NavigationMode {
    Normal,
    GoingBack,
    GoingForward,
    ClosingItem,
    ReopeningClosedItem,
    Disabled,
}

impl Default for NavigationMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct NavigationEntry {
    pub item: Arc<dyn WeakItemHandle>,
    pub data: Option<Box<dyn Any + Send>>,
    pub timestamp: usize,
    pub is_preview: bool,
}

#[derive(Clone)]
pub struct DraggedTab {
    pub pane: View<Pane>,
    pub item: Box<dyn ItemHandle>,
    pub ix: usize,
    pub detail: usize,
    pub is_active: bool,
}

impl EventEmitter<Event> for Pane {}

impl Pane {
    pub fn new(
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        next_timestamp: Arc<AtomicUsize>,
        can_drop_predicate: Option<Arc<dyn Fn(&dyn Any, &mut WindowContext) -> bool + 'static>>,
        double_click_dispatch_action: Box<dyn Action>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let subscriptions = vec![
            cx.on_focus(&focus_handle, Pane::focus_in),
            cx.on_focus_in(&focus_handle, Pane::focus_in),
            cx.on_focus_out(&focus_handle, Pane::focus_out),
            cx.observe_global::<SettingsStore>(Self::settings_changed),
        ];

        let handle = cx.view().downgrade();
        Self {
            alternate_file_items: (None, None),
            focus_handle,
            items: Vec::new(),
            activation_history: Vec::new(),
            next_activation_timestamp: next_timestamp.clone(),
            was_focused: false,
            zoomed: false,
            active_item_index: 0,
            preview_item_id: None,
            last_focus_handle_by_item: Default::default(),
            nav_history: NavHistory(Arc::new(Mutex::new(NavHistoryState {
                mode: NavigationMode::Normal,
                backward_stack: Default::default(),
                forward_stack: Default::default(),
                closed_stack: Default::default(),
                paths_by_item: Default::default(),
                pane: handle.clone(),
                next_timestamp,
            }))),
            toolbar: cx.new_view(|_| Toolbar::new()),
            new_item_menu: None,
            split_item_menu: None,
            tab_bar_scroll_handle: ScrollHandle::new(),
            drag_split_direction: None,
            workspace,
            project,
            can_drop_predicate,
            custom_drop_handle: None,
            can_split: true,
            should_display_tab_bar: Rc::new(|cx| TabBarSettings::get_global(cx).show),
            render_tab_bar_buttons: Rc::new(move |pane, cx| {
                // Ideally we would return a vec of elements here to pass directly to the [TabBar]'s
                // `end_slot`, but due to needing a view here that isn't possible.
                h_flex()
                    // Instead we need to replicate the spacing from the [TabBar]'s `end_slot` here.
                    .gap(Spacing::Small.rems(cx))
                    .child(
                        IconButton::new("plus", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|pane, _, cx| {
                                let menu = ContextMenu::build(cx, |menu, _| {
                                    menu.action("New File", NewFile.boxed_clone())
                                        .action("New Terminal", NewCenterTerminal.boxed_clone())
                                        .action("New Search", NewSearch.boxed_clone())
                                });
                                cx.subscribe(&menu, |pane, _, _: &DismissEvent, cx| {
                                    pane.focus(cx);
                                    pane.new_item_menu = None;
                                })
                                .detach();
                                pane.new_item_menu = Some(menu);
                            }))
                            .tooltip(|cx| Tooltip::text("New...", cx)),
                    )
                    .when_some(pane.new_item_menu.as_ref(), |el, new_item_menu| {
                        el.child(Self::render_menu_overlay(new_item_menu))
                    })
                    .child(
                        IconButton::new("split", IconName::Split)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|pane, _, cx| {
                                let menu = ContextMenu::build(cx, |menu, _| {
                                    menu.action("Split Right", SplitRight.boxed_clone())
                                        .action("Split Left", SplitLeft.boxed_clone())
                                        .action("Split Up", SplitUp.boxed_clone())
                                        .action("Split Down", SplitDown.boxed_clone())
                                });
                                cx.subscribe(&menu, |pane, _, _: &DismissEvent, cx| {
                                    pane.focus(cx);
                                    pane.split_item_menu = None;
                                })
                                .detach();
                                pane.split_item_menu = Some(menu);
                            }))
                            .tooltip(|cx| Tooltip::text("Split Pane", cx)),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        IconButton::new("toggle_zoom", IconName::Maximize)
                            .icon_size(IconSize::Small)
                            .selected(zoomed)
                            .selected_icon(IconName::Minimize)
                            .on_click(cx.listener(|pane, _, cx| {
                                pane.toggle_zoom(&crate::ToggleZoom, cx);
                            }))
                            .tooltip(move |cx| {
                                Tooltip::for_action(
                                    if zoomed { "Zoom Out" } else { "Zoom In" },
                                    &ToggleZoom,
                                    cx,
                                )
                            })
                    })
                    .when_some(pane.split_item_menu.as_ref(), |el, split_item_menu| {
                        el.child(Self::render_menu_overlay(split_item_menu))
                    })
                    .into_any_element()
            }),
            display_nav_history_buttons: Some(
                TabBarSettings::get_global(cx).show_nav_history_buttons,
            ),
            _subscriptions: subscriptions,
            double_click_dispatch_action,
            save_modals_spawned: HashSet::default(),
        }
    }

    fn alternate_file(&mut self, cx: &mut ViewContext<Pane>) {
        let (_, alternative) = &self.alternate_file_items;
        if let Some(alternative) = alternative {
            let existing = self
                .items()
                .find_position(|item| item.item_id() == alternative.id());
            if let Some((ix, _)) = existing {
                self.activate_item(ix, true, true, cx);
            } else {
                if let Some(upgraded) = alternative.upgrade() {
                    self.add_item(upgraded, true, true, None, cx);
                }
            }
        }
    }

    pub fn track_alternate_file_items(&mut self) {
        if let Some(item) = self.active_item().map(|item| item.downgrade_item()) {
            let (current, _) = &self.alternate_file_items;
            match current {
                Some(current) => {
                    if current.id() != item.id() {
                        self.alternate_file_items =
                            (Some(item), self.alternate_file_items.0.take());
                    }
                }
                None => {
                    self.alternate_file_items = (Some(item), None);
                }
            }
        }
    }

    pub fn has_focus(&self, cx: &WindowContext) -> bool {
        // We not only check whether our focus handle contains focus, but also
        // whether the active_item might have focus, because we might have just activated an item
        // but that hasn't rendered yet.
        // So before the next render, we might have transferred focus
        // to the item and `focus_handle.contains_focus` returns false because the `active_item`
        // is not hooked up to us in the dispatch tree.
        self.focus_handle.contains_focused(cx)
            || self
                .active_item()
                .map_or(false, |item| item.focus_handle(cx).contains_focused(cx))
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.was_focused {
            self.was_focused = true;
            cx.emit(Event::Focus);
            cx.notify();
        }

        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.focus_changed(true, cx);
        });

        if let Some(active_item) = self.active_item() {
            if self.focus_handle.is_focused(cx) {
                // Pane was focused directly. We need to either focus a view inside the active item,
                // or focus the active item itself
                if let Some(weak_last_focus_handle) =
                    self.last_focus_handle_by_item.get(&active_item.item_id())
                {
                    if let Some(focus_handle) = weak_last_focus_handle.upgrade() {
                        focus_handle.focus(cx);
                        return;
                    }
                }

                active_item.focus_handle(cx).focus(cx);
            } else if let Some(focused) = cx.focused() {
                if !self.context_menu_focused(cx) {
                    self.last_focus_handle_by_item
                        .insert(active_item.item_id(), focused.downgrade());
                }
            }
        }
    }

    fn context_menu_focused(&self, cx: &mut ViewContext<Self>) -> bool {
        self.new_item_menu
            .as_ref()
            .or(self.split_item_menu.as_ref())
            .map_or(false, |menu| menu.focus_handle(cx).is_focused(cx))
    }

    fn focus_out(&mut self, cx: &mut ViewContext<Self>) {
        self.was_focused = false;
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.focus_changed(false, cx);
        });
        cx.notify();
    }

    fn settings_changed(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(display_nav_history_buttons) = self.display_nav_history_buttons.as_mut() {
            *display_nav_history_buttons = TabBarSettings::get_global(cx).show_nav_history_buttons;
        }
        if !PreviewTabsSettings::get_global(cx).enabled {
            self.preview_item_id = None;
        }
        cx.notify();
    }

    pub fn active_item_index(&self) -> usize {
        self.active_item_index
    }

    pub fn activation_history(&self) -> &[ActivationHistoryEntry] {
        &self.activation_history
    }

    pub fn set_should_display_tab_bar<F>(&mut self, should_display_tab_bar: F)
    where
        F: 'static + Fn(&ViewContext<Pane>) -> bool,
    {
        self.should_display_tab_bar = Rc::new(should_display_tab_bar);
    }

    pub fn set_can_split(&mut self, can_split: bool, cx: &mut ViewContext<Self>) {
        self.can_split = can_split;
        cx.notify();
    }

    pub fn set_can_navigate(&mut self, can_navigate: bool, cx: &mut ViewContext<Self>) {
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.set_can_navigate(can_navigate, cx);
        });
        cx.notify();
    }

    pub fn set_render_tab_bar_buttons<F>(&mut self, cx: &mut ViewContext<Self>, render: F)
    where
        F: 'static + Fn(&mut Pane, &mut ViewContext<Pane>) -> AnyElement,
    {
        self.render_tab_bar_buttons = Rc::new(render);
        cx.notify();
    }

    pub fn set_custom_drop_handle<F>(&mut self, cx: &mut ViewContext<Self>, handle: F)
    where
        F: 'static + Fn(&mut Pane, &dyn Any, &mut ViewContext<Pane>) -> ControlFlow<(), ()>,
    {
        self.custom_drop_handle = Some(Arc::new(handle));
        cx.notify();
    }

    pub fn nav_history_for_item<T: Item>(&self, item: &View<T>) -> ItemNavHistory {
        ItemNavHistory {
            history: self.nav_history.clone(),
            item: Arc::new(item.downgrade()),
            is_preview: self.preview_item_id == Some(item.item_id()),
        }
    }

    pub fn nav_history(&self) -> &NavHistory {
        &self.nav_history
    }

    pub fn nav_history_mut(&mut self) -> &mut NavHistory {
        &mut self.nav_history
    }

    pub fn disable_history(&mut self) {
        self.nav_history.disable();
    }

    pub fn enable_history(&mut self) {
        self.nav_history.enable();
    }

    pub fn can_navigate_backward(&self) -> bool {
        !self.nav_history.0.lock().backward_stack.is_empty()
    }

    pub fn can_navigate_forward(&self) -> bool {
        !self.nav_history.0.lock().forward_stack.is_empty()
    }

    fn navigate_backward(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let pane = cx.view().downgrade();
            cx.window_context().defer(move |cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.go_back(pane, cx).detach_and_log_err(cx)
                })
            })
        }
    }

    fn navigate_forward(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let pane = cx.view().downgrade();
            cx.window_context().defer(move |cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.go_forward(pane, cx).detach_and_log_err(cx)
                })
            })
        }
    }

    fn history_updated(&mut self, cx: &mut ViewContext<Self>) {
        self.toolbar.update(cx, |_, cx| cx.notify());
    }

    pub fn preview_item_id(&self) -> Option<EntityId> {
        self.preview_item_id
    }

    fn preview_item_idx(&self) -> Option<usize> {
        if let Some(preview_item_id) = self.preview_item_id {
            self.items
                .iter()
                .position(|item| item.item_id() == preview_item_id)
        } else {
            None
        }
    }

    pub fn is_active_preview_item(&self, item_id: EntityId) -> bool {
        self.preview_item_id == Some(item_id)
    }

    /// Marks the item with the given ID as the preview item.
    /// This will be ignored if the global setting `preview_tabs` is disabled.
    pub fn set_preview_item_id(&mut self, item_id: Option<EntityId>, cx: &AppContext) {
        if PreviewTabsSettings::get_global(cx).enabled {
            self.preview_item_id = item_id;
        }
    }

    pub fn handle_item_edit(&mut self, item_id: EntityId, cx: &AppContext) {
        if let Some(preview_item_id) = self.preview_item_id {
            if preview_item_id == item_id {
                self.set_preview_item_id(None, cx)
            }
        }
    }

    pub(crate) fn open_item(
        &mut self,
        project_entry_id: Option<ProjectEntryId>,
        focus_item: bool,
        allow_preview: bool,
        cx: &mut ViewContext<Self>,
        build_item: impl FnOnce(&mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
    ) -> Box<dyn ItemHandle> {
        let mut existing_item = None;
        if let Some(project_entry_id) = project_entry_id {
            for (index, item) in self.items.iter().enumerate() {
                if item.is_singleton(cx)
                    && item.project_entry_ids(cx).as_slice() == [project_entry_id]
                {
                    let item = item.boxed_clone();
                    existing_item = Some((index, item));
                    break;
                }
            }
        }

        if let Some((index, existing_item)) = existing_item {
            // If the item is already open, and the item is a preview item
            // and we are not allowing items to open as preview, mark the item as persistent.
            if let Some(preview_item_id) = self.preview_item_id {
                if let Some(tab) = self.items.get(index) {
                    if tab.item_id() == preview_item_id && !allow_preview {
                        self.set_preview_item_id(None, cx);
                    }
                }
            }

            self.activate_item(index, focus_item, focus_item, cx);
            existing_item
        } else {
            // If the item is being opened as preview and we have an existing preview tab,
            // open the new item in the position of the existing preview tab.
            let destination_index = if allow_preview {
                self.close_current_preview_item(cx)
            } else {
                None
            };

            let new_item = build_item(cx);

            if allow_preview {
                self.set_preview_item_id(Some(new_item.item_id()), cx);
            }

            self.add_item(new_item.clone(), true, focus_item, destination_index, cx);

            new_item
        }
    }

    pub fn close_current_preview_item(&mut self, cx: &mut ViewContext<Self>) -> Option<usize> {
        let Some(item_idx) = self.preview_item_idx() else {
            return None;
        };

        let prev_active_item_index = self.active_item_index;
        self.remove_item(item_idx, false, false, cx);
        self.active_item_index = prev_active_item_index;

        if item_idx < self.items.len() {
            Some(item_idx)
        } else {
            None
        }
    }

    pub fn add_item(
        &mut self,
        item: Box<dyn ItemHandle>,
        activate_pane: bool,
        focus_item: bool,
        destination_index: Option<usize>,
        cx: &mut ViewContext<Self>,
    ) {
        if item.is_singleton(cx) {
            if let Some(&entry_id) = item.project_entry_ids(cx).get(0) {
                let project = self.project.read(cx);
                if let Some(project_path) = project.path_for_entry(entry_id, cx) {
                    let abs_path = project.absolute_path(&project_path, cx);
                    self.nav_history
                        .0
                        .lock()
                        .paths_by_item
                        .insert(item.item_id(), (project_path, abs_path));
                }
            }
        }
        // If no destination index is specified, add or move the item after the active item.
        let mut insertion_index = {
            cmp::min(
                if let Some(destination_index) = destination_index {
                    destination_index
                } else {
                    self.active_item_index + 1
                },
                self.items.len(),
            )
        };

        // Does the item already exist?
        let project_entry_id = if item.is_singleton(cx) {
            item.project_entry_ids(cx).get(0).copied()
        } else {
            None
        };

        let existing_item_index = self.items.iter().position(|existing_item| {
            if existing_item.item_id() == item.item_id() {
                true
            } else if existing_item.is_singleton(cx) {
                existing_item
                    .project_entry_ids(cx)
                    .get(0)
                    .map_or(false, |existing_entry_id| {
                        Some(existing_entry_id) == project_entry_id.as_ref()
                    })
            } else {
                false
            }
        });

        if let Some(existing_item_index) = existing_item_index {
            // If the item already exists, move it to the desired destination and activate it

            if existing_item_index != insertion_index {
                let existing_item_is_active = existing_item_index == self.active_item_index;

                // If the caller didn't specify a destination and the added item is already
                // the active one, don't move it
                if existing_item_is_active && destination_index.is_none() {
                    insertion_index = existing_item_index;
                } else {
                    self.items.remove(existing_item_index);
                    if existing_item_index < self.active_item_index {
                        self.active_item_index -= 1;
                    }
                    insertion_index = insertion_index.min(self.items.len());

                    self.items.insert(insertion_index, item.clone());

                    if existing_item_is_active {
                        self.active_item_index = insertion_index;
                    } else if insertion_index <= self.active_item_index {
                        self.active_item_index += 1;
                    }
                }

                cx.notify();
            }

            self.activate_item(insertion_index, activate_pane, focus_item, cx);
        } else {
            self.items.insert(insertion_index, item.clone());

            if insertion_index <= self.active_item_index
                && self.preview_item_idx() != Some(self.active_item_index)
            {
                self.active_item_index += 1;
            }

            self.activate_item(insertion_index, activate_pane, focus_item, cx);
            cx.notify();
        }

        cx.emit(Event::AddItem { item });
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> impl DoubleEndedIterator<Item = &Box<dyn ItemHandle>> {
        self.items.iter()
    }

    pub fn items_of_type<T: Render>(&self) -> impl '_ + Iterator<Item = View<T>> {
        self.items
            .iter()
            .filter_map(|item| item.to_any().downcast().ok())
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.items.get(self.active_item_index).cloned()
    }

    pub fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Point<Pixels>> {
        self.items
            .get(self.active_item_index)?
            .pixel_position_of_cursor(cx)
    }

    pub fn item_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Box<dyn ItemHandle>> {
        self.items.iter().find_map(|item| {
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == [entry_id] {
                Some(item.boxed_clone())
            } else {
                None
            }
        })
    }

    pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize> {
        self.items
            .iter()
            .position(|i| i.item_id() == item.item_id())
    }

    pub fn item_for_index(&self, ix: usize) -> Option<&dyn ItemHandle> {
        self.items.get(ix).map(|i| i.as_ref())
    }

    pub fn toggle_zoom(&mut self, _: &ToggleZoom, cx: &mut ViewContext<Self>) {
        if self.zoomed {
            cx.emit(Event::ZoomOut);
        } else if !self.items.is_empty() {
            if !self.focus_handle.contains_focused(cx) {
                cx.focus_self();
            }
            cx.emit(Event::ZoomIn);
        }
    }

    pub fn activate_item(
        &mut self,
        index: usize,
        activate_pane: bool,
        focus_item: bool,
        cx: &mut ViewContext<Self>,
    ) {
        use NavigationMode::{GoingBack, GoingForward};

        if index < self.items.len() {
            let prev_active_item_ix = mem::replace(&mut self.active_item_index, index);
            if prev_active_item_ix != self.active_item_index
                || matches!(self.nav_history.mode(), GoingBack | GoingForward)
            {
                if let Some(prev_item) = self.items.get(prev_active_item_ix) {
                    prev_item.deactivated(cx);
                }
            }
            cx.emit(Event::ActivateItem {
                local: activate_pane,
            });

            if let Some(newly_active_item) = self.items.get(index) {
                self.activation_history
                    .retain(|entry| entry.entity_id != newly_active_item.item_id());
                self.activation_history.push(ActivationHistoryEntry {
                    entity_id: newly_active_item.item_id(),
                    timestamp: self
                        .next_activation_timestamp
                        .fetch_add(1, Ordering::SeqCst),
                });
            }

            self.update_toolbar(cx);
            self.update_status_bar(cx);

            if focus_item {
                self.focus_active_item(cx);
            }

            self.tab_bar_scroll_handle.scroll_to_item(index);
            cx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, activate_pane: bool, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index > 0 {
            index -= 1;
        } else if !self.items.is_empty() {
            index = self.items.len() - 1;
        }
        self.activate_item(index, activate_pane, activate_pane, cx);
    }

    pub fn activate_next_item(&mut self, activate_pane: bool, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index + 1 < self.items.len() {
            index += 1;
        } else {
            index = 0;
        }
        self.activate_item(index, activate_pane, activate_pane, cx);
    }

    pub fn close_active_item(
        &mut self,
        action: &CloseActiveItem,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            // Close the window when there's no active items to close, if configured
            if WorkspaceSettings::get_global(cx)
                .when_closing_with_no_tabs
                .should_close()
            {
                cx.dispatch_action(Box::new(CloseWindow));
            }

            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        Some(self.close_item_by_id(
            active_item_id,
            action.save_intent.unwrap_or(SaveIntent::Close),
            cx,
        ))
    }

    pub fn close_item_by_id(
        &mut self,
        item_id_to_close: EntityId,
        save_intent: SaveIntent,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.close_items(cx, save_intent, move |view_id| view_id == item_id_to_close)
    }

    pub fn close_inactive_items(
        &mut self,
        action: &CloseInactiveItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        let active_item_id = self.items[self.active_item_index].item_id();
        Some(self.close_items(
            cx,
            action.save_intent.unwrap_or(SaveIntent::Close),
            move |item_id| item_id != active_item_id,
        ))
    }

    pub fn close_clean_items(
        &mut self,
        _: &CloseCleanItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let item_ids: Vec<_> = self
            .items()
            .filter(|item| !item.is_dirty(cx))
            .map(|item| item.item_id())
            .collect();
        Some(self.close_items(cx, SaveIntent::Close, move |item_id| {
            item_ids.contains(&item_id)
        }))
    }

    pub fn close_items_to_the_left(
        &mut self,
        _: &CloseItemsToTheLeft,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        Some(self.close_items_to_the_left_by_id(active_item_id, cx))
    }

    pub fn close_items_to_the_left_by_id(
        &mut self,
        item_id: EntityId,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .take_while(|item| item.item_id() != item_id)
            .map(|item| item.item_id())
            .collect();
        self.close_items(cx, SaveIntent::Close, move |item_id| {
            item_ids.contains(&item_id)
        })
    }

    pub fn close_items_to_the_right(
        &mut self,
        _: &CloseItemsToTheRight,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        Some(self.close_items_to_the_right_by_id(active_item_id, cx))
    }

    pub fn close_items_to_the_right_by_id(
        &mut self,
        item_id: EntityId,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .rev()
            .take_while(|item| item.item_id() != item_id)
            .map(|item| item.item_id())
            .collect();
        self.close_items(cx, SaveIntent::Close, move |item_id| {
            item_ids.contains(&item_id)
        })
    }

    pub fn close_all_items(
        &mut self,
        action: &CloseAllItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        Some(
            self.close_items(cx, action.save_intent.unwrap_or(SaveIntent::Close), |_| {
                true
            }),
        )
    }

    pub(super) fn file_names_for_prompt(
        items: &mut dyn Iterator<Item = &Box<dyn ItemHandle>>,
        all_dirty_items: usize,
        cx: &AppContext,
    ) -> (String, String) {
        /// Quantity of item paths displayed in prompt prior to cutoff..
        const FILE_NAMES_CUTOFF_POINT: usize = 10;
        let mut file_names: Vec<_> = items
            .filter_map(|item| {
                item.project_path(cx).and_then(|project_path| {
                    project_path
                        .path
                        .file_name()
                        .and_then(|name| name.to_str().map(ToOwned::to_owned))
                })
            })
            .take(FILE_NAMES_CUTOFF_POINT)
            .collect();
        let should_display_followup_text =
            all_dirty_items > FILE_NAMES_CUTOFF_POINT || file_names.len() != all_dirty_items;
        if should_display_followup_text {
            let not_shown_files = all_dirty_items - file_names.len();
            if not_shown_files == 1 {
                file_names.push(".. 1 file not shown".into());
            } else {
                file_names.push(format!(".. {} files not shown", not_shown_files));
            }
        }
        (
            format!(
                "Do you want to save changes to the following {} files?",
                all_dirty_items
            ),
            file_names.join("\n"),
        )
    }

    pub fn close_items(
        &mut self,
        cx: &mut ViewContext<Pane>,
        mut save_intent: SaveIntent,
        should_close: impl Fn(EntityId) -> bool,
    ) -> Task<Result<()>> {
        // Find the items to close.
        let mut items_to_close = Vec::new();
        let mut dirty_items = Vec::new();
        for item in &self.items {
            if should_close(item.item_id()) {
                items_to_close.push(item.boxed_clone());
                if item.is_dirty(cx) {
                    dirty_items.push(item.boxed_clone());
                }
            }
        }

        // If a buffer is open both in a singleton editor and in a multibuffer, make sure
        // to focus the singleton buffer when prompting to save that buffer, as opposed
        // to focusing the multibuffer, because this gives the user a more clear idea
        // of what content they would be saving.
        items_to_close.sort_by_key(|item| !item.is_singleton(cx));

        let workspace = self.workspace.clone();
        cx.spawn(|pane, mut cx| async move {
            if save_intent == SaveIntent::Close && dirty_items.len() > 1 {
                let answer = pane.update(&mut cx, |_, cx| {
                    let (prompt, detail) =
                        Self::file_names_for_prompt(&mut dirty_items.iter(), dirty_items.len(), cx);
                    cx.prompt(
                        PromptLevel::Warning,
                        &prompt,
                        Some(&detail),
                        &["Save all", "Discard all", "Cancel"],
                    )
                })?;
                match answer.await {
                    Ok(0) => save_intent = SaveIntent::SaveAll,
                    Ok(1) => save_intent = SaveIntent::Skip,
                    _ => {}
                }
            }
            let mut saved_project_items_ids = HashSet::default();
            for item in items_to_close.clone() {
                // Find the item's current index and its set of project item models. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let (item_ix, mut project_item_ids) = pane.update(&mut cx, |pane, cx| {
                    (pane.index_for_item(&*item), item.project_item_model_ids(cx))
                })?;
                let item_ix = if let Some(ix) = item_ix {
                    ix
                } else {
                    continue;
                };

                // Check if this view has any project items that are not open anywhere else
                // in the workspace, AND that the user has not already been prompted to save.
                // If there are any such project entries, prompt the user to save this item.
                let project = workspace.update(&mut cx, |workspace, cx| {
                    for item in workspace.items(cx) {
                        if !items_to_close
                            .iter()
                            .any(|item_to_close| item_to_close.item_id() == item.item_id())
                        {
                            let other_project_item_ids = item.project_item_model_ids(cx);
                            project_item_ids.retain(|id| !other_project_item_ids.contains(id));
                        }
                    }
                    workspace.project().clone()
                })?;
                let should_save = project_item_ids
                    .iter()
                    .any(|id| saved_project_items_ids.insert(*id));

                if should_save
                    && !Self::save_item(
                        project.clone(),
                        &pane,
                        item_ix,
                        &*item,
                        save_intent,
                        &mut cx,
                    )
                    .await?
                {
                    break;
                }

                // Remove the item from the pane.
                pane.update(&mut cx, |pane, cx| {
                    if let Some(item_ix) = pane
                        .items
                        .iter()
                        .position(|i| i.item_id() == item.item_id())
                    {
                        pane.remove_item(item_ix, false, true, cx);
                    }
                })
                .ok();
            }

            pane.update(&mut cx, |_, cx| cx.notify()).ok();
            Ok(())
        })
    }

    pub fn remove_item(
        &mut self,
        item_index: usize,
        activate_pane: bool,
        close_pane_if_empty: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.activation_history
            .retain(|entry| entry.entity_id != self.items[item_index].item_id());

        if item_index == self.active_item_index {
            let index_to_activate = self
                .activation_history
                .pop()
                .and_then(|last_activated_item| {
                    self.items.iter().enumerate().find_map(|(index, item)| {
                        (item.item_id() == last_activated_item.entity_id).then_some(index)
                    })
                })
                // We didn't have a valid activation history entry, so fallback
                // to activating the item to the left
                .unwrap_or_else(|| item_index.min(self.items.len()).saturating_sub(1));

            let should_activate = activate_pane || self.has_focus(cx);
            if self.items.len() == 1 && should_activate {
                self.focus_handle.focus(cx);
            } else {
                self.activate_item(index_to_activate, should_activate, should_activate, cx);
            }
        }

        let item = self.items.remove(item_index);

        cx.emit(Event::RemoveItem {
            item_id: item.item_id(),
        });
        if self.items.is_empty() {
            item.deactivated(cx);
            if close_pane_if_empty {
                self.update_toolbar(cx);
                cx.emit(Event::Remove);
            }
        }

        if item_index < self.active_item_index {
            self.active_item_index -= 1;
        }

        let mode = self.nav_history.mode();
        self.nav_history.set_mode(NavigationMode::ClosingItem);
        item.deactivated(cx);
        self.nav_history.set_mode(mode);

        if self.is_active_preview_item(item.item_id()) {
            self.set_preview_item_id(None, cx);
        }

        if let Some(path) = item.project_path(cx) {
            let abs_path = self
                .nav_history
                .0
                .lock()
                .paths_by_item
                .get(&item.item_id())
                .and_then(|(_, abs_path)| abs_path.clone());

            self.nav_history
                .0
                .lock()
                .paths_by_item
                .insert(item.item_id(), (path, abs_path));
        } else {
            self.nav_history
                .0
                .lock()
                .paths_by_item
                .remove(&item.item_id());
        }

        if self.items.is_empty() && close_pane_if_empty && self.zoomed {
            cx.emit(Event::ZoomOut);
        }

        cx.notify();
    }

    pub async fn save_item(
        project: Model<Project>,
        pane: &WeakView<Pane>,
        item_ix: usize,
        item: &dyn ItemHandle,
        save_intent: SaveIntent,
        cx: &mut AsyncWindowContext,
    ) -> Result<bool> {
        const CONFLICT_MESSAGE: &str =
                "This file has changed on disk since you started editing it. Do you want to overwrite it?";

        if save_intent == SaveIntent::Skip {
            return Ok(true);
        }

        let (mut has_conflict, mut is_dirty, mut can_save, can_save_as) = cx.update(|cx| {
            (
                item.has_conflict(cx),
                item.is_dirty(cx),
                item.can_save(cx),
                item.is_singleton(cx),
            )
        })?;

        // when saving a single buffer, we ignore whether or not it's dirty.
        if save_intent == SaveIntent::Save || save_intent == SaveIntent::SaveWithoutFormat {
            is_dirty = true;
        }

        if save_intent == SaveIntent::SaveAs {
            is_dirty = true;
            has_conflict = false;
            can_save = false;
        }

        if save_intent == SaveIntent::Overwrite {
            has_conflict = false;
        }

        let should_format = save_intent != SaveIntent::SaveWithoutFormat;

        if has_conflict && can_save {
            let answer = pane.update(cx, |pane, cx| {
                pane.activate_item(item_ix, true, true, cx);
                cx.prompt(
                    PromptLevel::Warning,
                    CONFLICT_MESSAGE,
                    None,
                    &["Overwrite", "Discard", "Cancel"],
                )
            })?;
            match answer.await {
                Ok(0) => {
                    pane.update(cx, |_, cx| item.save(should_format, project, cx))?
                        .await?
                }
                Ok(1) => pane.update(cx, |_, cx| item.reload(project, cx))?.await?,
                _ => return Ok(false),
            }
        } else if is_dirty && (can_save || can_save_as) {
            if save_intent == SaveIntent::Close {
                let will_autosave = cx.update(|cx| {
                    matches!(
                        WorkspaceSettings::get_global(cx).autosave,
                        AutosaveSetting::OnFocusChange | AutosaveSetting::OnWindowChange
                    ) && Self::can_autosave_item(item, cx)
                })?;
                if !will_autosave {
                    let item_id = item.item_id();
                    let answer_task = pane.update(cx, |pane, cx| {
                        if pane.save_modals_spawned.insert(item_id) {
                            pane.activate_item(item_ix, true, true, cx);
                            let prompt = dirty_message_for(item.project_path(cx));
                            Some(cx.prompt(
                                PromptLevel::Warning,
                                &prompt,
                                None,
                                &["Save", "Don't Save", "Cancel"],
                            ))
                        } else {
                            None
                        }
                    })?;
                    if let Some(answer_task) = answer_task {
                        let answer = answer_task.await;
                        pane.update(cx, |pane, _| {
                            if !pane.save_modals_spawned.remove(&item_id) {
                                debug_panic!(
                                    "save modal was not present in spawned modals after awaiting for its answer"
                                )
                            }
                        })?;
                        match answer {
                            Ok(0) => {}
                            Ok(1) => return Ok(true), // Don't save this file
                            _ => return Ok(false),    // Cancel
                        }
                    } else {
                        return Ok(false);
                    }
                }
            }

            if can_save {
                pane.update(cx, |_, cx| item.save(should_format, project, cx))?
                    .await?;
            } else if can_save_as {
                let abs_path = pane.update(cx, |pane, cx| {
                    pane.workspace
                        .update(cx, |workspace, cx| workspace.prompt_for_new_path(cx))
                })??;
                if let Some(abs_path) = abs_path.await.ok().flatten() {
                    pane.update(cx, |_, cx| item.save_as(project, abs_path, cx))?
                        .await?;
                } else {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn can_autosave_item(item: &dyn ItemHandle, cx: &AppContext) -> bool {
        let is_deleted = item.project_entry_ids(cx).is_empty();
        item.is_dirty(cx) && !item.has_conflict(cx) && item.can_save(cx) && !is_deleted
    }

    pub fn autosave_item(
        item: &dyn ItemHandle,
        project: Model<Project>,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        let format = if let AutosaveSetting::AfterDelay { .. } =
            WorkspaceSettings::get_global(cx).autosave
        {
            false
        } else {
            true
        };
        if Self::can_autosave_item(item, cx) {
            item.save(format, project, cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn focus(&mut self, cx: &mut ViewContext<Pane>) {
        cx.focus(&self.focus_handle);
    }

    pub fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            let focus_handle = active_item.focus_handle(cx);
            cx.focus(&focus_handle);
        }
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
    }

    pub fn toolbar(&self) -> &View<Toolbar> {
        &self.toolbar
    }

    pub fn handle_deleted_project_item(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Pane>,
    ) -> Option<()> {
        let (item_index_to_delete, item_id) = self.items().enumerate().find_map(|(i, item)| {
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == [entry_id] {
                Some((i, item.item_id()))
            } else {
                None
            }
        })?;

        self.remove_item(item_index_to_delete, false, true, cx);
        self.nav_history.remove_item(item_id);

        Some(())
    }

    fn update_toolbar(&mut self, cx: &mut ViewContext<Self>) {
        let active_item = self
            .items
            .get(self.active_item_index)
            .map(|item| item.as_ref());
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.set_active_item(active_item, cx);
        });
    }

    fn update_status_bar(&mut self, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        let pane = cx.view().clone();

        cx.window_context().defer(move |cx| {
            let Ok(status_bar) = workspace.update(cx, |workspace, _| workspace.status_bar.clone())
            else {
                return;
            };

            status_bar.update(cx, move |status_bar, cx| {
                status_bar.set_active_pane(&pane, cx);
            });
        });
    }

    fn render_tab(
        &self,
        ix: usize,
        item: &Box<dyn ItemHandle>,
        detail: usize,
        cx: &mut ViewContext<'_, Pane>,
    ) -> impl IntoElement {
        let is_active = ix == self.active_item_index;
        let is_preview = self
            .preview_item_id
            .map(|id| id == item.item_id())
            .unwrap_or(false);

        let label = item.tab_content(
            TabContentParams {
                detail: Some(detail),
                selected: is_active,
                preview: is_preview,
            },
            cx,
        );
        let close_side = &ItemSettings::get_global(cx).close_position;
        let indicator = render_item_indicator(item.boxed_clone(), cx);
        let item_id = item.item_id();
        let is_first_item = ix == 0;
        let is_last_item = ix == self.items.len() - 1;
        let position_relative_to_active_item = ix.cmp(&self.active_item_index);

        let tab = Tab::new(ix)
            .position(if is_first_item {
                TabPosition::First
            } else if is_last_item {
                TabPosition::Last
            } else {
                TabPosition::Middle(position_relative_to_active_item)
            })
            .close_side(match close_side {
                ClosePosition::Left => ui::TabCloseSide::Start,
                ClosePosition::Right => ui::TabCloseSide::End,
            })
            .selected(is_active)
            .on_click(
                cx.listener(move |pane: &mut Self, _, cx| pane.activate_item(ix, true, true, cx)),
            )
            // TODO: This should be a click listener with the middle mouse button instead of a mouse down listener.
            .on_mouse_down(
                MouseButton::Middle,
                cx.listener(move |pane, _event, cx| {
                    pane.close_item_by_id(item_id, SaveIntent::Close, cx)
                        .detach_and_log_err(cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |pane, event: &MouseDownEvent, cx| {
                    if let Some(id) = pane.preview_item_id {
                        if id == item_id && event.click_count > 1 {
                            pane.set_preview_item_id(None, cx);
                        }
                    }
                }),
            )
            .on_drag(
                DraggedTab {
                    item: item.boxed_clone(),
                    pane: cx.view().clone(),
                    detail,
                    is_active,
                    ix,
                },
                |tab, cx| cx.new_view(|_| tab.clone()),
            )
            .drag_over::<DraggedTab>(|tab, _, cx| {
                tab.bg(cx.theme().colors().drop_target_background)
            })
            .drag_over::<ProjectEntryId>(|tab, _, cx| {
                tab.bg(cx.theme().colors().drop_target_background)
            })
            .when_some(self.can_drop_predicate.clone(), |this, p| {
                this.can_drop(move |a, cx| p(a, cx))
            })
            .on_drop(cx.listener(move |this, dragged_tab: &DraggedTab, cx| {
                this.drag_split_direction = None;
                this.handle_tab_drop(dragged_tab, ix, cx)
            }))
            .on_drop(cx.listener(move |this, entry_id: &ProjectEntryId, cx| {
                this.drag_split_direction = None;
                this.handle_project_entry_drop(entry_id, cx)
            }))
            .on_drop(cx.listener(move |this, paths, cx| {
                this.drag_split_direction = None;
                this.handle_external_paths_drop(paths, cx)
            }))
            .when_some(item.tab_tooltip_text(cx), |tab, text| {
                tab.tooltip(move |cx| Tooltip::text(text.clone(), cx))
            })
            .start_slot::<Indicator>(indicator)
            .end_slot(
                IconButton::new("close tab", IconName::Close)
                    .shape(IconButtonShape::Square)
                    .icon_color(Color::Muted)
                    .size(ButtonSize::None)
                    .icon_size(IconSize::XSmall)
                    .on_click(cx.listener(move |pane, _, cx| {
                        pane.close_item_by_id(item_id, SaveIntent::Close, cx)
                            .detach_and_log_err(cx);
                    })),
            )
            .child(label);

        let single_entry_to_resolve = {
            let item_entries = self.items[ix].project_entry_ids(cx);
            if item_entries.len() == 1 {
                Some(item_entries[0])
            } else {
                None
            }
        };

        let pane = cx.view().downgrade();
        right_click_menu(ix).trigger(tab).menu(move |cx| {
            let pane = pane.clone();
            ContextMenu::build(cx, move |mut menu, cx| {
                if let Some(pane) = pane.upgrade() {
                    menu = menu
                        .entry(
                            "Close",
                            Some(Box::new(CloseActiveItem { save_intent: None })),
                            cx.handler_for(&pane, move |pane, cx| {
                                pane.close_item_by_id(item_id, SaveIntent::Close, cx)
                                    .detach_and_log_err(cx);
                            }),
                        )
                        .entry(
                            "Close Others",
                            Some(Box::new(CloseInactiveItems { save_intent: None })),
                            cx.handler_for(&pane, move |pane, cx| {
                                pane.close_items(cx, SaveIntent::Close, |id| id != item_id)
                                    .detach_and_log_err(cx);
                            }),
                        )
                        .separator()
                        .entry(
                            "Close Left",
                            Some(Box::new(CloseItemsToTheLeft)),
                            cx.handler_for(&pane, move |pane, cx| {
                                pane.close_items_to_the_left_by_id(item_id, cx)
                                    .detach_and_log_err(cx);
                            }),
                        )
                        .entry(
                            "Close Right",
                            Some(Box::new(CloseItemsToTheRight)),
                            cx.handler_for(&pane, move |pane, cx| {
                                pane.close_items_to_the_right_by_id(item_id, cx)
                                    .detach_and_log_err(cx);
                            }),
                        )
                        .separator()
                        .entry(
                            "Close Clean",
                            Some(Box::new(CloseCleanItems)),
                            cx.handler_for(&pane, move |pane, cx| {
                                if let Some(task) = pane.close_clean_items(&CloseCleanItems, cx) {
                                    task.detach_and_log_err(cx)
                                }
                            }),
                        )
                        .entry(
                            "Close All",
                            Some(Box::new(CloseAllItems { save_intent: None })),
                            cx.handler_for(&pane, |pane, cx| {
                                if let Some(task) =
                                    pane.close_all_items(&CloseAllItems { save_intent: None }, cx)
                                {
                                    task.detach_and_log_err(cx)
                                }
                            }),
                        );

                    if let Some(entry) = single_entry_to_resolve {
                        let parent_abs_path = pane
                            .update(cx, |pane, cx| {
                                pane.workspace.update(cx, |workspace, cx| {
                                    let project = workspace.project().read(cx);
                                    project.worktree_for_entry(entry, cx).and_then(|worktree| {
                                        let worktree = worktree.read(cx);
                                        let entry = worktree.entry_for_id(entry)?;
                                        let abs_path = worktree.absolutize(&entry.path).ok()?;
                                        let parent = if entry.is_symlink {
                                            abs_path.canonicalize().ok()?
                                        } else {
                                            abs_path
                                        }
                                        .parent()?
                                        .to_path_buf();
                                        Some(parent)
                                    })
                                })
                            })
                            .ok()
                            .flatten();

                        let entry_id = entry.to_proto();
                        menu = menu
                            .separator()
                            .entry(
                                "Reveal In Project Panel",
                                Some(Box::new(RevealInProjectPanel {
                                    entry_id: Some(entry_id),
                                })),
                                cx.handler_for(&pane, move |pane, cx| {
                                    pane.project.update(cx, |_, cx| {
                                        cx.emit(project::Event::RevealInProjectPanel(
                                            ProjectEntryId::from_proto(entry_id),
                                        ))
                                    });
                                }),
                            )
                            .when_some(parent_abs_path, |menu, abs_path| {
                                menu.entry(
                                    "Open in Terminal",
                                    Some(Box::new(OpenInTerminal)),
                                    cx.handler_for(&pane, move |_, cx| {
                                        cx.dispatch_action(
                                            OpenTerminal {
                                                working_directory: abs_path.clone(),
                                            }
                                            .boxed_clone(),
                                        );
                                    }),
                                )
                            });
                    }
                }

                menu
            })
        })
    }

    fn render_tab_bar(&mut self, cx: &mut ViewContext<'_, Pane>) -> impl IntoElement {
        let navigate_backward = IconButton::new("navigate_backward", IconName::ArrowLeft)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .on_click({
                let view = cx.view().clone();
                move |_, cx| view.update(cx, Self::navigate_backward)
            })
            .disabled(!self.can_navigate_backward())
            .tooltip(|cx| Tooltip::for_action("Go Back", &GoBack, cx));

        let navigate_forward = IconButton::new("navigate_forward", IconName::ArrowRight)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .on_click({
                let view = cx.view().clone();
                move |_, cx| view.update(cx, Self::navigate_forward)
            })
            .disabled(!self.can_navigate_forward())
            .tooltip(|cx| Tooltip::for_action("Go Forward", &GoForward, cx));

        TabBar::new("tab_bar")
            .track_scroll(self.tab_bar_scroll_handle.clone())
            .when(
                self.display_nav_history_buttons.unwrap_or_default(),
                |tab_bar| tab_bar.start_children(vec![navigate_backward, navigate_forward]),
            )
            .when(self.has_focus(cx), |tab_bar| {
                tab_bar.end_child({
                    let render_tab_buttons = self.render_tab_bar_buttons.clone();
                    render_tab_buttons(self, cx)
                })
            })
            .children(
                self.items
                    .iter()
                    .enumerate()
                    .zip(tab_details(&self.items, cx))
                    .map(|((ix, item), detail)| self.render_tab(ix, item, detail, cx)),
            )
            .child(
                div()
                    .id("tab_bar_drop_target")
                    .min_w_6()
                    // HACK: This empty child is currently necessary to force the drop target to appear
                    // despite us setting a min width above.
                    .child("")
                    .h_full()
                    .flex_grow()
                    .drag_over::<DraggedTab>(|bar, _, cx| {
                        bar.bg(cx.theme().colors().drop_target_background)
                    })
                    .drag_over::<ProjectEntryId>(|bar, _, cx| {
                        bar.bg(cx.theme().colors().drop_target_background)
                    })
                    .on_drop(cx.listener(move |this, dragged_tab: &DraggedTab, cx| {
                        this.drag_split_direction = None;
                        this.handle_tab_drop(dragged_tab, this.items.len(), cx)
                    }))
                    .on_drop(cx.listener(move |this, entry_id: &ProjectEntryId, cx| {
                        this.drag_split_direction = None;
                        this.handle_project_entry_drop(entry_id, cx)
                    }))
                    .on_drop(cx.listener(move |this, paths, cx| {
                        this.drag_split_direction = None;
                        this.handle_external_paths_drop(paths, cx)
                    }))
                    .on_click(cx.listener(move |this, event: &ClickEvent, cx| {
                        if event.up.click_count == 2 {
                            cx.dispatch_action(this.double_click_dispatch_action.boxed_clone())
                        }
                    })),
            )
    }

    pub fn render_menu_overlay(menu: &View<ContextMenu>) -> Div {
        div().absolute().bottom_0().right_0().size_0().child(
            deferred(
                anchored()
                    .anchor(AnchorCorner::TopRight)
                    .child(menu.clone()),
            )
            .with_priority(1),
        )
    }

    pub fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed
    }

    fn handle_drag_move<T>(&mut self, event: &DragMoveEvent<T>, cx: &mut ViewContext<Self>) {
        if !self.can_split {
            return;
        }

        let rect = event.bounds.size;

        let size = event.bounds.size.width.min(event.bounds.size.height)
            * WorkspaceSettings::get_global(cx).drop_target_size;

        let relative_cursor = Point::new(
            event.event.position.x - event.bounds.left(),
            event.event.position.y - event.bounds.top(),
        );

        let direction = if relative_cursor.x < size
            || relative_cursor.x > rect.width - size
            || relative_cursor.y < size
            || relative_cursor.y > rect.height - size
        {
            [
                SplitDirection::Up,
                SplitDirection::Right,
                SplitDirection::Down,
                SplitDirection::Left,
            ]
            .iter()
            .min_by_key(|side| match side {
                SplitDirection::Up => relative_cursor.y,
                SplitDirection::Right => rect.width - relative_cursor.x,
                SplitDirection::Down => rect.height - relative_cursor.y,
                SplitDirection::Left => relative_cursor.x,
            })
            .cloned()
        } else {
            None
        };

        if direction != self.drag_split_direction {
            self.drag_split_direction = direction;
        }
    }

    fn handle_tab_drop(
        &mut self,
        dragged_tab: &DraggedTab,
        ix: usize,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, dragged_tab, cx) {
                return;
            }
        }
        let mut to_pane = cx.view().clone();
        let split_direction = self.drag_split_direction;
        let item_id = dragged_tab.item.item_id();
        if let Some(preview_item_id) = self.preview_item_id {
            if item_id == preview_item_id {
                self.set_preview_item_id(None, cx);
            }
        }

        let from_pane = dragged_tab.pane.clone();
        self.workspace
            .update(cx, |_, cx| {
                cx.defer(move |workspace, cx| {
                    if let Some(split_direction) = split_direction {
                        to_pane = workspace.split_pane(to_pane, split_direction, cx);
                    }
                    workspace.move_item(from_pane, to_pane, item_id, ix, cx);
                });
            })
            .log_err();
    }

    fn handle_project_entry_drop(
        &mut self,
        project_entry_id: &ProjectEntryId,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, project_entry_id, cx) {
                return;
            }
        }
        let mut to_pane = cx.view().clone();
        let split_direction = self.drag_split_direction;
        let project_entry_id = *project_entry_id;
        self.workspace
            .update(cx, |_, cx| {
                cx.defer(move |workspace, cx| {
                    if let Some(path) = workspace
                        .project()
                        .read(cx)
                        .path_for_entry(project_entry_id, cx)
                    {
                        if let Some(split_direction) = split_direction {
                            to_pane = workspace.split_pane(to_pane, split_direction, cx);
                        }
                        workspace
                            .open_path(path, Some(to_pane.downgrade()), true, cx)
                            .detach_and_log_err(cx);
                    }
                });
            })
            .log_err();
    }

    fn handle_external_paths_drop(
        &mut self,
        paths: &ExternalPaths,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, paths, cx) {
                return;
            }
        }
        let mut to_pane = cx.view().clone();
        let mut split_direction = self.drag_split_direction;
        let paths = paths.paths().to_vec();
        let is_remote = self
            .workspace
            .update(cx, |workspace, cx| {
                if workspace.project().read(cx).is_remote() {
                    workspace.show_error(
                        &anyhow::anyhow!("Cannot drop files on a remote project"),
                        cx,
                    );
                    true
                } else {
                    false
                }
            })
            .unwrap_or(true);
        if is_remote {
            return;
        }

        self.workspace
            .update(cx, |workspace, cx| {
                let fs = Arc::clone(workspace.project().read(cx).fs());
                cx.spawn(|workspace, mut cx| async move {
                    let mut is_file_checks = FuturesUnordered::new();
                    for path in &paths {
                        is_file_checks.push(fs.is_file(path))
                    }
                    let mut has_files_to_open = false;
                    while let Some(is_file) = is_file_checks.next().await {
                        if is_file {
                            has_files_to_open = true;
                            break;
                        }
                    }
                    drop(is_file_checks);
                    if !has_files_to_open {
                        split_direction = None;
                    }

                    if let Some(open_task) = workspace
                        .update(&mut cx, |workspace, cx| {
                            if let Some(split_direction) = split_direction {
                                to_pane = workspace.split_pane(to_pane, split_direction, cx);
                            }
                            workspace.open_paths(
                                paths,
                                OpenVisible::OnlyDirectories,
                                Some(to_pane.downgrade()),
                                cx,
                            )
                        })
                        .ok()
                    {
                        let _opened_items: Vec<_> = open_task.await;
                    }
                })
                .detach();
            })
            .log_err();
    }

    pub fn display_nav_history_buttons(&mut self, display: Option<bool>) {
        self.display_nav_history_buttons = display;
    }
}

impl FocusableView for Pane {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Pane {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("Pane");
        if self.active_item().is_none() {
            key_context.add("EmptyPane");
        }

        let should_display_tab_bar = self.should_display_tab_bar.clone();
        let display_tab_bar = should_display_tab_bar(cx);

        v_flex()
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .size_full()
            .flex_none()
            .overflow_hidden()
            .on_action(cx.listener(|pane, _: &AlternateFile, cx| {
                pane.alternate_file(cx);
            }))
            .on_action(cx.listener(|pane, _: &SplitLeft, cx| pane.split(SplitDirection::Left, cx)))
            .on_action(cx.listener(|pane, _: &SplitUp, cx| pane.split(SplitDirection::Up, cx)))
            .on_action(
                cx.listener(|pane, _: &SplitRight, cx| pane.split(SplitDirection::Right, cx)),
            )
            .on_action(cx.listener(|pane, _: &SplitDown, cx| pane.split(SplitDirection::Down, cx)))
            .on_action(cx.listener(|pane, _: &GoBack, cx| pane.navigate_backward(cx)))
            .on_action(cx.listener(|pane, _: &GoForward, cx| pane.navigate_forward(cx)))
            .on_action(cx.listener(Pane::toggle_zoom))
            .on_action(cx.listener(|pane: &mut Pane, action: &ActivateItem, cx| {
                pane.activate_item(action.0, true, true, cx);
            }))
            .on_action(cx.listener(|pane: &mut Pane, _: &ActivateLastItem, cx| {
                pane.activate_item(pane.items.len() - 1, true, true, cx);
            }))
            .on_action(cx.listener(|pane: &mut Pane, _: &ActivatePrevItem, cx| {
                pane.activate_prev_item(true, cx);
            }))
            .on_action(cx.listener(|pane: &mut Pane, _: &ActivateNextItem, cx| {
                pane.activate_next_item(true, cx);
            }))
            .when(PreviewTabsSettings::get_global(cx).enabled, |this| {
                this.on_action(cx.listener(|pane: &mut Pane, _: &TogglePreviewTab, cx| {
                    if let Some(active_item_id) = pane.active_item().map(|i| i.item_id()) {
                        if pane.is_active_preview_item(active_item_id) {
                            pane.set_preview_item_id(None, cx);
                        } else {
                            pane.set_preview_item_id(Some(active_item_id), cx);
                        }
                    }
                }))
            })
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseActiveItem, cx| {
                    if let Some(task) = pane.close_active_item(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseInactiveItems, cx| {
                    if let Some(task) = pane.close_inactive_items(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseCleanItems, cx| {
                    if let Some(task) = pane.close_clean_items(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseItemsToTheLeft, cx| {
                    if let Some(task) = pane.close_items_to_the_left(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseItemsToTheRight, cx| {
                    if let Some(task) = pane.close_items_to_the_right(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(cx.listener(|pane: &mut Self, action: &CloseAllItems, cx| {
                if let Some(task) = pane.close_all_items(action, cx) {
                    task.detach_and_log_err(cx)
                }
            }))
            .on_action(
                cx.listener(|pane: &mut Self, action: &CloseActiveItem, cx| {
                    if let Some(task) = pane.close_active_item(action, cx) {
                        task.detach_and_log_err(cx)
                    }
                }),
            )
            .on_action(
                cx.listener(|pane: &mut Self, action: &RevealInProjectPanel, cx| {
                    let entry_id = action
                        .entry_id
                        .map(ProjectEntryId::from_proto)
                        .or_else(|| pane.active_item()?.project_entry_ids(cx).first().copied());
                    if let Some(entry_id) = entry_id {
                        pane.project.update(cx, |_, cx| {
                            cx.emit(project::Event::RevealInProjectPanel(entry_id))
                        });
                    }
                }),
            )
            .when(self.active_item().is_some() && display_tab_bar, |pane| {
                pane.child(self.render_tab_bar(cx))
            })
            .child({
                let has_worktrees = self.project.read(cx).worktrees().next().is_some();
                // main content
                div()
                    .flex_1()
                    .relative()
                    .group("")
                    .on_drag_move::<DraggedTab>(cx.listener(Self::handle_drag_move))
                    .on_drag_move::<ProjectEntryId>(cx.listener(Self::handle_drag_move))
                    .on_drag_move::<ExternalPaths>(cx.listener(Self::handle_drag_move))
                    .map(|div| {
                        if let Some(item) = self.active_item() {
                            div.v_flex()
                                .child(self.toolbar.clone())
                                .child(item.to_any())
                        } else {
                            let placeholder = div.h_flex().size_full().justify_center();
                            if has_worktrees {
                                placeholder
                            } else {
                                placeholder.child(
                                    Label::new("Open a file or project to get started.")
                                        .color(Color::Muted),
                                )
                            }
                        }
                    })
                    .child(
                        // drag target
                        div()
                            .invisible()
                            .absolute()
                            .bg(cx.theme().colors().drop_target_background)
                            .group_drag_over::<DraggedTab>("", |style| style.visible())
                            .group_drag_over::<ProjectEntryId>("", |style| style.visible())
                            .group_drag_over::<ExternalPaths>("", |style| style.visible())
                            .when_some(self.can_drop_predicate.clone(), |this, p| {
                                this.can_drop(move |a, cx| p(a, cx))
                            })
                            .on_drop(cx.listener(move |this, dragged_tab, cx| {
                                this.handle_tab_drop(dragged_tab, this.active_item_index(), cx)
                            }))
                            .on_drop(cx.listener(move |this, entry_id, cx| {
                                this.handle_project_entry_drop(entry_id, cx)
                            }))
                            .on_drop(cx.listener(move |this, paths, cx| {
                                this.handle_external_paths_drop(paths, cx)
                            }))
                            .map(|div| {
                                let size = DefiniteLength::Fraction(0.5);
                                match self.drag_split_direction {
                                    None => div.top_0().right_0().bottom_0().left_0(),
                                    Some(SplitDirection::Up) => {
                                        div.top_0().left_0().right_0().h(size)
                                    }
                                    Some(SplitDirection::Down) => {
                                        div.left_0().bottom_0().right_0().h(size)
                                    }
                                    Some(SplitDirection::Left) => {
                                        div.top_0().left_0().bottom_0().w(size)
                                    }
                                    Some(SplitDirection::Right) => {
                                        div.top_0().bottom_0().right_0().w(size)
                                    }
                                }
                            }),
                    )
            })
            .on_mouse_down(
                MouseButton::Navigate(NavigationDirection::Back),
                cx.listener(|pane, _, cx| {
                    if let Some(workspace) = pane.workspace.upgrade() {
                        let pane = cx.view().downgrade();
                        cx.window_context().defer(move |cx| {
                            workspace.update(cx, |workspace, cx| {
                                workspace.go_back(pane, cx).detach_and_log_err(cx)
                            })
                        })
                    }
                }),
            )
            .on_mouse_down(
                MouseButton::Navigate(NavigationDirection::Forward),
                cx.listener(|pane, _, cx| {
                    if let Some(workspace) = pane.workspace.upgrade() {
                        let pane = cx.view().downgrade();
                        cx.window_context().defer(move |cx| {
                            workspace.update(cx, |workspace, cx| {
                                workspace.go_forward(pane, cx).detach_and_log_err(cx)
                            })
                        })
                    }
                }),
            )
    }
}

impl ItemNavHistory {
    pub fn push<D: 'static + Send + Any>(&mut self, data: Option<D>, cx: &mut WindowContext) {
        self.history
            .push(data, self.item.clone(), self.is_preview, cx);
    }

    pub fn pop_backward(&mut self, cx: &mut WindowContext) -> Option<NavigationEntry> {
        self.history.pop(NavigationMode::GoingBack, cx)
    }

    pub fn pop_forward(&mut self, cx: &mut WindowContext) -> Option<NavigationEntry> {
        self.history.pop(NavigationMode::GoingForward, cx)
    }
}

impl NavHistory {
    pub fn for_each_entry(
        &self,
        cx: &AppContext,
        mut f: impl FnMut(&NavigationEntry, (ProjectPath, Option<PathBuf>)),
    ) {
        let borrowed_history = self.0.lock();
        borrowed_history
            .forward_stack
            .iter()
            .chain(borrowed_history.backward_stack.iter())
            .chain(borrowed_history.closed_stack.iter())
            .for_each(|entry| {
                if let Some(project_and_abs_path) =
                    borrowed_history.paths_by_item.get(&entry.item.id())
                {
                    f(entry, project_and_abs_path.clone());
                } else if let Some(item) = entry.item.upgrade() {
                    if let Some(path) = item.project_path(cx) {
                        f(entry, (path, None));
                    }
                }
            })
    }

    pub fn set_mode(&mut self, mode: NavigationMode) {
        self.0.lock().mode = mode;
    }

    pub fn mode(&self) -> NavigationMode {
        self.0.lock().mode
    }

    pub fn disable(&mut self) {
        self.0.lock().mode = NavigationMode::Disabled;
    }

    pub fn enable(&mut self) {
        self.0.lock().mode = NavigationMode::Normal;
    }

    pub fn pop(&mut self, mode: NavigationMode, cx: &mut WindowContext) -> Option<NavigationEntry> {
        let mut state = self.0.lock();
        let entry = match mode {
            NavigationMode::Normal | NavigationMode::Disabled | NavigationMode::ClosingItem => {
                return None
            }
            NavigationMode::GoingBack => &mut state.backward_stack,
            NavigationMode::GoingForward => &mut state.forward_stack,
            NavigationMode::ReopeningClosedItem => &mut state.closed_stack,
        }
        .pop_back();
        if entry.is_some() {
            state.did_update(cx);
        }
        entry
    }

    pub fn push<D: 'static + Send + Any>(
        &mut self,
        data: Option<D>,
        item: Arc<dyn WeakItemHandle>,
        is_preview: bool,
        cx: &mut WindowContext,
    ) {
        let state = &mut *self.0.lock();
        match state.mode {
            NavigationMode::Disabled => {}
            NavigationMode::Normal | NavigationMode::ReopeningClosedItem => {
                if state.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.backward_stack.pop_front();
                }
                state.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any + Send>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                    is_preview,
                });
                state.forward_stack.clear();
            }
            NavigationMode::GoingBack => {
                if state.forward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.forward_stack.pop_front();
                }
                state.forward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any + Send>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                    is_preview,
                });
            }
            NavigationMode::GoingForward => {
                if state.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.backward_stack.pop_front();
                }
                state.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any + Send>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                    is_preview,
                });
            }
            NavigationMode::ClosingItem => {
                if state.closed_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.closed_stack.pop_front();
                }
                state.closed_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any + Send>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                    is_preview,
                });
            }
        }
        state.did_update(cx);
    }

    pub fn remove_item(&mut self, item_id: EntityId) {
        let mut state = self.0.lock();
        state.paths_by_item.remove(&item_id);
        state
            .backward_stack
            .retain(|entry| entry.item.id() != item_id);
        state
            .forward_stack
            .retain(|entry| entry.item.id() != item_id);
        state
            .closed_stack
            .retain(|entry| entry.item.id() != item_id);
    }

    pub fn path_for_item(&self, item_id: EntityId) -> Option<(ProjectPath, Option<PathBuf>)> {
        self.0.lock().paths_by_item.get(&item_id).cloned()
    }
}

impl NavHistoryState {
    pub fn did_update(&self, cx: &mut WindowContext) {
        if let Some(pane) = self.pane.upgrade() {
            cx.defer(move |cx| {
                pane.update(cx, |pane, cx| pane.history_updated(cx));
            });
        }
    }
}

fn dirty_message_for(buffer_path: Option<ProjectPath>) -> String {
    let path = buffer_path
        .as_ref()
        .and_then(|p| {
            p.path
                .to_str()
                .and_then(|s| if s == "" { None } else { Some(s) })
        })
        .unwrap_or("This buffer");
    let path = truncate_and_remove_front(path, 80);
    format!("{path} contains unsaved edits. Do you want to save it?")
}

pub fn tab_details(items: &Vec<Box<dyn ItemHandle>>, cx: &AppContext) -> Vec<usize> {
    let mut tab_details = items.iter().map(|_| 0).collect::<Vec<_>>();
    let mut tab_descriptions = HashMap::default();
    let mut done = false;
    while !done {
        done = true;

        // Store item indices by their tab description.
        for (ix, (item, detail)) in items.iter().zip(&tab_details).enumerate() {
            if let Some(description) = item.tab_description(*detail, cx) {
                if *detail == 0
                    || Some(&description) != item.tab_description(detail - 1, cx).as_ref()
                {
                    tab_descriptions
                        .entry(description)
                        .or_insert(Vec::new())
                        .push(ix);
                }
            }
        }

        // If two or more items have the same tab description, increase their level
        // of detail and try again.
        for (_, item_ixs) in tab_descriptions.drain() {
            if item_ixs.len() > 1 {
                done = false;
                for ix in item_ixs {
                    tab_details[ix] += 1;
                }
            }
        }
    }

    tab_details
}

pub fn render_item_indicator(item: Box<dyn ItemHandle>, cx: &WindowContext) -> Option<Indicator> {
    maybe!({
        let indicator_color = match (item.has_conflict(cx), item.is_dirty(cx)) {
            (true, _) => Color::Warning,
            (_, true) => Color::Accent,
            (false, false) => return None,
        };

        Some(Indicator::dot().color(indicator_color))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::test::{TestItem, TestProjectItem};
    use gpui::{TestAppContext, VisualTestContext};
    use project::FakeFs;
    use settings::SettingsStore;
    use theme::LoadThemes;

    #[gpui::test]
    async fn test_remove_active_empty(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        pane.update(cx, |pane, cx| {
            assert!(pane
                .close_active_item(&CloseActiveItem { save_intent: None }, cx)
                .is_none())
        });
    }

    #[gpui::test]
    async fn test_add_item_with_new_item(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   a. Add before the active item
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| TestItem::new(cx).with_label("D"))),
                false,
                false,
                Some(0),
                cx,
            );
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   b. Add after the active item
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| TestItem::new(cx).with_label("D"))),
                false,
                false,
                Some(2),
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   c. Add at the end of the item list (including off the length)
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| TestItem::new(cx).with_label("D"))),
                false,
                false,
                Some(5),
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        // 2. Add without a destination index
        //   a. Add with active item at the start of the item list
        set_labeled_items(&pane, ["A*", "B", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| TestItem::new(cx).with_label("D"))),
                false,
                false,
                None,
                cx,
            );
        });
        set_labeled_items(&pane, ["A", "D*", "B", "C"], cx);

        //   b. Add with active item at the end of the item list
        set_labeled_items(&pane, ["A", "B", "C*"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| TestItem::new(cx).with_label("D"))),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);
    }

    #[gpui::test]
    async fn test_add_item_with_existing_item(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   1a. Add before the active item
        let [_, _, _, d] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(d, false, false, Some(0), cx);
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   1b. Add after the active item
        let [_, _, _, d] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(d, false, false, Some(2), cx);
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   1c. Add at the end of the item list (including off the length)
        let [a, _, _, _] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(a, false, false, Some(5), cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   1d. Add same item to active index
        let [_, b, _] = set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(b, false, false, Some(1), cx);
        });
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        //   1e. Add item to index after same item in last position
        let [_, _, c] = set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(c, false, false, Some(2), cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        // 2. Add without a destination index
        //   2a. Add with active item at the start of the item list
        let [_, _, _, d] = set_labeled_items(&pane, ["A*", "B", "C", "D"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(d, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A", "D*", "B", "C"], cx);

        //   2b. Add with active item at the end of the item list
        let [a, _, _, _] = set_labeled_items(&pane, ["A", "B", "C", "D*"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(a, false, false, None, cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   2c. Add active item to active item at end of list
        let [_, _, c] = set_labeled_items(&pane, ["A", "B", "C*"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(c, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        //   2d. Add active item to active item at start of list
        let [a, _, _] = set_labeled_items(&pane, ["A*", "B", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(a, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A*", "B", "C"], cx);
    }

    #[gpui::test]
    async fn test_add_item_with_same_project_entries(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        // singleton view
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_singleton(true)
                        .with_label("buffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
                })),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with the same project entry
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_singleton(true)
                        .with_label("buffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with different project entry
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_singleton(true)
                        .with_label("buffer 2")
                        .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)])
                })),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2*"], cx);

        // new multibuffer view with the same project entry
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_singleton(false)
                        .with_label("multibuffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2", "multibuffer 1*"], cx);

        // another multibuffer view with the same project entry
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.new_view(|cx| {
                    TestItem::new(cx)
                        .with_singleton(false)
                        .with_label("multibuffer 1b")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(
            &pane,
            ["buffer 1", "buffer 2", "multibuffer 1", "multibuffer 1b*"],
            cx,
        );
    }

    #[gpui::test]
    async fn test_remove_item_ordering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        add_labeled_item(&pane, "D", false, cx);
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(1, false, false, cx));
        add_labeled_item(&pane, "1", false, cx);
        assert_item_labels(&pane, ["A", "B", "1*", "C", "D"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C", "D"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(3, false, false, cx));
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "C*"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A*"], cx);
    }

    #[gpui::test]
    async fn test_close_inactive_items(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_inactive_items(&CloseInactiveItems { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["C*"], cx);
    }

    #[gpui::test]
    async fn test_close_clean_items(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", true, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", true, cx);
        add_labeled_item(&pane, "D", false, cx);
        add_labeled_item(&pane, "E", false, cx);
        assert_item_labels(&pane, ["A^", "B", "C^", "D", "E*"], cx);

        pane.update(cx, |pane, cx| pane.close_clean_items(&CloseCleanItems, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, ["A^", "C*^"], cx);
    }

    #[gpui::test]
    async fn test_close_items_to_the_left(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_items_to_the_left(&CloseItemsToTheLeft, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["C*", "D", "E"], cx);
    }

    #[gpui::test]
    async fn test_close_items_to_the_right(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_items_to_the_right(&CloseItemsToTheRight, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B", "C*"], cx);
    }

    #[gpui::test]
    async fn test_close_all_items(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_all_items(&CloseAllItems { save_intent: None }, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, [], cx);

        add_labeled_item(&pane, "A", true, cx);
        add_labeled_item(&pane, "B", true, cx);
        add_labeled_item(&pane, "C", true, cx);
        assert_item_labels(&pane, ["A^", "B^", "C*^"], cx);

        let save = pane
            .update(cx, |pane, cx| {
                pane.close_all_items(&CloseAllItems { save_intent: None }, cx)
            })
            .unwrap();

        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2);
        save.await.unwrap();
        assert_item_labels(&pane, [], cx);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(LoadThemes::JustBase, cx);
            crate::init_settings(cx);
            Project::init_settings(cx);
        });
    }

    fn add_labeled_item(
        pane: &View<Pane>,
        label: &str,
        is_dirty: bool,
        cx: &mut VisualTestContext,
    ) -> Box<View<TestItem>> {
        pane.update(cx, |pane, cx| {
            let labeled_item = Box::new(
                cx.new_view(|cx| TestItem::new(cx).with_label(label).with_dirty(is_dirty)),
            );
            pane.add_item(labeled_item.clone(), false, false, None, cx);
            labeled_item
        })
    }

    fn set_labeled_items<const COUNT: usize>(
        pane: &View<Pane>,
        labels: [&str; COUNT],
        cx: &mut VisualTestContext,
    ) -> [Box<View<TestItem>>; COUNT] {
        pane.update(cx, |pane, cx| {
            pane.items.clear();
            let mut active_item_index = 0;

            let mut index = 0;
            let items = labels.map(|mut label| {
                if label.ends_with('*') {
                    label = label.trim_end_matches('*');
                    active_item_index = index;
                }

                let labeled_item = Box::new(cx.new_view(|cx| TestItem::new(cx).with_label(label)));
                pane.add_item(labeled_item.clone(), false, false, None, cx);
                index += 1;
                labeled_item
            });

            pane.activate_item(active_item_index, false, false, cx);

            items
        })
    }

    // Assert the item label, with the active item label suffixed with a '*'
    fn assert_item_labels<const COUNT: usize>(
        pane: &View<Pane>,
        expected_states: [&str; COUNT],
        cx: &mut VisualTestContext,
    ) {
        pane.update(cx, |pane, cx| {
            let actual_states = pane
                .items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let mut state = item
                        .to_any()
                        .downcast::<TestItem>()
                        .unwrap()
                        .read(cx)
                        .label
                        .clone();
                    if ix == pane.active_item_index {
                        state.push('*');
                    }
                    if item.is_dirty(cx) {
                        state.push('^');
                    }
                    state
                })
                .collect::<Vec<_>>();

            assert_eq!(
                actual_states, expected_states,
                "pane items do not match expectation"
            );
        })
    }
}

impl Render for DraggedTab {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();
        let label = self.item.tab_content(
            TabContentParams {
                detail: Some(self.detail),
                selected: false,
                preview: false,
            },
            cx,
        );
        Tab::new("")
            .selected(self.is_active)
            .child(label)
            .render(cx)
            .font(ui_font)
    }
}
