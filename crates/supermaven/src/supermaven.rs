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
    AppContext, AsyncAppContext, Global, Subscription, Task, View, ViewContext, WeakView,
    WindowContext,
};
use messages::*;
use project::Project;
use serde::{Deserialize, Serialize};
use smol::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout, Command},
};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
};
use util::ResultExt;

pub struct Supermaven {
    _process: Child,
    next_state_id: SupermavenStateId,
    states: HashMap<SupermavenStateId, CompletionState>,
    update_txs: Vec<oneshot::Sender<()>>,
    outgoing_tx: mpsc::UnboundedSender<StateUpdateMessage>,
    _handle_outgoing_messages: Task<Result<()>>,
    _handle_incoming_messages: Task<Result<()>>,
    _maintain_editors: Subscription,
    registered_editors: HashMap<WeakView<Editor>, RegisteredEditor>,
}

impl Supermaven {
    pub fn launch(cx: &mut AppContext) -> Task<Result<()>> {
        cx.spawn(|cx| async move {
            let binary_path = &std::env::var("SUPERMAVEN_AGENT_BINARY")
                .unwrap_or_else(|_| "/Users/as-cii/Downloads/sm-agent".to_string());
            let binary_path = Path::new(binary_path);
            let mut process = Command::new(binary_path)
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
                    dbg!("GIMME EDITOR");
                    if editor.mode() == EditorMode::Full {
                        Self::update(cx, |this, cx| this.register_editor(editor, cx))
                    }
                }
            }),
            registered_editors: HashMap::default(),
        }
    }

    fn register_editor(&mut self, _editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        dbg!("!!!!!!!!!!!!!!");
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
        dbg!("!!!!!!!!!!");
        match event {
            EditorEvent::Edited | EditorEvent::SelectionsChanged { local: true } => {
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
                let _ = self.outgoing_tx.unbounded_send(StateUpdateMessage {
                    kind: StateUpdateKind::StateUpdate,
                    new_id: state_id.0.to_string(),
                    updates: vec![
                        StateUpdate::FileUpdate(FileUpdateMessage {
                            path: path.clone(),
                            content,
                        }),
                        StateUpdate::CursorPositionUpdate(CursorPositionUpdateMessage {
                            path,
                            offset,
                        }),
                    ],
                });
            }
            _ => {}
        }
    }

    async fn handle_outgoing_messages(
        mut outgoing: mpsc::UnboundedReceiver<StateUpdateMessage>,
        mut stdin: ChildStdin,
    ) -> Result<()> {
        while let Some(message) = outgoing.next().await {
            dbg!(&message);
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
                .context("failed to deserialize line from stdout")
                .log_err()
            else {
                continue;
            };

            cx.update(|cx| Self::update(cx, |this, cx| this.handle_message(message, cx)))?;
        }

        Ok(())
    }

    fn handle_message(&mut self, message: SupermavenMessage, _cx: &mut AppContext) {
        match message {
            SupermavenMessage::Response(response) => {
                if let Some(state) = self.states.get_mut(&response.state_id) {
                    state.completion.extend(response.items);
                    for update_tx in self.update_txs.drain(..) {
                        let _ = update_tx.send(());
                    }
                }
            }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use gpui::{Context, TestAppContext};
    use language::{
        language_settings::{AllLanguageSettings, AllLanguageSettingsContent},
        Buffer, BufferId, LanguageRegistry,
    };
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_exploratory(cx: &mut TestAppContext) {
        init_test(cx);
        let background_executor = cx.executor();
        background_executor.allow_parking();

        cx.update(Supermaven::launch).await.unwrap();

        let language_registry = Arc::new(LanguageRegistry::test(background_executor.clone()));

        let markdown = language_registry.language_for_name("Markdown");

        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::new(
                0,
                BufferId::new(cx.entity_id().as_u64()).unwrap(),
                "import this",
            );
            buffer.set_language_registry(language_registry);
            cx.spawn(|buffer, mut cx| async move {
                let markdown = markdown.await?;
                buffer.update(&mut cx, |buffer: &mut Buffer, cx| {
                    buffer.set_language(Some(markdown), cx);
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            buffer
        });

        dbg!("!!!!");
        // let buffer = dbg!(cx.new_model(|_cx| Buffer::new(0, BufferId::new(1).unwrap(), "Hello ")));
        let editor = cx.add_window(|cx| Editor::for_buffer(buffer, None, cx));

        editor.update(cx, |editor, cx| editor.insert("HEY", cx));

        // let state_update = StateUpdateMessage {
        //     kind: StateUpdateKind::StateUpdate,
        //     new_id: "123".into(),
        //     updates: vec![],
        // };
        //

        cx.executor()
            .timer(std::time::Duration::from_secs(60))
            .await;

        // supermaven.kill();
    }

    pub(crate) fn update_test_language_settings(
        cx: &mut TestAppContext,
        f: impl Fn(&mut AllLanguageSettingsContent),
    ) {
        _ = cx.update(|cx| {
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, f);
            });
        });
    }

    pub fn init_test(cx: &mut TestAppContext) {
        _ = cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            // release_channel::init("0.0.0", cx);
            // client::init_settings(cx);
            language::init(cx);
            Project::init_settings(cx);
            // workspace::init_settings(cx);
            editor::init(cx);
        });

        update_test_language_settings(cx, |_| {});
    }
}
