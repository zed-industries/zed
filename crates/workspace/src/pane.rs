use crate::{
    item::{
        ActivateOnClose, ClosePosition, Item, ItemHandle, ItemSettings, PreviewTabsSettings,
        ShowDiagnostics, TabContentParams, WeakItemHandle,
    },
    move_item,
    notifications::NotifyResultExt,
    toolbar::Toolbar,
    workspace_settings::{AutosaveSetting, TabBarSettings, WorkspaceSettings},
    CloseWindow, CopyPath, CopyRelativePath, NewFile, NewTerminal, OpenInTerminal, OpenTerminal,
    OpenVisible, SplitDirection, ToggleFileFinder, ToggleProjectSymbols, ToggleZoom, Workspace,
};
use anyhow::Result;
use collections::{BTreeSet, HashMap, HashSet, VecDeque};
use futures::{stream::FuturesUnordered, StreamExt};
use gpui::{
    actions, anchored, deferred, impl_actions, prelude::*, Action, AnchorCorner, AnyElement,
    AnyWindowHandle, AppContext, AsyncAppContext, ClickEvent, ClipboardItem, Div, DragMoveEvent,
    EntityId, EventEmitter, ExternalPaths, FocusHandle, FocusOutEvent, FocusableView, KeyContext,
    Model, MouseButton, MouseDownEvent, NavigationDirection, Pixels, Point, PromptLevel, Render,
    ScrollHandle, Subscription, Task, WeakFocusHandle, WeakModel,
};
use itertools::Itertools;
use language::DiagnosticSeverity;
use parking_lot::Mutex;
use project::{Project, ProjectEntryId, ProjectPath, WorktreeId};
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
    prelude::*, right_click_menu, ButtonSize, Color, DecoratedIcon, IconButton, IconButtonShape,
    IconDecoration, IconDecorationKind, IconName, IconSize, Indicator, Label, PopoverMenu,
    PopoverMenuHandle, Tab, TabBar, TabPosition, Tooltip,
};
use ui::{v_flex, ContextMenu};
use util::{debug_panic, maybe, truncate_and_remove_front, ResultExt};

/// A selected entry in e.g. project panel.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectedEntry {
    pub worktree_id: WorktreeId,
    pub entry_id: ProjectEntryId,
}

/// A group of selected entries from project panel.
#[derive(Debug)]
pub struct DraggedSelection {
    pub active_selection: SelectedEntry,
    pub marked_selections: Arc<BTreeSet<SelectedEntry>>,
}

impl DraggedSelection {
    pub fn items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SelectedEntry> + 'a> {
        if self.marked_selections.contains(&self.active_selection) {
            Box::new(self.marked_selections.iter())
        } else {
            Box::new(std::iter::once(&self.active_selection))
        }
    }
}

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
    #[serde(default)]
    pub close_pinned: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseAllItems {
    pub save_intent: Option<SaveIntent>,
    #[serde(default)]
    pub close_pinned: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseCleanItems {
    #[serde(default)]
    pub close_pinned: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseItemsToTheRight {
    #[serde(default)]
    pub close_pinned: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseItemsToTheLeft {
    #[serde(default)]
    pub close_pinned: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RevealInProjectPanel {
    pub entry_id: Option<u64>,
}

#[derive(Default, PartialEq, Clone, Deserialize)]
pub struct DeploySearch {
    #[serde(default)]
    pub replace_enabled: bool,
}

impl_actions!(
    pane,
    [
        CloseAllItems,
        CloseActiveItem,
        CloseCleanItems,
        CloseItemsToTheLeft,
        CloseItemsToTheRight,
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
        GoBack,
        GoForward,
        JoinIntoNext,
        JoinAll,
        ReopenClosedItem,
        SplitLeft,
        SplitUp,
        SplitRight,
        SplitDown,
        SplitHorizontal,
        SplitVertical,
        SwapItemLeft,
        SwapItemRight,
        TogglePreviewTab,
        TogglePinTab,
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
    AddItem {
        item: Box<dyn ItemHandle>,
    },
    ActivateItem {
        local: bool,
    },
    Remove {
        focus_on_pane: Option<Model<Pane>>,
    },
    RemoveItem {
        idx: usize,
    },
    RemovedItem {
        item_id: EntityId,
    },
    Split(SplitDirection),
    JoinAll,
    JoinIntoNext,
    ChangeItemTitle,
    Focus,
    ZoomIn,
    ZoomOut,
    UserSavedItem {
        item: Box<dyn WeakItemHandle>,
        save_intent: SaveIntent,
    },
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
            Event::Remove { .. } => f.write_str("Remove"),
            Event::RemoveItem { idx } => f.debug_struct("RemoveItem").field("idx", idx).finish(),
            Event::RemovedItem { item_id } => f
                .debug_struct("RemovedItem")
                .field("item_id", item_id)
                .finish(),
            Event::Split(direction) => f
                .debug_struct("Split")
                .field("direction", direction)
                .finish(),
            Event::JoinAll => f.write_str("JoinAll"),
            Event::JoinIntoNext => f.write_str("JoinIntoNext"),
            Event::ChangeItemTitle => f.write_str("ChangeItemTitle"),
            Event::Focus => f.write_str("Focus"),
            Event::ZoomIn => f.write_str("ZoomIn"),
            Event::ZoomOut => f.write_str("ZoomOut"),
            Event::UserSavedItem { item, save_intent } => f
                .debug_struct("UserSavedItem")
                .field("item", &item.id())
                .field("save_intent", save_intent)
                .finish(),
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
    toolbar: Model<Toolbar>,
    pub(crate) workspace: WeakModel<Workspace>,
    project: Model<Project>,
    drag_split_direction: Option<SplitDirection>,
    can_drop_predicate:
        Option<Arc<dyn Fn(&dyn Any, &mut gpui::Window, &mut gpui::AppContext) -> bool>>,
    custom_drop_handle: Option<
        Arc<dyn Fn(&mut Pane, &dyn Any, &Model<Pane>, &mut AppContext) -> ControlFlow<(), ()>>,
    >,
    can_split_predicate:
        Option<Arc<dyn Fn(&mut Self, &dyn Any, &Model<Self>, &mut AppContext) -> bool>>,
    should_display_tab_bar: Rc<dyn Fn(&Model<Pane>, &AppContext) -> bool>,
    render_tab_bar_buttons: Rc<
        dyn Fn(
            &mut Pane,
            &Model<Pane>,
            &mut Window,
            &mut AppContext,
        ) -> (Option<AnyElement>, Option<AnyElement>),
    >,
    _subscriptions: Vec<Subscription>,
    tab_bar_scroll_handle: ScrollHandle,
    /// Is None if navigation buttons are permanently turned off (and should not react to setting changes).
    /// Otherwise, when `display_nav_history_buttons` is Some, it determines whether nav buttons should be displayed.
    display_nav_history_buttons: Option<bool>,
    double_click_dispatch_action: Box<dyn Action>,
    save_modals_spawned: HashSet<EntityId>,
    pub new_item_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub split_item_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    pinned_tab_count: usize,
    diagnostics: HashMap<ProjectPath, DiagnosticSeverity>,
    zoom_out_on_close: bool,
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
    pane: WeakModel<Pane>,
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
    pub pane: Model<Pane>,
    pub item: Box<dyn ItemHandle>,
    pub ix: usize,
    pub detail: usize,
    pub is_active: bool,
}

impl EventEmitter<Event> for Pane {}

impl Pane {
    pub fn new(
        workspace: WeakModel<Workspace>,
        project: Model<Project>,
        next_timestamp: Arc<AtomicUsize>,
        can_drop_predicate: Option<
            Arc<dyn Fn(&dyn Any, &mut gpui::Window, &mut gpui::AppContext) -> bool + 'static>,
        >,
        double_click_dispatch_action: Box<dyn Action>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Self {
        let focus_handle = window.focus_handle();

        let subscriptions = vec![
            model.on_focus(&focus_handle, window, cx, Pane::focus_in),
            model.on_focus_in(&focus_handle, window, cx, Pane::focus_in),
            model.on_focus_out(&focus_handle, window, cx, Pane::focus_out),
            model.observe_global::<SettingsStore>(cx, Self::settings_changed),
            model.subscribe(&project, cx, Self::project_events),
        ];

        let handle = model.downgrade();
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
            toolbar: cx.new_model(|_, _| Toolbar::new()),
            tab_bar_scroll_handle: ScrollHandle::new(),
            drag_split_direction: None,
            workspace,
            project,
            can_drop_predicate,
            custom_drop_handle: None,
            can_split_predicate: None,
            should_display_tab_bar: Rc::new(|model, cx| TabBarSettings::get_global(cx).show),
            render_tab_bar_buttons: Rc::new(move |pane, model, window, cx| {
                if !pane.has_focus(window, cx) && !pane.context_menu_focused(window, cx) {
                    return (None, None);
                }
                // Ideally we would return a vec of elements here to pass directly to the [TabBar]'s
                // `end_slot`, but due to needing a view here that isn't possible.
                let right_children = h_flex()
                    // Instead we need to replicate the spacing from the [TabBar]'s `end_slot` here.
                    .gap(DynamicSpacing::Base04.rems(cx))
                    .child(
                        PopoverMenu::new("pane-tab-bar-popover-menu")
                            .trigger(
                                IconButton::new("plus", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .tooltip(|window, cx| Tooltip::text("New...", cx)),
                            )
                            .anchor(AnchorCorner::TopRight)
                            .with_handle(pane.new_item_context_menu_handle.clone())
                            .menu(move |window, cx| {
                                Some(ContextMenu::build(window, cx, |menu, model, window, cx| {
                                    menu.action("New File", NewFile.boxed_clone())
                                        .action(
                                            "Open File",
                                            ToggleFileFinder::default().boxed_clone(),
                                        )
                                        .separator()
                                        .action(
                                            "Search Project",
                                            DeploySearch {
                                                replace_enabled: false,
                                            }
                                            .boxed_clone(),
                                        )
                                        .action(
                                            "Search Symbols",
                                            ToggleProjectSymbols.boxed_clone(),
                                        )
                                        .separator()
                                        .action("New Terminal", NewTerminal.boxed_clone())
                                }))
                            }),
                    )
                    .child(
                        PopoverMenu::new("pane-tab-bar-split")
                            .trigger(
                                IconButton::new("split", IconName::Split)
                                    .icon_size(IconSize::Small)
                                    .tooltip(|window, cx| Tooltip::text("Split Pane", cx)),
                            )
                            .anchor(AnchorCorner::TopRight)
                            .with_handle(pane.split_item_context_menu_handle.clone())
                            .menu(move |window, cx| {
                                ContextMenu::build(window, cx, |menu, model, window, cx| {
                                    menu.action("Split Right", SplitRight.boxed_clone())
                                        .action("Split Left", SplitLeft.boxed_clone())
                                        .action("Split Up", SplitUp.boxed_clone())
                                        .action("Split Down", SplitDown.boxed_clone())
                                })
                                .into()
                            }),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        IconButton::new("toggle_zoom", IconName::Maximize)
                            .icon_size(IconSize::Small)
                            .selected(zoomed)
                            .selected_icon(IconName::Minimize)
                            .on_click(model.listener(|pane, _, model, window, cx| {
                                pane.toggle_zoom(&crate::ToggleZoom, model, window, cx);
                            }))
                            .tooltip(move |window, cx| {
                                Tooltip::for_action(
                                    if zoomed { "Zoom Out" } else { "Zoom In" },
                                    &ToggleZoom,
                                    window,
                                    cx,
                                )
                            })
                    })
                    .into_any_element()
                    .into();
                (None, right_children)
            }),
            display_nav_history_buttons: Some(
                TabBarSettings::get_global(cx).show_nav_history_buttons,
            ),
            _subscriptions: subscriptions,
            double_click_dispatch_action,
            save_modals_spawned: HashSet::default(),
            split_item_context_menu_handle: Default::default(),
            new_item_context_menu_handle: Default::default(),
            pinned_tab_count: 0,
            diagnostics: Default::default(),
            zoom_out_on_close: true,
        }
    }

    fn alternate_file(&mut self, model: &Model<Pane>, window: &mut Window, cx: &mut AppContext) {
        let (_, alternative) = &self.alternate_file_items;
        if let Some(alternative) = alternative {
            let existing = self
                .items()
                .find_position(|item| item.item_id() == alternative.id());
            if let Some((ix, _)) = existing {
                self.activate_item(ix, true, true, model, window, cx);
            } else if let Some(upgraded) = alternative.upgrade() {
                self.add_item(upgraded, true, true, None, model, window, cx);
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

    pub fn has_focus(&self, window: &Window, cx: &AppContext) -> bool {
        // We not only check whether our focus handle contains focus, but also
        // whether the active item might have focus, because we might have just activated an item
        // that hasn't rendered yet.
        // Before the next render, we might transfer focus
        // to the item, and `focus_handle.contains_focus` returns false because the `active_item`
        // is not hooked up to us in the dispatch tree.
        self.focus_handle.contains_focused(window)
            || self.active_item().map_or(false, |item| {
                item.item_focus_handle(cx).contains_focused(window)
            })
    }

    fn focus_in(&mut self, model: &Model<Self>, window: &mut Window, cx: &mut AppContext) {
        if !self.was_focused {
            self.was_focused = true;
            model.emit(Event::Focus, cx);
            model.notify(cx);
        }

        self.toolbar.update(cx, |toolbar, model, cx| {
            toolbar.focus_changed(true, model, cx);
        });

        if let Some(active_item) = self.active_item() {
            if self.focus_handle.is_focused(window) {
                // Pane was focused directly. We need to either focus a view inside the active item,
                // or focus the active item itself
                if let Some(weak_last_focus_handle) =
                    self.last_focus_handle_by_item.get(&active_item.item_id())
                {
                    if let Some(focus_handle) = weak_last_focus_handle.upgrade() {
                        focus_handle.focus(window);
                        return;
                    }
                }

                active_item.item_focus_handle(cx).focus(window);
            } else if let Some(focused) = window.focused() {
                if !self.context_menu_focused(window, cx) {
                    self.last_focus_handle_by_item
                        .insert(active_item.item_id(), focused.downgrade());
                }
            }
        }
    }

    pub fn context_menu_focused(&self, window: &Window, cx: &mut AppContext) -> bool {
        self.new_item_context_menu_handle.is_focused(window, cx)
            || self.split_item_context_menu_handle.is_focused(window, cx)
    }

    fn focus_out(
        &mut self,
        _event: &FocusOutEvent,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        self.was_focused = false;
        self.toolbar.update(cx, |toolbar, model, cx| {
            toolbar.focus_changed(false, model, cx);
        });
        model.notify(cx);
    }

    fn project_events(
        this: &mut Pane,
        _project: Model<Project>,
        event: &project::Event,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        match event {
            project::Event::DiskBasedDiagnosticsFinished { .. }
            | project::Event::DiagnosticsUpdated { .. } => {
                if ItemSettings::get_global(cx).show_diagnostics != ShowDiagnostics::Off {
                    this.update_diagnostics(model, cx);
                    model.notify(cx);
                }
            }
            _ => {}
        }
    }

    fn update_diagnostics(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let show_diagnostics = ItemSettings::get_global(cx).show_diagnostics;
        self.diagnostics = if show_diagnostics != ShowDiagnostics::Off {
            self.project
                .read(cx)
                .diagnostic_summaries(false, cx)
                .filter_map(|(project_path, _, diagnostic_summary)| {
                    if diagnostic_summary.error_count > 0 {
                        Some((project_path, DiagnosticSeverity::ERROR))
                    } else if diagnostic_summary.warning_count > 0
                        && show_diagnostics != ShowDiagnostics::Errors
                    {
                        Some((project_path, DiagnosticSeverity::WARNING))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>()
        } else {
            Default::default()
        }
    }

    fn settings_changed(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(display_nav_history_buttons) = self.display_nav_history_buttons.as_mut() {
            *display_nav_history_buttons = TabBarSettings::get_global(cx).show_nav_history_buttons;
        }
        if !PreviewTabsSettings::get_global(cx).enabled {
            self.preview_item_id = None;
        }
        self.update_diagnostics(model, cx);
        model.notify(cx);
    }

    pub fn active_item_index(&self) -> usize {
        self.active_item_index
    }

    pub fn activation_history(&self) -> &[ActivationHistoryEntry] {
        &self.activation_history
    }

    pub fn set_should_display_tab_bar<F>(&mut self, should_display_tab_bar: F)
    where
        F: 'static + Fn(&Model<Pane>, &AppContext) -> bool,
    {
        self.should_display_tab_bar = Rc::new(should_display_tab_bar);
    }

    pub fn set_can_split(
        &mut self,
        can_split_predicate: Option<
            Arc<dyn Fn(&mut Self, &dyn Any, &Model<Self>, &mut AppContext) -> bool + 'static>,
        >,
    ) {
        self.can_split_predicate = can_split_predicate;
    }

    pub fn set_can_navigate(
        &mut self,
        can_navigate: bool,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.toolbar.update(cx, |toolbar, model, cx| {
            toolbar.set_can_navigate(can_navigate, model, cx);
        });
        model.notify(cx);
    }

    pub fn set_render_tab_bar_buttons<F>(
        &mut self,
        model: &Model<Self>,
        cx: &mut AppContext,
        render: F,
    ) where
        F: 'static
            + Fn(
                &mut Pane,
                &Model<Pane>,
                &mut Window,
                &mut AppContext,
            ) -> (Option<AnyElement>, Option<AnyElement>),
    {
        self.render_tab_bar_buttons = Rc::new(render);
        model.notify(cx);
    }

    pub fn set_custom_drop_handle<F>(&mut self, model: &Model<Self>, cx: &mut AppContext, handle: F)
    where
        F: 'static + Fn(&mut Pane, &dyn Any, &Model<Pane>, &mut AppContext) -> ControlFlow<(), ()>,
    {
        self.custom_drop_handle = Some(Arc::new(handle));
        model.notify(cx);
    }

    pub fn nav_history_for_item<T: Item>(&self, item: &Model<T>) -> ItemNavHistory {
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

    fn navigate_backward(&mut self, model: &Model<Self>, window: &mut Window, cx: &mut AppContext) {
        if let Some(workspace) = self.workspace.upgrade() {
            let pane = model.downgrade();
            window.defer(cx, move |window, cx| {
                workspace.update(cx, |workspace, model, cx| {
                    workspace
                        .go_back(pane, model, window, cx)
                        .detach_and_log_err(cx)
                })
            })
        }
    }

    fn navigate_forward(&mut self, model: &Model<Self>, window: &mut Window, cx: &mut AppContext) {
        if let Some(workspace) = self.workspace.upgrade() {
            let pane = model.downgrade();
            window.defer(cx, move |window, cx| {
                workspace.update(cx, |workspace, model, cx| {
                    workspace
                        .go_forward(pane, model, window, cx)
                        .detach_and_log_err(cx)
                })
            })
        }
    }

    fn join_into_next(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        model.emit(Event::JoinIntoNext, cx);
    }

    fn join_all(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        model.emit(Event::JoinAll, cx);
    }

    fn history_updated(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        self.toolbar.update(cx, |_, model, cx| model.notify(cx));
    }

    pub fn preview_item_id(&self) -> Option<EntityId> {
        self.preview_item_id
    }

    pub fn preview_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.preview_item_id
            .and_then(|id| self.items.iter().find(|item| item.item_id() == id))
            .cloned()
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

    pub(crate) fn set_pinned_count(&mut self, count: usize) {
        self.pinned_tab_count = count;
    }

    pub(crate) fn pinned_count(&self) -> usize {
        self.pinned_tab_count
    }

    pub fn handle_item_edit(&mut self, item_id: EntityId, cx: &AppContext) {
        if let Some(preview_item) = self.preview_item() {
            if preview_item.item_id() == item_id && !preview_item.preserve_preview(cx) {
                self.set_preview_item_id(None, cx);
            }
        }
    }

    pub(crate) fn open_item(
        &mut self,
        project_entry_id: Option<ProjectEntryId>,
        focus_item: bool,
        allow_preview: bool,
        suggested_position: Option<usize>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
        build_item: impl FnOnce(&Model<Pane>, &mut AppContext) -> Box<dyn ItemHandle>,
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

            self.activate_item(index, focus_item, focus_item, model, window, cx);
            existing_item
        } else {
            // If the item is being opened as preview and we have an existing preview tab,
            // open the new item in the position of the existing preview tab.
            let destination_index = if allow_preview {
                self.close_current_preview_item(model, window, cx)
            } else {
                suggested_position
            };

            let new_item = build_item(model, cx);

            if allow_preview {
                self.set_preview_item_id(Some(new_item.item_id()), cx);
            }

            self.add_item(
                new_item.clone(),
                true,
                focus_item,
                destination_index,
                model,
                window,
                cx,
            );

            new_item
        }
    }

    pub fn close_current_preview_item(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<usize> {
        let item_idx = self.preview_item_idx()?;
        let id = self.preview_item_id()?;

        let prev_active_item_index = self.active_item_index;
        self.remove_item(id, false, false, model, window, cx);
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
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if item.is_singleton(cx) {
            if let Some(&entry_id) = item.project_entry_ids(cx).first() {
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
        // If no destination index is specified, add or move the item after the
        // active item (or at the start of tab bar, if the active item is pinned)
        let mut insertion_index = {
            cmp::min(
                if let Some(destination_index) = destination_index {
                    destination_index
                } else {
                    cmp::max(self.active_item_index + 1, self.pinned_count())
                },
                self.items.len(),
            )
        };

        // Does the item already exist?
        let project_entry_id = if item.is_singleton(cx) {
            item.project_entry_ids(cx).first().copied()
        } else {
            None
        };

        let existing_item_index = self.items.iter().position(|existing_item| {
            if existing_item.item_id() == item.item_id() {
                true
            } else if existing_item.is_singleton(cx) {
                existing_item
                    .project_entry_ids(cx)
                    .first()
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

                model.notify(cx);
            }

            self.activate_item(
                insertion_index,
                activate_pane,
                focus_item,
                model,
                window,
                cx,
            );
        } else {
            self.items.insert(insertion_index, item.clone());

            if insertion_index <= self.active_item_index
                && self.preview_item_idx() != Some(self.active_item_index)
            {
                self.active_item_index += 1;
            }

            self.activate_item(
                insertion_index,
                activate_pane,
                focus_item,
                model,
                window,
                cx,
            );
            model.notify(cx);
        }

        model.emit(Event::AddItem { item }, cx);
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> impl DoubleEndedIterator<Item = &Box<dyn ItemHandle>> {
        self.items.iter()
    }

    pub fn items_of_type<T: 'static>(&self) -> impl '_ + Iterator<Item = Model<T>> {
        self.items
            .iter()
            .filter_map(|item| item.model().downcast().ok())
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
            if item.is_singleton(cx) && (item.project_entry_ids(cx).as_slice() == [entry_id]) {
                Some(item.boxed_clone())
            } else {
                None
            }
        })
    }

    pub fn item_for_path(
        &self,
        project_path: ProjectPath,
        cx: &AppContext,
    ) -> Option<Box<dyn ItemHandle>> {
        self.items.iter().find_map(move |item| {
            if item.is_singleton(cx) && (item.project_path(cx).as_slice() == [project_path.clone()])
            {
                Some(item.boxed_clone())
            } else {
                None
            }
        })
    }

    pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize> {
        self.index_for_item_id(item.item_id())
    }

    fn index_for_item_id(&self, item_id: EntityId) -> Option<usize> {
        self.items.iter().position(|i| i.item_id() == item_id)
    }

    pub fn item_for_index(&self, ix: usize) -> Option<&dyn ItemHandle> {
        self.items.get(ix).map(|i| i.as_ref())
    }

    pub fn toggle_zoom(
        &mut self,
        _: &ToggleZoom,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if self.zoomed {
            model.emit(Event::ZoomOut, cx);
        } else if !self.items.is_empty() {
            if !self.focus_handle.contains_focused(window) {
                self.focus_handle.focus(window);
            }
            model.emit(Event::ZoomIn, cx);
        }
    }

    pub fn activate_item(
        &mut self,
        index: usize,
        activate_pane: bool,
        focus_item: bool,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        use NavigationMode::{GoingBack, GoingForward};

        if index < self.items.len() {
            let prev_active_item_ix = mem::replace(&mut self.active_item_index, index);
            if prev_active_item_ix != self.active_item_index
                || matches!(self.nav_history.mode(), GoingBack | GoingForward)
            {
                if let Some(prev_item) = self.items.get(prev_active_item_ix) {
                    prev_item.deactivated(window, cx);
                }
            }
            model.emit(
                Event::ActivateItem {
                    local: activate_pane,
                },
                cx,
            );

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

            self.update_toolbar(model, cx);
            self.update_status_bar(model, window, cx);

            if focus_item {
                self.focus_active_item(window, cx);
            }

            if !self.is_tab_pinned(index) {
                self.tab_bar_scroll_handle
                    .scroll_to_item(index - self.pinned_tab_count);
            }

            model.notify(cx);
        }
    }

    pub fn activate_prev_item(
        &mut self,
        activate_pane: bool,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let mut index = self.active_item_index;
        if index > 0 {
            index -= 1;
        } else if !self.items.is_empty() {
            index = self.items.len() - 1;
        }
        self.activate_item(index, activate_pane, activate_pane, model, window, cx);
    }

    pub fn activate_next_item(
        &mut self,
        activate_pane: bool,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let mut index = self.active_item_index;
        if index + 1 < self.items.len() {
            index += 1;
        } else {
            index = 0;
        }
        self.activate_item(index, activate_pane, activate_pane, model, window, cx);
    }

    pub fn swap_item_left(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let index = self.active_item_index;
        if index == 0 {
            return;
        }

        self.items.swap(index, index - 1);
        self.activate_item(index - 1, true, true, model, window, cx);
    }

    pub fn swap_item_right(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let index = self.active_item_index;
        if index + 1 == self.items.len() {
            return;
        }

        self.items.swap(index, index + 1);
        self.activate_item(index + 1, true, true, model, window, cx);
    }

    pub fn close_active_item(
        &mut self,
        action: &CloseActiveItem,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            // Close the window when there's no active items to close, if configured
            if WorkspaceSettings::get_global(cx)
                .when_closing_with_no_tabs
                .should_close()
            {
                window.dispatch_action(Box::new(CloseWindow), cx);
            }

            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        Some(self.close_item_by_id(
            active_item_id,
            action.save_intent.unwrap_or(SaveIntent::Close),
            model,
            window,
            cx,
        ))
    }

    pub fn close_item_by_id(
        &mut self,
        item_id_to_close: EntityId,
        save_intent: SaveIntent,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Task<Result<()>> {
        self.close_items(save_intent, model, window, cx, move |view_id| {
            view_id == item_id_to_close
        })
    }

    pub fn close_inactive_items(
        &mut self,
        action: &CloseInactiveItems,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        let active_item_id = self.items[self.active_item_index].item_id();
        let non_closeable_items = self.get_non_closeable_item_ids(action.close_pinned);
        Some(self.close_items(
            action.save_intent.unwrap_or(SaveIntent::Close),
            model,
            window,
            cx,
            move |item_id| item_id != active_item_id && !non_closeable_items.contains(&item_id),
        ))
    }

    pub fn close_clean_items(
        &mut self,
        action: &CloseCleanItems,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        let item_ids: Vec<_> = self
            .items()
            .filter(|item| !item.is_dirty(cx))
            .map(|item| item.item_id())
            .collect();
        let non_closeable_items = self.get_non_closeable_item_ids(action.close_pinned);
        Some(
            self.close_items(SaveIntent::Close, model, window, cx, move |item_id| {
                item_ids.contains(&item_id) && !non_closeable_items.contains(&item_id)
            }),
        )
    }

    pub fn close_items_to_the_left(
        &mut self,
        action: &CloseItemsToTheLeft,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        let non_closeable_items = self.get_non_closeable_item_ids(action.close_pinned);
        Some(self.close_items_to_the_left_by_id(
            active_item_id,
            non_closeable_items,
            model,
            window,
            cx,
        ))
    }

    pub fn close_items_to_the_left_by_id(
        &mut self,
        item_id: EntityId,
        non_closeable_items: Vec<EntityId>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .take_while(|item| item.item_id() != item_id)
            .map(|item| item.item_id())
            .collect();
        self.close_items(SaveIntent::Close, model, window, cx, move |item_id| {
            item_ids.contains(&item_id) && !non_closeable_items.contains(&item_id)
        })
    }

    pub fn close_items_to_the_right(
        &mut self,
        action: &CloseItemsToTheRight,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].item_id();
        let non_closeable_items = self.get_non_closeable_item_ids(action.close_pinned);
        Some(self.close_items_to_the_right_by_id(
            active_item_id,
            non_closeable_items,
            model,
            window,
            cx,
        ))
    }

    pub fn close_items_to_the_right_by_id(
        &mut self,
        item_id: EntityId,
        non_closeable_items: Vec<EntityId>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .rev()
            .take_while(|item| item.item_id() != item_id)
            .map(|item| item.item_id())
            .collect();
        self.close_items(SaveIntent::Close, model, window, cx, move |item_id| {
            item_ids.contains(&item_id) && !non_closeable_items.contains(&item_id)
        })
    }

    pub fn close_all_items(
        &mut self,
        action: &CloseAllItems,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        let non_closeable_items = self.get_non_closeable_item_ids(action.close_pinned);
        Some(self.close_items(
            action.save_intent.unwrap_or(SaveIntent::Close),
            model,
            window,
            cx,
            |item_id| !non_closeable_items.contains(&item_id),
        ))
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
        mut save_intent: SaveIntent,
        model: &Model<Pane>,
        window: &mut Window,
        cx: &mut AppContext,
        should_close: impl Fn(EntityId) -> bool,
    ) -> Task<Result<()>> {
        // Find the items to close.
        let mut items_to_close = Vec::new();
        let mut item_ids_to_close = HashSet::default();
        let mut dirty_items = Vec::new();
        for item in &self.items {
            if should_close(item.item_id()) {
                items_to_close.push(item.boxed_clone());
                item_ids_to_close.insert(item.item_id());
                if item.is_dirty(cx) {
                    dirty_items.push(item.boxed_clone());
                }
            }
        }

        let active_item_id = self.active_item().map(|item| item.item_id());

        items_to_close.sort_by_key(|item| {
            // Put the currently active item at the end, because if the currently active item is not closed last
            // closing the currently active item will cause the focus to switch to another item
            // This will cause Zed to expand the content of the currently active item
            active_item_id.filter(|&id| id == item.item_id()).is_some()
              // If a buffer is open both in a singleton editor and in a multibuffer, make sure
              // to focus the singleton buffer when prompting to save that buffer, as opposed
              // to focusing the multibuffer, because this gives the user a more clear idea
              // of what content they would be saving.
              || !item.is_singleton(cx)
        });

        let workspace = self.workspace.clone();
        let window = window.handle();
        model.spawn(cx, move |pane, mut cx| async move {
            if save_intent == SaveIntent::Close && dirty_items.len() > 1 {
                let answer = pane.update(&mut cx, |_, model, cx| {
                    let (prompt, detail) =
                        Self::file_names_for_prompt(&mut dirty_items.iter(), dirty_items.len(), cx);
                    window.update(cx, |window, cx| {
                        window.prompt(
                            PromptLevel::Warning,
                            &prompt,
                            Some(&detail),
                            &["Save all", "Discard all", "Cancel"],
                            cx,
                        )
                    })
                })??;
                match answer.await {
                    Ok(0) => save_intent = SaveIntent::SaveAll,
                    Ok(1) => save_intent = SaveIntent::Skip,
                    _ => {}
                }
            }
            let mut saved_project_items_ids = HashSet::default();
            for item_to_close in items_to_close {
                // Find the item's current index and its set of dirty project item models. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let mut dirty_project_item_ids = Vec::new();
                let Some(item_ix) = pane.update(&mut cx, |pane, model, cx| {
                    item_to_close.for_each_project_item(
                        cx,
                        &mut |project_item_id, project_item| {
                            if project_item.is_dirty() {
                                dirty_project_item_ids.push(project_item_id);
                            }
                        },
                    );
                    pane.index_for_item(&*item_to_close)
                })?
                else {
                    continue;
                };

                // Check if this view has any project items that are not open anywhere else
                // in the workspace, AND that the user has not already been prompted to save.
                // If there are any such project entries, prompt the user to save this item.
                let project = workspace.update(&mut cx, |workspace, window, cx| {
                    for open_item in workspace.items(cx) {
                        let open_item_id = open_item.item_id();
                        if !item_ids_to_close.contains(&open_item_id) {
                            let other_project_item_ids = open_item.project_item_model_ids(cx);
                            dirty_project_item_ids
                                .retain(|id| !other_project_item_ids.contains(id));
                        }
                    }
                    workspace.project().clone()
                })?;
                let should_save = dirty_project_item_ids
                    .iter()
                    .any(|id| saved_project_items_ids.insert(*id))
                    // Always propose to save singleton files without any project paths: those cannot be saved via multibuffer, as require a file path selection modal.
                    || cx
                        .update(|cx| {
                            item_to_close.is_dirty(cx)
                                && item_to_close.is_singleton(cx)
                                && item_to_close.project_path(cx).is_none()
                        })
                        .unwrap_or(false);

                if should_save
                    && !Self::save_item(
                        project.clone(),
                        &pane,
                        item_ix,
                        &*item_to_close,
                        save_intent,
                        window.clone(),
                        &mut cx,
                    )
                    .await?
                {
                    break;
                }

                // Remove the item from the pane.
                pane.update_in_window(window, &mut cx, |pane, model, window, cx| {
                    pane.remove_item(item_to_close.item_id(), false, true, model, window, cx);
                })
                .ok();
            }

            pane.update(&mut cx, |_, model, cx| model.notify(cx)).ok();
            Ok(())
        })
    }

    pub fn remove_item(
        &mut self,
        item_id: EntityId,
        activate_pane: bool,
        close_pane_if_empty: bool,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let Some(item_index) = self.index_for_item_id(item_id) else {
            return;
        };
        self._remove_item(
            item_index,
            activate_pane,
            close_pane_if_empty,
            None,
            model,
            window,
            cx,
        )
    }

    pub fn remove_item_and_focus_on_pane(
        &mut self,
        item_index: usize,
        activate_pane: bool,
        focus_on_pane_if_closed: Model<Pane>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        self._remove_item(
            item_index,
            activate_pane,
            true,
            Some(focus_on_pane_if_closed),
            model,
            window,
            cx,
        )
    }

    fn _remove_item(
        &mut self,
        item_index: usize,
        activate_pane: bool,
        close_pane_if_empty: bool,
        focus_on_pane_if_closed: Option<Model<Pane>>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let activate_on_close = &ItemSettings::get_global(cx).activate_on_close;
        self.activation_history
            .retain(|entry| entry.entity_id != self.items[item_index].item_id());

        if self.is_tab_pinned(item_index) {
            self.pinned_tab_count -= 1;
        }
        if item_index == self.active_item_index {
            let index_to_activate = match activate_on_close {
                ActivateOnClose::History => self
                    .activation_history
                    .pop()
                    .and_then(|last_activated_item| {
                        self.items.iter().enumerate().find_map(|(index, item)| {
                            (item.item_id() == last_activated_item.entity_id).then_some(index)
                        })
                    })
                    // We didn't have a valid activation history entry, so fallback
                    // to activating the item to the left
                    .unwrap_or_else(|| item_index.min(self.items.len()).saturating_sub(1)),
                ActivateOnClose::Neighbour => {
                    self.activation_history.pop();
                    if item_index + 1 < self.items.len() {
                        item_index + 1
                    } else {
                        item_index.saturating_sub(1)
                    }
                }
            };

            let should_activate = activate_pane || self.has_focus(window, cx);
            if self.items.len() == 1 && should_activate {
                self.focus_handle.focus(window);
            } else {
                self.activate_item(
                    index_to_activate,
                    should_activate,
                    should_activate,
                    model,
                    window,
                    cx,
                );
            }
        }

        model.emit(Event::RemoveItem { idx: item_index }, cx);

        let item = self.items.remove(item_index);

        model.emit(
            Event::RemovedItem {
                item_id: item.item_id(),
            },
            cx,
        );
        if self.items.is_empty() {
            item.deactivated(window, cx);
            if close_pane_if_empty {
                self.update_toolbar(model, cx);
                model.emit(
                    Event::Remove {
                        focus_on_pane: focus_on_pane_if_closed,
                    },
                    cx,
                );
            }
        }

        if item_index < self.active_item_index {
            self.active_item_index -= 1;
        }

        let mode = self.nav_history.mode();
        self.nav_history.set_mode(NavigationMode::ClosingItem);
        item.deactivated(window, cx);
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

        if self.zoom_out_on_close && self.items.is_empty() && close_pane_if_empty && self.zoomed {
            model.emit(Event::ZoomOut, cx);
        }

        model.notify(cx);
    }

    pub async fn save_item(
        project: Model<Project>,
        pane: &WeakModel<Pane>,
        item_ix: usize,
        item: &dyn ItemHandle,
        save_intent: SaveIntent,
        window: AnyWindowHandle,
        cx: &mut AsyncAppContext,
    ) -> Result<bool> {
        const CONFLICT_MESSAGE: &str =
                "This file has changed on disk since you started editing it. Do you want to overwrite it?";

        const DELETED_MESSAGE: &str =
                        "This file has been deleted on disk since you started editing it. Do you want to recreate it?";

        if save_intent == SaveIntent::Skip {
            return Ok(true);
        }

        let (mut has_conflict, mut is_dirty, mut can_save, is_singleton, has_deleted_file) = cx
            .update(|cx| {
                (
                    item.has_conflict(cx),
                    item.is_dirty(cx),
                    item.can_save(cx),
                    item.is_singleton(cx),
                    item.has_deleted_file(cx),
                )
            })?;

        let can_save_as = is_singleton;

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
            if has_deleted_file && is_singleton {
                let answer = pane.update(cx, |pane, model, cx| {
                    window.update(cx, |window, cx| {
                        pane.activate_item(item_ix, true, true, model, window, cx);
                        window.prompt(
                            PromptLevel::Warning,
                            DELETED_MESSAGE,
                            None,
                            &["Save", "Close", "Cancel"],
                            cx,
                        )
                    })
                })??;
                match answer.await {
                    Ok(0) => {
                        pane.update(cx, |_, model, cx| item.save(should_format, project, cx))?
                            .await?
                    }
                    Ok(1) => {
                        pane.update_in_window(window, cx, |pane, model, window, cx| {
                            pane.remove_item(item.item_id(), false, false, model, window, cx)
                        })?;
                    }
                    _ => return Ok(false),
                }
                return Ok(true);
            } else {
                let answer = pane.update(cx, |pane, model, cx| {
                    window.update(cx, |window, cx| {
                        pane.activate_item(item_ix, true, true, model, window, cx);
                        window.prompt(
                            PromptLevel::Warning,
                            CONFLICT_MESSAGE,
                            None,
                            &["Overwrite", "Discard", "Cancel"],
                            cx,
                        )
                    })
                })??;
                match answer.await {
                    Ok(0) => {
                        pane.update(cx, |_, model, cx| item.save(should_format, project, cx))?
                            .await?
                    }
                    Ok(1) => {
                        pane.update(cx, |_, model, cx| item.reload(project, cx))?
                            .await?
                    }
                    _ => return Ok(false),
                }
            }
        } else if is_dirty && (can_save || can_save_as) {
            if save_intent == SaveIntent::Close {
                let will_autosave = cx.update(|cx| {
                    matches!(
                        item.workspace_settings(cx).autosave,
                        AutosaveSetting::OnFocusChange | AutosaveSetting::OnWindowChange
                    ) && Self::can_autosave_item(item, cx)
                })?;
                if !will_autosave {
                    let item_id = item.item_id();
                    let answer_task = pane.update(cx, |pane, model, cx| {
                        if pane.save_modals_spawned.insert(item_id) {
                            let prompt = dirty_message_for(item.project_path(cx));
                            window
                                .update(cx, |window, cx| {
                                    pane.activate_item(item_ix, true, true, model, window, cx);
                                    window.prompt(
                                        PromptLevel::Warning,
                                        &prompt,
                                        None,
                                        &["Save", "Don't Save", "Cancel"],
                                        cx,
                                    )
                                })
                                .ok()
                        } else {
                            None
                        }
                    })?;
                    if let Some(answer_task) = answer_task {
                        let answer = answer_task.await;
                        pane.update(cx, |pane, model, _| {
                            if !pane.save_modals_spawned.remove(&item_id) {
                                debug_panic!(
                                    "save modal was not present in spawned modals after awaiting for its answer"
                                )
                            }
                        })?;
                        match answer {
                            Ok(0) => {}
                            Ok(1) => {
                                // Don't save this file
                                pane.update_in_window(window, cx, |pane, model, window, cx| {
                                    if pane.is_tab_pinned(item_ix) && !item.can_save(cx) {
                                        pane.pinned_tab_count -= 1;
                                    }
                                    item.discarded(project, cx)
                                })
                                .log_err();
                                return Ok(true);
                            }
                            _ => return Ok(false), // Cancel
                        }
                    } else {
                        return Ok(false);
                    }
                }
            }

            if can_save {
                pane.update_in_window(window, cx, |pane, model, window, cx| {
                    if pane.is_active_preview_item(item.item_id()) {
                        pane.set_preview_item_id(None, cx);
                    }
                    item.save(should_format, project, cx)
                })?
                .await?;
            } else if can_save_as {
                let abs_path = pane.update(cx, |pane, model, cx| {
                    pane.workspace.update(cx, |workspace, model, cx| {
                        workspace.prompt_for_new_path(model, cx)
                    })
                })??;
                if let Some(abs_path) = abs_path.await.ok().flatten() {
                    pane.update_in_window(window, cx, |pane, model, window, cx| {
                        if let Some(item) = pane.item_for_path(abs_path.clone(), cx) {
                            pane.remove_item(item.item_id(), false, false, model, window, cx);
                        }

                        item.save_as(project, abs_path, cx)
                    })?
                    .await?;
                } else {
                    return Ok(false);
                }
            }
        }

        pane.update(cx, |_, model, cx| {
            model.emit(
                Event::UserSavedItem {
                    item: item.downgrade_item(),
                    save_intent,
                },
                cx,
            );
            true
        })
    }

    fn can_autosave_item(item: &dyn ItemHandle, cx: &AppContext) -> bool {
        let is_deleted = item.project_entry_ids(cx).is_empty();
        item.is_dirty(cx) && !item.has_conflict(cx) && item.can_save(cx) && !is_deleted
    }

    pub fn autosave_item(
        item: &dyn ItemHandle,
        project: Model<Project>,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<()>> {
        let format = !matches!(
            item.workspace_settings(cx).autosave,
            AutosaveSetting::AfterDelay { .. }
        );
        if Self::can_autosave_item(item, cx) {
            item.save(format, project, cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn focus(&mut self, window: &mut Window, cx: &mut AppContext) {
        self.focus_handle.focus(window)
    }

    pub fn focus_active_item(&mut self, window: &mut Window, cx: &mut AppContext) {
        if let Some(active_item) = self.active_item() {
            active_item.item_focus_handle(cx).focus(window);
        }
    }

    pub fn split(&mut self, direction: SplitDirection, model: &Model<Self>, cx: &mut AppContext) {
        model.emit(Event::Split(direction), cx);
    }

    pub fn toolbar(&self) -> &Model<Toolbar> {
        &self.toolbar
    }

    pub fn handle_deleted_project_item(
        &mut self,
        entry_id: ProjectEntryId,
        model: &Model<Pane>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Option<()> {
        let item_id = self.items().find_map(|item| {
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == [entry_id] {
                Some(item.item_id())
            } else {
                None
            }
        })?;

        self.remove_item(item_id, false, true, model, window, cx);
        self.nav_history.remove_item(item_id);

        Some(())
    }

    fn update_toolbar(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let active_item = self
            .items
            .get(self.active_item_index)
            .map(|item| item.as_ref());
        self.toolbar.update(cx, |toolbar, model, cx| {
            toolbar.set_active_item(active_item, model, cx);
        });
    }

    fn update_status_bar(&mut self, model: &Model<Self>, window: &mut Window, cx: &mut AppContext) {
        let workspace = self.workspace.clone();
        let pane = model.clone();

        window.defer(cx, move |window, cx| {
            let Ok(status_bar) =
                workspace.update(cx, |workspace, model, _| workspace.status_bar.clone())
            else {
                return;
            };

            status_bar.update(cx, move |status_bar, model, cx| {
                status_bar.set_active_pane(&pane, model, cx);
            });
        });
    }

    fn entry_abs_path(&self, entry: ProjectEntryId, cx: &AppContext) -> Option<PathBuf> {
        let worktree = self
            .workspace
            .upgrade()?
            .read(cx)
            .project()
            .read(cx)
            .worktree_for_entry(entry, cx)?
            .read(cx);
        let entry = worktree.entry_for_id(entry)?;
        match &entry.canonical_path {
            Some(canonical_path) => Some(canonical_path.to_path_buf()),
            None => worktree.absolutize(&entry.path).ok(),
        }
    }

    pub fn icon_color(selected: bool) -> Color {
        if selected {
            Color::Default
        } else {
            Color::Muted
        }
    }

    fn toggle_pin_tab(
        &mut self,
        _: &TogglePinTab,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if self.items.is_empty() {
            return;
        }
        let active_tab_ix = self.active_item_index();
        if self.is_tab_pinned(active_tab_ix) {
            self.unpin_tab_at(active_tab_ix, model, window, cx);
        } else {
            self.pin_tab_at(active_tab_ix, model, window, cx);
        }
    }

    fn pin_tab_at(
        &mut self,
        ix: usize,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        maybe!({
            let pane = model.clone();
            let destination_index = self.pinned_tab_count.min(ix);
            self.pinned_tab_count += 1;
            let id = self.item_for_index(ix)?.item_id();
            window.defer(cx, move |window, cx| {
                move_item(&pane, &pane, id, destination_index, window, cx)
            });
            Some(())
        });
    }

    fn unpin_tab_at(
        &mut self,
        ix: usize,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        maybe!({
            let pane = model.clone();
            self.pinned_tab_count = self.pinned_tab_count.checked_sub(1)?;
            let destination_index = self.pinned_tab_count;

            let id = self.item_for_index(ix)?.item_id();

            window.defer(cx, move |window, cx| {
                move_item(&pane, &pane, id, destination_index, window, cx)
            });

            Some(())
        });
    }

    fn is_tab_pinned(&self, ix: usize) -> bool {
        self.pinned_tab_count > ix
    }

    fn has_pinned_tabs(&self) -> bool {
        self.pinned_tab_count != 0
    }

    fn render_tab(
        &self,
        ix: usize,
        item: &dyn ItemHandle,
        detail: usize,
        focus_handle: &FocusHandle,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
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
            window,
            cx,
        );

        let item_diagnostic = item
            .project_path(cx)
            .map_or(None, |project_path| self.diagnostics.get(&project_path));

        let decorated_icon = item_diagnostic.map_or(None, |diagnostic| {
            let icon = match item.tab_icon(cx) {
                Some(icon) => icon,
                None => return None,
            };

            let knockout_item_color = if is_active {
                cx.theme().colors().tab_active_background
            } else {
                cx.theme().colors().tab_bar_background
            };

            let (icon_decoration, icon_color) = if matches!(diagnostic, &DiagnosticSeverity::ERROR)
            {
                (IconDecorationKind::X, Color::Error)
            } else {
                (IconDecorationKind::Triangle, Color::Warning)
            };

            Some(DecoratedIcon::new(
                icon.size(IconSize::Small).color(Color::Muted),
                Some(
                    IconDecoration::new(icon_decoration, knockout_item_color, cx)
                        .color(icon_color.color(cx))
                        .position(Point {
                            x: px(-2.),
                            y: px(-2.),
                        }),
                ),
            ))
        });

        let icon = if decorated_icon.is_none() {
            match item_diagnostic {
                Some(&DiagnosticSeverity::ERROR) => None,
                Some(&DiagnosticSeverity::WARNING) => None,
                _ => item.tab_icon(cx).map(|icon| icon.color(Color::Muted)),
            }
            .map(|icon| icon.size(IconSize::Small))
        } else {
            None
        };

        let settings = ItemSettings::get_global(cx);
        let close_side = &settings.close_position;
        let always_show_close_button = settings.always_show_close_button;
        let indicator = render_item_indicator(item.boxed_clone(), window, cx);
        let item_id = item.item_id();
        let is_first_item = ix == 0;
        let is_last_item = ix == self.items.len() - 1;
        let is_pinned = self.is_tab_pinned(ix);
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
                model.listener(move |pane: &mut Self, _, model, window, cx| {
                    pane.activate_item(ix, true, true, model, window, cx)
                }),
            )
            // TODO: This should be a click listener with the middle mouse button instead of a mouse down listener.
            .on_mouse_down(
                MouseButton::Middle,
                model.listener(move |pane, _event, model, window, cx| {
                    pane.close_item_by_id(item_id, SaveIntent::Close, model, window, cx)
                        .detach_and_log_err(cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Left,
                model.listener(move |pane, event: &MouseDownEvent, model, window, cx| {
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
                    pane: model.clone(),
                    detail,
                    is_active,
                    ix,
                },
                |tab, _, window, cx| cx.new_model(|_, _| tab.clone()).into(),
            )
            .drag_over::<DraggedTab>(|tab, _, window, cx| {
                tab.bg(cx.theme().colors().drop_target_background)
            })
            .drag_over::<DraggedSelection>(|tab, _, window, cx| {
                tab.bg(cx.theme().colors().drop_target_background)
            })
            .when_some(self.can_drop_predicate.clone(), |this, p| {
                this.can_drop(move |a, window, cx| p(a, window, cx))
            })
            .on_drop(
                model.listener(move |this, dragged_tab: &DraggedTab, model, window, cx| {
                    this.drag_split_direction = None;
                    this.handle_tab_drop(dragged_tab, ix, model, window, cx)
                }),
            )
            .on_drop(model.listener(
                move |this, selection: &DraggedSelection, model, window, cx| {
                    this.drag_split_direction = None;
                    this.handle_dragged_selection_drop(selection, Some(ix), model, window, cx)
                },
            ))
            .on_drop(model.listener(move |this, paths, model, window, cx| {
                this.drag_split_direction = None;
                this.handle_external_paths_drop(paths, model, window, cx)
            }))
            .when_some(item.tab_tooltip_text(cx), |tab, text| {
                tab.tooltip(move |window, cx| Tooltip::text(text.clone(), cx))
            })
            .start_slot::<Indicator>(indicator)
            .map(|this| {
                let end_slot_action: &'static dyn Action;
                let end_slot_tooltip_text: &'static str;
                let end_slot = if is_pinned {
                    end_slot_action = &TogglePinTab;
                    end_slot_tooltip_text = "Unpin Tab";
                    IconButton::new("unpin tab", IconName::Pin)
                        .shape(IconButtonShape::Square)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .on_click(model.listener(move |pane, _, model, window, cx| {
                            pane.unpin_tab_at(ix, model, window, cx);
                        }))
                } else {
                    end_slot_action = &CloseActiveItem { save_intent: None };
                    end_slot_tooltip_text = "Close Tab";
                    IconButton::new("close tab", IconName::Close)
                        .when(!always_show_close_button, |button| {
                            button.visible_on_hover("")
                        })
                        .shape(IconButtonShape::Square)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .on_click(model.listener(move |pane, _, model, window, cx| {
                            pane.close_item_by_id(item_id, SaveIntent::Close, model, window, cx)
                                .detach_and_log_err(cx);
                        }))
                }
                .map(|this| {
                    if is_active {
                        let focus_handle = focus_handle.clone();
                        this.tooltip(move |window, cx| {
                            Tooltip::for_action_in(
                                end_slot_tooltip_text,
                                end_slot_action,
                                &focus_handle,
                                window,
                                cx,
                            )
                        })
                    } else {
                        this.tooltip(move |window, cx| Tooltip::text(end_slot_tooltip_text, cx))
                    }
                });
                this.end_slot(end_slot)
            })
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .children(
                        std::iter::once(if let Some(decorated_icon) = decorated_icon {
                            Some(div().child(decorated_icon.into_any_element()))
                        } else if let Some(icon) = icon {
                            Some(div().child(icon.into_any_element()))
                        } else {
                            None
                        })
                        .flatten(),
                    )
                    .child(label),
            );

        let single_entry_to_resolve = {
            let item_entries = self.items[ix].project_entry_ids(cx);
            if item_entries.len() == 1 {
                Some(item_entries[0])
            } else {
                None
            }
        };

        let is_pinned = self.is_tab_pinned(ix);
        let pane = model.downgrade();
        let menu_context = item.item_focus_handle(cx);
        right_click_menu(ix).trigger(tab).menu(move |window, cx| {
            let pane = pane.clone();
            let menu_context = menu_context.clone();
            ContextMenu::build(window, cx, move |mut menu, model, window, cx| {
                if let Some(pane) = pane.upgrade() {
                    menu = menu
                        .entry(
                            "Close",
                            Some(Box::new(CloseActiveItem { save_intent: None })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        pane.close_item_by_id(
                                            item_id,
                                            SaveIntent::Close,
                                            model,
                                            window,
                                            cx,
                                        )
                                        .detach_and_log_err(cx);
                                    })
                                }
                            },
                        )
                        .entry(
                            "Close Others",
                            Some(Box::new(CloseInactiveItems {
                                save_intent: None,
                                close_pinned: false,
                            })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        pane.close_items(
                                            SaveIntent::Close,
                                            model,
                                            window,
                                            cx,
                                            |id| id != item_id,
                                        )
                                        .detach_and_log_err(cx);
                                    });
                                }
                            },
                        )
                        .separator()
                        .entry(
                            "Close Left",
                            Some(Box::new(CloseItemsToTheLeft {
                                close_pinned: false,
                            })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        pane.close_items_to_the_left_by_id(
                                            item_id,
                                            pane.get_non_closeable_item_ids(false),
                                            model,
                                            window,
                                            cx,
                                        )
                                        .detach_and_log_err(cx);
                                    })
                                }
                            },
                        )
                        .entry(
                            "Close Right",
                            Some(Box::new(CloseItemsToTheRight {
                                close_pinned: false,
                            })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        pane.close_items_to_the_right_by_id(
                                            item_id,
                                            pane.get_non_closeable_item_ids(false),
                                            model,
                                            window,
                                            cx,
                                        )
                                        .detach_and_log_err(cx);
                                    })
                                }
                            },
                        )
                        .separator()
                        .entry(
                            "Close Clean",
                            Some(Box::new(CloseCleanItems {
                                close_pinned: false,
                            })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        if let Some(task) = pane.close_clean_items(
                                            &CloseCleanItems {
                                                close_pinned: false,
                                            },
                                            model,
                                            window,
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    })
                                }
                            },
                        )
                        .entry(
                            "Close All",
                            Some(Box::new(CloseAllItems {
                                save_intent: None,
                                close_pinned: false,
                            })),
                            {
                                let pane = pane.clone();
                                move |window, cx| {
                                    pane.update(cx, |pane, model, cx| {
                                        if let Some(task) = pane.close_all_items(
                                            &CloseAllItems {
                                                save_intent: None,
                                                close_pinned: false,
                                            },
                                            model,
                                            window,
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    })
                                }
                            },
                        );

                    let pin_tab_entries = {
                        let pane = pane.clone();
                        move |menu: ContextMenu| {
                            menu.separator().map(|this| {
                                if is_pinned {
                                    this.entry("Unpin Tab", Some(TogglePinTab.boxed_clone()), {
                                        let pane = pane.clone();
                                        move |window, cx| {
                                            pane.update(cx, |pane, model, cx| {
                                                pane.unpin_tab_at(ix, model, window, cx);
                                            })
                                        }
                                    })
                                } else {
                                    this.entry("Pin Tab", Some(TogglePinTab.boxed_clone()), {
                                        let pane = pane.clone();
                                        move |window, cx| {
                                            pane.update(cx, |pane, model, cx| {
                                                pane.pin_tab_at(ix, model, window, cx);
                                            })
                                        }
                                    })
                                }
                            })
                        }
                    };
                    if let Some(entry) = single_entry_to_resolve {
                        let entry_abs_path = pane.read(cx).entry_abs_path(entry, cx);
                        let parent_abs_path = entry_abs_path
                            .as_deref()
                            .and_then(|abs_path| Some(abs_path.parent()?.to_path_buf()));
                        let relative_path = pane
                            .read(cx)
                            .item_for_entry(entry, cx)
                            .and_then(|item| item.project_path(cx))
                            .map(|project_path| project_path.path);

                        let entry_id = entry.to_proto();
                        menu = menu
                            .separator()
                            .when_some(entry_abs_path, |menu, abs_path| {
                                menu.entry("Copy Path", Some(Box::new(CopyPath)), {
                                    let pane = pane.clone();
                                    move |window, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            abs_path.to_string_lossy().to_string(),
                                        ));
                                    }
                                })
                            })
                            .when_some(relative_path, |menu, relative_path| {
                                menu.entry(
                                    "Copy Relative Path",
                                    Some(Box::new(CopyRelativePath)),
                                    {
                                        let pane = pane.clone();
                                        move |window, cx| {
                                            pane.update(cx, |_, model, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    relative_path.to_string_lossy().to_string(),
                                                ));
                                            })
                                        }
                                    },
                                )
                            })
                            .map(pin_tab_entries)
                            .separator()
                            .entry(
                                "Reveal In Project Panel",
                                Some(Box::new(RevealInProjectPanel {
                                    entry_id: Some(entry_id),
                                })),
                                {
                                    let pane = pane.clone();
                                    move |window, cx| {
                                        pane.update(cx, |pane, model, cx| {
                                            pane.project.update(cx, |_, model, cx| {
                                                model.emit(
                                                    project::Event::RevealInProjectPanel(
                                                        ProjectEntryId::from_proto(entry_id),
                                                    ),
                                                    cx,
                                                )
                                            })
                                        })
                                    }
                                },
                            )
                            .when_some(parent_abs_path, |menu, parent_abs_path| {
                                menu.entry(
                                    "Open in Terminal",
                                    Some(Box::new(OpenInTerminal)),
                                    move |window, cx| {
                                        window.dispatch_action(
                                            OpenTerminal {
                                                working_directory: parent_abs_path.clone(),
                                            }
                                            .boxed_clone(),
                                            cx,
                                        );
                                    },
                                )
                            });
                    } else {
                        menu = menu.map(pin_tab_entries);
                    }
                }

                menu.context(menu_context)
            })
        })
    }

    fn render_tab_bar(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let navigate_backward = IconButton::new("navigate_backward", IconName::ArrowLeft)
            .icon_size(IconSize::Small)
            .on_click({
                let model = model.clone();
                move |_event, window, cx| {
                    model.update(cx, |pane, model, cx| {
                        pane.navigate_backward(model, window, cx)
                    })
                }
            })
            .disabled(!self.can_navigate_backward())
            .tooltip({
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    Tooltip::for_action_in("Go Back", &GoBack, &focus_handle, window, cx)
                }
            });

        let navigate_forward = IconButton::new("navigate_forward", IconName::ArrowRight)
            .icon_size(IconSize::Small)
            .on_click({
                let model = model.clone();
                move |_event, window, cx| {
                    model.update(cx, |pane, model, cx| {
                        pane.navigate_forward(model, window, cx)
                    })
                }
            })
            .disabled(!self.can_navigate_forward())
            .tooltip({
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    Tooltip::for_action_in("Go Forward", &GoForward, &focus_handle, window, cx)
                }
            });

        let mut tab_items = self
            .items
            .iter()
            .enumerate()
            .zip(tab_details(&self.items, cx))
            .map(|((ix, item), detail)| {
                self.render_tab(ix, &**item, detail, &focus_handle, model, window, cx)
            })
            .collect::<Vec<_>>();
        let tab_count = tab_items.len();
        let unpinned_tabs = tab_items.split_off(self.pinned_tab_count);
        let pinned_tabs = tab_items;
        TabBar::new("tab_bar")
            .when(
                self.display_nav_history_buttons.unwrap_or_default(),
                |tab_bar| {
                    tab_bar
                        .start_child(navigate_backward)
                        .start_child(navigate_forward)
                },
            )
            .map(|tab_bar| {
                let render_tab_buttons = self.render_tab_bar_buttons.clone();
                let (left_children, right_children) = render_tab_buttons(self, model, window, cx);

                tab_bar
                    .start_children(left_children)
                    .end_children(right_children)
            })
            .children(pinned_tabs.len().ne(&0).then(|| {
                h_flex()
                    .children(pinned_tabs)
                    .border_r_2()
                    .border_color(cx.theme().colors().border)
            }))
            .child(
                h_flex()
                    .id("unpinned tabs")
                    .overflow_x_scroll()
                    .w_full()
                    .track_scroll(&self.tab_bar_scroll_handle)
                    .children(unpinned_tabs)
                    .child(
                        div()
                            .id("tab_bar_drop_target")
                            .min_w_6()
                            // HACK: This empty child is currently necessary to force the drop target to appear
                            // despite us setting a min width above.
                            .child("")
                            .h_full()
                            .flex_grow()
                            .drag_over::<DraggedTab>(|bar, _, _window, cx| {
                                bar.bg(cx.theme().colors().drop_target_background)
                            })
                            .drag_over::<DraggedSelection>(|bar, _, _window, cx| {
                                bar.bg(cx.theme().colors().drop_target_background)
                            })
                            .on_drop({
                                model.listener(
                                    move |this, dragged_tab: &DraggedTab, model, window, cx| {
                                        this.drag_split_direction = None;
                                        this.handle_tab_drop(
                                            dragged_tab,
                                            this.items.len(),
                                            model,
                                            window,
                                            cx,
                                        );
                                    },
                                )
                            })
                            .on_drop(model.listener(
                                move |this, selection: &DraggedSelection, model, window, cx| {
                                    this.drag_split_direction = None;
                                    this.handle_project_entry_drop(
                                        &selection.active_selection.entry_id,
                                        Some(tab_count),
                                        model,
                                        window,
                                        cx,
                                    )
                                },
                            ))
                            .on_drop(model.listener(move |this, paths, model, window, cx| {
                                this.drag_split_direction = None;
                                this.handle_external_paths_drop(paths, model, window, cx)
                            }))
                            .on_click(model.listener(
                                move |this, event: &ClickEvent, _model, window, cx| {
                                    if event.up.click_count == 2 {
                                        window.dispatch_action(
                                            this.double_click_dispatch_action.boxed_clone(),
                                            cx,
                                        )
                                    }
                                },
                            )),
                    ),
            )
    }

    pub fn render_menu_overlay(menu: &Model<ContextMenu>) -> Div {
        div().absolute().bottom_0().right_0().size_0().child(
            deferred(
                anchored()
                    .anchor(AnchorCorner::TopRight)
                    .child(menu.clone()),
            )
            .with_priority(1),
        )
    }

    pub fn set_zoomed(&mut self, zoomed: bool, model: &Model<Self>, cx: &mut AppContext) {
        self.zoomed = zoomed;
        model.notify(cx);
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed
    }

    fn handle_drag_move<T: 'static>(
        &mut self,
        event: &DragMoveEvent<T>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        let can_split_predicate = self.can_split_predicate.take();
        let can_split = match &can_split_predicate {
            Some(can_split_predicate) => can_split_predicate(self, event.dragged_item(), model, cx),
            None => false,
        };
        self.can_split_predicate = can_split_predicate;
        if !can_split {
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
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, dragged_tab, model, cx) {
                return;
            }
        }
        let mut to_pane = model.clone();
        let split_direction = self.drag_split_direction;
        let item_id = dragged_tab.item.item_id();
        if let Some(preview_item_id) = self.preview_item_id {
            if item_id == preview_item_id {
                self.set_preview_item_id(None, cx);
            }
        }

        let from_pane = dragged_tab.pane.clone();
        let window = window.handle();
        self.workspace.defer(cx, move |workspace, model, cx| {
            window.update(cx, |window, cx| {
                if let Some(split_direction) = split_direction {
                    to_pane = workspace.split_pane(to_pane, split_direction, model, window, cx);
                }
                let old_ix = from_pane.read(cx).index_for_item_id(item_id);
                let old_len = to_pane.read(cx).items.len();
                move_item(&from_pane, &to_pane, item_id, ix, window, cx);
                if to_pane == from_pane {
                    if let Some(old_index) = old_ix {
                        to_pane.update(cx, |this, model, _| {
                            if old_index < this.pinned_tab_count
                                && (ix == this.items.len() || ix > this.pinned_tab_count)
                            {
                                this.pinned_tab_count -= 1;
                            } else if this.has_pinned_tabs()
                                && old_index >= this.pinned_tab_count
                                && ix < this.pinned_tab_count
                            {
                                this.pinned_tab_count += 1;
                            }
                        });
                    }
                } else {
                    to_pane.update(cx, |this, model, _| {
                        if this.items.len() > old_len // Did we not deduplicate on drag?
                                && this.has_pinned_tabs()
                                && ix < this.pinned_tab_count
                        {
                            this.pinned_tab_count += 1;
                        }
                    });
                    from_pane.update(cx, |this, model, _| {
                        if let Some(index) = old_ix {
                            if this.pinned_tab_count > index {
                                this.pinned_tab_count -= 1;
                            }
                        }
                    })
                }
            });
        });
    }

    fn handle_dragged_selection_drop(
        &mut self,
        dragged_selection: &DraggedSelection,
        dragged_onto: Option<usize>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, dragged_selection, model, cx) {
                return;
            }
        }
        self.handle_project_entry_drop(
            &dragged_selection.active_selection.entry_id,
            dragged_onto,
            model,
            window,
            cx,
        );
    }

    fn handle_project_entry_drop(
        &mut self,
        project_entry_id: &ProjectEntryId,
        target: Option<usize>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, project_entry_id, model, cx) {
                return;
            }
        }
        let mut to_pane = model.clone();
        let split_direction = self.drag_split_direction;
        let project_entry_id = *project_entry_id;
        let window = window.handle();
        self.workspace
            .update(cx, |_, model, cx| {
                model.defer_in_window(window, cx, move |workspace, model, window, cx| {
                    if let Some(path) = workspace
                        .project()
                        .read(cx)
                        .path_for_entry(project_entry_id, cx)
                    {
                        let load_path_task = workspace.load_path(path, window, cx);
                        let window = window.handle();
                        model
                            .spawn(cx, |workspace, mut cx| async move {
                                if let Some((project_entry_id, build_item)) =
                                    load_path_task.await.notify_async_err(window, &mut cx)
                                {
                                    let (to_pane, new_item_handle) = workspace
                                        .update_in_window(
                                            window,
                                            &mut cx,
                                            |workspace, model, window, cx| {
                                                if let Some(split_direction) = split_direction {
                                                    to_pane = workspace.split_pane(
                                                        to_pane,
                                                        split_direction,
                                                        model,
                                                        window,
                                                        cx,
                                                    );
                                                }
                                                let new_item_handle =
                                                    to_pane.update(cx, |pane, model, cx| {
                                                        pane.open_item(
                                                            project_entry_id,
                                                            true,
                                                            false,
                                                            target,
                                                            model,
                                                            window,
                                                            cx,
                                                            build_item,
                                                        )
                                                    });
                                                (to_pane, new_item_handle)
                                            },
                                        )
                                        .log_err()?;
                                    to_pane
                                        .update_in_window(
                                            window,
                                            &mut cx,
                                            |this, model, window, cx| {
                                                let Some(index) =
                                                    this.index_for_item(&*new_item_handle)
                                                else {
                                                    return;
                                                };

                                                if target.map_or(false, |target| {
                                                    this.is_tab_pinned(target)
                                                }) {
                                                    this.pin_tab_at(index, model, window, cx);
                                                }
                                            },
                                        )
                                        .ok()?
                                }
                                Some(())
                            })
                            .detach();
                    };
                })
            })
            .log_err();
    }

    fn handle_external_paths_drop(
        &mut self,
        paths: &ExternalPaths,
        model: &Model<Self>,
        window: &Window,
        cx: &mut AppContext,
    ) {
        if let Some(custom_drop_handle) = self.custom_drop_handle.clone() {
            if let ControlFlow::Break(()) = custom_drop_handle(self, paths, model, cx) {
                return;
            }
        }
        let mut to_pane = model.clone();
        let mut split_direction = self.drag_split_direction;
        let paths = paths.paths().to_vec();
        let is_remote = self
            .workspace
            .update(cx, |workspace, model, cx| {
                if workspace.project().read(cx).is_via_collab() {
                    workspace.show_error(
                        &anyhow::anyhow!("Cannot drop files on a remote project"),
                        model,
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

        let window = window.handle();
        self.workspace
            .update(cx, |workspace, model, cx| {
                let fs = Arc::clone(workspace.project().read(cx).fs());
                model
                    .spawn(cx, |workspace, mut cx| async move {
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

                        if let Ok(open_task) = workspace.update_in_window(
                            window,
                            &mut cx,
                            |workspace, model, window, cx| {
                                if let Some(split_direction) = split_direction {
                                    to_pane = workspace.split_pane(
                                        to_pane,
                                        split_direction,
                                        model,
                                        window,
                                        cx,
                                    );
                                }
                                workspace.open_paths(
                                    paths,
                                    OpenVisible::OnlyDirectories,
                                    Some(to_pane.downgrade()),
                                    model,
                                    cx,
                                )
                            },
                        ) {
                            let opened_items: Vec<_> = open_task.await;
                            _ = workspace.update(&mut cx, |workspace, model, cx| {
                                for item in opened_items.into_iter().flatten() {
                                    if let Err(e) = item {
                                        workspace.show_error(&e, model, cx);
                                    }
                                }
                            });
                        }
                    })
                    .detach();
            })
            .log_err();
    }

    pub fn display_nav_history_buttons(&mut self, display: Option<bool>) {
        self.display_nav_history_buttons = display;
    }

    fn get_non_closeable_item_ids(&self, close_pinned: bool) -> Vec<EntityId> {
        if close_pinned {
            return vec![];
        }

        self.items
            .iter()
            .map(|item| item.item_id())
            .filter(|item_id| {
                if let Some(ix) = self.index_for_item_id(*item_id) {
                    self.is_tab_pinned(ix)
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn drag_split_direction(&self) -> Option<SplitDirection> {
        self.drag_split_direction
    }

    pub fn set_zoom_out_on_close(&mut self, zoom_out_on_close: bool) {
        self.zoom_out_on_close = zoom_out_on_close;
    }
}

impl FocusableView for Pane {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Pane {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("Pane");
        if self.active_item().is_none() {
            key_context.add("EmptyPane");
        }

        let should_display_tab_bar = self.should_display_tab_bar.clone();
        let display_tab_bar = should_display_tab_bar(model, cx);
        let is_local = self.project.read(cx).is_local();

        v_flex()
            .key_context(key_context)
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .flex_none()
            .overflow_hidden()
            .on_action(
                model.listener(|pane, _: &AlternateFile, model, window, cx| {
                    pane.alternate_file(model, window, cx);
                }),
            )
            .on_action(model.listener(|pane, _: &SplitLeft, model, window, cx| {
                pane.split(SplitDirection::Left, model, cx)
            }))
            .on_action(model.listener(|pane, _: &SplitUp, model, window, cx| {
                pane.split(SplitDirection::Up, model, cx)
            }))
            .on_action(
                model.listener(|pane, _: &SplitHorizontal, model, window, cx| {
                    pane.split(SplitDirection::horizontal(cx), model, cx)
                }),
            )
            .on_action(
                model.listener(|pane, _: &SplitVertical, model, window, cx| {
                    pane.split(SplitDirection::vertical(cx), model, cx)
                }),
            )
            .on_action(model.listener(|pane, _: &SplitRight, model, window, cx| {
                pane.split(SplitDirection::Right, model, cx)
            }))
            .on_action(model.listener(|pane, _: &SplitDown, model, window, cx| {
                pane.split(SplitDirection::Down, model, cx)
            }))
            .on_action(model.listener(|pane, _: &GoBack, model, window, cx| {
                pane.navigate_backward(model, window, cx)
            }))
            .on_action(model.listener(|pane, _: &GoForward, model, window, cx| {
                pane.navigate_forward(model, window, cx)
            }))
            .on_action(model.listener(|pane, _: &JoinIntoNext, model, window, cx| {
                pane.join_into_next(model, cx)
            }))
            .on_action(
                model.listener(|pane, _: &JoinAll, model, window, cx| pane.join_all(model, cx)),
            )
            .on_action(model.listener(Pane::toggle_zoom))
            .on_action(model.listener(
                |pane: &mut Pane, action: &ActivateItem, model, window, cx| {
                    pane.activate_item(action.0, true, true, model, window, cx);
                },
            ))
            .on_action(model.listener(
                |pane: &mut Pane, _: &ActivateLastItem, model, window, cx| {
                    pane.activate_item(pane.items.len() - 1, true, true, model, window, cx);
                },
            ))
            .on_action(model.listener(
                |pane: &mut Pane, _: &ActivatePrevItem, model, window, cx| {
                    pane.activate_prev_item(true, model, window, cx);
                },
            ))
            .on_action(model.listener(
                |pane: &mut Pane, _: &ActivateNextItem, model, window, cx| {
                    pane.activate_next_item(true, model, window, cx);
                },
            ))
            .on_action(model.listener(|pane, _: &SwapItemLeft, model, window, cx| {
                pane.swap_item_left(model, window, cx)
            }))
            .on_action(
                model.listener(|pane, _: &SwapItemRight, model, window, cx| {
                    pane.swap_item_right(model, window, cx)
                }),
            )
            .on_action(model.listener(|pane, action, model, window, cx| {
                pane.toggle_pin_tab(action, model, window, cx);
            }))
            .when(PreviewTabsSettings::get_global(cx).enabled, |this| {
                this.on_action(model.listener(
                    |pane: &mut Pane, _: &TogglePreviewTab, model, window, cx| {
                        if let Some(active_item_id) = pane.active_item().map(|i| i.item_id()) {
                            if pane.is_active_preview_item(active_item_id) {
                                pane.set_preview_item_id(None, cx);
                            } else {
                                pane.set_preview_item_id(Some(active_item_id), cx);
                            }
                        }
                    },
                ))
            })
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseActiveItem, model, window, cx| {
                    if let Some(task) = pane.close_active_item(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseInactiveItems, model, window, cx| {
                    if let Some(task) = pane.close_inactive_items(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseCleanItems, model, window, cx| {
                    if let Some(task) = pane.close_clean_items(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseItemsToTheLeft, model, window, cx| {
                    if let Some(task) = pane.close_items_to_the_left(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseItemsToTheRight, model, window, cx| {
                    if let Some(task) = pane.close_items_to_the_right(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseAllItems, model, window, cx| {
                    if let Some(task) = pane.close_all_items(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &CloseActiveItem, model, window, cx| {
                    if let Some(task) = pane.close_active_item(action, model, window, cx) {
                        task.detach_and_log_err(cx)
                    }
                },
            ))
            .on_action(model.listener(
                |pane: &mut Self, action: &RevealInProjectPanel, model, window, cx| {
                    let entry_id = action
                        .entry_id
                        .map(ProjectEntryId::from_proto)
                        .or_else(|| pane.active_item()?.project_entry_ids(cx).first().copied());
                    if let Some(entry_id) = entry_id {
                        pane.project.update(cx, |_, model, cx| {
                            model.emit(project::Event::RevealInProjectPanel(entry_id), cx)
                        });
                    }
                },
            ))
            .when(self.active_item().is_some() && display_tab_bar, |pane| {
                pane.child(self.render_tab_bar(model, window, cx))
            })
            .child({
                let has_worktrees = self.project.read(cx).worktrees(cx).next().is_some();
                // main content
                div()
                    .flex_1()
                    .relative()
                    .group("")
                    .overflow_hidden()
                    .on_drag_move::<DraggedTab>(model.listener(Self::handle_drag_move))
                    .on_drag_move::<DraggedSelection>(model.listener(Self::handle_drag_move))
                    // .when(is_local, |div| {
                    //     div.on_drag_move::<ExternalPaths>(model.listener(Self::handle_drag_move))
                    // })
                    .map(|div| {
                        if let Some(item) = self.active_item() {
                            div.v_flex()
                                .size_full()
                                .overflow_hidden()
                                .child(self.toolbar.clone())
                                .child(item.view())
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
                            .group_drag_over::<DraggedSelection>("", |style| style.visible())
                            .when(is_local, |div| {
                                div.group_drag_over::<ExternalPaths>("", |style| style.visible())
                            })
                            .when_some(self.can_drop_predicate.clone(), |this, p| {
                                this.can_drop(move |a, model, cx| p(a, model, cx))
                            })
                            .on_drop(model.listener(move |this, dragged_tab, model, window, cx| {
                                this.handle_tab_drop(
                                    dragged_tab,
                                    this.active_item_index(),
                                    model,
                                    window,
                                    cx,
                                )
                            }))
                            .on_drop(model.listener(
                                move |this, selection: &DraggedSelection, model, window, cx| {
                                    this.handle_dragged_selection_drop(
                                        selection, None, model, window, cx,
                                    )
                                },
                            ))
                            .on_drop(model.listener(move |this, paths, model, window, cx| {
                                this.handle_external_paths_drop(paths, model, window, cx)
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
                model.listener(|pane, _, model, window, cx| {
                    if let Some(workspace) = pane.workspace.upgrade() {
                        let pane = model.downgrade();
                        window.defer(cx, move |window, cx| {
                            workspace.update(cx, |workspace, model, cx| {
                                workspace
                                    .go_back(pane, model, window, cx)
                                    .detach_and_log_err(cx)
                            })
                        })
                    }
                }),
            )
            .on_mouse_down(
                MouseButton::Navigate(NavigationDirection::Forward),
                model.listener(|pane, _, model, window, cx| {
                    if let Some(workspace) = pane.workspace.upgrade() {
                        let pane = model.downgrade();
                        window.defer(cx, move |window, cx| {
                            workspace.update(cx, |workspace, model, cx| {
                                workspace
                                    .go_forward(pane, model, window, cx)
                                    .detach_and_log_err(cx)
                            })
                        })
                    }
                }),
            )
    }
}

impl ItemNavHistory {
    pub fn push<D: 'static + Send + Any>(
        &mut self,
        data: Option<D>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        self.history
            .push(data, self.item.clone(), self.is_preview, window, cx);
    }

    pub fn pop_backward(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Option<NavigationEntry> {
        self.history.pop(NavigationMode::GoingBack, window, cx)
    }

    pub fn pop_forward(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Option<NavigationEntry> {
        self.history.pop(NavigationMode::GoingForward, window, cx)
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

    pub fn pop(
        &mut self,
        mode: NavigationMode,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Option<NavigationEntry> {
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
            state.did_update(window, cx);
        }
        entry
    }

    pub fn push<D: 'static + Send + Any>(
        &mut self,
        data: Option<D>,
        item: Arc<dyn WeakItemHandle>,
        is_preview: bool,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
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
        state.did_update(window, cx);
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
    pub fn did_update(&self, window: &mut gpui::Window, cx: &mut gpui::AppContext) {
        if let Some(pane) = self.pane.upgrade() {
            pane.defer(cx, move |pane, model, cx| {
                pane.history_updated(model, cx);
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
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
        })
        .unwrap_or("This buffer");
    let path = truncate_and_remove_front(path, 80);
    format!("{path} contains unsaved edits. Do you want to save it?")
}

pub fn tab_details(items: &[Box<dyn ItemHandle>], cx: &AppContext) -> Vec<usize> {
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

pub fn render_item_indicator(
    item: Box<dyn ItemHandle>,
    window: &Window,
    cx: &AppContext,
) -> Option<Indicator> {
    maybe!({
        let indicator_color = match (item.has_conflict(cx), item.is_dirty(cx)) {
            (true, _) => Color::Warning,
            (_, true) => Color::Accent,
            (false, false) => return None,
        };

        Some(Indicator::dot().color(indicator_color))
    })
}

impl Render for DraggedTab {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();
        let label = self.item.tab_content(
            TabContentParams {
                detail: Some(self.detail),
                selected: false,
                preview: false,
            },
            window,
            cx,
        );
        Tab::new("")
            .selected(self.is_active)
            .child(label)
            .render(window, cx)
            .font(ui_font)
    }
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            assert!(pane
                .close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
                .is_none())
        });
    }

    #[gpui::test]
    async fn test_add_item_with_new_item(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, _, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   a. Add before the active item
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(
                    cx.new_model(|model, cx| TestItem::new(model, window, cx).with_label("D")),
                ),
                false,
                false,
                Some(0),
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   b. Add after the active item
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(
                    cx.new_model(|model, cx| TestItem::new(model, window, cx).with_label("D")),
                ),
                false,
                false,
                Some(2),
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   c. Add at the end of the item list (including off the length)
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(
                    cx.new_model(|model, cx| TestItem::new(model, window, cx).with_label("D")),
                ),
                false,
                false,
                Some(5),
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        // 2. Add without a destination index
        //   a. Add with active item at the start of the item list
        set_labeled_items(&pane, ["A*", "B", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(
                    cx.new_model(|model, cx| TestItem::new(model, window, cx).with_label("D")),
                ),
                false,
                false,
                None,
                model,
                window,
                cx,
            );
        });
        set_labeled_items(&pane, ["A", "D*", "B", "C"], cx);

        //   b. Add with active item at the end of the item list
        set_labeled_items(&pane, ["A", "B", "C*"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(
                    cx.new_model(|model, cx| TestItem::new(model, window, cx).with_label("D")),
                ),
                false,
                false,
                None,
                model,
                window,
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());
        let window = cx.handle();

        // 1. Add with a destination index
        //   1a. Add before the active item
        let [_, _, _, d] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(d, false, false, Some(0), model, window, cx);
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   1b. Add after the active item
        let [_, _, _, d] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(d, false, false, Some(2), model, window, cx);
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   1c. Add at the end of the item list (including off the length)
        let [a, _, _, _] = set_labeled_items(&pane, ["A", "B*", "C", "D"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(a, false, false, Some(5), model, window, cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   1d. Add same item to active index
        let [_, b, _] = set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(b, false, false, Some(1), model, window, cx);
        });
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        //   1e. Add item to index after same item in last position
        let [_, _, c] = set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(c, false, false, Some(2), model, window, cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        // 2. Add without a destination index
        //   2a. Add with active item at the start of the item list
        let [_, _, _, d] = set_labeled_items(&pane, ["A*", "B", "C", "D"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(d, false, false, None, model, window, cx);
        });
        assert_item_labels(&pane, ["A", "D*", "B", "C"], cx);

        //   2b. Add with active item at the end of the item list
        let [a, _, _, _] = set_labeled_items(&pane, ["A", "B", "C", "D*"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(a, false, false, None, model, window, cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   2c. Add active item to active item at end of list
        let [_, _, c] = set_labeled_items(&pane, ["A", "B", "C*"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(c, false, false, None, model, window, cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        //   2d. Add active item to active item at start of list
        let [a, _, _] = set_labeled_items(&pane, ["A*", "B", "C"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(a, false, false, None, model, window, cx);
        });
        assert_item_labels(&pane, ["A*", "B", "C"], cx);
    }

    #[gpui::test]
    async fn test_add_item_with_same_project_entries(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        // singleton view
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_singleton(true)
                        .with_label("buffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "one.txt", cx)])
                })),
                false,
                false,
                None,
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with the same project entry
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_singleton(true)
                        .with_label("buffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with different project entry
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_singleton(true)
                        .with_label("buffer 2")
                        .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)])
                })),
                false,
                false,
                None,
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2*"], cx);

        // new multibuffer view with the same project entry
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_singleton(false)
                        .with_label("multibuffer 1")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                model,
                window,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2", "multibuffer 1*"], cx);

        // another multibuffer view with the same project entry
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.add_item(
                Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_singleton(false)
                        .with_label("multibuffer 1b")
                        .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)])
                })),
                false,
                false,
                None,
                model,
                window,
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
    async fn test_remove_item_ordering_history(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());
        let window = cx.handle();

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        add_labeled_item(&pane, "D", false, cx);
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.activate_item(1, false, false, model, window, cx)
        });
        add_labeled_item(&pane, "1", false, cx);
        assert_item_labels(&pane, ["A", "B", "1*", "C", "D"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C", "D"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.activate_item(3, false, false, model, window, cx)
        });
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "C*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A*"], cx);
    }

    #[gpui::test]
    async fn test_remove_item_ordering_neighbour(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update_global::<SettingsStore, ()>(|s, cx| {
            s.update_user_settings::<ItemSettings>(cx, |s| {
                s.activate_on_close = Some(ActivateOnClose::Neighbour);
            });
        });
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());
        let window = cx.handle();

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        add_labeled_item(&pane, "D", false, cx);
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.activate_item(1, false, false, model, window, cx)
        });
        add_labeled_item(&pane, "1", false, cx);
        assert_item_labels(&pane, ["A", "B", "1*", "C", "D"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B", "C*", "D"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.activate_item(3, false, false, model, window, cx)
        });
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A", "B*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_active_item(&CloseActiveItem { save_intent: None }, model, window, cx)
        })
        .unwrap()
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_inactive_items(
                &CloseInactiveItems {
                    save_intent: None,
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, _, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", true, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", true, cx);
        add_labeled_item(&pane, "D", false, cx);
        add_labeled_item(&pane, "E", false, cx);
        assert_item_labels(&pane, ["A^", "B", "C^", "D", "E*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_clean_items(
                &CloseCleanItems {
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_items_to_the_left(
                &CloseItemsToTheLeft {
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);
        pane.update_in_window(window, cx, |pane, model, window, cx| {
            pane.close_items_to_the_right(
                &CloseItemsToTheRight {
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
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
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, model, _| workspace.active_pane().clone());

        let item_a = add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        pane.update_in_window(cx.handle(), cx, |pane, model, window, cx| {
            let ix = pane.index_for_item_id(item_a.item_id()).unwrap();
            pane.pin_tab_at(ix, model, window, cx);
            pane.close_all_items(
                &CloseAllItems {
                    save_intent: None,
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["A*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            let ix = pane.index_for_item_id(item_a.item_id()).unwrap();
            pane.unpin_tab_at(ix, model, window, cx);
            pane.close_all_items(
                &CloseAllItems {
                    save_intent: None,
                    close_pinned: false,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();

        assert_item_labels(&pane, [], cx);

        add_labeled_item(&pane, "A", true, cx).update(cx, |item, _model, cx| {
            item.project_items
                .push(TestProjectItem::new(1, "A.txt", cx))
        });
        add_labeled_item(&pane, "B", true, cx).update(cx, |item, _model, cx| {
            item.project_items
                .push(TestProjectItem::new(2, "B.txt", cx))
        });
        add_labeled_item(&pane, "C", true, cx).update(cx, |item, _model, cx| {
            item.project_items
                .push(TestProjectItem::new(3, "C.txt", cx))
        });
        assert_item_labels(&pane, ["A^", "B^", "C*^"], cx);

        let save = pane
            .update_in_window(window, cx, |pane, model, window, cx| {
                pane.close_all_items(
                    &CloseAllItems {
                        save_intent: None,
                        close_pinned: false,
                    },
                    model,
                    window,
                    cx,
                )
            })
            .unwrap()
            .unwrap();

        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2);
        save.await.unwrap();
        assert_item_labels(&pane, [], cx);

        add_labeled_item(&pane, "A", true, cx);
        add_labeled_item(&pane, "B", true, cx);
        add_labeled_item(&pane, "C", true, cx);
        assert_item_labels(&pane, ["A^", "B^", "C*^"], cx);
        let save = pane
            .update_in_window(window, cx, |pane, model, window, cx| {
                pane.close_all_items(
                    &CloseAllItems {
                        save_intent: None,
                        close_pinned: false,
                    },
                    model,
                    window,
                    cx,
                )
            })
            .unwrap()
            .unwrap();

        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2);
        cx.executor().run_until_parked();
        cx.simulate_prompt_answer(2);
        cx.executor().run_until_parked();
        save.await.unwrap();
        assert_item_labels(&pane, ["A*^", "B^", "C^"], cx);
    }

    #[gpui::test]
    async fn test_close_all_items_including_pinned(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs, None, cx).await;
        let (workspace, cx) = cx.add_window_view(|model, window, cx| {
            Workspace::test_new(project.clone(), model, window, cx)
        });
        let window = cx.handle();
        let pane = workspace.update(cx, |workspace, _, _| workspace.active_pane().clone());

        let item_a = add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        pane.update_in_window(window, cx, |pane, model, window, cx| {
            let ix = pane.index_for_item_id(item_a.item_id()).unwrap();
            pane.pin_tab_at(ix, model, window, cx);
            pane.close_all_items(
                &CloseAllItems {
                    save_intent: None,
                    close_pinned: true,
                },
                model,
                window,
                cx,
            )
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
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
        pane: &Model<Pane>,
        label: &str,
        is_dirty: bool,
        cx: &mut VisualTestContext,
    ) -> Box<Model<TestItem>> {
        cx.update(|window, cx| {
            pane.update_in_window(window, cx, |pane, model, window, cx| {
                let labeled_item = Box::new(cx.new_model(|model, cx| {
                    TestItem::new(model, window, cx)
                        .with_label(label)
                        .with_dirty(is_dirty)
                }));
                pane.add_item(labeled_item.clone(), false, false, None, model, window, cx);
                labeled_item
            })
        })
        .unwrap()
    }

    fn set_labeled_items<const COUNT: usize>(
        pane: &Model<Pane>,
        labels: [&str; COUNT],
        cx: &mut VisualTestContext,
    ) -> [Box<Model<TestItem>>; COUNT] {
        cx.update(|window, cx| {
            pane.update_in_window(window, cx, |pane, model, window, cx| {
                pane.items.clear();
                let mut active_item_index = 0;

                let mut index = 0;
                let items = labels.map(|mut label| {
                    if label.ends_with('*') {
                        label = label.trim_end_matches('*');
                        active_item_index = index;
                    }

                    let labeled_item =
                        Box::new(cx.new_model(|model, cx| {
                            TestItem::new(model, window, cx).with_label(label)
                        }));
                    pane.add_item(labeled_item.clone(), false, false, None, model, window, cx);
                    index += 1;
                    labeled_item
                });

                pane.activate_item(active_item_index, false, false, model, window, cx);

                items
            })
        })
        .unwrap()
    }

    // Assert the item label, with the active item label suffixed with a '*'
    #[track_caller]
    fn assert_item_labels<const COUNT: usize>(
        pane: &Model<Pane>,
        expected_states: [&str; COUNT],
        cx: &mut VisualTestContext,
    ) {
        let actual_states = pane.update(cx, |pane, model, cx| {
            pane.items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let mut state = item
                        .model()
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
                .collect::<Vec<_>>()
        });
        assert_eq!(
            actual_states, expected_states,
            "pane items do not match expectation"
        );
    }
}
