#![allow(unused, dead_code)]

use std::sync::Arc;

use gpui::{AppContext, FocusHandle, FocusableView, View};
use ui::{
    div, px, relative, v_flex, vh, ActiveTheme, Button, ButtonCommon, ButtonSize,
    CheckboxWithLabel, Color, ElementId, ElevationIndex, FixedWidth, FluentBuilder, Headline,
    HeadlineSize, InteractiveElement, IntoElement, Label, LabelCommon, ParentElement, Render,
    Selection, SharedString, Spacing, StatefulInteractiveElement, Styled, StyledExt,
    StyledTypography, ViewContext, VisualContext, WindowContext,
};

const MAX_DIALOG_WIDTH: f32 = 440.0;
const MIN_DIALOG_WIDTH: f32 = 260.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AlertDialogLayout {
    /// For dialogs short titles and action names.
    ///
    /// Example:
    ///
    /// Title: "Discard changes?"
    ///
    /// Actions: "Cancel" | "Discard"
    #[default]
    Vertical,
    /// For dialogs with long titles or action names,
    /// or large amounts of content.
    ///
    /// As titles, action names or content get longer, the dialog
    /// automatically switches to this layout
    Horizontal,
}

/// An alert dialog that interrupts the user's workflow to convey critical information.
///
/// Use this component when immediate user attention or action is required.
///
/// It blocks all other interactions until the user responds, making it suitable
/// for important confirmations or critical error messages.
pub struct AlertDialog {
    /// The title of the alert dialog
    pub title: SharedString,

    /// The main message or content of the alert
    pub message: Option<SharedString>,

    /// The primart action the user can take
    pub primary_action: SharedString,

    /// The secondary action the user can take
    pub secondary_action: SharedString,

    /// A optional checkbox to show in the dialog
    ///
    /// Used to allow the user to opt-in or out of a secondary action,
    /// such as "Don't ask again" or "Suggest extensions automatically"
    pub checkbox: Option<(SharedString)>,

    focus_handle: FocusHandle,
}

struct AlertDialogView {
    dialog: AlertDialog,
    focus_handle: FocusHandle,
}

impl AlertDialog {
    /// Create a new alert dialog
    pub fn new(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut ViewContext<Self>) -> Self,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();

            f(
                Self {
                    title: "Untitled Alert".into(),
                    message: None,
                    primary_action: "OK".into(),
                    secondary_action: "Cancel".into(),
                    checkbox: None,
                    focus_handle,
                },
                cx,
            )
        })
    }

    /// Set the title of the alert dialog
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the main message or content of the alert dialog
    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set the primary action the user can take
    pub fn primary_action(mut self, primary_action: impl Into<SharedString>) -> Self {
        self.primary_action = primary_action.into();
        self
    }

    /// Set the secondary action the user can take
    pub fn secondary_action(mut self, secondary_action: impl Into<SharedString>) -> Self {
        self.secondary_action = secondary_action.into();
        self
    }

    /// Sets the checkbox to show in the dialog
    pub fn checkbox(mut self, checkbox: impl Into<SharedString>) -> Self {
        self.checkbox = Some(checkbox.into());
        self
    }

    fn dialog_layout(&self) -> AlertDialogLayout {
        let title_len = self.title.len();
        let primary_action_len = self.primary_action.len();
        let secondary_action_len = self.secondary_action.len();
        let message_len = self.message.as_ref().map_or(0, |m| m.len());

        if title_len > 35
            || primary_action_len > 14
            || secondary_action_len > 14
            || message_len > 80
        {
            AlertDialogLayout::Horizontal
        } else {
            AlertDialogLayout::Vertical
        }
    }

    fn render_button(&self, cx: &WindowContext, label: SharedString) -> impl IntoElement {
        let id_string: SharedString = format!("action-{}-button", label).into();
        let id: ElementId = ElementId::Name(id_string);

        Button::new(id, label)
            .size(ButtonSize::Large)
            .layer(ElevationIndex::ModalSurface)
            .when(
                self.dialog_layout() == AlertDialogLayout::Vertical,
                |this| this.full_width(),
            )
    }
}

impl Render for AlertDialog {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layout = self.dialog_layout();
        let spacing = if layout == AlertDialogLayout::Horizontal {
            Spacing::Large4X.rems(cx)
        } else {
            Spacing::XLarge.rems(cx)
        };

        v_flex()
            .min_w(px(MIN_DIALOG_WIDTH))
            .max_w(if layout == AlertDialogLayout::Horizontal {
                px(MAX_DIALOG_WIDTH)
            } else {
                px(MIN_DIALOG_WIDTH)
            })
            .max_h(relative(0.75))
            .flex_none()
            .overflow_hidden()
            .p(spacing)
            .gap(spacing)
            .rounded_lg()
            .font_ui(cx)
            .bg(ElevationIndex::ModalSurface.bg(cx))
            .shadow(ElevationIndex::ModalSurface.shadow())
            // Title and message
            .child(
                v_flex()
                    .w_full()
                    .min_h(px(1.))
                    .max_w_full()
                    .flex_grow()
                    .when(layout == AlertDialogLayout::Vertical, |this| {
                        this.justify_center().mx_auto()
                    })
                    .child(
                        div()
                            .when(layout == AlertDialogLayout::Vertical, |this| this.mx_auto())
                            .child(Headline::new(self.title.clone()).size(HeadlineSize::Small)),
                    )
                    .when_some(self.message.clone(), |this, message| {
                        this.child(
                            div()
                                .when(layout == AlertDialogLayout::Vertical, |this| this.mx_auto())
                                .text_color(cx.theme().colors().text_muted)
                                .text_ui(cx)
                                .child(message.clone()),
                        )
                    }),
            )
            // Actions & checkbox
            .child(
                div()
                    .flex()
                    .w_full()
                    .items_center()
                    // Force buttons to stack for Horizontal layout
                    .when(layout == AlertDialogLayout::Vertical, |this| {
                        this.flex_col()
                    })
                    .when(layout == AlertDialogLayout::Horizontal, |this| {
                        this.justify_between()
                            .h(ButtonSize::Large.rems())
                            .gap(Spacing::Medium.rems(cx))
                    })
                    .child(div().flex_shrink_0().when_some(
                        self.checkbox.clone(),
                        |this, (label)| {
                            this.child(CheckboxWithLabel::new(
                                ElementId::Name(label.clone()),
                                Label::new(label).color(Color::Muted),
                                false.into(),
                                // TODO: Pass on_click through self.checkbox
                                |_, _| {},
                            ))
                        },
                    ))
                    .child(
                        div()
                            .flex()
                            .gap(Spacing::Medium.rems(cx))
                            .when(layout == AlertDialogLayout::Vertical, |this| {
                                this.flex_col().w_full()
                            })
                            .when(layout == AlertDialogLayout::Horizontal, |this| {
                                this.items_center()
                            })
                            .child(self.render_button(cx, self.secondary_action.clone()))
                            .child(self.render_button(cx, self.primary_action.clone())),
                    ),
            )
    }
}

impl FocusableView for AlertDialog {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Example stories for [AlertDialog]
///
/// Run with `script/storybook alert_dialog`
#[cfg(feature = "stories")]
pub mod alert_dialog_stories {
    #![allow(missing_docs)]

    use gpui::{Render, View};
    use story::{Story, StoryItem, StorySection};
    use ui::{prelude::*, ElevationIndex};

    use super::AlertDialog;

    pub struct AlertDialogStory {
        vertical_alert_dialog: View<AlertDialog>,
        horizontal_alert_dialog: View<AlertDialog>,
        long_content_alert_dialog: View<AlertDialog>,
    }

    impl AlertDialogStory {
        pub fn new(cx: &mut WindowContext) -> Self {
            let vertical_alert_dialog = AlertDialog::new(cx, |mut dialog, cx| {
                dialog
                    .title("Discard changes?")
                    .message("Something bad could happen...")
                    .primary_action("Discard")
                    .secondary_action("Cancel")
            });

            let horizontal_alert_dialog = AlertDialog::new(cx, |mut dialog, cx| {
                dialog
                    .title("Do you want to leave the current call?")
                    .message("The current window will be closed, and connections to any shared projects will be terminated.")
                    .checkbox("Don't show again")
                    .primary_action("Leave Call")
                    .secondary_action("Cancel")
            });

            let long_content = r#"{
  "error": "RuntimeError",
  "message": "An unexpected error occurred during execution",
  "stackTrace": [
    {
      "fileName": "main.rs",
      "lineNumber": 42,
      "functionName": "process_data"
    },
    {
      "fileName": "utils.rs",
      "lineNumber": 23,
      "functionName": "validate_input"
    },
    {
      "fileName": "core.rs",
      "lineNumber": 105,
      "functionName": "execute_operation"
    }
  ]
}"#;

            let long_content_alert_dialog = AlertDialog::new(cx, |mut dialog, cx| {
                dialog
                    .title("A RuntimeError occured")
                    .message(long_content)
                    .primary_action("Send Report")
                    .secondary_action("Close")
            });

            Self {
                vertical_alert_dialog,
                horizontal_alert_dialog,
                long_content_alert_dialog,
            }
        }
    }

    impl Render for AlertDialogStory {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
            Story::container().child(
                StorySection::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(780.))
                            .h(px(380.))
                            .overflow_hidden()
                            .bg(ElevationIndex::Background.bg(cx))
                            .child(self.vertical_alert_dialog.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(580.))
                            .h(px(420.))
                            .overflow_hidden()
                            .bg(ElevationIndex::Background.bg(cx))
                            .child(self.horizontal_alert_dialog.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(580.))
                            .h(px(780.))
                            .overflow_hidden()
                            .bg(ElevationIndex::Background.bg(cx))
                            .child(self.long_content_alert_dialog.clone()),
                    ),
            )
        }
    }
}
