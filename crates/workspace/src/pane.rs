use super::{ItemViewHandle, SplitDirection};
use crate::{ItemHandle, Settings, Workspace};
use gpui::{
    action,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::CursorStyle,
    Entity, MutableAppContext, Quad, RenderContext, View, ViewContext,
};
use postage::watch;
use std::cmp;

action!(Split, SplitDirection);
action!(ActivateItem, usize);
action!(ActivatePrevItem);
action!(ActivateNextItem);
action!(CloseActiveItem);
action!(CloseItem, usize);

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

    cx.add_bindings(vec![
        Binding::new("shift-cmd-{", ActivatePrevItem, Some("Pane")),
        Binding::new("shift-cmd-}", ActivateNextItem, Some("Pane")),
        Binding::new("cmd-w", CloseActiveItem, Some("Pane")),
        Binding::new("cmd-k up", Split(SplitDirection::Up), Some("Pane")),
        Binding::new("cmd-k down", Split(SplitDirection::Down), Some("Pane")),
        Binding::new("cmd-k left", Split(SplitDirection::Left), Some("Pane")),
        Binding::new("cmd-k right", Split(SplitDirection::Right), Some("Pane")),
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
    active_item: usize,
    settings: watch::Receiver<Settings>,
}

impl Pane {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            item_views: Vec::new(),
            active_item: 0,
            settings,
        }
    }

    pub fn activate(&self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
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

        let item_view = item_handle.add_view(cx.window_id(), workspace, cx);
        self.add_item_view(item_view.boxed_clone(), cx);
        item_view
    }

    pub fn add_item_view(
        &mut self,
        item_view: Box<dyn ItemViewHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        item_view.added_to_pane(cx);
        let item_idx = cmp::min(self.active_item + 1, self.item_views.len());
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
            .get(self.active_item)
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
            self.active_item = index;
            self.focus_active_item(cx);
            cx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, cx: &mut ViewContext<Self>) {
        if self.active_item > 0 {
            self.active_item -= 1;
        } else if self.item_views.len() > 0 {
            self.active_item = self.item_views.len() - 1;
        }
        self.focus_active_item(cx);
        cx.notify();
    }

    pub fn activate_next_item(&mut self, cx: &mut ViewContext<Self>) {
        if self.active_item + 1 < self.item_views.len() {
            self.active_item += 1;
        } else {
            self.active_item = 0;
        }
        self.focus_active_item(cx);
        cx.notify();
    }

    pub fn close_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if !self.item_views.is_empty() {
            self.close_item(self.item_views[self.active_item].1.id(), cx)
        }
    }

    pub fn close_item(&mut self, item_id: usize, cx: &mut ViewContext<Self>) {
        self.item_views.retain(|(_, item)| item.id() != item_id);
        self.active_item = cmp::min(self.active_item, self.item_views.len().saturating_sub(1));
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
                let is_active = ix == self.active_item;

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
