use crate::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AppContext, Entity, View, ViewContext, WeakViewHandle,
};

pub struct Select {
    handle: WeakViewHandle<Self>,
    render_item: Box<dyn Fn(usize, ItemType, bool, &AppContext) -> AnyElement<Self>>,
    selected_item_ix: usize,
    item_count: usize,
    is_open: bool,
    list_state: UniformListState,
    build_style: Option<Box<dyn FnMut(&mut AppContext) -> SelectStyle>>,
}

#[derive(Clone, Default)]
pub struct SelectStyle {
    pub header: ContainerStyle,
    pub menu: ContainerStyle,
}

pub enum ItemType {
    Header,
    Selected,
    Unselected,
}

pub enum Event {}

impl Select {
    pub fn new<F: 'static + Fn(usize, ItemType, bool, &AppContext) -> AnyElement<Self>>(
        item_count: usize,
        cx: &mut ViewContext<Self>,
        render_item: F,
    ) -> Self {
        Self {
            handle: cx.weak_handle(),
            render_item: Box::new(render_item),
            selected_item_ix: 0,
            item_count,
            is_open: false,
            list_state: UniformListState::default(),
            build_style: Default::default(),
        }
    }

    pub fn with_style(mut self, f: impl 'static + FnMut(&mut AppContext) -> SelectStyle) -> Self {
        self.build_style = Some(Box::new(f));
        self
    }

    pub fn set_item_count(&mut self, count: usize, cx: &mut ViewContext<Self>) {
        self.item_count = count;
        cx.notify();
    }

    fn toggle(&mut self, cx: &mut ViewContext<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    pub fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>) {
        self.selected_item_ix = ix;
        self.is_open = false;
        cx.notify();
    }

    pub fn selected_index(&self) -> usize {
        self.selected_item_ix
    }
}

impl Entity for Select {
    type Event = Event;
}

impl View for Select {
    fn ui_name() -> &'static str {
        "Select"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if self.item_count == 0 {
            return Empty::new().into_any();
        }

        enum Header {}
        enum Item {}

        let style = if let Some(build_style) = self.build_style.as_mut() {
            (build_style)(cx)
        } else {
            Default::default()
        };
        let mut result = Flex::column().with_child(
            MouseEventHandler::new::<Header, _>(self.handle.id(), cx, |mouse_state, cx| {
                (self.render_item)(
                    self.selected_item_ix,
                    ItemType::Header,
                    mouse_state.hovered(),
                    cx,
                )
                .contained()
                .with_style(style.header)
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, move |_, this, cx| {
                this.toggle(cx);
            }),
        );
        if self.is_open {
            result.add_child(Overlay::new(
                UniformList::new(
                    self.list_state.clone(),
                    self.item_count,
                    cx,
                    move |this, mut range, items, cx| {
                        let selected_item_ix = this.selected_item_ix;
                        range.end = range.end.min(this.item_count);
                        items.extend(range.map(|ix| {
                            MouseEventHandler::new::<Item, _>(ix, cx, |mouse_state, cx| {
                                (this.render_item)(
                                    ix,
                                    if ix == selected_item_ix {
                                        ItemType::Selected
                                    } else {
                                        ItemType::Unselected
                                    },
                                    mouse_state.hovered(),
                                    cx,
                                )
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(MouseButton::Left, move |_, this, cx| {
                                this.set_selected_index(ix, cx);
                            })
                            .into_any()
                        }))
                    },
                )
                .constrained()
                .with_max_height(200.)
                .contained()
                .with_style(style.menu),
            ));
        }
        result.into_any()
    }
}
