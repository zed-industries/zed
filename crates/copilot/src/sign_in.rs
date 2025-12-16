use crate::{Copilot, Status, request::PromptUserDeviceFlow};
use anyhow::Context as _;
use gpui::{
    App, ClipboardItem, Context, DismissEvent, Element, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, MouseDownEvent, ParentElement, Render, Styled,
    Subscription, Window, WindowBounds, WindowOptions, div, point,
};
use ui::{ButtonLike, CommonAnimationExt, ConfiguredApiCard, Vector, VectorName, prelude::*};
use url::Url;
use util::ResultExt as _;
use workspace::{Toast, Workspace, notifications::NotificationId};

const COPILOT_SIGN_UP_URL: &str = "https://github.com/features/copilot";
const ERROR_LABEL: &str =
    "Copilot had issues starting. You can try reinstalling it and signing in again.";

struct CopilotStatusToast;

pub fn initiate_sign_in(window: &mut Window, cx: &mut App) {
    let is_reinstall = false;
    initiate_sign_in_impl(is_reinstall, window, cx)
}

pub fn initiate_sign_out(window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };

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

pub fn reinstall_and_sign_in(window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };
    let _ = copilot.update(cx, |copilot, cx| copilot.reinstall(cx));
    let is_reinstall = true;
    initiate_sign_in_impl(is_reinstall, window, cx);
}

fn open_copilot_code_verification_window(copilot: &Entity<Copilot>, window: &Window, cx: &mut App) {
    let current_window_center = window.bounds().center();
    let height = px(450.);
    let width = px(350.);
    let window_bounds = WindowBounds::Windowed(gpui::bounds(
        current_window_center - point(height / 2.0, width / 2.0),
        gpui::size(height, width),
    ));
    cx.open_window(
        WindowOptions {
            kind: gpui::WindowKind::PopUp,
            window_bounds: Some(window_bounds),
            is_resizable: false,
            is_movable: true,
            titlebar: Some(gpui::TitlebarOptions {
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| CopilotCodeVerification::new(&copilot, window, cx)),
    )
    .context("Failed to open Copilot code verification window")
    .log_err();
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

pub fn initiate_sign_in_impl(is_reinstall: bool, window: &mut Window, cx: &mut App) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };
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
                                open_copilot_code_verification_window(&copilot, window, cx);
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
            open_copilot_code_verification_window(&copilot, window, cx);
        }
    }
}

pub struct CopilotCodeVerification {
    status: Status,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    copilot: Entity<Copilot>,
    _subscription: Subscription,
    sign_up_url: Option<String>,
}

impl Focusable for CopilotCodeVerification {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for CopilotCodeVerification {}

impl CopilotCodeVerification {
    pub fn new(copilot: &Entity<Copilot>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        window.on_window_should_close(cx, |window, cx| {
            if let Some(this) = window.root::<CopilotCodeVerification>().flatten() {
                this.update(cx, |this, cx| {
                    this.before_dismiss(cx);
                });
            }
            true
        });
        cx.subscribe_in(
            &cx.entity(),
            window,
            |this, _, _: &DismissEvent, window, cx| {
                window.remove_window();
                this.before_dismiss(cx);
            },
        )
        .detach();

        let status = copilot.read(cx).status();
        // Determine sign-up URL based on verification_uri domain if available
        let sign_up_url = if let Status::SigningIn {
            prompt: Some(ref prompt),
        } = status
        {
            // Extract domain from verification_uri to construct sign-up URL
            Self::get_sign_up_url_from_verification(&prompt.verification_uri)
        } else {
            None
        };
        Self {
            status,
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            copilot: copilot.clone(),
            sign_up_url,
            _subscription: cx.observe(copilot, |this, copilot, cx| {
                let status = copilot.read(cx).status();
                match status {
                    Status::Authorized | Status::Unauthorized | Status::SigningIn { .. } => {
                        this.set_status(status, cx)
                    }
                    _ => cx.emit(DismissEvent),
                }
            }),
        }
    }

    pub fn set_status(&mut self, status: Status, cx: &mut Context<Self>) {
        // Update sign-up URL if we have a new verification URI
        if let Status::SigningIn {
            prompt: Some(ref prompt),
        } = status
        {
            self.sign_up_url = Self::get_sign_up_url_from_verification(&prompt.verification_uri);
        }
        self.status = status;
        cx.notify();
    }

    fn get_sign_up_url_from_verification(verification_uri: &str) -> Option<String> {
        // Extract domain from verification URI using url crate
        if let Ok(url) = Url::parse(verification_uri)
            && let Some(host) = url.host_str()
            && !host.contains("github.com")
        {
            // For GHE, construct URL from domain
            Some(format!("https://{}/features/copilot", host))
        } else {
            None
        }
    }

    fn render_device_code(data: &PromptUserDeviceFlow, cx: &mut Context<Self>) -> impl IntoElement {
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text().as_ref() == Some(&data.user_code))
            .unwrap_or(false);

        ButtonLike::new("copy-button")
            .full_width()
            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .p_1()
                    .justify_between()
                    .child(Label::new(data.user_code.clone()))
                    .child(Label::new(if copied { "Copied!" } else { "Copy" })),
            )
            .on_click({
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
            .gap_2p5()
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
                    .color(Color::Muted),
            )
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(
                        Button::new("connect-button", connect_button_label)
                            .full_width()
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Medium)
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
                            .size(ButtonSize::Medium)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    ),
            )
    }

    fn render_enabled_modal(cx: &mut Context<Self>) -> impl Element {
        v_flex()
            .gap_2()
            .text_center()
            .justify_center()
            .child(Headline::new("Copilot Enabled!").size(HeadlineSize::Large))
            .child(Label::new("You're all set to use GitHub Copilot.").color(Color::Muted))
            .child(
                Button::new("copilot-enabled-done-button", "Done")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_unauthorized_modal(&self, cx: &mut Context<Self>) -> impl Element {
        let sign_up_url = self
            .sign_up_url
            .as_deref()
            .unwrap_or(COPILOT_SIGN_UP_URL)
            .to_owned();
        let description = "Enable Copilot by connecting your existing license once you have subscribed or renewed your subscription.";

        v_flex()
            .gap_2()
            .text_center()
            .justify_center()
            .child(
                Headline::new("You must have an active GitHub Copilot subscription.")
                    .size(HeadlineSize::Large),
            )
            .child(Label::new(description).color(Color::Warning))
            .child(
                Button::new("copilot-subscribe-button", "Subscribe on GitHub")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .on_click(move |_, _, cx| cx.open_url(&sign_up_url)),
            )
            .child(
                Button::new("copilot-subscribe-cancel-button", "Cancel")
                    .full_width()
                    .size(ButtonSize::Medium)
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_error_modal(_cx: &mut Context<Self>) -> impl Element {
        v_flex()
            .gap_2()
            .text_center()
            .justify_center()
            .child(Headline::new("An Error Happened").size(HeadlineSize::Large))
            .child(Label::new(ERROR_LABEL).color(Color::Muted))
            .child(
                Button::new("copilot-subscribe-button", "Reinstall Copilot and Sign In")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .icon(IconName::Download)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .on_click(|_, window, cx| reinstall_and_sign_in(window, cx)),
            )
    }

    fn before_dismiss(
        &mut self,
        cx: &mut Context<'_, CopilotCodeVerification>,
    ) -> workspace::DismissDecision {
        self.copilot.update(cx, |copilot, cx| {
            if matches!(copilot.status(), Status::SigningIn { .. }) {
                copilot.sign_out(cx).detach_and_log_err(cx);
            }
        });
        workspace::DismissDecision::Dismiss(true)
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
                self.render_unauthorized_modal(cx).into_any_element()
            }
            Status::Authorized => {
                self.connect_clicked = false;
                Self::render_enabled_modal(cx).into_any_element()
            }
            Status::Error(..) => Self::render_error_modal(cx).into_any_element(),
            _ => div().into_any_element(),
        };

        v_flex()
            .id("copilot_code_verification")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .px_4()
            .py_8()
            .gap_2()
            .items_center()
            .justify_center()
            .elevation_3(cx)
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

impl ConfigurationView {
    fn is_starting(&self) -> bool {
        matches!(&self.copilot_status, Some(Status::Starting { .. }))
    }

    fn is_signing_in(&self) -> bool {
        matches!(
            &self.copilot_status,
            Some(Status::SigningIn { .. })
                | Some(Status::SignedOut {
                    awaiting_signing_in: true
                })
        )
    }

    fn is_error(&self) -> bool {
        matches!(&self.copilot_status, Some(Status::Error(_)))
    }

    fn has_no_status(&self) -> bool {
        self.copilot_status.is_none()
    }

    fn loading_message(&self) -> Option<SharedString> {
        if self.is_starting() {
            Some("Starting Copilot…".into())
        } else if self.is_signing_in() {
            Some("Signing into Copilot…".into())
        } else {
            None
        }
    }

    fn render_loading_button(
        &self,
        label: impl Into<SharedString>,
        edit_prediction: bool,
    ) -> impl IntoElement {
        ButtonLike::new("loading_button")
            .disabled(true)
            .style(ButtonStyle::Outlined)
            .when(edit_prediction, |this| this.size(ButtonSize::Medium))
            .child(
                h_flex()
                    .w_full()
                    .gap_1()
                    .justify_center()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                            .with_rotate_animation(4),
                    )
                    .child(Label::new(label)),
            )
    }

    fn render_sign_in_button(&self, edit_prediction: bool) -> impl IntoElement {
        let label = if edit_prediction {
            "Sign in to GitHub"
        } else {
            "Sign in to use GitHub Copilot"
        };

        Button::new("sign_in", label)
            .map(|this| {
                if edit_prediction {
                    this.size(ButtonSize::Medium)
                } else {
                    this.full_width()
                }
            })
            .style(ButtonStyle::Outlined)
            .icon(IconName::Github)
            .icon_color(Color::Muted)
            .icon_position(IconPosition::Start)
            .icon_size(IconSize::Small)
            .on_click(|_, window, cx| initiate_sign_in(window, cx))
    }

    fn render_reinstall_button(&self, edit_prediction: bool) -> impl IntoElement {
        let label = if edit_prediction {
            "Reinstall and Sign in"
        } else {
            "Reinstall Copilot and Sign in"
        };

        Button::new("reinstall_and_sign_in", label)
            .map(|this| {
                if edit_prediction {
                    this.size(ButtonSize::Medium)
                } else {
                    this.full_width()
                }
            })
            .style(ButtonStyle::Outlined)
            .icon(IconName::Download)
            .icon_color(Color::Muted)
            .icon_position(IconPosition::Start)
            .icon_size(IconSize::Small)
            .on_click(|_, window, cx| reinstall_and_sign_in(window, cx))
    }

    fn render_for_edit_prediction(&self) -> impl IntoElement {
        let container = |description: SharedString, action: AnyElement| {
            h_flex()
                .pt_2p5()
                .w_full()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .max_w_1_2()
                        .child(Label::new("Authenticate To Use"))
                        .child(
                            Label::new(description)
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
                .child(action)
        };

        let start_label = "To use Copilot for edit predictions, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot subscription.".into();
        let no_status_label = "Copilot requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different edit predictions provider.".into();

        if let Some(msg) = self.loading_message() {
            container(
                start_label,
                self.render_loading_button(msg, true).into_any_element(),
            )
            .into_any_element()
        } else if self.is_error() {
            container(
                ERROR_LABEL.into(),
                self.render_reinstall_button(true).into_any_element(),
            )
            .into_any_element()
        } else if self.has_no_status() {
            container(
                no_status_label,
                self.render_sign_in_button(true).into_any_element(),
            )
            .into_any_element()
        } else {
            container(
                start_label,
                self.render_sign_in_button(true).into_any_element(),
            )
            .into_any_element()
        }
    }

    fn render_for_chat(&self) -> impl IntoElement {
        let start_label = "To use Zed's agent with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.";
        let no_status_label = "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different LLM provider.";

        if let Some(msg) = self.loading_message() {
            v_flex()
                .gap_2()
                .child(Label::new(start_label))
                .child(self.render_loading_button(msg, false))
                .into_any_element()
        } else if self.is_error() {
            v_flex()
                .gap_2()
                .child(Label::new(ERROR_LABEL))
                .child(self.render_reinstall_button(false))
                .into_any_element()
        } else if self.has_no_status() {
            v_flex()
                .gap_2()
                .child(Label::new(no_status_label))
                .child(self.render_sign_in_button(false))
                .into_any_element()
        } else {
            v_flex()
                .gap_2()
                .child(Label::new(start_label))
                .child(self.render_sign_in_button(false))
                .into_any_element()
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.is_authenticated;

        if is_authenticated(cx) {
            return ConfiguredApiCard::new("Authorized")
                .button_label("Sign Out")
                .on_click(|_, window, cx| {
                    initiate_sign_out(window, cx);
                })
                .into_any_element();
        }

        if self.edit_prediction {
            self.render_for_edit_prediction().into_any_element()
        } else {
            self.render_for_chat().into_any_element()
        }
    }
}
