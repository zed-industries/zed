use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use acp_thread::MentionUri;
use anyhow::Result;
use editor::{CompletionProvider, Editor, ExcerptId};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, Task, WeakEntity};
use language::{Buffer, CodeLabel, HighlightId};
use lsp::CompletionContext;
use project::{
    Completion, CompletionIntent, CompletionResponse, Project, ProjectPath, Symbol, WorktreeId,
};
use prompt_store::PromptStore;
use rope::Point;
use text::{Anchor, ToPoint as _};
use ui::prelude::*;
use workspace::Workspace;

use agent::thread_store::{TextThreadStore, ThreadStore};

use crate::acp::message_editor::MessageEditor;
use crate::context_picker::file_context_picker::{FileMatch, search_files};
use crate::context_picker::rules_context_picker::{RulesContextEntry, search_rules};
use crate::context_picker::symbol_context_picker::SymbolMatch;
use crate::context_picker::symbol_context_picker::search_symbols;
use crate::context_picker::thread_context_picker::{
    ThreadContextEntry, ThreadMatch, search_threads,
};
use crate::context_picker::{
    ContextPickerAction, ContextPickerEntry, ContextPickerMode, RecentEntry,
    available_context_picker_entries, recent_context_picker_entries, selection_ranges,
};

pub(crate) enum Match {
    File(FileMatch),
    Symbol(SymbolMatch),
    Thread(ThreadMatch),
    Fetch(SharedString),
    Rules(RulesContextEntry),
    Entry(EntryMatch),
}

pub struct EntryMatch {
    mat: Option<StringMatch>,
    entry: ContextPickerEntry,
}

impl Match {
    pub fn score(&self) -> f64 {
        match self {
            Match::File(file) => file.mat.score,
            Match::Entry(mode) => mode.mat.as_ref().map(|mat| mat.score).unwrap_or(1.),
            Match::Thread(_) => 1.,
            Match::Symbol(_) => 1.,
            Match::Rules(_) => 1.,
            Match::Fetch(_) => 1.,
        }
    }
}

fn search(
    mode: Option<ContextPickerMode>,
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    recent_entries: Vec<RecentEntry>,
    prompt_store: Option<Entity<PromptStore>>,
    thread_store: WeakEntity<ThreadStore>,
    text_thread_context_store: WeakEntity<assistant_context::ContextStore>,
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
            if let Some((thread_store, context_store)) = thread_store
                .upgrade()
                .zip(text_thread_context_store.upgrade())
            {
                let search_threads_task = search_threads(
                    query.clone(),
                    cancellation_flag.clone(),
                    thread_store,
                    context_store,
                    cx,
                );
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

        Some(ContextPickerMode::Rules) => {
            if let Some(prompt_store) = prompt_store.as_ref() {
                let search_rules_task =
                    search_rules(query.clone(), cancellation_flag.clone(), prompt_store, cx);
                cx.background_spawn(async move {
                    search_rules_task
                        .await
                        .into_iter()
                        .map(Match::Rules)
                        .collect::<Vec<_>>()
                })
            } else {
                Task::ready(Vec::new())
            }
        }

        None => {
            if query.is_empty() {
                let mut matches = recent_entries
                    .into_iter()
                    .map(|entry| match entry {
                        RecentEntry::File {
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
                        RecentEntry::Thread(thread_context_entry) => Match::Thread(ThreadMatch {
                            thread: thread_context_entry,
                            is_recent: true,
                        }),
                    })
                    .collect::<Vec<_>>();

                matches.extend(
                    available_context_picker_entries(
                        &prompt_store,
                        &Some(thread_store.clone()),
                        &workspace,
                        cx,
                    )
                    .into_iter()
                    .map(|mode| {
                        Match::Entry(EntryMatch {
                            entry: mode,
                            mat: None,
                        })
                    }),
                );

                Task::ready(matches)
            } else {
                let executor = cx.background_executor().clone();

                let search_files_task =
                    search_files(query.clone(), cancellation_flag.clone(), &workspace, cx);

                let entries = available_context_picker_entries(
                    &prompt_store,
                    &Some(thread_store.clone()),
                    &workspace,
                    cx,
                );
                let entry_candidates = entries
                    .iter()
                    .enumerate()
                    .map(|(ix, entry)| StringMatchCandidate::new(ix, entry.keyword()))
                    .collect::<Vec<_>>();

                cx.background_spawn(async move {
                    let mut matches = search_files_task
                        .await
                        .into_iter()
                        .map(Match::File)
                        .collect::<Vec<_>>();

                    let entry_matches = fuzzy::match_strings(
                        &entry_candidates,
                        &query,
                        false,
                        true,
                        100,
                        &Arc::new(AtomicBool::default()),
                        executor,
                    )
                    .await;

                    matches.extend(entry_matches.into_iter().map(|mat| {
                        Match::Entry(EntryMatch {
                            entry: entries[mat.candidate_id],
                            mat: Some(mat),
                        })
                    }));

                    matches.sort_by(|a, b| {
                        b.score()
                            .partial_cmp(&a.score())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    matches
                })
            }
        }
    }
}

pub struct ContextPickerCompletionProvider {
    workspace: WeakEntity<Workspace>,
    thread_store: WeakEntity<ThreadStore>,
    text_thread_store: WeakEntity<TextThreadStore>,
    message_editor: WeakEntity<MessageEditor>,
}

impl ContextPickerCompletionProvider {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        thread_store: WeakEntity<ThreadStore>,
        text_thread_store: WeakEntity<TextThreadStore>,
        message_editor: WeakEntity<MessageEditor>,
    ) -> Self {
        Self {
            workspace,
            thread_store,
            text_thread_store,
            message_editor,
        }
    }

    fn completion_for_entry(
        entry: ContextPickerEntry,
        source_range: Range<Anchor>,
        message_editor: WeakEntity<MessageEditor>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        match entry {
            ContextPickerEntry::Mode(mode) => Some(Completion {
                replace_range: source_range.clone(),
                new_text: format!("@{} ", mode.keyword()),
                label: CodeLabel::plain(mode.label().to_string(), None),
                icon_path: Some(mode.icon().path().into()),
                documentation: None,
                source: project::CompletionSource::Custom,
                insert_text_mode: None,
                // This ensures that when a user accepts this completion, the
                // completion menu will still be shown after "@category " is
                // inserted
                confirm: Some(Arc::new(|_, _, _| true)),
            }),
            ContextPickerEntry::Action(action) => {
                let (new_text, on_action) = match action {
                    ContextPickerAction::AddSelections => {
                        const PLACEHOLDER: &str = "selection ";
                        let selections = selection_ranges(workspace, cx)
                            .into_iter()
                            .enumerate()
                            .map(|(ix, (buffer, range))| {
                                (
                                    buffer,
                                    range,
                                    (PLACEHOLDER.len() * ix)..(PLACEHOLDER.len() * (ix + 1) - 1),
                                )
                            })
                            .collect::<Vec<_>>();

                        let new_text: String = PLACEHOLDER.repeat(selections.len());

                        let callback = Arc::new({
                            let source_range = source_range.clone();
                            move |_, window: &mut Window, cx: &mut App| {
                                let selections = selections.clone();
                                let message_editor = message_editor.clone();
                                let source_range = source_range.clone();
                                window.defer(cx, move |window, cx| {
                                    message_editor
                                        .update(cx, |message_editor, cx| {
                                            message_editor.confirm_mention_for_selection(
                                                source_range,
                                                selections,
                                                window,
                                                cx,
                                            )
                                        })
                                        .ok();
                                });
                                false
                            }
                        });

                        (new_text, callback)
                    }
                };

                Some(Completion {
                    replace_range: source_range.clone(),
                    new_text,
                    label: CodeLabel::plain(action.label().to_string(), None),
                    icon_path: Some(action.icon().path().into()),
                    documentation: None,
                    source: project::CompletionSource::Custom,
                    insert_text_mode: None,
                    // This ensures that when a user accepts this completion, the
                    // completion menu will still be shown after "@category " is
                    // inserted
                    confirm: Some(on_action),
                })
            }
        }
    }

    fn completion_for_thread(
        thread_entry: ThreadContextEntry,
        source_range: Range<Anchor>,
        recent: bool,
        editor: WeakEntity<MessageEditor>,
        cx: &mut App,
    ) -> Completion {
        let uri = match &thread_entry {
            ThreadContextEntry::Thread { id, title } => MentionUri::Thread {
                id: id.clone(),
                name: title.to_string(),
            },
            ThreadContextEntry::Context { path, title } => MentionUri::TextThread {
                path: path.to_path_buf(),
                name: title.to_string(),
            },
        };

        let icon_for_completion = if recent {
            IconName::HistoryRerun.path().into()
        } else {
            uri.icon_path(cx)
        };

        let new_text = format!("{} ", uri.as_link());

        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(thread_entry.title().to_string(), None),
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_for_completion.clone()),
            confirm: Some(confirm_completion_callback(
                thread_entry.title().clone(),
                source_range.start,
                new_text_len - 1,
                editor,
                uri,
            )),
        }
    }

    fn completion_for_rules(
        rule: RulesContextEntry,
        source_range: Range<Anchor>,
        editor: WeakEntity<MessageEditor>,
        cx: &mut App,
    ) -> Completion {
        let uri = MentionUri::Rule {
            id: rule.prompt_id.into(),
            name: rule.title.to_string(),
        };
        let new_text = format!("{} ", uri.as_link());
        let new_text_len = new_text.len();
        let icon_path = uri.icon_path(cx);
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(rule.title.to_string(), None),
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_path.clone()),
            confirm: Some(confirm_completion_callback(
                rule.title.clone(),
                source_range.start,
                new_text_len - 1,
                editor,
                uri,
            )),
        }
    }

    pub(crate) fn completion_for_path(
        project_path: ProjectPath,
        path_prefix: &str,
        is_recent: bool,
        is_directory: bool,
        source_range: Range<Anchor>,
        message_editor: WeakEntity<MessageEditor>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Option<Completion> {
        let (file_name, directory) =
            crate::context_picker::file_context_picker::extract_file_name_and_directory(
                &project_path.path,
                path_prefix,
            );

        let label =
            build_code_label_for_full_path(&file_name, directory.as_ref().map(|s| s.as_ref()), cx);

        let abs_path = project.read(cx).absolute_path(&project_path, cx)?;

        let file_uri = MentionUri::File {
            abs_path,
            is_directory,
        };

        let crease_icon_path = file_uri.icon_path(cx);
        let completion_icon_path = if is_recent {
            IconName::HistoryRerun.path().into()
        } else {
            crease_icon_path.clone()
        };

        let new_text = format!("{} ", file_uri.as_link());
        let new_text_len = new_text.len();
        Some(Completion {
            replace_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(completion_icon_path),
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                file_name,
                source_range.start,
                new_text_len - 1,
                message_editor,
                file_uri,
            )),
        })
    }

    fn completion_for_symbol(
        symbol: Symbol,
        source_range: Range<Anchor>,
        message_editor: WeakEntity<MessageEditor>,
        workspace: Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        let project = workspace.read(cx).project().clone();

        let label = CodeLabel::plain(symbol.name.clone(), None);

        let abs_path = project.read(cx).absolute_path(&symbol.path, cx)?;
        let uri = MentionUri::Symbol {
            path: abs_path,
            name: symbol.name.clone(),
            line_range: symbol.range.start.0.row..symbol.range.end.0.row,
        };
        let new_text = format!("{} ", uri.as_link());
        let new_text_len = new_text.len();
        let icon_path = uri.icon_path(cx);
        Some(Completion {
            replace_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_path.clone()),
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                symbol.name.clone().into(),
                source_range.start,
                new_text_len - 1,
                message_editor,
                uri,
            )),
        })
    }

    fn completion_for_fetch(
        source_range: Range<Anchor>,
        url_to_fetch: SharedString,
        message_editor: WeakEntity<MessageEditor>,
        cx: &mut App,
    ) -> Option<Completion> {
        let new_text = format!("@fetch {} ", url_to_fetch.clone());
        let url_to_fetch = url::Url::parse(url_to_fetch.as_ref())
            .or_else(|_| url::Url::parse(&format!("https://{url_to_fetch}")))
            .ok()?;
        let mention_uri = MentionUri::Fetch {
            url: url_to_fetch.clone(),
        };
        let icon_path = mention_uri.icon_path(cx);
        Some(Completion {
            replace_range: source_range.clone(),
            new_text: new_text.clone(),
            label: CodeLabel::plain(url_to_fetch.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_path.clone()),
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                url_to_fetch.to_string().into(),
                source_range.start,
                new_text.len() - 1,
                message_editor,
                mention_uri,
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
        _excerpt_id: ExcerptId,
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

        let project = workspace.read(cx).project().clone();
        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let thread_store = self.thread_store.clone();
        let text_thread_store = self.text_thread_store.clone();
        let editor = self.message_editor.clone();
        let Ok((exclude_paths, exclude_threads)) =
            self.message_editor.update(cx, |message_editor, _cx| {
                message_editor.mentioned_path_and_threads()
            })
        else {
            return Task::ready(Ok(Vec::new()));
        };

        let MentionCompletion { mode, argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let recent_entries = recent_context_picker_entries(
            Some(thread_store.clone()),
            Some(text_thread_store.clone()),
            workspace.clone(),
            &exclude_paths,
            &exclude_threads,
            cx,
        );

        let prompt_store = thread_store
            .read_with(cx, |thread_store, _cx| thread_store.prompt_store().clone())
            .ok()
            .flatten();

        let search_task = search(
            mode,
            query,
            Arc::<AtomicBool>::default(),
            recent_entries,
            prompt_store,
            thread_store.clone(),
            text_thread_store.clone(),
            workspace.clone(),
            cx,
        );

        cx.spawn(async move |_, cx| {
            let matches = search_task.await;

            let completions = cx.update(|cx| {
                matches
                    .into_iter()
                    .filter_map(|mat| match mat {
                        Match::File(FileMatch { mat, is_recent }) => {
                            let project_path = ProjectPath {
                                worktree_id: WorktreeId::from_usize(mat.worktree_id),
                                path: mat.path.clone(),
                            };

                            Self::completion_for_path(
                                project_path,
                                &mat.path_prefix,
                                is_recent,
                                mat.is_dir,
                                source_range.clone(),
                                editor.clone(),
                                project.clone(),
                                cx,
                            )
                        }

                        Match::Symbol(SymbolMatch { symbol, .. }) => Self::completion_for_symbol(
                            symbol,
                            source_range.clone(),
                            editor.clone(),
                            workspace.clone(),
                            cx,
                        ),

                        Match::Thread(ThreadMatch {
                            thread, is_recent, ..
                        }) => Some(Self::completion_for_thread(
                            thread,
                            source_range.clone(),
                            is_recent,
                            editor.clone(),
                            cx,
                        )),

                        Match::Rules(user_rules) => Some(Self::completion_for_rules(
                            user_rules,
                            source_range.clone(),
                            editor.clone(),
                            cx,
                        )),

                        Match::Fetch(url) => Self::completion_for_fetch(
                            source_range.clone(),
                            url,
                            editor.clone(),
                            cx,
                        ),

                        Match::Entry(EntryMatch { entry, .. }) => Self::completion_for_entry(
                            entry,
                            source_range.clone(),
                            editor.clone(),
                            &workspace,
                            cx,
                        ),
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
    crease_text: SharedString,
    start: Anchor,
    content_len: usize,
    message_editor: WeakEntity<MessageEditor>,
    mention_uri: MentionUri,
) -> Arc<dyn Fn(CompletionIntent, &mut Window, &mut App) -> bool + Send + Sync> {
    Arc::new(move |_, window, cx| {
        let message_editor = message_editor.clone();
        let crease_text = crease_text.clone();
        let mention_uri = mention_uri.clone();
        window.defer(cx, move |window, cx| {
            message_editor
                .clone()
                .update(cx, |message_editor, cx| {
                    message_editor.confirm_completion(
                        crease_text,
                        start,
                        content_len,
                        mention_uri,
                        window,
                        cx,
                    )
                })
                .ok();
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
}
