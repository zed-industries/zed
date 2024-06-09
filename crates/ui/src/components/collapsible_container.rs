use crate::{prelude::*, ButtonLike};
use smallvec::SmallVec;

use gpui::*;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum ContainerStyle {
    #[default]
    None,
    Card,
}

struct ContainerStyles {
    pub background_color: Hsla,
    pub border_color: Hsla,
    pub text_color: Hsla,
}

#[derive(IntoElement)]
pub struct CollapsibleContainer {
    id: ElementId,
    base: ButtonLike,
    toggle: bool,
    /// A slot for content that appears before the label, like an icon or avatar.
    start_slot: Option<AnyElement>,
    /// A slot for content that appears after the label, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
    style: ContainerStyle,
    children: SmallVec<[AnyElement; 1]>,
}

impl CollapsibleContainer {
    pub fn new(id: impl Into<ElementId>, toggle: bool) -> Self {
        Self {
            id: id.into(),
            base: ButtonLike::new("button_base"),
            toggle,
            start_slot: None,
            end_slot: None,
            style: ContainerStyle::Card,
            children: SmallVec::new(),
        }
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn child<E: IntoElement>(mut self, child: E) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Clickable for CollapsibleContainer {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl RenderOnce for CollapsibleContainer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let color = cx.theme().colors();

        let styles = match self.style {
            ContainerStyle::None => ContainerStyles {
                background_color: color.ghost_element_background,
                border_color: color.border_transparent,
                text_color: color.text,
            },
            ContainerStyle::Card => ContainerStyles {
                background_color: color.elevated_surface_background,
                border_color: color.border,
                text_color: color.text,
            },
        };

        v_flex()
            .id(self.id)
            .relative()
            .rounded_md()
            .bg(styles.background_color)
            .border_1()
            .border_color(styles.border_color)
            .text_color(styles.text_color)
            .overflow_hidden()
            .child(
                h_flex()
                    .overflow_hidden()
                    .w_full()
                    .group("toggleable_container_header")
                    .border_b_1()
                    .border_color(if self.toggle {
                        styles.border_color
                    } else {
                        color.border_transparent
                    })
                    .child(
                        self.base.full_width().style(ButtonStyle::Subtle).child(
                            div()
                                .h_7()
                                .p_1()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_between()
                                .w_full()
                                .gap_1()
                                .cursor_pointer()
                                .group_hover("toggleable_container_header", |this| {
                                    this.bg(color.element_hover)
                                })
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            IconButton::new(
                                                "toggle_icon",
                                                match self.toggle {
                                                    true => IconName::ChevronDown,
                                                    false => IconName::ChevronRight,
                                                },
                                            )
                                            .icon_color(Color::Muted)
                                            .icon_size(IconSize::XSmall),
                                        )
                                        .child(
                                            div()
                                                .id("label_container")
                                                .flex()
                                                .gap_1()
                                                .items_center()
                                                .children(self.start_slot),
                                        ),
                                )
                                .child(h_flex().children(self.end_slot)),
                        ),
                    ),
            )
            .when(self.toggle, |this| {
                this.child(h_flex().flex_1().w_full().p_1().children(self.children))
            })
    }
}
