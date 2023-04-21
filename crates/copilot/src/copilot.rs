pub mod request;
mod sign_in;

use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use collections::HashMap;
use futures::{channel::oneshot, future::Shared, Future, FutureExt, TryFutureExt};
use gpui::{
    actions, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle,
};
use language::{
    point_from_lsp, point_to_lsp, Anchor, Bias, Buffer, BufferSnapshot, Language, PointUtf16,
    ToPointUtf16,
};
use log::{debug, error};
use lsp::{LanguageServer, LanguageServerId};
use node_runtime::NodeRuntime;
use request::{LogMessage, StatusNotification};
use settings::Settings;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    ffi::OsString,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    fs::remove_matching, github::latest_github_release, http::HttpClient, paths, ResultExt,
};

const COPILOT_AUTH_NAMESPACE: &'static str = "copilot_auth";
actions!(copilot_auth, [SignIn, SignOut]);

const COPILOT_NAMESPACE: &'static str = "copilot";
actions!(
    copilot,
    [Suggest, NextSuggestion, PreviousSuggestion, Reinstall]
);

pub fn init(http: Arc<dyn HttpClient>, node_runtime: Arc<NodeRuntime>, cx: &mut AppContext) {
    let copilot = cx.add_model({
        let node_runtime = node_runtime.clone();
        move |cx| Copilot::start(http, node_runtime, cx)
    });
    cx.set_global(copilot.clone());

    cx.observe(&copilot, |handle, cx| {
        let status = handle.read(cx).status();
        cx.update_global::<collections::CommandPaletteFilter, _, _>(
            move |filter, _cx| match status {
                Status::Disabled => {
                    filter.filtered_namespaces.insert(COPILOT_NAMESPACE);
                    filter.filtered_namespaces.insert(COPILOT_AUTH_NAMESPACE);
                }
                Status::Authorized => {
                    filter.filtered_namespaces.remove(COPILOT_NAMESPACE);
                    filter.filtered_namespaces.remove(COPILOT_AUTH_NAMESPACE);
                }
                _ => {
                    filter.filtered_namespaces.insert(COPILOT_NAMESPACE);
                    filter.filtered_namespaces.remove(COPILOT_AUTH_NAMESPACE);
                }
            },
        );
    })
    .detach();

    sign_in::init(cx);
    cx.add_global_action(|_: &SignIn, cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot
                .update(cx, |copilot, cx| copilot.sign_in(cx))
                .detach_and_log_err(cx);
        }
    });
    cx.add_global_action(|_: &SignOut, cx| {
        if let Some(copilot) = Copilot::global(cx) {
            copilot
                .update(cx, |copilot, cx| copilot.sign_out(cx))
                .detach_and_log_err(cx);
        }
    });

    cx.add_global_action(|_: &Reinstall, cx| {
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
    registered_buffers: HashMap<usize, RegisteredBuffer>,
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
}

struct RegisteredBuffer {
    id: usize,
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
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Copilot>,
    ) -> oneshot::Receiver<(i32, BufferSnapshot)> {
        let id = self.id;
        let (done_tx, done_rx) = oneshot::channel();

        if buffer.read(cx).version() == self.snapshot.version {
            let _ = done_tx.send((self.snapshot_version, self.snapshot.clone()));
        } else {
            let buffer = buffer.downgrade();
            let prev_pending_change =
                mem::replace(&mut self.pending_buffer_change, Task::ready(None));
            self.pending_buffer_change = cx.spawn_weak(|copilot, mut cx| async move {
                prev_pending_change.await;

                let old_version = copilot.upgrade(&cx)?.update(&mut cx, |copilot, _| {
                    let server = copilot.server.as_authenticated().log_err()?;
                    let buffer = server.registered_buffers.get_mut(&id)?;
                    Some(buffer.snapshot.version.clone())
                })?;
                let new_snapshot = buffer
                    .upgrade(&cx)?
                    .read_with(&cx, |buffer, _| buffer.snapshot());

                let content_changes = cx
                    .background()
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

                copilot.upgrade(&cx)?.update(&mut cx, |copilot, _| {
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
                })?;

                Some(())
            });
        }

        done_rx
    }
}

#[derive(Debug)]
pub struct Completion {
    uuid: String,
    pub range: Range<Anchor>,
    pub text: String,
}

pub struct Copilot {
    http: Arc<dyn HttpClient>,
    node_runtime: Arc<NodeRuntime>,
    server: CopilotServer,
    buffers: HashMap<usize, WeakModelHandle<Buffer>>,
}

impl Entity for Copilot {
    type Event = ();
}

impl Copilot {
    pub fn global(cx: &AppContext) -> Option<ModelHandle<Self>> {
        if cx.has_global::<ModelHandle<Self>>() {
            Some(cx.global::<ModelHandle<Self>>().clone())
        } else {
            None
        }
    }

    fn start(
        http: Arc<dyn HttpClient>,
        node_runtime: Arc<NodeRuntime>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.observe_global::<Settings, _>({
            let http = http.clone();
            let node_runtime = node_runtime.clone();
            move |this, cx| {
                if cx.global::<Settings>().features.copilot {
                    if matches!(this.server, CopilotServer::Disabled) {
                        let start_task = cx
                            .spawn({
                                let http = http.clone();
                                let node_runtime = node_runtime.clone();
                                move |this, cx| {
                                    Self::start_language_server(http, node_runtime, this, cx)
                                }
                            })
                            .shared();
                        this.server = CopilotServer::Starting { task: start_task };
                        cx.notify();
                    }
                } else {
                    this.server = CopilotServer::Disabled;
                    cx.notify();
                }
            }
        })
        .detach();

        if cx.global::<Settings>().features.copilot {
            let start_task = cx
                .spawn({
                    let http = http.clone();
                    let node_runtime = node_runtime.clone();
                    move |this, cx| async {
                        Self::start_language_server(http, node_runtime, this, cx).await
                    }
                })
                .shared();

            Self {
                http,
                node_runtime,
                server: CopilotServer::Starting { task: start_task },
                buffers: Default::default(),
            }
        } else {
            Self {
                http,
                node_runtime,
                server: CopilotServer::Disabled,
                buffers: Default::default(),
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(cx: &mut gpui::TestAppContext) -> (ModelHandle<Self>, lsp::FakeLanguageServer) {
        let (server, fake_server) =
            LanguageServer::fake("copilot".into(), Default::default(), cx.to_async());
        let http = util::http::FakeHttpClient::create(|_| async { unreachable!() });
        let this = cx.add_model(|cx| Self {
            http: http.clone(),
            node_runtime: NodeRuntime::new(http, cx.background().clone()),
            server: CopilotServer::Running(RunningCopilotServer {
                lsp: Arc::new(server),
                sign_in_status: SignInStatus::Authorized,
                registered_buffers: Default::default(),
            }),
            buffers: Default::default(),
        });
        (this, fake_server)
    }

    fn start_language_server(
        http: Arc<dyn HttpClient>,
        node_runtime: Arc<NodeRuntime>,
        this: ModelHandle<Self>,
        mut cx: AsyncAppContext,
    ) -> impl Future<Output = ()> {
        async move {
            let start_language_server = async {
                let server_path = get_copilot_lsp(http).await?;
                let node_path = node_runtime.binary_path().await?;
                let arguments: &[OsString] = &[server_path.into(), "--stdio".into()];
                let server = LanguageServer::new(
                    LanguageServerId(0),
                    &node_path,
                    arguments,
                    Path::new("/"),
                    None,
                    cx.clone(),
                )?;

                server
                    .on_notification::<LogMessage, _>(|params, _cx| {
                        match params.level {
                            // Copilot is pretty agressive about logging
                            0 => debug!("copilot: {}", params.message),
                            1 => debug!("copilot: {}", params.message),
                            _ => error!("copilot: {}", params.message),
                        }

                        debug!("copilot metadata: {}", params.metadata_str);
                        debug!("copilot extra: {:?}", params.extra);
                    })
                    .detach();

                server
                    .on_notification::<StatusNotification, _>(
                        |_, _| { /* Silence the notification */ },
                    )
                    .detach();

                let server = server.initialize(Default::default()).await?;

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
                        this.update_sign_in_status(status, cx);
                    }
                    Err(error) => {
                        this.server = CopilotServer::Error(error.to_string().into());
                        cx.notify()
                    }
                }
            })
        }
    }

    fn sign_in(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let CopilotServer::Running(server) = &mut self.server {
            let task = match &server.sign_in_status {
                SignInStatus::Authorized { .. } | SignInStatus::Unauthorized { .. } => {
                    Task::ready(Ok(())).shared()
                }
                SignInStatus::SigningIn { task, .. } => {
                    cx.notify();
                    task.clone()
                }
                SignInStatus::SignedOut => {
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
                                        Ok(request::SignInStatus::Ok { user })
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
                                        });
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
                            })
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

            cx.foreground()
                .spawn(task.map_err(|err| anyhow!("{:?}", err)))
        } else {
            // If we're downloading, wait until download is finished
            // If we're in a stuck state, display to the user
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    fn sign_out(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.update_sign_in_status(request::SignInStatus::NotSignedIn, cx);
        if let CopilotServer::Running(RunningCopilotServer { lsp: server, .. }) = &self.server {
            let server = server.clone();
            cx.background().spawn(async move {
                server
                    .request::<request::SignOut>(request::SignOutParams {})
                    .await?;
                anyhow::Ok(())
            })
        } else {
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    fn reinstall(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let start_task = cx
            .spawn({
                let http = self.http.clone();
                let node_runtime = self.node_runtime.clone();
                move |this, cx| async move {
                    clear_copilot_dir().await;
                    Self::start_language_server(http, node_runtime, this, cx).await
                }
            })
            .shared();

        self.server = CopilotServer::Starting {
            task: start_task.clone(),
        };

        cx.notify();

        cx.foreground().spawn(start_task)
    }

    pub fn register_buffer(&mut self, buffer: &ModelHandle<Buffer>, cx: &mut ModelContext<Self>) {
        let buffer_id = buffer.id();
        self.buffers.insert(buffer_id, buffer.downgrade());

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

            registered_buffers.entry(buffer.id()).or_insert_with(|| {
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
                    id: buffer_id,
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
                            this.buffers.remove(&buffer_id);
                            this.unregister_buffer(buffer_id);
                        }),
                    ],
                }
            });
        }
    }

    fn handle_buffer_event(
        &mut self,
        buffer: ModelHandle<Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Ok(server) = self.server.as_running() {
            if let Some(registered_buffer) = server.registered_buffers.get_mut(&buffer.id()) {
                match event {
                    language::Event::Edited => {
                        let _ = registered_buffer.report_changes(&buffer, cx);
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

    fn unregister_buffer(&mut self, buffer_id: usize) {
        if let Ok(server) = self.server.as_running() {
            if let Some(buffer) = server.registered_buffers.remove(&buffer_id) {
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
        buffer: &ModelHandle<Buffer>,
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
        buffer: &ModelHandle<Buffer>,
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
        cx.background().spawn(async move {
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
            Err(error) => return Task::ready(Err(error)),
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
        cx.background().spawn(async move {
            request.await?;
            Ok(())
        })
    }

    fn request_completions<R, T>(
        &mut self,
        buffer: &ModelHandle<Buffer>,
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
        let registered_buffer = server.registered_buffers.get_mut(&buffer.id()).unwrap();
        let snapshot = registered_buffer.report_changes(buffer, cx);
        let buffer = buffer.read(cx);
        let uri = registered_buffer.uri.clone();
        let settings = cx.global::<Settings>();
        let position = position.to_point_utf16(buffer);
        let language = buffer.language_at(position);
        let language_name = language.map(|language| language.name());
        let language_name = language_name.as_deref();
        let tab_size = settings.tab_size(language_name);
        let hard_tabs = settings.hard_tabs(language_name);
        let relative_path = buffer
            .file()
            .map(|file| file.path().to_path_buf())
            .unwrap_or_default();

        cx.foreground().spawn(async move {
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
        self.buffers.retain(|_, buffer| buffer.is_upgradable(cx));

        if let Ok(server) = self.server.as_running() {
            match lsp_status {
                request::SignInStatus::Ok { .. }
                | request::SignInStatus::MaybeOk { .. }
                | request::SignInStatus::AlreadySignedIn { .. } => {
                    server.sign_in_status = SignInStatus::Authorized;
                    for buffer in self.buffers.values().cloned().collect::<Vec<_>>() {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            self.register_buffer(&buffer, cx);
                        }
                    }
                }
                request::SignInStatus::NotAuthorized { .. } => {
                    server.sign_in_status = SignInStatus::Unauthorized;
                    for buffer_id in self.buffers.keys().copied().collect::<Vec<_>>() {
                        self.unregister_buffer(buffer_id);
                    }
                }
                request::SignInStatus::NotSignedIn => {
                    server.sign_in_status = SignInStatus::SignedOut;
                    for buffer_id in self.buffers.keys().copied().collect::<Vec<_>>() {
                        self.unregister_buffer(buffer_id);
                    }
                }
            }

            cx.notify();
        }
    }
}

fn id_for_language(language: Option<&Arc<Language>>) -> String {
    let language_name = language.map(|language| language.name());
    match language_name.as_deref() {
        Some("Plain Text") => "plaintext".to_string(),
        Some(language_name) => language_name.to_lowercase(),
        None => "plaintext".to_string(),
    }
}

fn uri_for_buffer(buffer: &ModelHandle<Buffer>, cx: &AppContext) -> lsp::Url {
    if let Some(file) = buffer.read(cx).file().and_then(|file| file.as_local()) {
        lsp::Url::from_file_path(file.abs_path(cx)).unwrap()
    } else {
        format!("buffer://{}", buffer.id()).parse().unwrap()
    }
}

async fn clear_copilot_dir() {
    remove_matching(&paths::COPILOT_DIR, |_| true).await
}

async fn get_copilot_lsp(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
    const SERVER_PATH: &'static str = "dist/agent.js";

    ///Check for the latest copilot language server and download it if we haven't already
    async fn fetch_latest(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
        let release = latest_github_release("zed-industries/copilot", http.clone()).await?;

        let version_dir = &*paths::COPILOT_DIR.join(format!("copilot-{}", release.name));

        fs::create_dir_all(version_dir).await?;
        let server_path = version_dir.join(SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            // Copilot LSP looks for this dist dir specifcially, so lets add it in.
            let dist_dir = version_dir.join("dist");
            fs::create_dir_all(dist_dir.as_path()).await?;

            let url = &release
                .assets
                .get(0)
                .context("Github release for copilot contained no assets")?
                .browser_download_url;

            let mut response = http
                .get(&url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading copilot release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(dist_dir).await?;

            remove_matching(&paths::COPILOT_DIR, |entry| entry != version_dir).await;
        }

        Ok(server_path)
    }

    match fetch_latest(http).await {
        ok @ Result::Ok(..) => ok,
        e @ Err(..) => {
            e.log_err();
            // Fetch a cached binary, if it exists
            (|| async move {
                let mut last_version_dir = None;
                let mut entries = fs::read_dir(paths::COPILOT_DIR.as_path()).await?;
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
            })()
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{executor::Deterministic, TestAppContext};

    #[gpui::test(iterations = 10)]
    async fn test_buffer_management(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        deterministic.forbid_parking();
        let (copilot, mut lsp) = Copilot::fake(cx);

        let buffer_1 = cx.add_model(|cx| Buffer::new(0, "Hello", cx));
        let buffer_1_uri: lsp::Url = format!("buffer://{}", buffer_1.id()).parse().unwrap();
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

        let buffer_2 = cx.add_model(|cx| Buffer::new(0, "Goodbye", cx));
        let buffer_2_uri: lsp::Url = format!("buffer://{}", buffer_2.id()).parse().unwrap();
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
        buffer_1
            .update(cx, |buffer, cx| {
                buffer.file_updated(
                    Arc::new(File {
                        abs_path: "/root/child/buffer-1".into(),
                        path: Path::new("child/buffer-1").into(),
                    }),
                    cx,
                )
            })
            .await;
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
                text_document: lsp::TextDocumentIdentifier::new(buffer_2_uri.clone()),
            }
        );
        assert_eq!(
            lsp.receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            lsp::DidCloseTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier::new(buffer_1_uri.clone()),
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
                    buffer_2_uri.clone(),
                    "plaintext".into(),
                    0,
                    "Goodbye".into()
                ),
            }
        );
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

        fn mtime(&self) -> std::time::SystemTime {
            todo!()
        }

        fn path(&self) -> &Arc<Path> {
            &self.path
        }

        fn full_path(&self, _: &AppContext) -> PathBuf {
            todo!()
        }

        fn file_name<'a>(&'a self, _: &'a AppContext) -> &'a std::ffi::OsStr {
            todo!()
        }

        fn is_deleted(&self) -> bool {
            todo!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            todo!()
        }

        fn to_proto(&self) -> rpc::proto::File {
            todo!()
        }
    }

    impl language::LocalFile for File {
        fn abs_path(&self, _: &AppContext) -> PathBuf {
            self.abs_path.clone()
        }

        fn load(&self, _: &AppContext) -> Task<Result<String>> {
            todo!()
        }

        fn buffer_reloaded(
            &self,
            _: u64,
            _: &clock::Global,
            _: language::RopeFingerprint,
            _: ::fs::LineEnding,
            _: std::time::SystemTime,
            _: &mut AppContext,
        ) {
            todo!()
        }
    }
}
