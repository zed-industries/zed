use std::{cell::Cell, ops::Range, rc::Rc};

use acp_thread::{AcpThread, AgentThreadEntry};
use agent_client_protocol::{PromptCapabilities, ToolCallId};
use agent2::HistoryStore;
use collections::HashMap;
use editor::{Editor, EditorMode, MinimapVisibility};
use gpui::{
    AnyEntity, App, AppContext as _, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    ScrollHandle, TextStyleRefinement, WeakEntity, Window,
};
use language::language_settings::SoftWrap;
use project::Project;
use prompt_store::PromptStore;
use settings::Settings as _;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::{Context, TextSize};
use workspace::Workspace;

use crate::acp::message_editor::{MessageEditor, MessageEditorEvent};

pub struct EntryViewState {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    entries: Vec<Entry>,
    prevent_slash_commands: bool,
    prompt_capabilities: Rc<Cell<PromptCapabilities>>,
}

impl EntryViewState {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<Cell<PromptCapabilities>>,
        prevent_slash_commands: bool,
    ) -> Self {
        Self {
            workspace,
            project,
            history_store,
            prompt_store,
            entries: Vec::new(),
            prevent_slash_commands,
            prompt_capabilities,
        }
    }

    pub fn entry(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index)
    }

    pub fn sync_entry(
        &mut self,
        index: usize,
        thread: &Entity<AcpThread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread_entry) = thread.read(cx).entries().get(index) else {
            return;
        };

        match thread_entry {
            AgentThreadEntry::UserMessage(message) => {
                let has_id = message.id.is_some();
                let chunks = message.chunks.clone();
                if let Some(Entry::UserMessage(editor)) = self.entries.get_mut(index) {
                    if !editor.focus_handle(cx).is_focused(window) {
                        // Only update if we are not editing.
                        // If we are, cancelling the edit will set the message to the newest content.
                        editor.update(cx, |editor, cx| {
                            editor.set_message(chunks, window, cx);
                        });
                    }
                } else {
                    let message_editor = cx.new(|cx| {
                        let mut editor = MessageEditor::new(
                            self.workspace.clone(),
                            self.project.clone(),
                            self.history_store.clone(),
                            self.prompt_store.clone(),
                            self.prompt_capabilities.clone(),
                            "Edit message － @ to include context",
                            self.prevent_slash_commands,
                            editor::EditorMode::AutoHeight {
                                min_lines: 1,
                                max_lines: None,
                            },
                            window,
                            cx,
                        );
                        if !has_id {
                            editor.set_read_only(true, cx);
                        }
                        editor.set_message(chunks, window, cx);
                        editor
                    });
                    cx.subscribe(&message_editor, move |_, editor, event, cx| {
                        cx.emit(EntryViewEvent {
                            entry_index: index,
                            view_event: ViewEvent::MessageEditorEvent(editor, *event),
                        })
                    })
                    .detach();
                    self.set_entry(index, Entry::UserMessage(message_editor));
                }
            }
            AgentThreadEntry::ToolCall(tool_call) => {
                let id = tool_call.id.clone();
                let terminals = tool_call.terminals().cloned().collect::<Vec<_>>();
                let diffs = tool_call.diffs().cloned().collect::<Vec<_>>();

                let views = if let Some(Entry::Content(views)) = self.entries.get_mut(index) {
                    views
                } else {
                    self.set_entry(index, Entry::empty());
                    let Some(Entry::Content(views)) = self.entries.get_mut(index) else {
                        unreachable!()
                    };
                    views
                };

                let is_tool_call_completed =
                    matches!(tool_call.status, acp_thread::ToolCallStatus::Completed);

                for terminal in terminals {
                    match views.entry(terminal.entity_id()) {
                        collections::hash_map::Entry::Vacant(entry) => {
                            let element = create_terminal(
                                self.workspace.clone(),
                                self.project.clone(),
                                terminal.clone(),
                                window,
                                cx,
                            )
                            .into_any();
                            cx.emit(EntryViewEvent {
                                entry_index: index,
                                view_event: ViewEvent::NewTerminal(id.clone()),
                            });
                            entry.insert(element);
                        }
                        collections::hash_map::Entry::Occupied(_entry) => {
                            if is_tool_call_completed && terminal.read(cx).output().is_none() {
                                cx.emit(EntryViewEvent {
                                    entry_index: index,
                                    view_event: ViewEvent::TerminalMovedToBackground(id.clone()),
                                });
                            }
                        }
                    }
                }

                for diff in diffs {
                    views.entry(diff.entity_id()).or_insert_with(|| {
                        let element = create_editor_diff(diff.clone(), window, cx).into_any();
                        cx.emit(EntryViewEvent {
                            entry_index: index,
                            view_event: ViewEvent::NewDiff(id.clone()),
                        });
                        element
                    });
                }
            }
            AgentThreadEntry::AssistantMessage(message) => {
                let entry = if let Some(Entry::AssistantMessage(entry)) =
                    self.entries.get_mut(index)
                {
                    entry
                } else {
                    self.set_entry(
                        index,
                        Entry::AssistantMessage(AssistantMessageEntry::default()),
                    );
                    let Some(Entry::AssistantMessage(entry)) = self.entries.get_mut(index) else {
                        unreachable!()
                    };
                    entry
                };
                entry.sync(message);
            }
        };
    }

    fn set_entry(&mut self, index: usize, entry: Entry) {
        if index == self.entries.len() {
            self.entries.push(entry);
        } else {
            self.entries[index] = entry;
        }
    }

    pub fn remove(&mut self, range: Range<usize>) {
        self.entries.drain(range);
    }

    pub fn settings_changed(&mut self, cx: &mut App) {
        for entry in self.entries.iter() {
            match entry {
                Entry::UserMessage { .. } | Entry::AssistantMessage { .. } => {}
                Entry::Content(response_views) => {
                    for view in response_views.values() {
                        if let Ok(diff_editor) = view.clone().downcast::<Editor>() {
                            diff_editor.update(cx, |diff_editor, cx| {
                                diff_editor.set_text_style_refinement(
                                    diff_editor_text_style_refinement(cx),
                                );
                                cx.notify();
                            })
                        }
                    }
                }
            }
        }
    }
}

impl EventEmitter<EntryViewEvent> for EntryViewState {}

pub struct EntryViewEvent {
    pub entry_index: usize,
    pub view_event: ViewEvent,
}

pub enum ViewEvent {
    NewDiff(ToolCallId),
    NewTerminal(ToolCallId),
    TerminalMovedToBackground(ToolCallId),
    MessageEditorEvent(Entity<MessageEditor>, MessageEditorEvent),
}

#[derive(Default, Debug)]
pub struct AssistantMessageEntry {
    scroll_handles_by_chunk_index: HashMap<usize, ScrollHandle>,
}

impl AssistantMessageEntry {
    pub fn scroll_handle_for_chunk(&self, ix: usize) -> Option<ScrollHandle> {
        self.scroll_handles_by_chunk_index.get(&ix).cloned()
    }

    pub fn sync(&mut self, message: &acp_thread::AssistantMessage) {
        if let Some(acp_thread::AssistantMessageChunk::Thought { .. }) = message.chunks.last() {
            let ix = message.chunks.len() - 1;
            let handle = self.scroll_handles_by_chunk_index.entry(ix).or_default();
            handle.scroll_to_bottom();
        }
    }
}

#[derive(Debug)]
pub enum Entry {
    UserMessage(Entity<MessageEditor>),
    AssistantMessage(AssistantMessageEntry),
    Content(HashMap<EntityId, AnyEntity>),
}

impl Entry {
    pub fn focus_handle(&self, cx: &App) -> Option<FocusHandle> {
        match self {
            Self::UserMessage(editor) => Some(editor.read(cx).focus_handle(cx)),
            Self::AssistantMessage(_) | Self::Content(_) => None,
        }
    }

    pub fn message_editor(&self) -> Option<&Entity<MessageEditor>> {
        match self {
            Self::UserMessage(editor) => Some(editor),
            Self::AssistantMessage(_) | Self::Content(_) => None,
        }
    }

    pub fn editor_for_diff(&self, diff: &Entity<acp_thread::Diff>) -> Option<Entity<Editor>> {
        self.content_map()?
            .get(&diff.entity_id())
            .cloned()
            .map(|entity| entity.downcast::<Editor>().unwrap())
    }

    pub fn terminal(
        &self,
        terminal: &Entity<acp_thread::Terminal>,
    ) -> Option<Entity<TerminalView>> {
        self.content_map()?
            .get(&terminal.entity_id())
            .cloned()
            .map(|entity| entity.downcast::<TerminalView>().unwrap())
    }

    pub fn scroll_handle_for_assistant_message_chunk(
        &self,
        chunk_ix: usize,
    ) -> Option<ScrollHandle> {
        match self {
            Self::AssistantMessage(message) => message.scroll_handle_for_chunk(chunk_ix),
            Self::UserMessage(_) | Self::Content(_) => None,
        }
    }

    fn content_map(&self) -> Option<&HashMap<EntityId, AnyEntity>> {
        match self {
            Self::Content(map) => Some(map),
            _ => None,
        }
    }

    fn empty() -> Self {
        Self::Content(HashMap::default())
    }

    #[cfg(test)]
    pub fn has_content(&self) -> bool {
        match self {
            Self::Content(map) => !map.is_empty(),
            Self::UserMessage(_) | Self::AssistantMessage(_) => false,
        }
    }
}

fn create_terminal(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    terminal: Entity<acp_thread::Terminal>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<TerminalView> {
    cx.new(|cx| {
        let mut view = TerminalView::new(
            terminal.read(cx).inner().clone(),
            workspace.clone(),
            None,
            project.downgrade(),
            window,
            cx,
        );
        view.set_embedded_mode(Some(1000), cx);
        view
    })
}

fn create_editor_diff(
    diff: Entity<acp_thread::Diff>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Editor> {
    cx.new(|cx| {
        let mut editor = Editor::new(
            EditorMode::Full {
                scale_ui_elements_with_buffer_font_size: false,
                show_active_line_background: false,
                sized_by_content: true,
            },
            diff.read(cx).multibuffer().clone(),
            None,
            window,
            cx,
        );
        editor.set_show_gutter(false, cx);
        editor.disable_inline_diagnostics();
        editor.disable_expand_excerpt_buttons(cx);
        editor.set_show_vertical_scrollbar(false, cx);
        editor.set_minimap_visibility(MinimapVisibility::Disabled, window, cx);
        editor.set_soft_wrap_mode(SoftWrap::None, cx);
        editor.scroll_manager.set_forbid_vertical_scroll(true);
        editor.set_show_indent_guides(false, cx);
        editor.set_read_only(true);
        editor.set_show_breakpoints(false, cx);
        editor.set_show_code_actions(false, cx);
        editor.set_show_git_diff_gutter(false, cx);
        editor.set_expand_all_diff_hunks(cx);
        editor.set_text_style_refinement(diff_editor_text_style_refinement(cx));
        editor
    })
}

fn diff_editor_text_style_refinement(cx: &mut App) -> TextStyleRefinement {
    TextStyleRefinement {
        font_size: Some(
            TextSize::Small
                .rems(cx)
                .to_pixels(ThemeSettings::get_global(cx).agent_font_size(cx))
                .into(),
        ),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, rc::Rc};

    use acp_thread::{AgentConnection, StubAgentConnection};
    use agent_client_protocol as acp;
    use agent_settings::AgentSettings;
    use agent2::HistoryStore;
    use assistant_context::ContextStore;
    use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
    use editor::{EditorSettings, RowInfo};
    use fs::FakeFs;
    use gpui::{AppContext as _, SemanticVersion, TestAppContext};

    use crate::acp::entry_view_state::EntryViewState;
    use multi_buffer::MultiBufferRow;
    use pretty_assertions::assert_matches;
    use project::Project;
    use serde_json::json;
    use settings::{Settings as _, SettingsStore};
    use theme::ThemeSettings;
    use util::path;
    use workspace::Workspace;

    #[gpui::test]
    async fn test_diff_sync(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "hello.txt": "hi world"
            }),
        )
        .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let tool_call = acp::ToolCall {
            id: acp::ToolCallId("tool".into()),
            title: "Tool call".into(),
            kind: acp::ToolKind::Other,
            status: acp::ToolCallStatus::InProgress,
            content: vec![acp::ToolCallContent::Diff {
                diff: acp::Diff {
                    path: "/project/hello.txt".into(),
                    old_text: Some("hi world".into()),
                    new_text: "hello world".into(),
                },
            }],
            locations: vec![],
            raw_input: None,
            raw_output: None,
        };
        let connection = Rc::new(StubAgentConnection::new());
        let thread = cx
            .update(|_, cx| {
                connection
                    .clone()
                    .new_thread(project.clone(), Path::new(path!("/project")), cx)
            })
            .await
            .unwrap();
        let session_id = thread.update(cx, |thread, _| thread.session_id().clone());

        cx.update(|_, cx| {
            connection.send_update(session_id, acp::SessionUpdate::ToolCall(tool_call), cx)
        });

        let context_store = cx.new(|cx| ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));

        let view_state = cx.new(|_cx| {
            EntryViewState::new(
                workspace.downgrade(),
                project.clone(),
                history_store,
                None,
                Default::default(),
                false,
            )
        });

        view_state.update_in(cx, |view_state, window, cx| {
            view_state.sync_entry(0, &thread, window, cx)
        });

        let diff = thread.read_with(cx, |thread, _cx| {
            thread
                .entries()
                .get(0)
                .unwrap()
                .diffs()
                .next()
                .unwrap()
                .clone()
        });

        cx.run_until_parked();

        let diff_editor = view_state.read_with(cx, |view_state, _cx| {
            view_state.entry(0).unwrap().editor_for_diff(&diff).unwrap()
        });
        assert_eq!(
            diff_editor.read_with(cx, |editor, cx| editor.text(cx)),
            "hi world\nhello world"
        );
        let row_infos = diff_editor.read_with(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            multibuffer
                .snapshot(cx)
                .row_infos(MultiBufferRow(0))
                .collect::<Vec<_>>()
        });
        assert_matches!(
            row_infos.as_slice(),
            [
                RowInfo {
                    multibuffer_row: Some(MultiBufferRow(0)),
                    diff_status: Some(DiffHunkStatus {
                        kind: DiffHunkStatusKind::Deleted,
                        ..
                    }),
                    ..
                },
                RowInfo {
                    multibuffer_row: Some(MultiBufferRow(1)),
                    diff_status: Some(DiffHunkStatus {
                        kind: DiffHunkStatusKind::Added,
                        ..
                    }),
                    ..
                }
            ]
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AgentSettings::register(cx);
            workspace::init_settings(cx);
            ThemeSettings::register(cx);
            release_channel::init(SemanticVersion::default(), cx);
            EditorSettings::register(cx);
        });
    }
}
