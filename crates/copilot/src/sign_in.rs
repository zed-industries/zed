use crate::{Copilot, Status, request::PromptUserDeviceFlow};
use gpui::{
    Animation, AnimationExt, App, AsyncApp, ClickEvent, ClipboardItem, Entity, EventEmitter,
    FocusHandle, Focusable, MouseDownEvent, Subscription, Transformation, WeakEntity, Window,
    actions, percentage, rems, svg,
};
use ui::{
    Button, ButtonStyle, Color, Headline, HeadlineSize, IconName, IntoElement, Label,
    ParentElement, Render, Vector, VectorName, div, h_flex, prelude::*, v_flex,
};

use std::time::Duration;
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::{ModalView, Toast, Workspace};

actions!(copilot, [Cancel]);

const COPILOT_SIGN_UP_URL: &str = "https://github.com/features/copilot";
const LOADING_ANIMATION_DURATION: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq)]
enum ConnectionState {
    Initial,
    Connected,
    Failed(String),
}

#[derive(Debug)]
pub enum SignInError {
    CopilotNotAvailable,
    WorkspaceNotFound,
    SignInFailed(String),
}

impl std::fmt::Display for SignInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignInError::CopilotNotAvailable => write!(f, "GitHub Copilot is not available"),
            SignInError::WorkspaceNotFound => write!(f, "No workspace found"),
            SignInError::SignInFailed(msg) => write!(f, "Sign-in failed: {}", msg),
        }
    }
}

impl std::error::Error for SignInError {}

impl From<anyhow::Error> for SignInError {
    fn from(err: anyhow::Error) -> Self {
        SignInError::SignInFailed(err.to_string())
    }
}

struct CopilotToastManager;

impl CopilotToastManager {
    fn show_status_toast(
        workspace: &mut Workspace,
        message: String,
        cx: &mut gpui::Context<Workspace>,
    ) {
        workspace.show_toast(
            Toast::new(NotificationId::unique::<CopilotStatusToast>(), message),
            cx,
        );
    }

    fn dismiss_status_toast(workspace: &mut Workspace, cx: &mut gpui::Context<Workspace>) {
        workspace.dismiss_toast(&NotificationId::unique::<CopilotStatusToast>(), cx);
    }
}

struct CopilotSignInService;

impl CopilotSignInService {
    fn validate_preconditions(
        window: &Window,
        cx: &App,
    ) -> Result<(Entity<Copilot>, Entity<Workspace>), SignInError> {
        let copilot = Copilot::global(cx).ok_or(SignInError::CopilotNotAvailable)?;

        let workspace = window
            .root::<Workspace>()
            .flatten()
            .ok_or(SignInError::WorkspaceNotFound)?;

        Ok((copilot, workspace))
    }

    fn ensure_copilot_started(copilot: &Entity<Copilot>, cx: &mut gpui::Context<Workspace>) {
        if matches!(copilot.read(cx).status(), Status::Disabled) {
            copilot.update(cx, |copilot, cx| copilot.start_copilot(false, true, cx));
        }
    }

    async fn handle_copilot_startup(
        copilot: Entity<Copilot>,
        workspace: WeakEntity<Workspace>,
        _is_reinstall: bool,
        cx: &mut AsyncApp,
    ) -> Result<(), SignInError> {
        workspace.update(cx, |workspace, cx| {
            match copilot.read(cx).status() {
                Status::Authorized => {
                    CopilotToastManager::show_status_toast(
                        workspace,
                        "Copilot has started.".to_string(),
                        cx,
                    );
                }
                _ => {
                    CopilotToastManager::dismiss_status_toast(workspace, cx);
                }
            }
        })?;

        Ok(())
    }

    fn start_sign_in_flow(
        workspace: &mut Workspace,
        copilot: &Entity<Copilot>,
        window: &mut Window,
        cx: &mut gpui::Context<Workspace>,
    ) -> Result<(), SignInError> {
        let task = copilot.update(cx, |copilot, cx| copilot.sign_in(cx));
        task.detach();

        workspace.toggle_modal(window, cx, |_, cx| {
            CopilotCodeVerification::new(copilot, cx)
        });

        Ok(())
    }
}

pub fn initiate_sign_in(window: &mut Window, cx: &mut App) -> Result<(), SignInError> {
    let (copilot, workspace) = CopilotSignInService::validate_preconditions(window, cx)?;

    workspace.update(cx, |workspace, cx| {
        initiate_sign_in_within_workspace(workspace, copilot, false, window, cx)
    })
}

pub fn reinstall_and_sign_in(window: &mut Window, cx: &mut App) -> Result<(), SignInError> {
    let (copilot, workspace) = CopilotSignInService::validate_preconditions(window, cx)?;

    workspace.update(cx, |workspace, cx| {
        reinstall_and_sign_in_within_workspace(workspace, copilot, window, cx)
    })
}

pub fn reinstall_and_sign_in_within_workspace(
    workspace: &mut Workspace,
    copilot: Entity<Copilot>,
    window: &mut Window,
    cx: &mut gpui::Context<Workspace>,
) -> Result<(), SignInError> {
    let _shared_task = copilot.update(cx, |copilot, cx| copilot.reinstall(cx));

    initiate_sign_in_within_workspace(workspace, copilot, true, window, cx)
}

pub fn initiate_sign_in_within_workspace(
    workspace: &mut Workspace,
    copilot: Entity<Copilot>,
    is_reinstall: bool,
    window: &mut Window,
    cx: &mut gpui::Context<Workspace>,
) -> Result<(), SignInError> {
    CopilotSignInService::ensure_copilot_started(&copilot, cx);

    match copilot.read(cx).status() {
        Status::Starting { task } => {
            let message = if is_reinstall {
                "Copilot is reinstalling..."
            } else {
                "Copilot is starting..."
            };

            CopilotToastManager::show_status_toast(workspace, message.to_string(), cx);

            let weak_workspace = workspace.weak_handle();
            let task_clone = task.clone();

            cx.spawn({
                let weak_workspace = weak_workspace.clone();
                async move |_handle, mut cx| {
                    task_clone.await;
                    if let Err(e) = CopilotSignInService::handle_copilot_startup(
                        copilot,
                        weak_workspace.clone(),
                        is_reinstall,
                        &mut cx,
                    )
                    .await
                    {
                        weak_workspace
                            .update(cx, |workspace, cx| {
                                workspace.show_error(&e.to_string(), cx);
                            })
                            .log_err();
                    }
                }
            })
            .detach();
        }
        _ => {
            CopilotSignInService::start_sign_in_flow(workspace, &copilot, window, cx)?;
        }
    }

    Ok(())
}

pub fn sign_out_within_workspace(
    workspace: &mut Workspace,
    copilot: Entity<Copilot>,
    cx: &mut gpui::Context<Workspace>,
) {
    CopilotToastManager::show_status_toast(workspace, "Signing out of Copilot...".to_string(), cx);

    let sign_out_task = copilot.update(cx, |copilot, cx| copilot.sign_out(cx));

    cx.spawn(
        async move |workspace: WeakEntity<Workspace>, cx| match sign_out_task.await {
            Ok(()) => {
                workspace
                    .update(cx, |workspace, cx| {
                        CopilotToastManager::show_status_toast(
                            workspace,
                            "Signed out of Copilot.".to_string(),
                            cx,
                        );
                    })
                    .log_err();
            }
            Err(err) => {
                workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_error(&err.to_string(), cx);
                    })
                    .log_err();
            }
        },
    )
    .detach();
}

pub struct CopilotCodeVerification {
    status: Status,
    connection_state: ConnectionState,
    focus_handle: FocusHandle,
    copilot: Entity<Copilot>,
    _subscription: Subscription,
}

impl Focusable for CopilotCodeVerification {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<gpui::DismissEvent> for CopilotCodeVerification {}

impl ModalView for CopilotCodeVerification {
    fn on_before_dismiss(
        &mut self,
        _: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> workspace::DismissDecision {
        if let Status::SigningIn { .. } = self.copilot.read(cx).status() {
            self.copilot.update(cx, |copilot, cx| {
                copilot.sign_out(cx).detach_and_log_err(cx);
            });
        }
        workspace::DismissDecision::Dismiss(true)
    }
}

impl CopilotCodeVerification {
    pub fn new(copilot: &Entity<Copilot>, cx: &mut gpui::Context<Self>) -> Self {
        let status = copilot.read(cx).status();

        Self {
            status,
            connection_state: ConnectionState::Initial,
            focus_handle: cx.focus_handle(),
            copilot: copilot.clone(),
            _subscription: cx.observe(copilot, Self::handle_status_update),
        }
    }

    fn handle_status_update(&mut self, copilot: Entity<Copilot>, cx: &mut gpui::Context<Self>) {
        let new_status = copilot.read(cx).status();

        match new_status {
            Status::Authorized => {
                self.status = new_status;
                self.connection_state = ConnectionState::Connected;
                cx.notify();
            }
            Status::Unauthorized => {
                self.status = new_status;
                self.connection_state = ConnectionState::Failed("Unauthorized".to_string());
                cx.notify();
            }
            Status::SigningIn { .. } => {
                self.status = new_status;
                cx.notify();
            }
            _ => {
                cx.emit(gpui::DismissEvent);
            }
        }
    }

    fn render_device_code(
        &self,
        data: &PromptUserDeviceFlow,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let user_code = data.user_code.clone();
        let is_copied = self.is_code_in_clipboard(&user_code, cx);

        let copy_label = if is_copied { "✓ Copied!" } else { "Copy" };

        h_flex()
            .w_full()
            .p_3()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .cursor_pointer()
            .justify_between()
            .items_center()
            .hover(|style| style)
            .on_mouse_down(gpui::MouseButton::Left, {
                let code = user_code.clone();
                move |_event: &MouseDownEvent, _window: &mut Window, cx: &mut App| {
                    cx.write_to_clipboard(ClipboardItem::new_string(code.clone()));
                }
            })
            .child(
                div()
                    .flex_1()
                    .child(Label::new(user_code.clone()).size(ui::LabelSize::Large)),
            )
            .child(
                div()
                    .flex_none()
                    .px_2()
                    .child(Label::new(copy_label).size(ui::LabelSize::Small)),
            )
    }

    fn is_code_in_clipboard(&self, user_code: &str, cx: &gpui::Context<Self>) -> bool {
        cx.read_from_clipboard()
            .and_then(|item| item.text())
            .map(|text| text == user_code)
            .unwrap_or(false)
    }

    fn render_sign_in_modal(
        &self,
        data: &PromptUserDeviceFlow,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let connect_button_config = self.get_connect_button_config();
        let verification_uri = data.verification_uri.clone();

        v_flex()
            .flex_1()
            .gap_4()
            .items_center()
            .child(self.render_header())
            .child(self.render_description())
            .child(self.render_device_code(data, cx))
            .child(self.render_instructions())
            .child(self.render_connect_button(connect_button_config, verification_uri, cx))
            .child(self.render_cancel_button(cx))
    }

    fn render_header(&self) -> impl IntoElement {
        Headline::new("Connect GitHub Copilot to Zed").size(HeadlineSize::Large)
    }

    fn render_description(&self) -> impl IntoElement {
        Label::new("An active GitHub Copilot subscription is required to use this feature.")
            .color(Color::Muted)
            .size(ui::LabelSize::Default)
    }

    fn render_instructions(&self) -> impl IntoElement {
        Label::new("Copy the code above and paste it on GitHub after clicking Connect.")
            .size(ui::LabelSize::Small)
            .color(Color::Muted)
    }

    fn get_connect_button_config(&self) -> (&'static str, bool) {
        match self.connection_state {
            ConnectionState::Initial => ("Connect to GitHub", false),
            ConnectionState::Connected => ("Connected!", true),
            ConnectionState::Failed(_) => ("Retry Connection", false),
        }
    }

    fn render_connect_button(
        &self,
        config: (&'static str, bool),
        verification_uri: String,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let (label, disabled) = config;

        Button::new("connect-button", label)
            .full_width()
            .style(ButtonStyle::Filled)
            .disabled(disabled)
            .when(disabled, |button| button.color(Color::Muted))
            .on_click(cx.listener(move |_this, _ev: &ClickEvent, _window, cx| {
                cx.open_url(&verification_uri);
            }))
    }

    fn render_cancel_button(&self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        Button::new("cancel-button", "Cancel")
            .full_width()
            .style(ButtonStyle::Subtle)
            .on_click(cx.listener(|_this, _ev: &ClickEvent, _window, cx| {
                cx.emit(gpui::DismissEvent);
            }))
    }

    fn render_success_modal(&self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        svg()
                            .size_6()
                            .path(IconName::ArrowCircle.path())
                            .text_color(cx.theme().status().success),
                    )
                    .child(
                        Headline::new("Copilot Connected!")
                            .size(HeadlineSize::Large)
                            .color(Color::Success),
                    ),
            )
            .child(
                Label::new("You can manage your Copilot settings from the status bar menu.")
                    .color(Color::Muted),
            )
            .child(
                Button::new("done-button", "Get Started")
                    .full_width()
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(|_this, _ev: &ClickEvent, _window, cx| {
                        cx.emit(gpui::DismissEvent)
                    })),
            )
    }

    fn render_unauthorized_modal(&self, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        svg()
                            .size_6()
                            .path("M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z")
                            .text_color(cx.theme().status().warning)
                    )
                    .child(
                        Headline::new("Subscription Required")
                            .size(HeadlineSize::Large)
                            .color(Color::Warning)
                    )
            )
            .child(
                Label::new("An active GitHub Copilot subscription is required. You can subscribe or renew your subscription on GitHub.")
                    .color(Color::Warning)
            )
            .child(
                Button::new("subscribe-button", "Subscribe on GitHub")
                    .full_width()
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(|_this, _ev: &ClickEvent, _window, cx| cx.open_url(COPILOT_SIGN_UP_URL)))
            )
            .child(
                Button::new("cancel-button", "Cancel")
                    .full_width()
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|_this, _ev: &ClickEvent, _window, cx| cx.emit(gpui::DismissEvent)))
            )
    }

    fn render_loading_state(
        &self,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let loading_icon = svg()
            .size_8()
            .path(IconName::ArrowCircle.path())
            .text_color(cx.theme().colors().text)
            .with_animation(
                "copilot_loading_spinner",
                Animation::new(LOADING_ANIMATION_DURATION).repeat(),
                |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
            );

        v_flex()
            .gap_4()
            .items_center()
            .child(loading_icon)
            .child(Label::new("Initializing Copilot...").color(Color::Muted))
    }
}

struct CopilotStatusToast;

impl Render for CopilotCodeVerification {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let content = match &self.status {
            Status::SigningIn { prompt: None } => {
                self.render_loading_state(window, cx).into_any_element()
            }
            Status::SigningIn {
                prompt: Some(prompt),
            } => self.render_sign_in_modal(prompt, cx).into_any_element(),
            Status::Unauthorized => self.render_unauthorized_modal(cx).into_any_element(),
            Status::Authorized => self.render_success_modal(cx).into_any_element(),
            _ => div()
                .child(Label::new("Unexpected state. Please try again."))
                .into_any_element(),
        };

        v_flex()
            .id("copilot-code-verification")
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .w(rems(24.0))
            .max_w_full()
            .items_center()
            .p_6()
            .gap_4()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_lg()
            .shadow_lg()
            .on_action(cx.listener(|_, _: &Cancel, _, cx| {
                cx.emit(gpui::DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _ev: &MouseDownEvent, window, _| {
                window.focus(&this.focus_handle);
            }))
            .child(
                Vector::new(VectorName::ZedXCopilot, rems(8.0), rems(4.0))
                    .color(Color::Custom(cx.theme().colors().icon)),
            )
            .child(content)
    }
}
