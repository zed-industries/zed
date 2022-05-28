use gpui::{
    elements::*, geometry::vector::Vector2F, keymap, platform::CursorStyle, Action, AppContext,
    Axis, Entity, MutableAppContext, RenderContext, SizeConstraint, View, ViewContext,
};
use menu::*;
use settings::Settings;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContextMenu::select_first);
    cx.add_action(ContextMenu::select_last);
    cx.add_action(ContextMenu::select_next);
    cx.add_action(ContextMenu::select_prev);
    cx.add_action(ContextMenu::confirm);
    cx.add_action(ContextMenu::cancel);
}

pub enum ContextMenuItem {
    Item {
        label: String,
        action: Box<dyn Action>,
    },
    Separator,
}

impl ContextMenuItem {
    pub fn item(label: impl ToString, action: impl 'static + Action) -> Self {
        Self::Item {
            label: label.to_string(),
            action: Box::new(action),
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    fn is_separator(&self) -> bool {
        matches!(self, Self::Separator)
    }
}

#[derive(Default)]
pub struct ContextMenu {
    position: Vector2F,
    items: Vec<ContextMenuItem>,
    selected_index: Option<usize>,
    visible: bool,
    previously_focused_view_id: Option<usize>,
}

impl Entity for ContextMenu {
    type Event = ();
}

impl View for ContextMenu {
    fn ui_name() -> &'static str {
        "ContextMenu"
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
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
        self.selected_index.take();
        cx.notify();
    }
}

impl ContextMenu {
    pub fn new() -> Self {
        Default::default()
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            if let Some(ContextMenuItem::Item { action, .. }) = self.items.get(ix) {
                let window_id = cx.window_id();
                let view_id = cx.view_id();
                cx.dispatch_action_at(window_id, view_id, action.as_ref());
            }
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if cx.handle().is_focused(cx) {
            let window_id = cx.window_id();
            (**cx).focus(window_id, self.previously_focused_view_id.take());
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.selected_index = self.items.iter().position(|item| !item.is_separator());
        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if !item.is_separator() {
                self.selected_index = Some(ix);
                cx.notify();
                break;
            }
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().skip(ix + 1) {
                if !item.is_separator() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.select_first(&Default::default(), cx);
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                if !item.is_separator() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.select_last(&Default::default(), cx);
        }
    }

    pub fn show(
        &mut self,
        position: Vector2F,
        items: impl IntoIterator<Item = ContextMenuItem>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut items = items.into_iter().peekable();
        if items.peek().is_some() {
            self.items = items.collect();
            self.position = position;
            self.visible = true;
            if !cx.is_self_focused() {
                self.previously_focused_view_id = cx.focused_view_id(cx.window_id());
            }
            cx.focus_self();
        } else {
            self.visible = false;
        }
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
                                let style = style
                                    .item
                                    .style_for(Default::default(), Some(ix) == self.selected_index);
                                Label::new(label.to_string(), style.label.clone())
                                    .contained()
                                    .with_style(style.container)
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
            .with_child(
                Flex::column()
                    .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                        match item {
                            ContextMenuItem::Item { action, .. } => {
                                let style = style
                                    .item
                                    .style_for(Default::default(), Some(ix) == self.selected_index);
                                KeystrokeLabel::new(
                                    action.boxed_clone(),
                                    style.keystroke.container,
                                    style.keystroke.text.clone(),
                                )
                                .boxed()
                            }
                            ContextMenuItem::Separator => Empty::new()
                                .collapsed()
                                .constrained()
                                .with_height(1.)
                                .contained()
                                .with_style(style.separator)
                                .boxed(),
                        }
                    }))
                    .boxed(),
            )
            .contained()
            .with_style(style.container)
    }

    fn render_menu(&self, cx: &mut RenderContext<Self>) -> impl Element {
        enum Menu {}
        enum MenuItem {}
        let style = cx.global::<Settings>().theme.context_menu.clone();
        MouseEventHandler::new::<Menu, _, _>(0, cx, |_, cx| {
            Flex::column()
                .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                    match item {
                        ContextMenuItem::Item { label, action } => {
                            let action = action.boxed_clone();
                            MouseEventHandler::new::<MenuItem, _, _>(ix, cx, |state, _| {
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
                                    .contained()
                                    .with_style(style.container)
                                    .boxed()
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(move |_, _, cx| {
                                cx.dispatch_any_action(action.boxed_clone());
                                cx.dispatch_action(Cancel);
                            })
                            .boxed()
                        }
                        ContextMenuItem::Separator => Empty::new()
                            .constrained()
                            .with_height(1.)
                            .contained()
                            .with_style(style.separator)
                            .boxed(),
                    }
                }))
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_mouse_down_out(|_, cx| cx.dispatch_action(Cancel))
        .on_right_mouse_down_out(|_, cx| cx.dispatch_action(Cancel))
    }
}
