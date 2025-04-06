use std::cell::RefCell;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Result;
use editor::{CompletionProvider, Editor, ExcerptId};
use file_icons::FileIcons;
use gpui::{App, Entity, Task, WeakEntity};
use http_client::HttpClientWithUrl;
use language::{Buffer, CodeLabel, HighlightId};
use lsp::CompletionContext;
use project::{Completion, CompletionIntent, ProjectPath, Symbol, WorktreeId};
use rope::Point;
use text::{Anchor, ToPoint};
use ui::prelude::*;
use workspace::Workspace;

use crate::context::AssistantContext;
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;

use super::fetch_context_picker::fetch_url_content;
use super::thread_context_picker::ThreadContextEntry;
use super::{
    ContextPickerMode, MentionLink, recent_context_picker_entries, supported_context_picker_modes,
};

pub struct ContextPickerCompletionProvider {
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    editor: WeakEntity<Editor>,
}

impl ContextPickerCompletionProvider {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        editor: WeakEntity<Editor>,
    ) -> Self {
        Self {
            workspace,
            context_store,
            thread_store,
            editor,
        }
    }

    fn default_completions(
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        context_store: Entity<ContextStore>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        editor: Entity<Editor>,
        workspace: Entity<Workspace>,
        cx: &App,
    ) -> Vec<Completion> {
        let mut completions = Vec::new();

        completions.extend(
            recent_context_picker_entries(
                context_store.clone(),
                thread_store.clone(),
                workspace.clone(),
                cx,
            )
            .iter()
            .filter_map(|entry| match entry {
                super::RecentEntry::File {
                    project_path,
                    path_prefix,
                } => Some(Self::completion_for_path(
                    project_path.clone(),
                    path_prefix,
                    true,
                    false,
                    excerpt_id,
                    source_range.clone(),
                    editor.clone(),
                    context_store.clone(),
                    cx,
                )),
                super::RecentEntry::Thread(thread_context_entry) => {
                    let thread_store = thread_store
                        .as_ref()
                        .and_then(|thread_store| thread_store.upgrade())?;
                    Some(Self::completion_for_thread(
                        thread_context_entry.clone(),
                        excerpt_id,
                        source_range.clone(),
                        true,
                        editor.clone(),
                        context_store.clone(),
                        thread_store,
                    ))
                }
            }),
        );

        completions.extend(
            supported_context_picker_modes(&thread_store)
                .iter()
                .map(|mode| {
                    Completion {
                        old_range: source_range.clone(),
                        new_text: format!("@{} ", mode.mention_prefix()),
                        label: CodeLabel::plain(mode.label().to_string(), None),
                        icon_path: Some(mode.icon().path().into()),
                        documentation: None,
                        source: project::CompletionSource::Custom,
                        // This ensures that when a user accepts this completion, the
                        // completion menu will still be shown after "@category " is
                        // inserted
                        confirm: Some(Arc::new(|_, _, _| true)),
                    }
                }),
        );
        completions
    }

    fn build_code_label_for_full_path(
        file_name: &str,
        directory: Option<&str>,
        cx: &App,
    ) -> CodeLabel {
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

    fn completion_for_thread(
        thread_entry: ThreadContextEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        recent: bool,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        thread_store: Entity<ThreadStore>,
    ) -> Completion {
        let icon_for_completion = if recent {
            IconName::HistoryRerun
        } else {
            IconName::MessageBubbles
        };
        let new_text = MentionLink::for_thread(&thread_entry);
        let new_text_len = new_text.len();
        Completion {
            old_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(thread_entry.summary.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_for_completion.path().into()),
            confirm: Some(confirm_completion_callback(
                IconName::MessageBubbles.path().into(),
                thread_entry.summary.clone(),
                excerpt_id,
                source_range.start,
                new_text_len,
                editor.clone(),
                move |cx| {
                    let thread_id = thread_entry.id.clone();
                    let context_store = context_store.clone();
                    let thread_store = thread_store.clone();
                    cx.spawn(async move |cx| {
                        let thread = thread_store
                            .update(cx, |thread_store, cx| {
                                thread_store.open_thread(&thread_id, cx)
                            })?
                            .await?;
                        context_store.update(cx, |context_store, cx| {
                            context_store.add_thread(thread, false, cx)
                        })
                    })
                    .detach_and_log_err(cx);
                },
            )),
        }
    }

    fn completion_for_fetch(
        source_range: Range<Anchor>,
        url_to_fetch: SharedString,
        excerpt_id: ExcerptId,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        http_client: Arc<HttpClientWithUrl>,
    ) -> Completion {
        let new_text = MentionLink::for_fetch(&url_to_fetch);
        let new_text_len = new_text.len();
        Completion {
            old_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(url_to_fetch.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::Globe.path().into()),
            confirm: Some(confirm_completion_callback(
                IconName::Globe.path().into(),
                url_to_fetch.clone(),
                excerpt_id,
                source_range.start,
                new_text_len,
                editor.clone(),
                move |cx| {
                    let context_store = context_store.clone();
                    let http_client = http_client.clone();
                    let url_to_fetch = url_to_fetch.clone();
                    cx.spawn(async move |cx| {
                        if context_store.update(cx, |context_store, _| {
                            context_store.includes_url(&url_to_fetch).is_some()
                        })? {
                            return Ok(());
                        }
                        let content = cx
                            .background_spawn(fetch_url_content(
                                http_client,
                                url_to_fetch.to_string(),
                            ))
                            .await?;
                        context_store.update(cx, |context_store, cx| {
                            context_store.add_fetched_url(url_to_fetch.to_string(), content, cx)
                        })
                    })
                    .detach_and_log_err(cx);
                },
            )),
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
        context_store: Entity<ContextStore>,
        cx: &App,
    ) -> Completion {
        let (file_name, directory) = super::file_context_picker::extract_file_name_and_directory(
            &project_path.path,
            path_prefix,
        );

        let label = Self::build_code_label_for_full_path(
            &file_name,
            directory.as_ref().map(|s| s.as_ref()),
            cx,
        );
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

        let new_text = MentionLink::for_file(&file_name, &full_path);
        let new_text_len = new_text.len();
        Completion {
            old_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(completion_icon_path),
            confirm: Some(confirm_completion_callback(
                crease_icon_path,
                file_name,
                excerpt_id,
                source_range.start,
                new_text_len,
                editor,
                move |cx| {
                    context_store.update(cx, |context_store, cx| {
                        let task = if is_directory {
                            context_store.add_directory(project_path.clone(), false, cx)
                        } else {
                            context_store.add_file_from_path(project_path.clone(), false, cx)
                        };
                        task.detach_and_log_err(cx);
                    })
                },
            )),
        }
    }

    fn completion_for_symbol(
        symbol: Symbol,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        workspace: Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        let path_prefix = workspace
            .read(cx)
            .project()
            .read(cx)
            .worktree_for_id(symbol.path.worktree_id, cx)?
            .read(cx)
            .root_name();

        let (file_name, directory) = super::file_context_picker::extract_file_name_and_directory(
            &symbol.path.path,
            path_prefix,
        );
        let full_path = if let Some(directory) = directory {
            format!("{}{}", directory, file_name)
        } else {
            file_name.to_string()
        };

        let comment_id = cx.theme().syntax().highlight_id("comment").map(HighlightId);
        let mut label = CodeLabel::plain(symbol.name.clone(), None);
        label.push_str(" ", None);
        label.push_str(&file_name, comment_id);

        let new_text = MentionLink::for_symbol(&symbol.name, &full_path);
        let new_text_len = new_text.len();
        Some(Completion {
            old_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::Code.path().into()),
            confirm: Some(confirm_completion_callback(
                IconName::Code.path().into(),
                symbol.name.clone().into(),
                excerpt_id,
                source_range.start,
                new_text_len,
                editor.clone(),
                move |cx| {
                    let symbol = symbol.clone();
                    let context_store = context_store.clone();
                    let workspace = workspace.clone();
                    super::symbol_context_picker::add_symbol(
                        symbol.clone(),
                        false,
                        workspace.clone(),
                        context_store.downgrade(),
                        cx,
                    )
                    .detach_and_log_err(cx);
                },
            )),
        })
    }
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
    ) -> Task<Result<Option<Vec<Completion>>>> {
        let state = buffer.update(cx, |buffer, _cx| {
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let offset_to_line = buffer.point_to_offset(line_start);
            let mut lines = buffer.text_for_range(line_start..position).lines();
            let line = lines.next()?;
            MentionCompletion::try_parse(line, offset_to_line)
        });
        let Some(state) = state else {
            return Task::ready(Ok(None));
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Ok(None));
        };
        let Some(context_store) = self.context_store.upgrade() else {
            return Task::ready(Ok(None));
        };

        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_before(state.source_range.end);

        let thread_store = self.thread_store.clone();
        let editor = self.editor.clone();
        let http_client = workspace.read(cx).client().http_client().clone();

        cx.spawn(async move |_, cx| {
            let mut completions = Vec::new();

            let MentionCompletion { mode, argument, .. } = state;

            let query = argument.unwrap_or_else(|| "".to_string());
            match mode {
                Some(ContextPickerMode::File) => {
                    let path_matches = cx
                        .update(|cx| {
                            super::file_context_picker::search_paths(
                                query,
                                Arc::<AtomicBool>::default(),
                                &workspace,
                                cx,
                            )
                        })?
                        .await;

                    if let Some(editor) = editor.upgrade() {
                        completions.reserve(path_matches.len());
                        cx.update(|cx| {
                            completions.extend(path_matches.iter().map(|mat| {
                                Self::completion_for_path(
                                    ProjectPath {
                                        worktree_id: WorktreeId::from_usize(mat.worktree_id),
                                        path: mat.path.clone(),
                                    },
                                    &mat.path_prefix,
                                    false,
                                    mat.is_dir,
                                    excerpt_id,
                                    source_range.clone(),
                                    editor.clone(),
                                    context_store.clone(),
                                    cx,
                                )
                            }));
                        })?;
                    }
                }
                Some(ContextPickerMode::Symbol) => {
                    if let Some(editor) = editor.upgrade() {
                        let symbol_matches = cx
                            .update(|cx| {
                                super::symbol_context_picker::search_symbols(
                                    query,
                                    Arc::new(AtomicBool::default()),
                                    &workspace,
                                    cx,
                                )
                            })?
                            .await?;
                        cx.update(|cx| {
                            completions.extend(symbol_matches.into_iter().filter_map(
                                |(_, symbol)| {
                                    Self::completion_for_symbol(
                                        symbol,
                                        excerpt_id,
                                        source_range.clone(),
                                        editor.clone(),
                                        context_store.clone(),
                                        workspace.clone(),
                                        cx,
                                    )
                                },
                            ));
                        })?;
                    }
                }
                Some(ContextPickerMode::Fetch) => {
                    if let Some(editor) = editor.upgrade() {
                        if !query.is_empty() {
                            completions.push(Self::completion_for_fetch(
                                source_range.clone(),
                                query.into(),
                                excerpt_id,
                                editor.clone(),
                                context_store.clone(),
                                http_client.clone(),
                            ));
                        }

                        context_store.update(cx, |store, _| {
                            let urls = store.context().iter().filter_map(|context| {
                                if let AssistantContext::FetchedUrl(context) = context {
                                    Some(context.url.clone())
                                } else {
                                    None
                                }
                            });
                            for url in urls {
                                completions.push(Self::completion_for_fetch(
                                    source_range.clone(),
                                    url,
                                    excerpt_id,
                                    editor.clone(),
                                    context_store.clone(),
                                    http_client.clone(),
                                ));
                            }
                        })?;
                    }
                }
                Some(ContextPickerMode::Thread) => {
                    if let Some((thread_store, editor)) = thread_store
                        .and_then(|thread_store| thread_store.upgrade())
                        .zip(editor.upgrade())
                    {
                        let threads = cx
                            .update(|cx| {
                                super::thread_context_picker::search_threads(
                                    query,
                                    thread_store.clone(),
                                    cx,
                                )
                            })?
                            .await;
                        for thread in threads {
                            completions.push(Self::completion_for_thread(
                                thread.clone(),
                                excerpt_id,
                                source_range.clone(),
                                false,
                                editor.clone(),
                                context_store.clone(),
                                thread_store.clone(),
                            ));
                        }
                    }
                }
                None => {
                    cx.update(|cx| {
                        if let Some(editor) = editor.upgrade() {
                            completions.extend(Self::default_completions(
                                excerpt_id,
                                source_range.clone(),
                                context_store.clone(),
                                thread_store.clone(),
                                editor,
                                workspace.clone(),
                                cx,
                            ));
                        }
                    })?;
                }
            }
            Ok(Some(completions))
        })
    }

    fn resolve_completions(
        &self,
        _buffer: Entity<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(true))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        _: &str,
        _: bool,
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
    add_context_fn: impl Fn(&mut App) -> () + Send + Sync + 'static,
) -> Arc<dyn Fn(CompletionIntent, &mut Window, &mut App) -> bool + Send + Sync> {
    Arc::new(move |_, window, cx| {
        add_context_fn(cx);

        let crease_text = crease_text.clone();
        let crease_icon_path = crease_icon_path.clone();
        let editor = editor.clone();
        window.defer(cx, move |window, cx| {
            crate::context_picker::insert_crease_for_mention(
                excerpt_id,
                start,
                content_len,
                crease_text,
                crease_icon_path,
                editor,
                window,
                cx,
            );
        });
        false
    })
}

#[derive(Debug, Default, PartialEq)]
struct MentionCompletion {
    source_range: Range<usize>,
    mode: Option<ContextPickerMode>,
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

        let mut mode = None;
        let mut argument = None;

        let mut parts = rest_of_line.split_whitespace();
        let mut end = last_mention_start + 1;
        if let Some(mode_text) = parts.next() {
            end += mode_text.len();
            mode = ContextPickerMode::try_from(mode_text).ok();
            match rest_of_line[mode_text.len()..].find(|c: char| !c.is_whitespace()) {
                Some(whitespace_count) => {
                    if let Some(argument_text) = parts.next() {
                        argument = Some(argument_text.to_string());
                        end += whitespace_count + argument_text.len();
                    }
                }
                None => {
                    // Rest of line is entirely whitespace
                    end += rest_of_line.len() - mode_text.len();
                }
            }
        }

        Some(Self {
            source_range: last_mention_start + offset_to_line..end + offset_to_line,
            mode,
            argument,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Focusable, TestAppContext, VisualTestContext};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{ops::Deref, path::PathBuf};
    use util::{path, separator};
    use workspace::AppState;

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

        assert_eq!(MentionCompletion::try_parse("test@", 0), None);
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
        let worktree_id = worktree.update(cx, |worktree, _| worktree.id());

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let paths = vec![
            separator!("a/one.txt"),
            separator!("a/two.txt"),
            separator!("a/three.txt"),
            separator!("a/four.txt"),
            separator!("b/five.txt"),
            separator!("b/six.txt"),
            separator!("b/seven.txt"),
        ];
        for path in paths {
            workspace
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
        }

        let item = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(
                    ProjectPath {
                        worktree_id,
                        path: PathBuf::from("editor").into(),
                    },
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .expect("Could not open test file");

        let editor = cx.update(|_, cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });

        let context_store = cx.new(|_| ContextStore::new(workspace.downgrade(), None));

        let editor_entity = editor.downgrade();
        editor.update_in(&mut cx, |editor, window, cx| {
            window.focus(&editor.focus_handle(cx));
            editor.set_completion_provider(Some(Box::new(ContextPickerCompletionProvider::new(
                workspace.downgrade(),
                context_store.downgrade(),
                None,
                editor_entity,
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
                    "editor dir/",
                    "seven.txt dir/b/",
                    "six.txt dir/b/",
                    "five.txt dir/b/",
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
            assert_eq!(editor.text(cx), "Lorem [@one.txt](@file:dir/a/one.txt)",);
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](@file:dir/a/one.txt) ",);
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum ",
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 37)]
            );
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum @file ",
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
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
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@editor](@file:dir/editor)"
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 71)
                ]
            );
        });

        cx.simulate_input("\n@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@editor](@file:dir/editor)\n@"
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 71)
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
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@editor](@file:dir/editor)\n[@seven.txt](@file:dir/b/seven.txt)"
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 71),
                    Point::new(1, 0)..Point::new(1, 35)
                ]
            );
        });
    }

    fn crease_ranges(editor: &Editor, cx: &mut App) -> Vec<Range<Point>> {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.display_map.update(cx, |display_map, cx| {
            display_map
                .snapshot(cx)
                .crease_snapshot
                .crease_items_with_offsets(&snapshot)
                .into_iter()
                .map(|(_, range)| range)
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
