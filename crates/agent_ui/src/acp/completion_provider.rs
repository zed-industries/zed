use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use agent::context_store::ContextStore;
use anyhow::Result;
use collections::HashMap;
use editor::display_map::CreaseId;
use editor::{CompletionProvider, Editor, ExcerptId, ToOffset as _};
use file_icons::FileIcons;
use fuzzy::StringMatch;
use gpui::{App, Entity, Task, WeakEntity};
use itertools::Itertools;
use language::{Buffer, CodeLabel, HighlightId};
use lsp::CompletionContext;
use project::{Completion, CompletionIntent, CompletionResponse, ProjectPath, Symbol, WorktreeId};
use rope::Point;
use text::{Anchor, OffsetRangeExt, ToPoint};
use ui::prelude::*;
use util::ResultExt as _;
use workspace::Workspace;

use agent::context::{AgentContextHandle, AgentContextKey};

use crate::context_picker::MentionLink;
use crate::context_picker::file_context_picker::{self, FileMatch, search_files};
use crate::message_editor::ContextCreasesAddon;

pub struct Mention {
    range: Range<editor::Anchor>,
    path: PathBuf,
}

pub struct MentionSet {
    mentions_by_crease_id: HashMap<CreaseId, Mention>,
}

pub struct ContextPickerCompletionProvider {
    workspace: WeakEntity<Workspace>,
    editor: WeakEntity<Editor>,
    mention_set: Rc<RefCell<MentionSet>>,
}

impl ContextPickerCompletionProvider {
    pub fn new(
        mention_set: Rc<RefCell<MentionSet>>,
        workspace: WeakEntity<Workspace>,
        editor: WeakEntity<Editor>,
    ) -> Self {
        Self {
            mention_set,
            workspace,
            editor,
        }
    }

    fn completion_for_path(
        project_path: ProjectPath,
        path_prefix: &str,
        is_recent: bool,
        is_directory: bool,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        cx: &App,
    ) -> Completion {
        let (file_name, directory) =
            file_context_picker::extract_file_name_and_directory(&project_path.path, path_prefix);

        let label =
            build_code_label_for_full_path(&file_name, directory.as_ref().map(|s| s.as_ref()), cx);
        let full_path = if let Some(directory) = directory {
            format!("{}{}", directory, file_name)
        } else {
            file_name.to_string()
        };

        let crease_icon_path = if is_directory {
            FileIcons::get_folder_icon(false, cx).unwrap_or_else(|| IconName::Folder.path().into())
        } else {
            FileIcons::get_icon(Path::new(&full_path), cx)
                .unwrap_or_else(|| IconName::File.path().into())
        };
        let completion_icon_path = if is_recent {
            IconName::HistoryRerun.path().into()
        } else {
            crease_icon_path.clone()
        };

        let new_text = format!("{} ", MentionLink::for_file(&file_name, &full_path));
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(completion_icon_path),
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                crease_icon_path,
                file_name,
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                context_store.clone(),
                move |_, cx| {
                    if is_directory {
                        Task::ready(
                            context_store
                                .update(cx, |context_store, cx| {
                                    context_store.add_directory(&project_path, false, cx)
                                })
                                .log_err()
                                .flatten(),
                        )
                    } else {
                        let result = context_store.update(cx, |context_store, cx| {
                            context_store.add_file_from_path(project_path.clone(), false, cx)
                        });
                        cx.spawn(async move |_| result.await.log_err().flatten())
                    }
                },
            )),
        }
    }
}

fn build_code_label_for_full_path(file_name: &str, directory: Option<&str>, cx: &App) -> CodeLabel {
    let comment_id = cx.theme().syntax().highlight_id("comment").map(HighlightId);
    let mut label = CodeLabel::default();

    label.push_str(&file_name, None);
    label.push_str(" ", None);

    if let Some(directory) = directory {
        label.push_str(&directory, comment_id);
    }

    label.filter_range = 0..label.text().len();

    label
}

impl CompletionProvider for ContextPickerCompletionProvider {
    fn completions(
        &self,
        excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let state = buffer.update(cx, |buffer, _cx| {
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let offset_to_line = buffer.point_to_offset(line_start);
            let mut lines = buffer.text_for_range(line_start..position).lines();
            let line = lines.next()?;
            MentionCompletion::try_parse(line, offset_to_line)
        });
        let Some(state) = state else {
            return Task::ready(Ok(Vec::new()));
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let editor = self.editor.clone();
        let http_client = workspace.read(cx).client().http_client();

        let MentionCompletion { argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let search_task = search_files(query.clone(), Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn(async move |_, cx| {
            let matches = search_task.await;
            let Some(editor) = editor.upgrade() else {
                return Ok(Vec::new());
            };

            let completions = cx.update(|cx| {
                matches
                    .into_iter()
                    .map(|mat| {
                        let path_match = &mat.mat;
                        let project_path = ProjectPath {
                            worktree_id: WorktreeId::from_usize(path_match.worktree_id),
                            path: path_match.path.clone(),
                        };

                        Self::completion_for_path(
                            project_path,
                            &path_match.path_prefix,
                            mat.is_recent,
                            path_match.is_dir,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            cx,
                        )
                    })
                    .collect()
            })?;

            Ok(vec![CompletionResponse {
                completions,
                // Since this does its own filtering (see `filter_completions()` returns false),
                // there is no benefit to computing whether this set of completions is incomplete.
                is_incomplete: true,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _menu_is_open: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let buffer = buffer.read(cx);
        let position = position.to_point(buffer);
        let line_start = Point::new(position.row, 0);
        let offset_to_line = buffer.point_to_offset(line_start);
        let mut lines = buffer.text_for_range(line_start..position).lines();
        if let Some(line) = lines.next() {
            MentionCompletion::try_parse(line, offset_to_line)
                .map(|completion| {
                    completion.source_range.start <= offset_to_line + position.column as usize
                        && completion.source_range.end >= offset_to_line + position.column as usize
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn sort_completions(&self) -> bool {
        false
    }

    fn filter_completions(&self) -> bool {
        false
    }
}

fn confirm_completion_callback(
    crease_icon_path: SharedString,
    crease_text: SharedString,
    excerpt_id: ExcerptId,
    start: Anchor,
    content_len: usize,
    editor: Entity<Editor>,
    context_store: Entity<ContextStore>,
    add_context_fn: impl Fn(&mut Window, &mut App) -> Task<Option<AgentContextHandle>>
    + Send
    + Sync
    + 'static,
) -> Arc<dyn Fn(CompletionIntent, &mut Window, &mut App) -> bool + Send + Sync> {
    Arc::new(move |_, window, cx| {
        let context = add_context_fn(window, cx);

        let crease_text = crease_text.clone();
        let crease_icon_path = crease_icon_path.clone();
        let editor = editor.clone();
        let context_store = context_store.clone();
        window.defer(cx, move |window, cx| {
            let crease_id = crate::context_picker::insert_crease_for_mention(
                excerpt_id,
                start,
                content_len,
                crease_text.clone(),
                crease_icon_path,
                editor.clone(),
                window,
                cx,
            );
            cx.spawn(async move |cx| {
                let crease_id = crease_id?;
                let context = context.await?;
                editor
                    .update(cx, |editor, cx| {
                        if let Some(addon) = editor.addon_mut::<ContextCreasesAddon>() {
                            addon.add_creases(
                                &context_store,
                                AgentContextKey(context),
                                [(crease_id, crease_text)],
                                cx,
                            );
                        }
                    })
                    .ok()
            })
            .detach();
        });
        false
    })
}

#[derive(Debug, Default, PartialEq)]
struct MentionCompletion {
    source_range: Range<usize>,
    argument: Option<String>,
}

impl MentionCompletion {
    fn try_parse(line: &str, offset_to_line: usize) -> Option<Self> {
        let last_mention_start = line.rfind('@')?;
        if last_mention_start >= line.len() {
            return Some(Self::default());
        }
        if last_mention_start > 0
            && line
                .chars()
                .nth(last_mention_start - 1)
                .map_or(false, |c| !c.is_whitespace())
        {
            return None;
        }

        let rest_of_line = &line[last_mention_start + 1..];
        let mut argument = None;

        let mut parts = rest_of_line.split_whitespace();
        let mut end = last_mention_start + 1;
        if let Some(argument_text) = parts.next() {
            end += argument_text.len();
            argument = Some(argument_text.to_string());
        }

        Some(Self {
            source_range: last_mention_start + offset_to_line..end + offset_to_line,
            argument,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::AnchorRangeExt;
    use gpui::{EventEmitter, FocusHandle, Focusable, TestAppContext, VisualTestContext};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{ops::Deref, rc::Rc};
    use util::path;
    use workspace::{AppState, Item};

    #[test]
    fn test_mention_completion_parse() {
        assert_eq!(MentionCompletion::try_parse("Lorem Ipsum", 0), None);

        assert_eq!(
            MentionCompletion::try_parse("Lorem @", 0),
            Some(MentionCompletion {
                source_range: 6..7,
                mode: None,
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file", 0),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: Some(ContextPickerMode::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file ", 0),
            Some(MentionCompletion {
                source_range: 6..12,
                mode: Some(ContextPickerMode::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs ", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs Ipsum", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @main", 0),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: None,
                argument: Some("main".to_string()),
            })
        );

        assert_eq!(MentionCompletion::try_parse("test@", 0), None);
    }

    struct AtMentionEditor(Entity<Editor>);

    impl Item for AtMentionEditor {
        type Event = ();

        fn include_in_nav_history() -> bool {
            false
        }

        fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
            "Test".into()
        }
    }

    impl EventEmitter<()> for AtMentionEditor {}

    impl Focusable for AtMentionEditor {
        fn focus_handle(&self, cx: &App) -> FocusHandle {
            self.0.read(cx).focus_handle(cx).clone()
        }
    }

    impl Render for AtMentionEditor {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0.clone().into_any_element()
        }
    }

    #[gpui::test]
    async fn test_context_completion_provider(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "editor": "",
                    "a": {
                        "one.txt": "",
                        "two.txt": "",
                        "three.txt": "",
                        "four.txt": ""
                    },
                    "b": {
                        "five.txt": "",
                        "six.txt": "",
                        "seven.txt": "",
                        "eight.txt": "",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let worktree = project.update(cx, |project, cx| {
            let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            worktrees.pop().unwrap()
        });
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let paths = vec![
            path!("a/one.txt"),
            path!("a/two.txt"),
            path!("a/three.txt"),
            path!("a/four.txt"),
            path!("b/five.txt"),
            path!("b/six.txt"),
            path!("b/seven.txt"),
            path!("b/eight.txt"),
        ];

        let mut opened_editors = Vec::new();
        for path in paths {
            let buffer = workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.open_path(
                        ProjectPath {
                            worktree_id,
                            path: Path::new(path).into(),
                        },
                        None,
                        false,
                        window,
                        cx,
                    )
                })
                .await
                .unwrap();
            opened_editors.push(buffer);
        }

        let editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let editor = cx.new(|cx| {
                Editor::new(
                    editor::EditorMode::full(),
                    multi_buffer::MultiBuffer::build_simple("", cx),
                    None,
                    window,
                    cx,
                )
            });
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(
                    Box::new(cx.new(|_| AtMentionEditor(editor.clone()))),
                    true,
                    true,
                    None,
                    window,
                    cx,
                );
            });
            editor
        });

        let context_store = cx.new(|_| ContextStore::new(project.downgrade(), None));

        let editor_entity = editor.downgrade();
        editor.update_in(&mut cx, |editor, window, cx| {
            let last_opened_buffer = opened_editors.last().and_then(|editor| {
                editor
                    .downcast::<Editor>()?
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .as_ref()
                    .map(Entity::downgrade)
            });
            window.focus(&editor.focus_handle(cx));
            editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
                workspace.downgrade(),
                context_store.downgrade(),
                None,
                None,
                editor_entity,
                last_opened_buffer,
            ))));
        });

        cx.simulate_input("Lorem ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem ");
            assert!(!editor.has_visible_completions_menu());
        });

        cx.simulate_input("@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @");
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                current_completion_labels(editor),
                &[
                    "seven.txt dir/b/",
                    "six.txt dir/b/",
                    "five.txt dir/b/",
                    "four.txt dir/a/",
                    "Files & Directories",
                    "Symbols",
                    "Fetch"
                ]
            );
        });

        // Select and confirm "File"
        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @file ");
            assert!(editor.has_visible_completions_menu());
        });

        cx.simulate_input("one");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @file one");
            assert!(editor.has_visible_completions_menu());
            assert_eq!(current_completion_labels(editor), vec!["one.txt dir/a/"]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](@file:dir/a/one.txt) ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](@file:dir/a/one.txt)  ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt)  Ipsum ",
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt)  Ipsum @file ",
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt)  Ipsum [@seven.txt](@file:dir/b/seven.txt) "
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 45)..Point::new(0, 80)
                ]
            );
        });

        cx.simulate_input("\n@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt)  Ipsum [@seven.txt](@file:dir/b/seven.txt) \n@"
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 45)..Point::new(0, 80)
                ]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt)  Ipsum [@seven.txt](@file:dir/b/seven.txt) \n[@six.txt](@file:dir/b/six.txt) "
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 45)..Point::new(0, 80),
                    Point::new(1, 0)..Point::new(1, 31)
                ]
            );
        });
    }

    fn fold_ranges(editor: &Editor, cx: &mut App) -> Vec<Range<Point>> {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.display_map.update(cx, |display_map, cx| {
            display_map
                .snapshot(cx)
                .folds_in_range(0..snapshot.len())
                .map(|fold| fold.range.to_point(&snapshot))
                .collect()
        })
    }

    fn current_completion_labels(editor: &Editor) -> Vec<String> {
        let completions = editor.current_completions().expect("Missing completions");
        completions
            .into_iter()
            .map(|completion| completion.label.text.to_string())
            .collect::<Vec<_>>()
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            client::init_settings(cx);
            language::init(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
            editor::init_settings(cx);
        });
    }
}
