use std::net::TcpListener;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use acp_thread::AgentConnection as _;
use agent_client_protocol::schema as acp;
use anyhow::{Context, Result};
use collections::HashMap;
use fs::Fs;
use gpui::{App, AsyncApp, Entity};
use language_model::{LanguageModelRegistry, SelectedModel};
use project::Project;
use serde::{Deserialize, Serialize};
use tiny_http::{Header, Response, Server};
use util::path_list::PathList;

use crate::global_native_agent;

#[derive(Clone)]
pub struct AgentHttpServerConfig {
    pub port: u16,
}

impl Default for AgentHttpServerConfig {
    fn default() -> Self {
        Self { port: 8765 }
    }
}

#[derive(Clone)]
pub struct AgentHttpServerHandle {
    _shutdown: async_channel::Sender<()>,
    pub new_sessions: async_channel::Receiver<(acp::SessionId, PathBuf, Option<String>)>,
}

enum AgentServerCommand {
    CreateAgent {
        request: CreateAgentRequest,
        reply: async_channel::Sender<Result<CreateAgentResponse>>,
    },
    PromptAgent {
        session_id: String,
        prompt: String,
        reply: async_channel::Sender<Result<PromptResponse>>,
    },
    GetAgentStatus {
        session_id: String,
        reply: async_channel::Sender<Result<AgentStatusResponse>>,
    },
    CloseAgent {
        session_id: String,
        reply: async_channel::Sender<Result<()>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    #[serde(default)]
    pub workdir: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentResponse {
    pub session_id: String,
    pub model: String,
    pub workdir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub session_id: String,
    pub stop_reason: Option<String>,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatusResponse {
    pub session_id: String,
    pub model: String,
    pub workdir: String,
    pub entry_count: usize,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorBody {
    error: String,
}

pub fn start_agent_http_server(
    config: AgentHttpServerConfig,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) -> Option<AgentHttpServerHandle> {
    let (commands_tx, commands_rx) = async_channel::unbounded::<AgentServerCommand>();
    let (shutdown_tx, shutdown_rx) = async_channel::bounded::<()>(1);
    let (ns_tx, ns_rx) = async_channel::unbounded::<(acp::SessionId, PathBuf, Option<String>)>();

    let listener = match TcpListener::bind(format!("127.0.0.1:{}", config.port)) {
        Ok(l) => l,
        Err(e) => {
            log::warn!("Failed to bind :{}: {e}", config.port);
            return None;
        }
    };
    let commands_tx_for_thread = commands_tx.clone();
    let addr = listener.local_addr().ok();

    thread::Builder::new()
        .name("agent-http-server".into())
        .spawn(move || {
            let server = match Server::from_listener(listener, None) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("server start: {e}");
                    return;
                }
            };
            if let Some(a) = &addr {
                log::info!("Agent HTTP server on http://{a}");
            }
            loop {
                let req = match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(r)) => r,
                    Ok(None) => {
                        if shutdown_rx.try_recv().is_ok() {
                            break;
                        }
                        continue;
                    }
                    Err(_) => continue,
                };
                route_request(req, &commands_tx_for_thread);
            }
            log::info!("Agent HTTP server stopped");
        })
        .ok();

    cx.spawn({
        let p = project.clone();
        let f = fs.clone();
        async move |cx| {
            run_command_loop(p, f, ns_tx, commands_rx, cx).await;
        }
    })
    .detach();

    Some(AgentHttpServerHandle {
        _shutdown: shutdown_tx,
        new_sessions: ns_rx,
    })
}

fn route_request(
    mut request: tiny_http::Request,
    commands: &async_channel::Sender<AgentServerCommand>,
) {
    let url = request.url().to_string();
    let method = request.method().as_str().to_string();
    let route = parse_route(&url);
    match (method.as_str(), route) {
        ("OPTIONS", _) => handle_preflight(request),
        ("GET", Route::Healthz) => respond_text(request, 200, "ok"),
        ("POST", Route::Agents) => {
            let body = read_body(&mut request);
            let (reply, rx) = async_channel::bounded(1);
            match serde_json::from_str::<CreateAgentRequest>(&body) {
                Ok(req) => {
                    let _ = commands.try_send(AgentServerCommand::CreateAgent {
                        request: req,
                        reply,
                    });
                    match rx.recv_blocking() {
                        Ok(Ok(r)) => respond_json(request, 200, &r),
                        Ok(Err(e)) => respond_json(
                            request,
                            500,
                            &ErrorBody {
                                error: format!("{e:#}"),
                            },
                        ),
                        Err(_) => respond_json(
                            request,
                            500,
                            &ErrorBody {
                                error: "no response".into(),
                            },
                        ),
                    }
                }
                Err(e) => respond_json(
                    request,
                    400,
                    &ErrorBody {
                        error: format!("Invalid: {e}"),
                    },
                ),
            }
        }
        ("POST", Route::AgentPrompt(sid)) => {
            let body = read_body(&mut request);
            let (reply, rx) = async_channel::bounded(1);
            match serde_json::from_str::<PromptRequest>(&body) {
                Ok(req) => {
                    let _ = commands.try_send(AgentServerCommand::PromptAgent {
                        session_id: sid,
                        prompt: req.prompt,
                        reply,
                    });
                    match rx.recv_blocking() {
                        Ok(Ok(r)) => respond_json(request, 200, &r),
                        Ok(Err(e)) => respond_json(
                            request,
                            500,
                            &ErrorBody {
                                error: format!("{e:#}"),
                            },
                        ),
                        Err(_) => respond_json(
                            request,
                            500,
                            &ErrorBody {
                                error: "no response".into(),
                            },
                        ),
                    }
                }
                Err(e) => respond_json(
                    request,
                    400,
                    &ErrorBody {
                        error: format!("Invalid: {e}"),
                    },
                ),
            }
        }
        ("GET", Route::Agent(sid)) => {
            let (reply, rx) = async_channel::bounded(1);
            let _ = commands.try_send(AgentServerCommand::GetAgentStatus {
                session_id: sid,
                reply,
            });
            match rx.recv_blocking() {
                Ok(Ok(r)) => respond_json(request, 200, &r),
                Ok(Err(e)) => respond_json(
                    request,
                    500,
                    &ErrorBody {
                        error: format!("{e:#}"),
                    },
                ),
                Err(_) => respond_json(
                    request,
                    500,
                    &ErrorBody {
                        error: "no response".into(),
                    },
                ),
            }
        }
        ("DELETE", Route::Agent(sid)) => {
            let (reply, rx) = async_channel::bounded(1);
            let _ = commands.try_send(AgentServerCommand::CloseAgent {
                session_id: sid,
                reply,
            });
            match rx.recv_blocking() {
                Ok(Ok(())) => respond_json(request, 200, &serde_json::json!({"status": "closed"})),
                Ok(Err(e)) => respond_json(
                    request,
                    500,
                    &ErrorBody {
                        error: format!("{e:#}"),
                    },
                ),
                Err(_) => respond_json(
                    request,
                    500,
                    &ErrorBody {
                        error: "no response".into(),
                    },
                ),
            }
        }
        _ => respond_json(
            request,
            404,
            &ErrorBody {
                error: "Not found".into(),
            },
        ),
    }
}

enum Route {
    Healthz,
    Agents,
    AgentPrompt(String),
    Agent(String),
}
fn parse_route(url: &str) -> Route {
    let path = url.split('?').next().unwrap_or(url);
    if path == "/healthz" {
        return Route::Healthz;
    }
    if path == "/agents" {
        return Route::Agents;
    }
    if let Some(rest) = path.strip_prefix("/agents/") {
        if let Some((id, action)) = rest.rsplit_once('/') {
            if action == "prompt" {
                return Route::AgentPrompt(id.to_string());
            }
        }
        return Route::Agent(rest.to_string());
    }
    Route::Healthz
}

fn read_body(request: &mut tiny_http::Request) -> String {
    let mut body = String::new();
    let reader = request.as_reader();
    std::io::Read::read_to_string(reader, &mut body).ok();
    body
}

fn respond_json<T: Serialize>(request: tiny_http::Request, status: u16, body: &T) {
    match serde_json::to_string(body) {
        Ok(json) => {
            let response = Response::from_string(json)
                .with_status_code(status)
                .with_header(Header::from_str("Content-Type: application/json").unwrap())
                .with_header(Header::from_str("Access-Control-Allow-Origin: *").unwrap());
            request.respond(response).ok();
        }
        Err(_) => respond_text(request, 500, "Internal server error"),
    }
}

fn respond_text(request: tiny_http::Request, status: u16, text: &str) {
    let response = Response::from_string(text)
        .with_status_code(status)
        .with_header(Header::from_str("Content-Type: text/plain").unwrap())
        .with_header(Header::from_str("Access-Control-Allow-Origin: *").unwrap());
    request.respond(response).ok();
}

fn handle_preflight(request: tiny_http::Request) {
    let response = Response::from_string("")
        .with_status_code(204)
        .with_header(Header::from_str("Access-Control-Allow-Origin: *").unwrap())
        .with_header(
            Header::from_str("Access-Control-Allow-Methods: GET, POST, DELETE, OPTIONS").unwrap(),
        )
        .with_header(Header::from_str("Access-Control-Allow-Headers: Content-Type").unwrap());
    request.respond(response).ok();
}

async fn run_command_loop(
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    ns_tx: async_channel::Sender<(acp::SessionId, PathBuf, Option<String>)>,
    receiver: async_channel::Receiver<AgentServerCommand>,
    cx: &mut AsyncApp,
) {
    struct SessionMeta {
        workdir: PathBuf,
        model: String,
    }
    let mut sessions: HashMap<String, SessionMeta> = HashMap::default();

    while let Ok(cmd) = receiver.recv().await {
        match cmd {
            AgentServerCommand::CreateAgent { request, reply } => {
                let r: Option<CreateAgentResponse> = cx.update(|cx| {
                    let dm = LanguageModelRegistry::global(cx).read(cx).default_model();
                    let (prov, mdl) = if let Some(ref d) = dm {
                        (d.provider.clone(), d.model.clone())
                    } else if let Some(req_m) = &request.model {
                        let s = SelectedModel::from_str(req_m).ok()?;
                        let m = LanguageModelRegistry::global(cx)
                            .read(cx)
                            .available_models(cx)
                            .find(|x| x.provider_id() == s.provider && x.id() == s.model)?;
                        let p = LanguageModelRegistry::global(cx)
                            .read(cx)
                            .provider(&m.provider_id())?;
                        (p, m)
                    } else {
                        return None;
                    };
                    let mn = request
                        .model
                        .clone()
                        .unwrap_or_else(|| format!("{}/{}", prov.id().0, mdl.id().0));
                    let wd = match &request.workdir {
                        Some(d) => PathBuf::from(d).canonicalize().ok()?,
                        None => project
                            .read(cx)
                            .visible_worktrees(cx)
                            .next()
                            .map(|w| w.read(cx).abs_path().to_path_buf())?,
                    };
                    let agent = global_native_agent(fs.clone(), cx);
                    let acp = agent.update(cx, |a, cx| a.new_session(project.clone(), cx));
                    let sid = acp.read_with(cx, |t, _| t.session_id().to_string());

                    // Trigger save so session appears in panel
                    acp.update(cx, |_, cx| cx.notify());

                    sessions.insert(
                        sid.clone(),
                        SessionMeta {
                            workdir: wd.clone(),
                            model: mn.clone(),
                        },
                    );
                    ns_tx
                        .try_send((
                            acp::SessionId::new(sid.clone()),
                            wd.clone(),
                            request.title.clone(),
                        ))
                        .ok();
                    Some(CreateAgentResponse {
                        session_id: sid,
                        model: mn,
                        workdir: wd.display().to_string(),
                    })
                });
                reply
                    .send(r.ok_or_else(|| anyhow::anyhow!("Failed")))
                    .await
                    .ok();
            }
            AgentServerCommand::PromptAgent {
                session_id,
                prompt,
                reply,
            } => {
                let sid = acp::SessionId::new(session_id.clone());
                let agent = cx.update(|cx| global_native_agent(fs.clone(), cx));
                let open_task = agent.update(cx, |a, cx| a.open_thread(sid.clone(), project.clone(), cx));

                let acp = match open_task.await {
                    Ok(acp) => acp,
                    Err(e) => {
                        reply
                            .send(Err(anyhow::anyhow!("Load failed: {e}")))
                            .await
                            .ok();
                        continue;
                    }
                };

                if !sessions.contains_key(&session_id) {
                    let wd = acp
                        .read_with(cx, |t, _| {
                            t.work_dirs()
                                .and_then(|d| d.paths().first().cloned())
                                .map(|p| p.to_path_buf())
                        })
                        .unwrap_or_default();
                    sessions.insert(
                        session_id.clone(),
                        SessionMeta {
                            workdir: wd,
                            model: String::new(),
                        },
                    );
                }

                let (acp, fut) = match cx.update(|cx| {
                    sessions.get(&session_id).context("Not found")?;
                    let msg = vec![acp::ContentBlock::Text(acp::TextContent::new(
                        prompt.clone(),
                    ))];
                    let f = acp.update(cx, |t, cx| t.send(msg, cx));
                    Ok((acp.clone(), f))
                }) {
                    Ok(x) => x,
                    Err(e) => {
                        reply.send(Err(e)).await.ok();
                        continue;
                    }
                };

                let res = fut.await;
                let resp = match res {
                    Ok(Some(r)) => {
                        let u = acp.read_with(cx, |t, _| {
                            t.token_usage().map(|u| PromptResponse {
                                session_id: session_id.clone(),
                                stop_reason: Some(format!("{:?}", r.stop_reason)),
                                input_tokens: Some(u.input_tokens),
                                output_tokens: Some(u.output_tokens),
                            })
                        });
                        u.unwrap_or(PromptResponse {
                            session_id: session_id.clone(),
                            stop_reason: Some(format!("{:?}", r.stop_reason)),
                            input_tokens: None,
                            output_tokens: None,
                        })
                    }
                    Ok(None) => PromptResponse {
                        session_id: session_id.clone(),
                        stop_reason: Some("completed".into()),
                        input_tokens: None,
                        output_tokens: None,
                    },
                    Err(e) => {
                        reply
                            .send(Err(anyhow::anyhow!("Prompt failed: {e}")))
                            .await
                            .ok();
                        continue;
                    }
                };
                reply.send(Ok(resp)).await.ok();
            }
            AgentServerCommand::GetAgentStatus { session_id, reply } => {
                let r = cx.update(|cx| {
                    let meta = sessions.get(&session_id).context("Not found")?;
                    let sid = acp::SessionId::new(session_id.clone());
                    let agent = global_native_agent(fs.clone(), cx);
                    let acp = agent
                        .read(cx)
                        .sessions
                        .get(&sid)
                        .map(|s| s.acp_thread.clone())
                        .context("Session not loaded")?;
                    let (ec, st) =
                        acp.read_with(cx, |t, _| (t.entries().len(), format!("{:?}", t.status())));
                    Ok(AgentStatusResponse {
                        session_id: session_id.clone(),
                        model: meta.model.clone(),
                        workdir: meta.workdir.display().to_string(),
                        entry_count: ec,
                        status: st,
                    })
                });
                reply.send(r).await.ok();
            }
            AgentServerCommand::CloseAgent { session_id, reply } => {
                cx.update(|cx| {
                    sessions.remove(&session_id);
                });
                reply.send(Ok(())).await.ok();
            }
        }
    }
}
