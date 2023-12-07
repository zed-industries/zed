use std::cmp::Ordering;
use std::rc::Rc;

use gpui::{AnyElement, AnyView, ClickEvent, IntoElement, MouseButton};
use smallvec::SmallVec;

use crate::prelude::*;

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

#[derive(IntoElement)]
pub struct Tab {
    id: ElementId,
    position: TabPosition,
    selected: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            position: TabPosition::First,
            on_click: None,
            tooltip: None,
            children: SmallVec::new(),
        }
    }

    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl Selectable for Tab {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Tab {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for Tab {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        const HEIGHT_IN_REMS: f32 = 30. / 16.;

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

        div()
            .h(rems(HEIGHT_IN_REMS))
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
                        this.pr_px().pl_px().border_b()
                    }
                }
                TabPosition::Middle(Ordering::Equal) => this.border_l().border_r().pb_px(),
                TabPosition::Middle(Ordering::Less) => this.border_l().pr_px().border_b(),
                TabPosition::Middle(Ordering::Greater) => this.border_r().pl_px().border_b(),
            })
            .child(
                h_stack()
                    .group("")
                    .id(self.id)
                    .relative()
                    .h_full()
                    .px_5()
                    .gap_1()
                    .text_color(text_color)
                    // .hover(|style| style.bg(tab_hover_bg))
                    // .active(|style| style.bg(tab_active_bg))
                    .when_some(self.on_click, |tab, on_click| {
                        tab.cursor_pointer().on_click(move |event, cx| {
                            // HACK: GPUI currently fires `on_click` with any mouse button,
                            // but we only care about the left button.
                            if event.down.button == MouseButton::Left {
                                (on_click)(event, cx)
                            }
                        })
                    })
                    .when_some(self.tooltip, |tab, tooltip| {
                        tab.tooltip(move |cx| tooltip(cx))
                    })
                    // .child(indicator)
                    // .child(close_button)
                    .children(self.children),
            )
    }
}
