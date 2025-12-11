use crate::{Copilot, Status, request::PromptUserDeviceFlow};
use gpui::{
    App, AsyncApp, ClipboardItem, Context, DismissEvent, Element, Entity, EventEmitter,
    FocusHandle, Focusable, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement, Point, Render, Styled, Subscription, Window, WindowBounds, WindowOptions, div,
    svg,
};
use ui::{Button, CommonAnimationExt, ConfiguredApiCard, Label, Vector, VectorName, prelude::*};
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::{ModalView, Toast, Workspace};

const COPILOT_SIGN_UP_URL: &str = "https://github.com/features/copilot";

struct CopilotStatusToast;

pub fn initiate_sign_in(window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };

    let is_reinstall = false;
    initiate_sign_in_within_workspace(copilot, is_reinstall, window, cx)
}

pub fn initiate_sign_out(window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        // todo! error?
        return;
    };
    sign_out_within_workspace(copilot, window, cx);
}

pub fn reinstall_and_sign_in(window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };
    reinstall_and_sign_in_within_workspace(copilot, window, cx);
}

pub fn reinstall_and_sign_in_within_workspace(
    copilot: Entity<Copilot>,
    window: &mut Window,
    cx: &mut App,
) {
    let _ = copilot.update(cx, |copilot, cx| copilot.reinstall(cx));
    let is_reinstall = true;
    initiate_sign_in_within_workspace(copilot, is_reinstall, window, cx);
}

fn open_copilot_code_verification_window(
    // todo! just take window param
    current_window_center: Point<Pixels>,
    copilot: &Entity<Copilot>,
    cx: &mut App,
) {
    // todo! actually center
    let window_size = px(400.);
    let window_bounds = WindowBounds::Windowed(gpui::bounds(
        current_window_center,
        gpui::size(window_size, window_size),
    ));
    cx.open_window(
        WindowOptions {
            kind: gpui::WindowKind::PopUp,
            window_bounds: Some(window_bounds),
            is_resizable: false,
            is_movable: true,
            ..Default::default()
        },
        |_, cx| cx.new(|cx| CopilotCodeVerification::new(&copilot, cx)),
    )
    .expect("todo!");
}

fn copilot_toast(message: Option<&'static str>, window: &Window, cx: &mut App) {
    const NOTIFICATION_ID: NotificationId = NotificationId::unique::<CopilotStatusToast>();

    let Some(workspace) = window.root::<Workspace>().flatten() else {
        return;
    };

    workspace.update(cx, |workspace, cx| match message {
        Some(message) => workspace.show_toast(Toast::new(NOTIFICATION_ID, message), cx),
        None => workspace.dismiss_toast(&NOTIFICATION_ID, cx),
    });
}

pub fn initiate_sign_in_within_workspace(
    copilot: Entity<Copilot>,
    is_reinstall: bool,
    window: &mut Window,
    cx: &mut App,
) {
    if matches!(copilot.read(cx).status(), Status::Disabled) {
        copilot.update(cx, |copilot, cx| copilot.start_copilot(false, true, cx));
    }
    match copilot.read(cx).status() {
        Status::Starting { task } => {
            copilot_toast(
                Some(if is_reinstall {
                    "Copilot is reinstalling…"
                } else {
                    "Copilot is starting…"
                }),
                window,
                cx,
            );

            window
                .spawn(cx, async move |cx| {
                    task.await;
                    cx.update(|window, cx| {
                        let Some(copilot) = Copilot::global(cx) else {
                            return;
                        };
                        match copilot.read(cx).status() {
                            Status::Authorized => {
                                copilot_toast(Some("Copilot has started."), window, cx)
                            }
                            _ => {
                                copilot_toast(None, window, cx);
                                copilot
                                    .update(cx, |copilot, cx| copilot.sign_in(cx))
                                    .detach_and_log_err(cx);
                                open_copilot_code_verification_window(
                                    window.bounds().center(),
                                    &copilot,
                                    cx,
                                );
                            }
                        }
                    })
                    .log_err();
                })
                .detach();
        }
        _ => {
            copilot
                .update(cx, |copilot, cx| copilot.sign_in(cx))
                .detach();
            open_copilot_code_verification_window(window.bounds().center(), &copilot, cx);
        }
    }
}

pub fn sign_out_within_workspace(copilot: Entity<Copilot>, window: &mut Window, cx: &mut App) {
    copilot_toast(Some("Signing out of Copilot…"), window, cx);

    let sign_out_task = copilot.update(cx, |copilot, cx| copilot.sign_out(cx));
    window
        .spawn(cx, async move |cx| match sign_out_task.await {
            Ok(()) => {
                cx.update(|window, cx| copilot_toast(Some("Signed out of Copilot"), window, cx))
            }
            Err(err) => cx.update(|window, cx| {
                if let Some(workspace) = window.root::<Workspace>().flatten() {
                    workspace.update(cx, |workspace, cx| {
                        workspace.show_error(&err, cx);
                    })
                } else {
                    log::error!("{:?}", err);
                }
            }),
        })
        .detach();
}

pub struct CopilotCodeVerification {
    status: Status,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    copilot: Entity<Copilot>,
    _subscription: Subscription,
}

impl Focusable for CopilotCodeVerification {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for CopilotCodeVerification {}
impl ModalView for CopilotCodeVerification {
    fn on_before_dismiss(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        self.copilot.update(cx, |copilot, cx| {
            if matches!(copilot.status(), Status::SigningIn { .. }) {
                copilot.sign_out(cx).detach_and_log_err(cx);
            }
        });
        workspace::DismissDecision::Dismiss(true)
    }
}

impl CopilotCodeVerification {
    pub fn new(copilot: &Entity<Copilot>, cx: &mut Context<Self>) -> Self {
        let status = dbg!(copilot.read(cx).status());
        Self {
            status,
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            copilot: copilot.clone(),
            _subscription: cx.observe(copilot, |this, copilot, cx| {
                let status = copilot.read(cx).status();
                match status {
                    Status::Authorized | Status::Unauthorized | Status::SigningIn { .. } => {
                        this.set_status(dbg!(status), cx)
                    }
                    status @ _ => _ = dbg!(status, cx.emit(DismissEvent)),
                }
            }),
        }
    }

    pub fn set_status(&mut self, status: Status, cx: &mut Context<Self>) {
        self.status = status;
        cx.notify();
    }

    fn render_device_code(data: &PromptUserDeviceFlow, cx: &mut Context<Self>) -> impl IntoElement {
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text().as_ref() == Some(&data.user_code))
            .unwrap_or(false);

        h_flex()
            .cursor_pointer()
            .w_full()
            .p_1p5()
            .border_1()
            .border_muted(cx)
            .rounded_sm()
            .justify_between()
            .child(Label::new(data.user_code.clone()))
            .child(Label::new(if copied { "Copied!" } else { "Copy" }))
            .on_mouse_down(MouseButton::Left, {
                let user_code = data.user_code.clone();
                move |_, window, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(user_code.clone()));
                    window.refresh();
                }
            })
    }

    fn render_prompting_modal(
        connect_clicked: bool,
        data: &PromptUserDeviceFlow,

        cx: &mut Context<Self>,
    ) -> impl Element {
        let connect_button_label = if connect_clicked {
            "Waiting for connection…"
        } else {
            "Connect to GitHub"
        };
        v_flex()
            .flex_1()
            .gap_2()
            .items_center()
            .text_center()
            .child(Headline::new("Use GitHub Copilot in Zed").size(HeadlineSize::Large))
            .child(
                Label::new("Using Copilot requires an active subscription on GitHub.")
                    .color(Color::Muted),
            )
            .child(Self::render_device_code(data, cx))
            .child(
                Label::new("Paste this code into GitHub after clicking the button below.")
                    .size(ui::LabelSize::Small),
            )
            .child(
                Button::new("connect-button", connect_button_label)
                    .full_width()
                    .style(ButtonStyle::Filled)
                    .on_click({
                        let verification_uri = data.verification_uri.clone();
                        cx.listener(move |this, _, _window, cx| {
                            cx.open_url(&verification_uri);
                            this.connect_clicked = true;
                        })
                    }),
            )
            .child(
                Button::new("copilot-enable-cancel-button", "Cancel")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| {
                        cx.emit(DismissEvent);
                    })),
            )
    }

    fn render_enabled_modal(cx: &mut Context<Self>) -> impl Element {
        v_flex()
            .gap_2()
            .child(Headline::new("Copilot Enabled!").size(HeadlineSize::Large))
            .child(Label::new(
                "You can update your settings or sign out from the Copilot menu in the status bar.",
            ))
            .child(
                Button::new("copilot-enabled-done-button", "Done")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_unauthorized_modal(cx: &mut Context<Self>) -> impl Element {
        v_flex()
            .child(Headline::new("You must have an active GitHub Copilot subscription.").size(HeadlineSize::Large))
            .child(Label::new(
                "You can enable Copilot by connecting your existing license once you have subscribed or renewed your subscription.",
            ).color(Color::Warning))
            .child(
                Button::new("copilot-subscribe-button", "Subscribe on GitHub")
                    .full_width()
                    .on_click(|_, _, cx| cx.open_url(COPILOT_SIGN_UP_URL)),
            )
            .child(
                Button::new("copilot-subscribe-cancel-button", "Cancel")
                    .full_width()
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }
}

impl Render for CopilotCodeVerification {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let prompt = match &self.status {
            Status::SigningIn { prompt: None } => Icon::new(IconName::ArrowCircle)
                .color(Color::Muted)
                .with_rotate_animation(2)
                .into_any_element(),
            Status::SigningIn {
                prompt: Some(prompt),
            } => Self::render_prompting_modal(self.connect_clicked, prompt, cx).into_any_element(),
            Status::Unauthorized => {
                self.connect_clicked = false;
                Self::render_unauthorized_modal(cx).into_any_element()
            }
            Status::Authorized => {
                self.connect_clicked = false;
                Self::render_enabled_modal(cx).into_any_element()
            }
            _ => div().into_any_element(),
        };

        v_flex()
            .size_full()
            .id("copilot code verification")
            .track_focus(&self.focus_handle(cx))
            .elevation_3(cx)
            // .w_96()
            .items_center()
            .p_4()
            .gap_2()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _| {
                window.focus(&this.focus_handle);
            }))
            .child(
                Vector::new(VectorName::ZedXCopilot, rems(8.), rems(4.))
                    .color(Color::Custom(cx.theme().colors().icon)),
            )
            .child(prompt)
    }
}

pub struct ConfigurationView {
    copilot_status: Option<Status>,
    is_authenticated: fn(cx: &App) -> bool,
    edit_prediction: bool,
    _subscription: Option<Subscription>,
}

pub enum ConfigurationMode {
    Chat,
    EditPrediction,
}

impl ConfigurationView {
    pub fn new(
        is_authenticated: fn(cx: &App) -> bool,
        mode: ConfigurationMode,
        cx: &mut Context<Self>,
    ) -> Self {
        let copilot = Copilot::global(cx);

        Self {
            copilot_status: copilot.as_ref().map(|copilot| copilot.read(cx).status()),
            is_authenticated,
            edit_prediction: matches!(mode, ConfigurationMode::EditPrediction),
            _subscription: copilot.as_ref().map(|copilot| {
                cx.observe(copilot, |this, model, cx| {
                    this.copilot_status = Some(model.read(cx).status());
                    cx.notify();
                })
            }),
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.is_authenticated;
        if is_authenticated(cx) {
            ConfiguredApiCard::new("Authorized")
                .button_label("Sign Out")
                .on_click(|_, window, cx| {
                    initiate_sign_out(window, cx);
                })
                .into_any_element()
        } else {
            let (start_copy, error_label) = if self.edit_prediction {
                (
                    "To use GitHub Copilot for edit predictions, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot subscription.",
                    "Copilot requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different edit prediction provider.",
                )
            } else {
                (
                    "To use Zed's agent with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.",
                    "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different LLM provider.",
                )
            };

            let loading_icon = Icon::new(IconName::ArrowCircle).with_rotate_animation(4);

            match &self.copilot_status {
                Some(status) => match status {
                    Status::Starting { task: _ } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Starting Copilot…"))
                        .into_any_element(),
                    Status::SigningIn { prompt: _ }
                    | Status::SignedOut {
                        awaiting_signing_in: true,
                    } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Signing into Copilot…"))
                        .into_any_element(),
                    Status::Error(_) => {
                        v_flex()
                            .gap_6()
                            .child(Label::new("Copilot had issues starting. Please try restarting it. If the issue persists, try reinstalling Copilot."))
                            .child(svg().size_8().path(IconName::CopilotError.path()))
                            .into_any_element()
                    }
                    _ => {
                        v_flex()
                            .gap_2()
                            .child(Label::new(start_copy).when(self.edit_prediction, |this| this.color(Color::Muted)))
                            .child(
                                Button::new("sign_in", "Sign in to use GitHub Copilot")
                                    .full_width()
                                    .style(ButtonStyle::Outlined)
                                    .icon_color(Color::Muted)
                                    .icon(IconName::Github)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
                                    .on_click(|_, window, cx| {
                                        initiate_sign_in(window, cx)
                                    }),
                            )
                            .into_any_element()
                    }
                },
                None => v_flex()
                    .gap_6()
                    .child(Label::new(error_label))
                    .into_any_element(),
            }
        }
    }
}
