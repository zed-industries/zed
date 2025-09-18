use std::collections::BTreeSet;
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use auto_update::AutoUpdater;
use editor::Editor;
use extension_host::ExtensionStore;
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, App, AsyncApp, DismissEvent, Entity, EventEmitter, Focusable, FontFeatures,
    ParentElement as _, PromptLevel, Render, SemanticVersion, SharedString, Task,
    TextStyleRefinement, WeakEntity,
};

use language::CursorShape;
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use release_channel::ReleaseChannel;
use remote::{
    ConnectionIdentifier, RemoteClient, RemoteConnectionOptions, RemotePlatform,
    SshConnectionOptions, SshPortForwardOption, WslConnectionOptions,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Color, CommonAnimationExt, Context, Icon, IconName, IconSize, InteractiveElement,
    IntoElement, Label, LabelCommon, Styled, Window, prelude::*,
};
use util::serde::default_true;
use workspace::{AppState, ModalView, Workspace};

#[derive(Deserialize)]
pub struct SshSettings {
    pub ssh_connections: Option<Vec<SshConnection>>,
    pub wsl_connections: Option<Vec<WslConnection>>,
    /// Whether to read ~/.ssh/config for ssh connection sources.
    #[serde(default = "default_true")]
    pub read_ssh_config: bool,
}

impl SshSettings {
    pub fn ssh_connections(&self) -> impl Iterator<Item = SshConnection> + use<> {
        self.ssh_connections.clone().into_iter().flatten()
    }

    pub fn wsl_connections(&self) -> impl Iterator<Item = WslConnection> + use<> {
        self.wsl_connections.clone().into_iter().flatten()
    }

    pub fn fill_connection_options_from_settings(&self, options: &mut SshConnectionOptions) {
        for conn in self.ssh_connections() {
            if conn.host == options.host
                && conn.username == options.username
                && conn.port == options.port
            {
                options.nickname = conn.nickname;
                options.upload_binary_over_ssh = conn.upload_binary_over_ssh.unwrap_or_default();
                options.args = Some(conn.args);
                options.port_forwards = conn.port_forwards;
                break;
            }
        }
    }

    pub fn connection_options_for(
        &self,
        host: String,
        port: Option<u16>,
        username: Option<String>,
    ) -> SshConnectionOptions {
        let mut options = SshConnectionOptions {
            host,
            port,
            username,
            ..Default::default()
        };
        self.fill_connection_options_from_settings(&mut options);
        options
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
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
    pub projects: BTreeSet<SshProject>,
    /// Name to use for this server in UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    // By default Zed will download the binary to the host directly.
    // If this is set to true, Zed will download the binary to your local machine,
    // and then upload it over the SSH connection. Useful if your SSH server has
    // limited outbound internet access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_binary_over_ssh: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_forwards: Option<Vec<SshPortForwardOption>>,
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
            port_forwards: val.port_forwards,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct WslConnection {
    pub distro_name: SharedString,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub projects: BTreeSet<SshProject>,
}

impl From<WslConnection> for WslConnectionOptions {
    fn from(val: WslConnection) -> Self {
        WslConnectionOptions {
            distro_name: val.distro_name.into(),
            user: val.user,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub enum Connection {
    Ssh(SshConnection),
    Wsl(WslConnection),
}

impl From<Connection> for RemoteConnectionOptions {
    fn from(val: Connection) -> Self {
        match val {
            Connection::Ssh(conn) => RemoteConnectionOptions::Ssh(conn.into()),
            Connection::Wsl(conn) => RemoteConnectionOptions::Wsl(conn.into()),
        }
    }
}

impl From<SshConnection> for Connection {
    fn from(val: SshConnection) -> Self {
        Connection::Ssh(val)
    }
}

impl From<WslConnection> for Connection {
    fn from(val: WslConnection) -> Self {
        Connection::Wsl(val)
    }
}

#[derive(Clone, Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema)]
pub struct SshProject {
    pub paths: Vec<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(None)]
pub struct RemoteSettingsContent {
    pub ssh_connections: Option<Vec<SshConnection>>,
    pub wsl_connections: Option<Vec<WslConnection>>,
    pub read_ssh_config: Option<bool>,
}

impl Settings for SshSettings {
    type FileContent = RemoteSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

pub struct RemoteConnectionPrompt {
    connection_string: SharedString,
    nickname: Option<SharedString>,
    is_wsl: bool,
    status_message: Option<SharedString>,
    prompt: Option<(Entity<Markdown>, oneshot::Sender<String>)>,
    cancellation: Option<oneshot::Sender<()>>,
    editor: Entity<Editor>,
}

impl Drop for RemoteConnectionPrompt {
    fn drop(&mut self) {
        if let Some(cancel) = self.cancellation.take() {
            cancel.send(()).ok();
        }
    }
}

pub struct RemoteConnectionModal {
    pub(crate) prompt: Entity<RemoteConnectionPrompt>,
    paths: Vec<PathBuf>,
    finished: bool,
}

impl RemoteConnectionPrompt {
    pub(crate) fn new(
        connection_string: String,
        nickname: Option<String>,
        is_wsl: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            connection_string: connection_string.into(),
            nickname: nickname.map(|nickname| nickname.into()),
            is_wsl,
            editor: cx.new(|cx| Editor::single_line(window, cx)),
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
        tx: oneshot::Sender<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let theme = ThemeSettings::get_global(cx);

        let refinement = TextStyleRefinement {
            font_family: Some(theme.buffer_font.family.clone()),
            font_features: Some(FontFeatures::disable_ligatures()),
            font_size: Some(theme.buffer_font_size(cx).into()),
            color: Some(cx.theme().colors().editor_foreground),
            background_color: Some(gpui::transparent_black()),
            ..Default::default()
        };

        self.editor.update(cx, |editor, cx| {
            if prompt.contains("yes/no") {
                editor.set_masked(false, cx);
            } else {
                editor.set_masked(true, cx);
            }
            editor.set_text_style_refinement(refinement);
            editor.set_cursor_shape(CursorShape::Block, cx);
        });

        let markdown = cx.new(|cx| Markdown::new_text(prompt.into(), cx));
        self.prompt = Some((markdown, tx));
        self.status_message.take();
        window.focus(&self.editor.focus_handle(cx));
        cx.notify();
    }

    pub fn set_status(&mut self, status: Option<String>, cx: &mut Context<Self>) {
        self.status_message = status.map(|s| s.into());
        cx.notify();
    }

    pub fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, tx)) = self.prompt.take() {
            self.status_message = Some("Connecting".into());
            self.editor.update(cx, |editor, cx| {
                tx.send(editor.text(cx)).ok();
                editor.clear(window, cx);
            });
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
                        .child(self.editor.clone()),
                )
                .when(window.capslock().on, |el| {
                    el.child(Label::new("⚠️ ⇪ is on"))
                })
            })
    }
}

impl RemoteConnectionModal {
    pub(crate) fn new(
        connection_options: &RemoteConnectionOptions,
        paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (connection_string, nickname, is_wsl) = match connection_options {
            RemoteConnectionOptions::Ssh(options) => {
                (options.connection_string(), options.nickname.clone(), false)
            }
            RemoteConnectionOptions::Wsl(options) => (options.distro_name.clone(), None, true),
        };
        Self {
            prompt: cx.new(|cx| {
                RemoteConnectionPrompt::new(connection_string, nickname, is_wsl, window, cx)
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
            tx.send(()).ok();
        }
        self.finished(cx);
    }
}

pub(crate) struct SshConnectionHeader {
    pub(crate) connection_string: SharedString,
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) nickname: Option<SharedString>,
    pub(crate) is_wsl: bool,
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

        let icon = match self.is_wsl {
            true => IconName::Linux,
            false => IconName::Server,
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
                            Label::new(path.to_string_lossy().to_string())
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
                }
                .render(window, cx),
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
    ) -> workspace::DismissDecision {
        workspace::DismissDecision::Dismiss(self.finished)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct RemoteClientDelegate {
    window: AnyWindowHandle,
    ui: WeakEntity<RemoteConnectionPrompt>,
    known_password: Option<String>,
}

impl remote::RemoteClientDelegate for RemoteClientDelegate {
    fn ask_password(&self, prompt: String, tx: oneshot::Sender<String>, cx: &mut AsyncApp) {
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
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<PathBuf>> {
        cx.spawn(async move |cx| {
            let binary_path = AutoUpdater::download_remote_server_release(
                platform.os,
                platform.arch,
                release_channel,
                version,
                cx,
            )
            .await
            .with_context(|| {
                format!(
                    "Downloading remote server binary (version: {}, os: {}, arch: {})",
                    version
                        .map(|v| format!("{}", v))
                        .unwrap_or("unknown".to_string()),
                    platform.os,
                    platform.arch,
                )
            })?;
            Ok(binary_path)
        })
    }

    fn get_download_params(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Option<(String, String)>>> {
        cx.spawn(async move |cx| {
            AutoUpdater::get_remote_server_release_url(
                platform.os,
                platform.arch,
                release_channel,
                version,
                cx,
            )
            .await
        })
    }
}

impl RemoteClientDelegate {
    fn update_status(&self, status: Option<&str>, cx: &mut AsyncApp) {
        self.window
            .update(cx, |_, _, cx| {
                self.ui.update(cx, |modal, cx| {
                    modal.set_status(status.map(|s| s.to_string()), cx);
                })
            })
            .ok();
    }
}

pub fn connect_over_ssh(
    unique_identifier: ConnectionIdentifier,
    connection_options: SshConnectionOptions,
    ui: Entity<RemoteConnectionPrompt>,
    window: &mut Window,
    cx: &mut App,
) -> Task<Result<Option<Entity<RemoteClient>>>> {
    let window = window.window_handle();
    let known_password = connection_options.password.clone();
    let (tx, rx) = oneshot::channel();
    ui.update(cx, |ui, _cx| ui.set_cancellation_tx(tx));

    remote::RemoteClient::ssh(
        unique_identifier,
        connection_options,
        rx,
        Arc::new(RemoteClientDelegate {
            window,
            ui: ui.downgrade(),
            known_password,
        }),
        cx,
    )
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
        RemoteConnectionOptions::Ssh(ssh_connection_options) => {
            ssh_connection_options.password.clone()
        }
        _ => None,
    };
    let (tx, rx) = oneshot::channel();
    ui.update(cx, |ui, _cx| ui.set_cancellation_tx(tx));

    remote::RemoteClient::new(
        unique_identifier,
        connection_options,
        rx,
        Arc::new(RemoteClientDelegate {
            window,
            ui: ui.downgrade(),
            known_password,
        }),
        cx,
    )
}

pub async fn open_remote_project(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<AppState>,
    open_options: workspace::OpenOptions,
    cx: &mut AsyncApp,
) -> Result<()> {
    let window = if let Some(window) = open_options.replace_window {
        window
    } else {
        let workspace_position = cx
            .update(|cx| {
                workspace::remote_workspace_position_from_db(connection_options.clone(), &paths, cx)
            })?
            .await
            .context("fetching ssh workspace position from db")?;

        let mut options =
            cx.update(|cx| (app_state.build_window_options)(workspace_position.display, cx))?;
        options.window_bounds = workspace_position.window_bounds;

        cx.open_window(options, |window, cx| {
            let project = project::Project::local(
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                None,
                cx,
            );
            cx.new(|cx| {
                let mut workspace = Workspace::new(None, project, app_state.clone(), window, cx);
                workspace.centered_layout = workspace_position.centered_layout;
                workspace
            })
        })?
    };

    loop {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let delegate = window.update(cx, {
            let paths = paths.clone();
            let connection_options = connection_options.clone();
            move |workspace, window, cx| {
                window.activate_window();
                workspace.toggle_modal(window, cx, |window, cx| {
                    RemoteConnectionModal::new(&connection_options, paths, window, cx)
                });

                let ui = workspace
                    .active_modal::<RemoteConnectionModal>(cx)?
                    .read(cx)
                    .prompt
                    .clone();

                ui.update(cx, |ui, _cx| {
                    ui.set_cancellation_tx(cancel_tx);
                });

                Some(Arc::new(RemoteClientDelegate {
                    window: window.window_handle(),
                    ui: ui.downgrade(),
                    known_password: if let RemoteConnectionOptions::Ssh(options) =
                        &connection_options
                    {
                        options.password.clone()
                    } else {
                        None
                    },
                }))
            }
        })?;

        let Some(delegate) = delegate else { break };

        let did_open_project = cx
            .update(|cx| {
                workspace::open_remote_project_with_new_connection(
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
            .update(cx, |workspace, _, cx| {
                if let Some(ui) = workspace.active_modal::<RemoteConnectionModal>(cx) {
                    ui.update(cx, |modal, cx| modal.finished(cx))
                }
            })
            .ok();

        if let Err(e) = did_open_project {
            log::error!("Failed to open project: {e:?}");
            let response = window
                .update(cx, |_, window, cx| {
                    window.prompt(
                        PromptLevel::Critical,
                        match connection_options {
                            RemoteConnectionOptions::Ssh(_) => "Failed to connect over SSH",
                            RemoteConnectionOptions::Wsl(_) => "Failed to connect to WSL",
                        },
                        Some(&e.to_string()),
                        &["Retry", "Ok"],
                        cx,
                    )
                })?
                .await;

            if response == Ok(0) {
                continue;
            }
        }

        window
            .update(cx, |workspace, _, cx| {
                if let Some(client) = workspace.project().read(cx).remote_client() {
                    ExtensionStore::global(cx)
                        .update(cx, |store, cx| store.register_remote_client(client, cx));
                }
            })
            .ok();

        break;
    }

    // Already showed the error to the user
    Ok(())
}
