use super::{ItemViewHandle, SplitDirection};
use crate::{settings::Settings, theme};
use gpui::{
    action,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    Border, Entity, MutableAppContext, Quad, RenderContext, View, ViewContext, ViewHandle,
};
use postage::watch;
use std::{cmp, path::Path, sync::Arc};

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

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    pub tabs: Vec<TabState>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TabState {
    pub title: String,
    pub active: bool,
}

pub struct Pane {
    items: Vec<Box<dyn ItemViewHandle>>,
    active_item: usize,
    settings: watch::Receiver<Settings>,
}

impl Pane {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            items: Vec::new(),
            active_item: 0,
            settings,
        }
    }

    pub fn activate(&self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
    }

    pub fn add_item(&mut self, item: Box<dyn ItemViewHandle>, cx: &mut ViewContext<Self>) -> usize {
        let item_idx = cmp::min(self.active_item + 1, self.items.len());
        self.items.insert(item_idx, item);
        cx.notify();
        item_idx
    }

    #[cfg(test)]
    pub fn items(&self) -> &[Box<dyn ItemViewHandle>] {
        &self.items
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemViewHandle>> {
        self.items.get(self.active_item).cloned()
    }

    pub fn activate_entry(
        &mut self,
        entry_id: (usize, Arc<Path>),
        cx: &mut ViewContext<Self>,
    ) -> bool {
        if let Some(index) = self.items.iter().position(|item| {
            item.entry_id(cx.as_ref())
                .map_or(false, |id| id == entry_id)
        }) {
            self.activate_item(index, cx);
            true
        } else {
            false
        }
    }

    pub fn item_index(&self, item: &dyn ItemViewHandle) -> Option<usize> {
        self.items.iter().position(|i| i.id() == item.id())
    }

    pub fn activate_item(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if index < self.items.len() {
            self.active_item = index;
            self.focus_active_item(cx);
            cx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, cx: &mut ViewContext<Self>) {
        if self.active_item > 0 {
            self.active_item -= 1;
        } else if self.items.len() > 0 {
            self.active_item = self.items.len() - 1;
        }
        self.focus_active_item(cx);
        cx.notify();
    }

    pub fn activate_next_item(&mut self, cx: &mut ViewContext<Self>) {
        if self.active_item + 1 < self.items.len() {
            self.active_item += 1;
        } else {
            self.active_item = 0;
        }
        self.focus_active_item(cx);
        cx.notify();
    }

    pub fn close_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if !self.items.is_empty() {
            self.close_item(self.items[self.active_item].id(), cx)
        }
    }

    pub fn close_item(&mut self, item_id: usize, cx: &mut ViewContext<Self>) {
        self.items.retain(|item| item.id() != item_id);
        self.active_item = cmp::min(self.active_item, self.items.len().saturating_sub(1));
        if self.items.is_empty() {
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
        let line_height = cx.font_cache().line_height(
            theme.workspace.tab.label.text.font_id,
            theme.workspace.tab.label.text.font_size,
        );

        let mut row = Flex::row();
        let last_item_ix = self.items.len() - 1;
        for (ix, item) in self.items.iter().enumerate() {
            let is_active = ix == self.active_item;

            enum Tab {}
            let border = &theme.workspace.tab.container.border;

            row.add_child(
                Expanded::new(
                    1.0,
                    MouseEventHandler::new::<Tab, _, _>(item.id(), cx, |mouse_state, cx| {
                        let title = item.title(cx);

                        let mut border = border.clone();
                        border.left = ix > 0;
                        border.right = ix == last_item_ix;
                        border.bottom = !is_active;

                        let mut container = Container::new(
                            Stack::new()
                                .with_child(
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
                                .with_child(
                                    Align::new(Self::render_tab_icon(
                                        item.id(),
                                        line_height - 2.,
                                        mouse_state.hovered,
                                        item.is_dirty(cx),
                                        item.has_conflict(cx),
                                        theme,
                                        cx,
                                    ))
                                    .right()
                                    .boxed(),
                                )
                                .boxed(),
                        )
                        .with_style(if is_active {
                            &theme.workspace.active_tab.container
                        } else {
                            &theme.workspace.tab.container
                        })
                        .with_border(border);

                        if is_active {
                            container = container.with_padding_bottom(border.width);
                        }

                        ConstrainedBox::new(
                            EventHandler::new(container.boxed())
                                .on_mouse_down(move |cx| {
                                    cx.dispatch_action(ActivateItem(ix));
                                    true
                                })
                                .boxed(),
                        )
                        .with_min_width(80.0)
                        .with_max_width(264.0)
                        .boxed()
                    })
                    .boxed(),
                )
                .named("tab"),
            );
        }

        // Ensure there's always a minimum amount of space after the last tab,
        // so that the tab's border doesn't abut the window's border.
        let mut border = Border::bottom(1.0, Color::default());
        border.color = theme.workspace.tab.container.border.color;

        row.add_child(
            ConstrainedBox::new(
                Container::new(Empty::new().boxed())
                    .with_border(border)
                    .boxed(),
            )
            .with_min_width(20.)
            .named("fixed-filler"),
        );

        row.add_child(
            Expanded::new(
                0.0,
                Container::new(Empty::new().boxed())
                    .with_border(border)
                    .boxed(),
            )
            .named("filler"),
        );

        ConstrainedBox::new(row.boxed())
            .with_height(line_height + 16.)
            .named("tabs")
    }

    fn render_tab_icon(
        item_id: usize,
        close_icon_size: f32,
        tab_hovered: bool,
        is_dirty: bool,
        has_conflict: bool,
        theme: &theme::Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum TabCloseButton {}

        let mut clicked_color = theme.workspace.tab.icon_dirty;
        clicked_color.a = 180;

        let current_color = if has_conflict {
            Some(theme.workspace.tab.icon_conflict)
        } else if is_dirty {
            Some(theme.workspace.tab.icon_dirty)
        } else {
            None
        };

        let icon = if tab_hovered {
            let close_color = current_color.unwrap_or(theme.workspace.tab.icon_close);
            let icon = Svg::new("icons/x.svg").with_color(close_color);

            MouseEventHandler::new::<TabCloseButton, _, _>(item_id, cx, |mouse_state, _| {
                if mouse_state.hovered {
                    Container::new(icon.with_color(Color::white()).boxed())
                        .with_background_color(if mouse_state.clicked {
                            clicked_color
                        } else {
                            theme.workspace.tab.icon_dirty
                        })
                        .with_corner_radius(close_icon_size / 2.)
                        .boxed()
                } else {
                    icon.boxed()
                }
            })
            .on_click(move |cx| cx.dispatch_action(CloseItem(item_id)))
            .named("close-tab-icon")
        } else {
            let diameter = 8.;
            ConstrainedBox::new(
                Canvas::new(move |bounds, cx| {
                    if let Some(current_color) = current_color {
                        let square = RectF::new(bounds.origin(), vec2f(diameter, diameter));
                        cx.scene.push_quad(Quad {
                            bounds: square,
                            background: Some(current_color),
                            border: Default::default(),
                            corner_radius: diameter / 2.,
                        });
                    }
                })
                .boxed(),
            )
            .with_width(diameter)
            .with_height(diameter)
            .named("unsaved-tab-icon")
        };

        ConstrainedBox::new(Align::new(icon).boxed())
            .with_width(close_icon_size)
            .named("tab-icon")
    }
}

impl Entity for Pane {
    type Event = Event;
}

impl View for Pane {
    fn ui_name() -> &'static str {
        "Pane"
    }

    fn render(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        if let Some(active_item) = self.active_item() {
            Flex::column()
                .with_child(self.render_tabs(cx))
                .with_child(Expanded::new(1.0, ChildView::new(active_item.id()).boxed()).boxed())
                .named("pane")
        } else {
            Empty::new().named("pane")
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_active_item(cx);
    }
}

pub trait PaneHandle {
    fn add_item_view(&self, item: Box<dyn ItemViewHandle>, cx: &mut MutableAppContext);
}

impl PaneHandle for ViewHandle<Pane> {
    fn add_item_view(&self, item: Box<dyn ItemViewHandle>, cx: &mut MutableAppContext) {
        item.set_parent_pane(self, cx);
        self.update(cx, |pane, cx| {
            let item_idx = pane.add_item(item, cx);
            pane.activate_item(item_idx, cx);
        });
    }
}
