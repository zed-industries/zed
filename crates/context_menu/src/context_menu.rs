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
        enum Tag {}

        if !self.visible {
            return Empty::new().boxed();
        }

        // Render the menu once at minimum width.
        let mut collapsed_menu = self.render_menu::<()>(false, cx).boxed();
        let expanded_menu = self
            .render_menu::<Tag>(true, cx)
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

    fn render_menu<Tag: 'static>(
        &mut self,
        expanded: bool,
        cx: &mut RenderContext<Self>,
    ) -> impl Element {
        let style = cx.global::<Settings>().theme.context_menu.clone();
        Flex::column()
            .with_children(
                (0..self.items.len())
                    .map(|ix| self.render_menu_item::<Tag>(ix, expanded, cx, &style)),
            )
            .contained()
            .with_style(style.container)
    }

    fn render_menu_item<T: 'static>(
        &self,
        ix: usize,
        expanded: bool,
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
                        .with_child({
                            let label = KeystrokeLabel::new(
                                action.boxed_clone(),
                                style.keystroke.container,
                                style.keystroke.text.clone(),
                            );
                            if expanded {
                                label.flex_float().boxed()
                            } else {
                                label.boxed()
                            }
                        })
                        .boxed()
                })
                .on_click(move |_, _, cx| cx.dispatch_any_action(action.boxed_clone()))
                .boxed()
            }
            ContextMenuItem::Separator => {
                let mut separator = Empty::new();
                if !expanded {
                    separator = separator.collapsed();
                }
                separator
                    .contained()
                    .with_style(style.separator)
                    .constrained()
                    .with_height(1.)
                    .boxed()
            }
        }
    }
}
