use super::{ItemHandle, SplitDirection};
use crate::{toolbar::Toolbar, Item, WeakItemHandle, Workspace};
use anyhow::Result;
use collections::{HashMap, HashSet, VecDeque};
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    impl_actions, impl_internal_actions,
    platform::{CursorStyle, NavigationDirection},
    AppContext, AsyncAppContext, Entity, ModelHandle, MutableAppContext, PromptLevel, Quad,
    RenderContext, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use project::{Project, ProjectEntryId, ProjectPath};
use serde::Deserialize;
use settings::Settings;
use std::{any::Any, cell::RefCell, mem, path::Path, rc::Rc};
use util::ResultExt;

actions!(
    pane,
    [
        ActivatePrevItem,
        ActivateNextItem,
        CloseActiveItem,
        CloseInactiveItems,
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

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivateItem(pub usize);

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

impl_actions!(pane, [GoBack, GoForward]);
impl_internal_actions!(pane, [CloseItem, ActivateItem]);

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|pane: &mut Pane, action: &ActivateItem, cx| {
        pane.activate_item(action.0, true, true, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivatePrevItem, cx| {
        pane.activate_prev_item(cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivateNextItem, cx| {
        pane.activate_next_item(cx);
    });
    cx.add_async_action(Pane::close_active_item);
    cx.add_async_action(Pane::close_inactive_items);
    cx.add_async_action(|workspace: &mut Workspace, action: &CloseItem, cx| {
        let pane = action.pane.upgrade(cx)?;
        let task = Pane::close_item(workspace, pane, action.item_id, cx);
        Some(cx.foreground().spawn(async move {
            task.await?;
            Ok(())
        }))
    });
    cx.add_action(|pane: &mut Pane, _: &SplitLeft, cx| pane.split(SplitDirection::Left, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitUp, cx| pane.split(SplitDirection::Up, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitRight, cx| pane.split(SplitDirection::Right, cx));
    cx.add_action(|pane: &mut Pane, _: &SplitDown, cx| pane.split(SplitDirection::Down, cx));
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

pub enum Event {
    Activate,
    ActivateItem { local: bool },
    Remove,
    Split(SplitDirection),
    ChangeItemTitle,
}

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    active_item_index: usize,
    autoscroll: bool,
    nav_history: Rc<RefCell<NavHistory>>,
    toolbar: ViewHandle<Toolbar>,
}

pub struct ItemNavHistory {
    history: Rc<RefCell<NavHistory>>,
    item: Rc<dyn WeakItemHandle>,
}

#[derive(Default)]
pub struct NavHistory {
    mode: NavigationMode,
    backward_stack: VecDeque<NavigationEntry>,
    forward_stack: VecDeque<NavigationEntry>,
    paths_by_item: HashMap<usize, ProjectPath>,
}

#[derive(Copy, Clone)]
enum NavigationMode {
    Normal,
    GoingBack,
    GoingForward,
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

impl Pane {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            items: Vec::new(),
            active_item_index: 0,
            autoscroll: false,
            nav_history: Default::default(),
            toolbar: cx.add_view(|_| Toolbar::new()),
        }
    }

    pub fn nav_history(&self) -> &Rc<RefCell<NavHistory>> {
        &self.nav_history
    }

    pub fn activate(&self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
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

    fn navigate_history(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        mode: NavigationMode,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<()> {
        workspace.activate_pane(pane.clone(), cx);

        let to_load = pane.update(cx, |pane, cx| {
            loop {
                // Retrieve the weak item handle from the history.
                let entry = pane.nav_history.borrow_mut().pop(mode)?;

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
                        .borrow_mut()
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
                    if let Some((project_entry_id, build_item)) = task.log_err() {
                        pane.update(&mut cx, |pane, _| {
                            pane.nav_history.borrow_mut().set_mode(mode);
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
                            pane.nav_history
                                .borrow_mut()
                                .set_mode(NavigationMode::Normal);
                            if let Some(data) = entry.data {
                                item.navigate(data, cx);
                            }
                        });
                    } else {
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
        build_item: impl FnOnce(&mut MutableAppContext) -> Box<dyn ItemHandle>,
    ) -> Box<dyn ItemHandle> {
        let existing_item = pane.update(cx, |pane, cx| {
            for (ix, item) in pane.items.iter().enumerate() {
                if item.project_path(cx).is_some()
                    && item.project_entry_ids(cx).as_slice() == &[project_entry_id]
                {
                    let item = item.boxed_clone();
                    pane.activate_item(ix, true, focus_item, cx);
                    return Some(item);
                }
            }
            None
        });
        if let Some(existing_item) = existing_item {
            existing_item
        } else {
            let item = build_item(cx);
            Self::add_item(workspace, pane, item.boxed_clone(), true, focus_item, cx);
            item
        }
    }

    pub(crate) fn add_item(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        item: Box<dyn ItemHandle>,
        activate_pane: bool,
        focus_item: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        // Prevent adding the same item to the pane more than once.
        if let Some(item_ix) = pane.read(cx).items.iter().position(|i| i.id() == item.id()) {
            pane.update(cx, |pane, cx| {
                pane.activate_item(item_ix, activate_pane, focus_item, cx)
            });
            return;
        }

        item.set_nav_history(pane.read(cx).nav_history.clone(), cx);
        item.added_to_pane(workspace, pane.clone(), cx);
        pane.update(cx, |pane, cx| {
            // If there is already an active item, then insert the new item
            // right after it. Otherwise, adjust the `active_item_index` field
            // before activating the new item, so that in the `activate_item`
            // method, we can detect that the active item is changing.
            let item_ix;
            if pane.active_item_index < pane.items.len() {
                item_ix = pane.active_item_index + 1
            } else {
                item_ix = pane.items.len();
                pane.active_item_index = usize::MAX;
            };

            pane.items.insert(item_ix, item);
            pane.activate_item(item_ix, activate_pane, focus_item, cx);
            cx.notify();
        });
    }

    pub fn items(&self) -> impl Iterator<Item = &Box<dyn ItemHandle>> {
        self.items.iter()
    }

    pub fn items_of_type<'a, T: View>(&'a self) -> impl 'a + Iterator<Item = ViewHandle<T>> {
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
            if item.is_singleton(cx) && item.project_entry_ids(cx).as_slice() == &[entry_id] {
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
            self.update_toolbar(cx);
            if focus_item {
                self.focus_active_item(cx);
            }
            if activate_pane {
                self.activate(cx);
            }
            self.autoscroll = true;
            cx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index > 0 {
            index -= 1;
        } else if self.items.len() > 0 {
            index = self.items.len() - 1;
        }
        self.activate_item(index, true, true, cx);
    }

    pub fn activate_next_item(&mut self, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index + 1 < self.items.len() {
            index += 1;
        } else {
            index = 0;
        }
        self.activate_item(index, true, true, cx);
    }

    pub fn close_active_item(
        workspace: &mut Workspace,
        _: &CloseActiveItem,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let pane_handle = workspace.active_pane().clone();
        let pane = pane_handle.read(cx);
        if pane.items.is_empty() {
            None
        } else {
            let item_id_to_close = pane.items[pane.active_item_index].id();
            let task = Self::close_items(workspace, pane_handle, cx, move |item_id| {
                item_id == item_id_to_close
            });
            Some(cx.foreground().spawn(async move {
                task.await?;
                Ok(())
            }))
        }
    }

    pub fn close_inactive_items(
        workspace: &mut Workspace,
        _: &CloseInactiveItems,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Task<Result<()>>> {
        let pane_handle = workspace.active_pane().clone();
        let pane = pane_handle.read(cx);
        if pane.items.is_empty() {
            None
        } else {
            let active_item_id = pane.items[pane.active_item_index].id();
            let task =
                Self::close_items(workspace, pane_handle, cx, move |id| id != active_item_id);
            Some(cx.foreground().spawn(async move {
                task.await?;
                Ok(())
            }))
        }
    }

    pub fn close_item(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        item_id_to_close: usize,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<bool>> {
        Self::close_items(workspace, pane, cx, move |view_id| {
            view_id == item_id_to_close
        })
    }

    pub fn close_items(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
        should_close: impl 'static + Fn(usize) -> bool,
    ) -> Task<Result<bool>> {
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
            let mut saved_project_entry_ids = HashSet::default();
            for item in items_to_close.clone() {
                // Find the item's current index and its set of project entries. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let (item_ix, mut project_entry_ids) = pane.read_with(&cx, |pane, cx| {
                    (pane.index_for_item(&*item), item.project_entry_ids(cx))
                });
                let item_ix = if let Some(ix) = item_ix {
                    ix
                } else {
                    continue;
                };

                // If an item hasn't yet been associated with a project entry, then always
                // prompt to save it before closing it. Otherwise, check if the item has
                // any project entries that are not open anywhere else in the workspace,
                // AND that the user has not already been prompted to save. If there are
                // any such project entries, prompt the user to save this item.
                let should_save = if project_entry_ids.is_empty() {
                    true
                } else {
                    workspace.read_with(&cx, |workspace, cx| {
                        for item in workspace.items(cx) {
                            if !items_to_close
                                .iter()
                                .any(|item_to_close| item_to_close.id() == item.id())
                            {
                                let other_project_entry_ids = item.project_entry_ids(cx);
                                project_entry_ids
                                    .retain(|id| !other_project_entry_ids.contains(&id));
                            }
                        }
                    });
                    project_entry_ids
                        .iter()
                        .any(|id| saved_project_entry_ids.insert(*id))
                };

                if should_save {
                    if !Self::save_item(project.clone(), &pane, item_ix, &item, true, &mut cx)
                        .await?
                    {
                        break;
                    }
                }

                // Remove the item from the pane.
                pane.update(&mut cx, |pane, cx| {
                    if let Some(item_ix) = pane.items.iter().position(|i| i.id() == item.id()) {
                        if item_ix == pane.active_item_index {
                            if item_ix + 1 < pane.items.len() {
                                pane.activate_next_item(cx);
                            } else if item_ix > 0 {
                                pane.activate_prev_item(cx);
                            }
                        }

                        let item = pane.items.remove(item_ix);
                        if pane.items.is_empty() {
                            item.deactivated(cx);
                            pane.update_toolbar(cx);
                            cx.emit(Event::Remove);
                        }

                        if item_ix < pane.active_item_index {
                            pane.active_item_index -= 1;
                        }

                        let mut nav_history = pane.nav_history.borrow_mut();
                        if let Some(path) = item.project_path(cx) {
                            nav_history.paths_by_item.insert(item.id(), path);
                        } else {
                            nav_history.paths_by_item.remove(&item.id());
                        }
                    }
                });
            }

            pane.update(&mut cx, |_, cx| cx.notify());
            Ok(true)
        })
    }

    pub async fn save_item(
        project: ModelHandle<Project>,
        pane: &ViewHandle<Pane>,
        item_ix: usize,
        item: &Box<dyn ItemHandle>,
        should_prompt_for_save: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<bool> {
        const CONFLICT_MESSAGE: &'static str =
            "This file has changed on disk since you started editing it. Do you want to overwrite it?";
        const DIRTY_MESSAGE: &'static str =
            "This file contains unsaved edits. Do you want to save it?";

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
            let should_save = if should_prompt_for_save {
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
                        .unwrap_or(Path::new("").into());

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

    pub fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            cx.focus(active_item);
        }
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
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

    fn render_tabs(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();

        enum Tabs {}
        enum Tab {}
        let pane = cx.handle();
        let tabs = MouseEventHandler::new::<Tabs, _, _>(0, cx, |mouse_state, cx| {
            let autoscroll = if mem::take(&mut self.autoscroll) {
                Some(self.active_item_index)
            } else {
                None
            };
            let mut row = Flex::row().scrollable::<Tabs, _>(1, autoscroll, cx);
            for (ix, item) in self.items.iter().enumerate() {
                let is_active = ix == self.active_item_index;

                row.add_child({
                    let tab_style = if is_active {
                        theme.workspace.active_tab.clone()
                    } else {
                        theme.workspace.tab.clone()
                    };
                    let title = item.tab_content(&tab_style, cx);

                    let mut style = if is_active {
                        theme.workspace.active_tab.clone()
                    } else {
                        theme.workspace.tab.clone()
                    };
                    if ix == 0 {
                        style.container.border.left = false;
                    }

                    MouseEventHandler::new::<Tab, _, _>(ix, cx, |_, cx| {
                        Container::new(
                            Flex::row()
                                .with_child(
                                    Align::new({
                                        let diameter = 7.0;
                                        let icon_color = if item.has_conflict(cx) {
                                            Some(style.icon_conflict)
                                        } else if item.is_dirty(cx) {
                                            Some(style.icon_dirty)
                                        } else {
                                            None
                                        };

                                        ConstrainedBox::new(
                                            Canvas::new(move |bounds, _, cx| {
                                                if let Some(color) = icon_color {
                                                    let square = RectF::new(
                                                        bounds.origin(),
                                                        vec2f(diameter, diameter),
                                                    );
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
                                                left: style.spacing,
                                                right: style.spacing,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .boxed(),
                                )
                                .with_child(
                                    Align::new(
                                        ConstrainedBox::new(if mouse_state.hovered {
                                            let item_id = item.id();
                                            enum TabCloseButton {}
                                            let icon = Svg::new("icons/x.svg");
                                            MouseEventHandler::new::<TabCloseButton, _, _>(
                                                item_id,
                                                cx,
                                                |mouse_state, _| {
                                                    if mouse_state.hovered {
                                                        icon.with_color(style.icon_close_active)
                                                            .boxed()
                                                    } else {
                                                        icon.with_color(style.icon_close).boxed()
                                                    }
                                                },
                                            )
                                            .with_padding(Padding::uniform(4.))
                                            .with_cursor_style(CursorStyle::PointingHand)
                                            .on_click({
                                                let pane = pane.clone();
                                                move |_, _, cx| {
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
                                        .with_width(style.icon_width)
                                        .boxed(),
                                    )
                                    .boxed(),
                                )
                                .boxed(),
                        )
                        .with_style(style.container)
                        .boxed()
                    })
                    .on_mouse_down(move |_, cx| {
                        cx.dispatch_action(ActivateItem(ix));
                    })
                    .boxed()
                })
            }

            row.add_child(
                Empty::new()
                    .contained()
                    .with_border(theme.workspace.tab.container.border)
                    .flex(0., true)
                    .named("filler"),
            );

            row.boxed()
        });

        ConstrainedBox::new(tabs.boxed())
            .with_height(theme.workspace.tab.height)
            .named("tabs")
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

        EventHandler::new(if let Some(active_item) = self.active_item() {
            Flex::column()
                .with_child(self.render_tabs(cx))
                .with_child(ChildView::new(&self.toolbar).boxed())
                .with_child(ChildView::new(active_item).flex(1., true).boxed())
                .boxed()
        } else {
            Empty::new().boxed()
        })
        .on_navigate_mouse_down(move |direction, cx| {
            let this = this.clone();
            match direction {
                NavigationDirection::Back => cx.dispatch_action(GoBack { pane: Some(this) }),
                NavigationDirection::Forward => cx.dispatch_action(GoForward { pane: Some(this) }),
            }

            true
        })
        .named("pane")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_active_item(cx);
    }
}

impl ItemNavHistory {
    pub fn new<T: Item>(history: Rc<RefCell<NavHistory>>, item: &ViewHandle<T>) -> Self {
        Self {
            history,
            item: Rc::new(item.downgrade()),
        }
    }

    pub fn history(&self) -> Rc<RefCell<NavHistory>> {
        self.history.clone()
    }

    pub fn push<D: 'static + Any>(&self, data: Option<D>) {
        self.history.borrow_mut().push(data, self.item.clone());
    }
}

impl NavHistory {
    pub fn disable(&mut self) {
        self.mode = NavigationMode::Disabled;
    }

    pub fn enable(&mut self) {
        self.mode = NavigationMode::Normal;
    }

    pub fn pop_backward(&mut self) -> Option<NavigationEntry> {
        self.backward_stack.pop_back()
    }

    pub fn pop_forward(&mut self) -> Option<NavigationEntry> {
        self.forward_stack.pop_back()
    }

    fn pop(&mut self, mode: NavigationMode) -> Option<NavigationEntry> {
        match mode {
            NavigationMode::Normal | NavigationMode::Disabled => None,
            NavigationMode::GoingBack => self.pop_backward(),
            NavigationMode::GoingForward => self.pop_forward(),
        }
    }

    fn set_mode(&mut self, mode: NavigationMode) {
        self.mode = mode;
    }

    pub fn push<D: 'static + Any>(&mut self, data: Option<D>, item: Rc<dyn WeakItemHandle>) {
        match self.mode {
            NavigationMode::Disabled => {}
            NavigationMode::Normal => {
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
        }
    }
}
