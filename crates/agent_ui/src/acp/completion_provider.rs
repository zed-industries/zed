use std::cell::{Cell, RefCell};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use acp_thread::{AcpThread, MentionUri};
use agent_client_protocol as acp;
use agent2::{HistoryEntry, HistoryStore};
use anyhow::Result;
use editor::{CompletionProvider, Editor, ExcerptId};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, Task, WeakEntity};
use language::{Buffer, CodeLabel, HighlightId};
use lsp::CompletionContext;
use project::{
    Completion, CompletionIntent, CompletionResponse, Project, ProjectPath, Symbol, WorktreeId,
    lsp_store::CompletionDocumentation,
};
use prompt_store::PromptStore;
use rope::Point;
use text::{Anchor, ToPoint as _};
use ui::prelude::*;
use workspace::Workspace;

use crate::AgentPanel;
use crate::acp::message_editor::MessageEditor;
use crate::context_picker::file_context_picker::{FileMatch, search_files};
use crate::context_picker::rules_context_picker::{RulesContextEntry, search_rules};
use crate::context_picker::symbol_context_picker::SymbolMatch;
use crate::context_picker::symbol_context_picker::search_symbols;
use crate::context_picker::{
    ContextPickerAction, ContextPickerEntry, ContextPickerMode, selection_ranges,
};

#[derive(Debug)]
enum CompletionType {
    Mention(MentionCompletion),
    SlashCommand(SlashCommandCompletion),
}

pub(crate) enum Match {
    File(FileMatch),
    Symbol(SymbolMatch),
    Thread(HistoryEntry),
    RecentThread(HistoryEntry),
    Fetch(SharedString),
    Rules(RulesContextEntry),
    Entry(EntryMatch),
}

pub struct EntryMatch {
    mat: Option<StringMatch>,
    entry: ContextPickerEntry,
}

#[derive(Debug, Clone)]
pub struct SlashCommandCompletion {
    pub source_range: Range<usize>,
    pub command_name: String,
    pub argument: Option<String>,
}

impl SlashCommandCompletion {
    fn try_parse(line: &str, offset_to_line: usize) -> Option<Self> {
        let last_slash_start = line.rfind('/')?;
        if last_slash_start >= line.len() {
            return Some(Self {
                source_range: last_slash_start + offset_to_line..last_slash_start + 1 + offset_to_line,
                command_name: String::new(),
                argument: None,
            });
        }
        
        // Check if slash is at word boundary (not preceded by alphanumeric)
        if last_slash_start > 0
            && line
                .chars()
                .nth(last_slash_start - 1)
                .is_some_and(|c| c.is_alphanumeric())
        {
            return None;
        }

        let rest_of_line = &line[last_slash_start + 1..];
        
        let mut command_name = String::new();
        let mut argument = None;
        
        let mut parts = rest_of_line.split_whitespace();
        let mut end = last_slash_start + 1;
        
        if let Some(cmd_text) = parts.next() {
            end += cmd_text.len();
            command_name = cmd_text.to_string();
            
            // Check for arguments after command name
            match rest_of_line[cmd_text.len()..].find(|c: char| !c.is_whitespace()) {
                Some(whitespace_count) => {
                    if let Some(arg_text) = parts.next() {
                        argument = Some(arg_text.to_string());
                        end += whitespace_count + arg_text.len();
                    }
                }
                None => {
                    // Rest of line is entirely whitespace
                    end += rest_of_line.len() - cmd_text.len();
                }
            }
        }
        
        Some(Self {
            source_range: last_slash_start + offset_to_line..end + offset_to_line,
            command_name,
            argument,
        })
    }
}

impl Match {
    pub fn score(&self) -> f64 {
        match self {
            Match::File(file) => file.mat.score,
            Match::Entry(mode) => mode.mat.as_ref().map(|mat| mat.score).unwrap_or(1.),
            Match::Thread(_) => 1.,
            Match::RecentThread(_) => 1.,
            Match::Symbol(_) => 1.,
            Match::Rules(_) => 1.,
            Match::Fetch(_) => 1.,
        }
    }
}

pub struct ContextPickerCompletionProvider {
    message_editor: WeakEntity<MessageEditor>,
    workspace: WeakEntity<Workspace>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    prompt_capabilities: Rc<Cell<acp::PromptCapabilities>>,
    thread: Rc<RefCell<Option<WeakEntity<AcpThread>>>>,
}

impl ContextPickerCompletionProvider {
    pub fn new(
        message_editor: WeakEntity<MessageEditor>,
        workspace: WeakEntity<Workspace>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<Cell<acp::PromptCapabilities>>,
    ) -> Self {
        Self {
            message_editor,
            workspace,
            history_store,
            prompt_store,
            prompt_capabilities,
            thread: Rc::new(RefCell::new(None)),
        }
    }

    /// Set the ACP thread for slash command support
    pub fn set_thread(&self, thread: WeakEntity<AcpThread>) {
        *self.thread.borrow_mut() = Some(thread);
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
                replace_range: source_range,
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
                Self::completion_for_action(action, source_range, message_editor, workspace, cx)
            }
        }
    }

    fn completion_for_thread(
        thread_entry: HistoryEntry,
        source_range: Range<Anchor>,
        recent: bool,
        editor: WeakEntity<MessageEditor>,
        cx: &mut App,
    ) -> Completion {
        let uri = thread_entry.mention_uri();

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
            icon_path: Some(icon_for_completion),
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
            icon_path: Some(icon_path),
            confirm: Some(confirm_completion_callback(
                rule.title,
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

        let uri = if is_directory {
            MentionUri::Directory { abs_path }
        } else {
            MentionUri::File { abs_path }
        };

        let crease_icon_path = uri.icon_path(cx);
        let completion_icon_path = if is_recent {
            IconName::HistoryRerun.path().into()
        } else {
            crease_icon_path
        };

        let new_text = format!("{} ", uri.as_link());
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
                uri,
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
            abs_path,
            name: symbol.name.clone(),
            line_range: symbol.range.start.0.row..=symbol.range.end.0.row,
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
            icon_path: Some(icon_path),
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                symbol.name.into(),
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
        let new_text = format!("@fetch {} ", url_to_fetch);
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
            icon_path: Some(icon_path),
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

    pub(crate) fn completion_for_action(
        action: ContextPickerAction,
        source_range: Range<Anchor>,
        message_editor: WeakEntity<MessageEditor>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
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
            replace_range: source_range,
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

    fn search(
        &self,
        mode: Option<ContextPickerMode>,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut App,
    ) -> Task<Vec<Match>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Vec::default());
        };
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
                let search_threads_task =
                    search_threads(query, cancellation_flag, &self.history_store, cx);
                cx.background_spawn(async move {
                    search_threads_task
                        .await
                        .into_iter()
                        .map(Match::Thread)
                        .collect()
                })
            }

            Some(ContextPickerMode::Fetch) => {
                if !query.is_empty() {
                    Task::ready(vec![Match::Fetch(query.into())])
                } else {
                    Task::ready(Vec::new())
                }
            }

            Some(ContextPickerMode::Rules) => {
                if let Some(prompt_store) = self.prompt_store.as_ref() {
                    let search_rules_task =
                        search_rules(query, cancellation_flag, prompt_store, cx);
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

            None if query.is_empty() => {
                let mut matches = self.recent_context_picker_entries(&workspace, cx);

                matches.extend(
                    self.available_context_picker_entries(&workspace, cx)
                        .into_iter()
                        .map(|mode| {
                            Match::Entry(EntryMatch {
                                entry: mode,
                                mat: None,
                            })
                        }),
                );

                Task::ready(matches)
            }
            None => {
                let executor = cx.background_executor().clone();

                let search_files_task =
                    search_files(query.clone(), cancellation_flag, &workspace, cx);

                let entries = self.available_context_picker_entries(&workspace, cx);
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

    fn recent_context_picker_entries(
        &self,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Vec<Match> {
        let mut recent = Vec::with_capacity(6);

        let mut mentions = self
            .message_editor
            .read_with(cx, |message_editor, _cx| message_editor.mentions())
            .unwrap_or_default();
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);

        if let Some(agent_panel) = workspace.panel::<AgentPanel>(cx)
            && let Some(thread) = agent_panel.read(cx).active_agent_thread(cx)
        {
            let thread = thread.read(cx);
            mentions.insert(MentionUri::Thread {
                id: thread.session_id().clone(),
                name: thread.title().into(),
            });
        }

        recent.extend(
            workspace
                .recent_navigation_history_iter(cx)
                .filter(|(_, abs_path)| {
                    abs_path.as_ref().is_none_or(|path| {
                        !mentions.contains(&MentionUri::File {
                            abs_path: path.clone(),
                        })
                    })
                })
                .take(4)
                .filter_map(|(project_path, _)| {
                    project
                        .worktree_for_id(project_path.worktree_id, cx)
                        .map(|worktree| {
                            let path_prefix = worktree.read(cx).root_name().into();
                            Match::File(FileMatch {
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
                            })
                        })
                }),
        );

        if self.prompt_capabilities.get().embedded_context {
            const RECENT_COUNT: usize = 2;
            let threads = self
                .history_store
                .read(cx)
                .recently_opened_entries(cx)
                .into_iter()
                .filter(|thread| !mentions.contains(&thread.mention_uri()))
                .take(RECENT_COUNT)
                .collect::<Vec<_>>();

            recent.extend(threads.into_iter().map(Match::RecentThread));
        }

        recent
    }

    fn available_context_picker_entries(
        &self,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Vec<ContextPickerEntry> {
        let embedded_context = self.prompt_capabilities.get().embedded_context;
        let mut entries = if embedded_context {
            vec![
                ContextPickerEntry::Mode(ContextPickerMode::File),
                ContextPickerEntry::Mode(ContextPickerMode::Symbol),
                ContextPickerEntry::Mode(ContextPickerMode::Thread),
            ]
        } else {
            // File is always available, but we don't need a mode entry
            vec![]
        };

        let has_selection = workspace
            .read(cx)
            .active_item(cx)
            .and_then(|item| item.downcast::<Editor>())
            .is_some_and(|editor| {
                editor.update(cx, |editor, cx| editor.has_non_empty_selection(cx))
            });
        if has_selection {
            entries.push(ContextPickerEntry::Action(
                ContextPickerAction::AddSelections,
            ));
        }

        if embedded_context {
            if self.prompt_store.is_some() {
                entries.push(ContextPickerEntry::Mode(ContextPickerMode::Rules));
            }

            entries.push(ContextPickerEntry::Mode(ContextPickerMode::Fetch));
        }

        entries
    }
}

fn build_code_label_for_full_path(file_name: &str, directory: Option<&str>, cx: &App) -> CodeLabel {
    let comment_id = cx.theme().syntax().highlight_id("comment").map(HighlightId);
    let mut label = CodeLabel::default();

    label.push_str(file_name, None);
    label.push_str(" ", None);

    if let Some(directory) = directory {
        label.push_str(directory, comment_id);
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
        // Get the buffer state first
        let (line, offset_to_line) = buffer.update(cx, |buffer, _cx| {
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let offset_to_line = buffer.point_to_offset(line_start);
            let mut lines = buffer.text_for_range(line_start..position).lines();
            let line = lines.next().unwrap_or("");
            (line.to_string(), offset_to_line)
        });
        
        // Then check for completions outside of the buffer update
        let completion_state = {
            // First try mention completion
            if let Some(mention) = MentionCompletion::try_parse(
                self.prompt_capabilities.get().embedded_context,
                &line,
                offset_to_line,
            ) {
                Some(CompletionType::Mention(mention))
            } else if let Some(thread) = self.thread.borrow().as_ref().cloned() {
                // Then try slash command completion (only if thread supports commands)
                if let Ok(supports_commands) = thread.read_with(cx, |thread, _| {
                    thread.supports_custom_commands()
                }) {
                    if supports_commands {
                        if let Some(slash) = SlashCommandCompletion::try_parse(&line, offset_to_line) {
                            Some(CompletionType::SlashCommand(slash))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        
        let Some(completion_type) = completion_state else {
            return Task::ready(Ok(Vec::new()));
        };
        
        match completion_type {
            CompletionType::Mention(state) => self.complete_mentions(state, buffer.clone(), buffer_position, cx),
            CompletionType::SlashCommand(state) => self.complete_slash_commands(state, buffer.clone(), buffer_position, cx),
        }
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
            // Check for @ mention completions
            if let Some(completion) = MentionCompletion::try_parse(
                self.prompt_capabilities.get().embedded_context,
                line,
                offset_to_line,
            ) {
                let in_range = completion.source_range.start <= offset_to_line + position.column as usize
                    && completion.source_range.end >= offset_to_line + position.column as usize;
                if in_range {
                    return true;
                }
            }
            
            // Check for slash command completions (only if thread supports commands)
            if let Some(thread) = self.thread.borrow().as_ref().cloned() {
                if let Ok(supports_commands) = thread.read_with(cx, |thread, _| {
                    thread.supports_custom_commands()
                }) {
                    if supports_commands {
                        if let Some(completion) = SlashCommandCompletion::try_parse(line, offset_to_line) {
                            let in_range = completion.source_range.start <= offset_to_line + position.column as usize
                                && completion.source_range.end >= offset_to_line + position.column as usize;
                            return in_range;
                        }
                    }
                }
            }
            
            false
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

impl ContextPickerCompletionProvider {
    fn complete_mentions(
        &self,
        state: MentionCompletion,
        buffer: Entity<Buffer>,
        _buffer_position: Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let project = workspace.read(cx).project().clone();
        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let editor = self.message_editor.clone();

        let MentionCompletion { mode, argument, .. } = state;
        let query = argument.unwrap_or_else(|| "".to_string());

        let search_task = self.search(mode, query, Arc::<AtomicBool>::default(), cx);

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

                        Match::Thread(thread) => Some(Self::completion_for_thread(
                            thread,
                            source_range.clone(),
                            false,
                            editor.clone(),
                            cx,
                        )),

                        Match::RecentThread(thread) => Some(Self::completion_for_thread(
                            thread,
                            source_range.clone(),
                            true,
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
                is_incomplete: true,
            }])
        })
    }

    fn complete_slash_commands(
        &self,
        state: SlashCommandCompletion,
        buffer: Entity<Buffer>,
        _buffer_position: Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let Some(thread) = self.thread.borrow().as_ref().cloned() else {
            return Task::ready(Ok(Vec::new()));
        };

        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range.start)
            ..snapshot.anchor_after(state.source_range.end);

        let command_prefix = state.command_name.clone();

        cx.spawn(async move |_, cx| {
            // Get session ID and connection from the thread
            let (session_id, connection) = thread.read_with(cx, |thread, _| {
                (thread.session_id().clone(), thread.connection().clone())
            })?;

            // Fetch commands from the agent
            let commands_task = cx.update(|cx| {
                connection.list_commands(&session_id, cx)
            })?;
            let response = commands_task.await?;

            // Filter commands matching the typed prefix
            let matching_commands: Vec<_> = response.commands
                .into_iter()
                .filter(|cmd| {
                    // Support both prefix matching and fuzzy matching
                    cmd.name.starts_with(&command_prefix) ||
                    cmd.name.to_lowercase().contains(&command_prefix.to_lowercase())
                })
                .collect();

            // Convert to project::Completion following existing patterns
            let completions: Vec<_> = matching_commands
                .into_iter()
                .map(|command| {
                    let new_text = if command.requires_argument {
                        format!("/{} ", command.name) // Add space for argument
                    } else {
                        format!("/{}", command.name)
                    };

                    Completion {
                        replace_range: source_range.clone(),
                        new_text: new_text.clone(),
                        label: CodeLabel::plain(command.name.clone(), None),
                        icon_path: Some(IconName::ZedAssistant.path().into()),
                        documentation: if !command.description.is_empty() {
                            Some(CompletionDocumentation::SingleLine(command.description.clone().into()))
                        } else {
                            None
                        },
                        source: project::CompletionSource::Custom,
                        insert_text_mode: None,
                        confirm: Some(Arc::new(move |_, _, _| {
                            // For now, just insert the text - command execution will be handled later
                            false
                        })),
                    }
                })
                .collect();

            Ok(vec![CompletionResponse {
                completions,
                is_incomplete: false,
            }])
        })
    }
}

pub(crate) fn search_threads(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    history_store: &Entity<HistoryStore>,
    cx: &mut App,
) -> Task<Vec<HistoryEntry>> {
    let threads = history_store.read(cx).entries().collect();
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
                    message_editor
                        .confirm_completion(
                            crease_text,
                            start,
                            content_len,
                            mention_uri,
                            window,
                            cx,
                        )
                        .detach();
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
    fn try_parse(allow_non_file_mentions: bool, line: &str, offset_to_line: usize) -> Option<Self> {
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

            if let Some(parsed_mode) = ContextPickerMode::try_from(mode_text).ok()
                && (allow_non_file_mentions || matches!(parsed_mode, ContextPickerMode::File))
            {
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
        assert_eq!(MentionCompletion::try_parse(true, "Lorem Ipsum", 0), None);

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @", 0),
            Some(MentionCompletion {
                source_range: 6..7,
                mode: None,
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @file", 0),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: Some(ContextPickerMode::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @file ", 0),
            Some(MentionCompletion {
                source_range: 6..12,
                mode: Some(ContextPickerMode::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @file main.rs", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @file main.rs ", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @file main.rs Ipsum", 0),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(ContextPickerMode::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @main", 0),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: None,
                argument: Some("main".to_string()),
            })
        );

        assert_eq!(MentionCompletion::try_parse(true, "test@", 0), None);

        // Allowed non-file mentions

        assert_eq!(
            MentionCompletion::try_parse(true, "Lorem @symbol main", 0),
            Some(MentionCompletion {
                source_range: 6..18,
                mode: Some(ContextPickerMode::Symbol),
                argument: Some("main".to_string()),
            })
        );

        // Disallowed non-file mentions

        assert_eq!(
            MentionCompletion::try_parse(false, "Lorem @symbol main", 0),
            Some(MentionCompletion {
                source_range: 6..18,
                mode: None,
                argument: Some("main".to_string()),
            })
        );
    }
}
