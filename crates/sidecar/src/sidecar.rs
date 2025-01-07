use std::{collections::HashMap, ops::Range, sync::Arc};

use axum::{Json, Router};
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture},
    io::BufReader,
    stream::BoxStream,
    AsyncBufReadExt, SinkExt, StreamExt as _,
};
use gpui::{AppContext, AsyncAppContext, Context, Model, Task, WeakModel, WeakView};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl, Request};
use language::Buffer;
use log::{error, info};
use rope::Point;
use serde::Deserialize;
use smol::future::FutureExt;
use text::Anchor;
use workspace::Workspace;

use crate::{
    thread::ThreadId,
    thread_store::ThreadStore,
    types::{
        AgentSessionChatRequest, AnthropicAPIKey, EditedCodeStreamingEvent,
        EditedCodeStreamingRequest, LLMClientConfig, LLMProvider, LLMProviderAPIKeys, LLMType,
        Model as LLMModel, OpenFileRequestPartial, OpenFileResponse, RepoRef, UIEventWithID,
        UserContext,
    },
};

pub struct Sidecar {
    client: Arc<HttpClientWithUrl>,
    thread_store: Model<ThreadStore>,
    workspace: WeakView<Workspace>,
    active_edits: HashMap<String, Edit>,
}

#[derive(Debug)]
pub struct Edit {
    file_path: String,
    original: Range<Anchor>,
    replacement: String,
    done: bool,
}

pub fn llm_client_config() -> LLMClientConfig {
    LLMClientConfig {
        slow_model: LLMType::ClaudeSonnet,
        fast_model: LLMType::ClaudeSonnet,
        models: HashMap::from_iter([(
            LLMType::ClaudeSonnet,
            LLMModel {
                context_length: 200_000,
                temperature: 0.0,
                provider: LLMProvider::Anthropic,
            },
        )]),
        // set env var for the anthropic api key
        providers: vec![LLMProviderAPIKeys::Anthropic(AnthropicAPIKey {
            api_key: std::env::var("ANTHROPIC_API_KEY")
                .expect("please set anthropic api key env variable"),
        })],
    }
}

const UI_SERVER_PORT: u16 = 29159;
type OnMainThreadTask = Box<
    dyn Send
        + Sync
        + 'static
        + FnOnce(WeakModel<Sidecar>, AsyncAppContext) -> LocalBoxFuture<'static, ()>,
>;

impl Sidecar {
    pub fn new(
        thread_store: Model<ThreadStore>,
        workspace: WeakView<Workspace>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        let client = Arc::new(HttpClientWithUrl::new(
            cx.http_client(),
            "http://localhost:42424/api",
            None,
        ));
        let model = cx.new_model(|_| Sidecar {
            client,
            workspace,
            thread_store,
            active_edits: Default::default(),
        });
        let weak = model.downgrade();
        let (tx, mut rx) = mpsc::unbounded::<OnMainThreadTask>();
        // run tasks sends by tokio thread
        cx.spawn(move |cx| async move {
            while let Some(f) = rx.next().await {
                let weak_model = weak.clone();
                cx.spawn(|cx| f(weak_model, cx)).detach();
            }
        })
        .detach();
        Self::start_server(tx);
        model
    }

    pub fn anchored_edit(
        &self,
        request: AgentSessionChatRequestMinimal,
    ) -> BoxFuture<'static, anyhow::Result<BoxStream<'static, anyhow::Result<UIEventWithID>>>> {
        let client = self.client.clone();
        async move {
            // FIXME: get it correctly
            let root = "/scratch/zed";
            let request = AgentSessionChatRequest {
                session_id: request.session_id,
                exchange_id: request.exchange_id,
                editor_url: format!("http://127.0.0.1:{port}", port = UI_SERVER_PORT),
                query: request.query,
                user_context: request.user_context,
                repo_ref: RepoRef::new(root),
                root_directory: root.into(),
                project_labels: vec![],
                codebase_search: false,
                access_token: String::new(),
                model_configuration: llm_client_config(),
                all_files: vec![],
                open_files: vec![],
                shell: "bash".into(),
            };
            let response = client
                .send(
                    Request::post(client.build_url("/agentic/agent_session_edit_anchored"))
                        .header("Content-Type", "application/json")
                        .body(AsyncBody::from(serde_json::to_vec(&request)?))?,
                )
                .await?;
            let body = response.into_body();
            let reader = BufReader::new(body);
            Ok(reader
                .lines()
                .filter_map(|line| async move {
                    match line {
                        Ok(line) => {
                            #[derive(Deserialize)]
                            #[serde(untagged)]
                            #[allow(unused)]
                            enum ResponseType {
                                Event(UIEventWithID),
                                Started { started: bool, session_id: String },
                                Done { done: String, session_id: String },
                            }
                            let line = line.strip_prefix("data:")?;
                            match serde_json::from_str(line) {
                                Ok(ResponseType::Event(response)) => Some(Ok(response)),
                                Ok(ResponseType::Started { .. } | ResponseType::Done { .. }) => {
                                    None
                                }
                                Err(error) => Some(Err(anyhow::format_err!(error))),
                            }
                        }
                        Err(error) => Some(Err(anyhow::format_err!(error))),
                    }
                })
                .boxed())
        }
        .boxed()
    }

    // we need to run axum on tokio runtime on a different thread and
    // send tasks to run on main thread that have access to editor types
    // this server sucks, ideally this changes to jsonrpc over stdio
    fn start_server(tx: mpsc::UnboundedSender<OnMainThreadTask>) {
        let _ = std::thread::Builder::new()
            .name("editor_server".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(Self::server(tx))
            });
    }

    async fn server(tx: mpsc::UnboundedSender<OnMainThreadTask>) -> anyhow::Result<()> {
        let tcp = tokio::net::TcpListener::bind(("127.0.0.1", UI_SERVER_PORT)).await?;
        axum::serve(
            tcp,
            Router::new().fallback({
                let mut tx = tx.clone();
                move |r: axum::http::Uri, body: String| async move {
                    let path = r.path().to_owned();
                    let (otx, orx) = oneshot::channel();
                    tx.send(Box::new(
                        move |this: WeakModel<Self>, cx: AsyncAppContext| {
                            Box::pin(async move {
                                let body = serde_json::from_str(&body).expect("invalid json");
                                let res =
                                    Self::handle_request(this.upgrade().unwrap(), &path, body, cx)
                                        .await;
                                otx.send(res).unwrap();
                            })
                        },
                    ))
                    .await
                    .unwrap();
                    Json(orx.await.unwrap())
                }
            }),
        )
        .await?;
        Ok(())
    }

    async fn open_buffer(
        this: &Model<Self>,
        path: &str,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Model<Buffer>> {
        let buf = cx
            .update(|cx| {
                let proj = this
                    .read(cx)
                    .workspace
                    .upgrade()
                    .unwrap()
                    .read(cx)
                    .project()
                    .clone();
                proj.update(cx, |p, cx| {
                    if let Some(path) = p.find_project_path(path.as_ref(), cx) {
                        p.open_buffer(path, cx)
                    } else {
                        Task::ready(Err(anyhow::format_err!("not found")))
                    }
                })
            })?
            .await?;
        Ok(buf)
    }

    async fn handle_request(
        this: Model<Self>,
        path: &str,
        body: serde_json::Value,
        mut cx: AsyncAppContext,
    ) -> serde_json::Value {
        info!("request: {path}");
        match path {
            "/new_exchange" => {
                let id: ThreadId = serde_json::from_value(body["session_id"].clone()).unwrap();
                cx.update(|cx| {
                    let thread = this
                        .read(cx)
                        .thread_store
                        .read(cx)
                        .open_thread(&id, cx)
                        .expect("thread not found");
                    let id = thread.update(cx, |t, _| t.next_message_id());
                    serde_json::json!({
                        "exchange_id": format!("{id}", id = id.0),
                    })
                })
                .expect("app is fine")
            }
            "/recent_edits" => {
                serde_json::json!({
                    "changed_files": [],
                })
            }
            "/file_open" => {
                let request = serde_json::from_value::<OpenFileRequestPartial>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx).await;
                let resp = if let Ok(buf) = buf {
                    let text = cx.update_model(&buf, |buf, _| buf.text()).unwrap();
                    OpenFileResponse {
                        fs_file_path: request.fs_file_path.clone(),
                        file_contents: text,
                        exists: true,
                        language: "rust".to_string(),
                    }
                } else {
                    OpenFileResponse {
                        fs_file_path: request.fs_file_path.clone(),
                        file_contents: String::new(),
                        exists: false,
                        language: "rust".to_string(),
                    }
                };
                serde_json::to_value(resp).unwrap()
            }
            "/create_file" => {
                // FIXME: actually create files
                serde_json::json!({
                    "done": false,
                    "fs_file_path": "",
                })
            }
            "/get_outline_nodes" => {
                let request = serde_json::from_value::<OpenFileRequestPartial>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();
                let text = cx.update_model(&buf, |buf, _| buf.text()).unwrap();
                serde_json::json!({
                    "file_content": text,
                    "language": "rust",
                    "outline_nodes": []
                })
            }
            "/apply_edits_streamed" => {
                let request = serde_json::from_value::<EditedCodeStreamingRequest>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();
                cx.update(|cx| {
                    this.update(cx, |this, cx| {
                        let start_point: Point = request.range.start_position.into();
                        let mut end_point: Point = request.range.end_position.into();
                        // sidecar range is inclusive row level
                        end_point.row += 1;
                        let range = buf.read(cx).anchor_after(start_point)
                            ..buf.read(cx).anchor_before(end_point);
                        let edit = this
                            .active_edits
                            .entry(request.edit_request_id.clone())
                            .or_insert(Edit {
                                file_path: request.fs_file_path.clone(),
                                original: range.clone(),
                                replacement: String::new(),
                                done: false,
                            });
                        match request.event {
                            EditedCodeStreamingEvent::Start => {}
                            EditedCodeStreamingEvent::Delta(d) => {
                                // ignore markdown
                                if !d.contains("```") {
                                    edit.replacement += &d;
                                }
                            }
                            EditedCodeStreamingEvent::End => {
                                edit.done = true;
                                dbg!(request.range.clone());
                                dbg!(edit.replacement.clone());
                                // apply edit
                                // TODO: use proposed edit api
                                buf.update(cx, |buf, cx| {
                                    buf.edit([(range, edit.replacement.clone() + "\n")], None, cx);
                                });
                            }
                        }
                    })
                });
                serde_json::json!({})
            }
            _ => {
                error!("unknown request: {path}");
                serde_json::json!({})
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSessionChatRequestMinimal {
    pub session_id: String,
    pub exchange_id: String,
    pub query: String,
    pub user_context: UserContext,
}
