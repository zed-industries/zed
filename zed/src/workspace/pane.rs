use super::{ItemViewHandle, SplitDirection};
use crate::settings::Settings;
use gpui::{
    color::ColorU,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    AppContext, Border, Entity, MutableAppContext, Quad, View, ViewContext,
};
use postage::watch;
use std::{cmp, path::Path, sync::Arc};

pub fn init(app: &mut MutableAppContext) {
    app.add_action(
        "pane:activate_item",
        |pane: &mut Pane, index: &usize, ctx| {
            pane.activate_item(*index, ctx);
        },
    );
    app.add_action("pane:activate_prev_item", |pane: &mut Pane, _: &(), ctx| {
        pane.activate_prev_item(ctx);
    });
    app.add_action("pane:activate_next_item", |pane: &mut Pane, _: &(), ctx| {
        pane.activate_next_item(ctx);
    });
    app.add_action("pane:close_active_item", |pane: &mut Pane, _: &(), ctx| {
        pane.close_active_item(ctx);
    });
    app.add_action(
        "pane:close_item",
        |pane: &mut Pane, item_id: &usize, ctx| {
            pane.close_item(*item_id, ctx);
        },
    );
    app.add_action("pane:split_up", |pane: &mut Pane, _: &(), ctx| {
        pane.split(SplitDirection::Up, ctx);
    });
    app.add_action("pane:split_down", |pane: &mut Pane, _: &(), ctx| {
        pane.split(SplitDirection::Down, ctx);
    });
    app.add_action("pane:split_left", |pane: &mut Pane, _: &(), ctx| {
        pane.split(SplitDirection::Left, ctx);
    });
    app.add_action("pane:split_right", |pane: &mut Pane, _: &(), ctx| {
        pane.split(SplitDirection::Right, ctx);
    });

    app.add_bindings(vec![
        Binding::new("shift-cmd-{", "pane:activate_prev_item", Some("Pane")),
        Binding::new("shift-cmd-}", "pane:activate_next_item", Some("Pane")),
        Binding::new("cmd-w", "pane:close_active_item", Some("Pane")),
        Binding::new("cmd-k up", "pane:split_up", Some("Pane")),
        Binding::new("cmd-k down", "pane:split_down", Some("Pane")),
        Binding::new("cmd-k left", "pane:split_left", Some("Pane")),
        Binding::new("cmd-k right", "pane:split_right", Some("Pane")),
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

    pub fn activate(&self, ctx: &mut ViewContext<Self>) {
        ctx.emit(Event::Activate);
    }

    pub fn add_item(
        &mut self,
        item: Box<dyn ItemViewHandle>,
        ctx: &mut ViewContext<Self>,
    ) -> usize {
        let item_idx = cmp::min(self.active_item + 1, self.items.len());
        self.items.insert(item_idx, item);
        ctx.notify();
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
        ctx: &mut ViewContext<Self>,
    ) -> bool {
        if let Some(index) = self.items.iter().position(|item| {
            item.entry_id(ctx.as_ref())
                .map_or(false, |id| id == entry_id)
        }) {
            self.activate_item(index, ctx);
            true
        } else {
            false
        }
    }

    pub fn item_index(&self, item: &dyn ItemViewHandle) -> Option<usize> {
        self.items.iter().position(|i| i.id() == item.id())
    }

    pub fn activate_item(&mut self, index: usize, ctx: &mut ViewContext<Self>) {
        if index < self.items.len() {
            self.active_item = index;
            self.focus_active_item(ctx);
            ctx.notify();
        }
    }

    pub fn activate_prev_item(&mut self, ctx: &mut ViewContext<Self>) {
        if self.active_item > 0 {
            self.active_item -= 1;
        } else if self.items.len() > 0 {
            self.active_item = self.items.len() - 1;
        }
        self.focus_active_item(ctx);
        ctx.notify();
    }

    pub fn activate_next_item(&mut self, ctx: &mut ViewContext<Self>) {
        if self.active_item + 1 < self.items.len() {
            self.active_item += 1;
        } else {
            self.active_item = 0;
        }
        self.focus_active_item(ctx);
        ctx.notify();
    }

    pub fn close_active_item(&mut self, ctx: &mut ViewContext<Self>) {
        if !self.items.is_empty() {
            self.close_item(self.items[self.active_item].id(), ctx)
        }
    }

    pub fn close_item(&mut self, item_id: usize, ctx: &mut ViewContext<Self>) {
        self.items.retain(|item| item.id() != item_id);
        self.active_item = cmp::min(self.active_item, self.items.len().saturating_sub(1));
        if self.items.is_empty() {
            ctx.emit(Event::Remove);
        }
        ctx.notify();
    }

    fn focus_active_item(&mut self, ctx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            ctx.focus(active_item.to_any());
        }
    }

    pub fn split(&mut self, direction: SplitDirection, ctx: &mut ViewContext<Self>) {
        ctx.emit(Event::Split(direction));
    }

    fn render_tabs(&self, ctx: &AppContext) -> ElementBox {
        let settings = self.settings.borrow();
        let border_color = ColorU::from_u32(0xdbdbdcff);
        let line_height = ctx.font_cache().line_height(
            ctx.font_cache().default_font(settings.ui_font_family),
            settings.ui_font_size,
        );

        let mut row = Flex::row();
        let last_item_ix = self.items.len() - 1;
        for (ix, item) in self.items.iter().enumerate() {
            enum Tab {}

            row.add_child(
                Expanded::new(
                    1.0,
                    MouseEventHandler::new::<Tab, _>(item.id(), ctx, |mouse_state| {
                        let title = item.title(ctx);

                        let mut border = Border::new(1.0, border_color);
                        border.left = ix > 0;
                        border.right = ix == last_item_ix;
                        border.bottom = ix != self.active_item;

                        let mut container = Container::new(
                            Stack::new()
                                .with_child(
                                    Align::new(
                                        Label::new(
                                            title,
                                            settings.ui_font_family,
                                            settings.ui_font_size,
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
                                        item.is_dirty(ctx),
                                        item.has_conflict(ctx),
                                        ctx,
                                    ))
                                    .right()
                                    .boxed(),
                                )
                                .boxed(),
                        )
                        .with_horizontal_padding(10.)
                        .with_border(border);

                        if ix == self.active_item {
                            container = container
                                .with_background_color(ColorU::white())
                                .with_padding_bottom(border.width);
                        } else {
                            container =
                                container.with_background_color(ColorU::from_u32(0xeaeaebff));
                        }

                        ConstrainedBox::new(
                            EventHandler::new(container.boxed())
                                .on_mouse_down(move |ctx| {
                                    ctx.dispatch_action("pane:activate_item", ix);
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
        row.add_child(
            ConstrainedBox::new(
                Container::new(Empty::new().boxed())
                    .with_border(Border::bottom(1.0, border_color))
                    .boxed(),
            )
            .with_min_width(20.)
            .named("fixed-filler"),
        );

        row.add_child(
            Expanded::new(
                0.0,
                Container::new(Empty::new().boxed())
                    .with_border(Border::bottom(1.0, border_color))
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
        ctx: &AppContext,
    ) -> ElementBox {
        enum TabCloseButton {}

        let dirty_color = ColorU::from_u32(0x556de8ff);
        let conflict_color = ColorU::from_u32(0xe45349ff);
        let mut clicked_color = dirty_color;
        clicked_color.a = 180;

        let current_color = if has_conflict {
            Some(conflict_color)
        } else if is_dirty {
            Some(dirty_color)
        } else {
            None
        };

        let icon = if tab_hovered {
            let mut icon = Svg::new("icons/x.svg");

            MouseEventHandler::new::<TabCloseButton, _>(item_id, ctx, |mouse_state| {
                if mouse_state.hovered {
                    Container::new(icon.with_color(ColorU::white()).boxed())
                        .with_background_color(if mouse_state.clicked {
                            clicked_color
                        } else {
                            dirty_color
                        })
                        .with_corner_radius(close_icon_size / 2.)
                        .boxed()
                } else {
                    if let Some(current_color) = current_color {
                        icon = icon.with_color(current_color);
                    }
                    icon.boxed()
                }
            })
            .on_click(move |ctx| ctx.dispatch_action("pane:close_item", item_id))
            .named("close-tab-icon")
        } else {
            let diameter = 8.;
            ConstrainedBox::new(
                Canvas::new(move |bounds, ctx| {
                    if let Some(current_color) = current_color {
                        let square = RectF::new(bounds.origin(), vec2f(diameter, diameter));
                        ctx.scene.push_quad(Quad {
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

    fn render<'a>(&self, app: &AppContext) -> ElementBox {
        if let Some(active_item) = self.active_item() {
            Flex::column()
                .with_child(self.render_tabs(app))
                .with_child(Expanded::new(1.0, ChildView::new(active_item.id()).boxed()).boxed())
                .named("pane")
        } else {
            Empty::new().named("pane")
        }
    }

    fn on_focus(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus_active_item(ctx);
    }

    // fn state(&self, app: &AppContext) -> Self::State {
    //     State {
    //         tabs: self
    //             .items
    //             .iter()
    //             .enumerate()
    //             .map(|(idx, item)| TabState {
    //                 title: item.title(app),
    //                 active: idx == self.active_item,
    //             })
    //             .collect(),
    //     }
    // }
}
