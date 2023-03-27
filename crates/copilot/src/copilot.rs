mod request;
mod sign_in;

use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use client::Client;
use futures::{future::Shared, FutureExt, TryFutureExt};
use gpui::{actions, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use language::{point_from_lsp, point_to_lsp, Anchor, Bias, Buffer, BufferSnapshot, ToPointUtf16};
use lsp::LanguageServer;
use settings::Settings;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    fs::remove_matching, github::latest_github_release, http::HttpClient, paths, ResultExt,
};

actions!(copilot, [SignIn, SignOut, NextSuggestion]);

pub fn init(client: Arc<Client>, cx: &mut MutableAppContext) {
    let copilot = cx.add_model(|cx| Copilot::start(client.http_client(), cx));
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
    sign_in::init(cx);
}

enum CopilotServer {
    Downloading,
    Error(Arc<str>),
    Started {
        server: Arc<LanguageServer>,
        status: SignInStatus,
    },
}

#[derive(Clone, Debug)]
enum SignInStatus {
    Authorized {
        user: String,
    },
    Unauthorized {
        user: String,
    },
    SigningIn {
        prompt: Option<request::PromptUserDeviceFlow>,
        task: Shared<Task<Result<(), Arc<anyhow::Error>>>>,
    },
    SignedOut,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Downloading,
    Error(Arc<str>),
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

#[derive(Debug)]
pub struct Completion {
    pub position: Anchor,
    pub text: String,
}

pub struct Copilot {
    server: CopilotServer,
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

    fn start(http: Arc<dyn HttpClient>, cx: &mut ModelContext<Self>) -> Self {
        // TODO: Don't eagerly download the LSP
        cx.spawn(|this, mut cx| async move {
            let start_language_server = async {
                let server_path = get_lsp_binary(http).await?;
                let server =
                    LanguageServer::new(0, &server_path, &["--stdio"], Path::new("/"), cx.clone())?;
                let server = server.initialize(Default::default()).await?;
                let status = server
                    .request::<request::CheckStatus>(request::CheckStatusParams {
                        local_checks_only: false,
                    })
                    .await?;
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
                        };
                        this.update_sign_in_status(status, cx);
                    }
                    Err(error) => {
                        this.server = CopilotServer::Error(error.to_string().into());
                    }
                }
            })
        })
        .detach();

        Self {
            server: CopilotServer::Downloading,
        }
    }

    fn sign_in(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let CopilotServer::Started { server, status } = &mut self.server {
            let task = match status {
                SignInStatus::Authorized { .. } | SignInStatus::Unauthorized { .. } => {
                    Task::ready(Ok(())).shared()
                }
                SignInStatus::SigningIn { task, .. } => task.clone(),
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
            Task::ready(Err(anyhow!("copilot hasn't started yet")))
        }
    }

    fn sign_out(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let CopilotServer::Started { server, status } = &mut self.server {
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

    pub fn completion<T>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Completion>>>
    where
        T: ToPointUtf16,
    {
        let server = match self.authorized_server() {
            Ok(server) => server,
            Err(error) => return Task::ready(Err(error)),
        };

        let buffer = buffer.read(cx).snapshot();
        let request = server
            .request::<request::GetCompletions>(build_completion_params(&buffer, position, cx));
        cx.background().spawn(async move {
            let result = request.await?;
            let completion = result
                .completions
                .into_iter()
                .next()
                .map(|completion| completion_from_lsp(completion, &buffer));
            anyhow::Ok(completion)
        })
    }

    pub fn completions_cycling<T>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>>
    where
        T: ToPointUtf16,
    {
        let server = match self.authorized_server() {
            Ok(server) => server,
            Err(error) => return Task::ready(Err(error)),
        };

        let buffer = buffer.read(cx).snapshot();
        let request = server.request::<request::GetCompletionsCycling>(build_completion_params(
            &buffer, position, cx,
        ));
        cx.background().spawn(async move {
            let result = request.await?;
            let completions = result
                .completions
                .into_iter()
                .map(|completion| completion_from_lsp(completion, &buffer))
                .collect();
            anyhow::Ok(completions)
        })
    }

    pub fn status(&self) -> Status {
        match &self.server {
            CopilotServer::Downloading => Status::Downloading,
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
                request::SignInStatus::Ok { user } | request::SignInStatus::MaybeOk { user } => {
                    SignInStatus::Authorized { user }
                }
                request::SignInStatus::NotAuthorized { user } => {
                    SignInStatus::Unauthorized { user }
                }
                _ => SignInStatus::SignedOut,
            };
            cx.notify();
        }
    }

    fn authorized_server(&self) -> Result<Arc<LanguageServer>> {
        match &self.server {
            CopilotServer::Downloading => Err(anyhow!("copilot is still downloading")),
            CopilotServer::Error(error) => Err(anyhow!(
                "copilot was not started because of an error: {}",
                error
            )),
            CopilotServer::Started { server, status } => {
                if matches!(status, SignInStatus::Authorized { .. }) {
                    Ok(server.clone())
                } else {
                    Err(anyhow!("must sign in before using copilot"))
                }
            }
        }
    }
}

fn build_completion_params<T>(
    buffer: &BufferSnapshot,
    position: T,
    cx: &AppContext,
) -> request::GetCompletionsParams
where
    T: ToPointUtf16,
{
    let position = position.to_point_utf16(&buffer);
    let language_name = buffer.language_at(position).map(|language| language.name());
    let language_name = language_name.as_deref();

    let path;
    let relative_path;
    if let Some(file) = buffer.file() {
        if let Some(file) = file.as_local() {
            path = file.abs_path(cx);
        } else {
            path = file.full_path(cx);
        }
        relative_path = file.path().to_path_buf();
    } else {
        path = PathBuf::from("/untitled");
        relative_path = PathBuf::from("untitled");
    }

    let settings = cx.global::<Settings>();
    let language_id = match language_name {
        Some("Plain Text") => "plaintext".to_string(),
        Some(language_name) => language_name.to_lowercase(),
        None => "plaintext".to_string(),
    };
    request::GetCompletionsParams {
        doc: request::GetCompletionsDocument {
            source: buffer.text(),
            tab_size: settings.tab_size(language_name).into(),
            indent_size: 1,
            insert_spaces: !settings.hard_tabs(language_name),
            uri: lsp::Url::from_file_path(&path).unwrap(),
            path: path.to_string_lossy().into(),
            relative_path: relative_path.to_string_lossy().into(),
            language_id,
            position: point_to_lsp(position),
            version: 0,
        },
    }
}

fn completion_from_lsp(completion: request::Completion, buffer: &BufferSnapshot) -> Completion {
    let position = buffer.clip_point_utf16(point_from_lsp(completion.position), Bias::Left);
    Completion {
        position: buffer.anchor_before(position),
        text: completion.display_text,
    }
}

async fn get_lsp_binary(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
    ///Check for the latest copilot language server and download it if we haven't already
    async fn fetch_latest(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
        let release = latest_github_release("zed-industries/copilot", http.clone()).await?;
        let asset_name = format!("copilot-darwin-{}.gz", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;

        fs::create_dir_all(&*paths::COPILOT_DIR).await?;
        let destination_path =
            paths::COPILOT_DIR.join(format!("copilot-{}-{}", release.name, consts::ARCH));

        if fs::metadata(&destination_path).await.is_err() {
            let mut response = http
                .get(&asset.browser_download_url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = fs::File::create(&destination_path).await?;
            futures::io::copy(decompressed_bytes, &mut file).await?;
            fs::set_permissions(
                &destination_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;

            remove_matching(&paths::COPILOT_DIR, |entry| entry != destination_path).await;
        }

        Ok(destination_path)
    }

    match fetch_latest(http).await {
        ok @ Result::Ok(..) => ok,
        e @ Err(..) => {
            e.log_err();
            // Fetch a cached binary, if it exists
            (|| async move {
                let mut last = None;
                let mut entries = fs::read_dir(paths::COPILOT_DIR.as_path()).await?;
                while let Some(entry) = entries.next().await {
                    last = Some(entry?.path());
                }
                last.ok_or_else(|| anyhow!("no cached binary"))
            })()
            .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use util::http;

    #[gpui::test]
    async fn test_smoke(cx: &mut TestAppContext) {
        Settings::test_async(cx);
        let http = http::client();
        let copilot = cx.add_model(|cx| Copilot::start(http, cx));
        smol::Timer::after(std::time::Duration::from_secs(2)).await;
        copilot
            .update(cx, |copilot, cx| copilot.sign_in(cx))
            .await
            .unwrap();
        dbg!(copilot.read_with(cx, |copilot, _| copilot.status()));

        let buffer = cx.add_model(|cx| language::Buffer::new(0, "fn foo() -> ", cx));
        dbg!(copilot
            .update(cx, |copilot, cx| copilot.completion(&buffer, 12, cx))
            .await
            .unwrap());
        dbg!(copilot
            .update(cx, |copilot, cx| copilot
                .completions_cycling(&buffer, 12, cx))
            .await
            .unwrap());
    }
}
