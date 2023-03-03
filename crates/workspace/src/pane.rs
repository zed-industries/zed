mod dragged_item_receiver;

use super::{ItemHandle, SplitDirection};
use crate::{
    dock::{icon_for_dock_anchor, AnchorDockBottom, AnchorDockRight, ExpandDock, HideDock},
    item::WeakItemHandle,
    toolbar::Toolbar,
    Item, NewFile, NewSearch, NewTerminal, Workspace,
};
use anyhow::Result;
use collections::{HashMap, HashSet, VecDeque};
use context_menu::{ContextMenu, ContextMenuItem};
use drag_and_drop::Draggable;
pub use dragged_item_receiver::{dragged_item_receiver, handle_dropped_item};
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    impl_actions, impl_internal_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, NavigationDirection},
    Action, AnyViewHandle, AnyWeakViewHandle, AppContext, AsyncAppContext, Entity, EventContext,
    ModelHandle, MouseButton, MutableAppContext, PromptLevel, Quad, RenderContext, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use project::{Project, ProjectEntryId, ProjectPath};
use serde::Deserialize;
use settings::{Autosave, DockAnchor, Settings};
use std::{any::Any, cell::RefCell, cmp, mem, path::Path, rc::Rc};
use theme::Theme;
use util::ResultExt;

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivateItem(pub usize);

actions!(
    pane,
    [
        ActivatePrevItem,
        ActivateNextItem,
        ActivateLastItem,
        CloseActiveItem,
        CloseInactiveItems,
        CloseCleanItems,
        CloseAllItems,
        ReopenClosedItem,
        SplitLeft,
        SplitUp,
        SplitRight,
        SplitDown,
    ]
);

#[derive(Clone, PartialEq)]
pub struct CloseItem {
    pub item_id: usize,
    pub pane: WeakViewHandle<Pane>,
}

#[derive(Clone, PartialEq)]
pub struct MoveItem {
    pub item_id: usize,
    pub from: WeakViewHandle<Pane>,
    pub to: WeakViewHandle<Pane>,
    pub destination_index: usize,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct GoBack {
    #[serde(skip_deserializing)]
    pub pane: Option<WeakViewHandle<Pane>>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct GoForward {
    #[serde(skip_deserializing)]
    pub pane: Option<WeakViewHandle<Pane>>,
}

#[derive(Clone, PartialEq)]
pub struct DeploySplitMenu {
    position: Vector2F,
}

#[derive(Clone, PartialEq)]
pub struct DeployDockMenu {
    position: Vector2F,
}

#[derive(Clone, PartialEq)]
pub struct DeployNewMenu {
    position: Vector2F,
}

impl_actions!(pane, [GoBack, GoForward, ActivateItem]);
impl_internal_actions!(
    pane,
    [
        CloseItem,
        DeploySplitMenu,
        DeployNewMenu,
        DeployDockMenu,
        MoveItem
    ]
);

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub fn init(cx: &mut MutableAppContext) {
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
    cx.add_async_action(Pane::close_all_items);
    cx.add_async_action(|workspace: &mut Workspace, action: &CloseItem, cx| {
        let pane = action.pane.upgrade(cx)?;
        let task = Pane::close_item(workspace, pane, action.item_id, cx);
        Some(cx.foreground().spawn(async move {
            task.await?;
            Ok(())
        }))
    });
    cx.add_action(
        |workspace,
         MoveItem {
             from,
             to,
             item_id,
             destination_index,
         },
         cx| {
            // Get item handle to move
            let from = if let Some(from) = from.upgrade(cx) {
                from
            } else {
                return;
            };

            // Add item to new pane at given index
            let to = if let Some(to) = to.upgrade(cx) {
                to
            } else {
                return;
            };

            Pane::move_item(workspace, from, to, *item_id, *destination_index, cx)
        },
    );
    cx.add_action(|pane: &mut Pane, _: &SplitLeft, cx| pane.split(SplitDirection::Left, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitUp, cx| pane.split(SplitDirection::Up, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitRight, cx| pane.split(SplitDirection::Right, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitDown, cx| pane.split(SplitDirection::Down, cx));
    cx.add_action(Pane::deploy_split_menu);
    cx.add_action(Pane::deploy_new_menu);
    cx.add_action(Pane::deploy_dock_menu);
    cx.add_action(|workspace: &mut Workspace, _: &ReopenClosedItem, cx| {
        Pane::reopen_closed_item(workspace, cx).detach();
    });
    cx.add_action(|workspace: &mut Workspace, action: &GoBack, cx| {
        Pane::go_back(
            workspace,
            action
                .pane
                .as_ref()
                .and_then(|weak_handle| weak_handle.upgrade(cx)),
            cx,
        )
        .detach();
    });
    cx.add_action(|workspace: &mut Workspace, action: &GoForward, cx| {
        Pane::go_forward(
            workspace,
            action
                .pane
                .as_ref()
                .and_then(|weak_handle| weak_handle.upgrade(cx)),
            cx,
        )
        .detach();
    });
}

#[derive(Debug)]
pub enum Event {
    ActivateItem { local: bool },
    Remove,
    RemoveItem { item_id: usize },
    Split(SplitDirection),
    ChangeItemTitle,
}

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    activation_history: Vec<usize>,
    is_active: bool,
    active_item_index: usize,
    last_focused_view_by_item: HashMap<usize, AnyWeakViewHandle>,
    autoscroll: bool,
    nav_history: Rc<RefCell<NavHistory>>,
    toolbar: ViewHandle<Toolbar>,
    tab_bar_context_menu: ViewHandle<ContextMenu>,
    docked: Option<DockAnchor>,
}

pub struct ItemNavHistory {
    history: Rc<RefCell<NavHistory>>,
    item: Rc<dyn WeakItemHandle>,
}

struct NavHistory {
    mode: NavigationMode,
    backward_stack: VecDeque<NavigationEntry>,
    forward_stack: VecDeque<NavigationEntry>,
    closed_stack: VecDeque<NavigationEntry>,
    paths_by_item: HashMap<usize, ProjectPath>,
    pane: WeakViewHandle<Pane>,
}

#[derive(Copy, Clone)]
enum NavigationMode {
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
}

struct DraggedItem {
    item: Box<dyn ItemHandle>,
    pane: WeakViewHandle<Pane>,
}

pub enum ReorderBehavior {
    None,
    MoveAfterActive,
    MoveToIndex(usize),
}

enum ItemType {
    Active,
    Inactive,
    Clean,
    All,
}

impl Pane {
    pub fn new(docked: Option<DockAnchor>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        let context_menu = cx.add_view(ContextMenu::new);
        Self {
            items: Vec::new(),
            activation_history: Vec::new(),
            is_active: true,
            active_item_index: 0,
            last_focused_view_by_item: Default::default(),
            autoscroll: false,
            nav_history: Rc::new(RefCell::new(NavHistory {
                mode: NavigationMode::Normal,
                backward_stack: Default::default(),
                forward_stack: Default::default(),
                closed_stack: Default::default(),
                paths_by_item: Default::default(),
                pane: handle.clone(),
            })),
            toolbar: cx.add_view(|_| Toolbar::new(handle)),
            tab_bar_context_menu: context_menu,
            docked,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, is_active: bool, cx: &mut ViewContext<Self>) {
        self.is_active = is_active;
        cx.notify();
    }

    pub fn set_docked(&mut self, docked: Option<DockAnchor>, cx: &mut ViewContext<Self>) {
        self.docked = docked;
        cx.notify();
    }

    pub fn nav_history_for_item<T: Item>(&self, item: &ViewHandle<T>) -> ItemNavHistory {
        ItemNavHistory {
            history: self.nav_history.clone(),
            item: Rc::new(item.downgrade()),
        }
    }

    pub fn go_back(
        workspace: &mut Workspace,
        pane: Option<ViewHandle<Pane>>,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<()> {
        Self::navigate_history(
            workspace,
            pane.unwrap_or_else(|| workspace.active_pane().clone()),
            NavigationMode::GoingBack,
            cx,
        )
    }

    pub fn go_forward(
        workspace: &mut Workspace,
        pane: Option<ViewHandle<Pane>>,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<()> {
        Self::navigate_history(
            workspace,
            pane.unwrap_or_else(|| workspace.active_pane().clone()),
            NavigationMode::GoingForward,
            cx,
        )
    }

    pub fn reopen_closed_item(
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<()> {
        Self::navigate_history(
            workspace,
            workspace.active_pane().clone(),
            NavigationMode::ReopeningClosedItem,
            cx,
        )
    }

    pub fn disable_history(&mut self) {
        self.nav_history.borrow_mut().disable();
    }

    pub fn enable_history(&mut self) {
        self.nav_history.borrow_mut().enable();
    }

    pub fn can_navigate_backward(&self) -> bool {
        !self.nav_history.borrow().backward_stack.is_empty()
    }

    pub fn can_navigate_forward(&self) -> bool {
        !self.nav_history.borrow().forward_stack.is_empty()
    }

    fn history_updated(&mut self, cx: &mut ViewContext<Self>) {
        self.toolbar.update(cx, |_, cx| cx.notify());
    }

    fn navigate_history(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        mode: NavigationMode,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<()> {
        cx.focus(pane.clone());

        let to_load = pane.update(cx, |pane, cx| {
            loop {
                // Retrieve the weak item handle from the history.
                let entry = pane.nav_history.borrow_mut().pop(mode, cx)?;

                // If the item is still present in this pane, then activate it.
                if let Some(index) = entry
                    .item
                    .upgrade(cx)
                    .and_then(|v| pane.index_for_item(v.as_ref()))
                {
                    let prev_active_item_index = pane.active_item_index;
                    pane.nav_history.borrow_mut().set_mode(mode);
                    pane.activate_item(index, true, true, cx);
                    pane.nav_history
                        .borrow_mut()
                        .set_mode(NavigationMode::Normal);

                    let mut navigated = prev_active_item_index != pane.active_item_index;
                    if let Some(data) = entry.data {
                        navigated |= pane.active_item()?.navigate(data, cx);
                    }

                    if navigated {
                        break None;
                    }
                }
                // If the item is no longer present in this pane, then retrieve its
                // project path in order to reopen it.
                else {
                    break pane
                        .nav_history
                        .borrow()
                        .paths_by_item
                        .get(&entry.item.id())
                        .cloned()
                        .map(|project_path| (project_path, entry));
                }
            }
        });

        if let Some((project_path, entry)) = to_load {
            // If the item was no longer present, then load it again from its previous path.
            let pane = pane.downgrade();
            let task = workspace.load_path(project_path, cx);
            cx.spawn(|workspace, mut cx| async move {
                let task = task.await;
                if let Some(pane) = pane.upgrade(&cx) {
                    let mut navigated = false;
                    if let Some((project_entry_id, build_item)) = task.log_err() {
                        let prev_active_item_id = pane.update(&mut cx, |pane, _| {
                            pane.nav_history.borrow_mut().set_mode(mode);
                            pane.active_item().map(|p| p.id())
                        });

                        let item = workspace.update(&mut cx, |workspace, cx| {
                            Self::open_item(
                                workspace,
                                pane.clone(),
                                project_entry_id,
                                true,
                                cx,
                                build_item,
                            )
                        });

                        pane.update(&mut cx, |pane, cx| {
                            navigated |= Some(item.id()) != prev_active_item_id;
                            pane.nav_history
                                .borrow_mut()
                                .set_mode(NavigationMode::Normal);
                            if let Some(data) = entry.data {
                                navigated |= item.navigate(data, cx);
                            }
                        });
                    }

                    if !navigated {
                        workspace
                            .update(&mut cx, |workspace, cx| {
                                Self::navigate_history(workspace, pane, mode, cx)
                            })
                            .await;
                    }
                }
            })
        } else {
            Task::ready(())
        }
    }

    pub(crate) fn open_item(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        project_entry_id: ProjectEntryId,
        focus_item: bool,
        cx: &mut ViewContext<Workspace>,
        build_item: impl FnOnce(&mut ViewContext<Pane>) -> Box<dyn ItemHandle>,
    ) -> Box<dyn ItemHandle> {
        let existing_item = pane.update(cx, |pane, cx| {
            for (index, item) in pane.items.iter().enumerate() {
                if item.is_singleton(cx)
                    && item.project_entry_ids(cx).as_slice() == [project_entry_id]
                {
                    let item = item.boxed_clone();
                    return Some((index, item));
                }
            }
            None
        });

        if let Some((index, existing_item)) = existing_item {
            pane.update(cx, |pane, cx| {
                pane.activate_item(index, focus_item, focus_item, cx);
            });
            existing_item
        } else {
            let new_item = pane.update(cx, |_, cx| build_item(cx));
            Pane::add_item(
                workspace,
                &pane,
                new_item.clone(),
                true,
                focus_item,
                None,
                cx,
            );
            new_item
        }
    }

    pub(crate) fn add_item(
        workspace: &mut Workspace,
        pane: &ViewHandle<Pane>,
        item: Box<dyn ItemHandle>,
        activate_pane: bool,
        focus_item: bool,
        destination_index: Option<usize>,
        cx: &mut ViewContext<Workspace>,
    ) {
        // If no destination index is specified, add or move the item after the active item.
        let mut insertion_index = {
            let pane = pane.read(cx);
            cmp::min(
                if let Some(destination_index) = destination_index {
                    destination_index
                } else {
                    pane.active_item_index + 1
                },
                pane.items.len(),
            )
        };

        item.added_to_pane(workspace, pane.clone(), cx);

        // Does the item already exist?
        let project_entry_id = if item.is_singleton(cx) {
            item.project_entry_ids(cx).get(0).copied()
        } else {
            None
        };

        let existing_item_index = pane.read(cx).items.iter().position(|existing_item| {
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
            pane.update(cx, |pane, cx| {
                if existing_item_index != insertion_index {
                    cx.reparent(&item);
                    let existing_item_is_active = existing_item_index == pane.active_item_index;

                    // If the caller didn't specify a destination and the added item is already
                    // the active one, don't move it
                    if existing_item_is_active && destination_index.is_none() {
                        insertion_index = existing_item_index;
                    } else {
                        pane.items.remove(existing_item_index);
                        if existing_item_index < pane.active_item_index {
                            pane.active_item_index -= 1;
                        }
                        insertion_index = insertion_index.min(pane.items.len());

                        pane.items.insert(insertion_index, item.clone());

                        if existing_item_is_active {
                            pane.active_item_index = insertion_index;
                        } else if insertion_index <= pane.active_item_index {
                            pane.active_item_index += 1;
                        }
                    }

                    cx.notify();
                }

                pane.activate_item(insertion_index, activate_pane, focus_item, cx);
            });
        } else {
            pane.update(cx, |pane, cx| {
                cx.reparent(&item);
                pane.items.insert(insertion_index, item);
                if insertion_index <= pane.active_item_index {
                    pane.active_item_index += 1;
                }

                pane.activate_item(insertion_index, activate_pane, focus_item, cx);
                cx.notify();
            });
        }
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> impl Iterator<Item = &Box<dyn ItemHandle>> {
        self.items.iter()
    }

    pub fn items_of_type<T: View>(&self) -> impl '_ + Iterator<Item = ViewHandle<T>> {
        self.items
            .iter()
            .filter_map(|item| item.to_any().downcast())
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.items.get(self.active_item_index).cloned()
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
                || matches!(self.nav_history.borrow().mode, GoingBack | GoingForward)
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
        workspace: &mut Workspace,
        _: &CloseActiveItem,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        Self::close_main(workspace, ItemType::Active, cx)
    }

    pub fn close_inactive_items(
        workspace: &mut Workspace,
        _: &CloseInactiveItems,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        Self::close_main(workspace, ItemType::Inactive, cx)
    }

    pub fn close_all_items(
        workspace: &mut Workspace,
        _: &CloseAllItems,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        Self::close_main(workspace, ItemType::All, cx)
    }

    pub fn close_clean_items(
        workspace: &mut Workspace,
        _: &CloseCleanItems,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        Self::close_main(workspace, ItemType::Clean, cx)
    }

    fn close_main(
        workspace: &mut Workspace,
        close_item_type: ItemType,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let pane_handle = workspace.active_pane().clone();
        let pane = pane_handle.read(cx);
        if pane.items.is_empty() {
            return None;
        }

        let active_item_id = pane.items[pane.active_item_index].id();
        let clean_item_ids: Vec<_> = pane
            .items()
            .filter(|item| !item.is_dirty(cx))
            .map(|item| item.id())
            .collect();
        let task =
            Self::close_items(
                workspace,
                pane_handle,
                cx,
                move |item_id| match close_item_type {
                    ItemType::Active => item_id == active_item_id,
                    ItemType::Inactive => item_id != active_item_id,
                    ItemType::Clean => clean_item_ids.contains(&item_id),
                    ItemType::All => true,
                },
            );

        Some(cx.foreground().spawn(async move {
            task.await?;
            Ok(())
        }))
    }

    pub fn close_item(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        item_id_to_close: usize,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<()>> {
        Self::close_items(workspace, pane, cx, move |view_id| {
            view_id == item_id_to_close
        })
    }

    pub fn close_items(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
        should_close: impl 'static + Fn(usize) -> bool,
    ) -> Task<Result<()>> {
        let project = workspace.project().clone();

        // Find the items to close.
        let mut items_to_close = Vec::new();
        for item in &pane.read(cx).items {
            if should_close(item.id()) {
                items_to_close.push(item.boxed_clone());
            }
        }

        // If a buffer is open both in a singleton editor and in a multibuffer, make sure
        // to focus the singleton buffer when prompting to save that buffer, as opposed
        // to focusing the multibuffer, because this gives the user a more clear idea
        // of what content they would be saving.
        items_to_close.sort_by_key(|item| !item.is_singleton(cx));

        cx.spawn(|workspace, mut cx| async move {
            let mut saved_project_items_ids = HashSet::default();
            for item in items_to_close.clone() {
                // Find the item's current index and its set of project item models. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let (item_ix, mut project_item_ids) = pane.read_with(&cx, |pane, cx| {
                    (pane.index_for_item(&*item), item.project_item_model_ids(cx))
                });
                let item_ix = if let Some(ix) = item_ix {
                    ix
                } else {
                    continue;
                };

                // Check if this view has any project items that are not open anywhere else
                // in the workspace, AND that the user has not already been prompted to save.
                // If there are any such project entries, prompt the user to save this item.
                workspace.read_with(&cx, |workspace, cx| {
                    for item in workspace.items(cx) {
                        if !items_to_close
                            .iter()
                            .any(|item_to_close| item_to_close.id() == item.id())
                        {
                            let other_project_item_ids = item.project_item_model_ids(cx);
                            project_item_ids.retain(|id| !other_project_item_ids.contains(id));
                        }
                    }
                });
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
                });
            }

            pane.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    fn remove_item(&mut self, item_index: usize, activate_pane: bool, cx: &mut ViewContext<Self>) {
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

            self.activate_item(index_to_activate, activate_pane, activate_pane, cx);
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

        self.nav_history
            .borrow_mut()
            .set_mode(NavigationMode::ClosingItem);
        item.deactivated(cx);
        self.nav_history
            .borrow_mut()
            .set_mode(NavigationMode::Normal);

        if let Some(path) = item.project_path(cx) {
            self.nav_history
                .borrow_mut()
                .paths_by_item
                .insert(item.id(), path);
        } else {
            self.nav_history
                .borrow_mut()
                .paths_by_item
                .remove(&item.id());
        }

        cx.notify();
    }

    pub async fn save_item(
        project: ModelHandle<Project>,
        pane: &ViewHandle<Pane>,
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
            });
            match answer.next().await {
                Some(0) => cx.update(|cx| item.save(project, cx)).await?,
                Some(1) => cx.update(|cx| item.reload(project, cx)).await?,
                _ => return Ok(false),
            }
        } else if is_dirty && (can_save || is_singleton) {
            let will_autosave = cx.read(|cx| {
                matches!(
                    cx.global::<Settings>().autosave,
                    Autosave::OnFocusChange | Autosave::OnWindowChange
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
                });
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
                    cx.update(|cx| item.save(project, cx)).await?;
                } else if is_singleton {
                    let start_abs_path = project
                        .read_with(cx, |project, cx| {
                            let worktree = project.visible_worktrees(cx).next()?;
                            Some(worktree.read(cx).as_local()?.abs_path().to_path_buf())
                        })
                        .unwrap_or_else(|| Path::new("").into());

                    let mut abs_path = cx.update(|cx| cx.prompt_for_new_path(&start_abs_path));
                    if let Some(abs_path) = abs_path.next().await.flatten() {
                        cx.update(|cx| item.save_as(project, abs_path, cx)).await?;
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
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>> {
        if Self::can_autosave_item(item, cx) {
            item.save(project, cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            cx.focus(active_item);
        }
    }

    pub fn move_item(
        workspace: &mut Workspace,
        from: ViewHandle<Pane>,
        to: ViewHandle<Pane>,
        item_id_to_move: usize,
        destination_index: usize,
        cx: &mut ViewContext<Workspace>,
    ) {
        let item_to_move = from
            .read(cx)
            .items()
            .enumerate()
            .find(|(_, item_handle)| item_handle.id() == item_id_to_move);

        if item_to_move.is_none() {
            log::warn!("Tried to move item handle which was not in `from` pane. Maybe tab was closed during drop");
            return;
        }
        let (item_ix, item_handle) = item_to_move.unwrap();
        let item_handle = item_handle.clone();

        if from != to {
            // Close item from previous pane
            from.update(cx, |from, cx| {
                from.remove_item(item_ix, false, cx);
            });
        }

        // This automatically removes duplicate items in the pane
        Pane::add_item(
            workspace,
            &to,
            item_handle,
            true,
            true,
            Some(destination_index),
            cx,
        );

        cx.focus(to);
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
    }

    fn deploy_split_menu(&mut self, action: &DeploySplitMenu, cx: &mut ViewContext<Self>) {
        self.tab_bar_context_menu.update(cx, |menu, cx| {
            menu.show(
                action.position,
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::item("Split Right", SplitRight),
                    ContextMenuItem::item("Split Left", SplitLeft),
                    ContextMenuItem::item("Split Up", SplitUp),
                    ContextMenuItem::item("Split Down", SplitDown),
                ],
                cx,
            );
        });
    }

    fn deploy_dock_menu(&mut self, action: &DeployDockMenu, cx: &mut ViewContext<Self>) {
        self.tab_bar_context_menu.update(cx, |menu, cx| {
            menu.show(
                action.position,
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::item("Anchor Dock Right", AnchorDockRight),
                    ContextMenuItem::item("Anchor Dock Bottom", AnchorDockBottom),
                    ContextMenuItem::item("Expand Dock", ExpandDock),
                ],
                cx,
            );
        });
    }

    fn deploy_new_menu(&mut self, action: &DeployNewMenu, cx: &mut ViewContext<Self>) {
        self.tab_bar_context_menu.update(cx, |menu, cx| {
            menu.show(
                action.position,
                AnchorCorner::TopRight,
                vec![
                    ContextMenuItem::item("New File", NewFile),
                    ContextMenuItem::item("New Terminal", NewTerminal),
                    ContextMenuItem::item("New Search", NewSearch),
                ],
                cx,
            );
        });
    }

    pub fn toolbar(&self) -> &ViewHandle<Toolbar> {
        &self.toolbar
    }

    fn update_toolbar(&mut self, cx: &mut ViewContext<Self>) {
        let active_item = self
            .items
            .get(self.active_item_index)
            .map(|item| item.as_ref());
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.set_active_pane_item(active_item, cx);
        });
    }

    fn render_tabs(&mut self, cx: &mut RenderContext<Self>) -> impl Element {
        let theme = cx.global::<Settings>().theme.clone();

        let pane = cx.handle();
        let autoscroll = if mem::take(&mut self.autoscroll) {
            Some(self.active_item_index)
        } else {
            None
        };

        let pane_active = self.is_active;

        enum Tabs {}
        let mut row = Flex::row().scrollable::<Tabs, _>(1, autoscroll, cx);
        for (ix, (item, detail)) in self
            .items
            .iter()
            .cloned()
            .zip(self.tab_details(cx))
            .enumerate()
        {
            let detail = if detail == 0 { None } else { Some(detail) };
            let tab_active = ix == self.active_item_index;

            row.add_child({
                enum TabDragReceiver {}
                let mut receiver =
                    dragged_item_receiver::<TabDragReceiver, _>(ix, ix, true, None, cx, {
                        let item = item.clone();
                        let pane = pane.clone();
                        let detail = detail.clone();

                        let theme = cx.global::<Settings>().theme.clone();

                        move |mouse_state, cx| {
                            let tab_style =
                                theme.workspace.tab_bar.tab_style(pane_active, tab_active);
                            let hovered = mouse_state.hovered();

                            enum Tab {}
                            MouseEventHandler::<Tab>::new(ix, cx, |_, cx| {
                                Self::render_tab(
                                    &item,
                                    pane.clone(),
                                    ix == 0,
                                    detail,
                                    hovered,
                                    tab_style,
                                    cx,
                                )
                            })
                            .on_down(MouseButton::Left, move |_, cx| {
                                cx.dispatch_action(ActivateItem(ix));
                            })
                            .on_click(MouseButton::Middle, {
                                let item = item.clone();
                                move |_, cx: &mut EventContext| {
                                    cx.dispatch_action(CloseItem {
                                        item_id: item.id(),
                                        pane: pane.clone(),
                                    })
                                }
                            })
                            .boxed()
                        }
                    });

                if !pane_active || !tab_active {
                    receiver = receiver.with_cursor_style(CursorStyle::PointingHand);
                }

                receiver
                    .as_draggable(
                        DraggedItem {
                            item,
                            pane: pane.clone(),
                        },
                        {
                            let theme = cx.global::<Settings>().theme.clone();

                            let detail = detail.clone();
                            move |dragged_item, cx: &mut RenderContext<Workspace>| {
                                let tab_style = &theme.workspace.tab_bar.dragged_tab;
                                Self::render_tab(
                                    &dragged_item.item,
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
                    .boxed()
            })
        }

        // Use the inactive tab style along with the current pane's active status to decide how to render
        // the filler
        let filler_index = self.items.len();
        let filler_style = theme.workspace.tab_bar.tab_style(pane_active, false);
        enum Filler {}
        row.add_child(
            dragged_item_receiver::<Filler, _>(0, filler_index, true, None, cx, |_, _| {
                Empty::new()
                    .contained()
                    .with_style(filler_style.container)
                    .with_border(filler_style.container.border)
                    .boxed()
            })
            .flex(1., true)
            .named("filler"),
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

    fn render_tab<V: View>(
        item: &Box<dyn ItemHandle>,
        pane: WeakViewHandle<Pane>,
        first: bool,
        detail: Option<usize>,
        hovered: bool,
        tab_style: &theme::Tab,
        cx: &mut RenderContext<V>,
    ) -> ElementBox {
        let title = item.tab_content(detail, &tab_style, cx);
        let mut container = tab_style.container.clone();
        if first {
            container.border.left = false;
        }

        Flex::row()
            .with_child(
                Align::new({
                    let diameter = 7.0;
                    let icon_color = if item.has_conflict(cx) {
                        Some(tab_style.icon_conflict)
                    } else if item.is_dirty(cx) {
                        Some(tab_style.icon_dirty)
                    } else {
                        None
                    };

                    ConstrainedBox::new(
                        Canvas::new(move |bounds, _, cx| {
                            if let Some(color) = icon_color {
                                let square = RectF::new(bounds.origin(), vec2f(diameter, diameter));
                                cx.scene.push_quad(Quad {
                                    bounds: square,
                                    background: Some(color),
                                    border: Default::default(),
                                    corner_radius: diameter / 2.,
                                });
                            }
                        })
                        .boxed(),
                    )
                    .with_width(diameter)
                    .with_height(diameter)
                    .boxed()
                })
                .boxed(),
            )
            .with_child(
                Container::new(Align::new(title).boxed())
                    .with_style(ContainerStyle {
                        margin: Margin {
                            left: tab_style.spacing,
                            right: tab_style.spacing,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .boxed(),
            )
            .with_child(
                Align::new(
                    ConstrainedBox::new(if hovered {
                        let item_id = item.id();
                        enum TabCloseButton {}
                        let icon = Svg::new("icons/x_mark_8.svg");
                        MouseEventHandler::<TabCloseButton>::new(item_id, cx, |mouse_state, _| {
                            if mouse_state.hovered() {
                                icon.with_color(tab_style.icon_close_active).boxed()
                            } else {
                                icon.with_color(tab_style.icon_close).boxed()
                            }
                        })
                        .with_padding(Padding::uniform(4.))
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, {
                            let pane = pane.clone();
                            move |_, cx| {
                                cx.dispatch_action(CloseItem {
                                    item_id,
                                    pane: pane.clone(),
                                })
                            }
                        })
                        .named("close-tab-icon")
                    } else {
                        Empty::new().boxed()
                    })
                    .with_width(tab_style.icon_width)
                    .boxed(),
                )
                .boxed(),
            )
            .contained()
            .with_style(container)
            .constrained()
            .with_height(tab_style.height)
            .boxed()
    }

    fn render_tab_bar_buttons(
        &mut self,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        Flex::row()
            // New menu
            .with_child(tab_bar_button(0, "icons/plus_12.svg", cx, |position| {
                DeployNewMenu { position }
            }))
            .with_child(
                self.docked
                    .map(|anchor| {
                        // Add the dock menu button if this pane is a dock
                        let dock_icon = icon_for_dock_anchor(anchor);

                        tab_bar_button(1, dock_icon, cx, |position| DeployDockMenu { position })
                    })
                    .unwrap_or_else(|| {
                        // Add the split menu if this pane is not a dock
                        tab_bar_button(2, "icons/split_12.svg", cx, |position| DeploySplitMenu {
                            position,
                        })
                    }),
            )
            // Add the close dock button if this pane is a dock
            .with_children(
                self.docked
                    .map(|_| tab_bar_button(3, "icons/x_mark_8.svg", cx, |_| HideDock)),
            )
            .contained()
            .with_style(theme.workspace.tab_bar.pane_button_container)
            .flex(1., false)
            .boxed()
    }
}

impl Entity for Pane {
    type Event = Event;
}

impl View for Pane {
    fn ui_name() -> &'static str {
        "Pane"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let this = cx.handle();

        enum MouseNavigationHandler {}

        Stack::new()
            .with_child(
                MouseEventHandler::<MouseNavigationHandler>::new(0, cx, |_, cx| {
                    let active_item_index = self.active_item_index;

                    if let Some(active_item) = self.active_item() {
                        Flex::column()
                            .with_child({
                                let theme = cx.global::<Settings>().theme.clone();

                                let mut stack = Stack::new();

                                enum TabBarEventHandler {}
                                stack.add_child(
                                    MouseEventHandler::<TabBarEventHandler>::new(0, cx, |_, _| {
                                        Empty::new()
                                            .contained()
                                            .with_style(theme.workspace.tab_bar.container)
                                            .boxed()
                                    })
                                    .on_down(MouseButton::Left, move |_, cx| {
                                        cx.dispatch_action(ActivateItem(active_item_index));
                                    })
                                    .boxed(),
                                );

                                let mut tab_row = Flex::row()
                                    .with_child(self.render_tabs(cx).flex(1., true).named("tabs"));

                                if self.is_active {
                                    tab_row.add_child(self.render_tab_bar_buttons(&theme, cx))
                                }

                                stack.add_child(tab_row.boxed());
                                stack
                                    .constrained()
                                    .with_height(theme.workspace.tab_bar.height)
                                    .flex(1., false)
                                    .named("tab bar")
                            })
                            .with_child({
                                enum PaneContentTabDropTarget {}
                                dragged_item_receiver::<PaneContentTabDropTarget, _>(
                                    0,
                                    self.active_item_index + 1,
                                    false,
                                    if self.docked.is_some() {
                                        None
                                    } else {
                                        Some(100.)
                                    },
                                    cx,
                                    {
                                        let toolbar = self.toolbar.clone();
                                        move |_, cx| {
                                            Flex::column()
                                                .with_child(
                                                    ChildView::new(&toolbar, cx).expanded().boxed(),
                                                )
                                                .with_child(
                                                    ChildView::new(active_item, cx)
                                                        .flex(1., true)
                                                        .boxed(),
                                                )
                                                .boxed()
                                        }
                                    },
                                )
                                .flex(1., true)
                                .boxed()
                            })
                            .boxed()
                    } else {
                        enum EmptyPane {}
                        let theme = cx.global::<Settings>().theme.clone();

                        dragged_item_receiver::<EmptyPane, _>(0, 0, false, None, cx, |_, _| {
                            Empty::new()
                                .contained()
                                .with_background_color(theme.workspace.background)
                                .boxed()
                        })
                        .on_down(MouseButton::Left, |_, cx| {
                            cx.focus_parent_view();
                        })
                        .boxed()
                    }
                })
                .on_down(MouseButton::Navigate(NavigationDirection::Back), {
                    let this = this.clone();
                    move |_, cx| {
                        cx.dispatch_action(GoBack {
                            pane: Some(this.clone()),
                        });
                    }
                })
                .on_down(MouseButton::Navigate(NavigationDirection::Forward), {
                    let this = this.clone();
                    move |_, cx| {
                        cx.dispatch_action(GoForward {
                            pane: Some(this.clone()),
                        })
                    }
                })
                .boxed(),
            )
            .with_child(ChildView::new(&self.tab_bar_context_menu, cx).boxed())
            .named("pane")
    }

    fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            if cx.is_self_focused() {
                // Pane was focused directly. We need to either focus a view inside the active item,
                // or focus the active item itself
                if let Some(weak_last_focused_view) =
                    self.last_focused_view_by_item.get(&active_item.id())
                {
                    if let Some(last_focused_view) = weak_last_focused_view.upgrade(cx) {
                        cx.focus(last_focused_view);
                        return;
                    } else {
                        self.last_focused_view_by_item.remove(&active_item.id());
                    }
                }

                cx.focus(active_item);
            } else if focused != self.tab_bar_context_menu {
                self.last_focused_view_by_item
                    .insert(active_item.id(), focused.downgrade());
            }
        }
    }

    fn keymap_context(&self, _: &AppContext) -> KeymapContext {
        let mut keymap = Self::default_keymap_context();
        if self.docked.is_some() {
            keymap.add_identifier("docked");
        }
        keymap
    }
}

fn tab_bar_button<A: Action>(
    index: usize,
    icon: &'static str,
    cx: &mut RenderContext<Pane>,
    action_builder: impl 'static + Fn(Vector2F) -> A,
) -> ElementBox {
    enum TabBarButton {}

    MouseEventHandler::<TabBarButton>::new(index, cx, |mouse_state, cx| {
        let theme = &cx.global::<Settings>().theme.workspace.tab_bar;
        let style = theme.pane_button.style_for(mouse_state, false);
        Svg::new(icon)
            .with_color(style.color)
            .constrained()
            .with_width(style.icon_width)
            .aligned()
            .constrained()
            .with_width(style.button_width)
            .with_height(style.button_width)
            // .aligned()
            .boxed()
    })
    .with_cursor_style(CursorStyle::PointingHand)
    .on_click(MouseButton::Left, move |e, cx| {
        cx.dispatch_action(action_builder(e.region.lower_right()));
    })
    .flex(1., false)
    .boxed()
}

impl ItemNavHistory {
    pub fn push<D: 'static + Any>(&self, data: Option<D>, cx: &mut MutableAppContext) {
        self.history.borrow_mut().push(data, self.item.clone(), cx);
    }

    pub fn pop_backward(&self, cx: &mut MutableAppContext) -> Option<NavigationEntry> {
        self.history.borrow_mut().pop(NavigationMode::GoingBack, cx)
    }

    pub fn pop_forward(&self, cx: &mut MutableAppContext) -> Option<NavigationEntry> {
        self.history
            .borrow_mut()
            .pop(NavigationMode::GoingForward, cx)
    }
}

impl NavHistory {
    fn set_mode(&mut self, mode: NavigationMode) {
        self.mode = mode;
    }

    fn disable(&mut self) {
        self.mode = NavigationMode::Disabled;
    }

    fn enable(&mut self) {
        self.mode = NavigationMode::Normal;
    }

    fn pop(&mut self, mode: NavigationMode, cx: &mut MutableAppContext) -> Option<NavigationEntry> {
        let entry = match mode {
            NavigationMode::Normal | NavigationMode::Disabled | NavigationMode::ClosingItem => {
                return None
            }
            NavigationMode::GoingBack => &mut self.backward_stack,
            NavigationMode::GoingForward => &mut self.forward_stack,
            NavigationMode::ReopeningClosedItem => &mut self.closed_stack,
        }
        .pop_back();
        if entry.is_some() {
            self.did_update(cx);
        }
        entry
    }

    fn push<D: 'static + Any>(
        &mut self,
        data: Option<D>,
        item: Rc<dyn WeakItemHandle>,
        cx: &mut MutableAppContext,
    ) {
        match self.mode {
            NavigationMode::Disabled => {}
            NavigationMode::Normal | NavigationMode::ReopeningClosedItem => {
                if self.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.backward_stack.pop_front();
                }
                self.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
                self.forward_stack.clear();
            }
            NavigationMode::GoingBack => {
                if self.forward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.forward_stack.pop_front();
                }
                self.forward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
            NavigationMode::GoingForward => {
                if self.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.backward_stack.pop_front();
                }
                self.backward_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
            NavigationMode::ClosingItem => {
                if self.closed_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.closed_stack.pop_front();
                }
                self.closed_stack.push_back(NavigationEntry {
                    item,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
        }
        self.did_update(cx);
    }

    fn did_update(&self, cx: &mut MutableAppContext) {
        if let Some(pane) = self.pane.upgrade(cx) {
            cx.defer(move |cx| pane.update(cx, |pane, cx| pane.history_updated(cx)));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::item::test::{TestItem, TestProjectItem};
    use gpui::{executor::Deterministic, TestAppContext};
    use project::FakeFs;

    #[gpui::test]
    async fn test_add_item_with_new_item(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   a. Add before the active item
        set_labeled_items(&workspace, &pane, ["A", "B*", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
                false,
                false,
                Some(0),
                cx,
            );
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   b. Add after the active item
        set_labeled_items(&workspace, &pane, ["A", "B*", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
                false,
                false,
                Some(2),
                cx,
            );
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   c. Add at the end of the item list (including off the length)
        set_labeled_items(&workspace, &pane, ["A", "B*", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(
                workspace,
                &pane,
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
        set_labeled_items(&workspace, &pane, ["A*", "B", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| TestItem::new().with_label("D"))),
                false,
                false,
                None,
                cx,
            );
        });
        set_labeled_items(&workspace, &pane, ["A", "D*", "B", "C"], cx);

        //   b. Add with active item at the end of the item list
        set_labeled_items(&workspace, &pane, ["A", "B", "C*"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(
                workspace,
                &pane,
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
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        // 1. Add with a destination index
        //   1a. Add before the active item
        let [_, _, _, d] = set_labeled_items(&workspace, &pane, ["A", "B*", "C", "D"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, d, false, false, Some(0), cx);
        });
        assert_item_labels(&pane, ["D*", "A", "B", "C"], cx);

        //   1b. Add after the active item
        let [_, _, _, d] = set_labeled_items(&workspace, &pane, ["A", "B*", "C", "D"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, d, false, false, Some(2), cx);
        });
        assert_item_labels(&pane, ["A", "B", "D*", "C"], cx);

        //   1c. Add at the end of the item list (including off the length)
        let [a, _, _, _] = set_labeled_items(&workspace, &pane, ["A", "B*", "C", "D"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, a, false, false, Some(5), cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   1d. Add same item to active index
        let [_, b, _] = set_labeled_items(&workspace, &pane, ["A", "B*", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, b, false, false, Some(1), cx);
        });
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        //   1e. Add item to index after same item in last position
        let [_, _, c] = set_labeled_items(&workspace, &pane, ["A", "B*", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, c, false, false, Some(2), cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        // 2. Add without a destination index
        //   2a. Add with active item at the start of the item list
        let [_, _, _, d] = set_labeled_items(&workspace, &pane, ["A*", "B", "C", "D"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, d, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A", "D*", "B", "C"], cx);

        //   2b. Add with active item at the end of the item list
        let [a, _, _, _] = set_labeled_items(&workspace, &pane, ["A", "B", "C", "D*"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, a, false, false, None, cx);
        });
        assert_item_labels(&pane, ["B", "C", "D", "A*"], cx);

        //   2c. Add active item to active item at end of list
        let [_, _, c] = set_labeled_items(&workspace, &pane, ["A", "B", "C*"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, c, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A", "B", "C*"], cx);

        //   2d. Add active item to active item at start of list
        let [a, _, _] = set_labeled_items(&workspace, &pane, ["A*", "B", "C"], cx);
        workspace.update(cx, |workspace, cx| {
            Pane::add_item(workspace, &pane, a, false, false, None, cx);
        });
        assert_item_labels(&pane, ["A*", "B", "C"], cx);
    }

    #[gpui::test]
    async fn test_add_item_with_same_project_entries(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        // singleton view
        workspace.update(cx, |workspace, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 1")
                .with_project_items(&[TestProjectItem::new(1, "one.txt", cx)]);

            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| item)),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with the same project entry
        workspace.update(cx, |workspace, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 1")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| item)),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1*"], cx);

        // new singleton view with different project entry
        workspace.update(cx, |workspace, cx| {
            let item = TestItem::new()
                .with_singleton(true)
                .with_label("buffer 2")
                .with_project_items(&[TestProjectItem::new(2, "2.txt", cx)]);

            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| item)),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2*"], cx);

        // new multibuffer view with the same project entry
        workspace.update(cx, |workspace, cx| {
            let item = TestItem::new()
                .with_singleton(false)
                .with_label("multibuffer 1")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| item)),
                false,
                false,
                None,
                cx,
            );
        });
        assert_item_labels(&pane, ["buffer 1", "buffer 2", "multibuffer 1*"], cx);

        // another multibuffer view with the same project entry
        workspace.update(cx, |workspace, cx| {
            let item = TestItem::new()
                .with_singleton(false)
                .with_label("multibuffer 1b")
                .with_project_items(&[TestProjectItem::new(1, "1.txt", cx)]);

            Pane::add_item(
                workspace,
                &pane,
                Box::new(cx.add_view(|_| item)),
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
    async fn test_remove_item_ordering(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, None, cx).await;
        let (_, workspace) =
            cx.add_window(|cx| Workspace::new(None, 0, project, |_, _| unimplemented!(), cx));
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        add_labled_item(&workspace, &pane, "A", cx);
        add_labled_item(&workspace, &pane, "B", cx);
        add_labled_item(&workspace, &pane, "C", cx);
        add_labled_item(&workspace, &pane, "D", cx);
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(1, false, false, cx));
        add_labled_item(&workspace, &pane, "1", cx);
        assert_item_labels(&pane, ["A", "B", "1*", "C", "D"], cx);

        workspace.update(cx, |workspace, cx| {
            Pane::close_active_item(workspace, &CloseActiveItem, cx);
        });
        deterministic.run_until_parked();
        assert_item_labels(&pane, ["A", "B*", "C", "D"], cx);

        pane.update(cx, |pane, cx| pane.activate_item(3, false, false, cx));
        assert_item_labels(&pane, ["A", "B", "C", "D*"], cx);

        workspace.update(cx, |workspace, cx| {
            Pane::close_active_item(workspace, &CloseActiveItem, cx);
        });
        deterministic.run_until_parked();
        assert_item_labels(&pane, ["A", "B*", "C"], cx);

        workspace.update(cx, |workspace, cx| {
            Pane::close_active_item(workspace, &CloseActiveItem, cx);
        });
        deterministic.run_until_parked();
        assert_item_labels(&pane, ["A", "C*"], cx);

        workspace.update(cx, |workspace, cx| {
            Pane::close_active_item(workspace, &CloseActiveItem, cx);
        });
        deterministic.run_until_parked();
        assert_item_labels(&pane, ["A*"], cx);
    }

    fn add_labled_item(
        workspace: &ViewHandle<Workspace>,
        pane: &ViewHandle<Pane>,
        label: &str,
        cx: &mut TestAppContext,
    ) -> Box<ViewHandle<TestItem>> {
        workspace.update(cx, |workspace, cx| {
            let labeled_item = Box::new(cx.add_view(|_| TestItem::new().with_label(label)));

            Pane::add_item(
                workspace,
                pane,
                labeled_item.clone(),
                false,
                false,
                None,
                cx,
            );

            labeled_item
        })
    }

    fn set_labeled_items<const COUNT: usize>(
        workspace: &ViewHandle<Workspace>,
        pane: &ViewHandle<Pane>,
        labels: [&str; COUNT],
        cx: &mut TestAppContext,
    ) -> [Box<ViewHandle<TestItem>>; COUNT] {
        pane.update(cx, |pane, _| {
            pane.items.clear();
        });

        workspace.update(cx, |workspace, cx| {
            let mut active_item_index = 0;

            let mut index = 0;
            let items = labels.map(|mut label| {
                if label.ends_with("*") {
                    label = label.trim_end_matches("*");
                    active_item_index = index;
                }

                let labeled_item = Box::new(cx.add_view(|_| TestItem::new().with_label(label)));
                Pane::add_item(
                    workspace,
                    pane,
                    labeled_item.clone(),
                    false,
                    false,
                    None,
                    cx,
                );
                index += 1;
                labeled_item
            });

            pane.update(cx, |pane, cx| {
                pane.activate_item(active_item_index, false, false, cx)
            });

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
                        .to_any()
                        .downcast::<TestItem>()
                        .unwrap()
                        .read(cx)
                        .label
                        .clone();
                    if ix == pane.active_item_index {
                        state.push('*');
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
