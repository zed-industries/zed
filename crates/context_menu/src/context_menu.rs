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

impl ContextMenuItem {
    pub fn item(label: String, action: impl 'static + Action) -> Self {
        Self::Item {
            label,
            action: Box::new(action),
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

pub struct ContextMenu {
    position: Vector2F,
    items: Vec<ContextMenuItem>,
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
        if !self.visible {
            return Empty::new().boxed();
        }

        // Render the menu once at minimum width.
        let mut collapsed_menu = self.render_menu_for_measurement(cx).boxed();
        let expanded_menu = self
            .render_menu(cx)
            .constrained()
            .dynamically(move |constraint, cx| {
                SizeConstraint::strict_along(
                    Axis::Horizontal,
                    collapsed_menu.layout(constraint, cx).x(),
                )
            })
            .boxed();

        Overlay::new(expanded_menu)
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
        self.position = position;
        self.visible = true;
        cx.focus_self();
        cx.notify();
    }

    fn render_menu_for_measurement(&self, cx: &mut RenderContext<Self>) -> impl Element {
        let style = cx.global::<Settings>().theme.context_menu.clone();
        Flex::row()
            .with_child(
                Flex::column()
                    .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                        match item {
                            ContextMenuItem::Item { label, .. } => {
                                let style = style.item.style_for(
                                    &Default::default(),
                                    Some(ix) == self.selected_index,
                                );
                                Label::new(label.to_string(), style.label.clone()).boxed()
                            }
                            ContextMenuItem::Separator => Empty::new()
                                .collapsed()
                                .contained()
                                .with_style(style.separator)
                                .constrained()
                                .with_height(1.)
                                .boxed(),
                        }
                    }))
                    .boxed(),
            )
            .with_child(
                Flex::column()
                    .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                        match item {
                            ContextMenuItem::Item { action, .. } => {
                                let style = style.item.style_for(
                                    &Default::default(),
                                    Some(ix) == self.selected_index,
                                );
                                KeystrokeLabel::new(
                                    action.boxed_clone(),
                                    style.keystroke.container,
                                    style.keystroke.text.clone(),
                                )
                                .boxed()
                            }
                            ContextMenuItem::Separator => Empty::new()
                                .collapsed()
                                .contained()
                                .with_style(style.separator)
                                .constrained()
                                .with_height(1.)
                                .boxed(),
                        }
                    }))
                    .boxed(),
            )
            .contained()
            .with_style(style.container)
    }

    fn render_menu(&self, cx: &mut RenderContext<Self>) -> impl Element {
        enum Tag {}
        let style = cx.global::<Settings>().theme.context_menu.clone();
        Flex::column()
            .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                match item {
                    ContextMenuItem::Item { label, action } => {
                        let action = action.boxed_clone();
                        MouseEventHandler::new::<Tag, _, _>(ix, cx, |state, _| {
                            let style =
                                style.item.style_for(state, Some(ix) == self.selected_index);
                            Flex::row()
                                .with_child(
                                    Label::new(label.to_string(), style.label.clone()).boxed(),
                                )
                                .with_child({
                                    KeystrokeLabel::new(
                                        action.boxed_clone(),
                                        style.keystroke.container,
                                        style.keystroke.text.clone(),
                                    )
                                    .flex_float()
                                    .boxed()
                                })
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
                        .boxed(),
                }
            }))
            .contained()
            .with_style(style.container)
    }
}
