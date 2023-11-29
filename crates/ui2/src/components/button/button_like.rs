use gpui::{rems, AnyElement, AnyView, ClickEvent, Div, Hsla, Rems, Stateful};
use smallvec::SmallVec;

use crate::h_stack;
use crate::prelude::*;

pub trait ButtonCommon: Clickable + Disableable {
    fn id(&self) -> &ElementId;
    fn style(self, style: ButtonStyle2) -> Self;
    fn size(self, size: ButtonSize2) -> Self;
    fn tooltip(self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ButtonStyle2 {
    #[default]
    Filled,
    // Tinted,
    Subtle,
    Transparent,
}

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub background: Hsla,
    pub border_color: Hsla,
    pub label_color: Hsla,
    pub icon_color: Hsla,
}

impl ButtonStyle2 {
    pub fn enabled(self, cx: &mut WindowContext) -> ButtonStyle {
        match self {
            ButtonStyle2::Filled => ButtonStyle {
                background: cx.theme().colors().element_background,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Subtle => ButtonStyle {
                background: cx.theme().colors().ghost_element_background,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Transparent => ButtonStyle {
                background: gpui::transparent_black(),
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
        }
    }

    pub fn hovered(self, cx: &mut WindowContext) -> ButtonStyle {
        match self {
            ButtonStyle2::Filled => ButtonStyle {
                background: cx.theme().colors().element_hover,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Subtle => ButtonStyle {
                background: cx.theme().colors().ghost_element_hover,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Transparent => ButtonStyle {
                background: gpui::transparent_black(),
                border_color: gpui::transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    pub fn active(self, cx: &mut WindowContext) -> ButtonStyle {
        match self {
            ButtonStyle2::Filled => ButtonStyle {
                background: cx.theme().colors().element_active,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Subtle => ButtonStyle {
                background: cx.theme().colors().ghost_element_active,
                border_color: gpui::transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Transparent => ButtonStyle {
                background: gpui::transparent_black(),
                border_color: gpui::transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    pub fn focused(self, cx: &mut WindowContext) -> ButtonStyle {
        match self {
            ButtonStyle2::Filled => ButtonStyle {
                background: cx.theme().colors().element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Subtle => ButtonStyle {
                background: cx.theme().colors().ghost_element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle2::Transparent => ButtonStyle {
                background: gpui::transparent_black(),
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Accent.color(cx),
                icon_color: Color::Accent.color(cx),
            },
        }
    }

    pub fn disabled(self, cx: &mut WindowContext) -> ButtonStyle {
        match self {
            ButtonStyle2::Filled => ButtonStyle {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle2::Subtle => ButtonStyle {
                background: cx.theme().colors().ghost_element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle2::Transparent => ButtonStyle {
                background: gpui::transparent_black(),
                border_color: gpui::transparent_black(),
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonSize2 {
    #[default]
    Default,
    Compact,
    None,
}

impl ButtonSize2 {
    fn height(self) -> Rems {
        match self {
            ButtonSize2::Default => rems(22. / 16.),
            ButtonSize2::Compact => rems(18. / 16.),
            ButtonSize2::None => rems(16. / 16.),
        }
    }
}

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    pub(super) style: ButtonStyle2,
    pub(super) disabled: bool,
    size: ButtonSize2,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: ButtonStyle2::default(),
            disabled: false,
            size: ButtonSize2::Default,
            tooltip: None,
            children: SmallVec::new(),
            on_click: None,
        }
    }
}

impl Disableable for ButtonLike {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

// impl Selectable for ButtonLike {
//     fn selected(&mut self, selected: bool) -> &mut Self {
//         todo!()
//     }

//     fn selected_tooltip(
//         &mut self,
//         tooltip: Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>,
//     ) -> &mut Self {
//         todo!()
//     }
// }

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn style(mut self, style: ButtonStyle2) -> Self {
        self.style = style;
        self
    }

    fn size(mut self, size: ButtonSize2) -> Self {
        self.size = size;
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl ParentElement for ButtonLike {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for ButtonLike {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        h_stack()
            .id(self.id.clone())
            .h(self.size.height())
            .rounded_md()
            .cursor_pointer()
            .gap_1()
            .px_1()
            .bg(self.style.enabled(cx).background)
            .hover(|hover| hover.bg(self.style.hovered(cx).background))
            .active(|active| active.bg(self.style.active(cx).background))
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |event, cx| (on_click)(event, cx)),
            )
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |cx| tooltip(cx))
            })
            .children(self.children)
    }
}
