use gpui::{
    rems, AnyElement, AnyView, ClickEvent, Div, Hsla, IntoElement, Rems, Stateful,
    StatefulInteractiveElement, WindowContext,
};
use smallvec::SmallVec;

use crate::{h_stack, prelude::*};

// 🚧 Heavily WIP 🚧

// #[derive(Default, PartialEq, Clone, Copy)]
// pub enum ButtonType2 {
//     #[default]
//     Button
//     ButtonLike, // if this can  be in button we can remove it
//     SplitButton, // Should just be it's own file/component
//     ToggleButton, // not it's own type, just implements Selectable
// }

// LabelButton:
// Button::label()
//
// IconButton:
// Button::icon()
//
// ButtonLike/CustomButton:
// Button::custom()

// About disabled:
// - [x] Disabled buttons should not be clickable
// - [ ] When disabled, button style should be clobbered with the disabled style (same a selected)

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition2 {
    #[default]
    Before,
    After,
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
