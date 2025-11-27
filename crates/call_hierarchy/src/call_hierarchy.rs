use std::ops::Range;
use std::sync::Arc;

use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, AsyncWindowContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    HighlightStyle, ParentElement, Render, Styled, StyledText, Task, TextStyle, WeakEntity, Window,
    actions, rems,
};
use language::{PointUtf16, ToPointUtf16, Unclipped};
use picker::{Picker, PickerDelegate};
use project::{CallHierarchyItem, Project};
use settings::Settings;
use theme::ThemeSettings;
use ui::{ListItem, ListItemSpacing, Tooltip, prelude::*, vh};
use util::{ResultExt, paths::PathExt};
use workspace::{ModalView, Workspace};

actions!(call_hierarchy, [ShowIncomingCalls, ShowOutgoingCalls]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CallHierarchyMode {
    #[default]
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub item: CallHierarchyItem,
    pub target: Unclipped<PointUtf16>,
}

pub async fn fetch_calls(
    item: &CallHierarchyItem,
    project: &Entity<Project>,
    buffer: &Entity<language::Buffer>,
    mode: CallHierarchyMode,
    cx: &mut AsyncWindowContext,
) -> Vec<Call> {
    match mode {
        CallHierarchyMode::Incoming => {
            let task = project.update(cx, |project, cx| {
                project.incoming_calls(buffer, item.clone(), cx)
            });
            let Some(task) = task.log_err() else {
                return Vec::new();
            };
            let Some(calls) = task.await.log_err().flatten() else {
                return Vec::new();
            };
            calls
                .into_iter()
                .map(|c| Call {
                    target: c
                        .from_ranges
                        .first()
                        .map_or(c.from.selection_range.start, |r| r.start),
                    item: c.from,
                })
                .collect()
        }
        CallHierarchyMode::Outgoing => {
            let task = project.update(cx, |project, cx| {
                project.outgoing_calls(buffer, item.clone(), cx)
            });
            let Some(task) = task.log_err() else {
                return Vec::new();
            };
            let Some(calls) = task.await.log_err().flatten() else {
                return Vec::new();
            };
            calls
                .into_iter()
                .map(|c| Call {
                    target: c.to.selection_range.start,
                    item: c.to,
                })
                .collect()
        }
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(CallHierarchyView::register).detach();
}

fn toggle_call_hierarchy(
    editor: Entity<Editor>,
    mode: CallHierarchyMode,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = window.root::<Workspace>().flatten() else {
        return;
    };

    let project = workspace.read(cx).project().clone();
    let workspace_weak = workspace.downgrade();

    workspace.update(cx, |workspace, cx| {
        workspace.toggle_modal(window, cx, |window, cx| {
            CallHierarchyView::new(editor, project, workspace_weak, mode, window, cx)
        });
    });
}

pub struct CallHierarchyView {
    picker: Entity<Picker<CallHierarchyDelegate>>,
    mode: CallHierarchyMode,
}

impl CallHierarchyView {
    fn register(editor: &mut Editor, _: Option<&mut Window>, cx: &mut Context<Editor>) {
        if editor.mode().is_full() {
            let handle = cx.entity().downgrade();
            editor
                .register_action({
                    let handle = handle.clone();
                    move |_: &ShowIncomingCalls, window, cx| {
                        if let Some(editor) = handle.upgrade() {
                            toggle_call_hierarchy(editor, CallHierarchyMode::Incoming, window, cx);
                        }
                    }
                })
                .detach();
            editor
                .register_action(move |_: &ShowOutgoingCalls, window, cx| {
                    if let Some(editor) = handle.upgrade() {
                        toggle_call_hierarchy(editor, CallHierarchyMode::Outgoing, window, cx);
                    }
                })
                .detach();
        }
    }

    fn new(
        editor: Entity<Editor>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        mode: CallHierarchyMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CallHierarchyView {
        let delegate =
            CallHierarchyDelegate::new(cx.entity().downgrade(), editor, project, workspace, mode);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).max_height(Some(vh(0.75, window)))
        });

        let picker_entity = picker.clone();
        cx.spawn_in(window, async move |_view, cx| {
            picker_entity
                .update_in(cx, |picker, window, cx| {
                    picker.delegate.fetch_calls(window, cx)
                })?
                .await;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        CallHierarchyView { picker, mode }
    }
}

impl Focusable for CallHierarchyView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for CallHierarchyView {}

impl ModalView for CallHierarchyView {}

impl Render for CallHierarchyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(34.))
            .when(self.mode == CallHierarchyMode::Incoming, |this| {
                this.on_action(cx.listener(|_this, _: &ShowIncomingCalls, _window, cx| {
                    cx.emit(DismissEvent);
                }))
            })
            .when(self.mode == CallHierarchyMode::Outgoing, |this| {
                this.on_action(cx.listener(|_this, _: &ShowOutgoingCalls, _window, cx| {
                    cx.emit(DismissEvent);
                }))
            })
            .child(self.picker.clone())
    }
}

pub struct CallHierarchyDelegate {
    view: WeakEntity<CallHierarchyView>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    editor: Entity<Editor>,
    mode: CallHierarchyMode,
    root_item: Option<CallHierarchyItem>,
    calls: Vec<Call>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl CallHierarchyDelegate {
    fn new(
        view: WeakEntity<CallHierarchyView>,
        editor: Entity<Editor>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        mode: CallHierarchyMode,
    ) -> Self {
        Self {
            view,
            workspace,
            project,
            editor,
            mode,
            root_item: None,
            calls: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn fetch_calls(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) -> Task<()> {
        let buffer = self.editor.read(cx).buffer().read(cx).as_singleton();
        let Some(buffer) = buffer else {
            return Task::ready(());
        };

        let position = self.editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            editor
                .selections
                .newest::<language::Point>(&snapshot)
                .head()
        });
        let position_utf16 = position.to_point_utf16(&buffer.read(cx).snapshot());

        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(&buffer, position_utf16, cx)
        });

        let project = self.project.clone();
        let buffer_clone = buffer.clone();
        let mode = self.mode;

        cx.spawn_in(window, async move |picker, mut cx| {
            let items = prepare_task.await;
            let Ok(Some(items)) = items else {
                return;
            };
            let Some(root_item) = items.into_iter().next() else {
                return;
            };

            let calls = fetch_calls(&root_item, &project, &buffer_clone, mode, &mut cx).await;

            picker
                .update_in(cx, |picker, _window, cx| {
                    picker.delegate.root_item = Some(root_item);
                    picker.delegate.calls = calls;
                    picker.delegate.matches = picker
                        .delegate
                        .calls
                        .iter()
                        .enumerate()
                        .map(|(index, _)| StringMatch {
                            candidate_id: index,
                            score: Default::default(),
                            positions: Default::default(),
                            string: Default::default(),
                        })
                        .collect();
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .log_err();
        })
    }
}

impl PickerDelegate for CallHierarchyDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.mode {
            CallHierarchyMode::Incoming => "Search incoming calls...".into(),
            CallHierarchyMode::Outgoing => "Search outgoing calls...".into(),
        }
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let candidates: Vec<StringMatchCandidate> = self
            .calls
            .iter()
            .enumerate()
            .map(|(id, call)| StringMatchCandidate::new(id, &call.item.name))
            .collect();

        if query.is_empty() {
            self.matches = self
                .calls
                .iter()
                .enumerate()
                .map(|(index, _)| StringMatch {
                    candidate_id: index,
                    score: Default::default(),
                    positions: Default::default(),
                    string: Default::default(),
                })
                .collect();
            self.selected_index = 0;
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                true,
                100,
                &Default::default(),
                executor,
            )
            .await;

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(call) = self.calls.get(mat.candidate_id) else {
            return;
        };

        let uri = call.item.uri.clone();
        let start_point = call.target;

        let abs_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => return,
        };

        let buffer_task = self
            .project
            .update(cx, |project, cx| project.open_local_buffer(&abs_path, cx));
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |_, cx| {
            let buffer = buffer_task.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let position = buffer.read(cx).clip_point_utf16(start_point, Bias::Left);
                let pane = if secondary {
                    workspace.adjacent_pane(window, cx)
                } else {
                    workspace.active_pane().clone()
                };

                let editor =
                    workspace.open_project_item::<Editor>(pane, buffer, true, true, window, cx);

                editor.update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([position..position]),
                    );
                });
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.view.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let call = self.calls.get(mat.candidate_id)?;

        let match_ranges = mat.positions.iter().map(|pos| *pos..*pos + 1);
        let (name_styled, detail_styled, path_styled) = render_item(call, match_ranges, cx);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .text_ui(cx)
                        .child(name_styled)
                        .when_some(detail_styled, |this, detail| this.child(detail)),
                )
                .when_some(path_styled, |this, path| {
                    this.tooltip(Tooltip::element(move |_window, _cx| {
                        div().max_w_72().child(path.clone()).into_any_element()
                    }))
                }),
        )
    }
}

/// Extracts display information from a `Call` for rendering in UI.
fn extract_call_display_info(call: &Call) -> (String, Option<String>, Option<String>) {
    let name = call.item.name.clone();
    let line_number = call.target.0.row + 1;
    let file_path = call.item.uri.to_file_path().ok();

    let file_with_line = file_path
        .as_ref()
        .map(|p| format!("{}:{}", p.compact().to_string_lossy(), line_number));

    let detail = call
        .item
        .detail
        .as_ref()
        .map_or(file_with_line.clone(), |d| Some(d.replace('\n', "↵")));

    (name, detail, file_with_line)
}

/// Renders call item name and detail as styled text, matching the outline panel's style.
/// Returns a tuple of (name_styled, detail_styled) where:
/// - name_styled: function-colored text with match highlight backgrounds
/// - detail_styled: muted-colored text (if detail is provided)
pub fn render_item(
    call_item: &Call,
    match_ranges: impl IntoIterator<Item = Range<usize>>,
    cx: &App,
) -> (StyledText, Option<StyledText>, Option<String>) {
    let (name, detail, path) = extract_call_display_info(call_item);

    let settings = ThemeSettings::get_global(cx);

    let base_text_style = TextStyle {
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };

    let function_color = cx
        .theme()
        .syntax()
        .get("function")
        .color
        .unwrap_or(cx.theme().colors().text);

    let highlight_style = HighlightStyle {
        background_color: Some(cx.theme().colors().text_accent.alpha(0.3)),
        ..Default::default()
    };

    let mut name_style = base_text_style.clone();
    name_style.color = function_color;

    let name_styled = StyledText::new(name).with_default_highlights(
        &name_style,
        match_ranges.into_iter().map(|r| (r, highlight_style)),
    );

    let detail_styled = detail.map(|d| {
        let mut detail_style = base_text_style;
        detail_style.color = cx.theme().colors().text_muted;
        StyledText::new(d).with_default_highlights(&detail_style, std::iter::empty())
    });

    (name_styled, detail_styled, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt as _;
    use gpui::TestAppContext;
    use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use project::{CallHierarchyItem, FakeFs};
    use serde_json::json;
    use std::sync::Arc;
    use util::{path, rel_path::rel_path};
    use workspace::AppState;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let _state = AppState::test(cx);
            init(cx);
            editor::init(cx);
        });
    }

    fn rust_lang() -> Arc<Language> {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        ))
    }

    fn make_lsp_call_hierarchy_item(
        name: &str,
        uri: lsp::Uri,
        line: u32,
    ) -> lsp::CallHierarchyItem {
        lsp::CallHierarchyItem {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            detail: Some(format!("fn {name}()")),
            uri,
            range: lsp::Range {
                start: lsp::Position { line, character: 0 },
                end: lsp::Position {
                    line,
                    character: 10,
                },
            },
            selection_range: lsp::Range {
                start: lsp::Position { line, character: 3 },
                end: lsp::Position {
                    line,
                    character: 3 + name.len() as u32,
                },
            },
            data: None,
        }
    }

    fn make_call(name: &str, uri: lsp::Uri, line: u32, detail: Option<String>) -> Call {
        Call {
            item: CallHierarchyItem {
                name: name.to_string(),
                kind: lsp::SymbolKind::FUNCTION,
                detail,
                uri,
                range: Unclipped(PointUtf16::new(line, 0))..Unclipped(PointUtf16::new(line, 10)),
                selection_range: Unclipped(PointUtf16::new(line, 3))
                    ..Unclipped(PointUtf16::new(line, 3 + name.len() as u32)),
                data: None,
            },
            target: Unclipped(PointUtf16::new(line, 3)),
        }
    }

    #[gpui::test]
    async fn test_extract_call_display_info_basic(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"main.rs": ""}}))
            .await;

        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        let call = make_call("my_function", test_uri, 10, None);
        let (name, detail, file_with_line) = extract_call_display_info(&call);

        #[cfg(not(windows))]
        let expected_path = "/test/src/main.rs:11";
        #[cfg(windows)]
        let expected_path = "C:\\test\\src\\main.rs:11";

        assert_eq!(name, "my_function");
        assert_eq!(detail.as_deref(), Some(expected_path));
        assert_eq!(file_with_line.as_deref(), Some(expected_path));
    }

    #[gpui::test]
    async fn test_extract_call_display_info_home_path_compacted(cx: &mut TestAppContext) {
        init_test(cx);

        let home_dir = util::paths::home_dir();
        let test_path = home_dir.join("projects/app/src/main.rs");

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            &home_dir,
            json!({"projects": {"app": {"src": {"main.rs": ""}}}}),
        )
        .await;

        let test_uri = lsp::Uri::from_file_path(&test_path).unwrap();

        let call = make_call("my_function", test_uri, 10, None);
        let (name, detail, file_with_line) = extract_call_display_info(&call);

        #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
        let expected_path = "~/projects/app/src/main.rs:11";
        #[cfg(windows)]
        let expected_path = format!("{}:11", test_path.to_string_lossy());

        assert_eq!(name, "my_function");
        assert_eq!(detail.as_deref(), Some(expected_path));
        assert_eq!(file_with_line.as_deref(), Some(expected_path));
    }

    #[gpui::test]
    async fn test_extract_call_display_info_with_detail(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"lib.rs": ""}}))
            .await;

        let test_uri = lsp::Uri::from_file_path(path!("/test/src/lib.rs")).unwrap();

        let call = make_call(
            "helper",
            test_uri,
            5,
            Some("fn helper() -> i32".to_string()),
        );
        let (name, detail, file_with_line) = extract_call_display_info(&call);

        #[cfg(not(windows))]
        let expected_path = "/test/src/lib.rs:6";
        #[cfg(windows)]
        let expected_path = "C:\\test\\src\\lib.rs:6";

        assert_eq!(name, "helper");
        assert_eq!(detail.as_deref(), Some("fn helper() -> i32"));
        assert_eq!(file_with_line.as_deref(), Some(expected_path));
    }

    #[gpui::test]
    async fn test_extract_call_display_info_multiline_detail(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": ""})).await;

        let test_uri = lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap();

        let call = make_call("foo", test_uri, 0, Some("line1\nline2\nline3".to_string()));
        let (name, detail, _) = extract_call_display_info(&call);

        assert_eq!(name, "foo");
        assert_eq!(detail.as_deref(), Some("line1↵line2↵line3"));
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_basic(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "main.rs": "fn main() { helper(); }\nfn helper() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let mut fake_servers = project.read_with(cx, |project, _| {
            project.languages().register_fake_lsp(
                "Rust",
                FakeLspAdapter {
                    capabilities: lsp::ServerCapabilities {
                        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(
                            true,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/src/main.rs"), cx)
            })
            .await
            .unwrap();

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/main.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                        to: make_lsp_call_hierarchy_item("helper", uri, 1),
                        from_ranges: vec![],
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor().run_until_parked();

        let has_modal = workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<CallHierarchyView>(cx).is_some()
        });
        assert!(has_modal, "Call hierarchy modal should be open");
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_incoming_mode(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "main.rs": "fn main() { helper(); }\nfn helper() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let mut fake_servers = project.read_with(cx, |project, _| {
            project.languages().register_fake_lsp(
                "Rust",
                FakeLspAdapter {
                    capabilities: lsp::ServerCapabilities {
                        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(
                            true,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/src/main.rs"), cx)
            })
            .await
            .unwrap();

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/main.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: vec![lsp::Range {
                            start: lsp::Position {
                                line: 0,
                                character: 12,
                            },
                            end: lsp::Position {
                                line: 0,
                                character: 18,
                            },
                        }],
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let has_modal = workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<CallHierarchyView>(cx).is_some()
        });
        assert!(
            has_modal,
            "Call hierarchy modal should be open in incoming mode"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_filtering(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({
                "src": {
                    "main.rs": "fn main() { foo(); bar(); baz(); }\nfn foo() {}\nfn bar() {}\nfn baz() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let mut fake_servers = project.read_with(cx, |project, _| {
            project.languages().register_fake_lsp(
                "Rust",
                FakeLspAdapter {
                    capabilities: lsp::ServerCapabilities {
                        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(
                            true,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/src/main.rs"), cx)
            })
            .await
            .unwrap();

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/main.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("foo", uri.clone(), 1),
                            from_ranges: vec![],
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("bar_1", uri.clone(), 2),
                            from_ranges: vec![],
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("bar_2", uri.clone(), 2),
                            from_ranges: vec![],
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("baz", uri, 3),
                            from_ranges: vec![],
                        },
                    ]))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor().run_until_parked();

        let modal = workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<CallHierarchyView>(cx)
        });
        assert!(modal.is_some(), "Modal should be open");

        let modal = modal.unwrap();
        let match_count_before =
            modal.read_with(cx, |view, cx| view.picker.read(cx).delegate.matches.len());
        assert_eq!(match_count_before, 4, "Should have 4 matches initially");

        let task = modal.update_in(cx, |view, window, cx| {
            view.picker.update(cx, |picker, cx| {
                picker
                    .delegate
                    .update_matches("bar".to_string(), window, cx)
            })
        });
        task.await;
        cx.executor().run_until_parked();

        let match_count_after =
            modal.read_with(cx, |view, cx| view.picker.read(cx).delegate.matches.len());
        assert_eq!(
            match_count_after, 2,
            "Filter 'bar' should match 2 entries (bar, baz)"
        );
    }
}
