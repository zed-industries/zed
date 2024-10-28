use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use auto_update::AutoUpdater;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    percentage, Animation, AnimationExt, AnyWindowHandle, AsyncAppContext, DismissEvent,
    EventEmitter, FocusableView, ParentElement as _, PromptLevel, Render, SemanticVersion,
    SharedString, Task, TextStyleRefinement, Transformation, View, WeakView,
};
use gpui::{AppContext, Model};

use language::CursorShape;
use markdown::{Markdown, MarkdownStyle};
use release_channel::{AppVersion, ReleaseChannel};
use remote::ssh_session::ServerBinary;
use remote::{SshConnectionOptions, SshPlatform, SshRemoteClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use theme::ThemeSettings;
use ui::{
    prelude::*, ActiveTheme, Color, Icon, IconName, IconSize, InteractiveElement, IntoElement,
    Label, LabelCommon, Styled, ViewContext, VisualContext, WindowContext,
};
use workspace::{AppState, ModalView, Workspace};

#[derive(Deserialize)]
pub struct SshSettings {
    pub ssh_connections: Option<Vec<SshConnection>>,
}

impl SshSettings {
    pub fn ssh_connections(&self) -> impl Iterator<Item = SshConnection> {
        self.ssh_connections.clone().into_iter().flatten()
    }

    pub fn connection_options_for(
        &self,
        host: String,
        port: Option<u16>,
        username: Option<String>,
    ) -> SshConnectionOptions {
        for conn in self.ssh_connections() {
            if conn.host == host && conn.username == username && conn.port == port {
                return SshConnectionOptions {
                    nickname: conn.nickname,
                    upload_binary_over_ssh: conn.upload_binary_over_ssh.unwrap_or_default(),
                    args: Some(conn.args),
                    host,
                    port,
                    username,
                    password: None,
                };
            }
        }
        SshConnectionOptions {
            host,
            port,
            username,
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SshConnection {
    pub host: SharedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub projects: Vec<SshProject>,
    /// Name to use for this server in UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    // By default Zed will download the binary to the host directly.
    // If this is set to true, Zed will download the binary to your local machine,
    // and then upload it over the SSH connection. Useful if your SSH server has
    // limited outbound internet access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_binary_over_ssh: Option<bool>,
}

impl From<SshConnection> for SshConnectionOptions {
    fn from(val: SshConnection) -> Self {
        SshConnectionOptions {
            host: val.host.into(),
            username: val.username,
            port: val.port,
            password: None,
            args: Some(val.args),
            nickname: val.nickname,
            upload_binary_over_ssh: val.upload_binary_over_ssh.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SshProject {
    pub paths: Vec<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct RemoteSettingsContent {
    pub ssh_connections: Option<Vec<SshConnection>>,
}

impl Settings for SshSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = RemoteSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

pub struct SshPrompt {
    connection_string: SharedString,
    nickname: Option<SharedString>,
    status_message: Option<SharedString>,
    prompt: Option<(View<Markdown>, oneshot::Sender<Result<String>>)>,
    cancellation: Option<oneshot::Sender<()>>,
    editor: View<Editor>,
}

impl Drop for SshPrompt {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancellation.take() {
            cancel.send(()).ok();
        }
    }
}

pub struct SshConnectionModal {
    pub(crate) prompt: View<SshPrompt>,
    paths: Vec<PathBuf>,
    finished: bool,
}

impl SshPrompt {
    pub(crate) fn new(
        connection_options: &SshConnectionOptions,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let connection_string = connection_options.connection_string().into();
        let nickname = connection_options.nickname.clone().map(|s| s.into());

        Self {
            connection_string,
            nickname,
            editor: cx.new_view(Editor::single_line),
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
        tx: oneshot::Sender<Result<String>>,
        cx: &mut ViewContext<Self>,
    ) {
        let theme = ThemeSettings::get_global(cx);

        let mut text_style = cx.text_style();
        let refinement = TextStyleRefinement {
            font_family: Some(theme.buffer_font.family.clone()),
            font_size: Some(theme.buffer_font_size.into()),
            color: Some(cx.theme().colors().editor_foreground),
            background_color: Some(gpui::transparent_black()),
            ..Default::default()
        };

        text_style.refine(&refinement);
        self.editor.update(cx, |editor, cx| {
            if prompt.contains("yes/no") {
                editor.set_masked(false, cx);
            } else {
                editor.set_masked(true, cx);
            }
            editor.set_text_style_refinement(refinement);
            editor.set_cursor_shape(CursorShape::Block, cx);
        });
        let markdown_style = MarkdownStyle {
            base_text_style: text_style,
            selection_background_color: cx.theme().players().local().selection,
            ..Default::default()
        };
        let markdown = cx.new_view(|cx| Markdown::new_text(prompt, markdown_style, None, cx, None));
        self.prompt = Some((markdown, tx));
        self.status_message.take();
        cx.focus_view(&self.editor);
        cx.notify();
    }

    pub fn set_status(&mut self, status: Option<String>, cx: &mut ViewContext<Self>) {
        self.status_message = status.map(|s| s.into());
        cx.notify();
    }

    pub fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((_, tx)) = self.prompt.take() {
            self.status_message = Some("Connecting".into());
            self.editor.update(cx, |editor, cx| {
                tx.send(Ok(editor.text(cx))).ok();
                editor.clear(cx);
            });
        }
    }
}

impl Render for SshPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let cx = cx.window_context();

        v_flex()
            .key_context("PasswordPrompt")
            .py_2()
            .px_3()
            .size_full()
            .text_buffer(cx)
            .when_some(self.status_message.clone(), |el, status_message| {
                el.child(
                    h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::Medium)
                                .with_animation(
                                    "arrow-circle",
                                    Animation::new(Duration::from_secs(2)).repeat(),
                                    |icon, delta| {
                                        icon.transform(Transformation::rotate(percentage(delta)))
                                    },
                                ),
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
                        .child(prompt.0.clone())
                        .child(self.editor.clone()),
                )
            })
    }
}

impl SshConnectionModal {
    pub(crate) fn new(
        connection_options: &SshConnectionOptions,
        paths: Vec<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            prompt: cx.new_view(|cx| SshPrompt::new(connection_options, cx)),
            finished: false,
            paths,
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        self.prompt.update(cx, |prompt, cx| prompt.confirm(cx))
    }

    pub fn finished(&mut self, cx: &mut ViewContext<Self>) {
        self.finished = true;
        cx.emit(DismissEvent);
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        if let Some(tx) = self
            .prompt
            .update(cx, |prompt, _cx| prompt.cancellation.take())
        {
            tx.send(()).ok();
        }
        self.finished(cx);
    }
}

pub(crate) struct SshConnectionHeader {
    pub(crate) connection_string: SharedString,
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) nickname: Option<SharedString>,
}

impl RenderOnce for SshConnectionHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();

        let mut header_color = theme.colors().text;
        header_color.fade_out(0.96);

        let (main_label, meta_label) = if let Some(nickname) = self.nickname {
            (nickname, Some(format!("({})", self.connection_string)))
        } else {
            (self.connection_string, None)
        };

        h_flex()
            .px(Spacing::XLarge.rems(cx))
            .pt(Spacing::Large.rems(cx))
            .pb(Spacing::Small.rems(cx))
            .rounded_t_md()
            .w_full()
            .gap_1p5()
            .child(Icon::new(IconName::Server).size(IconSize::XSmall))
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
                            Label::new(path.to_string_lossy().to_string())
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        }),
                    )),
            )
    }
}

impl Render for SshConnectionModal {
    fn render(&mut self, cx: &mut ui::ViewContext<Self>) -> impl ui::IntoElement {
        let nickname = self.prompt.read(cx).nickname.clone();
        let connection_string = self.prompt.read(cx).connection_string.clone();

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
                }
                .render(cx),
            )
            .child(
                div()
                    .w_full()
                    .rounded_b_lg()
                    .bg(body_color)
                    .border_t_1()
                    .border_color(theme.colors().border_variant)
                    .child(self.prompt.clone()),
            )
    }
}

impl FocusableView for SshConnectionModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.prompt.read(cx).editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for SshConnectionModal {}

impl ModalView for SshConnectionModal {
    fn on_before_dismiss(&mut self, _: &mut ViewContext<Self>) -> workspace::DismissDecision {
        return workspace::DismissDecision::Dismiss(self.finished);
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct SshClientDelegate {
    window: AnyWindowHandle,
    ui: WeakView<SshPrompt>,
    known_password: Option<String>,
}

impl remote::SshClientDelegate for SshClientDelegate {
    fn ask_password(
        &self,
        prompt: String,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<String>> {
        let (tx, rx) = oneshot::channel();
        let mut known_password = self.known_password.clone();
        if let Some(password) = known_password.take() {
            tx.send(Ok(password)).ok();
        } else {
            self.window
                .update(cx, |_, cx| {
                    self.ui.update(cx, |modal, cx| {
                        modal.set_prompt(prompt, tx, cx);
                    })
                })
                .ok();
        }
        rx
    }

    fn set_status(&self, status: Option<&str>, cx: &mut AsyncAppContext) {
        self.update_status(status, cx)
    }

    fn get_server_binary(
        &self,
        platform: SshPlatform,
        upload_binary_over_ssh: bool,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<(ServerBinary, SemanticVersion)>> {
        let (tx, rx) = oneshot::channel();
        let this = self.clone();
        cx.spawn(|mut cx| async move {
            tx.send(
                this.get_server_binary_impl(platform, upload_binary_over_ssh, &mut cx)
                    .await,
            )
            .ok();
        })
        .detach();
        rx
    }

    fn remote_server_binary_path(
        &self,
        platform: SshPlatform,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf> {
        let release_channel = cx.update(|cx| ReleaseChannel::global(cx))?;
        Ok(paths::remote_server_dir_relative().join(format!(
            "zed-remote-server-{}-{}-{}",
            release_channel.dev_name(),
            platform.os,
            platform.arch
        )))
    }
}

impl SshClientDelegate {
    fn update_status(&self, status: Option<&str>, cx: &mut AsyncAppContext) {
        self.window
            .update(cx, |_, cx| {
                self.ui.update(cx, |modal, cx| {
                    modal.set_status(status.map(|s| s.to_string()), cx);
                })
            })
            .ok();
    }

    async fn get_server_binary_impl(
        &self,
        platform: SshPlatform,
        upload_binary_via_ssh: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<(ServerBinary, SemanticVersion)> {
        let (version, release_channel) = cx.update(|cx| {
            let version = AppVersion::global(cx);
            let channel = ReleaseChannel::global(cx);

            (version, channel)
        })?;

        // In dev mode, build the remote server binary from source
        #[cfg(debug_assertions)]
        if release_channel == ReleaseChannel::Dev {
            let result = self.build_local(cx, platform, version).await?;
            // Fall through to a remote binary if we're not able to compile a local binary
            if let Some((path, version)) = result {
                return Ok((ServerBinary::LocalBinary(path), version));
            }
        }

        // For nightly channel, always get latest
        let current_version = if release_channel == ReleaseChannel::Nightly {
            None
        } else {
            Some(version)
        };

        self.update_status(
            Some(&format!("Checking remote server release {}", version)),
            cx,
        );

        if upload_binary_via_ssh {
            let binary_path = AutoUpdater::download_remote_server_release(
                platform.os,
                platform.arch,
                release_channel,
                current_version,
                cx,
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to download remote server binary (version: {}, os: {}, arch: {}): {}",
                    version,
                    platform.os,
                    platform.arch,
                    e
                )
            })?;

            Ok((ServerBinary::LocalBinary(binary_path), version))
        } else {
            let (request_url, request_body) = AutoUpdater::get_remote_server_release_url(
                    platform.os,
                    platform.arch,
                    release_channel,
                    current_version,
                    cx,
                )
                .await
                .map_err(|e| {
                    anyhow!(
                        "Failed to get remote server binary download url (version: {}, os: {}, arch: {}): {}",
                        version,
                        platform.os,
                        platform.arch,
                        e
                    )
                })?;

            Ok((
                ServerBinary::ReleaseUrl {
                    url: request_url,
                    body: request_body,
                },
                version,
            ))
        }
    }

    #[cfg(debug_assertions)]
    async fn build_local(
        &self,
        cx: &mut AsyncAppContext,
        platform: SshPlatform,
        version: gpui::SemanticVersion,
    ) -> Result<Option<(PathBuf, gpui::SemanticVersion)>> {
        use smol::process::{Command, Stdio};

        async fn run_cmd(command: &mut Command) -> Result<()> {
            let output = command
                .kill_on_drop(true)
                .stderr(Stdio::inherit())
                .output()
                .await?;
            if !output.status.success() {
                Err(anyhow!("Failed to run command: {:?}", command))?;
            }
            Ok(())
        }

        if platform.arch == std::env::consts::ARCH && platform.os == std::env::consts::OS {
            self.update_status(Some("Building remote server binary from source"), cx);
            log::info!("building remote server binary from source");
            run_cmd(Command::new("cargo").args([
                "build",
                "--package",
                "remote_server",
                "--features",
                "debug-embed",
                "--target-dir",
                "target/remote_server",
            ]))
            .await?;

            self.update_status(Some("Compressing binary"), cx);

            run_cmd(Command::new("gzip").args([
                "-9",
                "-f",
                "target/remote_server/debug/remote_server",
            ]))
            .await?;

            let path = std::env::current_dir()?.join("target/remote_server/debug/remote_server.gz");
            return Ok(Some((path, version)));
        } else if let Some(triple) = platform.triple() {
            smol::fs::create_dir_all("target/remote_server").await?;

            self.update_status(Some("Installing cross.rs for cross-compilation"), cx);
            log::info!("installing cross");
            run_cmd(Command::new("cargo").args([
                "install",
                "cross",
                "--git",
                "https://github.com/cross-rs/cross",
            ]))
            .await?;

            self.update_status(
                Some(&format!(
                    "Building remote server binary from source for {} with Docker",
                    &triple
                )),
                cx,
            );
            log::info!("building remote server binary from source for {}", &triple);
            run_cmd(
                Command::new("cross")
                    .args([
                        "build",
                        "--package",
                        "remote_server",
                        "--features",
                        "debug-embed",
                        "--target-dir",
                        "target/remote_server",
                        "--target",
                        &triple,
                    ])
                    .env(
                        "CROSS_CONTAINER_OPTS",
                        "--mount type=bind,src=./target,dst=/app/target",
                    ),
            )
            .await?;

            self.update_status(Some("Compressing binary"), cx);

            run_cmd(Command::new("gzip").args([
                "-9",
                "-f",
                &format!("target/remote_server/{}/debug/remote_server", triple),
            ]))
            .await?;

            let path = std::env::current_dir()?.join(format!(
                "target/remote_server/{}/debug/remote_server.gz",
                triple
            ));

            return Ok(Some((path, version)));
        } else {
            return Ok(None);
        }
    }
}

pub fn connect_over_ssh(
    unique_identifier: String,
    connection_options: SshConnectionOptions,
    ui: View<SshPrompt>,
    cx: &mut WindowContext,
) -> Task<Result<Option<Model<SshRemoteClient>>>> {
    let window = cx.window_handle();
    let known_password = connection_options.password.clone();
    let (tx, rx) = oneshot::channel();
    ui.update(cx, |ui, _cx| ui.set_cancellation_tx(tx));

    remote::SshRemoteClient::new(
        unique_identifier,
        connection_options,
        rx,
        Arc::new(SshClientDelegate {
            window,
            ui: ui.downgrade(),
            known_password,
        }),
        cx,
    )
}

pub async fn open_ssh_project(
    connection_options: SshConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncAppContext,
) -> Result<()> {
    let window = if let Some(window) = open_options.replace_window {
        window
    } else {
        let options = cx.update(|cx| (app_state.build_window_options)(None, cx))?;
        cx.open_window(options, |cx| {
            let project = project::Project::local(
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                None,
                cx,
            );
            cx.new_view(|cx| Workspace::new(None, project, app_state.clone(), cx))
        })?
    };

    loop {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let delegate = window.update(cx, {
            let connection_options = connection_options.clone();
            let paths = paths.clone();
            move |workspace, cx| {
                cx.activate_window();
                workspace.toggle_modal(cx, |cx| {
                    SshConnectionModal::new(&connection_options, paths, cx)
                });

                let ui = workspace
                    .active_modal::<SshConnectionModal>(cx)?
                    .read(cx)
                    .prompt
                    .clone();

                ui.update(cx, |ui, _cx| {
                    ui.set_cancellation_tx(cancel_tx);
                });

                Some(Arc::new(SshClientDelegate {
                    window: cx.window_handle(),
                    ui: ui.downgrade(),
                    known_password: connection_options.password.clone(),
                }))
            }
        })?;

        let Some(delegate) = delegate else { break };

        let did_open_ssh_project = cx
            .update(|cx| {
                workspace::open_ssh_project(
                    window,
                    connection_options.clone(),
                    cancel_rx,
                    delegate.clone(),
                    app_state.clone(),
                    paths.clone(),
                    cx,
                )
            })?
            .await;

        window
            .update(cx, |workspace, cx| {
                if let Some(ui) = workspace.active_modal::<SshConnectionModal>(cx) {
                    ui.update(cx, |modal, cx| modal.finished(cx))
                }
            })
            .ok();

        if let Err(e) = did_open_ssh_project {
            log::error!("Failed to open project: {:?}", e);
            let response = window
                .update(cx, |_, cx| {
                    cx.prompt(
                        PromptLevel::Critical,
                        "Failed to connect over SSH",
                        Some(&e.to_string()),
                        &["Retry", "Ok"],
                    )
                })?
                .await;

            if response == Ok(0) {
                continue;
            }
        }

        break;
    }

    // Already showed the error to the user
    Ok(())
}
