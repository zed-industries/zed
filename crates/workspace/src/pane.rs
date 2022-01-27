use super::{ItemViewHandle, SplitDirection};
use crate::{ItemHandle, ItemView, Settings, WeakItemViewHandle, Workspace};
use collections::{HashMap, VecDeque};
use gpui::{
    action,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::CursorStyle,
    AnyViewHandle, Entity, MutableAppContext, Quad, RenderContext, Task, View, ViewContext,
    ViewHandle,
};
use postage::watch;
use project::ProjectPath;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    cmp, mem,
    rc::Rc,
};
use util::ResultExt;

action!(Split, SplitDirection);
action!(ActivateItem, usize);
action!(ActivatePrevItem);
action!(ActivateNextItem);
action!(CloseActiveItem);
action!(CloseItem, usize);
action!(GoBack);
action!(GoForward);

const MAX_NAVIGATION_HISTORY_LEN: usize = 1024;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|pane: &mut Pane, action: &ActivateItem, cx| {
        pane.activate_item(action.0, cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivatePrevItem, cx| {
        pane.activate_prev_item(cx);
    });
    cx.add_action(|pane: &mut Pane, _: &ActivateNextItem, cx| {
        pane.activate_next_item(cx);
    });
    cx.add_action(|pane: &mut Pane, _: &CloseActiveItem, cx| {
        pane.close_active_item(cx);
    });
    cx.add_action(|pane: &mut Pane, action: &CloseItem, cx| {
        pane.close_item(action.0, cx);
    });
    cx.add_action(|pane: &mut Pane, action: &Split, cx| {
        pane.split(action.0, cx);
    });
    cx.add_action(|workspace: &mut Workspace, _: &GoBack, cx| {
        Pane::go_back(workspace, cx).detach();
    });
    cx.add_action(|workspace: &mut Workspace, _: &GoForward, cx| {
        Pane::go_forward(workspace, cx).detach();
    });

    cx.add_bindings(vec![
        Binding::new("shift-cmd-{", ActivatePrevItem, Some("Pane")),
        Binding::new("shift-cmd-}", ActivateNextItem, Some("Pane")),
        Binding::new("cmd-w", CloseActiveItem, Some("Pane")),
        Binding::new("cmd-k up", Split(SplitDirection::Up), Some("Pane")),
        Binding::new("cmd-k down", Split(SplitDirection::Down), Some("Pane")),
        Binding::new("cmd-k left", Split(SplitDirection::Left), Some("Pane")),
        Binding::new("cmd-k right", Split(SplitDirection::Right), Some("Pane")),
        Binding::new("ctrl--", GoBack, Some("Pane")),
        Binding::new("shift-ctrl-_", GoForward, Some("Pane")),
    ]);
}

pub enum Event {
    Activate,
    Remove,
    Split(SplitDirection),
}

pub struct Pane {
    item_views: Vec<(usize, Box<dyn ItemViewHandle>)>,
    active_item_index: usize,
    settings: watch::Receiver<Settings>,
    nav_history: Rc<RefCell<NavHistory>>,
    toolbars: HashMap<TypeId, Box<dyn ToolbarHandle>>,
    active_toolbar_type: Option<TypeId>,
    active_toolbar_visible: bool,
}

pub trait Toolbar: View {
    fn active_item_changed(
        &mut self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> bool;
}

trait ToolbarHandle {
    fn active_item_changed(
        &self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut MutableAppContext,
    ) -> bool;
    fn to_any(&self) -> AnyViewHandle;
}

pub struct ItemNavHistory {
    history: Rc<RefCell<NavHistory>>,
    item_view: Rc<dyn WeakItemViewHandle>,
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
}

impl Default for NavigationMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct NavigationEntry {
    pub item_view: Rc<dyn WeakItemViewHandle>,
    pub data: Option<Box<dyn Any>>,
}

impl Pane {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            item_views: Vec::new(),
            active_item_index: 0,
            settings,
            nav_history: Default::default(),
            toolbars: Default::default(),
            active_toolbar_type: Default::default(),
            active_toolbar_visible: false,
        }
    }

    pub fn activate(&self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
    }

    pub fn go_back(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Task<()> {
        Self::navigate_history(
            workspace,
            workspace.active_pane().clone(),
            NavigationMode::GoingBack,
            cx,
        )
    }

    pub fn go_forward(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Task<()> {
        Self::navigate_history(
            workspace,
            workspace.active_pane().clone(),
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
        let to_load = pane.update(cx, |pane, cx| {
            // Retrieve the weak item handle from the history.
            let entry = pane.nav_history.borrow_mut().pop(mode)?;

            // If the item is still present in this pane, then activate it.
            if let Some(index) = entry
                .item_view
                .upgrade(cx)
                .and_then(|v| pane.index_for_item_view(v.as_ref()))
            {
                if let Some(item_view) = pane.active_item() {
                    pane.nav_history.borrow_mut().set_mode(mode);
                    item_view.deactivated(cx);
                    pane.nav_history
                        .borrow_mut()
                        .set_mode(NavigationMode::Normal);
                }

                pane.active_item_index = index;
                pane.focus_active_item(cx);
                if let Some(data) = entry.data {
                    pane.active_item()?.navigate(data, cx);
                }
                cx.notify();
                None
            }
            // If the item is no longer present in this pane, then retrieve its
            // project path in order to reopen it.
            else {
                pane.nav_history
                    .borrow_mut()
                    .paths_by_item
                    .get(&entry.item_view.id())
                    .cloned()
                    .map(|project_path| (project_path, entry))
            }
        });

        if let Some((project_path, entry)) = to_load {
            // If the item was no longer present, then load it again from its previous path.
            let pane = pane.downgrade();
            let task = workspace.load_path(project_path, cx);
            cx.spawn(|workspace, mut cx| async move {
                let item = task.await;
                if let Some(pane) = cx.read(|cx| pane.upgrade(cx)) {
                    if let Some(item) = item.log_err() {
                        workspace.update(&mut cx, |workspace, cx| {
                            pane.update(cx, |p, _| p.nav_history.borrow_mut().set_mode(mode));
                            let item_view = workspace.open_item_in_pane(item, &pane, cx);
                            pane.update(cx, |p, _| {
                                p.nav_history.borrow_mut().set_mode(NavigationMode::Normal)
                            });

                            if let Some(data) = entry.data {
                                item_view.navigate(data, cx);
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

    pub fn open_item<T>(
        &mut self,
        item_handle: T,
        workspace: &Workspace,
        cx: &mut ViewContext<Self>,
    ) -> Box<dyn ItemViewHandle>
    where
        T: 'static + ItemHandle,
    {
        for (ix, (item_id, item_view)) in self.item_views.iter().enumerate() {
            if *item_id == item_handle.id() {
                let item_view = item_view.boxed_clone();
                self.activate_item(ix, cx);
                return item_view;
            }
        }

        let item_view =
            item_handle.add_view(cx.window_id(), workspace, self.nav_history.clone(), cx);
        self.add_item_view(item_view.boxed_clone(), cx);
        item_view
    }

    pub fn add_item_view(
        &mut self,
        mut item_view: Box<dyn ItemViewHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        item_view.added_to_pane(cx);
        let item_idx = cmp::min(self.active_item_index + 1, self.item_views.len());
        self.item_views
            .insert(item_idx, (item_view.item_handle(cx).id(), item_view));
        self.activate_item(item_idx, cx);
        cx.notify();
    }

    pub fn contains_item(&self, item: &dyn ItemHandle) -> bool {
        let item_id = item.id();
        self.item_views
            .iter()
            .any(|(existing_item_id, _)| *existing_item_id == item_id)
    }

    pub fn item_views(&self) -> impl Iterator<Item = &Box<dyn ItemViewHandle>> {
        self.item_views.iter().map(|(_, view)| view)
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemViewHandle>> {
        self.item_views
            .get(self.active_item_index)
            .map(|(_, view)| view.clone())
    }

    pub fn index_for_item_view(&self, item_view: &dyn ItemViewHandle) -> Option<usize> {
        self.item_views
            .iter()
            .position(|(_, i)| i.id() == item_view.id())
    }

    pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize> {
        self.item_views.iter().position(|(id, _)| *id == item.id())
    }

    pub fn activate_item(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if index < self.item_views.len() {
            let prev_active_item_ix = mem::replace(&mut self.active_item_index, index);
            if prev_active_item_ix != self.active_item_index {
                self.item_views[prev_active_item_ix].1.deactivated(cx);
            }
            self.update_active_toolbar(cx);
            self.focus_active_item(cx);
            cx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index > 0 {
            index -= 1;
        } else if self.item_views.len() > 0 {
            index = self.item_views.len() - 1;
        }
        self.activate_item(index, cx);
    }

    pub fn activate_next_item(&mut self, cx: &mut ViewContext<Self>) {
        let mut index = self.active_item_index;
        if index + 1 < self.item_views.len() {
            index += 1;
        } else {
            index = 0;
        }
        self.activate_item(index, cx);
    }

    pub fn close_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if !self.item_views.is_empty() {
            self.close_item(self.item_views[self.active_item_index].1.id(), cx)
        }
    }

    pub fn close_item(&mut self, item_view_id: usize, cx: &mut ViewContext<Self>) {
        let mut item_ix = 0;
        self.item_views.retain(|(_, item_view)| {
            if item_view.id() == item_view_id {
                if item_ix == self.active_item_index {
                    item_view.deactivated(cx);
                }

                let mut nav_history = self.nav_history.borrow_mut();
                if let Some(path) = item_view.project_path(cx) {
                    nav_history.paths_by_item.insert(item_view.id(), path);
                } else {
                    nav_history.paths_by_item.remove(&item_view.id());
                }

                item_ix += 1;
                false
            } else {
                item_ix += 1;
                true
            }
        });
        self.activate_item(
            cmp::min(
                self.active_item_index,
                self.item_views.len().saturating_sub(1),
            ),
            cx,
        );

        if self.item_views.is_empty() {
            self.update_active_toolbar(cx);
            cx.emit(Event::Remove);
        }
    }

    fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            cx.focus(active_item);
        }
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
    }

    pub fn show_toolbar<F, V>(&mut self, cx: &mut ViewContext<Self>, build_toolbar: F)
    where
        F: FnOnce(&mut ViewContext<V>) -> V,
        V: Toolbar,
    {
        let type_id = TypeId::of::<V>();
        let active_item = self.active_item();
        self.toolbars
            .entry(type_id)
            .or_insert_with(|| Box::new(cx.add_view(build_toolbar)));
        self.active_toolbar_type = Some(type_id);
        self.active_toolbar_visible = self.toolbars[&type_id].active_item_changed(active_item, cx);
        cx.notify();
    }

    pub fn hide_toolbar(&mut self, cx: &mut ViewContext<Self>) {
        self.active_toolbar_type = None;
        self.active_toolbar_visible = false;
        self.focus_active_item(cx);
        cx.notify();
    }

    pub fn active_toolbar(&self) -> Option<AnyViewHandle> {
        let type_id = self.active_toolbar_type?;
        let toolbar = self.toolbars.get(&type_id)?;
        if self.active_toolbar_visible {
            Some(toolbar.to_any())
        } else {
            None
        }
    }

    fn update_active_toolbar(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(type_id) = self.active_toolbar_type {
            if let Some(toolbar) = self.toolbars.get(&type_id) {
                self.active_toolbar_visible = toolbar.active_item_changed(
                    Some(self.item_views[self.active_item_index].1.clone()),
                    cx,
                );
            }
        }
    }

    fn render_tabs(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme;

        enum Tabs {}
        let tabs = MouseEventHandler::new::<Tabs, _, _, _>(cx.view_id(), cx, |mouse_state, cx| {
            let mut row = Flex::row();
            for (ix, (_, item_view)) in self.item_views.iter().enumerate() {
                let is_active = ix == self.active_item_index;

                row.add_child({
                    let tab_style = if is_active {
                        theme.workspace.active_tab.clone()
                    } else {
                        theme.workspace.tab.clone()
                    };
                    let title = item_view.tab_content(&tab_style, cx);

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
                                        let icon_color = if item_view.has_conflict(cx) {
                                            Some(style.icon_conflict)
                                        } else if item_view.is_dirty(cx) {
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
                                            let item_id = item_view.id();
                                            enum TabCloseButton {}
                                            let icon = Svg::new("icons/x.svg");
                                            MouseEventHandler::new::<TabCloseButton, _, _, _>(
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
                    .flexible(0., true)
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
        if let Some(active_item) = self.active_item() {
            Flex::column()
                .with_child(self.render_tabs(cx))
                .with_children(
                    self.active_toolbar()
                        .as_ref()
                        .map(|view| ChildView::new(view).boxed()),
                )
                .with_child(ChildView::new(active_item).flexible(1., true).boxed())
                .named("pane")
        } else {
            Empty::new().named("pane")
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_active_item(cx);
    }
}

impl<T: Toolbar> ToolbarHandle for ViewHandle<T> {
    fn active_item_changed(
        &self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut MutableAppContext,
    ) -> bool {
        self.update(cx, |this, cx| this.active_item_changed(item, cx))
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }
}

impl ItemNavHistory {
    pub fn new<T: ItemView>(history: Rc<RefCell<NavHistory>>, item_view: &ViewHandle<T>) -> Self {
        Self {
            history,
            item_view: Rc::new(item_view.downgrade()),
        }
    }

    pub fn history(&self) -> Rc<RefCell<NavHistory>> {
        self.history.clone()
    }

    pub fn push<D: 'static + Any>(&self, data: Option<D>) {
        self.history.borrow_mut().push(data, self.item_view.clone());
    }
}

impl NavHistory {
    pub fn pop_backward(&mut self) -> Option<NavigationEntry> {
        self.backward_stack.pop_back()
    }

    pub fn pop_forward(&mut self) -> Option<NavigationEntry> {
        self.forward_stack.pop_back()
    }

    fn pop(&mut self, mode: NavigationMode) -> Option<NavigationEntry> {
        match mode {
            NavigationMode::Normal => None,
            NavigationMode::GoingBack => self.pop_backward(),
            NavigationMode::GoingForward => self.pop_forward(),
        }
    }

    fn set_mode(&mut self, mode: NavigationMode) {
        self.mode = mode;
    }

    pub fn push<D: 'static + Any>(
        &mut self,
        data: Option<D>,
        item_view: Rc<dyn WeakItemViewHandle>,
    ) {
        match self.mode {
            NavigationMode::Normal => {
                if self.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.backward_stack.pop_front();
                }
                self.backward_stack.push_back(NavigationEntry {
                    item_view,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
                self.forward_stack.clear();
            }
            NavigationMode::GoingBack => {
                if self.forward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.forward_stack.pop_front();
                }
                self.forward_stack.push_back(NavigationEntry {
                    item_view,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
            NavigationMode::GoingForward => {
                if self.backward_stack.len() >= MAX_NAVIGATION_HISTORY_LEN {
                    self.backward_stack.pop_front();
                }
                self.backward_stack.push_back(NavigationEntry {
                    item_view,
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
        }
    }
}
