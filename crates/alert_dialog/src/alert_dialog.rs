#![allow(unused, dead_code)]
#![deny(missing_docs)]
//! Provides the Alert Dialog UI component – A modal dialog that interrupts the user's workflow to convey critical information.

use std::sync::Arc;

use gpui::{
    Action, AppContext, ClickEvent, DismissEvent, EventEmitter, FocusHandle, FocusableView, View,
    WeakView,
};
use ui::{
    div, px, relative, v_flex, vh, ActiveTheme, Button, ButtonCommon, ButtonSize,
    CheckboxWithLabel, Clickable, Color, ElementId, ElevationIndex, FixedWidth, FluentBuilder,
    Headline, HeadlineSize, IconButton, IconButtonShape, IconName, InteractiveElement, IntoElement,
    Label, LabelCommon, ParentElement, Render, RenderOnce, Selection, SharedString, Spacing,
    StatefulInteractiveElement, Styled, StyledExt, StyledTypography, ViewContext, VisualContext,
    WindowContext,
};
use workspace::{ModalView, Workspace};

#[derive(Clone, IntoElement)]
struct AlertDialogButton {
    id: ElementId,
    label: SharedString,
    on_click: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    focus_handle: FocusHandle,
}

impl AlertDialogButton {
    fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            on_click: None,
            focus_handle,
        }
    }

    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for AlertDialogButton {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        Button::new(self.id, self.label)
            .size(ButtonSize::Large)
            .layer(ElevationIndex::ModalSurface)
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, cx| {
                    on_click(event, cx);
                })
            })
    }
}

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
#[derive(Clone)]
pub struct AlertDialog {
    /// The title of the alert dialog
    pub title: SharedString,

    /// The main message or content of the alert
    pub message: Option<SharedString>,

    /// The primary action the user can take
    primary_action: AlertDialogButton,

    /// The secondary action the user can take
    secondary_action: AlertDialogButton,

    focus_handle: FocusHandle,
}

impl EventEmitter<DismissEvent> for AlertDialog {}
impl FocusableView for AlertDialog {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl ModalView for AlertDialog {
    fn fade_out_background(&self) -> bool {
        true
    }
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
                    primary_action: AlertDialogButton::new(
                        "primary-action",
                        "OK",
                        focus_handle.clone(),
                    ),
                    secondary_action: AlertDialogButton::new(
                        "secondary-action",
                        "Cancel",
                        focus_handle.clone(),
                    ),
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
    pub fn primary_action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.primary_action =
            AlertDialogButton::new("primary-action", label, self.focus_handle.clone())
                .on_click(handler);
        self
    }

    /// Set the secondary action the user can take
    pub fn secondary_action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.secondary_action =
            AlertDialogButton::new("secondary-action", label, self.focus_handle.clone())
                .on_click(handler);
        self
    }

    /// Set the secondary action to a dismiss action with a custom label
    ///
    /// Example: "Close", "Dismiss", "No"
    pub fn secondary_dismiss_action(mut self, label: impl Into<SharedString>) -> Self {
        self.secondary_action =
            AlertDialogButton::new("secondary-action", label, self.focus_handle.clone())
                .on_click(|_, cx| cx.dispatch_action(menu::Cancel.boxed_clone()));
        self
    }

    fn render_button(&self, cx: &WindowContext, action: AlertDialogButton) -> impl IntoElement {
        let id_string: SharedString = format!("action-{}-button", action.label).into();
        let id: ElementId = ElementId::Name(id_string);

        Button::new(id, action.label)
            .size(ButtonSize::Large)
            .layer(ElevationIndex::ModalSurface)
            .when(
                self.dialog_layout() == AlertDialogLayout::Vertical,
                |this| this.full_width(),
            )
            .when_some(action.on_click, |this, on_click| {
                this.on_click(move |event, cx| {
                    on_click(event, cx);
                })
            })
    }

    fn dialog_layout(&self) -> AlertDialogLayout {
        let title_len = self.title.len();
        let primary_action_len = self.primary_action.label.len();
        let secondary_action_len = self.secondary_action.label.len();
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

    /// Spawns the alert dialog in a new modal
    pub fn show(&self, workspace: WeakView<Workspace>, cx: &mut ViewContext<Self>) {
        let this = self.clone();
        let focus_handle = self.focus_handle.clone();
        cx.spawn(|_, mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| this);
                cx.focus(&focus_handle);
            })
        })
        .detach();
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
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
            .key_context("Alert")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .occlude()
            .min_w(px(MIN_DIALOG_WIDTH))
            .max_w(if layout == AlertDialogLayout::Horizontal {
                px(MAX_DIALOG_WIDTH)
            } else {
                px(MIN_DIALOG_WIDTH)
            })
            .max_h(vh(0.75, cx))
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
                    // This is a flex hack. Layout breaks without it ¯\_(ツ)_/¯
                    .min_h(px(1.))
                    .max_w_full()
                    .flex_grow()
                    .when(layout == AlertDialogLayout::Vertical, |this| {
                        // If we had `.text_center()` we would use it here instead of centering the content
                        // since this approach will only work as long as the content is a single line
                        this.justify_center().mx_auto()
                    })
                    .child(
                        div()
                            // Same as above, if `.text_center()` is supported in the future, use here.
                            .when(layout == AlertDialogLayout::Vertical, |this| this.mx_auto())
                            .child(Headline::new(self.title.clone()).size(HeadlineSize::Small)),
                    )
                    .when_some(self.message.clone(), |this, message| {
                        // TODO: When content will be long (example: a document, log or stack trace)
                        // we should render some sort of styled container, as well as allow the content to scroll
                        this.child(
                            div()
                                // Same as above, if `.text_center()` is supported in the future, use here.
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
                    .child(div().flex_shrink_0())
                    .child(
                        div()
                            .flex()
                            .gap(Spacing::Medium.rems(cx))
                            .when(layout == AlertDialogLayout::Vertical, |this| {
                                this.flex_col_reverse().w_full()
                            })
                            .when(layout == AlertDialogLayout::Horizontal, |this| {
                                this.items_center()
                            })
                            .child(self.secondary_action.clone())
                            .child(self.primary_action.clone()),
                    ),
            )
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
                    .primary_action("Discard", |_, _| {
                        println!("Discarded!");
                    })
                    .secondary_action("Cancel", |_, _| {
                        println!("Cancelled!");
                    })
            });

            let horizontal_alert_dialog = AlertDialog::new(cx, |mut dialog, cx| {
                dialog
                    .title("Do you want to leave the current call?")
                    .message("The current window will be closed, and connections to any shared projects will be terminated.")
                    .primary_action("Leave Call", |_, _| {})
                    .secondary_action("Cancel", |_, _| {})
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
                    .title("A RuntimeError occurred")
                    .message(long_content)
                    .primary_action("Send Report", |_, _| {})
                    .secondary_action("Close", |_, _| {})
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
