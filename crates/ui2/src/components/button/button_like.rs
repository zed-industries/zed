use gpui::{relative, DefiniteLength};
use gpui::{rems, transparent_black, AnyElement, AnyView, ClickEvent, Div, Hsla, Rems, Stateful};
use smallvec::SmallVec;

use crate::h_stack;
use crate::prelude::*;

pub trait ButtonCommon: Clickable + Disableable {
    fn id(&self) -> &ElementId;
    fn style(self, style: ButtonStyle) -> Self;
    fn size(self, size: ButtonSize) -> Self;
    fn tooltip(self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Filled,
    // Tinted,
    Subtle,
    Transparent,
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonLikeStyles {
    pub background: Hsla,
    #[allow(unused)]
    pub border_color: Hsla,
    #[allow(unused)]
    pub label_color: Hsla,
    #[allow(unused)]
    pub icon_color: Hsla,
}

impl ButtonStyle {
    pub(crate) fn enabled(self, cx: &mut WindowContext) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_background,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_background,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
        }
    }

    pub(crate) fn hovered(self, cx: &mut WindowContext) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_hover,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    pub(crate) fn active(self, cx: &mut WindowContext) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    #[allow(unused)]
    pub(crate) fn focused(self, cx: &mut WindowContext) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Accent.color(cx),
                icon_color: Color::Accent.color(cx),
            },
        }
    }

    pub(crate) fn disabled(self, cx: &mut WindowContext) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    #[default]
    Default,
    Compact,
    None,
}

impl ButtonSize {
    fn height(self) -> Rems {
        match self {
            ButtonSize::Default => rems(22. / 16.),
            ButtonSize::Compact => rems(18. / 16.),
            ButtonSize::None => rems(16. / 16.),
        }
    }
}

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    pub(super) style: ButtonStyle,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    pub(super) width: Option<DefiniteLength>,
    size: ButtonSize,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: ButtonStyle::default(),
            disabled: false,
            selected: false,
            width: None,
            size: ButtonSize::Default,
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

impl Selectable for ButtonLike {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl FixedWidth for ButtonLike {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.width = Some(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.width = Some(relative(1.));
        self
    }
}

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
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
            .flex_none()
            .h(self.size.height())
            .when_some(self.width, |this, width| this.w(width))
            .rounded_md()
            .cursor_pointer()
            .gap_1()
            .px_1()
            .bg(self.style.enabled(cx).background)
            .hover(|hover| hover.bg(self.style.hovered(cx).background))
            .active(|active| active.bg(self.style.active(cx).background))
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |event, cx| {
                        cx.stop_propagation();
                        (on_click)(event, cx)
                    })
                },
            )
            .when_some(self.tooltip, |this, tooltip| {
                if !self.selected {
                    this.tooltip(move |cx| tooltip(cx))
                } else {
                    this
                }
            })
            .children(self.children)
    }
}
