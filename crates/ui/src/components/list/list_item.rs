use std::sync::Arc;

use gpui::{AnyElement, AnyView, ClickEvent, MouseButton, MouseDownEvent, Pixels, px};
use smallvec::SmallVec;

use crate::{Disclosure, prelude::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ListItemSpacing {
    #[default]
    Dense,
    ExtraDense,
    Sparse,
}

#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    group_name: Option<SharedString>,
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
    /// A slot for content that only renders on hover or focus,
    /// Elements are conditionally rendered only when hover/focus occurs
    end_slot_invisible_button: Option<AnyElement>,
    toggle: Option<bool>,
    inset: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    on_secondary_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
    selectable: bool,
    always_show_disclosure_icon: bool,
    outlined: bool,
    rounded: bool,
    overflow_x: bool,
    focused: Option<bool>,
    hovered: bool,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            group_name: None,
            disabled: false,
            selected: false,
            spacing: ListItemSpacing::Dense,
            indent_level: 0,
            indent_step_size: px(12.),
            start_slot: None,
            end_slot: None,
            end_hover_slot: None,
            end_slot_invisible_button: None,
            toggle: None,
            inset: false,
            on_click: None,
            on_secondary_mouse_down: None,
            on_toggle: None,
            tooltip: None,
            children: SmallVec::new(),
            selectable: true,
            always_show_disclosure_icon: false,
            outlined: false,
            rounded: false,
            overflow_x: false,
            focused: None,
            hovered: false,
        }
    }

    pub fn group_name(mut self, group_name: impl Into<SharedString>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    pub fn spacing(mut self, spacing: ListItemSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn selectable(mut self, has_hover: bool) -> Self {
        self.selectable = has_hover;
        self
    }

    pub fn always_show_disclosure_icon(mut self, show: bool) -> Self {
        self.always_show_disclosure_icon = show;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_secondary_mouse_down(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary_mouse_down = Some(Box::new(handler));
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
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
        on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
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

    pub fn end_slot_invisible_button<E: IntoElement>(
        mut self,
        element: impl Into<Option<E>>,
    ) -> Self {
        self.end_slot_invisible_button = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn outlined(mut self) -> Self {
        self.outlined = true;
        self
    }

    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    pub fn overflow_x(mut self) -> Self {
        self.overflow_x = true;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = Some(focused);
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }
}

impl Disableable for ListItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ListItem {
    fn toggle_state(mut self, selected: bool) -> Self {
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .when_some(self.group_name, |this, group| this.group(group))
            .w_full()
            .relative()
            // When an item is inset draw the indent spacing outside of the item
            .when(self.inset, |this| {
                this.ml(self.indent_level as f32 * self.indent_step_size)
                    .px(DynamicSpacing::Base04.rems(cx))
            })
            .when(!self.inset && !self.disabled, |this| {
                this
                    // TODO: Add focus state
                    // .when(self.state == InteractionState::Focused, |this| {
                    .when_some(self.focused, |this, focused| {
                        if focused {
                            this.border_1()
                                .border_color(cx.theme().colors().border_focused)
                        } else {
                            this.border_1()
                        }
                    })
                    .when(self.selectable, |this| {
                        this.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
                            .when(self.outlined, |this| this.rounded_sm())
                            .when(self.selected, |this| {
                                this.bg(cx.theme().colors().ghost_element_selected)
                            })
                    })
            })
            .when(self.rounded, |this| this.rounded_sm())
            // .on_hover(move |this, hovered, cx| {
            //     this.hovered = *hovered;
            //     // cx.notify();
            //     this
            // })
            // .on_hover(cx.listener(move |this, hovered, window, cx| {
            //     if *hovered {
            //         this.hovered = true;
            //     } else {
            //         this.hovered = false;
            //     }
            //     cx.notify();
            //     this
            // }))
            .child(
                h_flex()
                    .id("inner_list_item")
                    .group("list_item")
                    .w_full()
                    .relative()
                    .gap_1()
                    .px(DynamicSpacing::Base06.rems(cx))
                    .map(|this| match self.spacing {
                        ListItemSpacing::Dense => this,
                        ListItemSpacing::ExtraDense => this.py_neg_px(),
                        ListItemSpacing::Sparse => this.py_1(),
                    })
                    .when(self.inset && !self.disabled, |this| {
                        this
                            // TODO: Add focus state
                            //.when(self.state == InteractionState::Focused, |this| {
                            .when_some(self.focused, |this, focused| {
                                if focused {
                                    this.border_1()
                                        .border_color(cx.theme().colors().border_focused)
                                } else {
                                    this.border_1()
                                }
                            })
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
                    .when_some(
                        self.on_click.filter(|_| !self.disabled),
                        |this, on_click| this.cursor_pointer().on_click(on_click),
                    )
                    .when(self.outlined, |this| {
                        this.border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_sm()
                            .overflow_hidden()
                    })
                    .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                        this.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                            (on_mouse_down)(event, window, cx)
                        })
                    })
                    .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip))
                    .map(|this| {
                        if self.inset {
                            this.rounded_sm()
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
                            .when(is_open && !self.always_show_disclosure_icon, |this| {
                                this.visible_on_hover("")
                            })
                            .child(Disclosure::new("toggle", is_open).on_toggle(self.on_toggle))
                    }))
                    .child(
                        h_flex()
                            .flex_grow()
                            .flex_shrink_0()
                            .flex_basis(relative(0.25))
                            .gap(DynamicSpacing::Base06.rems(cx))
                            .map(|list_content| {
                                if self.overflow_x {
                                    list_content
                                } else {
                                    list_content.overflow_hidden()
                                }
                            })
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
                                .right(DynamicSpacing::Base06.rems(cx))
                                .top_0()
                                .visible_on_hover("list_item")
                                .child(end_hover_slot),
                        )
                    })
                    .when_some(self.end_slot_invisible_button, |this, container| {
                        let should_show = self.selected || self.focused.is_some() || self.hovered;

                        if should_show {
                            this.child(container)
                        } else {
                            this
                        }
                    }),
            )
    }
}
