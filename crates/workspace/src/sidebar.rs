use gpui::{
    elements::*, impl_actions, platform::CursorStyle, AnyViewHandle, Entity, RenderContext, View,
    ViewContext, ViewHandle,
};
use serde::Deserialize;
use settings::Settings;
use std::{cell::RefCell, rc::Rc};
use theme::Theme;

use crate::StatusItemView;

pub struct Sidebar {
    side: Side,
    items: Vec<Item>,
    active_item_ix: Option<usize>,
    actual_width: Rc<RefCell<f32>>,
    custom_width: Rc<RefCell<f32>>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone)]
struct Item {
    icon_path: &'static str,
    view: AnyViewHandle,
}

pub struct SidebarButtons {
    sidebar: ViewHandle<Sidebar>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToggleSidebarItem {
    pub side: Side,
    pub item_index: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToggleSidebarItemFocus {
    pub side: Side,
    pub item_index: usize,
}

impl_actions!(workspace, [ToggleSidebarItem, ToggleSidebarItemFocus]);

impl Sidebar {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            items: Default::default(),
            active_item_ix: None,
            actual_width: Rc::new(RefCell::new(260.)),
            custom_width: Rc::new(RefCell::new(260.)),
        }
    }

    pub fn add_item(
        &mut self,
        icon_path: &'static str,
        view: AnyViewHandle,
        cx: &mut ViewContext<Self>,
    ) {
        self.items.push(Item { icon_path, view });
        cx.notify()
    }

    pub fn activate_item(&mut self, item_ix: usize, cx: &mut ViewContext<Self>) {
        self.active_item_ix = Some(item_ix);
        cx.notify();
    }

    pub fn toggle_item(&mut self, item_ix: usize, cx: &mut ViewContext<Self>) {
        if self.active_item_ix == Some(item_ix) {
            self.active_item_ix = None;
        } else {
            self.active_item_ix = Some(item_ix);
        }
        cx.notify();
    }

    pub fn active_item(&self) -> Option<&AnyViewHandle> {
        self.active_item_ix
            .and_then(|ix| self.items.get(ix))
            .map(|item| &item.view)
    }

    fn render_resize_handle(&self, theme: &Theme, cx: &mut RenderContext<Self>) -> ElementBox {
        let actual_width = self.actual_width.clone();
        let custom_width = self.custom_width.clone();
        let side = self.side;
        MouseEventHandler::new::<Self, _, _>(side as usize, cx, |_, _| {
            Empty::new()
                .contained()
                .with_style(theme.workspace.sidebar_resize_handle)
                .boxed()
        })
        .with_padding(Padding {
            left: 4.,
            right: 4.,
            ..Default::default()
        })
        .with_cursor_style(CursorStyle::ResizeLeftRight)
        .on_drag(move |delta, cx| {
            let prev_width = *actual_width.borrow();
            match side {
                Side::Left => *custom_width.borrow_mut() = 0f32.max(prev_width + delta.x()),
                Side::Right => *custom_width.borrow_mut() = 0f32.max(prev_width - delta.x()),
            }

            cx.notify();
        })
        .boxed()
    }
}

impl Entity for Sidebar {
    type Event = ();
}

impl View for Sidebar {
    fn ui_name() -> &'static str {
        "Sidebar"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        if let Some(active_item) = self.active_item() {
            let mut container = Flex::row();
            if matches!(self.side, Side::Right) {
                container.add_child(self.render_resize_handle(&theme, cx));
            }

            container.add_child(
                Hook::new(
                    ChildView::new(active_item)
                        .constrained()
                        .with_max_width(*self.custom_width.borrow())
                        .boxed(),
                )
                .on_after_layout({
                    let actual_width = self.actual_width.clone();
                    move |size, _| *actual_width.borrow_mut() = size.x()
                })
                .flex(1., false)
                .boxed(),
            );
            if matches!(self.side, Side::Left) {
                container.add_child(self.render_resize_handle(&theme, cx));
            }
            container.boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl SidebarButtons {
    pub fn new(sidebar: ViewHandle<Sidebar>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&sidebar, |_, _, cx| cx.notify()).detach();
        Self { sidebar }
    }
}

impl Entity for SidebarButtons {
    type Event = ();
}

impl View for SidebarButtons {
    fn ui_name() -> &'static str {
        "SidebarToggleButton"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx
            .global::<Settings>()
            .theme
            .workspace
            .status_bar
            .sidebar_buttons;
        let sidebar = self.sidebar.read(cx);
        let item_style = theme.item;
        let hover_item_style = theme.item_hover;
        let active_item_style = theme.item_active;
        let active_ix = sidebar.active_item_ix;
        let side = sidebar.side;
        let group_style = match side {
            Side::Left => theme.group_left,
            Side::Right => theme.group_right,
        };
        let items = sidebar.items.clone();
        Flex::row()
            .with_children(items.iter().enumerate().map(|(ix, item)| {
                MouseEventHandler::new::<Self, _, _>(ix, cx, move |state, _| {
                    let style = if Some(ix) == active_ix {
                        active_item_style
                    } else if state.hovered {
                        hover_item_style
                    } else {
                        item_style
                    };
                    Svg::new(item.icon_path)
                        .with_color(style.icon_color)
                        .constrained()
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(move |cx| {
                    cx.dispatch_action(ToggleSidebarItem {
                        side,
                        item_index: ix,
                    })
                })
                .boxed()
            }))
            .contained()
            .with_style(group_style)
            .boxed()
    }
}

impl StatusItemView for SidebarButtons {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn crate::ItemHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}
