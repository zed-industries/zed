use gpui::{
    elements::*, geometry::vector::Vector2F, Action, Entity, RenderContext, View, ViewContext,
};
use settings::Settings;
use std::{marker::PhantomData, sync::Arc};

pub enum ContextMenuItem {
    Item {
        label: String,
        action: Box<dyn Action>,
    },
    Separator,
}

pub struct ContextMenu<T> {
    position: Vector2F,
    items: Arc<[ContextMenuItem]>,
    state: UniformListState,
    selected_index: Option<usize>,
    widest_item_index: Option<usize>,
    visible: bool,
    _phantom: PhantomData<T>,
}

impl<T: 'static> Entity for ContextMenu<T> {
    type Event = ();
}

impl<T: 'static> View for ContextMenu<T> {
    fn ui_name() -> &'static str {
        "ContextMenu"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if !self.visible {
            return Empty::new().boxed();
        }

        let theme = &cx.global::<Settings>().theme;
        let menu_style = &theme.project_panel.context_menu;
        let separator_style = menu_style.separator;
        let item_style = menu_style.item.clone();
        let items = self.items.clone();
        let selected_ix = self.selected_index;
        Overlay::new(
            UniformList::new(
                self.state.clone(),
                self.items.len(),
                move |range, elements, cx| {
                    let start = range.start;
                    elements.extend(items[range].iter().enumerate().map(|(ix, item)| {
                        let item_ix = start + ix;
                        match item {
                            ContextMenuItem::Item { label, action } => {
                                let action = action.boxed_clone();
                                MouseEventHandler::new::<T, _, _>(item_ix, cx, |state, _| {
                                    let style =
                                        item_style.style_for(state, Some(item_ix) == selected_ix);
                                    Flex::row()
                                        .with_child(
                                            Label::new(label.to_string(), style.label.clone())
                                                .boxed(),
                                        )
                                        .boxed()
                                })
                                .on_click(move |_, _, cx| {
                                    cx.dispatch_any_action(action.boxed_clone())
                                })
                                .boxed()
                            }
                            ContextMenuItem::Separator => {
                                Empty::new().contained().with_style(separator_style).boxed()
                            }
                        }
                    }))
                },
            )
            .with_width_from_item(self.widest_item_index)
            .boxed(),
        )
        .with_abs_position(self.position)
        .contained()
        .with_style(menu_style.container)
        .boxed()
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.visible = false;
        cx.notify();
    }
}

impl<T: 'static> ContextMenu<T> {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            items: Arc::from([]),
            state: Default::default(),
            selected_index: Default::default(),
            widest_item_index: Default::default(),
            visible: false,
            _phantom: PhantomData,
        }
    }

    pub fn show(
        &mut self,
        position: Vector2F,
        items: impl IntoIterator<Item = ContextMenuItem>,
        cx: &mut ViewContext<Self>,
    ) {
        self.items = items.into_iter().collect();
        self.widest_item_index = self
            .items
            .iter()
            .enumerate()
            .max_by_key(|(_, item)| match item {
                ContextMenuItem::Item { label, .. } => label.chars().count(),
                ContextMenuItem::Separator => 0,
            })
            .map(|(ix, _)| ix);
        self.position = position;
        self.visible = true;
        cx.focus_self();
        cx.notify();
    }
}
