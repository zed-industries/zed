use std::{collections::HashMap, ops::Range};

use acp_thread::AcpThread;
use editor::{Editor, EditorMode, MinimapVisibility, MultiBuffer};
use gpui::{
    AnyEntity, App, AppContext as _, Entity, EntityId, TextStyleRefinement, WeakEntity, Window,
};
use language::language_settings::SoftWrap;
use settings::Settings as _;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::TextSize;
use workspace::Workspace;

#[derive(Default)]
pub struct EntryViewState {
    entries: Vec<Entry>,
}

impl EntryViewState {
    pub fn entry(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index)
    }

    pub fn sync_entry(
        &mut self,
        workspace: WeakEntity<Workspace>,
        thread: Entity<AcpThread>,
        index: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        debug_assert!(index <= self.entries.len());
        let entry = if let Some(entry) = self.entries.get_mut(index) {
            entry
        } else {
            self.entries.push(Entry::default());
            self.entries.last_mut().unwrap()
        };

        entry.sync_diff_multibuffers(&thread, index, window, cx);
        entry.sync_terminals(&workspace, &thread, index, window, cx);
    }

    pub fn remove(&mut self, range: Range<usize>) {
        self.entries.drain(range);
    }

    pub fn settings_changed(&mut self, cx: &mut App) {
        for entry in self.entries.iter() {
            for view in entry.views.values() {
                if let Ok(diff_editor) = view.clone().downcast::<Editor>() {
                    diff_editor.update(cx, |diff_editor, cx| {
                        diff_editor
                            .set_text_style_refinement(diff_editor_text_style_refinement(cx));
                        cx.notify();
                    })
                }
            }
        }
    }
}

pub struct Entry {
    views: HashMap<EntityId, AnyEntity>,
}

impl Entry {
    pub fn editor_for_diff(&self, diff: &Entity<MultiBuffer>) -> Option<Entity<Editor>> {
        self.views
            .get(&diff.entity_id())
            .cloned()
            .map(|entity| entity.downcast::<Editor>().unwrap())
    }

    pub fn terminal(
        &self,
        terminal: &Entity<acp_thread::Terminal>,
    ) -> Option<Entity<TerminalView>> {
        self.views
            .get(&terminal.entity_id())
            .cloned()
            .map(|entity| entity.downcast::<TerminalView>().unwrap())
    }

    fn sync_diff_multibuffers(
        &mut self,
        thread: &Entity<AcpThread>,
        index: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(entry) = thread.read(cx).entries().get(index) else {
            return;
        };

        let multibuffers = entry
            .diffs()
            .map(|diff| diff.read(cx).multibuffer().clone());

        let multibuffers = multibuffers.collect::<Vec<_>>();

        for multibuffer in multibuffers {
            if self.views.contains_key(&multibuffer.entity_id()) {
                return;
            }

            let editor = cx.new(|cx| {
                let mut editor = Editor::new(
                    EditorMode::Full {
                        scale_ui_elements_with_buffer_font_size: false,
                        show_active_line_background: false,
                        sized_by_content: true,
                    },
                    multibuffer.clone(),
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
            });

            let entity_id = multibuffer.entity_id();
            self.views.insert(entity_id, editor.into_any());
        }
    }

    fn sync_terminals(
        &mut self,
        workspace: &WeakEntity<Workspace>,
        thread: &Entity<AcpThread>,
        index: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(entry) = thread.read(cx).entries().get(index) else {
            return;
        };

        let terminals = entry
            .terminals()
            .map(|terminal| terminal.clone())
            .collect::<Vec<_>>();

        for terminal in terminals {
            if self.views.contains_key(&terminal.entity_id()) {
                return;
            }

            let Some(strong_workspace) = workspace.upgrade() else {
                return;
            };

            let terminal_view = cx.new(|cx| {
                let mut view = TerminalView::new(
                    terminal.read(cx).inner().clone(),
                    workspace.clone(),
                    None,
                    strong_workspace.read(cx).project().downgrade(),
                    window,
                    cx,
                );
                view.set_embedded_mode(Some(1000), cx);
                view
            });

            let entity_id = terminal.entity_id();
            self.views.insert(entity_id, terminal_view.into_any());
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.views.len()
    }
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

impl Default for Entry {
    fn default() -> Self {
        Self {
            // Avoid allocating in the heap by default
            views: HashMap::with_capacity(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, rc::Rc};

    use acp_thread::{AgentConnection, StubAgentConnection};
    use agent_client_protocol as acp;
    use agent_settings::AgentSettings;
    use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
    use editor::{EditorSettings, RowInfo};
    use fs::FakeFs;
    use gpui::{SemanticVersion, TestAppContext};
    use multi_buffer::MultiBufferRow;
    use pretty_assertions::assert_matches;
    use project::Project;
    use serde_json::json;
    use settings::{Settings as _, SettingsStore};
    use theme::ThemeSettings;
    use util::path;
    use workspace::Workspace;

    use crate::acp::entry_view_state::EntryViewState;

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
                    .new_thread(project, Path::new(path!("/project")), cx)
            })
            .await
            .unwrap();
        let session_id = thread.update(cx, |thread, _| thread.session_id().clone());

        cx.update(|_, cx| {
            connection.send_update(session_id, acp::SessionUpdate::ToolCall(tool_call), cx)
        });

        let mut view_state = EntryViewState::default();
        cx.update(|window, cx| {
            view_state.sync_entry(workspace.downgrade(), thread.clone(), 0, window, cx);
        });

        let multibuffer = thread.read_with(cx, |thread, cx| {
            thread
                .entries()
                .get(0)
                .unwrap()
                .diffs()
                .next()
                .unwrap()
                .read(cx)
                .multibuffer()
                .clone()
        });

        cx.run_until_parked();

        let entry = view_state.entry(0).unwrap();
        let diff_editor = entry.editor_for_diff(&multibuffer).unwrap();
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
