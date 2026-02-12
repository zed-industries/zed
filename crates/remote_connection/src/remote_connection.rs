use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use askpass::EncryptedPassword;
use auto_update::AutoUpdater;
use futures::{FutureExt as _, channel::oneshot, select};
use gpui::{
    AnyWindowHandle, App, AsyncApp, DismissEvent, Entity, EventEmitter, Focusable, FontFeatures,
    ParentElement as _, Render, SharedString, Task, TextStyleRefinement, WeakEntity,
};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use release_channel::ReleaseChannel;
use remote::{ConnectionIdentifier, RemoteClient, RemoteConnectionOptions, RemotePlatform};
use semver::Version;
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Color, CommonAnimationExt, Context, InteractiveElement, IntoElement, KeyBinding,
    LabelCommon, ListItem, Styled, Window, prelude::*,
};
use ui_input::{ERASED_EDITOR_FACTORY, ErasedEditor};
use workspace::{DismissDecision, ModalView};

pub struct RemoteConnectionPrompt {
    connection_string: SharedString,
    nickname: Option<SharedString>,
    is_wsl: bool,
    is_devcontainer: bool,
    status_message: Option<SharedString>,
    prompt: Option<(Entity<Markdown>, oneshot::Sender<EncryptedPassword>)>,
    cancellation: Option<oneshot::Sender<()>>,
    editor: Arc<dyn ErasedEditor>,
}

impl Drop for RemoteConnectionPrompt {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancellation.take() {
            log::debug!("cancelling remote connection");
            cancel.send(()).ok();
        }
    }
}

pub struct RemoteConnectionModal {
    pub prompt: Entity<RemoteConnectionPrompt>,
    paths: Vec<PathBuf>,
    finished: bool,
}

impl RemoteConnectionPrompt {
    pub fn new(
        connection_string: String,
        nickname: Option<String>,
        is_wsl: bool,
        is_devcontainer: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor_factory = ERASED_EDITOR_FACTORY
            .get()
            .expect("ErasedEditorFactory to be initialized");
        let editor = (editor_factory)(window, cx);

        Self {
            connection_string: connection_string.into(),
            nickname: nickname.map(|nickname| nickname.into()),
            is_wsl,
            is_devcontainer,
            editor,
            status_message: None,
            cancellation: None,
            prompt: None,
        }
    }

    pub fn set_cancellation_tx(&mut self, tx: oneshot::Sender<()>) {
        self.cancellation = Some(tx);
    }

    pub fn set_prompt(
        &mut self,
        prompt: String,
        tx: oneshot::Sender<EncryptedPassword>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_yes_no = prompt.contains("yes/no");
        self.editor.set_masked(!is_yes_no, window, cx);

        let markdown = cx.new(|cx| Markdown::new_text(prompt.into(), cx));
        self.prompt = Some((markdown, tx));
        self.status_message.take();
        window.focus(&self.editor.focus_handle(cx), cx);
        cx.notify();
    }

    pub fn set_status(&mut self, status: Option<String>, cx: &mut Context<Self>) {
        self.status_message = status.map(|s| s.into());
        cx.notify();
    }

    pub fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, tx)) = self.prompt.take() {
            self.status_message = Some("Connecting".into());

            let pw = self.editor.text(cx);
            if let Ok(secure) = EncryptedPassword::try_from(pw.as_ref()) {
                tx.send(secure).ok();
            }
            self.editor.clear(window, cx);
        }
    }
}

impl Render for RemoteConnectionPrompt {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = ThemeSettings::get_global(cx);

        let mut text_style = window.text_style();
        let refinement = TextStyleRefinement {
            font_family: Some(theme.buffer_font.family.clone()),
            font_features: Some(FontFeatures::disable_ligatures()),
            font_size: Some(theme.buffer_font_size(cx).into()),
            color: Some(cx.theme().colors().editor_foreground),
            background_color: Some(gpui::transparent_black()),
            ..Default::default()
        };

        text_style.refine(&refinement);
        let markdown_style = MarkdownStyle {
            base_text_style: text_style,
            selection_background_color: cx.theme().colors().element_selection_background,
            ..Default::default()
        };

        v_flex()
            .key_context("PasswordPrompt")
            .p_2()
            .size_full()
            .text_buffer(cx)
            .when_some(self.status_message.clone(), |el, status_message| {
                el.child(
                    h_flex()
                        .gap_2()
                        .child(
                            Icon::new(IconName::ArrowCircle)
                                .color(Color::Muted)
                                .with_rotate_animation(2),
                        )
                        .child(
                            div()
                                .text_ellipsis()
                                .overflow_x_hidden()
                                .child(format!("{}…", status_message)),
                        ),
                )
            })
            .when_some(self.prompt.as_ref(), |el, prompt| {
                el.child(
                    div()
                        .size_full()
                        .overflow_hidden()
                        .child(MarkdownElement::new(prompt.0.clone(), markdown_style))
                        .child(self.editor.render(window, cx)),
                )
                .when(window.capslock().on, |el| {
                    el.child(Label::new("⚠️ ⇪ is on"))
                })
            })
    }
}

impl RemoteConnectionModal {
    pub fn new(
        connection_options: &RemoteConnectionOptions,
        paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (connection_string, nickname, is_wsl, is_devcontainer) = match connection_options {
            RemoteConnectionOptions::Ssh(options) => (
                options.connection_string(),
                options.nickname.clone(),
                false,
                false,
            ),
            RemoteConnectionOptions::Wsl(options) => {
                (options.distro_name.clone(), None, true, false)
            }
            RemoteConnectionOptions::Docker(options) => (options.name.clone(), None, false, true),
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(options) => {
                (format!("mock-{}", options.id), None, false, false)
            }
        };
        Self {
            prompt: cx.new(|cx| {
                RemoteConnectionPrompt::new(
                    connection_string,
                    nickname,
                    is_wsl,
                    is_devcontainer,
                    window,
                    cx,
                )
            }),
            finished: false,
            paths,
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.prompt
            .update(cx, |prompt, cx| prompt.confirm(window, cx))
    }

    pub fn finished(&mut self, cx: &mut Context<Self>) {
        self.finished = true;
        cx.emit(DismissEvent);
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self
            .prompt
            .update(cx, |prompt, _cx| prompt.cancellation.take())
        {
            log::debug!("cancelling remote connection");
            tx.send(()).ok();
        }
        self.finished(cx);
    }
}

pub struct SshConnectionHeader {
    pub connection_string: SharedString,
    pub paths: Vec<PathBuf>,
    pub nickname: Option<SharedString>,
    pub is_wsl: bool,
    pub is_devcontainer: bool,
}

impl RenderOnce for SshConnectionHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut header_color = theme.colors().text;
        header_color.fade_out(0.96);

        let (main_label, meta_label) = if let Some(nickname) = self.nickname {
            (nickname, Some(format!("({})", self.connection_string)))
        } else {
            (self.connection_string, None)
        };

        let icon = if self.is_wsl {
            IconName::Linux
        } else if self.is_devcontainer {
            IconName::Box
        } else {
            IconName::Server
        };

        h_flex()
            .px(DynamicSpacing::Base12.rems(cx))
            .pt(DynamicSpacing::Base08.rems(cx))
            .pb(DynamicSpacing::Base04.rems(cx))
            .rounded_t_sm()
            .w_full()
            .gap_1p5()
            .child(Icon::new(icon).size(IconSize::Small))
            .child(
                h_flex()
                    .gap_1()
                    .overflow_x_hidden()
                    .child(
                        div()
                            .max_w_96()
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .child(Headline::new(main_label).size(HeadlineSize::XSmall)),
                    )
                    .children(
                        meta_label.map(|label| {
                            Label::new(label).color(Color::Muted).size(LabelSize::Small)
                        }),
                    )
                    .child(div().overflow_x_hidden().text_ellipsis().children(
                        self.paths.into_iter().map(|path| {
                            Label::new(path.to_string_lossy().into_owned())
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        }),
                    )),
            )
    }
}

impl Render for RemoteConnectionModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let nickname = self.prompt.read(cx).nickname.clone();
        let connection_string = self.prompt.read(cx).connection_string.clone();
        let is_wsl = self.prompt.read(cx).is_wsl;
        let is_devcontainer = self.prompt.read(cx).is_devcontainer;

        let theme = cx.theme().clone();
        let body_color = theme.colors().editor_background;

        v_flex()
            .elevation_3(cx)
            .w(rems(34.))
            .border_1()
            .border_color(theme.colors().border)
            .key_context("SshConnectionModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .child(
                SshConnectionHeader {
                    paths: self.paths.clone(),
                    connection_string,
                    nickname,
                    is_wsl,
                    is_devcontainer,
                }
                .render(window, cx),
            )
            .child(
                div()
                    .w_full()
                    .bg(body_color)
                    .border_y_1()
                    .border_color(theme.colors().border_variant)
                    .child(self.prompt.clone()),
            )
            .child(
                div().w_full().py_1().child(
                    ListItem::new("li-devcontainer-go-back")
                        .inset(true)
                        .spacing(ui::ListItemSpacing::Sparse)
                        .start_slot(Icon::new(IconName::Close).color(Color::Muted))
                        .child(Label::new("Cancel"))
                        .end_slot(
                            KeyBinding::for_action_in(&menu::Cancel, &self.focus_handle(cx), cx)
                                .size(rems_from_px(12.)),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.dismiss(&menu::Cancel, window, cx);
                        })),
                ),
            )
    }
}

impl Focusable for RemoteConnectionModal {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.prompt.read(cx).editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for RemoteConnectionModal {}

impl ModalView for RemoteConnectionModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(self.finished)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct RemoteClientDelegate {
    window: AnyWindowHandle,
    ui: WeakEntity<RemoteConnectionPrompt>,
    known_password: Option<EncryptedPassword>,
}

impl RemoteClientDelegate {
    pub fn new(
        window: AnyWindowHandle,
        ui: WeakEntity<RemoteConnectionPrompt>,
        known_password: Option<EncryptedPassword>,
    ) -> Self {
        Self {
            window,
            ui,
            known_password,
        }
    }
}

impl remote::RemoteClientDelegate for RemoteClientDelegate {
    fn ask_password(
        &self,
        prompt: String,
        tx: oneshot::Sender<EncryptedPassword>,
        cx: &mut AsyncApp,
    ) {
        let mut known_password = self.known_password.clone();
        if let Some(password) = known_password.take() {
            tx.send(password).ok();
        } else {
            self.window
                .update(cx, |_, window, cx| {
                    self.ui.update(cx, |modal, cx| {
                        modal.set_prompt(prompt, tx, window, cx);
                    })
                })
                .ok();
        }
    }

    fn set_status(&self, status: Option<&str>, cx: &mut AsyncApp) {
        self.update_status(status, cx)
    }

    fn download_server_binary_locally(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<Version>,
        cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<PathBuf>> {
        let this = self.clone();
        cx.spawn(async move |cx| {
            AutoUpdater::download_remote_server_release(
                release_channel,
                version.clone(),
                platform.os.as_str(),
                platform.arch.as_str(),
                move |status, cx| this.set_status(Some(status), cx),
                cx,
            )
            .await
            .with_context(|| {
                format!(
                    "Downloading remote server binary (version: {}, os: {}, arch: {})",
                    version
                        .as_ref()
                        .map(|v| format!("{}", v))
                        .unwrap_or("unknown".to_string()),
                    platform.os,
                    platform.arch,
                )
            })
        })
    }

    fn get_download_url(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<Version>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Option<String>>> {
        cx.spawn(async move |cx| {
            AutoUpdater::get_remote_server_release_url(
                release_channel,
                version,
                platform.os.as_str(),
                platform.arch.as_str(),
                cx,
            )
            .await
        })
    }
}

impl RemoteClientDelegate {
    fn update_status(&self, status: Option<&str>, cx: &mut AsyncApp) {
        cx.update(|cx| {
            self.ui
                .update(cx, |modal, cx| {
                    modal.set_status(status.map(|s| s.to_string()), cx);
                })
                .ok()
        });
    }
}

pub fn connect(
    unique_identifier: ConnectionIdentifier,
    connection_options: RemoteConnectionOptions,
    ui: Entity<RemoteConnectionPrompt>,
    window: &mut Window,
    cx: &mut App,
) -> Task<Result<Option<Entity<RemoteClient>>>> {
    let window = window.window_handle();
    let known_password = match &connection_options {
        RemoteConnectionOptions::Ssh(ssh_connection_options) => ssh_connection_options
            .password
            .as_deref()
            .and_then(|pw| pw.try_into().ok()),
        _ => None,
    };
    let (tx, mut rx) = oneshot::channel();
    ui.update(cx, |ui, _cx| ui.set_cancellation_tx(tx));

    let delegate = Arc::new(RemoteClientDelegate {
        window,
        ui: ui.downgrade(),
        known_password,
    });

    cx.spawn(async move |cx| {
        let connection = remote::connect(connection_options, delegate.clone(), cx);
        let connection = select! {
            _ = rx => return Ok(None),
            result = connection.fuse() => result,
        }?;

        cx.update(|cx| remote::RemoteClient::new(unique_identifier, connection, rx, delegate, cx))
            .await
    })
}

use anyhow::Context as _;
