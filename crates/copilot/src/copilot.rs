pub mod copilot_chat;
mod copilot_completion_provider;
pub mod request;
mod sign_in;

use ::fs::Fs;
use anyhow::{anyhow, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use collections::{HashMap, HashSet};
use command_palette_hooks::CommandPaletteFilter;
use futures::{channel::oneshot, future::Shared, Future, FutureExt, TryFutureExt};
use gpui::{
    actions, AppContext, AsyncAppContext, Context, Entity, EntityId, EventEmitter, Global, Model,
    ModelContext, Task, WeakModel,
};
use http_client::github::latest_github_release;
use http_client::HttpClient;
use language::{
    language_settings::{all_language_settings, language_settings, InlineCompletionProvider},
    point_from_lsp, point_to_lsp, Anchor, Bias, Buffer, BufferSnapshot, Language, PointUtf16,
    ToPointUtf16,
};
use lsp::{LanguageServer, LanguageServerBinary, LanguageServerId};
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use request::StatusNotification;
use settings::SettingsStore;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::TypeId,
    env,
    ffi::OsString,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{fs::remove_matching, maybe, ResultExt};

pub use copilot_completion_provider::CopilotCompletionProvider;
pub use sign_in::CopilotCodeVerification;

actions!(
    copilot,
    [
        Suggest,
        NextSuggestion,
        PreviousSuggestion,
        Reinstall,
        SignIn,
        SignOut
    ]
);

pub fn init(
    new_server_id: LanguageServerId,
    fs: Arc<dyn Fs>,
    http: Arc<dyn HttpClient>,
    node_runtime: Arc<dyn NodeRuntime>,
    cx: &mut AppContext,
) {
    copilot_chat::init(fs, http.clone(), cx);

    let copilot = cx.new_model({
        let node_runtime = node_runtime.clone();
        move |cx| Copilot::start(new_server_id, http, node_runtime, cx)
    });
    Copilot::set_global(copilot.clone(), cx);
    cx.observe(&copilot, |handle, cx| {
        let copilot_action_types = [
            TypeId::of::<Suggest>(),
            TypeId::of::<NextSuggestion>(),
            TypeId::of::<PreviousSuggestion>(),
            TypeId::of::<Reinstall>(),
        ];
        let copilot_auth_action_types = [TypeId::of::<SignOut>()];
        let copilot_no_auth_action_types = [TypeId::of::<SignIn>()];
        let status = handle.read(cx).status();
        let filter = CommandPaletteFilter::global_mut(cx);

        match status {
            Status::Disabled => {
                filter.hide_action_types(&copilot_action_types);
                filter.hide_action_types(&copilot_auth_action_types);
                filter.hide_action_types(&copilot_no_auth_action_types);
            }
            Status::Authorized => {
                filter.hide_action_types(&copilot_no_auth_action_types);
                filter.show_action_types(
                    copilot_action_types
                        .iter()
                        .chain(&copilot_auth_action_types),
                );
            }
            _ => {
                filter.hide_action_types(&copilot_action_types);
                filter.hide_action_types(&copilot_auth_action_types);
                filter.show_action_types(copilot_no_auth_action_types.iter());
            }
        }
    })
    .detach();

    cx.on_action(|_: &SignIn, cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot
                .update(cx, |copilot, cx| copilot.sign_in(cx))
                .detach_and_log_err(cx);
        }
    });
    cx.on_action(|_: &SignOut, cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot
                .update(cx, |copilot, cx| copilot.sign_out(cx))
                .detach_and_log_err(cx);
        }
    });
    cx.on_action(|_: &Reinstall, cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot
                .update(cx, |copilot, cx| copilot.reinstall(cx))
                .detach();
        }
    });
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
        if matches!(server.sign_in_status, SignInStatus::Authorized { .. }) {
            Ok(server)
        } else {
            Err(anyhow!("must sign in before using copilot"))
        }
    }

    fn as_running(&mut self) -> Result<&mut RunningCopilotServer> {
        match self {
            CopilotServer::Starting { .. } => Err(anyhow!("copilot is still starting")),
            CopilotServer::Disabled => Err(anyhow!("copilot is disabled")),
            CopilotServer::Error(error) => Err(anyhow!(
                "copilot was not started because of an error: {}",
                error
            )),
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
    SignedOut,
}

#[derive(Debug, Clone)]
pub enum Status {
    Starting {
        task: Shared<Task<()>>,
    },
    Error(Arc<str>),
    Disabled,
    SignedOut,
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

    pub fn is_disabled(&self) -> bool {
        matches!(self, Status::Disabled)
    }
}

struct RegisteredBuffer {
    uri: lsp::Url,
    language_id: String,
    snapshot: BufferSnapshot,
    snapshot_version: i32,
    _subscriptions: [gpui::Subscription; 2],
    pending_buffer_change: Task<Option<()>>,
}

impl RegisteredBuffer {
    fn report_changes(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Copilot>,
    ) -> oneshot::Receiver<(i32, BufferSnapshot)> {
        let (done_tx, done_rx) = oneshot::channel();

        if buffer.read(cx).version() == self.snapshot.version {
            let _ = done_tx.send((self.snapshot_version, self.snapshot.clone()));
        } else {
            let buffer = buffer.downgrade();
            let id = buffer.entity_id();
            let prev_pending_change =
                mem::replace(&mut self.pending_buffer_change, Task::ready(None));
            self.pending_buffer_change = cx.spawn(move |copilot, mut cx| async move {
                prev_pending_change.await;

                let old_version = copilot
                    .update(&mut cx, |copilot, _| {
                        let server = copilot.server.as_authenticated().log_err()?;
                        let buffer = server.registered_buffers.get_mut(&id)?;
                        Some(buffer.snapshot.version.clone())
                    })
                    .ok()??;
                let new_snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot()).ok()?;

                let content_changes = cx
                    .background_executor()
                    .spawn({
                        let new_snapshot = new_snapshot.clone();
                        async move {
                            new_snapshot
                                .edits_since::<(PointUtf16, usize)>(&old_version)
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
                    .update(&mut cx, |copilot, _| {
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
                                .log_err();
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
    http: Arc<dyn HttpClient>,
    node_runtime: Arc<dyn NodeRuntime>,
    server: CopilotServer,
    buffers: HashSet<WeakModel<Buffer>>,
    server_id: LanguageServerId,
    _subscription: gpui::Subscription,
}

pub enum Event {
    CopilotLanguageServerStarted,
    CopilotAuthSignedIn,
    CopilotAuthSignedOut,
}

impl EventEmitter<Event> for Copilot {}

struct GlobalCopilot(Model<Copilot>);

impl Global for GlobalCopilot {}

impl Copilot {
    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalCopilot>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(copilot: Model<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalCopilot(copilot));
    }

    fn start(
        new_server_id: LanguageServerId,
        http: Arc<dyn HttpClient>,
        node_runtime: Arc<dyn NodeRuntime>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut this = Self {
            server_id: new_server_id,
            http,
            node_runtime,
            server: CopilotServer::Disabled,
            buffers: Default::default(),
            _subscription: cx.on_app_quit(Self::shutdown_language_server),
        };
        this.enable_or_disable_copilot(cx);
        cx.observe_global::<SettingsStore>(move |this, cx| this.enable_or_disable_copilot(cx))
            .detach();
        this
    }

    fn shutdown_language_server(
        &mut self,
        _cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = ()> {
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

    fn enable_or_disable_copilot(&mut self, cx: &mut ModelContext<Self>) {
        let server_id = self.server_id;
        let http = self.http.clone();
        let node_runtime = self.node_runtime.clone();
        if all_language_settings(None, cx).inline_completions.provider
            == InlineCompletionProvider::Copilot
        {
            if matches!(self.server, CopilotServer::Disabled) {
                let start_task = cx
                    .spawn(move |this, cx| {
                        Self::start_language_server(server_id, http, node_runtime, this, cx)
                    })
                    .shared();
                self.server = CopilotServer::Starting { task: start_task };
                cx.notify();
            }
        } else {
            self.server = CopilotServer::Disabled;
            cx.notify();
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(cx: &mut gpui::TestAppContext) -> (Model<Self>, lsp::FakeLanguageServer) {
        use lsp::FakeLanguageServer;
        use node_runtime::FakeNodeRuntime;

        let (server, fake_server) = FakeLanguageServer::new(
            LanguageServerId(0),
            LanguageServerBinary {
                path: "path/to/copilot".into(),
                arguments: vec![],
                env: None,
            },
            "copilot".into(),
            Default::default(),
            cx.to_async(),
        );
        let http = http_client::FakeHttpClient::create(|_| async { unreachable!() });
        let node_runtime = FakeNodeRuntime::new();
        let this = cx.new_model(|cx| Self {
            server_id: LanguageServerId(0),
            http: http.clone(),
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

    fn start_language_server(
        new_server_id: LanguageServerId,
        http: Arc<dyn HttpClient>,
        node_runtime: Arc<dyn NodeRuntime>,
        this: WeakModel<Self>,
        mut cx: AsyncAppContext,
    ) -> impl Future<Output = ()> {
        async move {
            let start_language_server = async {
                let server_path = get_copilot_lsp(http).await?;
                let node_path = node_runtime.binary_path().await?;
                let arguments: Vec<OsString> = vec![server_path.into(), "--stdio".into()];
                let binary = LanguageServerBinary {
                    path: node_path,
                    arguments,
                    // TODO: We could set HTTP_PROXY etc here and fix the copilot issue.
                    env: None,
                };

                let root_path = if cfg!(target_os = "windows") {
                    Path::new("C:/")
                } else {
                    Path::new("/")
                };

                let server = LanguageServer::new(
                    Arc::new(Mutex::new(None)),
                    new_server_id,
                    binary,
                    root_path,
                    None,
                    cx.clone(),
                )?;

                server
                    .on_notification::<StatusNotification, _>(
                        |_, _| { /* Silence the notification */ },
                    )
                    .detach();
                let server = cx.update(|cx| server.initialize(None, cx))?.await?;

                let status = server
                    .request::<request::CheckStatus>(request::CheckStatusParams {
                        local_checks_only: false,
                    })
                    .await?;

                server
                    .request::<request::SetEditorInfo>(request::SetEditorInfoParams {
                        editor_info: request::EditorInfo {
                            name: "zed".into(),
                            version: env!("CARGO_PKG_VERSION").into(),
                        },
                        editor_plugin_info: request::EditorPluginInfo {
                            name: "zed-copilot".into(),
                            version: "0.0.1".into(),
                        },
                    })
                    .await?;

                anyhow::Ok((server, status))
            };

            let server = start_language_server.await;
            this.update(&mut cx, |this, cx| {
                cx.notify();
                match server {
                    Ok((server, status)) => {
                        this.server = CopilotServer::Running(RunningCopilotServer {
                            lsp: server,
                            sign_in_status: SignInStatus::SignedOut,
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
    }

    pub fn sign_in(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let CopilotServer::Running(server) = &mut self.server {
            let task = match &server.sign_in_status {
                SignInStatus::Authorized { .. } => Task::ready(Ok(())).shared(),
                SignInStatus::SigningIn { task, .. } => {
                    cx.notify();
                    task.clone()
                }
                SignInStatus::SignedOut | SignInStatus::Unauthorized { .. } => {
                    let lsp = server.lsp.clone();
                    let task = cx
                        .spawn(|this, mut cx| async move {
                            let sign_in = async {
                                let sign_in = lsp
                                    .request::<request::SignInInitiate>(
                                        request::SignInInitiateParams {},
                                    )
                                    .await?;
                                match sign_in {
                                    request::SignInInitiateResult::AlreadySignedIn { user } => {
                                        Ok(request::SignInStatus::Ok { user: Some(user) })
                                    }
                                    request::SignInInitiateResult::PromptUserDeviceFlow(flow) => {
                                        this.update(&mut cx, |this, cx| {
                                            if let CopilotServer::Running(RunningCopilotServer {
                                                sign_in_status: status,
                                                ..
                                            }) = &mut this.server
                                            {
                                                if let SignInStatus::SigningIn {
                                                    prompt: prompt_flow,
                                                    ..
                                                } = status
                                                {
                                                    *prompt_flow = Some(flow.clone());
                                                    cx.notify();
                                                }
                                            }
                                        })?;
                                        let response = lsp
                                            .request::<request::SignInConfirm>(
                                                request::SignInConfirmParams {
                                                    user_code: flow.user_code,
                                                },
                                            )
                                            .await?;
                                        Ok(response)
                                    }
                                }
                            };

                            let sign_in = sign_in.await;
                            this.update(&mut cx, |this, cx| match sign_in {
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

            cx.background_executor()
                .spawn(task.map_err(|err| anyhow!("{:?}", err)))
        } else {
            // If we're downloading, wait until download is finished
            // If we're in a stuck state, display to the user
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    pub fn sign_out(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.update_sign_in_status(request::SignInStatus::NotSignedIn, cx);
        if let CopilotServer::Running(RunningCopilotServer { lsp: server, .. }) = &self.server {
            let server = server.clone();
            cx.background_executor().spawn(async move {
                server
                    .request::<request::SignOut>(request::SignOutParams {})
                    .await?;
                anyhow::Ok(())
            })
        } else {
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    pub fn reinstall(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let start_task = cx
            .spawn({
                let http = self.http.clone();
                let node_runtime = self.node_runtime.clone();
                let server_id = self.server_id;
                move |this, cx| async move {
                    clear_copilot_dir().await;
                    Self::start_language_server(server_id, http, node_runtime, this, cx).await
                }
            })
            .shared();

        self.server = CopilotServer::Starting {
            task: start_task.clone(),
        };

        cx.notify();

        cx.background_executor().spawn(start_task)
    }

    pub fn language_server(&self) -> Option<&Arc<LanguageServer>> {
        if let CopilotServer::Running(server) = &self.server {
            Some(&server.lsp)
        } else {
            None
        }
    }

    pub fn register_buffer(&mut self, buffer: &Model<Buffer>, cx: &mut ModelContext<Self>) {
        let weak_buffer = buffer.downgrade();
        self.buffers.insert(weak_buffer.clone());

        if let CopilotServer::Running(RunningCopilotServer {
            lsp: server,
            sign_in_status: status,
            registered_buffers,
            ..
        }) = &mut self.server
        {
            if !matches!(status, SignInStatus::Authorized { .. }) {
                return;
            }

            registered_buffers
                .entry(buffer.entity_id())
                .or_insert_with(|| {
                    let uri: lsp::Url = uri_for_buffer(buffer, cx);
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
                        .log_err();

                    RegisteredBuffer {
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
                    }
                });
        }
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Ok(server) = self.server.as_running() {
            if let Some(registered_buffer) = server.registered_buffers.get_mut(&buffer.entity_id())
            {
                match event {
                    language::Event::Edited => {
                        drop(registered_buffer.report_changes(&buffer, cx));
                    }
                    language::Event::Saved => {
                        server
                            .lsp
                            .notify::<lsp::notification::DidSaveTextDocument>(
                                lsp::DidSaveTextDocumentParams {
                                    text_document: lsp::TextDocumentIdentifier::new(
                                        registered_buffer.uri.clone(),
                                    ),
                                    text: None,
                                },
                            )?;
                    }
                    language::Event::FileHandleChanged | language::Event::LanguageChanged => {
                        let new_language_id = id_for_language(buffer.read(cx).language());
                        let new_uri = uri_for_buffer(&buffer, cx);
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
                                )?;
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
                                )?;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn unregister_buffer(&mut self, buffer: &WeakModel<Buffer>) {
        if let Ok(server) = self.server.as_running() {
            if let Some(buffer) = server.registered_buffers.remove(&buffer.entity_id()) {
                server
                    .lsp
                    .notify::<lsp::notification::DidCloseTextDocument>(
                        lsp::DidCloseTextDocumentParams {
                            text_document: lsp::TextDocumentIdentifier::new(buffer.uri),
                        },
                    )
                    .log_err();
            }
        }
    }

    pub fn completions<T>(
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        T: ToPointUtf16,
    {
        self.request_completions::<request::GetCompletions, _>(buffer, position, cx)
    }

    pub fn completions_cycling<T>(
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        T: ToPointUtf16,
    {
        self.request_completions::<request::GetCompletionsCycling, _>(buffer, position, cx)
    }

    pub fn accept_completion(
        &mut self,
        completion: &Completion,
        cx: &mut ModelContext<Self>,
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
        cx.background_executor().spawn(async move {
            request.await?;
            Ok(())
        })
    }

    pub fn discard_completions(
        &mut self,
        completions: &[Completion],
        cx: &mut ModelContext<Self>,
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
        cx.background_executor().spawn(async move {
            request.await?;
            Ok(())
        })
    }

    fn request_completions<R, T>(
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
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
        let settings = language_settings(buffer.language_at(position).as_ref(), buffer.file(), cx);
        let tab_size = settings.tab_size;
        let hard_tabs = settings.hard_tabs;
        let relative_path = buffer
            .file()
            .map(|file| file.path().to_path_buf())
            .unwrap_or_default();

        cx.background_executor().spawn(async move {
            let (version, snapshot) = snapshot.await?;
            let result = lsp
                .request::<R>(request::GetCompletionsParams {
                    doc: request::GetCompletionsDocument {
                        uri,
                        tab_size: tab_size.into(),
                        indent_size: 1,
                        insert_spaces: !hard_tabs,
                        relative_path: relative_path.to_string_lossy().into(),
                        position: point_to_lsp(position),
                        version: version.try_into().unwrap(),
                    },
                })
                .await?;
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
                    SignInStatus::Authorized { .. } => Status::Authorized,
                    SignInStatus::Unauthorized { .. } => Status::Unauthorized,
                    SignInStatus::SigningIn { prompt, .. } => Status::SigningIn {
                        prompt: prompt.clone(),
                    },
                    SignInStatus::SignedOut => Status::SignedOut,
                }
            }
        }
    }

    fn update_sign_in_status(
        &mut self,
        lsp_status: request::SignInStatus,
        cx: &mut ModelContext<Self>,
    ) {
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
                    server.sign_in_status = SignInStatus::SignedOut;
                    cx.emit(Event::CopilotAuthSignedOut);
                    for buffer in self.buffers.iter().cloned().collect::<Vec<_>>() {
                        self.unregister_buffer(&buffer);
                    }
                }
            }

            cx.notify();
        }
    }
}

fn id_for_language(language: Option<&Arc<Language>>) -> String {
    language
        .map(|language| language.lsp_id())
        .unwrap_or_else(|| "plaintext".to_string())
}

fn uri_for_buffer(buffer: &Model<Buffer>, cx: &AppContext) -> lsp::Url {
    if let Some(file) = buffer.read(cx).file().and_then(|file| file.as_local()) {
        lsp::Url::from_file_path(file.abs_path(cx)).unwrap()
    } else {
        format!("buffer://{}", buffer.entity_id()).parse().unwrap()
    }
}

async fn clear_copilot_dir() {
    remove_matching(paths::copilot_dir(), |_| true).await
}

async fn get_copilot_lsp(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
    const SERVER_PATH: &str = "dist/agent.js";

    ///Check for the latest copilot language server and download it if we haven't already
    async fn fetch_latest(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
        let release =
            latest_github_release("zed-industries/copilot", true, false, http.clone()).await?;

        let version_dir = &paths::copilot_dir().join(format!("copilot-{}", release.tag_name));

        fs::create_dir_all(version_dir).await?;
        let server_path = version_dir.join(SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            // Copilot LSP looks for this dist dir specifically, so lets add it in.
            let dist_dir = version_dir.join("dist");
            fs::create_dir_all(dist_dir.as_path()).await?;

            let url = &release
                .assets
                .get(0)
                .context("Github release for copilot contained no assets")?
                .browser_download_url;

            let mut response = http
                .get(url, Default::default(), true)
                .await
                .context("error downloading copilot release")?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(dist_dir).await?;

            remove_matching(paths::copilot_dir(), |entry| entry != version_dir).await;
        }

        Ok(server_path)
    }

    match fetch_latest(http).await {
        ok @ Result::Ok(..) => ok,
        e @ Err(..) => {
            e.log_err();
            // Fetch a cached binary, if it exists
            maybe!(async {
                let mut last_version_dir = None;
                let mut entries = fs::read_dir(paths::copilot_dir()).await?;
                while let Some(entry) = entries.next().await {
                    let entry = entry?;
                    if entry.file_type().await?.is_dir() {
                        last_version_dir = Some(entry.path());
                    }
                }
                let last_version_dir =
                    last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
                let server_path = last_version_dir.join(SERVER_PATH);
                if server_path.exists() {
                    Ok(server_path)
                } else {
                    Err(anyhow!(
                        "missing executable in directory {:?}",
                        last_version_dir
                    ))
                }
            })
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test(iterations = 10)]
    async fn test_buffer_management(cx: &mut TestAppContext) {
        let (copilot, mut lsp) = Copilot::fake(cx);

        let buffer_1 = cx.new_model(|cx| Buffer::local("Hello", cx));
        let buffer_1_uri: lsp::Url = format!("buffer://{}", buffer_1.entity_id().as_u64())
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

        let buffer_2 = cx.new_model(|cx| Buffer::local("Goodbye", cx));
        let buffer_2_uri: lsp::Url = format!("buffer://{}", buffer_2.entity_id().as_u64())
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
                    abs_path: "/root/child/buffer-1".into(),
                    path: Path::new("child/buffer-1").into(),
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
        let buffer_1_uri = lsp::Url::from_file_path("/root/child/buffer-1").unwrap();
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
        lsp.handle_request::<request::SignOut, _, _>(|_, _| async {
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
        lsp.handle_request::<request::SignInInitiate, _, _>(|_, _| async {
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
        path: Arc<Path>,
    }

    impl language::File for File {
        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            Some(self)
        }

        fn mtime(&self) -> Option<std::time::SystemTime> {
            unimplemented!()
        }

        fn path(&self) -> &Arc<Path> {
            &self.path
        }

        fn full_path(&self, _: &AppContext) -> PathBuf {
            unimplemented!()
        }

        fn file_name<'a>(&'a self, _: &'a AppContext) -> &'a std::ffi::OsStr {
            unimplemented!()
        }

        fn is_deleted(&self) -> bool {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            unimplemented!()
        }

        fn to_proto(&self, _: &AppContext) -> rpc::proto::File {
            unimplemented!()
        }

        fn worktree_id(&self) -> usize {
            0
        }

        fn is_private(&self) -> bool {
            false
        }
    }

    impl language::LocalFile for File {
        fn abs_path(&self, _: &AppContext) -> PathBuf {
            self.abs_path.clone()
        }

        fn load(&self, _: &AppContext) -> Task<Result<String>> {
            unimplemented!()
        }
    }
}
