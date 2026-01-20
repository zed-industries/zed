use std::cmp::Reverse;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::acp::AcpThreadHistory;
use acp_thread::{AgentSessionInfo, MentionUri};
use anyhow::Result;
use editor::{
    CompletionProvider, Editor, ExcerptId, code_context_menus::COMPLETION_MENU_MAX_WIDTH,
};
use fuzzy::{PathMatch, StringMatch, StringMatchCandidate};
use gpui::{App, BackgroundExecutor, Entity, SharedString, Task, WeakEntity};
use language::{Buffer, CodeLabel, CodeLabelBuilder, HighlightId};
use lsp::CompletionContext;
use ordered_float::OrderedFloat;
use project::lsp_store::{CompletionDocumentation, SymbolLocation};
use project::{
    Completion, CompletionDisplayOptions, CompletionIntent, CompletionResponse, DiagnosticSummary,
    PathMatchCandidateSet, Project, ProjectPath, Symbol, WorktreeId,
};
use prompt_store::{PromptStore, UserPromptId};
use rope::Point;
use text::{Anchor, ToPoint as _};
use ui::prelude::*;
use util::ResultExt as _;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use util::truncate_and_remove_front;
use workspace::Workspace;

use crate::AgentPanel;
use crate::mention_set::MentionSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PromptContextEntry {
    Mode(PromptContextType),
    Action(PromptContextAction),
}

impl PromptContextEntry {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Mode(mode) => mode.keyword(),
            Self::Action(action) => action.keyword(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PromptContextType {
    File,
    Symbol,
    Fetch,
    Thread,
    Rules,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PromptContextAction {
    AddSelections,
}

impl PromptContextAction {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::AddSelections => "selection",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::AddSelections => "Selection",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::AddSelections => IconName::Reader,
        }
    }
}

impl TryFrom<&str> for PromptContextType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "file" => Ok(Self::File),
            "symbol" => Ok(Self::Symbol),
            "fetch" => Ok(Self::Fetch),
            "thread" => Ok(Self::Thread),
            "rule" => Ok(Self::Rules),
            "diagnostics" => Ok(Self::Diagnostics),
            _ => Err(format!("Invalid context picker mode: {}", value)),
        }
    }
}

impl PromptContextType {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Fetch => "fetch",
            Self::Thread => "thread",
            Self::Rules => "rule",
            Self::Diagnostics => "diagnostics",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "Files & Directories",
            Self::Symbol => "Symbols",
            Self::Fetch => "Fetch",
            Self::Thread => "Threads",
            Self::Rules => "Rules",
            Self::Diagnostics => "Diagnostics",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::File => IconName::File,
            Self::Symbol => IconName::Code,
            Self::Fetch => IconName::ToolWeb,
            Self::Thread => IconName::Thread,
            Self::Rules => IconName::Reader,
            Self::Diagnostics => IconName::Warning,
        }
    }
}

pub(crate) enum Match {
    File(FileMatch),
    Symbol(SymbolMatch),
    Thread(AgentSessionInfo),
    RecentThread(AgentSessionInfo),
    Fetch(SharedString),
    Rules(RulesContextEntry),
    Entry(EntryMatch),
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

pub struct EntryMatch {
    mat: Option<StringMatch>,
    entry: PromptContextEntry,
}

fn session_title(session: &AgentSessionInfo) -> SharedString {
    session
        .title
        .clone()
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| SharedString::new_static("New Thread"))
}

#[derive(Debug, Clone)]
pub struct RulesContextEntry {
    pub prompt_id: UserPromptId,
    pub title: SharedString,
}

#[derive(Debug, Clone)]
pub struct AvailableCommand {
    pub name: Arc<str>,
    pub description: Arc<str>,
    pub requires_argument: bool,
}

pub trait PromptCompletionProviderDelegate: Send + Sync + 'static {
    fn supports_context(&self, mode: PromptContextType, cx: &App) -> bool {
        self.supported_modes(cx).contains(&mode)
    }
    fn supported_modes(&self, cx: &App) -> Vec<PromptContextType>;
    fn supports_images(&self, cx: &App) -> bool;

    fn available_commands(&self, cx: &App) -> Vec<AvailableCommand>;
    fn confirm_command(&self, cx: &mut App);
}

pub struct PromptCompletionProvider<T: PromptCompletionProviderDelegate> {
    source: Arc<T>,
    editor: WeakEntity<Editor>,
    mention_set: Entity<MentionSet>,
    history: WeakEntity<AcpThreadHistory>,
    prompt_store: Option<Entity<PromptStore>>,
    workspace: WeakEntity<Workspace>,
}

impl<T: PromptCompletionProviderDelegate> PromptCompletionProvider<T> {
    pub fn new(
        source: T,
        editor: WeakEntity<Editor>,
        mention_set: Entity<MentionSet>,
        history: WeakEntity<AcpThreadHistory>,
        prompt_store: Option<Entity<PromptStore>>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        Self {
            source: Arc::new(source),
            editor,
            mention_set,
            workspace,
            history,
            prompt_store,
        }
    }

    fn completion_for_entry(
        entry: PromptContextEntry,
        source_range: Range<Anchor>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        match entry {
            PromptContextEntry::Mode(mode) => Some(Completion {
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
            PromptContextEntry::Action(action) => Self::completion_for_action(
                action,
                source_range,
                editor,
                mention_set,
                workspace,
                cx,
            ),
        }
    }

    fn completion_for_thread(
        thread_entry: AgentSessionInfo,
        source_range: Range<Anchor>,
        recent: bool,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
        cx: &mut App,
    ) -> Completion {
        let title = session_title(&thread_entry);
        let uri = MentionUri::Thread {
            id: thread_entry.session_id,
            name: title.to_string(),
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
            label: CodeLabel::plain(title.to_string(), None),
            documentation: None,
            insert_text_mode: None,
            source: project::CompletionSource::Custom,
            match_start: None,
            snippet_deduplication_key: None,
            icon_path: Some(icon_for_completion),
            confirm: Some(confirm_completion_callback(
                title,
                source_range.start,
                new_text_len - 1,
                uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        }
    }

    fn completion_for_rules(
        rule: RulesContextEntry,
        source_range: Range<Anchor>,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
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
            match_start: None,
            snippet_deduplication_key: None,
            icon_path: Some(icon_path),
            confirm: Some(confirm_completion_callback(
                rule.title,
                source_range.start,
                new_text_len - 1,
                uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        }
    }

    pub(crate) fn completion_for_path(
        project_path: ProjectPath,
        path_prefix: &RelPath,
        is_recent: bool,
        is_directory: bool,
        source_range: Range<Anchor>,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
        project: Entity<Project>,
        label_max_chars: usize,
        cx: &mut App,
    ) -> Option<Completion> {
        let path_style = project.read(cx).path_style(cx);
        let (file_name, directory) =
            extract_file_name_and_directory(&project_path.path, path_prefix, path_style);

        let label = build_code_label_for_path(
            &file_name,
            directory.as_ref().map(|s| s.as_ref()),
            None,
            label_max_chars,
            cx,
        );

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
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                file_name,
                source_range.start,
                new_text_len - 1,
                uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        })
    }

    fn completion_for_symbol(
        symbol: Symbol,
        source_range: Range<Anchor>,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
        label_max_chars: usize,
        cx: &mut App,
    ) -> Option<Completion> {
        let project = workspace.read(cx).project().clone();

        let (abs_path, file_name) = match &symbol.path {
            SymbolLocation::InProject(project_path) => (
                project.read(cx).absolute_path(&project_path, cx)?,
                project_path.path.file_name()?.to_string().into(),
            ),
            SymbolLocation::OutsideProject {
                abs_path,
                signature: _,
            } => (
                PathBuf::from(abs_path.as_ref()),
                abs_path.file_name().map(|f| f.to_string_lossy())?,
            ),
        };

        let label = build_code_label_for_path(
            &symbol.name,
            Some(&file_name),
            Some(symbol.range.start.0.row + 1),
            label_max_chars,
            cx,
        );

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
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                symbol.name.into(),
                source_range.start,
                new_text_len - 1,
                uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        })
    }

    fn completion_for_fetch(
        source_range: Range<Anchor>,
        url_to_fetch: SharedString,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
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
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                url_to_fetch.to_string().into(),
                source_range.start,
                new_text.len() - 1,
                mention_uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        })
    }

    pub(crate) fn completion_for_action(
        action: PromptContextAction,
        source_range: Range<Anchor>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Option<Completion> {
        let (new_text, on_action) = match action {
            PromptContextAction::AddSelections => {
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
                        let editor = editor.clone();
                        let selections = selections.clone();
                        let mention_set = mention_set.clone();
                        let source_range = source_range.clone();
                        window.defer(cx, move |window, cx| {
                            if let Some(editor) = editor.upgrade() {
                                mention_set
                                    .update(cx, |store, cx| {
                                        store.confirm_mention_for_selection(
                                            source_range,
                                            selections,
                                            editor,
                                            window,
                                            cx,
                                        )
                                    })
                                    .ok();
                            }
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
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            // This ensures that when a user accepts this completion, the
            // completion menu will still be shown after "@category " is
            // inserted
            confirm: Some(on_action),
        })
    }

    fn completion_for_diagnostics(
        source_range: Range<Anchor>,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
        cx: &mut App,
    ) -> Vec<Completion> {
        let summary = workspace
            .read(cx)
            .project()
            .read(cx)
            .diagnostic_summary(false, cx);
        if summary.error_count == 0 && summary.warning_count == 0 {
            return Vec::new();
        }
        let icon_path = MentionUri::Diagnostics {
            include_errors: true,
            include_warnings: false,
        }
        .icon_path(cx);

        let mut completions = Vec::new();

        let cases = [
            (summary.error_count > 0, true, false),
            (summary.warning_count > 0, false, true),
            (
                summary.error_count > 0 && summary.warning_count > 0,
                true,
                true,
            ),
        ];

        for (condition, include_errors, include_warnings) in cases {
            if condition {
                completions.push(Self::build_diagnostics_completion(
                    diagnostics_submenu_label(summary, include_errors, include_warnings),
                    source_range.clone(),
                    source.clone(),
                    editor.clone(),
                    mention_set.clone(),
                    workspace.clone(),
                    icon_path.clone(),
                    include_errors,
                    include_warnings,
                    summary,
                ));
            }
        }

        completions
    }

    fn build_diagnostics_completion(
        menu_label: String,
        source_range: Range<Anchor>,
        source: Arc<T>,
        editor: WeakEntity<Editor>,
        mention_set: WeakEntity<MentionSet>,
        workspace: Entity<Workspace>,
        icon_path: SharedString,
        include_errors: bool,
        include_warnings: bool,
        summary: DiagnosticSummary,
    ) -> Completion {
        let uri = MentionUri::Diagnostics {
            include_errors,
            include_warnings,
        };
        let crease_text = diagnostics_crease_label(summary, include_errors, include_warnings);
        let display_text = format!("@{}", crease_text);
        let new_text = format!("[{}]({}) ", display_text, uri.to_uri());
        let new_text_len = new_text.len();
        Completion {
            replace_range: source_range.clone(),
            new_text,
            label: CodeLabel::plain(menu_label, None),
            documentation: None,
            source: project::CompletionSource::Custom,
            icon_path: Some(icon_path),
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: Some(confirm_completion_callback(
                crease_text,
                source_range.start,
                new_text_len - 1,
                uri,
                source,
                editor,
                mention_set,
                workspace,
            )),
        }
    }

    fn search_slash_commands(&self, query: String, cx: &mut App) -> Task<Vec<AvailableCommand>> {
        let commands = self.source.available_commands(cx);
        if commands.is_empty() {
            return Task::ready(Vec::new());
        }

        cx.spawn(async move |cx| {
            let candidates = commands
                .iter()
                .enumerate()
                .map(|(id, command)| StringMatchCandidate::new(id, &command.name))
                .collect::<Vec<_>>();

            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                100,
                &Arc::new(AtomicBool::default()),
                cx.background_executor().clone(),
            )
            .await;

            matches
                .into_iter()
                .map(|mat| commands[mat.candidate_id].clone())
                .collect()
        })
    }

    fn search_mentions(
        &self,
        mode: Option<PromptContextType>,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut App,
    ) -> Task<Vec<Match>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Vec::default());
        };
        match mode {
            Some(PromptContextType::File) => {
                let search_files_task = search_files(query, cancellation_flag, &workspace, cx);
                cx.background_spawn(async move {
                    search_files_task
                        .await
                        .into_iter()
                        .map(Match::File)
                        .collect()
                })
            }

            Some(PromptContextType::Symbol) => {
                let search_symbols_task = search_symbols(query, cancellation_flag, &workspace, cx);
                cx.background_spawn(async move {
                    search_symbols_task
                        .await
                        .into_iter()
                        .map(Match::Symbol)
                        .collect()
                })
            }

            Some(PromptContextType::Thread) => {
                if let Some(history) = self.history.upgrade() {
                    let sessions = history.read(cx).sessions().to_vec();
                    let search_task =
                        filter_sessions_by_query(query, cancellation_flag, sessions, cx);
                    cx.spawn(async move |_cx| {
                        search_task.await.into_iter().map(Match::Thread).collect()
                    })
                } else {
                    Task::ready(Vec::new())
                }
            }

            Some(PromptContextType::Fetch) => {
                if !query.is_empty() {
                    Task::ready(vec![Match::Fetch(query.into())])
                } else {
                    Task::ready(Vec::new())
                }
            }

            Some(PromptContextType::Rules) => {
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

            Some(PromptContextType::Diagnostics) => Task::ready(Vec::new()),

            None if query.is_empty() => {
                let recent_task = self.recent_context_picker_entries(&workspace, cx);
                let entries = self
                    .available_context_picker_entries(&workspace, cx)
                    .into_iter()
                    .map(|mode| {
                        Match::Entry(EntryMatch {
                            entry: mode,
                            mat: None,
                        })
                    })
                    .collect::<Vec<_>>();

                cx.spawn(async move |_cx| {
                    let mut matches = recent_task.await;
                    matches.extend(entries);
                    matches
                })
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
    ) -> Task<Vec<Match>> {
        let mut recent = Vec::with_capacity(6);

        let mut mentions = self
            .mention_set
            .read_with(cx, |store, _cx| store.mentions());
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);
        let include_root_name = workspace.visible_worktrees(cx).count() > 1;

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
                            let path_prefix = if include_root_name {
                                worktree.read(cx).root_name().into()
                            } else {
                                RelPath::empty().into()
                            };
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

        if !self.source.supports_context(PromptContextType::Thread, cx) {
            return Task::ready(recent);
        }

        if let Some(history) = self.history.upgrade() {
            const RECENT_COUNT: usize = 2;
            recent.extend(
                history
                    .read(cx)
                    .sessions()
                    .into_iter()
                    .filter(|session| {
                        let uri = MentionUri::Thread {
                            id: session.session_id.clone(),
                            name: session_title(session).to_string(),
                        };
                        !mentions.contains(&uri)
                    })
                    .take(RECENT_COUNT)
                    .cloned()
                    .map(Match::RecentThread),
            );
            return Task::ready(recent);
        }

        Task::ready(recent)
    }

    fn available_context_picker_entries(
        &self,
        workspace: &Entity<Workspace>,
        cx: &mut App,
    ) -> Vec<PromptContextEntry> {
        let mut entries = vec![
            PromptContextEntry::Mode(PromptContextType::File),
            PromptContextEntry::Mode(PromptContextType::Symbol),
        ];

        if self.source.supports_context(PromptContextType::Thread, cx) {
            entries.push(PromptContextEntry::Mode(PromptContextType::Thread));
        }

        let has_selection = workspace
            .read(cx)
            .active_item(cx)
            .and_then(|item| item.downcast::<Editor>())
            .is_some_and(|editor| {
                editor.update(cx, |editor, cx| {
                    editor.has_non_empty_selection(&editor.display_snapshot(cx))
                })
            });
        if has_selection {
            entries.push(PromptContextEntry::Action(
                PromptContextAction::AddSelections,
            ));
        }

        if self.prompt_store.is_some() && self.source.supports_context(PromptContextType::Rules, cx)
        {
            entries.push(PromptContextEntry::Mode(PromptContextType::Rules));
        }

        if self.source.supports_context(PromptContextType::Fetch, cx) {
            entries.push(PromptContextEntry::Mode(PromptContextType::Fetch));
        }

        if self
            .source
            .supports_context(PromptContextType::Diagnostics, cx)
        {
            let summary = workspace
                .read(cx)
                .project()
                .read(cx)
                .diagnostic_summary(false, cx);
            if summary.error_count > 0 || summary.warning_count > 0 {
                entries.push(PromptContextEntry::Mode(PromptContextType::Diagnostics));
            }
        }

        entries
    }
}

impl<T: PromptCompletionProviderDelegate> CompletionProvider for PromptCompletionProvider<T> {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: Anchor,
        _trigger: CompletionContext,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let state = buffer.update(cx, |buffer, cx| {
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let offset_to_line = buffer.point_to_offset(line_start);
            let mut lines = buffer.text_for_range(line_start..position).lines();
            let line = lines.next()?;
            PromptCompletion::try_parse(line, offset_to_line, &self.source.supported_modes(cx))
        });
        let Some(state) = state else {
            return Task::ready(Ok(Vec::new()));
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let project = workspace.read(cx).project().clone();
        let snapshot = buffer.read(cx).snapshot();
        let source_range = snapshot.anchor_before(state.source_range().start)
            ..snapshot.anchor_after(state.source_range().end);

        let source = self.source.clone();
        let editor = self.editor.clone();
        let mention_set = self.mention_set.downgrade();
        match state {
            PromptCompletion::SlashCommand(SlashCommandCompletion {
                command, argument, ..
            }) => {
                let search_task = self.search_slash_commands(command.unwrap_or_default(), cx);
                cx.background_spawn(async move {
                    let completions = search_task
                        .await
                        .into_iter()
                        .map(|command| {
                            let new_text = if let Some(argument) = argument.as_ref() {
                                format!("/{} {}", command.name, argument)
                            } else {
                                format!("/{} ", command.name)
                            };

                            let is_missing_argument =
                                command.requires_argument && argument.is_none();
                            Completion {
                                replace_range: source_range.clone(),
                                new_text,
                                label: CodeLabel::plain(command.name.to_string(), None),
                                documentation: Some(CompletionDocumentation::MultiLinePlainText(
                                    command.description.into(),
                                )),
                                source: project::CompletionSource::Custom,
                                icon_path: None,
                                match_start: None,
                                snippet_deduplication_key: None,
                                insert_text_mode: None,
                                confirm: Some(Arc::new({
                                    let source = source.clone();
                                    move |intent, _window, cx| {
                                        if !is_missing_argument {
                                            cx.defer({
                                                let source = source.clone();
                                                move |cx| match intent {
                                                    CompletionIntent::Complete
                                                    | CompletionIntent::CompleteWithInsert
                                                    | CompletionIntent::CompleteWithReplace => {
                                                        source.confirm_command(cx);
                                                    }
                                                    CompletionIntent::Compose => {}
                                                }
                                            });
                                        }
                                        false
                                    }
                                })),
                            }
                        })
                        .collect();

                    Ok(vec![CompletionResponse {
                        completions,
                        display_options: CompletionDisplayOptions {
                            dynamic_width: true,
                        },
                        // Since this does its own filtering (see `filter_completions()` returns false),
                        // there is no benefit to computing whether this set of completions is incomplete.
                        is_incomplete: true,
                    }])
                })
            }
            PromptCompletion::Mention(MentionCompletion { mode, argument, .. }) => {
                if let Some(PromptContextType::Diagnostics) = mode {
                    if argument.is_some() {
                        return Task::ready(Ok(Vec::new()));
                    }

                    let completions = Self::completion_for_diagnostics(
                        source_range.clone(),
                        source.clone(),
                        editor.clone(),
                        mention_set.clone(),
                        workspace.clone(),
                        cx,
                    );
                    if !completions.is_empty() {
                        return Task::ready(Ok(vec![CompletionResponse {
                            completions,
                            display_options: CompletionDisplayOptions::default(),
                            is_incomplete: false,
                        }]));
                    }
                }

                let query = argument.unwrap_or_default();
                let search_task =
                    self.search_mentions(mode, query, Arc::<AtomicBool>::default(), cx);

                // Calculate maximum characters available for the full label (file_name + space + directory)
                // based on maximum menu width after accounting for padding, spacing, and icon width
                let label_max_chars = {
                    // Base06 left padding + Base06 gap + Base06 right padding + icon width
                    let used_pixels = DynamicSpacing::Base06.px(cx) * 3.0
                        + IconSize::XSmall.rems() * window.rem_size();

                    let style = window.text_style();
                    let font_id = window.text_system().resolve_font(&style.font());
                    let font_size = TextSize::Small.rems(cx).to_pixels(window.rem_size());

                    // Fallback em_width of 10px matches file_finder.rs fallback for TextSize::Small
                    let em_width = cx
                        .text_system()
                        .em_width(font_id, font_size)
                        .unwrap_or(px(10.0));

                    // Calculate available pixels for text (file_name + directory)
                    // Using max width since dynamic_width allows the menu to expand up to this
                    let available_pixels = COMPLETION_MENU_MAX_WIDTH - used_pixels;

                    // Convert to character count (total available for file_name + directory)
                    (f32::from(available_pixels) / f32::from(em_width)) as usize
                };

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

                                    Self::completion_for_path(
                                        project_path,
                                        &path_prefix,
                                        is_recent,
                                        mat.is_dir,
                                        source_range.clone(),
                                        source.clone(),
                                        editor.clone(),
                                        mention_set.clone(),
                                        workspace.clone(),
                                        project.clone(),
                                        label_max_chars,
                                        cx,
                                    )
                                }
                                Match::Symbol(SymbolMatch { symbol, .. }) => {
                                    Self::completion_for_symbol(
                                        symbol,
                                        source_range.clone(),
                                        source.clone(),
                                        editor.clone(),
                                        mention_set.clone(),
                                        workspace.clone(),
                                        label_max_chars,
                                        cx,
                                    )
                                }
                                Match::Thread(thread) => Some(Self::completion_for_thread(
                                    thread,
                                    source_range.clone(),
                                    false,
                                    source.clone(),
                                    editor.clone(),
                                    mention_set.clone(),
                                    workspace.clone(),
                                    cx,
                                )),
                                Match::RecentThread(thread) => Some(Self::completion_for_thread(
                                    thread,
                                    source_range.clone(),
                                    true,
                                    source.clone(),
                                    editor.clone(),
                                    mention_set.clone(),
                                    workspace.clone(),
                                    cx,
                                )),
                                Match::Rules(user_rules) => Some(Self::completion_for_rules(
                                    user_rules,
                                    source_range.clone(),
                                    source.clone(),
                                    editor.clone(),
                                    mention_set.clone(),
                                    workspace.clone(),
                                    cx,
                                )),
                                Match::Fetch(url) => Self::completion_for_fetch(
                                    source_range.clone(),
                                    url,
                                    source.clone(),
                                    editor.clone(),
                                    mention_set.clone(),
                                    workspace.clone(),
                                    cx,
                                ),
                                Match::Entry(EntryMatch { entry, .. }) => {
                                    Self::completion_for_entry(
                                        entry,
                                        source_range.clone(),
                                        editor.clone(),
                                        mention_set.clone(),
                                        &workspace,
                                        cx,
                                    )
                                }
                            })
                            .collect::<Vec<_>>()
                    });

                    Ok(vec![CompletionResponse {
                        completions,
                        display_options: CompletionDisplayOptions {
                            dynamic_width: true,
                        },
                        // Since this does its own filtering (see `filter_completions()` returns false),
                        // there is no benefit to computing whether this set of completions is incomplete.
                        is_incomplete: true,
                    }])
                })
            }
        }
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let buffer = buffer.read(cx);
        let position = position.to_point(buffer);
        let line_start = Point::new(position.row, 0);
        let offset_to_line = buffer.point_to_offset(line_start);
        let mut lines = buffer.text_for_range(line_start..position).lines();
        if let Some(line) = lines.next() {
            PromptCompletion::try_parse(line, offset_to_line, &self.source.supported_modes(cx))
                .filter(|completion| {
                    // Right now we don't support completing arguments of slash commands
                    let is_slash_command_with_argument = matches!(
                        completion,
                        PromptCompletion::SlashCommand(SlashCommandCompletion {
                            argument: Some(_),
                            ..
                        })
                    );
                    !is_slash_command_with_argument
                })
                .map(|completion| {
                    completion.source_range().start <= offset_to_line + position.column as usize
                        && completion.source_range().end
                            >= offset_to_line + position.column as usize
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

fn confirm_completion_callback<T: PromptCompletionProviderDelegate>(
    crease_text: SharedString,
    start: Anchor,
    content_len: usize,
    mention_uri: MentionUri,
    source: Arc<T>,
    editor: WeakEntity<Editor>,
    mention_set: WeakEntity<MentionSet>,
    workspace: Entity<Workspace>,
) -> Arc<dyn Fn(CompletionIntent, &mut Window, &mut App) -> bool + Send + Sync> {
    Arc::new(move |_, window, cx| {
        let source = source.clone();
        let editor = editor.clone();
        let mention_set = mention_set.clone();
        let crease_text = crease_text.clone();
        let mention_uri = mention_uri.clone();
        let workspace = workspace.clone();
        window.defer(cx, move |window, cx| {
            if let Some(editor) = editor.upgrade() {
                mention_set
                    .clone()
                    .update(cx, |mention_set, cx| {
                        mention_set
                            .confirm_mention_completion(
                                crease_text,
                                start,
                                content_len,
                                mention_uri,
                                source.supports_images(cx),
                                editor,
                                &workspace,
                                window,
                                cx,
                            )
                            .detach();
                    })
                    .ok();
            }
        });
        false
    })
}

#[derive(Debug, PartialEq)]
enum PromptCompletion {
    SlashCommand(SlashCommandCompletion),
    Mention(MentionCompletion),
}

impl PromptCompletion {
    fn source_range(&self) -> Range<usize> {
        match self {
            Self::SlashCommand(completion) => completion.source_range.clone(),
            Self::Mention(completion) => completion.source_range.clone(),
        }
    }

    fn try_parse(
        line: &str,
        offset_to_line: usize,
        supported_modes: &[PromptContextType],
    ) -> Option<Self> {
        if line.contains('@') {
            if let Some(mention) =
                MentionCompletion::try_parse(line, offset_to_line, supported_modes)
            {
                return Some(Self::Mention(mention));
            }
        }
        SlashCommandCompletion::try_parse(line, offset_to_line).map(Self::SlashCommand)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SlashCommandCompletion {
    pub source_range: Range<usize>,
    pub command: Option<String>,
    pub argument: Option<String>,
}

impl SlashCommandCompletion {
    pub fn try_parse(line: &str, offset_to_line: usize) -> Option<Self> {
        // If we decide to support commands that are not at the beginning of the prompt, we can remove this check
        if !line.starts_with('/') || offset_to_line != 0 {
            return None;
        }

        let (prefix, last_command) = line.rsplit_once('/')?;
        if prefix.chars().last().is_some_and(|c| !c.is_whitespace())
            || last_command.starts_with(char::is_whitespace)
        {
            return None;
        }

        let mut argument = None;
        let mut command = None;
        if let Some((command_text, args)) = last_command.split_once(char::is_whitespace) {
            if !args.is_empty() {
                argument = Some(args.trim_end().to_string());
            }
            command = Some(command_text.to_string());
        } else if !last_command.is_empty() {
            command = Some(last_command.to_string());
        };

        Some(Self {
            source_range: prefix.len() + offset_to_line
                ..line
                    .rfind(|c: char| !c.is_whitespace())
                    .unwrap_or_else(|| line.len())
                    + 1
                    + offset_to_line,
            command,
            argument,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
struct MentionCompletion {
    source_range: Range<usize>,
    mode: Option<PromptContextType>,
    argument: Option<String>,
}

impl MentionCompletion {
    fn try_parse(
        line: &str,
        offset_to_line: usize,
        supported_modes: &[PromptContextType],
    ) -> Option<Self> {
        let last_mention_start = line.rfind('@')?;

        // No whitespace immediately after '@'
        if line[last_mention_start + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_whitespace())
        {
            return None;
        }

        //  Must be a word boundary before '@'
        if last_mention_start > 0
            && line[..last_mention_start]
                .chars()
                .last()
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
            // Safe since we check no leading whitespace above
            end += mode_text.len();

            if let Some(parsed_mode) = PromptContextType::try_from(mode_text).ok()
                && supported_modes.contains(&parsed_mode)
            {
                mode = Some(parsed_mode);
            } else {
                argument = Some(mode_text.to_string());
            }
            match rest_of_line[mode_text.len()..].find(|c: char| !c.is_whitespace()) {
                Some(whitespace_count) => {
                    if let Some(argument_text) = parts.next() {
                        // If mode wasn't recognized but we have an argument, don't suggest completions
                        // (e.g. '@something word')
                        if mode.is_none() && !argument_text.is_empty() {
                            return None;
                        }

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

fn diagnostics_label(
    summary: DiagnosticSummary,
    include_errors: bool,
    include_warnings: bool,
) -> String {
    let mut parts = Vec::new();

    if include_errors && summary.error_count > 0 {
        parts.push(format!(
            "{} {}",
            summary.error_count,
            pluralize("error", summary.error_count)
        ));
    }

    if include_warnings && summary.warning_count > 0 {
        parts.push(format!(
            "{} {}",
            summary.warning_count,
            pluralize("warning", summary.warning_count)
        ));
    }

    if parts.is_empty() {
        return "Diagnostics".into();
    }

    let body = if parts.len() == 2 {
        format!("{} and {}", parts[0], parts[1])
    } else {
        parts
            .pop()
            .expect("at least one part present after non-empty check")
    };

    format!("Diagnostics: {body}")
}

fn diagnostics_submenu_label(
    summary: DiagnosticSummary,
    include_errors: bool,
    include_warnings: bool,
) -> String {
    match (include_errors, include_warnings) {
        (true, true) => format!(
            "{} {} & {} {}",
            summary.error_count,
            pluralize("error", summary.error_count),
            summary.warning_count,
            pluralize("warning", summary.warning_count)
        ),
        (true, _) => format!(
            "{} {}",
            summary.error_count,
            pluralize("error", summary.error_count)
        ),
        (_, true) => format!(
            "{} {}",
            summary.warning_count,
            pluralize("warning", summary.warning_count)
        ),
        _ => "Diagnostics".into(),
    }
}

fn diagnostics_crease_label(
    summary: DiagnosticSummary,
    include_errors: bool,
    include_warnings: bool,
) -> SharedString {
    diagnostics_label(summary, include_errors, include_warnings).into()
}

fn pluralize(noun: &str, count: usize) -> String {
    if count == 1 {
        noun.to_string()
    } else {
        format!("{noun}s")
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
        let (visible_match_candidates, external_match_candidates): (Vec<_>, Vec<_>) = project
            .update(cx, |project, cx| {
                symbols
                    .iter()
                    .enumerate()
                    .map(|(id, symbol)| StringMatchCandidate::new(id, symbol.label.filter_text()))
                    .partition(|candidate| match &symbols[candidate.id].path {
                        SymbolLocation::InProject(project_path) => project
                            .entry_for_path(project_path, cx)
                            .is_some_and(|e| !e.is_ignored),
                        SymbolLocation::OutsideProject { .. } => false,
                    })
            });
        // Try to support rust-analyzer's path based symbols feature which
        // allows to search by rust path syntax, in that case we only want to
        // filter names by the last segment
        // Ideally this was a first class LSP feature (rich queries)
        let query = query
            .rsplit_once("::")
            .map_or(&*query, |(_, suffix)| suffix)
            .to_owned();
        // Note if you make changes to this filtering below, also change `project_symbols::ProjectSymbolsDelegate::filter`
        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.foreground_executor().block_on(fuzzy::match_strings(
            &visible_match_candidates,
            &query,
            false,
            true,
            MAX_MATCHES,
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let mut external_matches = cx.foreground_executor().block_on(fuzzy::match_strings(
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

fn filter_sessions_by_query(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    sessions: Vec<AgentSessionInfo>,
    cx: &mut App,
) -> Task<Vec<AgentSessionInfo>> {
    if query.is_empty() {
        return Task::ready(sessions);
    }
    let executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        filter_sessions(query, cancellation_flag, sessions, executor).await
    })
}

async fn filter_sessions(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    sessions: Vec<AgentSessionInfo>,
    executor: BackgroundExecutor,
) -> Vec<AgentSessionInfo> {
    let titles = sessions.iter().map(session_title).collect::<Vec<_>>();
    let candidates = titles
        .iter()
        .enumerate()
        .map(|(id, title)| StringMatchCandidate::new(id, title.as_ref()))
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
        .map(|mat| sessions[mat.candidate_id].clone())
        .collect()
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
                    Some(RulesContextEntry {
                        prompt_id: metadata.id.as_user()?,
                        title: metadata.title?,
                    })
                }
            })
            .collect::<Vec<_>>()
    })
}

pub struct SymbolMatch {
    pub symbol: Symbol,
}

pub struct FileMatch {
    pub mat: PathMatch,
    pub is_recent: bool,
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

fn build_code_label_for_path(
    file: &str,
    directory: Option<&str>,
    line_number: Option<u32>,
    label_max_chars: usize,
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
        let file_name_chars = file.chars().count();
        // Account for: file_name + space (ellipsis is handled by truncate_and_remove_front)
        let directory_max_chars = label_max_chars
            .saturating_sub(file_name_chars)
            .saturating_sub(1);
        let truncated_directory = truncate_and_remove_front(directory, directory_max_chars.max(5));
        label.push_str(&truncated_directory, variable_highlight_id);
    }
    if let Some(line_number) = line_number {
        label.push_str(&format!(" L{}", line_number), variable_highlight_id);
    }
    label.build()
}

fn selection_ranges(
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Vec<(Entity<Buffer>, Range<text::Anchor>)> {
    let Some(editor) = workspace
        .read(cx)
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return Vec::new();
    };

    editor.update(cx, |editor, cx| {
        let selections = editor.selections.all_adjusted(&editor.display_snapshot(cx));

        let buffer = editor.buffer().clone().read(cx);
        let snapshot = buffer.snapshot(cx);

        selections
            .into_iter()
            .map(|s| snapshot.anchor_after(s.start)..snapshot.anchor_before(s.end))
            .flat_map(|range| {
                let (start_buffer, start) = buffer.text_anchor_for_position(range.start, cx)?;
                let (end_buffer, end) = buffer.text_anchor_for_position(range.end, cx)?;
                if start_buffer != end_buffer {
                    return None;
                }
                Some((start_buffer, start..end))
            })
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn test_prompt_completion_parse() {
        let supported_modes = vec![PromptContextType::File, PromptContextType::Symbol];

        assert_eq!(
            PromptCompletion::try_parse("/", 0, &supported_modes),
            Some(PromptCompletion::SlashCommand(SlashCommandCompletion {
                source_range: 0..1,
                command: None,
                argument: None,
            }))
        );

        assert_eq!(
            PromptCompletion::try_parse("@", 0, &supported_modes),
            Some(PromptCompletion::Mention(MentionCompletion {
                source_range: 0..1,
                mode: None,
                argument: None,
            }))
        );

        assert_eq!(
            PromptCompletion::try_parse("/test @file", 0, &supported_modes),
            Some(PromptCompletion::Mention(MentionCompletion {
                source_range: 6..11,
                mode: Some(PromptContextType::File),
                argument: None,
            }))
        );
    }

    #[test]
    fn test_slash_command_completion_parse() {
        assert_eq!(
            SlashCommandCompletion::try_parse("/", 0),
            Some(SlashCommandCompletion {
                source_range: 0..1,
                command: None,
                argument: None,
            })
        );

        assert_eq!(
            SlashCommandCompletion::try_parse("/help", 0),
            Some(SlashCommandCompletion {
                source_range: 0..5,
                command: Some("help".to_string()),
                argument: None,
            })
        );

        assert_eq!(
            SlashCommandCompletion::try_parse("/help ", 0),
            Some(SlashCommandCompletion {
                source_range: 0..5,
                command: Some("help".to_string()),
                argument: None,
            })
        );

        assert_eq!(
            SlashCommandCompletion::try_parse("/help arg1", 0),
            Some(SlashCommandCompletion {
                source_range: 0..10,
                command: Some("help".to_string()),
                argument: Some("arg1".to_string()),
            })
        );

        assert_eq!(
            SlashCommandCompletion::try_parse("/help arg1 arg2", 0),
            Some(SlashCommandCompletion {
                source_range: 0..15,
                command: Some("help".to_string()),
                argument: Some("arg1 arg2".to_string()),
            })
        );

        assert_eq!(
            SlashCommandCompletion::try_parse("/拿不到命令 拿不到命令 ", 0),
            Some(SlashCommandCompletion {
                source_range: 0..30,
                command: Some("拿不到命令".to_string()),
                argument: Some("拿不到命令".to_string()),
            })
        );

        assert_eq!(SlashCommandCompletion::try_parse("Lorem Ipsum", 0), None);

        assert_eq!(SlashCommandCompletion::try_parse("Lorem /", 0), None);

        assert_eq!(SlashCommandCompletion::try_parse("Lorem /help", 0), None);

        assert_eq!(SlashCommandCompletion::try_parse("Lorem/", 0), None);

        assert_eq!(SlashCommandCompletion::try_parse("/ ", 0), None);
    }

    #[test]
    fn test_mention_completion_parse() {
        let supported_modes = vec![PromptContextType::File, PromptContextType::Symbol];
        let supported_modes_with_diagnostics = vec![
            PromptContextType::File,
            PromptContextType::Symbol,
            PromptContextType::Diagnostics,
        ];

        assert_eq!(
            MentionCompletion::try_parse("Lorem Ipsum", 0, &supported_modes),
            None
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..7,
                mode: None,
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: Some(PromptContextType::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file ", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..12,
                mode: Some(PromptContextType::File),
                argument: None,
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(PromptContextType::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs ", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(PromptContextType::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @file main.rs Ipsum", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..19,
                mode: Some(PromptContextType::File),
                argument: Some("main.rs".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @main", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..11,
                mode: None,
                argument: Some("main".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @main ", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..12,
                mode: None,
                argument: Some("main".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @main m", 0, &supported_modes),
            None
        );

        assert_eq!(
            MentionCompletion::try_parse("test@", 0, &supported_modes),
            None
        );

        // Allowed non-file mentions

        assert_eq!(
            MentionCompletion::try_parse("Lorem @symbol main", 0, &supported_modes),
            Some(MentionCompletion {
                source_range: 6..18,
                mode: Some(PromptContextType::Symbol),
                argument: Some("main".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(
                "Lorem @symbol agent_ui::completion_provider",
                0,
                &supported_modes
            ),
            Some(MentionCompletion {
                source_range: 6..43,
                mode: Some(PromptContextType::Symbol),
                argument: Some("agent_ui::completion_provider".to_string()),
            })
        );

        assert_eq!(
            MentionCompletion::try_parse(
                "Lorem @diagnostics",
                0,
                &supported_modes_with_diagnostics
            ),
            Some(MentionCompletion {
                source_range: 6..18,
                mode: Some(PromptContextType::Diagnostics),
                argument: None,
            })
        );

        // Disallowed non-file mentions
        assert_eq!(
            MentionCompletion::try_parse("Lorem @symbol main", 0, &[PromptContextType::File]),
            None
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem@symbol", 0, &supported_modes),
            None,
            "Should not parse mention inside word"
        );

        assert_eq!(
            MentionCompletion::try_parse("Lorem @ file", 0, &supported_modes),
            None,
            "Should not parse with a space after @"
        );

        assert_eq!(
            MentionCompletion::try_parse("@ file", 0, &supported_modes),
            None,
            "Should not parse with a space after @ at the start of the line"
        );
    }

    #[gpui::test]
    async fn test_filter_sessions_by_query(cx: &mut TestAppContext) {
        let mut alpha = AgentSessionInfo::new("session-alpha");
        alpha.title = Some("Alpha Session".into());
        let mut beta = AgentSessionInfo::new("session-beta");
        beta.title = Some("Beta Session".into());

        let sessions = vec![alpha.clone(), beta];

        let task = {
            let mut app = cx.app.borrow_mut();
            filter_sessions_by_query(
                "Alpha".into(),
                Arc::new(AtomicBool::default()),
                sessions,
                &mut app,
            )
        };

        let results = task.await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].session_id, alpha.session_id);
    }
}
