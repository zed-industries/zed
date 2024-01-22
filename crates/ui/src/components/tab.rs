use crate::prelude::*;
use gpui::{AnyElement, IntoElement, Stateful};
use smallvec::SmallVec;
use std::cmp::Ordering;

/// The position of a [`Tab`] within a list of tabs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabPosition {
    /// The tab is first in the list.
    First,

    /// The tab is in the middle of the list (i.e., it is not the first or last tab).
    ///
    /// The [`Ordering`] is where this tab is positioned with respect to the selected tab.
    Middle(Ordering),

    /// The tab is last in the list.
    Last,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabCloseSide {
    Start,
    End,
}

#[derive(IntoElement)]
pub struct Tab {
    div: Stateful<Div>,
    selected: bool,
    position: TabPosition,
    close_side: TabCloseSide,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            div: div()
                .id(id.clone())
                .debug_selector(|| format!("TAB-{}", id)),
            selected: false,
            position: TabPosition::First,
            close_side: TabCloseSide::End,
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
        }
    }

    pub const CONTAINER_HEIGHT_IN_REMS: f32 = 29. / 16.;

    const CONTENT_HEIGHT_IN_REMS: f32 = 28. / 16.;

    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }

    pub fn close_side(mut self, close_side: TabCloseSide) -> Self {
        self.close_side = close_side;
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl Selectable for Tab {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    #[allow(refining_impl_trait)]
    fn render(self, cx: &mut WindowContext) -> Stateful<Div> {
        let (text_color, tab_bg, _tab_hover_bg, _tab_active_bg) = match self.selected {
            false => (
                cx.theme().colors().text_muted,
                cx.theme().colors().tab_inactive_background,
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().ghost_element_active,
            ),
            true => (
                cx.theme().colors().text,
                cx.theme().colors().tab_active_background,
                cx.theme().colors().element_hover,
                cx.theme().colors().element_active,
            ),
        };

        self.div
            .h(rems(Self::CONTAINER_HEIGHT_IN_REMS))
            .bg(tab_bg)
            .border_color(cx.theme().colors().border)
            .map(|this| match self.position {
                TabPosition::First => {
                    if self.selected {
                        this.pl_px().border_r().pb_px()
                    } else {
                        this.pl_px().pr_px().border_b()
                    }
                }
                TabPosition::Last => {
                    if self.selected {
                        this.border_l().border_r().pb_px()
                    } else {
                        this.pr_px().pl_px().border_b().border_r()
                    }
                }
                TabPosition::Middle(Ordering::Equal) => this.border_l().border_r().pb_px(),
                TabPosition::Middle(Ordering::Less) => this.border_l().pr_px().border_b(),
                TabPosition::Middle(Ordering::Greater) => this.border_r().pl_px().border_b(),
            })
            .cursor_pointer()
            .child(
                h_flex()
                    .group("")
                    .relative()
                    .h(rems(Self::CONTENT_HEIGHT_IN_REMS))
                    .px_5()
                    .gap_1()
                    .text_color(text_color)
                    // .hover(|style| style.bg(tab_hover_bg))
                    // .active(|style| style.bg(tab_active_bg))
                    .child(
                        h_flex()
                            .w_3()
                            .h_3()
                            .justify_center()
                            .absolute()
                            .map(|this| match self.close_side {
                                TabCloseSide::Start => this.right_1(),
                                TabCloseSide::End => this.left_1(),
                            })
                            .children(self.start_slot),
                    )
                    .child(
                        h_flex()
                            .w_3()
                            .h_3()
                            .justify_center()
                            .absolute()
                            .map(|this| match self.close_side {
                                TabCloseSide::Start => this.left_1(),
                                TabCloseSide::End => this.right_1(),
                            })
                            .visible_on_hover("")
                            .children(self.end_slot),
                    )
                    .children(self.children),
            )
    }
}
