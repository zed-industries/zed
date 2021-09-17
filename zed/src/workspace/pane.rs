use super::{ItemViewHandle, SplitDirection};
use crate::settings::Settings;
use gpui::{
    action,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::CursorStyle,
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

const MAX_TAB_TITLE_LEN: usize = 24;

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

        enum Tabs {}
        let tabs = MouseEventHandler::new::<Tabs, _, _, _>(0, cx, |mouse_state, cx| {
            let mut row = Flex::row();
            for (ix, item) in self.items.iter().enumerate() {
                let is_active = ix == self.active_item;

                row.add_child({
                    let mut title = item.title(cx);
                    if title.len() > MAX_TAB_TITLE_LEN {
                        let mut truncated_len = MAX_TAB_TITLE_LEN;
                        while !title.is_char_boundary(truncated_len) {
                            truncated_len -= 1;
                        }
                        title.truncate(truncated_len);
                        title.push('…');
                    }

                    let mut style = theme.workspace.tab.clone();
                    if is_active {
                        style = theme.workspace.active_tab.clone();
                        style.container.border.bottom = false;
                        style.container.padding.bottom += style.container.border.width;
                    }
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
                                            let item_id = item.id();
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

            row.boxed()
        });

        ConstrainedBox::new(tabs.boxed())
            .with_height(line_height + 16.)
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
