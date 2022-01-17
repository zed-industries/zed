use super::{ItemViewHandle, SplitDirection};
use crate::{ItemHandle, ItemView, Settings, WeakItemViewHandle, Workspace};
use collections::HashMap;
use gpui::{
    action,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::CursorStyle,
    Entity, MutableAppContext, Quad, RenderContext, View, ViewContext,
};
use postage::watch;
use project::ProjectPath;
use std::{any::Any, cell::RefCell, cmp, mem, rc::Rc};
use util::TryFutureExt;

action!(Split, SplitDirection);
action!(ActivateItem, usize);
action!(ActivatePrevItem);
action!(ActivateNextItem);
action!(CloseActiveItem);
action!(CloseItem, usize);
action!(GoBack);
action!(GoForward);

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
    cx.add_action(Pane::go_back);
    cx.add_action(Pane::go_forward);

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

const MAX_TAB_TITLE_LEN: usize = 24;

pub struct Pane {
    item_views: Vec<(usize, Box<dyn ItemViewHandle>)>,
    active_item_index: usize,
    settings: watch::Receiver<Settings>,
    navigation: Rc<Navigation>,
}

#[derive(Default)]
pub struct Navigation(RefCell<NavigationHistory>);

#[derive(Default)]
struct NavigationHistory {
    mode: NavigationHistoryMode,
    backward_stack: Vec<NavigationEntry>,
    forward_stack: Vec<NavigationEntry>,
    paths_by_item: HashMap<usize, ProjectPath>,
}

enum NavigationHistoryMode {
    Normal,
    GoingBack,
    GoingForward,
}

impl Default for NavigationHistoryMode {
    fn default() -> Self {
        Self::Normal
    }
}

struct NavigationEntry {
    item_view: Box<dyn WeakItemViewHandle>,
    data: Option<Box<dyn Any>>,
}

impl Pane {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            item_views: Vec::new(),
            active_item_index: 0,
            settings,
            navigation: Default::default(),
        }
    }

    pub fn activate(&self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
    }

    pub fn go_back(workspace: &mut Workspace, _: &GoBack, cx: &mut ViewContext<Workspace>) {
        let project_path = workspace.active_pane().update(cx, |pane, cx| {
            let mut navigation = pane.navigation.0.borrow_mut();
            if let Some(entry) = navigation.backward_stack.pop() {
                if let Some(index) = entry
                    .item_view
                    .upgrade(cx)
                    .and_then(|v| pane.index_for_item_view(v.as_ref()))
                {
                    if let Some(item_view) = pane.active_item() {
                        pane.navigation.0.borrow_mut().mode = NavigationHistoryMode::GoingBack;
                        item_view.deactivated(cx);
                        pane.navigation.0.borrow_mut().mode = NavigationHistoryMode::Normal;
                    }

                    pane.active_item_index = index;
                    drop(navigation);
                    pane.focus_active_item(cx);
                    cx.notify();
                } else {
                    return navigation.paths_by_item.get(&entry.item_view.id()).cloned();
                }
            }

            None
        });

        if let Some(project_path) = project_path {
            let task = workspace.load_path(project_path, cx);
            cx.spawn(|workspace, mut cx| {
                async move {
                    let item = task.await?;
                    workspace.update(&mut cx, |workspace, cx| {
                        let pane = workspace.active_pane().clone();
                        pane.update(cx, |pane, cx| {
                            pane.navigation.0.borrow_mut().mode = NavigationHistoryMode::GoingBack;
                            pane.open_item(item, workspace, cx);
                            pane.navigation.0.borrow_mut().mode = NavigationHistoryMode::Normal;
                        });
                    });
                    Ok(())
                }
                .log_err()
            })
            .detach();
        }
    }

    pub fn go_forward(&mut self, _: &GoForward, cx: &mut ViewContext<Self>) {
        if self.navigation.0.borrow().forward_stack.is_empty() {
            return;
        }

        if let Some(item_view) = self.active_item() {
            self.navigation.0.borrow_mut().mode = NavigationHistoryMode::GoingForward;
            item_view.deactivated(cx);
            self.navigation.0.borrow_mut().mode = NavigationHistoryMode::Normal;
        }

        let mut navigation = self.navigation.0.borrow_mut();
        if let Some(entry) = navigation.forward_stack.pop() {
            if let Some(index) = entry
                .item_view
                .upgrade(cx)
                .and_then(|v| self.index_for_item_view(v.as_ref()))
            {
                self.active_item_index = index;
                drop(navigation);
                self.focus_active_item(cx);
                cx.notify();
            }
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
            item_handle.add_view(cx.window_id(), workspace, self.navigation.clone(), cx);
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
                self.focus_active_item(cx);
                cx.notify();
            }
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
                let mut navigation = self.navigation.0.borrow_mut();
                if let Some(path) = item_view.project_path(cx) {
                    navigation.paths_by_item.insert(item_view.id(), path);
                } else {
                    navigation.paths_by_item.remove(&item_view.id());
                }

                if item_ix == self.active_item_index {
                    item_view.deactivated(cx);
                }
                item_ix += 1;
                false
            } else {
                item_ix += 1;
                true
            }
        });
        self.active_item_index = cmp::min(
            self.active_item_index,
            self.item_views.len().saturating_sub(1),
        );

        if self.item_views.is_empty() {
            cx.emit(Event::Remove);
        }
        cx.notify();
    }

    fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            cx.focus(active_item.to_any());
        }
    }

    pub fn split(&mut self, direction: SplitDirection, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Split(direction));
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
                    let mut title = item_view.title(cx);
                    if title.len() > MAX_TAB_TITLE_LEN {
                        let mut truncated_len = MAX_TAB_TITLE_LEN;
                        while !title.is_char_boundary(truncated_len) {
                            truncated_len -= 1;
                        }
                        title.truncate(truncated_len);
                        title.push('…');
                    }

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
                                    Container::new(
                                        Align::new(
                                            Label::new(
                                                title,
                                                if is_active {
                                                    theme.workspace.active_tab.label.clone()
                                                } else {
                                                    theme.workspace.tab.label.clone()
                                                },
                                            )
                                            .boxed(),
                                        )
                                        .boxed(),
                                    )
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
                .with_child(ChildView::new(active_item.id()).flexible(1., true).boxed())
                .named("pane")
        } else {
            Empty::new().named("pane")
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_active_item(cx);
    }
}

impl Navigation {
    pub fn push<D: 'static + Any, T: ItemView>(&self, data: Option<D>, cx: &mut ViewContext<T>) {
        let mut state = self.0.borrow_mut();
        match state.mode {
            NavigationHistoryMode::Normal => {
                state.backward_stack.push(NavigationEntry {
                    item_view: Box::new(cx.weak_handle()),
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
                state.forward_stack.clear();
            }
            NavigationHistoryMode::GoingBack => {
                state.forward_stack.push(NavigationEntry {
                    item_view: Box::new(cx.weak_handle()),
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
            NavigationHistoryMode::GoingForward => {
                state.backward_stack.push(NavigationEntry {
                    item_view: Box::new(cx.weak_handle()),
                    data: data.map(|data| Box::new(data) as Box<dyn Any>),
                });
            }
        }
    }
}
