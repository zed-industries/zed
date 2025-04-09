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

use crate::context_picker::file_context_picker::search_files;
use crate::context_picker::symbol_context_picker::search_symbols;
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;

use super::fetch_context_picker::fetch_url_content;
use super::file_context_picker::FileMatch;
use super::symbol_context_picker::SymbolMatch;
use super::thread_context_picker::{ThreadContextEntry, ThreadMatch, search_threads};
use super::{
    ContextPickerMode, MentionLink, RecentEntry, recent_context_picker_entries,
    supported_context_picker_modes,
};

pub(crate) enum Match {
    Symbol(SymbolMatch),
    File(FileMatch),
    Thread(ThreadMatch),
    Fetch(SharedString),
    Mode(ContextPickerMode),
}

fn search(
    mode: Option<ContextPickerMode>,
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    recent_entries: Vec<RecentEntry>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    workspace: Entity<Workspace>,
    cx: &mut App,
) -> Task<Vec<Match>> {
    match mode {
        Some(ContextPickerMode::File) => {
            let search_files_task =
                search_files(query.clone(), cancellation_flag.clone(), &workspace, cx);
            cx.background_spawn(async move {
                search_files_task
                    .await
                    .into_iter()
                    .map(Match::File)
                    .collect()
            })
        }
        Some(ContextPickerMode::Symbol) => {
            let search_symbols_task =
                search_symbols(query.clone(), cancellation_flag.clone(), &workspace, cx);
            cx.background_spawn(async move {
                search_symbols_task
                    .await
                    .into_iter()
                    .map(Match::Symbol)
                    .collect()
            })
        }
        Some(ContextPickerMode::Thread) => {
            if let Some(thread_store) = thread_store.as_ref().and_then(|t| t.upgrade()) {
                let search_threads_task =
                    search_threads(query.clone(), cancellation_flag.clone(), thread_store, cx);
                cx.background_spawn(async move {
                    search_threads_task
                        .await
                        .into_iter()
                        .map(Match::Thread)
                        .collect()
                })
            } else {
                Task::ready(Vec::new())
            }
        }
        Some(ContextPickerMode::Fetch) => {
            if !query.is_empty() {
                Task::ready(vec![Match::Fetch(query.into())])
            } else {
                Task::ready(Vec::new())
            }
        }
        None => {
            if query.is_empty() {
                let mut matches = recent_entries
                    .into_iter()
                    .map(|entry| match entry {
                        super::RecentEntry::File {
                            project_path,
                            path_prefix,
                        } => Match::File(FileMatch {
                            mat: fuzzy::PathMatch {
                                score: 1.,
                                positions: Vec::new(),
                                worktree_id: project_path.worktree_id.to_usize(),
                                path: project_path.path,
                                path_prefix,
                                is_dir: false,
                                distance_to_relative_ancestor: 0,
                            },
                            is_recent: true,
                        }),
                        super::RecentEntry::Thread(thread_context_entry) => {
                            Match::Thread(ThreadMatch {
                                thread: thread_context_entry,
                                is_recent: true,
                            })
                        }
                    })
                    .collect::<Vec<_>>();

                matches.extend(
                    supported_context_picker_modes(&thread_store)
                        .into_iter()
                        .map(Match::Mode),
                );

                Task::ready(matches)
            } else {
                let search_files_task =
                    search_files(query.clone(), cancellation_flag.clone(), &workspace, cx);
                cx.background_spawn(async move {
                    search_files_task
                        .await
                        .into_iter()
                        .map(Match::File)
                        .collect()
                })
            }
        }
    }
}

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

    fn completion_for_mode(source_range: Range<Anchor>, mode: ContextPickerMode) -> Completion {
        Completion {
            replace_range: source_range.clone(),
            new_text: format!("@{} ", mode.mention_prefix()),
            label: CodeLabel::plain(mode.label().to_string(), None),
            icon_path: Some(mode.icon().path().into()),
            documentation: None,
            source: project::CompletionSource::Custom,
            insert_text_mode: None,
            // This ensures that when a user accepts this completion, the
            // completion menu will still be shown after "@category " is
            // inserted
            confirm: Some(Arc::new(|_, _, _| true)),
        }
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
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(thread_entry.summary.to_string(), None),
            documentation: None,
            insert_text_mode: None,
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
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(url_to_fetch.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::Globe.path().into()),
            insert_text_mode: None,
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

        let new_text = MentionLink::for_file(&file_name, &full_path);
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
            replace_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::Code.path().into()),
            insert_text_mode: None,
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

        let Some((workspace, context_store)) =
            self.workspace.upgrade().zip(self.context_store.upgrade())
        else {
            return Task::ready(Ok(None));
        };

        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_before(state.source_range.end);

        let thread_store = self.thread_store.clone();
        let editor = self.editor.clone();
        let http_client = workspace.read(cx).client().http_client().clone();

        let MentionCompletion { mode, argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let recent_entries = recent_context_picker_entries(
            context_store.clone(),
            thread_store.clone(),
            workspace.clone(),
            cx,
        );

        let search_task = search(
            mode,
            query,
            Arc::<AtomicBool>::default(),
            recent_entries,
            thread_store.clone(),
            workspace.clone(),
            cx,
        );

        cx.spawn(async move |_, cx| {
            let matches = search_task.await;
            let Some(editor) = editor.upgrade() else {
                return Ok(None);
            };

            Ok(Some(cx.update(|cx| {
                matches
                    .into_iter()
                    .filter_map(|mat| match mat {
                        Match::File(FileMatch { mat, is_recent }) => {
                            Some(Self::completion_for_path(
                                ProjectPath {
                                    worktree_id: WorktreeId::from_usize(mat.worktree_id),
                                    path: mat.path.clone(),
                                },
                                &mat.path_prefix,
                                is_recent,
                                mat.is_dir,
                                excerpt_id,
                                source_range.clone(),
                                editor.clone(),
                                context_store.clone(),
                                cx,
                            ))
                        }
                        Match::Symbol(SymbolMatch { symbol, .. }) => Self::completion_for_symbol(
                            symbol,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            context_store.clone(),
                            workspace.clone(),
                            cx,
                        ),
                        Match::Thread(ThreadMatch {
                            thread, is_recent, ..
                        }) => {
                            let thread_store = thread_store.as_ref().and_then(|t| t.upgrade())?;
                            Some(Self::completion_for_thread(
                                thread,
                                excerpt_id,
                                source_range.clone(),
                                is_recent,
                                editor.clone(),
                                context_store.clone(),
                                thread_store,
                            ))
                        }
                        Match::Fetch(url) => Some(Self::completion_for_fetch(
                            source_range.clone(),
                            url,
                            excerpt_id,
                            editor.clone(),
                            context_store.clone(),
                            http_client.clone(),
                        )),
                        Match::Mode(mode) => {
                            Some(Self::completion_for_mode(source_range.clone(), mode))
                        }
                    })
                    .collect()
            })?))
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

            if let Some(parsed_mode) = ContextPickerMode::try_from(mode_text).ok() {
                mode = Some(parsed_mode);
            } else {
                argument = Some(mode_text.to_string());
            }
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
    use gpui::{EventEmitter, FocusHandle, Focusable, TestAppContext, VisualTestContext};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::SettingsStore;
    use std::ops::Deref;
    use util::{path, separator};
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

        let editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let editor = cx.new(|cx| {
                Editor::new(
                    editor::EditorMode::Full,
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
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@seven.txt](@file:dir/b/seven.txt)"
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 79)
                ]
            );
        });

        cx.simulate_input("\n@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@seven.txt](@file:dir/b/seven.txt)\n@"
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 79)
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
                "Lorem [@one.txt](@file:dir/a/one.txt) Ipsum [@seven.txt](@file:dir/b/seven.txt)\n[@six.txt](@file:dir/b/six.txt)"
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                crease_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 37),
                    Point::new(0, 44)..Point::new(0, 79),
                    Point::new(1, 0)..Point::new(1, 31)
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
