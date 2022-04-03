use super::{ItemHandle, SplitDirection};
use crate::{toolbar::Toolbar, Item, Settings, WeakItemHandle, Workspace};
use collections::{HashMap, VecDeque};
use futures::StreamExt;
use gpui::{
    action,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::{CursorStyle, NavigationDirection},
    AppContext, Entity, ModelHandle, MutableAppContext, PromptLevel, Quad, RenderContext, Task,
    View, ViewContext, ViewHandle, WeakViewHandle,
};
use project::{Project, ProjectEntryId, ProjectPath};
use std::{any::Any, cell::RefCell, cmp, mem, path::Path, rc::Rc};
use util::ResultExt;

action!(Split, SplitDirection);
action!(ActivateItem, usize);
action!(ActivatePrevItem);
action!(ActivateNextItem);
action!(CloseActiveItem);
action!(CloseInactiveItems);
action!(CloseItem, usize);
action!(GoBack, Option<WeakViewHandle<Pane>>);
action!(GoForward, Option<WeakViewHandle<Pane>>);

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|pane: &mut Pane, action: &ActivateItem, cx| {
        pane.activate_item(action.0, true, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivatePrevItem, cx| {
        pane.activate_prev_item(cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivateNextItem, cx| {
        pane.activate_next_item(cx);
    });
    cx.add_action(|pane: &mut Pane, _: &CloseActiveItem, cx| {
        pane.close_active_item(cx).detach();
    });
    cx.add_action(|pane: &mut Pane, _: &CloseInactiveItems, cx| {
        pane.close_inactive_items(cx).detach();
    });
    cx.add_action(|pane: &mut Pane, action: &CloseItem, cx| {
        pane.close_item(action.0, cx).detach();
    });
    cx.add_action(|pane: &mut Pane, action: &Split, cx| {
        pane.split(action.0, cx);
    });
    cx.add_action(|workspace: &mut Workspace, action: &GoBack, cx| {
        Pane::go_back(
            workspace,
            action
                .0
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
                .0
                .as_ref()
                .and_then(|weak_handle| weak_handle.upgrade(cx)),
            cx,
        )
        .detach();
    });

    cx.add_bindings(vec![
        Binding::new("shift-cmd-{", ActivatePrevItem, Some("Pane")),
        Binding::new("shift-cmd-}", ActivateNextItem, Some("Pane")),
        Binding::new("cmd-w", CloseActiveItem, Some("Pane")),
        Binding::new("alt-cmd-w", CloseInactiveItems, Some("Pane")),
        Binding::new("cmd-k up", Split(SplitDirection::Up), Some("Pane")),
        Binding::new("cmd-k down", Split(SplitDirection::Down), Some("Pane")),
        Binding::new("cmd-k left", Split(SplitDirection::Left), Some("Pane")),
        Binding::new("cmd-k right", Split(SplitDirection::Right), Some("Pane")),
        Binding::new("ctrl--", GoBack(None), Some("Pane")),
        Binding::new("shift-ctrl-_", GoForward(None), Some("Pane")),
    ]);
}

pub enum Event {
    Activate,
    ActivateItem { local: bool },
    Remove,
    Split(SplitDirection),
}

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    active_item_index: usize,
    nav_history: Rc<RefCell<NavHistory>>,
    toolbar: ViewHandle<Toolbar>,
    project: ModelHandle<Project>,
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
    pub fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            items: Vec::new(),
            active_item_index: 0,
            nav_history: Default::default(),
            toolbar: cx.add_view(|_| Toolbar::new()),
            project,
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
                    if let Some(item) = pane.active_item() {
                        pane.nav_history.borrow_mut().set_mode(mode);
                        item.deactivated(cx);
                        pane.nav_history
                            .borrow_mut()
                            .set_mode(NavigationMode::Normal);
                    }

                    let prev_active_index = mem::replace(&mut pane.active_item_index, index);
                    pane.focus_active_item(cx);
                    let mut navigated = prev_active_index != pane.active_item_index;
                    if let Some(data) = entry.data {
                        navigated |= pane.active_item()?.navigate(data, cx);
                    }

                    if navigated {
                        cx.notify();
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
        cx: &mut ViewContext<Workspace>,
        build_item: impl FnOnce(&mut MutableAppContext) -> Box<dyn ItemHandle>,
    ) -> Box<dyn ItemHandle> {
        let existing_item = pane.update(cx, |pane, cx| {
            for (ix, item) in pane.items.iter().enumerate() {
                if item.project_entry_id(cx) == Some(project_entry_id) {
                    let item = item.boxed_clone();
                    pane.activate_item(ix, true, cx);
                    return Some(item);
                }
            }
            None
        });
        if let Some(existing_item) = existing_item {
            existing_item
        } else {
            let item = build_item(cx);
            Self::add_item(workspace, pane, item.boxed_clone(), true, cx);
            item
        }
    }

    pub(crate) fn add_item(
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        item: Box<dyn ItemHandle>,
        local: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        // Prevent adding the same item to the pane more than once.
        if let Some(item_ix) = pane.read(cx).items.iter().position(|i| i.id() == item.id()) {
            pane.update(cx, |pane, cx| pane.activate_item(item_ix, local, cx));
            return;
        }

        item.set_nav_history(pane.read(cx).nav_history.clone(), cx);
        item.added_to_pane(workspace, pane.clone(), cx);
        pane.update(cx, |pane, cx| {
            let item_idx = cmp::min(pane.active_item_index + 1, pane.items.len());
            pane.items.insert(item_idx, item);
            pane.activate_item(item_idx, local, cx);
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

    pub fn project_entry_id_for_item(
        &self,
        item: &dyn ItemHandle,
        cx: &AppContext,
    ) -> Option<ProjectEntryId> {
        self.items.iter().find_map(|existing| {
            if existing.id() == item.id() {
                existing.project_entry_id(cx)
            } else {
                None
            }
        })
    }

    pub fn item_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Box<dyn ItemHandle>> {
        self.items.iter().find_map(|item| {
            if item.project_entry_id(cx) == Some(entry_id) {
                Some(item.boxed_clone())
            } else {
                None
            }
        })
    }

    pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize> {
        self.items.iter().position(|i| i.id() == item.id())
    }

    pub fn activate_item(&mut self, index: usize, local: bool, cx: &mut ViewContext<Self>) {
        if index < self.items.len() {
            let prev_active_item_ix = mem::replace(&mut self.active_item_index, index);
            if prev_active_item_ix != self.active_item_index
                && prev_active_item_ix < self.items.len()
            {
                self.items[prev_active_item_ix].deactivated(cx);
                cx.emit(Event::ActivateItem { local });
            }
            self.update_toolbar(cx);
            if local {
                self.focus_active_item(cx);
                self.activate(cx);
            }
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
        self.activate_item(index, true, cx);
    }

    pub fn activate_next_item(&mut self, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index + 1 < self.items.len() {
            index += 1;
        } else {
            index = 0;
        }
        self.activate_item(index, true, cx);
    }

    pub fn close_active_item(&mut self, cx: &mut ViewContext<Self>) -> Task<()> {
        if self.items.is_empty() {
            Task::ready(())
        } else {
            self.close_item(self.items[self.active_item_index].id(), cx)
        }
    }

    pub fn close_inactive_items(&mut self, cx: &mut ViewContext<Self>) -> Task<()> {
        if self.items.is_empty() {
            Task::ready(())
        } else {
            let active_item_id = self.items[self.active_item_index].id();
            self.close_items(cx, move |id| id != active_item_id)
        }
    }

    pub fn close_item(&mut self, view_id_to_close: usize, cx: &mut ViewContext<Self>) -> Task<()> {
        self.close_items(cx, move |view_id| view_id == view_id_to_close)
    }

    pub fn close_items(
        &mut self,
        cx: &mut ViewContext<Self>,
        should_close: impl 'static + Fn(usize) -> bool,
    ) -> Task<()> {
        const CONFLICT_MESSAGE: &'static str = "This file has changed on disk since you started editing it. Do you want to overwrite it?";
        const DIRTY_MESSAGE: &'static str =
            "This file contains unsaved edits. Do you want to save it?";

        let project = self.project.clone();
        cx.spawn(|this, mut cx| async move {
            while let Some(item_to_close_ix) = this.read_with(&cx, |this, _| {
                this.items.iter().position(|item| should_close(item.id()))
            }) {
                let item =
                    this.read_with(&cx, |this, _| this.items[item_to_close_ix].boxed_clone());
                if cx.read(|cx| item.is_dirty(cx)) {
                    if cx.read(|cx| item.can_save(cx)) {
                        let mut answer = this.update(&mut cx, |this, cx| {
                            this.activate_item(item_to_close_ix, true, cx);
                            cx.prompt(
                                PromptLevel::Warning,
                                DIRTY_MESSAGE,
                                &["Save", "Don't Save", "Cancel"],
                            )
                        });

                        match answer.next().await {
                            Some(0) => {
                                if cx
                                    .update(|cx| item.save(project.clone(), cx))
                                    .await
                                    .log_err()
                                    .is_none()
                                {
                                    break;
                                }
                            }
                            Some(1) => {}
                            _ => break,
                        }
                    } else if cx.read(|cx| item.can_save_as(cx)) {
                        let mut answer = this.update(&mut cx, |this, cx| {
                            this.activate_item(item_to_close_ix, true, cx);
                            cx.prompt(
                                PromptLevel::Warning,
                                DIRTY_MESSAGE,
                                &["Save", "Don't Save", "Cancel"],
                            )
                        });

                        match answer.next().await {
                            Some(0) => {
                                let start_abs_path = project
                                    .read_with(&cx, |project, cx| {
                                        let worktree = project.visible_worktrees(cx).next()?;
                                        Some(worktree.read(cx).as_local()?.abs_path().to_path_buf())
                                    })
                                    .unwrap_or(Path::new("").into());

                                let mut abs_path =
                                    cx.update(|cx| cx.prompt_for_new_path(&start_abs_path));
                                if let Some(abs_path) = abs_path.next().await.flatten() {
                                    if cx
                                        .update(|cx| item.save_as(project.clone(), abs_path, cx))
                                        .await
                                        .log_err()
                                        .is_none()
                                    {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            Some(1) => {}
                            _ => break,
                        }
                    }
                } else if cx.read(|cx| item.has_conflict(cx) && item.can_save(cx)) {
                    let mut answer = this.update(&mut cx, |this, cx| {
                        this.activate_item(item_to_close_ix, true, cx);
                        cx.prompt(
                            PromptLevel::Warning,
                            CONFLICT_MESSAGE,
                            &["Overwrite", "Discard", "Cancel"],
                        )
                    });

                    match answer.next().await {
                        Some(0) => {
                            if cx
                                .update(|cx| item.save(project.clone(), cx))
                                .await
                                .log_err()
                                .is_none()
                            {
                                break;
                            }
                        }
                        Some(1) => {
                            if cx
                                .update(|cx| item.reload(project.clone(), cx))
                                .await
                                .log_err()
                                .is_none()
                            {
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                this.update(&mut cx, |this, cx| {
                    if let Some(item_ix) = this.items.iter().position(|i| i.id() == item.id()) {
                        if item_ix == this.active_item_index {
                            if item_ix + 1 < this.items.len() {
                                this.activate_next_item(cx);
                            } else if item_ix > 0 {
                                this.activate_prev_item(cx);
                            }
                        }

                        let item = this.items.remove(item_ix);
                        if this.items.is_empty() {
                            item.deactivated(cx);
                            cx.emit(Event::Remove);
                        }

                        if item_ix < this.active_item_index {
                            this.active_item_index -= 1;
                        }

                        let mut nav_history = this.nav_history.borrow_mut();
                        if let Some(path) = item.project_path(cx) {
                            nav_history.paths_by_item.insert(item.id(), path);
                        } else {
                            nav_history.paths_by_item.remove(&item.id());
                        }
                    }
                });
            }

            this.update(&mut cx, |_, cx| cx.notify());
        })
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

    fn render_tabs(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();

        enum Tabs {}
        let tabs = MouseEventHandler::new::<Tabs, _, _>(0, cx, |mouse_state, cx| {
            let mut row = Flex::row();
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

                    EventHandler::new(
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
                                            .on_click(move |cx| {
                                                cx.dispatch_action(CloseItem(item_id))
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
                        .boxed(),
                    )
                    .on_mouse_down(move |cx| {
                        cx.dispatch_action(ActivateItem(ix));
                        true
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
                NavigationDirection::Back => cx.dispatch_action(GoBack(Some(this))),
                NavigationDirection::Forward => cx.dispatch_action(GoForward(Some(this))),
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

#[cfg(test)]
mod tests {
    use crate::WorkspaceParams;

    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_close_items(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        let params = cx.update(WorkspaceParams::test);
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        let item1 = cx.add_view(window_id, |_| {
            let mut item = TestItem::new();
            item.has_conflict = true;
            item
        });
        let item2 = cx.add_view(window_id, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.has_conflict = true;
            item
        });
        let item3 = cx.add_view(window_id, |_| {
            let mut item = TestItem::new();
            item.has_conflict = true;
            item
        });
        let item4 = cx.add_view(window_id, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item
        });
        let item5 = cx.add_view(window_id, |_| {
            let mut item = TestItem::new();
            item.is_dirty = true;
            item.can_save = false;
            item
        });
        let pane = workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(item1.clone()), cx);
            workspace.add_item(Box::new(item2.clone()), cx);
            workspace.add_item(Box::new(item3.clone()), cx);
            workspace.add_item(Box::new(item4.clone()), cx);
            workspace.add_item(Box::new(item5.clone()), cx);
            workspace.active_pane().clone()
        });

        let close_items = pane.update(cx, |pane, cx| {
            pane.activate_item(1, true, cx);
            assert_eq!(pane.active_item().unwrap().id(), item2.id());

            let item1_id = item1.id();
            let item3_id = item3.id();
            let item4_id = item4.id();
            let item5_id = item5.id();
            pane.close_items(cx, move |id| {
                [item1_id, item3_id, item4_id, item5_id].contains(&id)
            })
        });

        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, _| {
            assert_eq!(pane.items.len(), 5);
            assert_eq!(pane.active_item().unwrap().id(), item1.id());
        });

        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item1.read(cx).save_count, 1);
            assert_eq!(item1.read(cx).save_as_count, 0);
            assert_eq!(item1.read(cx).reload_count, 0);
            assert_eq!(pane.items.len(), 4);
            assert_eq!(pane.active_item().unwrap().id(), item3.id());
        });

        cx.simulate_prompt_answer(window_id, 1);
        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item3.read(cx).save_count, 0);
            assert_eq!(item3.read(cx).save_as_count, 0);
            assert_eq!(item3.read(cx).reload_count, 1);
            assert_eq!(pane.items.len(), 3);
            assert_eq!(pane.active_item().unwrap().id(), item4.id());
        });

        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item4.read(cx).save_count, 1);
            assert_eq!(item4.read(cx).save_as_count, 0);
            assert_eq!(item4.read(cx).reload_count, 0);
            assert_eq!(pane.items.len(), 2);
            assert_eq!(pane.active_item().unwrap().id(), item5.id());
        });

        cx.simulate_prompt_answer(window_id, 0);
        cx.foreground().run_until_parked();
        cx.simulate_new_path_selection(|_| Some(Default::default()));
        close_items.await;
        pane.read_with(cx, |pane, cx| {
            assert_eq!(item5.read(cx).save_count, 0);
            assert_eq!(item5.read(cx).save_as_count, 1);
            assert_eq!(item5.read(cx).reload_count, 0);
            assert_eq!(pane.items.len(), 1);
            assert_eq!(pane.active_item().unwrap().id(), item2.id());
        });
    }

    struct TestItem {
        save_count: usize,
        save_as_count: usize,
        reload_count: usize,
        is_dirty: bool,
        has_conflict: bool,
        can_save: bool,
    }

    impl TestItem {
        fn new() -> Self {
            Self {
                save_count: 0,
                save_as_count: 0,
                reload_count: 0,
                is_dirty: false,
                has_conflict: false,
                can_save: true,
            }
        }
    }

    impl Entity for TestItem {
        type Event = ();
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
        fn tab_content(&self, _: &theme::Tab, _: &AppContext) -> ElementBox {
            Empty::new().boxed()
        }

        fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
            None
        }

        fn project_entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
            None
        }

        fn set_nav_history(&mut self, _: ItemNavHistory, _: &mut ViewContext<Self>) {}

        fn is_dirty(&self, _: &AppContext) -> bool {
            self.is_dirty
        }

        fn has_conflict(&self, _: &AppContext) -> bool {
            self.has_conflict
        }

        fn can_save(&self, _: &AppContext) -> bool {
            self.can_save
        }

        fn save(
            &mut self,
            _: ModelHandle<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_count += 1;
            Task::ready(Ok(()))
        }

        fn can_save_as(&self, _: &AppContext) -> bool {
            true
        }

        fn save_as(
            &mut self,
            _: ModelHandle<Project>,
            _: std::path::PathBuf,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.save_as_count += 1;
            Task::ready(Ok(()))
        }

        fn reload(
            &mut self,
            _: ModelHandle<Project>,
            _: &mut ViewContext<Self>,
        ) -> Task<anyhow::Result<()>> {
            self.reload_count += 1;
            Task::ready(Ok(()))
        }
    }
}
