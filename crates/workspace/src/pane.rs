mod dragged_item_receiver;

use super::{ItemHandle, SplitDirection};
pub use crate::toolbar::Toolbar;
use crate::{
    item::{ItemSettings, WeakItemHandle},
    notify_of_new_dock, AutosaveSetting, Item, NewCenterTerminal, NewFile, NewSearch, ToggleZoom,
    Workspace, WorkspaceSettings,
};
use anyhow::Result;
use collections::{HashMap, HashSet, VecDeque};
use context_menu::{ContextMenu, ContextMenuItem};
use drag_and_drop::{DragAndDrop, Draggable};
use dragged_item_receiver::dragged_item_receiver;
use fs::repository::GitFileStatus;
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    impl_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton, NavigationDirection, PromptLevel},
    Action, AnyViewHandle, AnyWeakViewHandle, AppContext, AsyncAppContext, Entity, EventContext,
    LayoutContext, ModelHandle, MouseRegion, Quad, Task, View, ViewContext, ViewHandle,
    WeakViewHandle, WindowContext,
};
use project::{Project, ProjectEntryId, ProjectPath};
use serde::Deserialize;
use std::{
    any::Any,
    cell::RefCell,
    cmp, mem,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use theme::{Theme, ThemeSettings};

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivateItem(pub usize);

#[derive(Clone, PartialEq)]
pub struct CloseItemById {
    pub item_id: usize,
    pub pane: WeakViewHandle<Pane>,
}

#[derive(Clone, PartialEq)]
pub struct CloseItemsToTheLeftById {
    pub item_id: usize,
    pub pane: WeakViewHandle<Pane>,
}

#[derive(Clone, PartialEq)]
pub struct CloseItemsToTheRightById {
    pub item_id: usize,
    pub pane: WeakViewHandle<Pane>,
}

actions!(
    pane,
    [
        ActivatePrevItem,
        ActivateNextItem,
        ActivateLastItem,
        CloseActiveItem,
        CloseInactiveItems,
        CloseCleanItems,
        CloseItemsToTheLeft,
        CloseItemsToTheRight,
        CloseAllItems,
        GoBack,
        GoForward,
        ReopenClosedItem,
        SplitLeft,
        SplitUp,
        SplitRight,
        SplitDown,
    ]
);

impl_actions!(pane, [ActivateItem]);

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub type BackgroundActions = fn() -> &'static [(&'static str, &'static dyn Action)];

pub fn init(cx: &mut AppContext) {
    cx.add_action(Pane::toggle_zoom);
    cx.add_action(|pane: &mut Pane, action: &ActivateItem, cx| {
        pane.activate_item(action.0, true, true, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivateLastItem, cx| {
        pane.activate_item(pane.items.len() - 1, true, true, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivatePrevItem, cx| {
        pane.activate_prev_item(true, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivateNextItem, cx| {
        pane.activate_next_item(true, cx);
    });
    cx.add_async_action(Pane::close_active_item);
    cx.add_async_action(Pane::close_inactive_items);
    cx.add_async_action(Pane::close_clean_items);
    cx.add_async_action(Pane::close_items_to_the_left);
    cx.add_async_action(Pane::close_items_to_the_right);
    cx.add_async_action(Pane::close_all_items);
    cx.add_action(|pane: &mut Pane, _: &SplitLeft, cx| pane.split(SplitDirection::Left, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitUp, cx| pane.split(SplitDirection::Up, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitRight, cx| pane.split(SplitDirection::Right, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitDown, cx| pane.split(SplitDirection::Down, cx));
}

#[derive(Debug)]
pub enum Event {
    AddItem { item: Box<dyn ItemHandle> },
    ActivateItem { local: bool },
    Remove,
    RemoveItem { item_id: usize },
    Split(SplitDirection),
    ChangeItemTitle,
    Focus,
    ZoomIn,
    ZoomOut,
}

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    activation_history: Vec<usize>,
    zoomed: bool,
    active_item_index: usize,
    last_focused_view_by_item: HashMap<usize, AnyWeakViewHandle>,
    autoscroll: bool,
    nav_history: NavHistory,
    toolbar: ViewHandle<Toolbar>,
    tab_bar_context_menu: TabBarContextMenu,
    tab_context_menu: ViewHandle<ContextMenu>,
    _background_actions: BackgroundActions,
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
    has_focus: bool,
    can_drop: Rc<dyn Fn(&DragAndDrop<Workspace>, &WindowContext) -> bool>,
    can_split: bool,
    render_tab_bar_buttons: Rc<dyn Fn(&mut Pane, &mut ViewContext<Pane>) -> AnyElement<Pane>>,
}

pub struct ItemNavHistory {
    history: NavHistory,
    item: Rc<dyn WeakItemHandle>,
}

#[derive(Clone)]
pub struct NavHistory(Rc<RefCell<NavHistoryState>>);

struct NavHistoryState {
    mode: NavigationMode,
    backward_stack: VecDeque<NavigationEntry>,
    forward_stack: VecDeque<NavigationEntry>,
    closed_stack: VecDeque<NavigationEntry>,
    paths_by_item: HashMap<usize, (ProjectPath, Option<PathBuf>)>,
    pane: WeakViewHandle<Pane>,
    next_timestamp: Arc<AtomicUsize>,
}

#[derive(Copy, Clone)]
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
    pub item: Rc<dyn WeakItemHandle>,
    pub data: Option<Box<dyn Any>>,
    pub timestamp: usize,
}

pub struct DraggedItem {
    pub handle: Box<dyn ItemHandle>,
    pub pane: WeakViewHandle<Pane>,
}

pub enum ReorderBehavior {
    None,
    MoveAfterActive,
    MoveToIndex(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabBarContextMenuKind {
    New,
    Split,
}

struct TabBarContextMenu {
    kind: TabBarContextMenuKind,
    handle: ViewHandle<ContextMenu>,
}

impl TabBarContextMenu {
    fn handle_if_kind(&self, kind: TabBarContextMenuKind) -> Option<ViewHandle<ContextMenu>> {
        if self.kind == kind {
            return Some(self.handle.clone());
        }
        None
    }
}

impl Pane {
    pub fn new(
        workspace: WeakViewHandle<Workspace>,
        project: ModelHandle<Project>,
        background_actions: BackgroundActions,
        next_timestamp: Arc<AtomicUsize>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let pane_view_id = cx.view_id();
        let handle = cx.weak_handle();
        let context_menu = cx.add_view(|cx| ContextMenu::new(pane_view_id, cx));
        context_menu.update(cx, |menu, _| {
            menu.set_position_mode(OverlayPositionMode::Local)
        });

        Self {
            items: Vec::new(),
            activation_history: Vec::new(),
            zoomed: false,
            active_item_index: 0,
            last_focused_view_by_item: Default::default(),
            autoscroll: false,
            nav_history: NavHistory(Rc::new(RefCell::new(NavHistoryState {
                mode: NavigationMode::Normal,
                backward_stack: Default::default(),
                forward_stack: Default::default(),
                closed_stack: Default::default(),
                paths_by_item: Default::default(),
                pane: handle.clone(),
                next_timestamp,
            }))),
            toolbar: cx.add_view(|_| Toolbar::new(Some(handle))),
            tab_bar_context_menu: TabBarContextMenu {
                kind: TabBarContextMenuKind::New,
                handle: context_menu,
            },
            tab_context_menu: cx.add_view(|cx| ContextMenu::new(pane_view_id, cx)),
            _background_actions: background_actions,
            workspace,
            project,
            has_focus: false,
            can_drop: Rc::new(|_, _| true),
            can_split: true,
            render_tab_bar_buttons: Rc::new(|pane, cx| {
                Flex::row()
                    // New menu
                    .with_child(Self::render_tab_bar_button(
                        0,
                        "icons/plus_12.svg",
                        false,
                        Some(("New...".into(), None)),
                        cx,
                        |pane, cx| pane.deploy_new_menu(cx),
                        |pane, cx| {
                            pane.tab_bar_context_menu
                                .handle
                                .update(cx, |menu, _| menu.delay_cancel())
                        },
                        pane.tab_bar_context_menu
                            .handle_if_kind(TabBarContextMenuKind::New),
                    ))
                    .with_child(Self::render_tab_bar_button(
                        1,
                        "icons/split_12.svg",
                        false,
                        Some(("Split Pane".into(), None)),
                        cx,
                        |pane, cx| pane.deploy_split_menu(cx),
                        |pane, cx| {
                            pane.tab_bar_context_menu
                                .handle
                                .update(cx, |menu, _| menu.delay_cancel())
                        },
                        pane.tab_bar_context_menu
                            .handle_if_kind(TabBarContextMenuKind::Split),
                    ))
                    .with_child({
                        let icon_path;
                        let tooltip_label;
                        if pane.is_zoomed() {
                            icon_path = "icons/minimize_8.svg";
                            tooltip_label = "Zoom In".into();
                        } else {
                            icon_path = "icons/maximize_8.svg";
                            tooltip_label = "Zoom In".into();
                        }

                        Pane::render_tab_bar_button(
                            2,
                            icon_path,
                            pane.is_zoomed(),
                            Some((tooltip_label, Some(Box::new(ToggleZoom)))),
                            cx,
                            move |pane, cx| pane.toggle_zoom(&Default::default(), cx),
                            move |_, _| {},
                            None,
                        )
                    })
                    .into_any()
            }),
        }
    }

    pub(crate) fn workspace(&self) -> &WeakViewHandle<Workspace> {
        &self.workspace
    }

    pub fn has_focus(&self) -> bool {
        self.has_focus
    }

    pub fn active_item_index(&self) -> usize {
        self.active_item_index
    }

    pub fn on_can_drop<F>(&mut self, can_drop: F)
    where
        F: 'static + Fn(&DragAndDrop<Workspace>, &WindowContext) -> bool,
    {
        self.can_drop = Rc::new(can_drop);
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
        F: 'static + Fn(&mut Pane, &mut ViewContext<Pane>) -> AnyElement<Pane>,
    {
        self.render_tab_bar_buttons = Rc::new(render);
        cx.notify();
    }

    pub fn nav_history_for_item<T: Item>(&self, item: &ViewHandle<T>) -> ItemNavHistory {
        ItemNavHistory {
            history: self.nav_history.clone(),
            item: Rc::new(item.downgrade()),
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
        !self.nav_history.0.borrow().backward_stack.is_empty()
    }

    pub fn can_navigate_forward(&self) -> bool {
        !self.nav_history.0.borrow().forward_stack.is_empty()
    }

    fn history_updated(&mut self, cx: &mut ViewContext<Self>) {
        self.toolbar.update(cx, |_, cx| cx.notify());
    }

    pub(crate) fn open_item(
        &mut self,
        project_entry_id: ProjectEntryId,
        focus_item: bool,
        cx: &mut ViewContext<Self>,
        build_item: impl FnOnce(&mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
    ) -> Box<dyn ItemHandle> {
        let mut existing_item = None;
        for (index, item) in self.items.iter().enumerate() {
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == [project_entry_id]
            {
                let item = item.boxed_clone();
                existing_item = Some((index, item));
                break;
            }
        }

        if let Some((index, existing_item)) = existing_item {
            self.activate_item(index, focus_item, focus_item, cx);
            existing_item
        } else {
            let new_item = build_item(cx);
            self.add_item(new_item.clone(), true, focus_item, None, cx);
            new_item
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
                        .borrow_mut()
                        .paths_by_item
                        .insert(item.id(), (project_path, abs_path));
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
            if existing_item.id() == item.id() {
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
            if insertion_index <= self.active_item_index {
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

    pub fn items(&self) -> impl Iterator<Item = &Box<dyn ItemHandle>> + DoubleEndedIterator {
        self.items.iter()
    }

    pub fn items_of_type<T: View>(&self) -> impl '_ + Iterator<Item = ViewHandle<T>> {
        self.items
            .iter()
            .filter_map(|item| item.as_any().clone().downcast())
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.items.get(self.active_item_index).cloned()
    }

    pub fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Vector2F> {
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
        self.items.iter().position(|i| i.id() == item.id())
    }

    pub fn toggle_zoom(&mut self, _: &ToggleZoom, cx: &mut ViewContext<Self>) {
        // Potentially warn the user of the new keybinding
        let workspace_handle = self.workspace().clone();
        cx.spawn(|_, mut cx| async move { notify_of_new_dock(&workspace_handle, &mut cx) })
            .detach();

        if self.zoomed {
            cx.emit(Event::ZoomOut);
        } else if !self.items.is_empty() {
            if !self.has_focus {
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

                cx.emit(Event::ActivateItem {
                    local: activate_pane,
                });
            }

            if let Some(newly_active_item) = self.items.get(index) {
                self.activation_history
                    .retain(|&previously_active_item_id| {
                        previously_active_item_id != newly_active_item.id()
                    });
                self.activation_history.push(newly_active_item.id());
            }

            self.update_toolbar(cx);

            if focus_item {
                self.focus_active_item(cx);
            }

            self.autoscroll = true;
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
        _: &CloseActiveItem,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].id();
        Some(self.close_item_by_id(active_item_id, cx))
    }

    pub fn close_item_by_id(
        &mut self,
        item_id_to_close: usize,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.close_items(cx, move |view_id| view_id == item_id_to_close)
    }

    pub fn close_inactive_items(
        &mut self,
        _: &CloseInactiveItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        let active_item_id = self.items[self.active_item_index].id();
        Some(self.close_items(cx, move |item_id| item_id != active_item_id))
    }

    pub fn close_clean_items(
        &mut self,
        _: &CloseCleanItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let item_ids: Vec<_> = self
            .items()
            .filter(|item| !item.is_dirty(cx))
            .map(|item| item.id())
            .collect();
        Some(self.close_items(cx, move |item_id| item_ids.contains(&item_id)))
    }

    pub fn close_items_to_the_left(
        &mut self,
        _: &CloseItemsToTheLeft,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].id();
        Some(self.close_items_to_the_left_by_id(active_item_id, cx))
    }

    pub fn close_items_to_the_left_by_id(
        &mut self,
        item_id: usize,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .take_while(|item| item.id() != item_id)
            .map(|item| item.id())
            .collect();
        self.close_items(cx, move |item_id| item_ids.contains(&item_id))
    }

    pub fn close_items_to_the_right(
        &mut self,
        _: &CloseItemsToTheRight,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }
        let active_item_id = self.items[self.active_item_index].id();
        Some(self.close_items_to_the_right_by_id(active_item_id, cx))
    }

    pub fn close_items_to_the_right_by_id(
        &mut self,
        item_id: usize,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let item_ids: Vec<_> = self
            .items()
            .rev()
            .take_while(|item| item.id() != item_id)
            .map(|item| item.id())
            .collect();
        self.close_items(cx, move |item_id| item_ids.contains(&item_id))
    }

    pub fn close_all_items(
        &mut self,
        _: &CloseAllItems,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.items.is_empty() {
            return None;
        }

        Some(self.close_items(cx, move |_| true))
    }

    pub fn close_items(
        &mut self,
        cx: &mut ViewContext<Pane>,
        should_close: impl 'static + Fn(usize) -> bool,
    ) -> Task<Result<()>> {
        // Find the items to close.
        let mut items_to_close = Vec::new();
        for item in &self.items {
            if should_close(item.id()) {
                items_to_close.push(item.boxed_clone());
            }
        }

        // If a buffer is open both in a singleton editor and in a multibuffer, make sure
        // to focus the singleton buffer when prompting to save that buffer, as opposed
        // to focusing the multibuffer, because this gives the user a more clear idea
        // of what content they would be saving.
        items_to_close.sort_by_key(|item| !item.is_singleton(cx));

        let workspace = self.workspace.clone();
        cx.spawn(|pane, mut cx| async move {
            let mut saved_project_items_ids = HashSet::default();
            for item in items_to_close.clone() {
                // Find the item's current index and its set of project item models. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let (item_ix, mut project_item_ids) = pane.read_with(&cx, |pane, cx| {
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
                let project = workspace.read_with(&cx, |workspace, cx| {
                    for item in workspace.items(cx) {
                        if !items_to_close
                            .iter()
                            .any(|item_to_close| item_to_close.id() == item.id())
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
                    && !Self::save_item(project.clone(), &pane, item_ix, &*item, true, &mut cx)
                        .await?
                {
                    break;
                }

                // Remove the item from the pane.
                pane.update(&mut cx, |pane, cx| {
                    if let Some(item_ix) = pane.items.iter().position(|i| i.id() == item.id()) {
                        pane.remove_item(item_ix, false, cx);
                    }
                })?;
            }

            pane.update(&mut cx, |_, cx| cx.notify())?;
            Ok(())
        })
    }

    pub fn remove_item(
        &mut self,
        item_index: usize,
        activate_pane: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.activation_history
            .retain(|&history_entry| history_entry != self.items[item_index].id());

        if item_index == self.active_item_index {
            let index_to_activate = self
                .activation_history
                .pop()
                .and_then(|last_activated_item| {
                    self.items.iter().enumerate().find_map(|(index, item)| {
                        (item.id() == last_activated_item).then_some(index)
                    })
                })
                // We didn't have a valid activation history entry, so fallback
                // to activating the item to the left
                .unwrap_or_else(|| item_index.min(self.items.len()).saturating_sub(1));

            let should_activate = activate_pane || self.has_focus;
            self.activate_item(index_to_activate, should_activate, should_activate, cx);
        }

        let item = self.items.remove(item_index);

        cx.emit(Event::RemoveItem { item_id: item.id() });
        if self.items.is_empty() {
            item.deactivated(cx);
            self.update_toolbar(cx);
            cx.emit(Event::Remove);
        }

        if item_index < self.active_item_index {
            self.active_item_index -= 1;
        }

        self.nav_history.set_mode(NavigationMode::ClosingItem);
        item.deactivated(cx);
        self.nav_history.set_mode(NavigationMode::Normal);

        if let Some(path) = item.project_path(cx) {
            let abs_path = self
                .nav_history
                .0
                .borrow()
                .paths_by_item
                .get(&item.id())
                .and_then(|(_, abs_path)| abs_path.clone());

            self.nav_history
                .0
                .borrow_mut()
                .paths_by_item
                .insert(item.id(), (path, abs_path));
        } else {
            self.nav_history
                .0
                .borrow_mut()
                .paths_by_item
                .remove(&item.id());
        }

        if self.items.is_empty() && self.zoomed {
            cx.emit(Event::ZoomOut);
        }

        cx.notify();
    }

    pub async fn save_item(
        project: ModelHandle<Project>,
        pane: &WeakViewHandle<Pane>,
        item_ix: usize,
        item: &dyn ItemHandle,
        should_prompt_for_save: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<bool> {
        const CONFLICT_MESSAGE: &str =
            "This file has changed on disk since you started editing it. Do you want to overwrite it?";
        const DIRTY_MESSAGE: &str = "This file contains unsaved edits. Do you want to save it?";

        let (has_conflict, is_dirty, can_save, is_singleton) = cx.read(|cx| {
            (
                item.has_conflict(cx),
                item.is_dirty(cx),
                item.can_save(cx),
                item.is_singleton(cx),
            )
        });

        if has_conflict && can_save {
            let mut answer = pane.update(cx, |pane, cx| {
                pane.activate_item(item_ix, true, true, cx);
                cx.prompt(
                    PromptLevel::Warning,
                    CONFLICT_MESSAGE,
                    &["Overwrite", "Discard", "Cancel"],
                )
            })?;
            match answer.next().await {
                Some(0) => pane.update(cx, |_, cx| item.save(project, cx))?.await?,
                Some(1) => pane.update(cx, |_, cx| item.reload(project, cx))?.await?,
                _ => return Ok(false),
            }
        } else if is_dirty && (can_save || is_singleton) {
            let will_autosave = cx.read(|cx| {
                matches!(
                    settings::get::<WorkspaceSettings>(cx).autosave,
                    AutosaveSetting::OnFocusChange | AutosaveSetting::OnWindowChange
                ) && Self::can_autosave_item(&*item, cx)
            });
            let should_save = if should_prompt_for_save && !will_autosave {
                let mut answer = pane.update(cx, |pane, cx| {
                    pane.activate_item(item_ix, true, true, cx);
                    cx.prompt(
                        PromptLevel::Warning,
                        DIRTY_MESSAGE,
                        &["Save", "Don't Save", "Cancel"],
                    )
                })?;
                match answer.next().await {
                    Some(0) => true,
                    Some(1) => false,
                    _ => return Ok(false),
                }
            } else {
                true
            };

            if should_save {
                if can_save {
                    pane.update(cx, |_, cx| item.save(project, cx))?.await?;
                } else if is_singleton {
                    let start_abs_path = project
                        .read_with(cx, |project, cx| {
                            let worktree = project.visible_worktrees(cx).next()?;
                            Some(worktree.read(cx).as_local()?.abs_path().to_path_buf())
                        })
                        .unwrap_or_else(|| Path::new("").into());

                    let mut abs_path = cx.update(|cx| cx.prompt_for_new_path(&start_abs_path));
                    if let Some(abs_path) = abs_path.next().await.flatten() {
                        pane.update(cx, |_, cx| item.save_as(project, abs_path, cx))?
                            .await?;
                    } else {
                        return Ok(false);
                    }
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
        project: ModelHandle<Project>,
        cx: &mut WindowContext,
    ) -> Task<Result<()>> {
        if Self::can_autosave_item(item, cx) {
            item.save(project, cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            cx.focus(active_item.as_any());
        }
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
    }

    fn deploy_split_menu(&mut self, cx: &mut ViewContext<Self>) {
        self.tab_bar_context_menu.handle.update(cx, |menu, cx| {
            menu.toggle(
                Default::default(),
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::action("Split Right", SplitRight),
                    ContextMenuItem::action("Split Left", SplitLeft),
                    ContextMenuItem::action("Split Up", SplitUp),
                    ContextMenuItem::action("Split Down", SplitDown),
                ],
                cx,
            );
        });

        self.tab_bar_context_menu.kind = TabBarContextMenuKind::Split;
    }

    fn deploy_new_menu(&mut self, cx: &mut ViewContext<Self>) {
        self.tab_bar_context_menu.handle.update(cx, |menu, cx| {
            menu.toggle(
                Default::default(),
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::action("New File", NewFile),
                    ContextMenuItem::action("New Terminal", NewCenterTerminal),
                    ContextMenuItem::action("New Search", NewSearch),
                ],
                cx,
            );
        });

        self.tab_bar_context_menu.kind = TabBarContextMenuKind::New;
    }

    fn deploy_tab_context_menu(
        &mut self,
        position: Vector2F,
        target_item_id: usize,
        cx: &mut ViewContext<Self>,
    ) {
        let active_item_id = self.items[self.active_item_index].id();
        let is_active_item = target_item_id == active_item_id;
        let target_pane = cx.weak_handle();

        // The `CloseInactiveItems` action should really be called "CloseOthers" and the behaviour should be dynamically based on the tab the action is ran on.  Currently, this is a weird action because you can run it on a non-active tab and it will close everything by the actual active tab

        self.tab_context_menu.update(cx, |menu, cx| {
            menu.show(
                position,
                AnchorCorner::TopLeft,
                if is_active_item {
                    vec![
                        ContextMenuItem::action("Close Active Item", CloseActiveItem),
                        ContextMenuItem::action("Close Inactive Items", CloseInactiveItems),
                        ContextMenuItem::action("Close Clean Items", CloseCleanItems),
                        ContextMenuItem::action("Close Items To The Left", CloseItemsToTheLeft),
                        ContextMenuItem::action("Close Items To The Right", CloseItemsToTheRight),
                        ContextMenuItem::action("Close All Items", CloseAllItems),
                    ]
                } else {
                    // In the case of the user right clicking on a non-active tab, for some item-closing commands, we need to provide the id of the tab, for the others, we can reuse the existing command.
                    vec![
                        ContextMenuItem::handler("Close Inactive Item", {
                            let pane = target_pane.clone();
                            move |cx| {
                                if let Some(pane) = pane.upgrade(cx) {
                                    pane.update(cx, |pane, cx| {
                                        pane.close_item_by_id(target_item_id, cx)
                                            .detach_and_log_err(cx);
                                    })
                                }
                            }
                        }),
                        ContextMenuItem::action("Close Inactive Items", CloseInactiveItems),
                        ContextMenuItem::action("Close Clean Items", CloseCleanItems),
                        ContextMenuItem::handler("Close Items To The Left", {
                            let pane = target_pane.clone();
                            move |cx| {
                                if let Some(pane) = pane.upgrade(cx) {
                                    pane.update(cx, |pane, cx| {
                                        pane.close_items_to_the_left_by_id(target_item_id, cx)
                                            .detach_and_log_err(cx);
                                    })
                                }
                            }
                        }),
                        ContextMenuItem::handler("Close Items To The Right", {
                            let pane = target_pane.clone();
                            move |cx| {
                                if let Some(pane) = pane.upgrade(cx) {
                                    pane.update(cx, |pane, cx| {
                                        pane.close_items_to_the_right_by_id(target_item_id, cx)
                                            .detach_and_log_err(cx);
                                    })
                                }
                            }
                        }),
                        ContextMenuItem::action("Close All Items", CloseAllItems),
                    ]
                },
                cx,
            );
        });
    }

    pub fn toolbar(&self) -> &ViewHandle<Toolbar> {
        &self.toolbar
    }

    pub fn handle_deleted_project_item(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Pane>,
    ) -> Option<()> {
        let (item_index_to_delete, item_id) = self.items().enumerate().find_map(|(i, item)| {
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == [entry_id] {
                Some((i, item.id()))
            } else {
                None
            }
        })?;

        self.remove_item(item_index_to_delete, false, cx);
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

    fn render_tabs(&mut self, cx: &mut ViewContext<Self>) -> impl Element<Self> {
        let theme = theme::current(cx).clone();

        let pane = cx.handle().downgrade();
        let autoscroll = if mem::take(&mut self.autoscroll) {
            Some(self.active_item_index)
        } else {
            None
        };

        let pane_active = self.has_focus;

        enum Tabs {}
        let mut row = Flex::row().scrollable::<Tabs>(1, autoscroll, cx);
        for (ix, (item, detail)) in self
            .items
            .iter()
            .cloned()
            .zip(self.tab_details(cx))
            .enumerate()
        {
            let git_status = item
                .project_path(cx)
                .and_then(|path| self.project.read(cx).entry_for_path(&path, cx))
                .and_then(|entry| entry.git_status());

            let detail = if detail == 0 { None } else { Some(detail) };
            let tab_active = ix == self.active_item_index;

            row.add_child({
                enum TabDragReceiver {}
                let mut receiver =
                    dragged_item_receiver::<TabDragReceiver, _, _>(self, ix, ix, true, None, cx, {
                        let item = item.clone();
                        let pane = pane.clone();
                        let detail = detail.clone();

                        let theme = theme::current(cx).clone();
                        let mut tooltip_theme = theme.tooltip.clone();
                        tooltip_theme.max_text_width = None;
                        let tab_tooltip_text =
                            item.tab_tooltip_text(cx).map(|text| text.into_owned());

                        let mut tab_style = theme
                            .workspace
                            .tab_bar
                            .tab_style(pane_active, tab_active)
                            .clone();
                        let should_show_status = settings::get::<ItemSettings>(cx).git_status;
                        if should_show_status && git_status != None {
                            tab_style.label.text.color = match git_status.unwrap() {
                                GitFileStatus::Added => tab_style.git.inserted,
                                GitFileStatus::Modified => tab_style.git.modified,
                                GitFileStatus::Conflict => tab_style.git.conflict,
                            };
                        }

                        move |mouse_state, cx| {
                            let hovered = mouse_state.hovered();

                            enum Tab {}
                            let mouse_event_handler =
                                MouseEventHandler::<Tab, Pane>::new(ix, cx, |_, cx| {
                                    Self::render_tab(
                                        &item,
                                        pane.clone(),
                                        ix == 0,
                                        detail,
                                        hovered,
                                        &tab_style,
                                        cx,
                                    )
                                })
                                .on_down(MouseButton::Left, move |_, this, cx| {
                                    this.activate_item(ix, true, true, cx);
                                })
                                .on_click(MouseButton::Middle, {
                                    let item_id = item.id();
                                    move |_, pane, cx| {
                                        pane.close_item_by_id(item_id, cx).detach_and_log_err(cx);
                                    }
                                })
                                .on_down(
                                    MouseButton::Right,
                                    move |event, pane, cx| {
                                        pane.deploy_tab_context_menu(event.position, item.id(), cx);
                                    },
                                );

                            if let Some(tab_tooltip_text) = tab_tooltip_text {
                                mouse_event_handler
                                    .with_tooltip::<Self>(
                                        ix,
                                        tab_tooltip_text,
                                        None,
                                        tooltip_theme,
                                        cx,
                                    )
                                    .into_any()
                            } else {
                                mouse_event_handler.into_any()
                            }
                        }
                    });

                if !pane_active || !tab_active {
                    receiver = receiver.with_cursor_style(CursorStyle::PointingHand);
                }

                receiver.as_draggable(
                    DraggedItem {
                        handle: item,
                        pane: pane.clone(),
                    },
                    {
                        let theme = theme::current(cx).clone();

                        let detail = detail.clone();
                        move |dragged_item: &DraggedItem, cx: &mut ViewContext<Workspace>| {
                            let tab_style = &theme.workspace.tab_bar.dragged_tab;
                            Self::render_dragged_tab(
                                &dragged_item.handle,
                                dragged_item.pane.clone(),
                                false,
                                detail,
                                false,
                                &tab_style,
                                cx,
                            )
                        }
                    },
                )
            })
        }

        // Use the inactive tab style along with the current pane's active status to decide how to render
        // the filler
        let filler_index = self.items.len();
        let filler_style = theme.workspace.tab_bar.tab_style(pane_active, false);
        enum Filler {}
        row.add_child(
            dragged_item_receiver::<Filler, _, _>(self, 0, filler_index, true, None, cx, |_, _| {
                Empty::new()
                    .contained()
                    .with_style(filler_style.container)
                    .with_border(filler_style.container.border)
            })
            .flex(1., true)
            .into_any_named("filler"),
        );

        row
    }

    fn tab_details(&self, cx: &AppContext) -> Vec<usize> {
        let mut tab_details = (0..self.items.len()).map(|_| 0).collect::<Vec<_>>();

        let mut tab_descriptions = HashMap::default();
        let mut done = false;
        while !done {
            done = true;

            // Store item indices by their tab description.
            for (ix, (item, detail)) in self.items.iter().zip(&tab_details).enumerate() {
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

    fn render_tab(
        item: &Box<dyn ItemHandle>,
        pane: WeakViewHandle<Pane>,
        first: bool,
        detail: Option<usize>,
        hovered: bool,
        tab_style: &theme::Tab,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let title = item.tab_content(detail, &tab_style, cx);
        Self::render_tab_with_title(title, item, pane, first, hovered, tab_style, cx)
    }

    fn render_dragged_tab(
        item: &Box<dyn ItemHandle>,
        pane: WeakViewHandle<Pane>,
        first: bool,
        detail: Option<usize>,
        hovered: bool,
        tab_style: &theme::Tab,
        cx: &mut ViewContext<Workspace>,
    ) -> AnyElement<Workspace> {
        let title = item.dragged_tab_content(detail, &tab_style, cx);
        Self::render_tab_with_title(title, item, pane, first, hovered, tab_style, cx)
    }

    fn render_tab_with_title<T: View>(
        title: AnyElement<T>,
        item: &Box<dyn ItemHandle>,
        pane: WeakViewHandle<Pane>,
        first: bool,
        hovered: bool,
        tab_style: &theme::Tab,
        cx: &mut ViewContext<T>,
    ) -> AnyElement<T> {
        let mut container = tab_style.container.clone();
        if first {
            container.border.left = false;
        }

        let buffer_jewel_element = {
            let diameter = 7.0;
            let icon_color = if item.has_conflict(cx) {
                Some(tab_style.icon_conflict)
            } else if item.is_dirty(cx) {
                Some(tab_style.icon_dirty)
            } else {
                None
            };

            Canvas::new(move |scene, bounds, _, _, _| {
                if let Some(color) = icon_color {
                    let square = RectF::new(bounds.origin(), vec2f(diameter, diameter));
                    scene.push_quad(Quad {
                        bounds: square,
                        background: Some(color),
                        border: Default::default(),
                        corner_radius: diameter / 2.,
                    });
                }
            })
            .constrained()
            .with_width(diameter)
            .with_height(diameter)
            .aligned()
        };

        let title_element = title.aligned().contained().with_style(ContainerStyle {
            margin: Margin {
                left: tab_style.spacing,
                right: tab_style.spacing,
                ..Default::default()
            },
            ..Default::default()
        });

        let close_element = if hovered {
            let item_id = item.id();
            enum TabCloseButton {}
            let icon = Svg::new("icons/x_mark_8.svg");
            MouseEventHandler::<TabCloseButton, _>::new(item_id, cx, |mouse_state, _| {
                if mouse_state.hovered() {
                    icon.with_color(tab_style.icon_close_active)
                } else {
                    icon.with_color(tab_style.icon_close)
                }
            })
            .with_padding(Padding::uniform(4.))
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, {
                let pane = pane.clone();
                move |_, _, cx| {
                    let pane = pane.clone();
                    cx.window_context().defer(move |cx| {
                        if let Some(pane) = pane.upgrade(cx) {
                            pane.update(cx, |pane, cx| {
                                pane.close_item_by_id(item_id, cx).detach_and_log_err(cx);
                            });
                        }
                    });
                }
            })
            .into_any_named("close-tab-icon")
            .constrained()
        } else {
            Empty::new().constrained()
        }
        .with_width(tab_style.close_icon_width)
        .aligned();

        let close_right = settings::get::<ItemSettings>(cx).close_position.right();

        if close_right {
            Flex::row()
                .with_child(buffer_jewel_element)
                .with_child(title_element)
                .with_child(close_element)
        } else {
            Flex::row()
                .with_child(close_element)
                .with_child(title_element)
                .with_child(buffer_jewel_element)
        }
        .contained()
        .with_style(container)
        .constrained()
        .with_height(tab_style.height)
        .into_any()
    }

    pub fn render_tab_bar_button<
        F1: 'static + Fn(&mut Pane, &mut EventContext<Pane>),
        F2: 'static + Fn(&mut Pane, &mut EventContext<Pane>),
    >(
        index: usize,
        icon: &'static str,
        is_active: bool,
        tooltip: Option<(String, Option<Box<dyn Action>>)>,
        cx: &mut ViewContext<Pane>,
        on_click: F1,
        on_down: F2,
        context_menu: Option<ViewHandle<ContextMenu>>,
    ) -> AnyElement<Pane> {
        enum TabBarButton {}

        let mut button = MouseEventHandler::<TabBarButton, _>::new(index, cx, |mouse_state, cx| {
            let theme = &settings::get::<ThemeSettings>(cx).theme.workspace.tab_bar;
            let style = theme.pane_button.in_state(is_active).style_for(mouse_state);
            Svg::new(icon)
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .with_height(style.button_width)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_down(MouseButton::Left, move |_, pane, cx| on_down(pane, cx))
        .on_click(MouseButton::Left, move |_, pane, cx| on_click(pane, cx))
        .into_any();
        if let Some((tooltip, action)) = tooltip {
            let tooltip_style = settings::get::<ThemeSettings>(cx).theme.tooltip.clone();
            button = button
                .with_tooltip::<TabBarButton>(index, tooltip, action, tooltip_style, cx)
                .into_any();
        }

        Stack::new()
            .with_child(button)
            .with_children(
                context_menu.map(|menu| ChildView::new(&menu, cx).aligned().bottom().right()),
            )
            .flex(1., false)
            .into_any_named("tab bar button")
    }

    fn render_blank_pane(&self, theme: &Theme, _cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let background = theme.workspace.background;
        Empty::new()
            .contained()
            .with_background_color(background)
            .into_any()
    }

    pub fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed
    }
}

impl Entity for Pane {
    type Event = Event;
}

impl View for Pane {
    fn ui_name() -> &'static str {
        "Pane"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum MouseNavigationHandler {}

        MouseEventHandler::<MouseNavigationHandler, _>::new(0, cx, |_, cx| {
            let active_item_index = self.active_item_index;

            if let Some(active_item) = self.active_item() {
                Flex::column()
                    .with_child({
                        let theme = theme::current(cx).clone();

                        let mut stack = Stack::new();

                        enum TabBarEventHandler {}
                        stack.add_child(
                            MouseEventHandler::<TabBarEventHandler, _>::new(0, cx, |_, _| {
                                Empty::new()
                                    .contained()
                                    .with_style(theme.workspace.tab_bar.container)
                            })
                            .on_down(
                                MouseButton::Left,
                                move |_, this, cx| {
                                    this.activate_item(active_item_index, true, true, cx);
                                },
                            ),
                        );

                        let mut tab_row = Flex::row()
                            .with_child(self.render_tabs(cx).flex(1., true).into_any_named("tabs"));

                        if self.has_focus {
                            let render_tab_bar_buttons = self.render_tab_bar_buttons.clone();
                            tab_row.add_child(
                                (render_tab_bar_buttons)(self, cx)
                                    .contained()
                                    .with_style(theme.workspace.tab_bar.pane_button_container)
                                    .flex(1., false)
                                    .into_any(),
                            )
                        }

                        stack.add_child(tab_row);
                        stack
                            .constrained()
                            .with_height(theme.workspace.tab_bar.height)
                            .flex(1., false)
                            .into_any_named("tab bar")
                    })
                    .with_child({
                        enum PaneContentTabDropTarget {}
                        dragged_item_receiver::<PaneContentTabDropTarget, _, _>(
                            self,
                            0,
                            self.active_item_index + 1,
                            !self.can_split,
                            if self.can_split { Some(100.) } else { None },
                            cx,
                            {
                                let toolbar = self.toolbar.clone();
                                let toolbar_hidden = toolbar.read(cx).hidden();
                                move |_, cx| {
                                    Flex::column()
                                        .with_children(
                                            (!toolbar_hidden)
                                                .then(|| ChildView::new(&toolbar, cx).expanded()),
                                        )
                                        .with_child(
                                            ChildView::new(active_item.as_any(), cx).flex(1., true),
                                        )
                                }
                            },
                        )
                        .flex(1., true)
                    })
                    .with_child(ChildView::new(&self.tab_context_menu, cx))
                    .into_any()
            } else {
                enum EmptyPane {}
                let theme = theme::current(cx).clone();

                dragged_item_receiver::<EmptyPane, _, _>(self, 0, 0, false, None, cx, |_, cx| {
                    self.render_blank_pane(&theme, cx)
                })
                .on_down(MouseButton::Left, |_, _, cx| {
                    cx.focus_parent();
                })
                .into_any()
            }
        })
        .on_down(
            MouseButton::Navigate(NavigationDirection::Back),
            move |_, pane, cx| {
                if let Some(workspace) = pane.workspace.upgrade(cx) {
                    let pane = cx.weak_handle();
                    cx.window_context().defer(move |cx| {
                        workspace.update(cx, |workspace, cx| {
                            workspace.go_back(pane, cx).detach_and_log_err(cx)
                        })
                    })
                }
            },
        )
        .on_down(MouseButton::Navigate(NavigationDirection::Forward), {
            move |_, pane, cx| {
                if let Some(workspace) = pane.workspace.upgrade(cx) {
                    let pane = cx.weak_handle();
                    cx.window_context().defer(move |cx| {
                        workspace.update(cx, |workspace, cx| {
                            workspace.go_forward(pane, cx).detach_and_log_err(cx)
                        })
                    })
                }
            }
        })
        .into_any_named("pane")
    }

    fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.has_focus {
            self.has_focus = true;
            cx.emit(Event::Focus);
            cx.notify();
        }

        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.focus_changed(true, cx);
        });

        if let Some(active_item) = self.active_item() {
            if cx.is_self_focused() {
                // Pane was focused directly. We need to either focus a view inside the active item,
                // or focus the active item itself
                if let Some(weak_last_focused_view) =
                    self.last_focused_view_by_item.get(&active_item.id())
                {
                    if let Some(last_focused_view) = weak_last_focused_view.upgrade(cx) {
                        cx.focus(&last_focused_view);
                        return;
                    } else {
                        self.last_focused_view_by_item.remove(&active_item.id());
                    }
                }

                cx.focus(active_item.as_any());
            } else if focused != self.tab_bar_context_menu.handle {
                self.last_focused_view_by_item
                    .insert(active_item.id(), focused.downgrade());
            }
        }
    }

    fn focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = false;
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.focus_changed(false, cx);
        });
        cx.notify();
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
    }
}

impl ItemNavHistory {
    pub fn push<D: 'static + Any>(&mut self, data: Option<D>, cx: &mut WindowContext) {
        self.history.push(data, self.item.clone(), cx);
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
        let borrowed_history = self.0.borrow();
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
                } else if let Some(item) = entry.item.upgrade(cx) {
                    if let Some(path) = item.project_path(cx) {
                        f(entry, (path, None));
                    }
                }
            })
    }

    pub fn set_mode(&mut self, mode: NavigationMode) {
        self.0.borrow_mut().mode = mode;
    }

    pub fn mode(&self) -> NavigationMode {
        self.0.borrow().mode
    }

    pub fn disable(&mut self) {
        self.0.borrow_mut().mode = NavigationMode::Disabled;
    }

    pub fn enable(&mut self) {
        self.0.borrow_mut().mode = NavigationMode::Normal;
    }

    pub fn pop(&mut self, mode: NavigationMode, cx: &mut WindowContext) -> Option<NavigationEntry> {
        let mut state = self.0.borrow_mut();
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

    pub fn push<D: 'static + Any>(
        &mut self,
        data: Option<D>,
        item: Rc<dyn WeakItemHandle>,
        cx: &mut WindowContext,
    ) {
        let state = &mut *self.0.borrow_mut();
        match state.mode {
            NavigationMode::Disabled => {}
            NavigationMode::Normal | NavigationMode::ReopeningClosedItem => {
                if state.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.backward_stack.pop_front();
                }
                state.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                });
                state.forward_stack.clear();
            }
            NavigationMode::GoingBack => {
                if state.forward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.forward_stack.pop_front();
                }
                state.forward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                });
            }
            NavigationMode::GoingForward => {
                if state.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.backward_stack.pop_front();
                }
                state.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                });
            }
            NavigationMode::ClosingItem => {
                if state.closed_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    state.closed_stack.pop_front();
                }
                state.closed_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                    timestamp: state.next_timestamp.fetch_add(1, Ordering::SeqCst),
                });
            }
        }
        state.did_update(cx);
    }

    pub fn remove_item(&mut self, item_id: usize) {
        let mut state = self.0.borrow_mut();
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

    pub fn path_for_item(&self, item_id: usize) -> Option<(ProjectPath, Option<PathBuf>)> {
        self.0.borrow().paths_by_item.get(&item_id).cloned()
    }
}

impl NavHistoryState {
    pub fn did_update(&self, cx: &mut WindowContext) {
        if let Some(pane) = self.pane.upgrade(cx) {
            cx.defer(move |cx| {
                pane.update(cx, |pane, cx| pane.history_updated(cx));
            });
        }
    }
}

pub struct PaneBackdrop<V: View> {
    child_view: usize,
    child: AnyElement<V>,
}

impl<V: View> PaneBackdrop<V> {
    pub fn new(pane_item_view: usize, child: AnyElement<V>) -> Self {
        PaneBackdrop {
            child,
            child_view: pane_item_view,
        }
    }
}

impl<V: View> Element<V> for PaneBackdrop<V> {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, view, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let background = theme::current(cx).editor.background;

        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        scene.push_quad(gpui::Quad {
            bounds: RectF::new(bounds.origin(), bounds.size()),
            background: Some(background),
            ..Default::default()
        });

        let child_view_id = self.child_view;
        scene.push_mouse_region(
            MouseRegion::new::<Self>(child_view_id, 0, visible_bounds).on_down(
                gpui::platform::MouseButton::Left,
                move |_, _: &mut V, cx| {
                    let window = cx.window();
                    cx.app_context().focus(window, Some(child_view_id))
                },
            ),
        );

        scene.paint_layer(Some(bounds), |scene| {
            self.child
                .paint(scene, bounds.origin(), visible_bounds, view, cx)
        })
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _bounds: RectF,
        _visible_bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> serde_json::Value {
        gpui::json::json!({
            "type": "Pane Back Drop",
            "view": self.child_view,
            "child": self.child.debug(view, cx),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::test::{TestItem, TestProjectItem};
    use gpui::TestAppContext;
    use project::FakeFs;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_remove_active_empty(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        pane.update(cx, |pane, cx| {
            assert!(pane.close_active_item(&CloseActiveItem, cx).is_none())
        });
    }

    #[gpui::test]
    async fn test_add_item_with_new_item(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   a. Add before the active item
        set_labeled_items(&pane, ["A", "B*", "C"], cx);
        pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
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
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
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
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
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
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
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
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
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
        cx.foreground().forbid_parking();
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

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
        cx.foreground().forbid_parking();
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        // singleton view
        pane.update(cx, |pane, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 1")
                .with_project_items(&[TestProjectItem::new(1, "one.txt", cx)]);

            pane.add_item(Box::new(cx.add_view(|_| item)), false, false, None, cx);
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with the same project entry
        pane.update(cx, |pane, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 1")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            pane.add_item(Box::new(cx.add_view(|_| item)), false, false, None, cx);
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with different project entry
        pane.update(cx, |pane, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 2")
                .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)]);
            pane.add_item(Box::new(cx.add_view(|_| item)), false, false, None, cx);
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2*"], cx);

        // new multibuffer view with the same project entry
        pane.update(cx, |pane, cx| {
            let item = TestItem::new()
                .with_singleton(false)
                .with_label("multibuffer 1")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            pane.add_item(Box::new(cx.add_view(|_| item)), false, false, None, cx);
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2", "multibuffer 1*"], cx);

        // another multibuffer view with the same project entry
        pane.update(cx, |pane, cx| {
            let item = TestItem::new()
                .with_singleton(false)
                .with_label("multibuffer 1b")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            pane.add_item(Box::new(cx.add_view(|_| item)), false, false, None, cx);
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
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        add_labeled_item(&pane, "D", false, cx);
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(1, false, false, cx));
        add_labeled_item(&pane, "1", false, cx);
        assert_item_labels(&pane, ["A", "B", "1*", "C", "D"], cx);

        pane.update(cx, |pane, cx| pane.close_active_item(&CloseActiveItem, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C", "D"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(3, false, false, cx));
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update(cx, |pane, cx| pane.close_active_item(&CloseActiveItem, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        pane.update(cx, |pane, cx| pane.close_active_item(&CloseActiveItem, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, ["A", "C*"], cx);

        pane.update(cx, |pane, cx| pane.close_active_item(&CloseActiveItem, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, ["A*"], cx);
    }

    #[gpui::test]
    async fn test_close_inactive_items(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        set_labeled_items(&pane, ["A", "B", "C*", "D", "E"], cx);

        pane.update(cx, |pane, cx| {
            pane.close_inactive_items(&CloseInactiveItems, cx)
        })
        .unwrap()
        .await
        .unwrap();
        assert_item_labels(&pane, ["C*"], cx);
    }

    #[gpui::test]
    async fn test_close_clean_items(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

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
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

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
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

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
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        add_labeled_item(&pane, "A", false, cx);
        add_labeled_item(&pane, "B", false, cx);
        add_labeled_item(&pane, "C", false, cx);
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        pane.update(cx, |pane, cx| pane.close_all_items(&CloseAllItems, cx))
            .unwrap()
            .await
            .unwrap();
        assert_item_labels(&pane, [], cx);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            theme::init((), cx);
            crate::init_settings(cx);
            Project::init_settings(cx);
        });
    }

    fn add_labeled_item(
        pane: &ViewHandle<Pane>,
        label: &str,
        is_dirty: bool,
        cx: &mut TestAppContext,
    ) -> Box<ViewHandle<TestItem>> {
        pane.update(cx, |pane, cx| {
            let labeled_item =
                Box::new(cx.add_view(|_| TestItem::new().with_label(label).with_dirty(is_dirty)));
            pane.add_item(labeled_item.clone(), false, false, None, cx);
            labeled_item
        })
    }

    fn set_labeled_items<const COUNT: usize>(
        pane: &ViewHandle<Pane>,
        labels: [&str; COUNT],
        cx: &mut TestAppContext,
    ) -> [Box<ViewHandle<TestItem>>; COUNT] {
        pane.update(cx, |pane, cx| {
            pane.items.clear();
            let mut active_item_index = 0;

            let mut index = 0;
            let items = labels.map(|mut label| {
                if label.ends_with("*") {
                    label = label.trim_end_matches("*");
                    active_item_index = index;
                }

                let labeled_item = Box::new(cx.add_view(|_| TestItem::new().with_label(label)));
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
        pane: &ViewHandle<Pane>,
        expected_states: [&str; COUNT],
        cx: &mut TestAppContext,
    ) {
        pane.read_with(cx, |pane, cx| {
            let actual_states = pane
                .items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let mut state = item
                        .as_any()
                        .downcast_ref::<TestItem>()
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
