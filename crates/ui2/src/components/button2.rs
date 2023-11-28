use gpui::{
    Action, AnyElement, AnyView, DefiniteLength, DispatchPhase, Div, IntoElement, MouseButton,
    MouseDownEvent, Stateful, StatefulInteractiveElement, WindowContext,
};
use smallvec::SmallVec;

use crate::{h_stack, prelude::*};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonType2 {
    #[default]
    DefaultButton,
    ButtonLike,
    SplitButton,
    ToggleButton,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition2 {
    #[default]
    Before,
    After,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonAppearance2 {
    #[default]
    Filled,
    // Tinted,
    Subtle,
    Transparent,
}

impl ButtonAppearance2 {
    pub fn bg(
        self,
        cx: &mut WindowContext,
        state: InteractionState,
        selected: bool,
        disabled: bool,
    ) -> gpui::Hsla {
        match self {
            ButtonAppearance2::Filled => {
                if disabled {
                    cx.theme().colors().element_disabled
                } else if selected {
                    cx.theme().colors().element_selected
                } else if state == InteractionState::Hovered {
                    cx.theme().colors().element_hover
                } else if state == InteractionState::Active {
                    cx.theme().colors().element_active
                } else {
                    cx.theme().colors().element_background
                }
            }
            ButtonAppearance2::Subtle => {
                if disabled {
                    cx.theme().colors().ghost_element_disabled
                } else if selected {
                    cx.theme().colors().ghost_element_selected
                } else if state == InteractionState::Hovered {
                    cx.theme().colors().ghost_element_hover
                } else if state == InteractionState::Active {
                    cx.theme().colors().ghost_element_active
                } else {
                    cx.theme().colors().ghost_element_background
                }
            }
            ButtonAppearance2::Transparent => {
                if disabled {
                    gpui::transparent_black()
                } else if selected {
                    gpui::transparent_black()
                } else if state == InteractionState::Hovered {
                    gpui::transparent_black()
                } else if state == InteractionState::Active {
                    cx.theme().colors().ghost_element_active
                } else {
                    gpui::transparent_black()
                }
            }
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

pub trait ButtonCommon {
    fn id(&self) -> &ElementId;
    fn appearance(&mut self, appearance: ButtonAppearance2) -> &mut Self;
    fn state(&mut self, state: InteractionState) -> &mut Self;
    fn disabled(&mut self, disabled: bool) -> &mut Self;
    fn size(&mut self, size: ButtonSize2) -> &mut Self;
    fn tooltip(&mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> &mut Self;
    fn on_click(
        &mut self,
        handler: impl 'static + Fn(&MouseDownEvent, &mut WindowContext),
    ) -> &mut Self;
    fn action(&mut self, action: impl Action + 'static) -> &mut Self;
    // fn width(&mut self, width: DefiniteLength) -> &mut Self;
}

pub trait SelectableButtonCommon {
    fn selected(&mut self, selected: bool) -> &mut Self;
    fn selected_tooltip(
        &mut self,
        tooltip: Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>,
    ) -> &mut Self;
}

pub trait FixedButtonCommon {
    fn width(&mut self, width: DefiniteLength) -> &mut Self;
    fn full_width(&mut self) -> &mut Self;
}

fn button_action<E: InteractiveElement>(
    mut this: E,
    click_handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
) -> E {
    this.on_mouse_down(MouseButton::Left, move |event, cx| {
        cx.stop_propagation();
        click_handler(event, cx);
    });
    this
}

// pub struct Button {
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

// impl ButtonCommon for Button {
//     fn id(&self) -> &ElementId {
//         &self.id
//     }

//     fn appearance(&mut self, appearance: ButtonAppearance) -> &mut Self {
//         self.appearance = appearance;
//         self
//     }
//     // implement methods from ButtonCommon trait...
// }

// impl Button {
//     pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
//         Self {
//             id: id.into(),
//             label: Some(label.into()),
//             // initialize other fields with default values...
//         }
//     }

//     // ... Define other builder methods specific to Button type...
// }

// pub struct IconButton {
//     // Base properties...
//     id: ElementId,
//     icon: Icon,
//     icon_color: Option<Color>,
//     // More fields as necessary...
// }

// impl ButtonCommon for IconButton {
//     fn id(&self) -> &ElementId {
//         &self.id
//     }
//     // ... Implement other methods from ButtonCommon trait with builder patterns...
// }

// impl IconButton {
//     pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
//         Self {
//             id: id.into(),
//             icon,
//             // initialize other fields with default values...
//         }
//     }

//     // ... Define other builder methods specific to IconButton type...
// }

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    appearance: ButtonAppearance2,
    state: InteractionState,
    disabled: bool,
    size: ButtonSize2,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    on_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>, children: SmallVec<[AnyElement; 2]>) -> Self {
        Self {
            id: id.into(),
            appearance: ButtonAppearance2::default(),
            state: InteractionState::default(),
            disabled: false,
            size: ButtonSize2::Default,
            tooltip: None,
            children,
            on_mouse_down: None,
        }
    }
}

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn appearance(&mut self, appearance: ButtonAppearance2) -> &mut Self {
        self.appearance = appearance;
        self
    }

    fn state(&mut self, state: InteractionState) -> &mut Self {
        self.state = state;
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

    fn tooltip(
        &mut self,
        tooltip: Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>,
    ) -> &mut Self {
        self.tooltip = Some(tooltip);
        self
    }

    fn on_click(mut self, handler: impl 'static + Fn(&MouseDownEvent, &mut WindowContext)) -> Self {
        self.on_mouse_down = Some(Box::new(handler));
        self
    }
    fn action(self, action: Box<dyn Action>) -> Self {
        self.on_click(move |this, cx| cx.dispatch_action(action.boxed_clone()))
    }
}

impl RenderOnce for ButtonLike {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let background =
            ButtonAppearance2::bg(self.appearance, cx, self.state, false, self.disabled);

        let mut button_like = h_stack()
            .id(self.id.clone())
            .rounded_md()
            .cursor_pointer()
            .gap_1()
            .p_1();

        button_like
            .bg(ButtonAppearance2::bg(
                self.appearance,
                cx,
                self.state,
                false,
                self.disabled,
            ))
            .hover(|hover| {
                hover.bg(ButtonAppearance2::bg(
                    self.appearance,
                    cx,
                    InteractionState::Hovered,
                    false,
                    self.disabled,
                ))
            })
            .active(|active| {
                active.bg(ButtonAppearance2::bg(
                    self.appearance,
                    cx,
                    InteractionState::Active,
                    false,
                    self.disabled,
                ))
            });

        if let Some(click_handler) = self.on_mouse_down {
            button_like = button_like.on_mouse_down(MouseButton::Left, move |event, cx| {
                cx.stop_propagation();
                click_handler(event, cx);
            })
        }

        if let Some(tooltip) = self.tooltip {
            button_like = button_like.tooltip(move |cx| tooltip(cx))
        }

        button_like.children(self.children);

        button_like
    }
}

impl ParentElement for ButtonLike {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

// pub struct ToggleButton {
//     // based on either IconButton or Button, with additional 'selected: bool' property
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
