mod messages;
mod supermaven_completion_provider;

use anyhow::{Context as _, Result};
use collections::BTreeMap;
use futures::{channel::mpsc, io::BufReader, AsyncBufReadExt, StreamExt};
use gpui::{
    AppContext, AsyncAppContext, Bounds, DevicePixels, EntityId, Global, InteractiveText, Model,
    Render, StyledText, Task, ViewContext,
};
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
pub use supermaven_completion_provider::*;
use ui::prelude::*;
use util::{http::HttpClient, ResultExt};

pub fn init(client: Arc<dyn HttpClient>, cx: &mut AppContext) {
    cx.set_global(Supermaven::Starting);

    let mut provider = all_language_settings(None, cx).inline_completions.provider;
    if provider == language::language_settings::InlineCompletionProvider::Supermaven {
        Supermaven::update(cx, |supermaven, cx| supermaven.start(client.clone(), cx));
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_provider = all_language_settings(None, cx).inline_completions.provider;
        if new_provider != provider {
            provider = new_provider;
            if provider == language::language_settings::InlineCompletionProvider::Supermaven {
                Supermaven::update(cx, |supermaven, cx| supermaven.start(client.clone(), cx));
            } else {
                Supermaven::update(cx, |supermaven, _cx| supermaven.stop());
            }
        }
    })
    .detach();
}

pub enum Supermaven {
    Starting,
    FailedDownload {
        error: anyhow::Error,
    },
    Spawned {
        _process: Child,
        next_state_id: SupermavenCompletionStateId,
        states: BTreeMap<SupermavenCompletionStateId, SupermavenCompletionState>,
        outgoing_tx: mpsc::UnboundedSender<OutboundMessage>,
        _handle_outgoing_messages: Task<Result<()>>,
        _handle_incoming_messages: Task<Result<()>>,
    },
}

impl Supermaven {
    pub fn start(&mut self, client: Arc<dyn HttpClient>, cx: &mut AppContext) {
        if let Self::Starting = self {
            cx.spawn(|cx| async move {
                // todo!(): Don't download the most up to date binary every time, check to see if a recent is downloaded
                let binary_path = supermaven_api::download_latest(client).await?;

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
                cx.update(|cx| {
                    Self::update(cx, |this, cx| {
                        if let Self::Starting = this {
                            let (outgoing_tx, outgoing_rx) = mpsc::unbounded();
                            outgoing_tx
                                .unbounded_send(OutboundMessage::UseFreeVersion)
                                .unwrap();
                            *this = Self::Spawned {
                                _process: process,
                                next_state_id: SupermavenCompletionStateId::default(),
                                states: BTreeMap::default(),
                                outgoing_tx,
                                _handle_outgoing_messages: cx.spawn(|_cx| {
                                    Self::handle_outgoing_messages(outgoing_rx, stdin)
                                }),
                                _handle_incoming_messages: cx
                                    .spawn(|cx| Self::handle_incoming_messages(stdout, cx)),
                            };
                        }
                    });
                })
            })
            .detach_and_log_err(cx);
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
        if let Self::Spawned {
            next_state_id,
            states,
            outgoing_tx,
            ..
        } = self
        {
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
            let state_id = *next_state_id;
            next_state_id.0 += 1;

            let (updates_tx, mut updates_rx) = watch::channel();
            postage::stream::Stream::try_recv(&mut updates_rx).unwrap();

            states.insert(
                state_id,
                SupermavenCompletionState {
                    buffer_id,
                    range: cursor_position.bias_left(buffer)..cursor_position.bias_right(buffer),
                    completion: Vec::new(),
                    text: String::new(),
                    updates_tx,
                },
            );
            let _ = outgoing_tx.unbounded_send(OutboundMessage::StateUpdate(StateUpdateMessage {
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
        if let Self::Spawned { states, .. } = self {
            states.get(&id)
        } else {
            None
        }
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

    async fn handle_incoming_messages(stdout: ChildStdout, cx: AsyncAppContext) -> Result<()> {
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

            cx.update(|cx| Self::update(cx, |this, cx| this.handle_message(message, cx)))?;
        }

        Ok(())
    }

    fn handle_message(&mut self, message: SupermavenMessage, cx: &mut AppContext) {
        match message {
            SupermavenMessage::ActivationRequest(request) => {
                let Some(activate_url) = request.activate_url else {
                    return;
                };

                cx.open_window(
                    gpui::WindowOptions {
                        bounds: Some(Bounds::new(
                            gpui::point(DevicePixels::from(0), DevicePixels::from(0)),
                            gpui::size(DevicePixels::from(800), DevicePixels::from(600)),
                        )),
                        titlebar: None,
                        focus: false,
                        ..Default::default()
                    },
                    |cx| cx.new_view(|_cx| ActivationRequestPrompt::new(activate_url.into())),
                );
            }
            SupermavenMessage::Response(response) => {
                if let Self::Spawned { states, .. } = self {
                    let state_id = SupermavenCompletionStateId(response.state_id.parse().unwrap());
                    if let Some(state) = states.get_mut(&state_id) {
                        for item in &response.items {
                            if let ResponseItem::Text { text } = item {
                                state.text.push_str(text);
                            }
                        }
                        state.completion.extend(response.items);
                        *state.updates_tx.borrow_mut() = ();
                    }
                }
            }
            SupermavenMessage::Passthrough { passthrough } => self.handle_message(*passthrough, cx),
            _ => {
                log::warn!("unhandled message: {:?}", message);
            }
        }
    }
}

impl Global for Supermaven {}

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

struct ActivationRequestPrompt {
    activate_url: SharedString,
}

impl ActivationRequestPrompt {
    fn new(activate_url: SharedString) -> Self {
        Self { activate_url }
    }
}

impl Render for ActivationRequestPrompt {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        InteractiveText::new(
            "activation_prompt",
            StyledText::new(self.activate_url.clone()),
        )
        .on_click(vec![0..self.activate_url.len()], {
            let activate_url = self.activate_url.clone();
            move |_, cx| cx.open_url(&activate_url)
        })
    }
}
