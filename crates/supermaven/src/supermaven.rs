mod messages;

use anyhow::{Context as _, Result};
use collections::HashMap;
use editor::{Editor, EditorEvent, EditorMode};
use futures::{
    channel::{mpsc, oneshot},
    io::BufReader,
    AsyncBufReadExt, StreamExt,
};
use gpui::{
    AppContext, AsyncAppContext, Bounds, Global, GlobalPixels, InteractiveText, Render, StyledText,
    Subscription, Task, View, ViewContext, WeakView, WindowContext,
};
use messages::*;
use serde::{Deserialize, Serialize};
use smol::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout, Command},
};
use std::{path::PathBuf, process::Stdio};
use ui::prelude::*;
use util::ResultExt;

pub struct Supermaven {
    _process: Child,
    next_state_id: SupermavenStateId,
    states: HashMap<SupermavenStateId, CompletionState>,
    update_txs: Vec<oneshot::Sender<()>>,
    outgoing_tx: mpsc::UnboundedSender<OutboundMessage>,
    _handle_outgoing_messages: Task<Result<()>>,
    _handle_incoming_messages: Task<Result<()>>,
    _maintain_editors: Subscription,
    registered_editors: HashMap<WeakView<Editor>, RegisteredEditor>,
}

impl Supermaven {
    pub fn launch(cx: &mut AppContext) -> Task<Result<()>> {
        cx.spawn(|cx| async move {
            let binary_path = std::env::var("SUPERMAVEN_AGENT_BINARY")
                .expect("set SUPERMAVEN_AGENT_BINARY env variable");
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
                let supermaven = Self::new(process, stdin, stdout, cx);
                cx.set_global(supermaven);
            })
        })
    }

    fn new(process: Child, stdin: ChildStdin, stdout: ChildStdout, cx: &mut AppContext) -> Self {
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded();
        outgoing_tx
            .unbounded_send(OutboundMessage::UseFreeVersion)
            .unwrap();

        Self {
            _process: process,
            next_state_id: SupermavenStateId::default(),
            states: HashMap::default(),
            update_txs: Vec::new(),
            outgoing_tx,
            _handle_outgoing_messages: cx
                .spawn(|_cx| Self::handle_outgoing_messages(outgoing_rx, stdin)),
            _handle_incoming_messages: cx.spawn(|cx| Self::handle_incoming_messages(stdout, cx)),
            _maintain_editors: cx.observe_new_views({
                |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
                    if editor.mode() == EditorMode::Full {
                        Self::update(cx, |this, cx| this.register_editor(editor, cx))
                    }
                }
            }),
            registered_editors: HashMap::default(),
        }
    }

    fn register_editor(&mut self, _editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        let editor_handle = cx.view().clone();
        self.registered_editors.insert(
            editor_handle.downgrade(),
            RegisteredEditor {
                _subscription: cx.window_context().subscribe(
                    &editor_handle,
                    |editor, event, cx| {
                        Self::update(cx, |this, cx| this.handle_editor_event(editor, event, cx))
                    },
                ),
            },
        );
    }

    fn handle_editor_event(
        &mut self,
        editor: View<Editor>,
        event: &EditorEvent,
        cx: &mut WindowContext,
    ) {
        match event {
            EditorEvent::Edited => {
                // todo!("address multi-buffers")
                let offset = editor.read(cx).selections.newest::<usize>(cx).head();
                let path = editor
                    .read(cx)
                    .file_at(offset, cx)
                    .and_then(|file| Some(file.as_local()?.abs_path(cx)))
                    .unwrap_or_else(|| PathBuf::from("untitled"))
                    .to_string_lossy()
                    .to_string();
                let content = editor.read(cx).text(cx);
                let state_id = self.next_state_id;
                self.next_state_id.0 += 1;

                self.states.insert(
                    state_id,
                    CompletionState {
                        prefix: content[..offset].to_string(),
                        suffix: content[offset..].to_string(),
                        completion: Vec::new(),
                    },
                );
                let _ = self
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
            }
            _ => {}
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
                            gpui::point(GlobalPixels::from(0.), GlobalPixels::from(0.)),
                            gpui::size(GlobalPixels::from(800.), GlobalPixels::from(600.)),
                        )),
                        titlebar: None,
                        focus: false,
                        ..Default::default()
                    },
                    |cx| cx.new_view(|_cx| ActivationRequestPrompt::new(activate_url.into())),
                );
            }
            SupermavenMessage::Response(response) => {
                let state_id = SupermavenStateId(response.state_id.parse().unwrap());
                if let Some(state) = self.states.get_mut(&state_id) {
                    state.completion.extend(response.items);
                    for update_tx in self.update_txs.drain(..) {
                        let _ = update_tx.send(());
                    }
                }
            }
            SupermavenMessage::Passthrough { passthrough } => self.handle_message(*passthrough, cx),
            _ => {
                dbg!(&message);
            }
        }
    }
}

impl Global for Supermaven {}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct SupermavenStateId(usize);

#[allow(dead_code)]
struct CompletionState {
    prefix: String,
    suffix: String,
    completion: Vec<ResponseItem>,
}

struct RegisteredEditor {
    _subscription: Subscription,
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use gpui::{Context, TestAppContext};
    use language::{language_settings::AllLanguageSettings, Buffer, BufferId, LanguageRegistry};
    use project::Project;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_exploratory(cx: &mut TestAppContext) {
        env_logger::init();

        init_test(cx);
        let background_executor = cx.executor();
        background_executor.allow_parking();

        cx.update(Supermaven::launch).await.unwrap();

        let language_registry = Arc::new(LanguageRegistry::test(background_executor.clone()));

        let python = language_registry.language_for_name("Python");

        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), "");
            buffer.set_language_registry(language_registry);
            cx.spawn(|buffer, mut cx| async move {
                let python = python.await?;
                buffer.update(&mut cx, |buffer: &mut Buffer, cx| {
                    buffer.set_language(Some(python), cx);
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            buffer
        });

        let editor = cx.add_window(|cx| Editor::for_buffer(buffer, None, cx));
        editor
            .update(cx, |editor, cx| editor.insert("import numpy as ", cx))
            .unwrap();

        cx.executor()
            .timer(std::time::Duration::from_secs(60))
            .await;
    }

    pub fn init_test(cx: &mut TestAppContext) {
        _ = cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            Project::init_settings(cx);
            editor::init(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
            });
        });
    }
}
