use gpui::{
    Animation, AnimationExt, App, ClipboardItem, Context, DismissEvent, Element, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, MouseDownEvent,
    ParentElement, Render, SharedString, Styled, Subscription, Transformation, Window, div,
    percentage, rems, svg,
};
use menu;
use std::time::Duration;
use ui::{Button, Icon, IconName, Label, Vector, VectorName, prelude::*};

use crate::ModalView;

/// Configuration for the OAuth device flow modal.
/// This allows extensions to specify the text and appearance of the modal.
#[derive(Clone)]
pub struct OAuthDeviceFlowModalConfig {
    /// The user code to display (e.g., "ABC-123").
    pub user_code: String,
    /// The URL the user needs to visit to authorize (for the "Connect" button).
    pub verification_url: String,
    /// The headline text for the modal (e.g., "Use GitHub Copilot in Zed.").
    pub headline: String,
    /// A description to show below the headline.
    pub description: String,
    /// Label for the connect button (e.g., "Connect to GitHub").
    pub connect_button_label: String,
    /// Success headline shown when authorization completes.
    pub success_headline: String,
    /// Success message shown when authorization completes.
    pub success_message: String,
    /// Optional path to an SVG icon file (absolute path on disk).
    pub icon_path: Option<SharedString>,
}

/// The current status of the OAuth device flow.
#[derive(Clone, Debug)]
pub enum OAuthDeviceFlowStatus {
    /// Waiting for user to click connect and authorize.
    Prompting,
    /// User clicked connect, waiting for authorization.
    WaitingForAuthorization,
    /// Successfully authorized.
    Authorized,
    /// Authorization failed with an error message.
    Failed(String),
}

/// Shared state for the OAuth device flow that can be observed by the modal.
pub struct OAuthDeviceFlowState {
    pub config: OAuthDeviceFlowModalConfig,
    pub status: OAuthDeviceFlowStatus,
}

impl EventEmitter<()> for OAuthDeviceFlowState {}

impl OAuthDeviceFlowState {
    pub fn new(config: OAuthDeviceFlowModalConfig) -> Self {
        Self {
            config,
            status: OAuthDeviceFlowStatus::Prompting,
        }
    }

    /// Update the status of the OAuth flow.
    pub fn set_status(&mut self, status: OAuthDeviceFlowStatus, cx: &mut Context<Self>) {
        self.status = status;
        cx.emit(());
        cx.notify();
    }
}

/// A generic OAuth device flow modal that can be used by extensions.
pub struct OAuthDeviceFlowModal {
    state: Entity<OAuthDeviceFlowState>,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    _subscription: Subscription,
}

impl Focusable for OAuthDeviceFlowModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for OAuthDeviceFlowModal {}

impl ModalView for OAuthDeviceFlowModal {}

impl OAuthDeviceFlowModal {
    pub fn new(state: Entity<OAuthDeviceFlowState>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe(&state, |_, _, cx| {
            cx.notify();
        });

        Self {
            state,
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            _subscription: subscription,
        }
    }

    fn render_icon(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let icon_color = Color::Custom(cx.theme().colors().icon);
        // Match ZedXCopilot visual appearance
        let icon_size = rems(2.5);
        let plus_size = rems(0.875);
        // The "+" in ZedXCopilot SVG has fill-opacity="0.5"
        let plus_color = cx.theme().colors().icon.opacity(0.5);

        if let Some(icon_path) = &state.config.icon_path {
            // Show "[Provider Icon] + [Zed Logo]" format to match built-in Copilot modal
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::from_external_svg(icon_path.clone())
                        .size(ui::IconSize::Custom(icon_size))
                        .color(icon_color),
                )
                .child(
                    svg()
                        .size(plus_size)
                        .path("icons/plus.svg")
                        .text_color(plus_color),
                )
                .child(Vector::new(VectorName::ZedLogo, icon_size, icon_size).color(icon_color))
                .into_any_element()
        } else {
            // Fallback to just Zed logo if no provider icon
            Vector::new(VectorName::ZedLogo, icon_size, icon_size)
                .color(icon_color)
                .into_any_element()
        }
    }

    fn render_device_code(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let user_code = state.config.user_code.clone();
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text().as_ref() == Some(&user_code))
            .unwrap_or(false);
        let user_code_for_click = user_code.clone();

        h_flex()
            .w_full()
            .p_1()
            .border_1()
            .border_muted(cx)
            .rounded_sm()
            .cursor_pointer()
            .justify_between()
            .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(user_code_for_click.clone()));
                window.refresh();
            })
            .child(div().flex_1().child(Label::new(user_code)))
            .child(div().flex_none().px_1().child(Label::new(if copied {
                "Copied!"
            } else {
                "Copy"
            })))
    }

    fn render_prompting_modal(&self, cx: &mut Context<Self>) -> impl Element {
        let (connect_button_label, verification_url, headline, description) = {
            let state = self.state.read(cx);
            let label = if self.connect_clicked {
                "Waiting for connection...".to_string()
            } else {
                state.config.connect_button_label.clone()
            };
            (
                label,
                state.config.verification_url.clone(),
                state.config.headline.clone(),
                state.config.description.clone(),
            )
        };

        v_flex()
            .flex_1()
            .gap_2()
            .items_center()
            .child(Headline::new(headline).size(HeadlineSize::Large))
            .child(Label::new(description).color(Color::Muted))
            .child(self.render_device_code(cx))
            .child(
                Label::new("Paste this code into GitHub after clicking the button below.")
                    .size(ui::LabelSize::Small),
            )
            .child(
                Button::new("connect-button", connect_button_label)
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        cx.open_url(&verification_url);
                        this.connect_clicked = true;
                    }))
                    .full_width()
                    .style(ButtonStyle::Filled),
            )
            .child(
                Button::new("cancel-button", "Cancel")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| {
                        cx.emit(DismissEvent);
                    })),
            )
    }

    fn render_authorized_modal(&self, cx: &mut Context<Self>) -> impl Element {
        let state = self.state.read(cx);
        let success_headline = state.config.success_headline.clone();
        let success_message = state.config.success_message.clone();

        v_flex()
            .gap_2()
            .child(Headline::new(success_headline).size(HeadlineSize::Large))
            .child(Label::new(success_message))
            .child(
                Button::new("done-button", "Done")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_failed_modal(&self, error: &str, cx: &mut Context<Self>) -> impl Element {
        v_flex()
            .gap_2()
            .child(Headline::new("Authorization Failed").size(HeadlineSize::Large))
            .child(Label::new(error.to_string()).color(Color::Error))
            .child(
                Button::new("close-button", "Close")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_loading(window: &mut Window, _cx: &mut Context<Self>) -> impl Element {
        let loading_icon = svg()
            .size_8()
            .path(IconName::ArrowCircle.path())
            .text_color(window.text_style().color)
            .with_animation(
                "icon_circle_arrow",
                Animation::new(Duration::from_secs(2)).repeat(),
                |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
            );

        h_flex().justify_center().child(loading_icon)
    }
}

impl Render for OAuthDeviceFlowModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.state.read(cx).status.clone();

        let prompt = match &status {
            OAuthDeviceFlowStatus::Prompting => self.render_prompting_modal(cx).into_any_element(),
            OAuthDeviceFlowStatus::WaitingForAuthorization => {
                if self.connect_clicked {
                    self.render_prompting_modal(cx).into_any_element()
                } else {
                    Self::render_loading(window, cx).into_any_element()
                }
            }
            OAuthDeviceFlowStatus::Authorized => {
                self.render_authorized_modal(cx).into_any_element()
            }
            OAuthDeviceFlowStatus::Failed(error) => {
                self.render_failed_modal(error, cx).into_any_element()
            }
        };

        v_flex()
            .id("oauth-device-flow-modal")
            .track_focus(&self.focus_handle(cx))
            .elevation_3(cx)
            .w_96()
            .items_center()
            .p_4()
            .gap_2()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                window.focus(&this.focus_handle, cx);
            }))
            .child(self.render_icon(cx))
            .child(prompt)
    }
}
