use std::sync::Arc;

use gpui::{px, AnyElement, AnyView, ClickEvent, MouseButton, MouseDownEvent, Pixels};
use smallvec::SmallVec;

use crate::{prelude::*, Disclosure};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ListItemSpacing {
    #[default]
    Dense,
    Sparse,
}

#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    disabled: bool,
    selected: bool,
    spacing: ListItemSpacing,
    indent_level: usize,
    indent_step_size: Pixels,
    /// A slot for content that appears before the children, like an icon or avatar.
    start_slot: Option<AnyElement>,
    /// A slot for content that appears after the children, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
    /// A slot for content that appears on hover after the children
    /// It will obscure the `end_slot` when visible.
    end_hover_slot: Option<AnyElement>,
    toggle: Option<bool>,
    inset: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>>,
    on_secondary_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
    selectable: bool,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            selected: false,
            spacing: ListItemSpacing::Dense,
            indent_level: 0,
            indent_step_size: px(12.),
            start_slot: None,
            end_slot: None,
            end_hover_slot: None,
            toggle: None,
            inset: false,
            on_click: None,
            on_secondary_mouse_down: None,
            on_toggle: None,
            tooltip: None,
            children: SmallVec::new(),
            selectable: true,
        }
    }

    pub fn spacing(mut self, spacing: ListItemSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn selectable(mut self, has_hover: bool) -> Self {
        self.selectable = has_hover;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_secondary_mouse_down(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_secondary_mouse_down = Some(Box::new(handler));
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn indent_level(mut self, indent_level: usize) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn indent_step_size(mut self, indent_step_size: Pixels) -> Self {
        self.indent_step_size = indent_step_size;
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(on_toggle));
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_hover_slot<E: IntoElement>(mut self, end_hover_slot: impl Into<Option<E>>) -> Self {
        self.end_hover_slot = end_hover_slot.into().map(IntoElement::into_any_element);
        self
    }
}

impl Disableable for ListItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for ListItem {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for ListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .w_full()
            .relative()
            // When an item is inset draw the indent spacing outside of the item
            .when(self.inset, |this| {
                this.ml(self.indent_level as f32 * self.indent_step_size)
                    .px_2()
            })
            .when(!self.inset && !self.disabled, |this| {
                this
                    // TODO: Add focus state
                    // .when(self.state == InteractionState::Focused, |this| {
                    //     this.border_1()
                    //         .border_color(cx.theme().colors().border_focused)
                    // })
                    .when(self.selectable, |this| {
                        this.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                            .when(self.selected, |this| {
                                this.bg(cx.theme().colors().ghost_element_selected)
                            })
                    })
            })
            .child(
                h_flex()
                    .id("inner_list_item")
                    .w_full()
                    .relative()
                    .gap_1()
                    .px_2()
                    .map(|this| match self.spacing {
                        ListItemSpacing::Dense => this,
                        ListItemSpacing::Sparse => this.py_1(),
                    })
                    .group("list_item")
                    .when(self.inset && !self.disabled, |this| {
                        this
                            // TODO: Add focus state
                            // .when(self.state == InteractionState::Focused, |this| {
                            //     this.border_1()
                            //         .border_color(cx.theme().colors().border_focused)
                            // })
                            .when(self.selectable, |this| {
                                this.hover(|style| {
                                    style.bg(cx.theme().colors().ghost_element_hover)
                                })
                                .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                                .when(self.selected, |this| {
                                    this.bg(cx.theme().colors().ghost_element_selected)
                                })
                            })
                    })
                    .when_some(self.on_click, |this, on_click| {
                        this.cursor_pointer().on_click(on_click)
                    })
                    .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                        this.on_mouse_down(MouseButton::Right, move |event, cx| {
                            (on_mouse_down)(event, cx)
                        })
                    })
                    .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip))
                    .map(|this| {
                        if self.inset {
                            this.rounded_md()
                        } else {
                            // When an item is not inset draw the indent spacing inside of the item
                            this.ml(self.indent_level as f32 * self.indent_step_size)
                        }
                    })
                    .children(self.toggle.map(|is_open| {
                        div()
                            .flex()
                            .absolute()
                            .left(rems(-1.))
                            .when(is_open, |this| this.visible_on_hover(""))
                            .child(Disclosure::new("toggle", is_open).on_toggle(self.on_toggle))
                    }))
                    .child(
                        h_flex()
                            .flex_grow()
                            .flex_shrink_0()
                            .flex_basis(relative(0.25))
                            .gap_1()
                            .overflow_hidden()
                            .children(self.start_slot)
                            .children(self.children),
                    )
                    .when_some(self.end_slot, |this, end_slot| {
                        this.justify_between().child(
                            h_flex()
                                .flex_shrink()
                                .overflow_hidden()
                                .when(self.end_hover_slot.is_some(), |this| {
                                    this.visible()
                                        .group_hover("list_item", |this| this.invisible())
                                })
                                .child(end_slot),
                        )
                    })
                    .when_some(self.end_hover_slot, |this, end_hover_slot| {
                        this.child(
                            h_flex()
                                .h_full()
                                .absolute()
                                .right_2()
                                .top_0()
                                .visible_on_hover("list_item")
                                .child(end_hover_slot),
                        )
                    }),
            )
    }
}
