pub mod copilot_chat;
mod copilot_edit_prediction_delegate;
pub mod copilot_responses;
pub mod request;
mod sign_in;

use crate::sign_in::initiate_sign_out;
use ::fs::Fs;
use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use command_palette_hooks::CommandPaletteFilter;
use futures::{Future, FutureExt, TryFutureExt, channel::oneshot, future::Shared};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, EventEmitter, Global, Task,
    WeakEntity, actions,
};
use http_client::HttpClient;
use language::language_settings::CopilotSettings;
use language::{
    Anchor, Bias, Buffer, BufferSnapshot, Language, PointUtf16, ToPointUtf16,
    language_settings::{EditPredictionProvider, all_language_settings, language_settings},
    point_from_lsp, point_to_lsp,
};
use lsp::{LanguageServer, LanguageServerBinary, LanguageServerId, LanguageServerName};
use node_runtime::{NodeRuntime, VersionStrategy};
use parking_lot::Mutex;
use project::DisableAiSettings;
use request::StatusNotification;
use semver::Version;
use serde_json::json;
use settings::{Settings, SettingsStore};
use std::{
    any::TypeId,
    collections::hash_map::Entry,
    env,
    ffi::OsString,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::Dimensions;
use util::{ResultExt, fs::remove_matching, rel_path::RelPath};
use workspace::Workspace;

pub use crate::copilot_edit_prediction_delegate::CopilotEditPredictionDelegate;
pub use crate::sign_in::{
    ConfigurationMode, ConfigurationView, CopilotCodeVerification, initiate_sign_in,
    reinstall_and_sign_in,
};

actions!(
    copilot,
    [
        /// Requests a code completion suggestion from Copilot.
        Suggest,
        /// Cycles to the next Copilot suggestion.
        NextSuggestion,
        /// Cycles to the previous Copilot suggestion.
        PreviousSuggestion,
        /// Reinstalls the Copilot language server.
        Reinstall,
        /// Signs in to GitHub Copilot.
        SignIn,
        /// Signs out of GitHub Copilot.
        SignOut
    ]
);

pub fn init(
    new_server_id: LanguageServerId,
    fs: Arc<dyn Fs>,
    http: Arc<dyn HttpClient>,
    node_runtime: NodeRuntime,
    cx: &mut App,
) {
    let language_settings = all_language_settings(None, cx);
    let configuration = copilot_chat::CopilotChatConfiguration {
        enterprise_uri: language_settings
            .edit_predictions
            .copilot
            .enterprise_uri
            .clone(),
    };
    copilot_chat::init(fs.clone(), http.clone(), configuration, cx);

    let copilot = cx.new(move |cx| Copilot::start(new_server_id, fs, node_runtime, cx));
    Copilot::set_global(copilot.clone(), cx);
    cx.observe(&copilot, |copilot, cx| {
        copilot.update(cx, |copilot, cx| copilot.update_action_visibilities(cx));
    })
    .detach();
    cx.observe_global::<SettingsStore>(|cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot.update(cx, |copilot, cx| copilot.update_action_visibilities(cx));
        }
    })
    .detach();

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, _: &SignIn, window, cx| {
            initiate_sign_in(window, cx);
        });
        workspace.register_action(|_, _: &Reinstall, window, cx| {
            reinstall_and_sign_in(window, cx);
        });
        workspace.register_action(|_, _: &SignOut, window, cx| {
            initiate_sign_out(window, cx);
        });
    })
    .detach();
}

enum CopilotServer {
    Disabled,
    Starting { task: Shared<Task<()>> },
    Error(Arc<str>),
    Running(RunningCopilotServer),
}

impl CopilotServer {
    fn as_authenticated(&mut self) -> Result<&mut RunningCopilotServer> {
        let server = self.as_running()?;
        anyhow::ensure!(
            matches!(server.sign_in_status, SignInStatus::Authorized),
            "must sign in before using copilot"
        );
        Ok(server)
    }

    fn as_running(&mut self) -> Result<&mut RunningCopilotServer> {
        match self {
            CopilotServer::Starting { .. } => anyhow::bail!("copilot is still starting"),
            CopilotServer::Disabled => anyhow::bail!("copilot is disabled"),
            CopilotServer::Error(error) => {
                anyhow::bail!("copilot was not started because of an error: {error}")
            }
            CopilotServer::Running(server) => Ok(server),
        }
    }
}

struct RunningCopilotServer {
    lsp: Arc<LanguageServer>,
    sign_in_status: SignInStatus,
    registered_buffers: HashMap<EntityId, RegisteredBuffer>,
}

#[derive(Clone, Debug)]
enum SignInStatus {
    Authorized,
    Unauthorized,
    SigningIn {
        prompt: Option<request::PromptUserDeviceFlow>,
        task: Shared<Task<Result<(), Arc<anyhow::Error>>>>,
    },
    SignedOut {
        awaiting_signing_in: bool,
    },
}

#[derive(Debug, Clone)]
pub enum Status {
    Starting {
        task: Shared<Task<()>>,
    },
    Error(Arc<str>),
    Disabled,
    SignedOut {
        awaiting_signing_in: bool,
    },
    SigningIn {
        prompt: Option<request::PromptUserDeviceFlow>,
    },
    Unauthorized,
    Authorized,
}

impl Status {
    pub fn is_authorized(&self) -> bool {
        matches!(self, Status::Authorized)
    }

    pub fn is_configured(&self) -> bool {
        matches!(
            self,
            Status::Starting { .. }
                | Status::Error(_)
                | Status::SigningIn { .. }
                | Status::Authorized
        )
    }
}

struct RegisteredBuffer {
    uri: lsp::Uri,
    language_id: String,
    snapshot: BufferSnapshot,
    snapshot_version: i32,
    _subscriptions: [gpui::Subscription; 2],
    pending_buffer_change: Task<Option<()>>,
}

impl RegisteredBuffer {
    fn report_changes(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Copilot>,
    ) -> oneshot::Receiver<(i32, BufferSnapshot)> {
        let (done_tx, done_rx) = oneshot::channel();

        if buffer.read(cx).version() == self.snapshot.version {
            let _ = done_tx.send((self.snapshot_version, self.snapshot.clone()));
        } else {
            let buffer = buffer.downgrade();
            let id = buffer.entity_id();
            let prev_pending_change =
                mem::replace(&mut self.pending_buffer_change, Task::ready(None));
            self.pending_buffer_change = cx.spawn(async move |copilot, cx| {
                prev_pending_change.await;

                let old_version = copilot
                    .update(cx, |copilot, _| {
                        let server = copilot.server.as_authenticated().log_err()?;
                        let buffer = server.registered_buffers.get_mut(&id)?;
                        Some(buffer.snapshot.version.clone())
                    })
                    .ok()??;
                let new_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot()).ok()?;

                let content_changes = cx
                    .background_spawn({
                        let new_snapshot = new_snapshot.clone();
                        async move {
                            new_snapshot
                                .edits_since::<Dimensions<PointUtf16, usize>>(&old_version)
                                .map(|edit| {
                                    let edit_start = edit.new.start.0;
                                    let edit_end = edit_start + (edit.old.end.0 - edit.old.start.0);
                                    let new_text = new_snapshot
                                        .text_for_range(edit.new.start.1..edit.new.end.1)
                                        .collect();
                                    lsp::TextDocumentContentChangeEvent {
                                        range: Some(lsp::Range::new(
                                            point_to_lsp(edit_start),
                                            point_to_lsp(edit_end),
                                        )),
                                        range_length: None,
                                        text: new_text,
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    })
                    .await;

                copilot
                    .update(cx, |copilot, _| {
                        let server = copilot.server.as_authenticated().log_err()?;
                        let buffer = server.registered_buffers.get_mut(&id)?;
                        if !content_changes.is_empty() {
                            buffer.snapshot_version += 1;
                            buffer.snapshot = new_snapshot;
                            server
                                .lsp
                                .notify::<lsp::notification::DidChangeTextDocument>(
                                    lsp::DidChangeTextDocumentParams {
                                        text_document: lsp::VersionedTextDocumentIdentifier::new(
                                            buffer.uri.clone(),
                                            buffer.snapshot_version,
                                        ),
                                        content_changes,
                                    },
                                )
                                .ok();
                        }
                        let _ = done_tx.send((buffer.snapshot_version, buffer.snapshot.clone()));
                        Some(())
                    })
                    .ok()?;

                Some(())
            });
        }

        done_rx
    }
}

#[derive(Debug)]
pub struct Completion {
    pub uuid: String,
    pub range: Range<Anchor>,
    pub text: String,
}

pub struct Copilot {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    server: CopilotServer,
    buffers: HashSet<WeakEntity<Buffer>>,
    server_id: LanguageServerId,
    _subscription: gpui::Subscription,
}

pub enum Event {
    CopilotLanguageServerStarted,
    CopilotAuthSignedIn,
    CopilotAuthSignedOut,
}

impl EventEmitter<Event> for Copilot {}

struct GlobalCopilot(Entity<Copilot>);

impl Global for GlobalCopilot {}

impl Copilot {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalCopilot>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(copilot: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalCopilot(copilot));
    }

    fn start(
        new_server_id: LanguageServerId,
        fs: Arc<dyn Fs>,
        node_runtime: NodeRuntime,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            server_id: new_server_id,
            fs,
            node_runtime,
            server: CopilotServer::Disabled,
            buffers: Default::default(),
            _subscription: cx.on_app_quit(Self::shutdown_language_server),
        };
        this.start_copilot(true, false, cx);
        cx.observe_global::<SettingsStore>(move |this, cx| {
            this.start_copilot(true, false, cx);
            if let Ok(server) = this.server.as_running() {
                notify_did_change_config_to_server(&server.lsp, cx)
                    .context("copilot setting change: did change configuration")
                    .log_err();
            }
        })
        .detach();
        this
    }

    fn shutdown_language_server(
        &mut self,
        _cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> + use<> {
        let shutdown = match mem::replace(&mut self.server, CopilotServer::Disabled) {
            CopilotServer::Running(server) => Some(Box::pin(async move { server.lsp.shutdown() })),
            _ => None,
        };

        async move {
            if let Some(shutdown) = shutdown {
                shutdown.await;
            }
        }
    }

    pub fn start_copilot(
        &mut self,
        check_edit_prediction_provider: bool,
        awaiting_sign_in_after_start: bool,
        cx: &mut Context<Self>,
    ) {
        if !matches!(self.server, CopilotServer::Disabled) {
            return;
        }
        let language_settings = all_language_settings(None, cx);
        if check_edit_prediction_provider
            && language_settings.edit_predictions.provider != EditPredictionProvider::Copilot
        {
            return;
        }
        let server_id = self.server_id;
        let fs = self.fs.clone();
        let node_runtime = self.node_runtime.clone();
        let env = self.build_env(&language_settings.edit_predictions.copilot);
        let start_task = cx
            .spawn(async move |this, cx| {
                Self::start_language_server(
                    server_id,
                    fs,
                    node_runtime,
                    env,
                    this,
                    awaiting_sign_in_after_start,
                    cx,
                )
                .await
            })
            .shared();
        self.server = CopilotServer::Starting { task: start_task };
        cx.notify();
    }

    fn build_env(&self, copilot_settings: &CopilotSettings) -> Option<HashMap<String, String>> {
        let proxy_url = copilot_settings.proxy.clone()?;
        let no_verify = copilot_settings.proxy_no_verify;
        let http_or_https_proxy = if proxy_url.starts_with("http:") {
            Some("HTTP_PROXY")
        } else if proxy_url.starts_with("https:") {
            Some("HTTPS_PROXY")
        } else {
            log::error!(
                "Unsupported protocol scheme for language server proxy (must be http or https)"
            );
            None
        };

        let mut env = HashMap::default();

        if let Some(proxy_type) = http_or_https_proxy {
            env.insert(proxy_type.to_string(), proxy_url);
            if let Some(true) = no_verify {
                env.insert("NODE_TLS_REJECT_UNAUTHORIZED".to_string(), "0".to_string());
            };
        }

        if let Ok(oauth_token) = env::var(copilot_chat::COPILOT_OAUTH_ENV_VAR) {
            env.insert(copilot_chat::COPILOT_OAUTH_ENV_VAR.to_string(), oauth_token);
        }

        if env.is_empty() { None } else { Some(env) }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(cx: &mut gpui::TestAppContext) -> (Entity<Self>, lsp::FakeLanguageServer) {
        use fs::FakeFs;
        use lsp::FakeLanguageServer;
        use node_runtime::NodeRuntime;

        let (server, fake_server) = FakeLanguageServer::new(
            LanguageServerId(0),
            LanguageServerBinary {
                path: "path/to/copilot".into(),
                arguments: vec![],
                env: None,
            },
            "copilot".into(),
            Default::default(),
            &mut cx.to_async(),
        );
        let node_runtime = NodeRuntime::unavailable();
        let this = cx.new(|cx| Self {
            server_id: LanguageServerId(0),
            fs: FakeFs::new(cx.background_executor().clone()),
            node_runtime,
            server: CopilotServer::Running(RunningCopilotServer {
                lsp: Arc::new(server),
                sign_in_status: SignInStatus::Authorized,
                registered_buffers: Default::default(),
            }),
            _subscription: cx.on_app_quit(Self::shutdown_language_server),
            buffers: Default::default(),
        });
        (this, fake_server)
    }

    async fn start_language_server(
        new_server_id: LanguageServerId,
        fs: Arc<dyn Fs>,
        node_runtime: NodeRuntime,
        env: Option<HashMap<String, String>>,
        this: WeakEntity<Self>,
        awaiting_sign_in_after_start: bool,
        cx: &mut AsyncApp,
    ) {
        let start_language_server = async {
            let server_path = get_copilot_lsp(fs, node_runtime.clone()).await?;
            let node_path = node_runtime.binary_path().await?;
            ensure_node_version_for_copilot(&node_path).await?;

            let arguments: Vec<OsString> = vec![
                "--experimental-sqlite".into(),
                server_path.into(),
                "--stdio".into(),
            ];
            let binary = LanguageServerBinary {
                path: node_path,
                arguments,
                env,
            };

            let root_path = if cfg!(target_os = "windows") {
                Path::new("C:/")
            } else {
                Path::new("/")
            };

            let server_name = LanguageServerName("copilot".into());
            let server = LanguageServer::new(
                Arc::new(Mutex::new(None)),
                new_server_id,
                server_name,
                binary,
                root_path,
                None,
                Default::default(),
                cx,
            )?;

            server
                .on_notification::<StatusNotification, _>(|_, _| { /* Silence the notification */ })
                .detach();

            let configuration = lsp::DidChangeConfigurationParams {
                settings: Default::default(),
            };

            let editor_info = request::SetEditorInfoParams {
                editor_info: request::EditorInfo {
                    name: "zed".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                },
                editor_plugin_info: request::EditorPluginInfo {
                    name: "zed-copilot".into(),
                    version: "0.0.1".into(),
                },
            };
            let editor_info_json = serde_json::to_value(&editor_info)?;

            let server = cx
                .update(|cx| {
                    let mut params = server.default_initialize_params(false, cx);
                    params.initialization_options = Some(editor_info_json);
                    server.initialize(params, configuration.into(), cx)
                })?
                .await?;

            this.update(cx, |_, cx| notify_did_change_config_to_server(&server, cx))?
                .context("copilot: did change configuration")?;

            let status = server
                .request::<request::CheckStatus>(request::CheckStatusParams {
                    local_checks_only: false,
                })
                .await
                .into_response()
                .context("copilot: check status")?;

            anyhow::Ok((server, status))
        };

        let server = start_language_server.await;
        this.update(cx, |this, cx| {
            cx.notify();

            if env::var("ZED_FORCE_COPILOT_ERROR").is_ok() {
                this.server = CopilotServer::Error(
                    "Forced error for testing (ZED_FORCE_COPILOT_ERROR)".into(),
                );
                return;
            }

            match server {
                Ok((server, status)) => {
                    this.server = CopilotServer::Running(RunningCopilotServer {
                        lsp: server,
                        sign_in_status: SignInStatus::SignedOut {
                            awaiting_signing_in: awaiting_sign_in_after_start,
                        },
                        registered_buffers: Default::default(),
                    });
                    cx.emit(Event::CopilotLanguageServerStarted);
                    this.update_sign_in_status(status, cx);
                }
                Err(error) => {
                    this.server = CopilotServer::Error(error.to_string().into());
                    cx.notify()
                }
            }
        })
        .ok();
    }

    pub fn is_authenticated(&self) -> bool {
        return matches!(
            self.server,
            CopilotServer::Running(RunningCopilotServer {
                sign_in_status: SignInStatus::Authorized,
                ..
            })
        );
    }

    pub fn sign_in(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        if let CopilotServer::Running(server) = &mut self.server {
            let task = match &server.sign_in_status {
                SignInStatus::Authorized => Task::ready(Ok(())).shared(),
                SignInStatus::SigningIn { task, .. } => {
                    cx.notify();
                    task.clone()
                }
                SignInStatus::SignedOut { .. } | SignInStatus::Unauthorized => {
                    let lsp = server.lsp.clone();
                    let task = cx
                        .spawn(async move |this, cx| {
                            let sign_in = async {
                                let sign_in = lsp
                                    .request::<request::SignInInitiate>(
                                        request::SignInInitiateParams {},
                                    )
                                    .await
                                    .into_response()
                                    .context("copilot sign-in")?;
                                match sign_in {
                                    request::SignInInitiateResult::AlreadySignedIn { user } => {
                                        Ok(request::SignInStatus::Ok { user: Some(user) })
                                    }
                                    request::SignInInitiateResult::PromptUserDeviceFlow(flow) => {
                                        this.update(cx, |this, cx| {
                                            if let CopilotServer::Running(RunningCopilotServer {
                                                sign_in_status: status,
                                                ..
                                            }) = &mut this.server
                                                && let SignInStatus::SigningIn {
                                                    prompt: prompt_flow,
                                                    ..
                                                } = status
                                            {
                                                *prompt_flow = Some(flow.clone());
                                                cx.notify();
                                            }
                                        })?;
                                        let response = lsp
                                            .request::<request::SignInConfirm>(
                                                request::SignInConfirmParams {
                                                    user_code: flow.user_code,
                                                },
                                            )
                                            .await
                                            .into_response()
                                            .context("copilot: sign in confirm")?;
                                        Ok(response)
                                    }
                                }
                            };

                            let sign_in = sign_in.await;
                            this.update(cx, |this, cx| match sign_in {
                                Ok(status) => {
                                    this.update_sign_in_status(status, cx);
                                    Ok(())
                                }
                                Err(error) => {
                                    this.update_sign_in_status(
                                        request::SignInStatus::NotSignedIn,
                                        cx,
                                    );
                                    Err(Arc::new(error))
                                }
                            })?
                        })
                        .shared();
                    server.sign_in_status = SignInStatus::SigningIn {
                        prompt: None,
                        task: task.clone(),
                    };
                    cx.notify();
                    task
                }
            };

            cx.background_spawn(task.map_err(|err| anyhow!("{err:?}")))
        } else {
            // If we're downloading, wait until download is finished
            // If we're in a stuck state, display to the user
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    pub(crate) fn sign_out(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.update_sign_in_status(request::SignInStatus::NotSignedIn, cx);
        match &self.server {
            CopilotServer::Running(RunningCopilotServer { lsp: server, .. }) => {
                let server = server.clone();
                cx.background_spawn(async move {
                    server
                        .request::<request::SignOut>(request::SignOutParams {})
                        .await
                        .into_response()
                        .context("copilot: sign in confirm")?;
                    anyhow::Ok(())
                })
            }
            CopilotServer::Disabled => cx.background_spawn(async {
                clear_copilot_config_dir().await;
                anyhow::Ok(())
            }),
            _ => Task::ready(Err(anyhow!("copilot hasn't started yet"))),
        }
    }

    pub(crate) fn reinstall(&mut self, cx: &mut Context<Self>) -> Shared<Task<()>> {
        let language_settings = all_language_settings(None, cx);
        let env = self.build_env(&language_settings.edit_predictions.copilot);
        let start_task = cx
            .spawn({
                let fs = self.fs.clone();
                let node_runtime = self.node_runtime.clone();
                let server_id = self.server_id;
                async move |this, cx| {
                    clear_copilot_dir().await;
                    Self::start_language_server(server_id, fs, node_runtime, env, this, false, cx)
                        .await
                }
            })
            .shared();

        self.server = CopilotServer::Starting {
            task: start_task.clone(),
        };

        cx.notify();

        start_task
    }

    pub fn language_server(&self) -> Option<&Arc<LanguageServer>> {
        if let CopilotServer::Running(server) = &self.server {
            Some(&server.lsp)
        } else {
            None
        }
    }

    pub fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let weak_buffer = buffer.downgrade();
        self.buffers.insert(weak_buffer.clone());

        if let CopilotServer::Running(RunningCopilotServer {
            lsp: server,
            sign_in_status: status,
            registered_buffers,
            ..
        }) = &mut self.server
        {
            if !matches!(status, SignInStatus::Authorized) {
                return;
            }

            let entry = registered_buffers.entry(buffer.entity_id());
            if let Entry::Vacant(e) = entry {
                let Ok(uri) = uri_for_buffer(buffer, cx) else {
                    return;
                };
                let language_id = id_for_language(buffer.read(cx).language());
                let snapshot = buffer.read(cx).snapshot();
                server
                    .notify::<lsp::notification::DidOpenTextDocument>(
                        lsp::DidOpenTextDocumentParams {
                            text_document: lsp::TextDocumentItem {
                                uri: uri.clone(),
                                language_id: language_id.clone(),
                                version: 0,
                                text: snapshot.text(),
                            },
                        },
                    )
                    .ok();

                e.insert(RegisteredBuffer {
                    uri,
                    language_id,
                    snapshot,
                    snapshot_version: 0,
                    pending_buffer_change: Task::ready(Some(())),
                    _subscriptions: [
                        cx.subscribe(buffer, |this, buffer, event, cx| {
                            this.handle_buffer_event(buffer, event, cx).log_err();
                        }),
                        cx.observe_release(buffer, move |this, _buffer, _cx| {
                            this.buffers.remove(&weak_buffer);
                            this.unregister_buffer(&weak_buffer);
                        }),
                    ],
                });
            }
        }
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Ok(server) = self.server.as_running()
            && let Some(registered_buffer) = server.registered_buffers.get_mut(&buffer.entity_id())
        {
            match event {
                language::BufferEvent::Edited => {
                    drop(registered_buffer.report_changes(&buffer, cx));
                }
                language::BufferEvent::Saved => {
                    server
                        .lsp
                        .notify::<lsp::notification::DidSaveTextDocument>(
                            lsp::DidSaveTextDocumentParams {
                                text_document: lsp::TextDocumentIdentifier::new(
                                    registered_buffer.uri.clone(),
                                ),
                                text: None,
                            },
                        )
                        .ok();
                }
                language::BufferEvent::FileHandleChanged
                | language::BufferEvent::LanguageChanged(_) => {
                    let new_language_id = id_for_language(buffer.read(cx).language());
                    let Ok(new_uri) = uri_for_buffer(&buffer, cx) else {
                        return Ok(());
                    };
                    if new_uri != registered_buffer.uri
                        || new_language_id != registered_buffer.language_id
                    {
                        let old_uri = mem::replace(&mut registered_buffer.uri, new_uri);
                        registered_buffer.language_id = new_language_id;
                        server
                            .lsp
                            .notify::<lsp::notification::DidCloseTextDocument>(
                                lsp::DidCloseTextDocumentParams {
                                    text_document: lsp::TextDocumentIdentifier::new(old_uri),
                                },
                            )
                            .ok();
                        server
                            .lsp
                            .notify::<lsp::notification::DidOpenTextDocument>(
                                lsp::DidOpenTextDocumentParams {
                                    text_document: lsp::TextDocumentItem::new(
                                        registered_buffer.uri.clone(),
                                        registered_buffer.language_id.clone(),
                                        registered_buffer.snapshot_version,
                                        registered_buffer.snapshot.text(),
                                    ),
                                },
                            )
                            .ok();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn unregister_buffer(&mut self, buffer: &WeakEntity<Buffer>) {
        if let Ok(server) = self.server.as_running()
            && let Some(buffer) = server.registered_buffers.remove(&buffer.entity_id())
        {
            server
                .lsp
                .notify::<lsp::notification::DidCloseTextDocument>(
                    lsp::DidCloseTextDocumentParams {
                        text_document: lsp::TextDocumentIdentifier::new(buffer.uri),
                    },
                )
                .ok();
        }
    }

    pub fn completions<T>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        T: ToPointUtf16,
    {
        self.request_completions::<request::GetCompletions, _>(buffer, position, cx)
    }

    pub fn completions_cycling<T>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        T: ToPointUtf16,
    {
        self.request_completions::<request::GetCompletionsCycling, _>(buffer, position, cx)
    }

    pub fn accept_completion(
        &mut self,
        completion: &Completion,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let server = match self.server.as_authenticated() {
            Ok(server) => server,
            Err(error) => return Task::ready(Err(error)),
        };
        let request =
            server
                .lsp
                .request::<request::NotifyAccepted>(request::NotifyAcceptedParams {
                    uuid: completion.uuid.clone(),
                });
        cx.background_spawn(async move {
            request
                .await
                .into_response()
                .context("copilot: notify accepted")?;
            Ok(())
        })
    }

    pub fn discard_completions(
        &mut self,
        completions: &[Completion],
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let server = match self.server.as_authenticated() {
            Ok(server) => server,
            Err(_) => return Task::ready(Ok(())),
        };
        let request =
            server
                .lsp
                .request::<request::NotifyRejected>(request::NotifyRejectedParams {
                    uuids: completions
                        .iter()
                        .map(|completion| completion.uuid.clone())
                        .collect(),
                });
        cx.background_spawn(async move {
            request
                .await
                .into_response()
                .context("copilot: notify rejected")?;
            Ok(())
        })
    }

    fn request_completions<R, T>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        R: 'static
            + lsp::request::Request<
                Params = request::GetCompletionsParams,
                Result = request::GetCompletionsResult,
            >,
        T: ToPointUtf16,
    {
        self.register_buffer(buffer, cx);

        let server = match self.server.as_authenticated() {
            Ok(server) => server,
            Err(error) => return Task::ready(Err(error)),
        };
        let lsp = server.lsp.clone();
        let registered_buffer = server
            .registered_buffers
            .get_mut(&buffer.entity_id())
            .unwrap();
        let snapshot = registered_buffer.report_changes(buffer, cx);
        let buffer = buffer.read(cx);
        let uri = registered_buffer.uri.clone();
        let position = position.to_point_utf16(buffer);
        let settings = language_settings(
            buffer.language_at(position).map(|l| l.name()),
            buffer.file(),
            cx,
        );
        let tab_size = settings.tab_size;
        let hard_tabs = settings.hard_tabs;
        let relative_path = buffer
            .file()
            .map_or(RelPath::empty().into(), |file| file.path().clone());

        cx.background_spawn(async move {
            let (version, snapshot) = snapshot.await?;
            let result = lsp
                .request::<R>(request::GetCompletionsParams {
                    doc: request::GetCompletionsDocument {
                        uri,
                        tab_size: tab_size.into(),
                        indent_size: 1,
                        insert_spaces: !hard_tabs,
                        relative_path: relative_path.to_proto(),
                        position: point_to_lsp(position),
                        version: version.try_into().unwrap(),
                    },
                })
                .await
                .into_response()
                .context("copilot: get completions")?;
            let completions = result
                .completions
                .into_iter()
                .map(|completion| {
                    let start = snapshot
                        .clip_point_utf16(point_from_lsp(completion.range.start), Bias::Left);
                    let end =
                        snapshot.clip_point_utf16(point_from_lsp(completion.range.end), Bias::Left);
                    Completion {
                        uuid: completion.uuid,
                        range: snapshot.anchor_before(start)..snapshot.anchor_after(end),
                        text: completion.text,
                    }
                })
                .collect();
            anyhow::Ok(completions)
        })
    }

    pub fn status(&self) -> Status {
        match &self.server {
            CopilotServer::Starting { task } => Status::Starting { task: task.clone() },
            CopilotServer::Disabled => Status::Disabled,
            CopilotServer::Error(error) => Status::Error(error.clone()),
            CopilotServer::Running(RunningCopilotServer { sign_in_status, .. }) => {
                match sign_in_status {
                    SignInStatus::Authorized => Status::Authorized,
                    SignInStatus::Unauthorized => Status::Unauthorized,
                    SignInStatus::SigningIn { prompt, .. } => Status::SigningIn {
                        prompt: prompt.clone(),
                    },
                    SignInStatus::SignedOut {
                        awaiting_signing_in,
                    } => Status::SignedOut {
                        awaiting_signing_in: *awaiting_signing_in,
                    },
                }
            }
        }
    }

    fn update_sign_in_status(&mut self, lsp_status: request::SignInStatus, cx: &mut Context<Self>) {
        self.buffers.retain(|buffer| buffer.is_upgradable());

        if let Ok(server) = self.server.as_running() {
            match lsp_status {
                request::SignInStatus::Ok { user: Some(_) }
                | request::SignInStatus::MaybeOk { .. }
                | request::SignInStatus::AlreadySignedIn { .. } => {
                    server.sign_in_status = SignInStatus::Authorized;
                    cx.emit(Event::CopilotAuthSignedIn);
                    for buffer in self.buffers.iter().cloned().collect::<Vec<_>>() {
                        if let Some(buffer) = buffer.upgrade() {
                            self.register_buffer(&buffer, cx);
                        }
                    }
                }
                request::SignInStatus::NotAuthorized { .. } => {
                    server.sign_in_status = SignInStatus::Unauthorized;
                    for buffer in self.buffers.iter().cloned().collect::<Vec<_>>() {
                        self.unregister_buffer(&buffer);
                    }
                }
                request::SignInStatus::Ok { user: None } | request::SignInStatus::NotSignedIn => {
                    if !matches!(server.sign_in_status, SignInStatus::SignedOut { .. }) {
                        server.sign_in_status = SignInStatus::SignedOut {
                            awaiting_signing_in: false,
                        };
                    }
                    cx.emit(Event::CopilotAuthSignedOut);
                    for buffer in self.buffers.iter().cloned().collect::<Vec<_>>() {
                        self.unregister_buffer(&buffer);
                    }
                }
            }

            cx.notify();
        }
    }

    fn update_action_visibilities(&self, cx: &mut App) {
        let signed_in_actions = [
            TypeId::of::<Suggest>(),
            TypeId::of::<NextSuggestion>(),
            TypeId::of::<PreviousSuggestion>(),
            TypeId::of::<Reinstall>(),
        ];
        let auth_actions = [TypeId::of::<SignOut>()];
        let no_auth_actions = [TypeId::of::<SignIn>()];
        let status = self.status();

        let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let filter = CommandPaletteFilter::global_mut(cx);

        if is_ai_disabled {
            filter.hide_action_types(&signed_in_actions);
            filter.hide_action_types(&auth_actions);
            filter.hide_action_types(&no_auth_actions);
        } else {
            match status {
                Status::Disabled => {
                    filter.hide_action_types(&signed_in_actions);
                    filter.hide_action_types(&auth_actions);
                    filter.hide_action_types(&no_auth_actions);
                }
                Status::Authorized => {
                    filter.hide_action_types(&no_auth_actions);
                    filter.show_action_types(signed_in_actions.iter().chain(&auth_actions));
                }
                _ => {
                    filter.hide_action_types(&signed_in_actions);
                    filter.hide_action_types(&auth_actions);
                    filter.show_action_types(&no_auth_actions);
                }
            }
        }
    }
}

fn id_for_language(language: Option<&Arc<Language>>) -> String {
    language
        .map(|language| language.lsp_id())
        .unwrap_or_else(|| "plaintext".to_string())
}

fn uri_for_buffer(buffer: &Entity<Buffer>, cx: &App) -> Result<lsp::Uri, ()> {
    if let Some(file) = buffer.read(cx).file().and_then(|file| file.as_local()) {
        lsp::Uri::from_file_path(file.abs_path(cx))
    } else {
        format!("buffer://{}", buffer.entity_id())
            .parse()
            .map_err(|_| ())
    }
}

fn notify_did_change_config_to_server(
    server: &Arc<LanguageServer>,
    cx: &mut Context<Copilot>,
) -> std::result::Result<(), anyhow::Error> {
    let copilot_settings = all_language_settings(None, cx)
        .edit_predictions
        .copilot
        .clone();

    if let Some(copilot_chat) = copilot_chat::CopilotChat::global(cx) {
        copilot_chat.update(cx, |chat, cx| {
            chat.set_configuration(
                copilot_chat::CopilotChatConfiguration {
                    enterprise_uri: copilot_settings.enterprise_uri.clone(),
                },
                cx,
            );
        });
    }

    let settings = json!({
        "http": {
            "proxy": copilot_settings.proxy,
            "proxyStrictSSL": !copilot_settings.proxy_no_verify.unwrap_or(false)
        },
        "github-enterprise": {
            "uri": copilot_settings.enterprise_uri
        }
    });

    server
        .notify::<lsp::notification::DidChangeConfiguration>(lsp::DidChangeConfigurationParams {
            settings,
        })
        .ok();
    Ok(())
}

async fn clear_copilot_dir() {
    remove_matching(paths::copilot_dir(), |_| true).await
}

async fn clear_copilot_config_dir() {
    remove_matching(copilot_chat::copilot_chat_config_dir(), |_| true).await
}

async fn ensure_node_version_for_copilot(node_path: &Path) -> anyhow::Result<()> {
    const MIN_COPILOT_NODE_VERSION: Version = Version::new(20, 8, 0);

    log::info!("Checking Node.js version for Copilot at: {:?}", node_path);

    let output = util::command::new_smol_command(node_path)
        .arg("--version")
        .output()
        .await
        .with_context(|| format!("checking Node.js version at {:?}", node_path))?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to run node --version for Copilot. stdout: {}, stderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    let version_str = String::from_utf8_lossy(&output.stdout);
    let version = Version::parse(version_str.trim().trim_start_matches('v'))
        .with_context(|| format!("parsing Node.js version from '{}'", version_str.trim()))?;

    if version < MIN_COPILOT_NODE_VERSION {
        anyhow::bail!(
            "GitHub Copilot language server requires Node.js {MIN_COPILOT_NODE_VERSION} or later, but found {version}. \
            Please update your Node.js version or configure a different Node.js path in settings."
        );
    }

    log::info!(
        "Node.js version {} meets Copilot requirements (>= {})",
        version,
        MIN_COPILOT_NODE_VERSION
    );
    Ok(())
}

async fn get_copilot_lsp(fs: Arc<dyn Fs>, node_runtime: NodeRuntime) -> anyhow::Result<PathBuf> {
    const PACKAGE_NAME: &str = "@github/copilot-language-server";
    const SERVER_PATH: &str =
        "node_modules/@github/copilot-language-server/dist/language-server.js";

    let latest_version = node_runtime
        .npm_package_latest_version(PACKAGE_NAME)
        .await?;
    let server_path = paths::copilot_dir().join(SERVER_PATH);

    fs.create_dir(paths::copilot_dir()).await?;

    let should_install = node_runtime
        .should_install_npm_package(
            PACKAGE_NAME,
            &server_path,
            paths::copilot_dir(),
            VersionStrategy::Latest(&latest_version),
        )
        .await;
    if should_install {
        node_runtime
            .npm_install_packages(paths::copilot_dir(), &[(PACKAGE_NAME, &latest_version)])
            .await?;
    }

    Ok(server_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use util::{path, paths::PathStyle, rel_path::rel_path};

    #[gpui::test(iterations = 10)]
    async fn test_buffer_management(cx: &mut TestAppContext) {
        let (copilot, mut lsp) = Copilot::fake(cx);

        let buffer_1 = cx.new(|cx| Buffer::local("Hello", cx));
        let buffer_1_uri: lsp::Uri = format!("buffer://{}", buffer_1.entity_id().as_u64())
            .parse()
            .unwrap();
        copilot.update(cx, |copilot, cx| copilot.register_buffer(&buffer_1, cx));
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await,
            lsp::DidOpenTextDocumentParams {
                text_document: lsp::TextDocumentItem::new(
                    buffer_1_uri.clone(),
                    "plaintext".into(),
                    0,
                    "Hello".into()
                ),
            }
        );

        let buffer_2 = cx.new(|cx| Buffer::local("Goodbye", cx));
        let buffer_2_uri: lsp::Uri = format!("buffer://{}", buffer_2.entity_id().as_u64())
            .parse()
            .unwrap();
        copilot.update(cx, |copilot, cx| copilot.register_buffer(&buffer_2, cx));
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await,
            lsp::DidOpenTextDocumentParams {
                text_document: lsp::TextDocumentItem::new(
                    buffer_2_uri.clone(),
                    "plaintext".into(),
                    0,
                    "Goodbye".into()
                ),
            }
        );

        buffer_1.update(cx, |buffer, cx| buffer.edit([(5..5, " world")], None, cx));
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidChangeTextDocument>()
                .await,
            lsp::DidChangeTextDocumentParams {
                text_document: lsp::VersionedTextDocumentIdentifier::new(buffer_1_uri.clone(), 1),
                content_changes: vec![lsp::TextDocumentContentChangeEvent {
                    range: Some(lsp::Range::new(
                        lsp::Position::new(0, 5),
                        lsp::Position::new(0, 5)
                    )),
                    range_length: None,
                    text: " world".into(),
                }],
            }
        );

        // Ensure updates to the file are reflected in the LSP.
        buffer_1.update(cx, |buffer, cx| {
            buffer.file_updated(
                Arc::new(File {
                    abs_path: path!("/root/child/buffer-1").into(),
                    path: rel_path("child/buffer-1").into(),
                }),
                cx,
            )
        });
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            lsp::DidCloseTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier::new(buffer_1_uri),
            }
        );
        let buffer_1_uri = lsp::Uri::from_file_path(path!("/root/child/buffer-1")).unwrap();
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await,
            lsp::DidOpenTextDocumentParams {
                text_document: lsp::TextDocumentItem::new(
                    buffer_1_uri.clone(),
                    "plaintext".into(),
                    1,
                    "Hello world".into()
                ),
            }
        );

        // Ensure all previously-registered buffers are closed when signing out.
        lsp.set_request_handler::<request::SignOut, _, _>(|_, _| async {
            Ok(request::SignOutResult {})
        });
        copilot
            .update(cx, |copilot, cx| copilot.sign_out(cx))
            .await
            .unwrap();
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            lsp::DidCloseTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier::new(buffer_1_uri.clone()),
            }
        );
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            lsp::DidCloseTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier::new(buffer_2_uri.clone()),
            }
        );

        // Ensure all previously-registered buffers are re-opened when signing in.
        lsp.set_request_handler::<request::SignInInitiate, _, _>(|_, _| async {
            Ok(request::SignInInitiateResult::AlreadySignedIn {
                user: "user-1".into(),
            })
        });
        copilot
            .update(cx, |copilot, cx| copilot.sign_in(cx))
            .await
            .unwrap();

        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await,
            lsp::DidOpenTextDocumentParams {
                text_document: lsp::TextDocumentItem::new(
                    buffer_1_uri.clone(),
                    "plaintext".into(),
                    0,
                    "Hello world".into()
                ),
            }
        );
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await,
            lsp::DidOpenTextDocumentParams {
                text_document: lsp::TextDocumentItem::new(
                    buffer_2_uri.clone(),
                    "plaintext".into(),
                    0,
                    "Goodbye".into()
                ),
            }
        );
        // Dropping a buffer causes it to be closed on the LSP side as well.
        cx.update(|_| drop(buffer_2));
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            lsp::DidCloseTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier::new(buffer_2_uri),
            }
        );
    }

    struct File {
        abs_path: PathBuf,
        path: Arc<RelPath>,
    }

    impl language::File for File {
        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            Some(self)
        }

        fn disk_state(&self) -> language::DiskState {
            language::DiskState::Present {
                mtime: ::fs::MTime::from_seconds_and_nanos(100, 42),
            }
        }

        fn path(&self) -> &Arc<RelPath> {
            &self.path
        }

        fn path_style(&self, _: &App) -> PathStyle {
            PathStyle::local()
        }

        fn full_path(&self, _: &App) -> PathBuf {
            unimplemented!()
        }

        fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
            unimplemented!()
        }

        fn to_proto(&self, _: &App) -> rpc::proto::File {
            unimplemented!()
        }

        fn worktree_id(&self, _: &App) -> settings::WorktreeId {
            settings::WorktreeId::from_usize(0)
        }

        fn is_private(&self) -> bool {
            false
        }
    }

    impl language::LocalFile for File {
        fn abs_path(&self, _: &App) -> PathBuf {
            self.abs_path.clone()
        }

        fn load(&self, _: &App) -> Task<Result<String>> {
            unimplemented!()
        }

        fn load_bytes(&self, _cx: &App) -> Task<Result<Vec<u8>>> {
            unimplemented!()
        }
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}
