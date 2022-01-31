use super::Workspace;
use crate::Settings;
use gpui::{
    action, elements::*, platform::CursorStyle, AnyViewHandle, MutableAppContext, RenderContext,
};
use std::{cell::RefCell, rc::Rc};

pub struct Sidebar {
    side: Side,
    items: Vec<Item>,
    active_item_ix: Option<usize>,
    width: Rc<RefCell<f32>>,
}

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

struct Item {
    icon_path: &'static str,
    view: AnyViewHandle,
}

action!(ToggleSidebarItem, SidebarItemId);
action!(ToggleSidebarItemFocus, SidebarItemId);

#[derive(Clone)]
pub struct SidebarItemId {
    pub side: Side,
    pub item_index: usize,
}

impl Sidebar {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            items: Default::default(),
            active_item_ix: None,
            width: Rc::new(RefCell::new(260.)),
        }
    }

    pub fn add_item(&mut self, icon_path: &'static str, view: AnyViewHandle) {
        self.items.push(Item { icon_path, view });
    }

    pub fn activate_item(&mut self, item_ix: usize) {
        self.active_item_ix = Some(item_ix);
    }

    pub fn toggle_item(&mut self, item_ix: usize) {
        if self.active_item_ix == Some(item_ix) {
            self.active_item_ix = None;
        } else {
            self.active_item_ix = Some(item_ix);
        }
    }

    pub fn active_item(&self) -> Option<&AnyViewHandle> {
        self.active_item_ix
            .and_then(|ix| self.items.get(ix))
            .map(|item| &item.view)
    }

    fn theme<'a>(&self, settings: &'a Settings) -> &'a theme::Sidebar {
        match self.side {
            Side::Left => &settings.theme.workspace.left_sidebar,
            Side::Right => &settings.theme.workspace.right_sidebar,
        }
    }

    pub fn render(&self, settings: &Settings, cx: &mut RenderContext<Workspace>) -> ElementBox {
        let side = self.side;
        let theme = self.theme(settings);

        ConstrainedBox::new(
            Container::new(
                Flex::column()
                    .with_children(self.items.iter().enumerate().map(|(item_index, item)| {
                        let theme = if Some(item_index) == self.active_item_ix {
                            &theme.active_item
                        } else {
                            &theme.item
                        };
                        enum SidebarButton {}
                        MouseEventHandler::new::<SidebarButton, _, _, _>(
                            item.view.id(),
                            cx,
                            |_, _| {
                                ConstrainedBox::new(
                                    Align::new(
                                        ConstrainedBox::new(
                                            Svg::new(item.icon_path)
                                                .with_color(theme.icon_color)
                                                .boxed(),
                                        )
                                        .with_height(theme.icon_size)
                                        .boxed(),
                                    )
                                    .boxed(),
                                )
                                .with_height(theme.height)
                                .boxed()
                            },
                        )
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_mouse_down(move |cx| {
                            cx.dispatch_action(ToggleSidebarItem(SidebarItemId {
                                side,
                                item_index,
                            }))
                        })
                        .boxed()
                    }))
                    .boxed(),
            )
            .with_style(theme.container)
            .boxed(),
        )
        .with_width(theme.width)
        .boxed()
    }

    pub fn render_active_item(
        &self,
        settings: &Settings,
        cx: &mut MutableAppContext,
    ) -> Option<ElementBox> {
        if let Some(active_item) = self.active_item() {
            let mut container = Flex::row();
            if matches!(self.side, Side::Right) {
                container.add_child(self.render_resize_handle(settings, cx));
            }

            container.add_child(
                Hook::new(
                    ConstrainedBox::new(ChildView::new(active_item).boxed())
                        .with_max_width(*self.width.borrow())
                        .boxed(),
                )
                .on_after_layout({
                    let width = self.width.clone();
                    move |size, _| *width.borrow_mut() = size.x()
                })
                .flexible(1., false)
                .boxed(),
            );
            if matches!(self.side, Side::Left) {
                container.add_child(self.render_resize_handle(settings, cx));
            }
            Some(container.boxed())
        } else {
            None
        }
    }

    fn render_resize_handle(
        &self,
        settings: &Settings,
        mut cx: &mut MutableAppContext,
    ) -> ElementBox {
        let width = self.width.clone();
        let side = self.side;
        MouseEventHandler::new::<Self, _, _, _>(self.side.id(), &mut cx, |_, _| {
            Container::new(Empty::new().boxed())
                .with_style(self.theme(settings).resize_handle)
                .boxed()
        })
        .with_padding(Padding {
            left: 4.,
            right: 4.,
            ..Default::default()
        })
        .with_cursor_style(CursorStyle::ResizeLeftRight)
        .on_drag(move |delta, cx| {
            let prev_width = *width.borrow();
            match side {
                Side::Left => *width.borrow_mut() = 0f32.max(prev_width + delta.x()),
                Side::Right => *width.borrow_mut() = 0f32.max(prev_width - delta.x()),
            }

            cx.notify();
        })
        .boxed()
    }
}

impl Side {
    fn id(self) -> usize {
        match self {
            Side::Left => 0,
            Side::Right => 1,
        }
    }
}
