use gpui::{
    AnyElement, AnyView, ClickEvent, Div, Hsla, IntoElement, Stateful, StatefulInteractiveElement,
    WindowContext,
};
use smallvec::SmallVec;

use crate::{h_stack, prelude::*};

// 🚧 Heavily WIP 🚧

// #[derive(Default, PartialEq, Clone, Copy)]
// pub enum ButtonType2 {
//     #[default]
//     DefaultButton,
//     IconButton,
//     ButtonLike,
//     SplitButton,
//     ToggleButton,
// }

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition2 {
    #[default]
    Before,
    After,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonStyle2 {
    #[default]
    Filled,
    // Tinted,
    Subtle,
    Transparent,
}

#[derive(Debug, Clone, Copy)]
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

// pub struct Button {
//     id: ElementId,
//     icon: Option<Icon>,
//     icon_color: Option<Color>,
//     icon_position: Option<IconPosition2>,
//     label: Option<Label>,
//     label_color: Option<Color>,
//     appearance: ButtonAppearance2,
//     state: InteractionState,
//     selected: bool,
//     disabled: bool,
//     tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
//     width: Option<DefiniteLength>,
//     action: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
//     secondary_action: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
//     /// Used to pass down some content to the button
//     /// to enable creating custom buttons.
//     children: SmallVec<[AnyElement; 2]>,
// }

pub trait ButtonCommon: Clickable {
    fn id(&self) -> &ElementId;
    fn style(&mut self, style: ButtonStyle2) -> &mut Self;
    fn disabled(&mut self, disabled: bool) -> &mut Self;
    fn size(&mut self, size: ButtonSize2) -> &mut Self;
    fn tooltip(&mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> &mut Self;
    // fn width(&mut self, width: DefiniteLength) -> &mut Self;
}

// pub struct LabelButton {
//     // Base properties...
//     id: ElementId,
//     appearance: ButtonAppearance,
//     state: InteractionState,
//     disabled: bool,
//     size: ButtonSize,
//     tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
//     width: Option<DefiniteLength>,
//     // Button-specific properties...
//     label: Option<SharedString>,
//     label_color: Option<Color>,
//     icon: Option<Icon>,
//     icon_color: Option<Color>,
//     icon_position: Option<IconPosition>,
//     // Define more fields for additional properties as needed
// }

// impl ButtonCommon for LabelButton {
//     fn id(&self) -> &ElementId {
//         &self.id
//     }

//     fn appearance(&mut self, appearance: ButtonAppearance) -> &mut Self {
//         self.style= style;
//         self
//     }
//     // implement methods from ButtonCommon trait...
// }

// impl LabelButton {
//     pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
//         Self {
//             id: id.into(),
//             label: Some(label.into()),
//             // initialize other fields with default values...
//         }
//     }

//     // ... Define other builder methods specific to Button type...
// }

// TODO: Icon Button

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    style: ButtonStyle2,
    disabled: bool,
    size: ButtonSize2,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn children(
        &mut self,
        children: impl IntoIterator<Item = impl Into<AnyElement>>,
    ) -> &mut Self {
        self.children = children.into_iter().map(Into::into).collect();
        self
    }

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

impl Clickable for ButtonLike {
    fn on_click(
        &mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> &mut Self {
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

    fn style(&mut self, style: ButtonStyle2) -> &mut Self {
        self.style = style;
        self
    }

    fn disabled(&mut self, disabled: bool) -> &mut Self {
        self.disabled = disabled;
        self
    }

    fn size(&mut self, size: ButtonSize2) -> &mut Self {
        self.size = size;
        self
    }

    fn tooltip(&mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> &mut Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for ButtonLike {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let mut button_like = h_stack()
            .id(self.id.clone())
            .rounded_md()
            .cursor_pointer()
            .gap_1()
            .p_1()
            .bg(self.style.enabled(cx).background)
            .hover(|hover| hover.bg(self.style.hovered(cx).background))
            .active(|active| active.bg(self.style.active(cx).background))
            .on_click({
                let on_click = self.on_click;
                move |event, cx| {
                    if let Some(on_click) = &on_click {
                        (on_click)(event, cx)
                    }
                }
            });

        for child in self.children.into_iter() {
            button_like = button_like.child(child);
        }

        if let Some(tooltip) = self.tooltip {
            button_like = button_like.tooltip(move |cx| tooltip(cx))
        }

        button_like
    }
}

impl ParentElement for ButtonLike {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

// pub struct ToggleButton {
//     // based on either IconButton2 or Button, with additional 'selected: bool' property
// }

// impl ButtonCommon for ToggleButton {
//     fn id(&self) -> &ElementId {
//         &self.id
//     }
//     // ... Implement other methods from ButtonCommon trait with builder patterns...
// }

// impl ToggleButton {
//     pub fn new() -> Self {
//         // Initialize with default values
//         Self {
//             // ... initialize fields, possibly with defaults or required parameters...
//         }
//     }

//     // ... Define other builder methods specific to ToggleButton type...
// }

// pub struct SplitButton {
//     // Base properties...
//     id: ElementId,
//     // Button-specific properties, possibly including a DefaultButton
//     secondary_action: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>>,
//     // More fields as necessary...
// }

// impl ButtonCommon for SplitButton {
//     fn id(&self) -> &ElementId {
//         &self.id
//     }
//     // ... Implement other methods from ButtonCommon trait with builder patterns...
// }

// impl SplitButton {
//     pub fn new(id: impl Into<ElementId>) -> Self {
//         Self {
//             id: id.into(),
//             // ... initialize other fields with default values...
//         }
//     }

//     // ... Define other builder methods specific to SplitButton type...
// }
