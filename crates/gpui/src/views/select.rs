use crate::{
    actions, elements::*, impl_actions, AppContext, Entity, MutableAppContext, RenderContext, View,
    ViewContext, WeakViewHandle,
};

pub struct Select {
    handle: WeakViewHandle<Self>,
    render_item: Box<dyn Fn(usize, ItemType, bool, &AppContext) -> ElementBox>,
    selected_item_ix: usize,
    item_count: usize,
    is_open: bool,
    list_state: UniformListState,
    build_style: Option<Box<dyn FnMut(&mut MutableAppContext) -> SelectStyle>>,
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

#[derive(Clone)]
pub struct SelectItem(pub usize);

actions!(select, [ToggleSelect]);
impl_actions!(select, [SelectItem]);

pub enum Event {}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Select::toggle);
    cx.add_action(Select::select_item);
}

impl Select {
    pub fn new<F: 'static + Fn(usize, ItemType, bool, &AppContext) -> ElementBox>(
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

    pub fn with_style(
        mut self,
        f: impl 'static + FnMut(&mut MutableAppContext) -> SelectStyle,
    ) -> Self {
        self.build_style = Some(Box::new(f));
        self
    }

    pub fn set_item_count(&mut self, count: usize, cx: &mut ViewContext<Self>) {
        self.item_count = count;
        cx.notify();
    }

    fn toggle(&mut self, _: &ToggleSelect, cx: &mut ViewContext<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn select_item(&mut self, action: &SelectItem, cx: &mut ViewContext<Self>) {
        self.selected_item_ix = action.0;
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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if self.item_count == 0 {
            return Empty::new().boxed();
        }

        enum Header {}
        enum Item {}

        let style = if let Some(build_style) = self.build_style.as_mut() {
            (build_style)(cx)
        } else {
            Default::default()
        };
        let mut result = Flex::column().with_child(
            MouseEventHandler::new::<Header, _, _>(self.handle.id(), cx, |mouse_state, cx| {
                Container::new((self.render_item)(
                    self.selected_item_ix,
                    ItemType::Header,
                    mouse_state.hovered,
                    cx,
                ))
                .with_style(style.header)
                .boxed()
            })
            .on_click(move |cx| cx.dispatch_action(ToggleSelect))
            .boxed(),
        );
        if self.is_open {
            let handle = self.handle.clone();
            result.add_child(
                Overlay::new(
                    Container::new(
                        ConstrainedBox::new(
                            UniformList::new(
                                self.list_state.clone(),
                                self.item_count,
                                move |mut range, items, cx| {
                                    let handle = handle.upgrade(cx).unwrap();
                                    let this = handle.read(cx);
                                    let selected_item_ix = this.selected_item_ix;
                                    range.end = range.end.min(this.item_count);
                                    items.extend(range.map(|ix| {
                                        MouseEventHandler::new::<Item, _, _>(
                                            ix,
                                            cx,
                                            |mouse_state, cx| {
                                                (handle.read(cx).render_item)(
                                                    ix,
                                                    if ix == selected_item_ix {
                                                        ItemType::Selected
                                                    } else {
                                                        ItemType::Unselected
                                                    },
                                                    mouse_state.hovered,
                                                    cx,
                                                )
                                            },
                                        )
                                        .on_click(move |cx| cx.dispatch_action(SelectItem(ix)))
                                        .boxed()
                                    }))
                                },
                            )
                            .boxed(),
                        )
                        .with_max_height(200.)
                        .boxed(),
                    )
                    .with_style(style.menu)
                    .boxed(),
                )
                .boxed(),
            )
        }
        result.boxed()
    }
}
