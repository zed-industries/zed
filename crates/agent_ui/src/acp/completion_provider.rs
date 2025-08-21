use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use acp_thread::{MentionUri, selection_name};
use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use editor::display_map::CreaseId;
use editor::{CompletionProvider, Editor, ExcerptId, ToOffset as _};
use file_icons::FileIcons;
use futures::future::try_join_all;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, Task, WeakEntity};
use http_client::HttpClientWithUrl;
use itertools::Itertools as _;
use language::{Buffer, CodeLabel, HighlightId};
use lsp::CompletionContext;
use parking_lot::Mutex;
use project::{
    Completion, CompletionIntent, CompletionResponse, Project, ProjectPath, Symbol, WorktreeId,
};
use prompt_store::PromptStore;
use rope::Point;
use text::{Anchor, OffsetRangeExt as _, ToPoint as _};
use ui::prelude::*;
use url::Url;
use workspace::Workspace;
use workspace::notifications::NotifyResultExt;

use agent::{
    context::RULES_ICON,
    thread_store::{TextThreadStore, ThreadStore},
};

use crate::context_picker::fetch_context_picker::fetch_url_content;
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

#[derive(Default)]
pub struct MentionSet {
    uri_by_crease_id: HashMap<CreaseId, MentionUri>,
    fetch_results: HashMap<Url, String>,
}

impl MentionSet {
    pub fn insert(&mut self, crease_id: CreaseId, uri: MentionUri) {
        self.uri_by_crease_id.insert(crease_id, uri);
    }

    pub fn add_fetch_result(&mut self, url: Url, content: String) {
        self.fetch_results.insert(url, content);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = CreaseId> {
        self.fetch_results.clear();
        self.uri_by_crease_id.drain().map(|(id, _)| id)
    }

    pub fn contents(
        &self,
        project: Entity<Project>,
        thread_store: Entity<ThreadStore>,
        text_thread_store: Entity<TextThreadStore>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<HashMap<CreaseId, Mention>>> {
        let contents = self
            .uri_by_crease_id
            .iter()
            .map(|(&crease_id, uri)| {
                match uri {
                    MentionUri::File(path) => {
                        let uri = uri.clone();
                        let path = path.to_path_buf();
                        let buffer_task = project.update(cx, |project, cx| {
                            let path = project
                                .find_project_path(path, cx)
                                .context("Failed to find project path")?;
                            anyhow::Ok(project.open_buffer(path, cx))
                        });

                        cx.spawn(async move |cx| {
                            let buffer = buffer_task?.await?;
                            let content = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                            anyhow::Ok((crease_id, Mention { uri, content }))
                        })
                    }
                    MentionUri::Symbol {
                        path, line_range, ..
                    }
                    | MentionUri::Selection {
                        path, line_range, ..
                    } => {
                        let uri = uri.clone();
                        let path_buf = path.clone();
                        let line_range = line_range.clone();

                        let buffer_task = project.update(cx, |project, cx| {
                            let path = project
                                .find_project_path(&path_buf, cx)
                                .context("Failed to find project path")?;
                            anyhow::Ok(project.open_buffer(path, cx))
                        });

                        cx.spawn(async move |cx| {
                            let buffer = buffer_task?.await?;
                            let content = buffer.read_with(cx, |buffer, _cx| {
                                buffer
                                    .text_for_range(
                                        Point::new(line_range.start, 0)
                                            ..Point::new(
                                                line_range.end,
                                                buffer.line_len(line_range.end),
                                            ),
                                    )
                                    .collect()
                            })?;

                            anyhow::Ok((crease_id, Mention { uri, content }))
                        })
                    }
                    MentionUri::Thread { id: thread_id, .. } => {
                        let open_task = thread_store.update(cx, |thread_store, cx| {
                            thread_store.open_thread(&thread_id, window, cx)
                        });

                        let uri = uri.clone();
                        cx.spawn(async move |cx| {
                            let thread = open_task.await?;
                            let content = thread.read_with(cx, |thread, _cx| {
                                thread.latest_detailed_summary_or_text().to_string()
                            })?;

                            anyhow::Ok((crease_id, Mention { uri, content }))
                        })
                    }
                    MentionUri::TextThread { path, .. } => {
                        let context = text_thread_store.update(cx, |text_thread_store, cx| {
                            text_thread_store.open_local_context(path.as_path().into(), cx)
                        });
                        let uri = uri.clone();
                        cx.spawn(async move |cx| {
                            let context = context.await?;
                            let xml = context.update(cx, |context, cx| context.to_xml(cx))?;
                            anyhow::Ok((crease_id, Mention { uri, content: xml }))
                        })
                    }
                    MentionUri::Rule { id: prompt_id, .. } => {
                        let Some(prompt_store) = thread_store.read(cx).prompt_store().clone()
                        else {
                            return Task::ready(Err(anyhow!("missing prompt store")));
                        };
                        let text_task = prompt_store.read(cx).load(*prompt_id, cx);
                        let uri = uri.clone();
                        cx.spawn(async move |_| {
                            // TODO: report load errors instead of just logging
                            let text = text_task.await?;
                            anyhow::Ok((crease_id, Mention { uri, content: text }))
                        })
                    }
                    MentionUri::Fetch { url } => {
                        let Some(content) = self.fetch_results.get(&url) else {
                            return Task::ready(Err(anyhow!("missing fetch result")));
                        };
                        Task::ready(Ok((
                            crease_id,
                            Mention {
                                uri: uri.clone(),
                                content: content.clone(),
                            },
                        )))
                    }
                }
            })
            .collect::<Vec<_>>();

        cx.spawn(async move |_cx| {
            let contents = try_join_all(contents).await?.into_iter().collect();
            anyhow::Ok(contents)
        })
    }
}

#[derive(Debug)]
pub struct Mention {
    pub uri: MentionUri,
    pub content: String,
}

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
    mention_set: Arc<Mutex<MentionSet>>,
    workspace: WeakEntity<Workspace>,
    thread_store: WeakEntity<ThreadStore>,
    text_thread_store: WeakEntity<TextThreadStore>,
    editor: WeakEntity<Editor>,
}

impl ContextPickerCompletionProvider {
    pub fn new(
        mention_set: Arc<Mutex<MentionSet>>,
        workspace: WeakEntity<Workspace>,
        thread_store: WeakEntity<ThreadStore>,
        text_thread_store: WeakEntity<TextThreadStore>,
        editor: WeakEntity<Editor>,
    ) -> Self {
        Self {
            mention_set,
            workspace,
            thread_store,
            text_thread_store,
            editor,
        }
    }

    fn completion_for_entry(
        entry: ContextPickerEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
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
                        let selections = selection_ranges(workspace, cx);

                        const PLACEHOLDER: &str = "selection ";

                        let new_text = std::iter::repeat(PLACEHOLDER)
                            .take(selections.len())
                            .chain(std::iter::once(""))
                            .join(" ");

                        let callback = Arc::new({
                            let mention_set = mention_set.clone();
                            let selections = selections.clone();
                            move |_, window: &mut Window, cx: &mut App| {
                                let editor = editor.clone();
                                let mention_set = mention_set.clone();
                                let selections = selections.clone();
                                window.defer(cx, move |window, cx| {
                                    let mut current_offset = 0;

                                    for (buffer, selection_range) in selections {
                                        let snapshot =
                                            editor.read(cx).buffer().read(cx).snapshot(cx);
                                        let Some(start) = snapshot
                                            .anchor_in_excerpt(excerpt_id, source_range.start)
                                        else {
                                            return;
                                        };

                                        let offset = start.to_offset(&snapshot) + current_offset;
                                        let text_len = PLACEHOLDER.len() - 1;

                                        let range = snapshot.anchor_after(offset)
                                            ..snapshot.anchor_after(offset + text_len);

                                        let path = buffer
                                            .read(cx)
                                            .file()
                                            .map_or(PathBuf::from("untitled"), |file| {
                                                file.path().to_path_buf()
                                            });

                                        let point_range = snapshot
                                            .as_singleton()
                                            .map(|(_, _, snapshot)| {
                                                selection_range.to_point(&snapshot)
                                            })
                                            .unwrap_or_default();
                                        let line_range = point_range.start.row..point_range.end.row;
                                        let crease = crate::context_picker::crease_for_mention(
                                            selection_name(&path, &line_range).into(),
                                            IconName::Reader.path().into(),
                                            range,
                                            editor.downgrade(),
                                        );

                                        let [crease_id]: [_; 1] =
                                            editor.update(cx, |editor, cx| {
                                                let crease_ids =
                                                    editor.insert_creases(vec![crease.clone()], cx);
                                                editor.fold_creases(
                                                    vec![crease],
                                                    false,
                                                    window,
                                                    cx,
                                                );
                                                crease_ids.try_into().unwrap()
                                            });

                                        mention_set.lock().insert(
                                            crease_id,
                                            MentionUri::Selection { path, line_range },
                                        );

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
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        recent: bool,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
    ) -> Completion {
        let icon_for_completion = if recent {
            IconName::HistoryRerun
        } else {
            IconName::Thread
        };

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
        let new_text = format!("{} ", uri.as_link());

        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(thread_entry.title().to_string(), None),
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
                editor.clone(),
                mention_set,
                uri,
            )),
        }
    }

    fn completion_for_rules(
        rule: RulesContextEntry,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
    ) -> Completion {
        let uri = MentionUri::Rule {
            id: rule.prompt_id.into(),
            name: rule.title.to_string(),
        };
        let new_text = format!("{} ", uri.as_link());
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(rule.title.to_string(), None),
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(RULES_ICON.path().into()),
            confirm: Some(confirm_completion_callback(
                RULES_ICON.path().into(),
                rule.title.clone(),
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor.clone(),
                mention_set,
                uri,
            )),
        }
    }

    pub(crate) fn completion_for_path(
        project_path: ProjectPath,
        path_prefix: &str,
        is_recent: bool,
        is_directory: bool,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
        project: Entity<Project>,
        cx: &App,
    ) -> Option<Completion> {
        let (file_name, directory) =
            crate::context_picker::file_context_picker::extract_file_name_and_directory(
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

        let abs_path = project.read(cx).absolute_path(&project_path, cx)?;

        let file_uri = MentionUri::File(abs_path);
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
                crease_icon_path,
                file_name,
                excerpt_id,
                source_range.start,
                new_text_len - 1,
                editor,
                mention_set.clone(),
                file_uri,
            )),
        })
    }

    fn completion_for_symbol(
        symbol: Symbol,
        excerpt_id: ExcerptId,
        source_range: Range<Anchor>,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
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
                new_text_len - 1,
                editor.clone(),
                mention_set.clone(),
                uri,
            )),
        })
    }

    fn completion_for_fetch(
        source_range: Range<Anchor>,
        url_to_fetch: SharedString,
        excerpt_id: ExcerptId,
        editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
        http_client: Arc<HttpClientWithUrl>,
    ) -> Option<Completion> {
        let new_text = format!("@fetch {} ", url_to_fetch.clone());
        let new_text_len = new_text.len();
        Some(Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(url_to_fetch.to_string(), None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(IconName::ToolWeb.path().into()),
            insert_text_mode: None,
            confirm: Some({
                let start = source_range.start;
                let content_len = new_text_len - 1;
                let editor = editor.clone();
                let url_to_fetch = url_to_fetch.clone();
                let source_range = source_range.clone();
                Arc::new(move |_, window, cx| {
                    let Some(url) = url::Url::parse(url_to_fetch.as_ref())
                        .or_else(|_| url::Url::parse(&format!("https://{url_to_fetch}")))
                        .notify_app_err(cx)
                    else {
                        return false;
                    };
                    let mention_uri = MentionUri::Fetch { url: url.clone() };

                    let editor = editor.clone();
                    let mention_set = mention_set.clone();
                    let http_client = http_client.clone();
                    let source_range = source_range.clone();
                    window.defer(cx, move |window, cx| {
                        let url = url.clone();

                        let Some(crease_id) = crate::context_picker::insert_crease_for_mention(
                            excerpt_id,
                            start,
                            content_len,
                            url.to_string().into(),
                            IconName::ToolWeb.path().into(),
                            editor.clone(),
                            window,
                            cx,
                        ) else {
                            return;
                        };

                        let editor = editor.clone();
                        let mention_set = mention_set.clone();
                        let http_client = http_client.clone();
                        let source_range = source_range.clone();
                        window
                            .spawn(cx, async move |cx| {
                                if let Some(content) =
                                    fetch_url_content(http_client, url.to_string())
                                        .await
                                        .notify_async_err(cx)
                                {
                                    mention_set.lock().add_fetch_result(url, content);
                                    mention_set.lock().insert(crease_id, mention_uri.clone());
                                } else {
                                    // Remove crease if we failed to fetch
                                    editor
                                        .update(cx, |editor, cx| {
                                            let snapshot = editor.buffer().read(cx).snapshot(cx);
                                            let Some(anchor) = snapshot
                                                .anchor_in_excerpt(excerpt_id, source_range.start)
                                            else {
                                                return;
                                            };
                                            editor.display_map.update(cx, |display_map, cx| {
                                                display_map.unfold_intersecting(
                                                    vec![anchor..anchor],
                                                    true,
                                                    cx,
                                                );
                                            });
                                            editor.remove_creases([crease_id], cx);
                                        })
                                        .ok();
                                }
                                Some(())
                            })
                            .detach();
                    });
                    false
                })
            }),
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
        let http_client = workspace.read(cx).client().http_client();
        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let thread_store = self.thread_store.clone();
        let text_thread_store = self.text_thread_store.clone();
        let editor = self.editor.clone();

        let MentionCompletion { mode, argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let (exclude_paths, exclude_threads) = {
            let mention_set = self.mention_set.lock();

            let mut excluded_paths = HashSet::default();
            let mut excluded_threads = HashSet::default();

            for uri in mention_set.uri_by_crease_id.values() {
                match uri {
                    MentionUri::File(path) => {
                        excluded_paths.insert(path.clone());
                    }
                    MentionUri::Thread { id, .. } => {
                        excluded_threads.insert(id.clone());
                    }
                    _ => {}
                }
            }

            (excluded_paths, excluded_threads)
        };

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

        let mention_set = self.mention_set.clone();

        cx.spawn(async move |_, cx| {
            let matches = search_task.await;
            let Some(editor) = editor.upgrade() else {
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

                            Self::completion_for_path(
                                project_path,
                                &mat.path_prefix,
                                is_recent,
                                mat.is_dir,
                                excerpt_id,
                                source_range.clone(),
                                editor.clone(),
                                mention_set.clone(),
                                project.clone(),
                                cx,
                            )
                        }

                        Match::Symbol(SymbolMatch { symbol, .. }) => Self::completion_for_symbol(
                            symbol,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            mention_set.clone(),
                            workspace.clone(),
                            cx,
                        ),

                        Match::Thread(ThreadMatch {
                            thread, is_recent, ..
                        }) => Some(Self::completion_for_thread(
                            thread,
                            excerpt_id,
                            source_range.clone(),
                            is_recent,
                            editor.clone(),
                            mention_set.clone(),
                        )),

                        Match::Rules(user_rules) => Some(Self::completion_for_rules(
                            user_rules,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            mention_set.clone(),
                        )),

                        Match::Fetch(url) => Self::completion_for_fetch(
                            source_range.clone(),
                            url,
                            excerpt_id,
                            editor.clone(),
                            mention_set.clone(),
                            http_client.clone(),
                        ),

                        Match::Entry(EntryMatch { entry, .. }) => Self::completion_for_entry(
                            entry,
                            excerpt_id,
                            source_range.clone(),
                            editor.clone(),
                            mention_set.clone(),
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
    crease_icon_path: SharedString,
    crease_text: SharedString,
    excerpt_id: ExcerptId,
    start: Anchor,
    content_len: usize,
    editor: Entity<Editor>,
    mention_set: Arc<Mutex<MentionSet>>,
    mention_uri: MentionUri,
) -> Arc<dyn Fn(CompletionIntent, &mut Window, &mut App) -> bool + Send + Sync> {
    Arc::new(move |_, window, cx| {
        let crease_text = crease_text.clone();
        let crease_icon_path = crease_icon_path.clone();
        let editor = editor.clone();
        let mention_set = mention_set.clone();
        let mention_uri = mention_uri.clone();
        window.defer(cx, move |window, cx| {
            if let Some(crease_id) = crate::context_picker::insert_crease_for_mention(
                excerpt_id,
                start,
                content_len,
                crease_text.clone(),
                crease_icon_path,
                editor.clone(),
                window,
                cx,
            ) {
                mention_set.lock().insert(crease_id, mention_uri.clone());
            }
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
    use editor::AnchorRangeExt;
    use gpui::{EventEmitter, FocusHandle, Focusable, TestAppContext, VisualTestContext};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::SettingsStore;
    use smol::stream::StreamExt as _;
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
                        "one.txt": "1",
                        "two.txt": "2",
                        "three.txt": "3",
                        "four.txt": "4"
                    },
                    "b": {
                        "five.txt": "5",
                        "six.txt": "6",
                        "seven.txt": "7",
                        "eight.txt": "8",
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

        let mention_set = Arc::new(Mutex::new(MentionSet::default()));

        let thread_store = cx.new(|cx| ThreadStore::fake(project.clone(), cx));
        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));

        let editor_entity = editor.downgrade();
        editor.update_in(&mut cx, |editor, window, cx| {
            window.focus(&editor.focus_handle(cx));
            editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
                mention_set.clone(),
                workspace.downgrade(),
                thread_store.downgrade(),
                text_thread_store.downgrade(),
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
                    "eight.txt dir/b/",
                    "seven.txt dir/b/",
                    "six.txt dir/b/",
                    "five.txt dir/b/",
                    "Files & Directories",
                    "Symbols",
                    "Threads",
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
            assert_eq!(editor.text(cx), "Lorem [@one.txt](file:///dir/a/one.txt) ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        let contents = cx
            .update(|window, cx| {
                mention_set.lock().contents(
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0].content, "1");
        assert_eq!(
            contents[0].uri.to_uri().to_string(),
            "file:///dir/a/one.txt"
        );

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](file:///dir/a/one.txt)  ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum ",
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum @file ",
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        let contents = cx
            .update(|window, cx| {
                mention_set.lock().contents(
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        assert_eq!(contents.len(), 2);
        let new_mention = contents
            .iter()
            .find(|mention| mention.uri.to_uri().to_string() == "file:///dir/b/eight.txt")
            .unwrap();
        assert_eq!(new_mention.content, "8");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) "
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![
                    Point::new(0, 6)..Point::new(0, 39),
                    Point::new(0, 47)..Point::new(0, 84)
                ]
            );
        });

        let plain_text_language = Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Plain Text".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["txt".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        // Register the language and fake LSP
        let language_registry = project.read_with(&cx, |project, _| project.languages().clone());
        language_registry.add(plain_text_language);

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Plain Text",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    workspace_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        // Open the buffer to trigger LSP initialization
        let buffer = project
            .update(&mut cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a/one.txt"), cx)
            })
            .await
            .unwrap();

        // Register the buffer with language servers
        let _handle = project.update(&mut cx, |project, cx| {
            project.register_buffer_with_language_servers(&buffer, cx)
        });

        cx.run_until_parked();

        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            |_, _| async move {
                Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                    #[allow(deprecated)]
                    lsp::SymbolInformation {
                        name: "MySymbol".into(),
                        location: lsp::Location {
                            uri: lsp::Url::from_file_path(path!("/dir/a/one.txt")).unwrap(),
                            range: lsp::Range::new(
                                lsp::Position::new(0, 0),
                                lsp::Position::new(0, 1),
                            ),
                        },
                        kind: lsp::SymbolKind::CONSTANT,
                        tags: None,
                        container_name: None,
                        deprecated: None,
                    },
                ])))
            },
        );

        cx.simulate_input("@symbol ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) @symbol "
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                current_completion_labels(editor),
                &[
                    "MySymbol",
                ]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let contents = cx
            .update(|window, cx| {
                mention_set.lock().contents(
                    project.clone(),
                    thread_store,
                    text_thread_store,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        assert_eq!(contents.len(), 3);
        let new_mention = contents
            .iter()
            .find(|mention| {
                mention.uri.to_uri().to_string() == "file:///dir/a/one.txt?symbol=MySymbol#L1:1"
            })
            .unwrap();
        assert_eq!(new_mention.content, "1");

        cx.run_until_parked();

        editor.read_with(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) [@MySymbol](file:///dir/a/one.txt?symbol=MySymbol#L1:1) "
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
