use std::cell::RefCell;
use std::cmp::Reverse;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use agent::{HistoryEntry, HistoryStore};
use anyhow::{Context as _, Result, anyhow, bail};
use editor::{CompletionProvider, Editor, ExcerptId, ToOffset as _};
use file_icons::FileIcons;
use futures::AsyncReadExt as _;
use fuzzy::{PathMatch, StringMatch, StringMatchCandidate};
use gpui::{App, Entity, Task, WeakEntity};
use http_client::{AsyncBody, HttpClientWithUrl};
use itertools::Itertools;
use language::{Buffer, CodeLabel, CodeLabelBuilder, HighlightId};
use lsp::CompletionContext;
use ordered_float::OrderedFloat;
use project::lsp_store::SymbolLocation;
use project::{
    Completion, CompletionDisplayOptions, CompletionIntent, CompletionResponse, DocumentSymbol,
    PathMatchCandidateSet, Project, ProjectPath, Symbol, WorktreeId,
};
use prompt_store::{PromptId, PromptStore};
use rope::Point;
use text::{Anchor, OffsetRangeExt, ToPoint};
use ui::prelude::*;
use util::ResultExt as _;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use workspace::Workspace;

use crate::{
    context::{AgentContextHandle, AgentContextKey, RULES_ICON},
    context_store::ContextStore,
};

use crate::context_picker::{
    ContextPickerAction, ContextPickerEntry, ContextPickerMode, MentionLink, RecentEntry,
    RulesContextEntry, available_context_picker_entries, crease_for_mention,
    recent_context_picker_entries_with_store, selection_ranges,
};
use crate::inline_prompt_editor::ContextCreasesAddon;

pub(crate) enum Match {
    File(FileMatch),
    Symbol(SymbolMatch),
    Thread(HistoryEntry),
    RecentThread(HistoryEntry),
    Fetch(SharedString),
    Rules(RulesContextEntry),
    Entry(EntryMatch),
}

pub struct FileMatch {
    pub mat: PathMatch,
    pub is_recent: bool,
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
            Match::RecentThread(_) => 1.,
            Match::Symbol(_) => 1.,
            Match::Fetch(_) => 1.,
            Match::Rules(_) => 1.,
        }
    }
}

fn search(
    mode: Option<ContextPickerMode>,
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    recent_entries: Vec<RecentEntry>,
    prompt_store: Option<WeakEntity<PromptStore>>,
    thread_store: Option<WeakEntity<HistoryStore>>,
    workspace: Entity<Workspace>,
    cx: &mut App,
) -> Task<Vec<Match>> {
    match mode {
        Some(ContextPickerMode::File) => {
            let search_files_task = search_files(query, cancellation_flag, &workspace, cx);
            cx.background_spawn(async move {
                search_files_task
                    .await
                    .into_iter()
                    .map(Match::File)
                    .collect()
            })
        }

        Some(ContextPickerMode::Symbol) => {
            let search_symbols_task = search_symbols(query, cancellation_flag, &workspace, cx);
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
                    search_threads(query, cancellation_flag, &thread_store, cx);
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
            if let Some(prompt_store) = prompt_store.as_ref().and_then(|p| p.upgrade()) {
                let search_rules_task = search_rules(query, cancellation_flag, &prompt_store, cx);
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
                        RecentEntry::Thread(entry) => Match::RecentThread(entry),
                    })
                    .collect::<Vec<_>>();

                matches.extend(
                    available_context_picker_entries(&prompt_store, &thread_store, &workspace, cx)
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
                    search_files(query.clone(), cancellation_flag, &workspace, cx);

                let entries =
                    available_context_picker_entries(&prompt_store, &thread_store, &workspace, cx);
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

pub struct ContextCompletionProvider {
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: Option<WeakEntity<HistoryStore>>,
    prompt_store: Option<WeakEntity<PromptStore>>,
    editor: WeakEntity<Editor>,
    excluded_buffer: Option<WeakEntity<Buffer>>,
}

impl ContextCompletionProvider {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        thread_store: Option<WeakEntity<HistoryStore>>,
        prompt_store: Option<WeakEntity<PromptStore>>,
        editor: WeakEntity<Editor>,
        exclude_buffer: Option<WeakEntity<Buffer>>,
    ) -> Self {
        Self {
            workspace,
            context_store,
            thread_store,
            prompt_store,
            editor,
            excluded_buffer: exclude_buffer,
        }
    }

    fn completion_for_entry(
        entry: ContextPickerEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        match entry {
            ContextPickerEntry::Mode(mode) => Some(Completion {
                replace_range: source_range,
                new_text: format!("@{} ", mode.keyword()),
                label: CodeLabel::plain(mode.label().to_string(), None),
                icon_path: Some(mode.icon().path().into()),
                documentation: None,
                source: project::CompletionSource::Custom,
                match_start: None,
                snippet_deduplication_key: None,
                insert_text_mode: None,
                // This ensures that when a user accepts this completion, the
                // completion menu will still be shown after "@category " is
                // inserted
                confirm: Some(Arc::new(|_, _, _| true)),
            }),
            ContextPickerEntry::Action(action) => {
                let (new_text, on_action) = match action {
                    ContextPickerAction::AddSelections => {
                        let selections = selection_ranges(workspace, cx);

                        let selection_infos = selections
                            .iter()
                            .map(|(buffer, range)| {
                                let full_path = buffer
                                    .read(cx)
                                    .file()
                                    .map(|file| file.full_path(cx))
                                    .unwrap_or_else(|| PathBuf::from("untitled"));
                                let file_name = full_path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let line_range = range.to_point(&buffer.read(cx).snapshot());

                                let link = MentionLink::for_selection(
                                    &file_name,
                                    &full_path.to_string_lossy(),
                                    line_range.start.row as usize..line_range.end.row as usize,
                                );
                                (file_name, link, line_range)
                            })
                            .collect::<Vec<_>>();

                        let new_text = format!(
                            "{} ",
                            selection_infos.iter().map(|(_, link, _)| link).join(" ")
                        );

                        let callback = Arc::new({
                            move |_, window: &mut Window, cx: &mut App| {
                                context_store.update(cx, |context_store, cx| {
                                    for (buffer, range) in &selections {
                                        context_store.add_selection(
                                            buffer.clone(),
                                            range.clone(),
                                            cx,
                                        );
                                    }
                                });

                                let editor = editor.clone();
                                let selection_infos = selection_infos.clone();
                                window.defer(cx, move |window, cx| {
                                    let mut current_offset = 0;
                                    for (file_name, link, line_range) in selection_infos.iter() {
                                        let snapshot =
                                            editor.read(cx).buffer().read(cx).snapshot(cx);
                                        let Some(start) = snapshot
                                            .anchor_in_excerpt(excerpt_id, source_range.start)
                                        else {
                                            return;
                                        };

                                        let offset = start.to_offset(&snapshot) + current_offset;
                                        let text_len = link.len();

                                        let range = snapshot.anchor_after(offset)
                                            ..snapshot.anchor_after(offset + text_len);

                                        let crease = crease_for_mention(
                                            format!(
                                                "{} ({}-{})",
                                                file_name,
                                                line_range.start.row + 1,
                                                line_range.end.row + 1
                                            )
                                            .into(),
                                            IconName::Reader.path().into(),
                                            range,
                                            editor.downgrade(),
                                        );

                                        editor.update(cx, |editor, cx| {
                                            editor.insert_creases(vec![crease.clone()], cx);
                                            editor.fold_creases(vec![crease], false, window, cx);
                                        });

                                        current_offset += text_len + 1;
                                    }
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
                    match_start: None,
                    snippet_deduplication_key: None,
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
        thread_entry: HistoryEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        recent: bool,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        thread_store: Entity<HistoryStore>,
        project: Entity<Project>,
    ) -> Completion {
        let icon_for_completion = if recent {
            IconName::HistoryRerun
        } else {
            IconName::Thread
        };
        let new_text = format!("{} ", MentionLink::for_thread(&thread_entry));
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(thread_entry.title().to_string(), None),
            match_start: None,
            snippet_deduplication_key: None,
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_for_completion.path().into()),
            confirm: Some(confirm_completion_callback(
                IconName::Thread.path().into(),
                thread_entry.title().clone(),
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                context_store.clone(),
                move |window, cx| match &thread_entry {
                    HistoryEntry::AcpThread(thread) => {
                        let context_store = context_store.clone();
                        let load_thread_task = agent::load_agent_thread(
                            thread.id.clone(),
                            thread_store.clone(),
                            project.clone(),
                            cx,
                        );
                        window.spawn::<_, Option<_>>(cx, async move |cx| {
                            let thread = load_thread_task.await.log_err()?;
                            let context = context_store
                                .update(cx, |context_store, cx| {
                                    context_store.add_thread(thread, false, cx)
                                })
                                .ok()??;
                            Some(context)
                        })
                    }
                    HistoryEntry::TextThread(thread) => {
                        let path = thread.path.clone();
                        let context_store = context_store.clone();
                        let thread_store = thread_store.clone();
                        cx.spawn::<_, Option<_>>(async move |cx| {
                            let thread = thread_store
                                .update(cx, |store, cx| store.load_text_thread(path, cx))
                                .ok()?
                                .await
                                .log_err()?;
                            let context = context_store
                                .update(cx, |context_store, cx| {
                                    context_store.add_text_thread(thread, false, cx)
                                })
                                .ok()??;
                            Some(context)
                        })
                    }
                },
            )),
        }
    }

    fn completion_for_rules(
        rules: RulesContextEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
    ) -> Completion {
        let new_text = format!("{} ", MentionLink::for_rule(&rules));
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(rules.title.to_string(), None),
            match_start: None,
            snippet_deduplication_key: None,
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(RULES_ICON.path().into()),
            confirm: Some(confirm_completion_callback(
                RULES_ICON.path().into(),
                rules.title.clone(),
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                context_store.clone(),
                move |_, cx| {
                    let user_prompt_id = rules.prompt_id;
                    let context = context_store.update(cx, |context_store, cx| {
                        context_store.add_rules(user_prompt_id, false, cx)
                    });
                    Task::ready(context)
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
        let new_text = format!("{} ", MentionLink::for_fetch(&url_to_fetch));
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(url_to_fetch.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::ToolWeb.path().into()),
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                IconName::ToolWeb.path().into(),
                url_to_fetch.clone(),
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                context_store.clone(),
                move |_, cx| {
                    let context_store = context_store.clone();
                    let http_client = http_client.clone();
                    let url_to_fetch = url_to_fetch.clone();
                    cx.spawn(async move |cx| {
                        if let Some(context) = context_store
                            .read_with(cx, |context_store, _| {
                                context_store.get_url_context(url_to_fetch.clone())
                            })
                            .ok()?
                        {
                            return Some(context);
                        }
                        let content = cx
                            .background_spawn(fetch_url_content(
                                http_client,
                                url_to_fetch.to_string(),
                            ))
                            .await
                            .log_err()?;
                        context_store
                            .update(cx, |context_store, cx| {
                                context_store.add_fetched_url(url_to_fetch.to_string(), content, cx)
                            })
                            .ok()
                    })
                },
            )),
        }
    }

    fn completion_for_path(
        project_path: ProjectPath,
        path_prefix: &RelPath,
        is_recent: bool,
        is_directory: bool,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        path_style: PathStyle,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        cx: &App,
    ) -> Completion {
        let (file_name, directory) =
            extract_file_name_and_directory(&project_path.path, path_prefix, path_style);

        let label =
            build_code_label_for_path(&file_name, directory.as_ref().map(|s| s.as_ref()), None, cx);
        let full_path = if let Some(directory) = directory {
            format!("{}{}", directory, file_name)
        } else {
            file_name.to_string()
        };

        let path = Path::new(&full_path);
        let crease_icon_path = if is_directory {
            FileIcons::get_folder_icon(false, path, cx)
                .unwrap_or_else(|| IconName::Folder.path().into())
        } else {
            FileIcons::get_icon(path, cx).unwrap_or_else(|| IconName::File.path().into())
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
            match_start: None,
            snippet_deduplication_key: None,
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

    fn completion_for_symbol(
        symbol: Symbol,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        context_store: Entity<ContextStore>,
        workspace: Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        let path_style = workspace.read(cx).path_style(cx);
        let SymbolLocation::InProject(symbol_path) = &symbol.path else {
            return None;
        };
        let _path_prefix = workspace
            .read(cx)
            .project()
            .read(cx)
            .worktree_for_id(symbol_path.worktree_id, cx)?;
        let path_prefix = RelPath::empty();

        let (file_name, directory) =
            extract_file_name_and_directory(&symbol_path.path, path_prefix, path_style);
        let full_path = if let Some(directory) = directory {
            format!("{}{}", directory, file_name)
        } else {
            file_name.to_string()
        };

        let label = build_code_label_for_path(
            &symbol.name,
            Some(&file_name),
            Some(symbol.range.start.0.row + 1),
            cx,
        );

        let new_text = format!("{} ", MentionLink::for_symbol(&symbol.name, &full_path));
        let new_text_len = new_text.len();
        Some(Completion {
            replace_range: source_range.clone(),
            new_text,
            label,
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::Code.path().into()),
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                IconName::Code.path().into(),
                symbol.name.clone().into(),
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                context_store.clone(),
                move |_, cx| {
                    let symbol = symbol.clone();
                    let context_store = context_store.clone();
                    let workspace = workspace.clone();
                    let result =
                        add_symbol(symbol, false, workspace, context_store.downgrade(), cx);
                    cx.spawn(async move |_| result.await.log_err()?.0)
                },
            )),
        })
    }
}

pub fn build_code_label_for_path(
    file: &str,
    directory: Option<&str>,
    line_number: Option<u32>,
    cx: &App,
) -> CodeLabel {
    let variable_highlight_id = cx
        .theme()
        .syntax()
        .highlight_id("variable")
        .map(HighlightId);
    let mut label = CodeLabelBuilder::default();

    label.push_str(file, None);
    label.push_str(" ", None);

    if let Some(directory) = directory {
        label.push_str(directory, variable_highlight_id);
    }

    if let Some(line_number) = line_number {
        label.push_str(&format!(" L{}", line_number), variable_highlight_id);
    }

    label.build()
}

impl CompletionProvider for ContextCompletionProvider {
    fn completions(
        &self,
        excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let snapshot = buffer.read(cx).snapshot();
        let position = buffer_position.to_point(&snapshot);
        let line_start = Point::new(position.row, 0);
        let offset_to_line = snapshot.point_to_offset(line_start);
        let mut lines = snapshot.text_for_range(line_start..position).lines();
        let Some(line) = lines.next() else {
            return Task::ready(Ok(Vec::new()));
        };
        let Some(state) = MentionCompletion::try_parse(line, offset_to_line) else {
            return Task::ready(Ok(Vec::new()));
        };

        let Some((workspace, context_store)) =
            self.workspace.upgrade().zip(self.context_store.upgrade())
        else {
            return Task::ready(Ok(Vec::new()));
        };

        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let thread_store = self.thread_store.clone();
        let prompt_store = self.prompt_store.clone();
        let editor = self.editor.clone();
        let http_client = workspace.read(cx).client().http_client();
        let path_style = workspace.read(cx).path_style(cx);

        let MentionCompletion { mode, argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let excluded_path = self
            .excluded_buffer
            .as_ref()
            .and_then(WeakEntity::upgrade)
            .and_then(|b| b.read(cx).file())
            .map(|file| ProjectPath::from_file(file.as_ref(), cx));

        let recent_entries = recent_context_picker_entries_with_store(
            context_store.clone(),
            thread_store.clone(),
            workspace.clone(),
            excluded_path.clone(),
            cx,
        );

        let search_task = search(
            mode,
            query,
            Arc::<AtomicBool>::default(),
            recent_entries,
            prompt_store,
            thread_store.clone(),
            workspace.clone(),
            cx,
        );
        let project = workspace.read(cx).project().downgrade();

        cx.spawn(async move |_, cx| {
            let matches = search_task.await;
            let Some((editor, project)) = editor.upgrade().zip(project.upgrade()) else {
                return Ok(Vec::new());
            };

            let completions = cx.update(|cx| {
                matches
                    .into_iter()
                    .filter_map(|mat| match mat {
                        Match::File(FileMatch { mat, is_recent }) => {
                            let project_path = ProjectPath {
                                worktree_id: WorktreeId::from_usize(mat.worktree_id),
                                path: mat.path.clone(),
                            };

                            if excluded_path.as_ref() == Some(&project_path) {
                                return None;
                            }

                            // If path is empty, this means we're matching with the root directory itself
                            // so we use the path_prefix as the name
                            let path_prefix = if mat.path.is_empty() {
                                project
                                    .read(cx)
                                    .worktree_for_id(project_path.worktree_id, cx)
                                    .map(|wt| wt.read(cx).root_name().into())
                                    .unwrap_or_else(|| mat.path_prefix.clone())
                            } else {
                                mat.path_prefix.clone()
                            };

                            Some(Self::completion_for_path(
                                project_path,
                                &path_prefix,
                                is_recent,
                                mat.is_dir,
                                excerpt_id,
                                source_range.clone(),
                                path_style,
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
                        Match::Thread(thread) => {
                            let thread_store = thread_store.as_ref().and_then(|t| t.upgrade())?;
                            Some(Self::completion_for_thread(
                                thread,
                                excerpt_id,
                                source_range.clone(),
                                false,
                                editor.clone(),
                                context_store.clone(),
                                thread_store,
                                project.clone(),
                            ))
                        }
                        Match::RecentThread(thread) => {
                            let thread_store = thread_store.as_ref().and_then(|t| t.upgrade())?;
                            Some(Self::completion_for_thread(
                                thread,
                                excerpt_id,
                                source_range.clone(),
                                true,
                                editor.clone(),
                                context_store.clone(),
                                thread_store,
                                project.clone(),
                            ))
                        }
                        Match::Rules(user_rules) => Some(Self::completion_for_rules(
                            user_rules,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            context_store.clone(),
                        )),

                        Match::Fetch(url) => Some(Self::completion_for_fetch(
                            source_range.clone(),
                            url,
                            excerpt_id,
                            editor.clone(),
                            context_store.clone(),
                            http_client.clone(),
                        )),

                        Match::Entry(EntryMatch { entry, .. }) => Self::completion_for_entry(
                            entry,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            context_store.clone(),
                            &workspace,
                            cx,
                        ),
                    })
                    .collect()
            })?;

            Ok(vec![CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions::default(),
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
                .is_some_and(|c| !c.is_whitespace())
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

pub(crate) fn search_files(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Task<Vec<FileMatch>> {
    if query.is_empty() {
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);
        let visible_worktrees = workspace.visible_worktrees(cx).collect::<Vec<_>>();
        let include_root_name = visible_worktrees.len() > 1;

        let recent_matches = workspace
            .recent_navigation_history(Some(10), cx)
            .into_iter()
            .map(|(project_path, _)| {
                let path_prefix = if include_root_name {
                    project
                        .worktree_for_id(project_path.worktree_id, cx)
                        .map(|wt| wt.read(cx).root_name().into())
                        .unwrap_or_else(|| RelPath::empty().into())
                } else {
                    RelPath::empty().into()
                };

                FileMatch {
                    mat: PathMatch {
                        score: 0.,
                        positions: Vec::new(),
                        worktree_id: project_path.worktree_id.to_usize(),
                        path: project_path.path,
                        path_prefix,
                        distance_to_relative_ancestor: 0,
                        is_dir: false,
                    },
                    is_recent: true,
                }
            });

        let file_matches = visible_worktrees.into_iter().flat_map(|worktree| {
            let worktree = worktree.read(cx);
            let path_prefix: Arc<RelPath> = if include_root_name {
                worktree.root_name().into()
            } else {
                RelPath::empty().into()
            };
            worktree.entries(false, 0).map(move |entry| FileMatch {
                mat: PathMatch {
                    score: 0.,
                    positions: Vec::new(),
                    worktree_id: worktree.id().to_usize(),
                    path: entry.path.clone(),
                    path_prefix: path_prefix.clone(),
                    distance_to_relative_ancestor: 0,
                    is_dir: entry.is_dir(),
                },
                is_recent: false,
            })
        });

        Task::ready(recent_matches.chain(file_matches).collect())
    } else {
        let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);

                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree.root_entry().is_some_and(|entry| entry.is_ignored),
                    include_root_name,
                    candidates: project::Candidates::Entries,
                }
            })
            .collect::<Vec<_>>();

        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.as_str(),
                &None,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await
            .into_iter()
            .map(|mat| FileMatch {
                mat,
                is_recent: false,
            })
            .collect::<Vec<_>>()
        })
    }
}

pub fn extract_file_name_and_directory(
    path: &RelPath,
    path_prefix: &RelPath,
    path_style: PathStyle,
) -> (SharedString, Option<SharedString>) {
    // If path is empty, this means we're matching with the root directory itself
    // so we use the path_prefix as the name
    if path.is_empty() && !path_prefix.is_empty() {
        return (path_prefix.display(path_style).to_string().into(), None);
    }

    let full_path = path_prefix.join(path);
    let file_name = full_path.file_name().unwrap_or_default();
    let display_path = full_path.display(path_style);
    let (directory, file_name) = display_path.split_at(display_path.len() - file_name.len());
    (
        file_name.to_string().into(),
        Some(SharedString::new(directory)).filter(|dir| !dir.is_empty()),
    )
}

pub(crate) fn add_symbol(
    symbol: Symbol,
    remove_if_exists: bool,
    workspace: Entity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Task<Result<(Option<AgentContextHandle>, bool)>> {
    let project = workspace.read(cx).project().clone();
    let open_buffer_task = project.update(cx, |project, cx| {
        let SymbolLocation::InProject(symbol_path) = &symbol.path else {
            return Task::ready(Err(anyhow!("can't add symbol from outside of project")));
        };
        project.open_buffer(symbol_path.clone(), cx)
    });
    cx.spawn(async move |cx| {
        let buffer = open_buffer_task.await?;
        let document_symbols = project
            .update(cx, |project, cx| project.document_symbols(&buffer, cx))?
            .await?;

        // Try to find a matching document symbol. Document symbols include
        // not only the symbol itself (e.g. function name), but they also
        // include the context that they contain (e.g. function body).
        let (name, range, enclosing_range) = if let Some(DocumentSymbol {
            name,
            range,
            selection_range,
            ..
        }) =
            find_matching_symbol(&symbol, document_symbols.as_slice())
        {
            (name, selection_range, range)
        } else {
            // If we do not find a matching document symbol, fall back to
            // just the symbol itself
            (symbol.name, symbol.range.clone(), symbol.range)
        };

        let (range, enclosing_range) = buffer.read_with(cx, |buffer, _| {
            (
                buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                buffer.anchor_after(enclosing_range.start)
                    ..buffer.anchor_before(enclosing_range.end),
            )
        })?;

        context_store.update(cx, move |context_store, cx| {
            context_store.add_symbol(
                buffer,
                name.into(),
                range,
                enclosing_range,
                remove_if_exists,
                cx,
            )
        })
    })
}

fn find_matching_symbol(symbol: &Symbol, candidates: &[DocumentSymbol]) -> Option<DocumentSymbol> {
    let mut candidates = candidates.iter();
    let mut candidate = candidates.next()?;

    loop {
        if candidate.range.start > symbol.range.end {
            return None;
        }
        if candidate.range.end < symbol.range.start {
            candidate = candidates.next()?;
            continue;
        }
        if candidate.selection_range == symbol.range {
            return Some(candidate.clone());
        }
        if candidate.range.start <= symbol.range.start && symbol.range.end <= candidate.range.end {
            candidates = candidate.children.iter();
            candidate = candidates.next()?;
            continue;
        }
        return None;
    }
}

pub struct SymbolMatch {
    pub symbol: Symbol,
}

pub(crate) fn search_symbols(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Task<Vec<SymbolMatch>> {
    let symbols_task = workspace.update(cx, |workspace, cx| {
        workspace
            .project()
            .update(cx, |project, cx| project.symbols(&query, cx))
    });
    let project = workspace.read(cx).project().clone();
    cx.spawn(async move |cx| {
        let Some(symbols) = symbols_task.await.log_err() else {
            return Vec::new();
        };
        let Some((visible_match_candidates, external_match_candidates)): Option<(Vec<_>, Vec<_>)> =
            project
                .update(cx, |project, cx| {
                    symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| {
                            StringMatchCandidate::new(id, symbol.label.filter_text())
                        })
                        .partition(|candidate| match &symbols[candidate.id].path {
                            SymbolLocation::InProject(project_path) => project
                                .entry_for_path(project_path, cx)
                                .is_some_and(|e| !e.is_ignored),
                            SymbolLocation::OutsideProject { .. } => false,
                        })
                })
                .log_err()
        else {
            return Vec::new();
        };

        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.background_executor().block(fuzzy::match_strings(
            &visible_match_candidates,
            &query,
            false,
            true,
            MAX_MATCHES,
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let mut external_matches = cx.background_executor().block(fuzzy::match_strings(
            &external_match_candidates,
            &query,
            false,
            true,
            MAX_MATCHES - visible_matches.len().min(MAX_MATCHES),
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let sort_key_for_match = |mat: &StringMatch| {
            let symbol = &symbols[mat.candidate_id];
            (Reverse(OrderedFloat(mat.score)), symbol.label.filter_text())
        };

        visible_matches.sort_unstable_by_key(sort_key_for_match);
        external_matches.sort_unstable_by_key(sort_key_for_match);
        let mut matches = visible_matches;
        matches.append(&mut external_matches);

        matches
            .into_iter()
            .map(|mut mat| {
                let symbol = symbols[mat.candidate_id].clone();
                let filter_start = symbol.label.filter_range.start;
                for position in &mut mat.positions {
                    *position += filter_start;
                }
                SymbolMatch { symbol }
            })
            .collect()
    })
}

pub(crate) fn search_threads(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    thread_store: &Entity<HistoryStore>,
    cx: &mut App,
) -> Task<Vec<HistoryEntry>> {
    let threads = thread_store.read(cx).entries().collect();
    if query.is_empty() {
        return Task::ready(threads);
    }

    let executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        let candidates = threads
            .iter()
            .enumerate()
            .map(|(id, thread)| StringMatchCandidate::new(id, thread.title()))
            .collect::<Vec<_>>();
        let matches = fuzzy::match_strings(
            &candidates,
            &query,
            false,
            true,
            100,
            &cancellation_flag,
            executor,
        )
        .await;

        matches
            .into_iter()
            .map(|mat| threads[mat.candidate_id].clone())
            .collect()
    })
}

pub(crate) fn search_rules(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    prompt_store: &Entity<PromptStore>,
    cx: &mut App,
) -> Task<Vec<RulesContextEntry>> {
    let search_task = prompt_store.read(cx).search(query, cancellation_flag, cx);
    cx.background_spawn(async move {
        search_task
            .await
            .into_iter()
            .flat_map(|metadata| {
                // Default prompts are filtered out as they are automatically included.
                if metadata.default {
                    None
                } else {
                    match metadata.id {
                        PromptId::EditWorkflow => None,
                        PromptId::User { uuid } => Some(RulesContextEntry {
                            prompt_id: uuid,
                            title: metadata.title?,
                        }),
                    }
                }
            })
            .collect::<Vec<_>>()
    })
}

pub(crate) async fn fetch_url_content(
    http_client: Arc<HttpClientWithUrl>,
    url: String,
) -> Result<String> {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum ContentType {
        Html,
        Plaintext,
        Json,
    }

    use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};

    let url = if !url.starts_with("https://") && !url.starts_with("http://") {
        format!("https://{url}")
    } else {
        url
    };

    let mut response = http_client.get(&url, AsyncBody::default(), true).await?;

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading response body")?;

    if response.status().is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        bail!(
            "status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let Some(content_type) = response.headers().get("content-type") else {
        bail!("missing Content-Type header");
    };
    let content_type = content_type
        .to_str()
        .context("invalid Content-Type header")?;
    let content_type = match content_type {
        "text/html" => ContentType::Html,
        "text/plain" => ContentType::Plaintext,
        "application/json" => ContentType::Json,
        _ => ContentType::Html,
    };

    match content_type {
        ContentType::Html => {
            let mut handlers: Vec<TagHandler> = vec![
                Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                Rc::new(RefCell::new(markdown::ParagraphHandler)),
                Rc::new(RefCell::new(markdown::HeadingHandler)),
                Rc::new(RefCell::new(markdown::ListHandler)),
                Rc::new(RefCell::new(markdown::TableHandler::new())),
                Rc::new(RefCell::new(markdown::StyledTextHandler)),
            ];
            if url.contains("wikipedia.org") {
                use html_to_markdown::structure::wikipedia;

                handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                handlers.push(Rc::new(
                    RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                ));
            } else {
                handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
            }

            convert_html_to_markdown(&body[..], &mut handlers)
        }
        ContentType::Plaintext => Ok(std::str::from_utf8(&body)?.to_owned()),
        ContentType::Json => {
            let json: serde_json::Value = serde_json::from_slice(&body)?;

            Ok(format!(
                "```json\n{}\n```",
                serde_json::to_string_pretty(&json)?
            ))
        }
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
    use util::{path, rel_path::rel_path};
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
            self.0.read(cx).focus_handle(cx)
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
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
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
            rel_path("a/one.txt"),
            rel_path("a/two.txt"),
            rel_path("a/three.txt"),
            rel_path("a/four.txt"),
            rel_path("b/five.txt"),
            rel_path("b/six.txt"),
            rel_path("b/seven.txt"),
            rel_path("b/eight.txt"),
        ];

        let slash = PathStyle::local().separator();

        let mut opened_editors = Vec::new();
        for path in paths {
            let buffer = workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.open_path(
                        ProjectPath {
                            worktree_id,
                            path: path.into(),
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

        let context_store = cx.new(|_| ContextStore::new(project.downgrade()));

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
            editor.set_completion_provider(Some(Rc::new(ContextCompletionProvider::new(
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
                    format!("seven.txt b{slash}"),
                    format!("six.txt b{slash}"),
                    format!("five.txt b{slash}"),
                    format!("four.txt a{slash}"),
                    "Files & Directories".into(),
                    "Symbols".into(),
                    "Fetch".into()
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
            assert_eq!(
                current_completion_labels(editor),
                vec![format!("one.txt a{slash}")]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt) ")
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 33)]
            );
        });

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  ")
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 33)]
            );
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  Ipsum "),
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 33)]
            );
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  Ipsum @file "),
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 33)]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  Ipsum [@seven.txt](@file:b{slash}seven.txt) ")
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 33),
                    Point::new(0, 41)..Point::new(0, 72)
                ]
            );
        });

        cx.simulate_input("\n@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  Ipsum [@seven.txt](@file:b{slash}seven.txt) \n@")
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 33),
                    Point::new(0, 41)..Point::new(0, 72)
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
                format!("Lorem [@one.txt](@file:a{slash}one.txt)  Ipsum [@seven.txt](@file:b{slash}seven.txt) \n[@six.txt](@file:b{slash}six.txt) ")
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 33),
                    Point::new(0, 41)..Point::new(0, 72),
                    Point::new(1, 0)..Point::new(1, 27)
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_context_completion_provider_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/project1"),
                json!({
                    "a": {
                        "one.txt": "",
                        "two.txt": "",
                    }
                }),
            )
            .await;

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/project2"),
                json!({
                    "b": {
                        "three.txt": "",
                        "four.txt": "",
                    }
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            [path!("/project1").as_ref(), path!("/project2").as_ref()],
            cx,
        )
        .await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let worktrees = project.update(cx, |project, cx| {
            let worktrees = project.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 2);
            worktrees
        });

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        let slash = PathStyle::local().separator();

        for (worktree_idx, paths) in [
            vec![rel_path("a/one.txt"), rel_path("a/two.txt")],
            vec![rel_path("b/three.txt"), rel_path("b/four.txt")],
        ]
        .iter()
        .enumerate()
        {
            let worktree_id = worktrees[worktree_idx].read_with(&cx, |wt, _| wt.id());
            for path in paths {
                workspace
                    .update_in(&mut cx, |workspace, window, cx| {
                        workspace.open_path(
                            ProjectPath {
                                worktree_id,
                                path: (*path).into(),
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

        let context_store = cx.new(|_| ContextStore::new(project.downgrade()));

        let editor_entity = editor.downgrade();
        editor.update_in(&mut cx, |editor, window, cx| {
            window.focus(&editor.focus_handle(cx));
            editor.set_completion_provider(Some(Rc::new(ContextCompletionProvider::new(
                workspace.downgrade(),
                context_store.downgrade(),
                None,
                None,
                editor_entity,
                None,
            ))));
        });

        cx.simulate_input("@");

        // With multiple worktrees, we should see the project name as prefix
        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "@");
            assert!(editor.has_visible_completions_menu());
            let labels = current_completion_labels(editor);

            assert!(
                labels.contains(&format!("four.txt project2{slash}b{slash}")),
                "Expected 'four.txt project2{slash}b{slash}' in labels: {:?}",
                labels
            );
            assert!(
                labels.contains(&format!("three.txt project2{slash}b{slash}")),
                "Expected 'three.txt project2{slash}b{slash}' in labels: {:?}",
                labels
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "@file ");
            assert!(editor.has_visible_completions_menu());
        });

        cx.simulate_input("one");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "@file one");
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                current_completion_labels(editor),
                vec![format!("one.txt project1{slash}a{slash}")]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("[@one.txt](@file:project1{slash}a{slash}one.txt) ")
            );
            assert!(!editor.has_visible_completions_menu());
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
            .map(|completion| completion.label.text)
            .collect::<Vec<_>>()
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }
}
