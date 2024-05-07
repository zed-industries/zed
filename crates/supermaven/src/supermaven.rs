mod messages;
mod supermaven_completion_provider;

pub use supermaven_completion_provider::*;

use anyhow::{Context as _, Result};
#[allow(unused_imports)]
use client::{proto, Client};
use collections::BTreeMap;

use futures::{channel::mpsc, io::BufReader, AsyncBufReadExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, EntityId, Global, Model, ModelContext, Task, WeakModel};
use language::{language_settings::all_language_settings, Anchor, Buffer, ToOffset};
use messages::*;
use postage::watch;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use smol::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout, Command},
};
use std::{ops::Range, path::PathBuf, process::Stdio, sync::Arc};
use ui::prelude::*;
use util::ResultExt;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let supermaven = cx.new_model(|_| Supermaven::Starting);
    Supermaven::set_global(supermaven.clone(), cx);

    let mut provider = all_language_settings(None, cx).inline_completions.provider;
    if provider == language::language_settings::InlineCompletionProvider::Supermaven {
        supermaven.update(cx, |supermaven, cx| supermaven.start(client.clone(), cx));
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_provider = all_language_settings(None, cx).inline_completions.provider;
        if new_provider != provider {
            provider = new_provider;
            if provider == language::language_settings::InlineCompletionProvider::Supermaven {
                supermaven.update(cx, |supermaven, cx| supermaven.start(client.clone(), cx));
            } else {
                supermaven.update(cx, |supermaven, _cx| supermaven.stop());
            }
        }
    })
    .detach();
}

pub enum Supermaven {
    Starting,
    FailedDownload { error: anyhow::Error },
    Spawned(SupermavenAgent),
    Error { error: anyhow::Error },
}

#[derive(Clone)]
pub enum AccountStatus {
    Unknown,
    NeedsActivation { activate_url: String },
    Ready,
}

#[derive(Clone)]
struct SupermavenGlobal(Model<Supermaven>);

impl Global for SupermavenGlobal {}

impl Supermaven {
    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<SupermavenGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(supermaven: Model<Self>, cx: &mut AppContext) {
        cx.set_global(SupermavenGlobal(supermaven));
    }

    pub fn start(&mut self, client: Arc<Client>, cx: &mut ModelContext<Self>) {
        if let Self::Starting = self {
            cx.spawn(|this, mut cx| async move {
                let binary_path =
                    supermaven_api::get_supermaven_agent_path(client.http_client()).await?;

                this.update(&mut cx, |this, cx| {
                    if let Self::Starting = this {
                        *this =
                            Self::Spawned(SupermavenAgent::new(binary_path, client.clone(), cx)?);
                    }
                    anyhow::Ok(())
                })
            })
            .detach_and_log_err(cx)
        }
    }

    pub fn stop(&mut self) {
        *self = Self::Starting;
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Spawned { .. })
    }

    pub fn complete(
        &mut self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &AppContext,
    ) -> Option<SupermavenCompletion> {
        if let Self::Spawned(agent) = self {
            let buffer_id = buffer.entity_id();
            let buffer = buffer.read(cx);
            let path = buffer
                .file()
                .and_then(|file| Some(file.as_local()?.abs_path(cx)))
                .unwrap_or_else(|| PathBuf::from("untitled"))
                .to_string_lossy()
                .to_string();
            let content = buffer.text();
            let offset = cursor_position.to_offset(buffer);
            let state_id = agent.next_state_id;
            agent.next_state_id.0 += 1;

            let (updates_tx, mut updates_rx) = watch::channel();
            postage::stream::Stream::try_recv(&mut updates_rx).unwrap();

            agent.states.insert(
                state_id,
                SupermavenCompletionState {
                    buffer_id,
                    range: cursor_position.bias_left(buffer)..cursor_position.bias_right(buffer),
                    completion: Vec::new(),
                    text: String::new(),
                    updates_tx,
                },
            );
            let _ = agent
                .outgoing_tx
                .unbounded_send(OutboundMessage::StateUpdate(StateUpdateMessage {
                    new_id: state_id.0.to_string(),
                    updates: vec![
                        StateUpdate::FileUpdate(FileUpdateMessage {
                            path: path.clone(),
                            content,
                        }),
                        StateUpdate::CursorUpdate(CursorPositionUpdateMessage { path, offset }),
                    ],
                }));

            Some(SupermavenCompletion {
                id: state_id,
                updates: updates_rx,
            })
        } else {
            None
        }
    }

    pub fn completion(
        &self,
        id: SupermavenCompletionStateId,
    ) -> Option<&SupermavenCompletionState> {
        if let Self::Spawned(agent) = self {
            agent.states.get(&id)
        } else {
            None
        }
    }
}

pub struct SupermavenAgent {
    _process: Child,
    next_state_id: SupermavenCompletionStateId,
    states: BTreeMap<SupermavenCompletionStateId, SupermavenCompletionState>,
    outgoing_tx: mpsc::UnboundedSender<OutboundMessage>,
    _handle_outgoing_messages: Task<Result<()>>,
    _handle_incoming_messages: Task<Result<()>>,
    pub account_status: AccountStatus,
    service_tier: Option<ServiceTier>,
    #[allow(dead_code)]
    client: Arc<Client>,
}

impl SupermavenAgent {
    fn new(
        binary_path: PathBuf,
        client: Arc<Client>,
        cx: &mut ModelContext<Supermaven>,
    ) -> Result<Self> {
        let mut process = Command::new(&binary_path)
            .arg("stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("failed to start the binary")?;

        let stdin = process
            .stdin
            .take()
            .context("failed to get stdin for process")?;
        let stdout = process
            .stdout
            .take()
            .context("failed to get stdout for process")?;

        let (outgoing_tx, outgoing_rx) = mpsc::unbounded();

        cx.spawn({
            let client = client.clone();
            let outgoing_tx = outgoing_tx.clone();
            move |this, mut cx| async move {
                let mut status = client.status();
                while let Some(status) = status.next().await {
                    if status.is_connected() {
                        let api_key = client.request(proto::GetSupermavenApiKey {}).await?.api_key;
                        outgoing_tx
                            .unbounded_send(OutboundMessage::SetApiKey(SetApiKey { api_key }))
                            .ok();
                        this.update(&mut cx, |this, cx| {
                            if let Supermaven::Spawned(this) = this {
                                this.account_status = AccountStatus::Ready;
                                cx.notify();
                            }
                        })?;
                        break;
                    }
                }
                return anyhow::Ok(());
            }
        })
        .detach();

        Ok(Self {
            _process: process,
            next_state_id: SupermavenCompletionStateId::default(),
            states: BTreeMap::default(),
            outgoing_tx,
            _handle_outgoing_messages: cx
                .spawn(|_, _cx| Self::handle_outgoing_messages(outgoing_rx, stdin)),
            _handle_incoming_messages: cx
                .spawn(|this, cx| Self::handle_incoming_messages(this, stdout, cx)),
            account_status: AccountStatus::Unknown,
            service_tier: None,
            client,
        })
    }

    async fn handle_outgoing_messages(
        mut outgoing: mpsc::UnboundedReceiver<OutboundMessage>,
        mut stdin: ChildStdin,
    ) -> Result<()> {
        while let Some(message) = outgoing.next().await {
            let bytes = serde_json::to_vec(&message)?;
            stdin.write_all(&bytes).await?;
            stdin.write_all(&[b'\n']).await?;
        }
        Ok(())
    }

    async fn handle_incoming_messages(
        this: WeakModel<Supermaven>,
        stdout: ChildStdout,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        const MESSAGE_PREFIX: &str = "SM-MESSAGE ";

        let stdout = BufReader::new(stdout);
        let mut lines = stdout.lines();
        while let Some(line) = lines.next().await {
            let Some(line) = line.context("failed to read line from stdout").log_err() else {
                continue;
            };
            let Some(line) = line.strip_prefix(MESSAGE_PREFIX) else {
                continue;
            };
            let Some(message) = serde_json::from_str::<SupermavenMessage>(&line)
                .with_context(|| format!("failed to deserialize line from stdout: {:?}", line))
                .log_err()
            else {
                continue;
            };

            this.update(&mut cx, |this, _cx| {
                if let Supermaven::Spawned(this) = this {
                    this.handle_message(message);
                }
                Task::ready(anyhow::Ok(()))
            })?
            .await?;
        }

        Ok(())
    }

    fn handle_message(&mut self, message: SupermavenMessage) {
        match message {
            SupermavenMessage::ActivationRequest(request) => {
                self.account_status = match request.activate_url {
                    Some(activate_url) => AccountStatus::NeedsActivation {
                        activate_url: activate_url.clone(),
                    },
                    None => AccountStatus::Ready,
                };
            }
            SupermavenMessage::ServiceTier { service_tier } => {
                self.service_tier = Some(service_tier);
            }
            SupermavenMessage::Response(response) => {
                let state_id = SupermavenCompletionStateId(response.state_id.parse().unwrap());
                if let Some(state) = self.states.get_mut(&state_id) {
                    for item in &response.items {
                        if let ResponseItem::Text { text } = item {
                            state.text.push_str(text);
                        }
                    }
                    state.completion.extend(response.items);
                    *state.updates_tx.borrow_mut() = ();
                }
            }
            SupermavenMessage::Passthrough { passthrough } => self.handle_message(*passthrough),
            _ => {
                log::warn!("unhandled message: {:?}", message);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SupermavenCompletionStateId(usize);

#[allow(dead_code)]
pub struct SupermavenCompletionState {
    buffer_id: EntityId,
    range: Range<Anchor>,
    completion: Vec<ResponseItem>,
    text: String,
    updates_tx: watch::Sender<()>,
}

pub struct SupermavenCompletion {
    pub id: SupermavenCompletionStateId,
    pub updates: watch::Receiver<()>,
}
