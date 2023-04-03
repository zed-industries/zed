mod request;
mod sign_in;

use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use client::Client;
use collections::HashMap;
use futures::{future::Shared, Future, FutureExt, TryFutureExt};
use gpui::{
    actions, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task,
};
use language::{point_from_lsp, point_to_lsp, Anchor, Bias, Buffer, Language, ToPointUtf16};
use log::{debug, error};
use lsp::LanguageServer;
use node_runtime::NodeRuntime;
use request::{LogMessage, StatusNotification};
use settings::Settings;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    ffi::OsString,
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
    [NextSuggestion, PreviousSuggestion, Toggle, Reinstall]
);

pub fn init(client: Arc<Client>, node_runtime: Arc<NodeRuntime>, cx: &mut MutableAppContext) {
    let copilot = cx.add_model(|cx| Copilot::start(client.http_client(), node_runtime, cx));
    cx.set_global(copilot.clone());
    cx.add_global_action(|_: &SignIn, cx| {
        let copilot = Copilot::global(cx).unwrap();
        copilot
            .update(cx, |copilot, cx| copilot.sign_in(cx))
            .detach_and_log_err(cx);
    });
    cx.add_global_action(|_: &SignOut, cx| {
        let copilot = Copilot::global(cx).unwrap();
        copilot
            .update(cx, |copilot, cx| copilot.sign_out(cx))
            .detach_and_log_err(cx);
    });

    cx.add_global_action(|_: &Reinstall, cx| {
        let copilot = Copilot::global(cx).unwrap();
        copilot
            .update(cx, |copilot, cx| copilot.reinstall(cx))
            .detach();
    });

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
}

enum CopilotServer {
    Disabled,
    Starting {
        task: Shared<Task<()>>,
    },
    Error(Arc<str>),
    Started {
        server: Arc<LanguageServer>,
        status: SignInStatus,
        subscriptions_by_buffer_id: HashMap<usize, gpui::Subscription>,
    },
}

#[derive(Clone, Debug)]
enum SignInStatus {
    Authorized {
        _user: String,
    },
    Unauthorized {
        _user: String,
    },
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

#[derive(Debug, PartialEq, Eq)]
pub struct Completion {
    pub range: Range<Anchor>,
    pub text: String,
}

pub struct Copilot {
    http: Arc<dyn HttpClient>,
    node_runtime: Arc<NodeRuntime>,
    server: CopilotServer,
}

impl Entity for Copilot {
    type Event = ();
}

impl Copilot {
    pub fn starting_task(&self) -> Option<Shared<Task<()>>> {
        match self.server {
            CopilotServer::Starting { ref task } => Some(task.clone()),
            _ => None,
        }
    }

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
                if cx.global::<Settings>().enable_copilot_integration {
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

        if cx.global::<Settings>().enable_copilot_integration {
            let start_task = cx
                .spawn({
                    let http = http.clone();
                    let node_runtime = node_runtime.clone();
                    move |this, cx| Self::start_language_server(http, node_runtime, this, cx)
                })
                .shared();

            Self {
                http,
                node_runtime,
                server: CopilotServer::Starting { task: start_task },
            }
        } else {
            Self {
                http,
                node_runtime,
                server: CopilotServer::Disabled,
            }
        }
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
                    0,
                    &node_path,
                    arguments,
                    Path::new("/"),
                    None,
                    cx.clone(),
                )?;

                let server = server.initialize(Default::default()).await?;
                let status = server
                    .request::<request::CheckStatus>(request::CheckStatusParams {
                        local_checks_only: false,
                    })
                    .await?;

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

                anyhow::Ok((server, status))
            };

            let server = start_language_server.await;
            this.update(&mut cx, |this, cx| {
                cx.notify();
                match server {
                    Ok((server, status)) => {
                        this.server = CopilotServer::Started {
                            server,
                            status: SignInStatus::SignedOut,
                            subscriptions_by_buffer_id: Default::default(),
                        };
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
        if let CopilotServer::Started { server, status, .. } = &mut self.server {
            let task = match status {
                SignInStatus::Authorized { .. } | SignInStatus::Unauthorized { .. } => {
                    Task::ready(Ok(())).shared()
                }
                SignInStatus::SigningIn { task, .. } => {
                    cx.notify();
                    task.clone()
                }
                SignInStatus::SignedOut => {
                    let server = server.clone();
                    let task = cx
                        .spawn(|this, mut cx| async move {
                            let sign_in = async {
                                let sign_in = server
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
                                            if let CopilotServer::Started { status, .. } =
                                                &mut this.server
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
                                        let response = server
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
                    *status = SignInStatus::SigningIn {
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
        if let CopilotServer::Started { server, status, .. } = &mut self.server {
            *status = SignInStatus::SignedOut;
            cx.notify();

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

    fn request_completions<R, T>(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        R: lsp::request::Request<
            Params = request::GetCompletionsParams,
            Result = request::GetCompletionsResult,
        >,
        T: ToPointUtf16,
    {
        let buffer_id = buffer.id();
        let uri: lsp::Url = format!("buffer://{}", buffer_id).parse().unwrap();
        let snapshot = buffer.read(cx).snapshot();
        let server = match &mut self.server {
            CopilotServer::Starting { .. } => {
                return Task::ready(Err(anyhow!("copilot is still starting")))
            }
            CopilotServer::Disabled => return Task::ready(Err(anyhow!("copilot is disabled"))),
            CopilotServer::Error(error) => {
                return Task::ready(Err(anyhow!(
                    "copilot was not started because of an error: {}",
                    error
                )))
            }
            CopilotServer::Started {
                server,
                status,
                subscriptions_by_buffer_id,
            } => {
                if matches!(status, SignInStatus::Authorized { .. }) {
                    subscriptions_by_buffer_id
                        .entry(buffer_id)
                        .or_insert_with(|| {
                            server
                                .notify::<lsp::notification::DidOpenTextDocument>(
                                    lsp::DidOpenTextDocumentParams {
                                        text_document: lsp::TextDocumentItem {
                                            uri: uri.clone(),
                                            language_id: id_for_language(
                                                buffer.read(cx).language(),
                                            ),
                                            version: 0,
                                            text: snapshot.text(),
                                        },
                                    },
                                )
                                .log_err();

                            let uri = uri.clone();
                            cx.observe_release(buffer, move |this, _, _| {
                                if let CopilotServer::Started {
                                    server,
                                    subscriptions_by_buffer_id,
                                    ..
                                } = &mut this.server
                                {
                                    server
                                        .notify::<lsp::notification::DidCloseTextDocument>(
                                            lsp::DidCloseTextDocumentParams {
                                                text_document: lsp::TextDocumentIdentifier::new(
                                                    uri.clone(),
                                                ),
                                            },
                                        )
                                        .log_err();
                                    subscriptions_by_buffer_id.remove(&buffer_id);
                                }
                            })
                        });

                    server.clone()
                } else {
                    return Task::ready(Err(anyhow!("must sign in before using copilot")));
                }
            }
        };

        let settings = cx.global::<Settings>();
        let position = position.to_point_utf16(&snapshot);
        let language = snapshot.language_at(position);
        let language_name = language.map(|language| language.name());
        let language_name = language_name.as_deref();
        let tab_size = settings.tab_size(language_name);
        let hard_tabs = settings.hard_tabs(language_name);
        let language_id = id_for_language(language);

        let path;
        let relative_path;
        if let Some(file) = snapshot.file() {
            if let Some(file) = file.as_local() {
                path = file.abs_path(cx);
            } else {
                path = file.full_path(cx);
            }
            relative_path = file.path().to_path_buf();
        } else {
            path = PathBuf::new();
            relative_path = PathBuf::new();
        }

        cx.background().spawn(async move {
            let result = server
                .request::<R>(request::GetCompletionsParams {
                    doc: request::GetCompletionsDocument {
                        source: snapshot.text(),
                        tab_size: tab_size.into(),
                        indent_size: 1,
                        insert_spaces: !hard_tabs,
                        uri,
                        path: path.to_string_lossy().into(),
                        relative_path: relative_path.to_string_lossy().into(),
                        language_id,
                        position: point_to_lsp(position),
                        version: 0,
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
            CopilotServer::Started { status, .. } => match status {
                SignInStatus::Authorized { .. } => Status::Authorized,
                SignInStatus::Unauthorized { .. } => Status::Unauthorized,
                SignInStatus::SigningIn { prompt, .. } => Status::SigningIn {
                    prompt: prompt.clone(),
                },
                SignInStatus::SignedOut => Status::SignedOut,
            },
        }
    }

    fn update_sign_in_status(
        &mut self,
        lsp_status: request::SignInStatus,
        cx: &mut ModelContext<Self>,
    ) {
        if let CopilotServer::Started { status, .. } = &mut self.server {
            *status = match lsp_status {
                request::SignInStatus::Ok { user }
                | request::SignInStatus::MaybeOk { user }
                | request::SignInStatus::AlreadySignedIn { user } => {
                    SignInStatus::Authorized { _user: user }
                }
                request::SignInStatus::NotAuthorized { user } => {
                    SignInStatus::Unauthorized { _user: user }
                }
                request::SignInStatus::NotSignedIn => SignInStatus::SignedOut,
            };
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
