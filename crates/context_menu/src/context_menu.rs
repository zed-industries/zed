use gpui::{
    elements::*, geometry::vector::Vector2F, Action, Axis, Entity, RenderContext, SizeConstraint,
    View, ViewContext,
};
use settings::Settings;

pub enum ContextMenuItem {
    Item {
        label: String,
        action: Box<dyn Action>,
    },
    Separator,
}

pub struct ContextMenu {
    position: Vector2F,
    items: Vec<ContextMenuItem>,
    widest_item_index: usize,
    selected_index: Option<usize>,
    visible: bool,
}

impl Entity for ContextMenu {
    type Event = ();
}

impl View for ContextMenu {
    fn ui_name() -> &'static str {
        "ContextMenu"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Tag {}

        if !self.visible {
            return Empty::new().boxed();
        }

        let style = cx.global::<Settings>().theme.context_menu.clone();

        let mut widest_item = self.render_menu_item::<()>(self.widest_item_index, cx, &style);

        Overlay::new(
            Flex::column()
                .with_children(
                    (0..self.items.len()).map(|ix| self.render_menu_item::<Tag>(ix, cx, &style)),
                )
                .constrained()
                .dynamically(move |constraint, cx| {
                    SizeConstraint::strict_along(
                        Axis::Horizontal,
                        widest_item.layout(constraint, cx).x(),
                    )
                })
                .contained()
                .with_style(style.container)
                .boxed(),
        )
        .with_abs_position(self.position)
        .boxed()
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.visible = false;
        cx.notify();
    }
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            items: Default::default(),
            selected_index: Default::default(),
            widest_item_index: Default::default(),
            visible: false,
        }
    }

    pub fn show(
        &mut self,
        position: Vector2F,
        items: impl IntoIterator<Item = ContextMenuItem>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut items = items.into_iter().peekable();
        assert!(items.peek().is_some(), "must have at least one item");
        self.items = items.collect();
        self.widest_item_index = self
            .items
            .iter()
            .enumerate()
            .max_by_key(|(_, item)| match item {
                ContextMenuItem::Item { label, .. } => label.chars().count(),
                ContextMenuItem::Separator => 0,
            })
            .unwrap()
            .0;
        self.position = position;
        self.visible = true;
        cx.focus_self();
        cx.notify();
    }

    fn render_menu_item<T: 'static>(
        &self,
        ix: usize,
        cx: &mut RenderContext<ContextMenu>,
        style: &theme::ContextMenu,
    ) -> ElementBox {
        match &self.items[ix] {
            ContextMenuItem::Item { label, action } => {
                let action = action.boxed_clone();
                MouseEventHandler::new::<T, _, _>(ix, cx, |state, _| {
                    let style = style.item.style_for(state, Some(ix) == self.selected_index);
                    Flex::row()
                        .with_child(Label::new(label.to_string(), style.label.clone()).boxed())
                        .boxed()
                })
                .on_click(move |_, _, cx| cx.dispatch_any_action(action.boxed_clone()))
                .boxed()
            }
            ContextMenuItem::Separator => Empty::new()
                .contained()
                .with_style(style.separator)
                .constrained()
                .with_height(1.)
                .flex(1., false)
                .boxed(),
        }
    }
}
