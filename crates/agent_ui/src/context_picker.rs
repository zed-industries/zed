use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use agent::{HistoryEntry, HistoryEntryId, HistoryStore};
use agent_client_protocol as acp;
use anyhow::Result;
use collections::HashSet;
use editor::Editor;
use gpui::{App, Entity, WeakEntity};
use language::Buffer;
use project::ProjectPath;
use prompt_store::{PromptStore, UserPromptId};
use ui::{IconName, SharedString};
use util::rel_path::RelPath;
use workspace::Workspace;

use crate::{context::RULES_ICON, context_store::ContextStore};

#[derive(Debug, Clone)]
pub struct RulesContextEntry {
    pub prompt_id: UserPromptId,
    pub title: SharedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextPickerEntry {
    Mode(ContextType),
    Action(ContextAction),
}

impl ContextPickerEntry {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Mode(mode) => mode.keyword(),
            Self::Action(action) => action.keyword(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextType {
    File,
    Symbol,
    Fetch,
    Thread,
    Rules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextAction {
    AddSelections,
}

impl ContextAction {
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

impl TryFrom<&str> for ContextType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "file" => Ok(Self::File),
            "symbol" => Ok(Self::Symbol),
            "fetch" => Ok(Self::Fetch),
            "thread" => Ok(Self::Thread),
            "rule" => Ok(Self::Rules),
            _ => Err(format!("Invalid context picker mode: {}", value)),
        }
    }
}

impl ContextType {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Fetch => "fetch",
            Self::Thread => "thread",
            Self::Rules => "rule",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "Files & Directories",
            Self::Symbol => "Symbols",
            Self::Fetch => "Fetch",
            Self::Thread => "Threads",
            Self::Rules => "Rules",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::File => IconName::File,
            Self::Symbol => IconName::Code,
            Self::Fetch => IconName::ToolWeb,
            Self::Thread => IconName::Thread,
            Self::Rules => RULES_ICON,
        }
    }
}

pub(crate) enum RecentEntry {
    File {
        project_path: ProjectPath,
        path_prefix: Arc<RelPath>,
    },
    Thread(HistoryEntry),
}

pub(crate) fn available_context_picker_entries(
    prompt_store: &Option<WeakEntity<PromptStore>>,
    thread_store: &Option<WeakEntity<HistoryStore>>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Vec<ContextPickerEntry> {
    let mut entries = vec![
        ContextPickerEntry::Mode(ContextType::File),
        ContextPickerEntry::Mode(ContextType::Symbol),
    ];

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
        entries.push(ContextPickerEntry::Action(ContextAction::AddSelections));
    }

    if thread_store.is_some() {
        entries.push(ContextPickerEntry::Mode(ContextType::Thread));
    }

    if prompt_store.is_some() {
        entries.push(ContextPickerEntry::Mode(ContextType::Rules));
    }

    entries.push(ContextPickerEntry::Mode(ContextType::Fetch));

    entries
}

pub(crate) fn recent_context_picker_entries_with_store(
    context_store: Entity<ContextStore>,
    thread_store: Option<WeakEntity<HistoryStore>>,
    workspace: Entity<Workspace>,
    exclude_path: Option<ProjectPath>,
    cx: &App,
) -> Vec<RecentEntry> {
    let project = workspace.read(cx).project();

    let mut exclude_paths = context_store.read(cx).file_paths(cx);
    exclude_paths.extend(exclude_path);

    let exclude_paths = exclude_paths
        .into_iter()
        .filter_map(|project_path| project.read(cx).absolute_path(&project_path, cx))
        .collect();

    let exclude_threads = context_store.read(cx).thread_ids();

    recent_context_picker_entries(thread_store, workspace, &exclude_paths, exclude_threads, cx)
}

pub(crate) fn recent_context_picker_entries(
    thread_store: Option<WeakEntity<HistoryStore>>,
    workspace: Entity<Workspace>,
    exclude_paths: &HashSet<PathBuf>,
    exclude_threads: &HashSet<acp::SessionId>,
    cx: &App,
) -> Vec<RecentEntry> {
    let mut recent = Vec::with_capacity(6);
    let workspace = workspace.read(cx);
    let project = workspace.project().read(cx);
    let include_root_name = workspace.visible_worktrees(cx).count() > 1;

    recent.extend(
        workspace
            .recent_navigation_history_iter(cx)
            .filter(|(_, abs_path)| {
                abs_path
                    .as_ref()
                    .is_none_or(|path| !exclude_paths.contains(path.as_path()))
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
                        RecentEntry::File {
                            project_path,
                            path_prefix,
                        }
                    })
            }),
    );

    if let Some(thread_store) = thread_store.and_then(|store| store.upgrade()) {
        const RECENT_THREADS_COUNT: usize = 2;
        recent.extend(
            thread_store
                .read(cx)
                .recently_opened_entries(cx)
                .iter()
                .filter(|e| match e.id() {
                    HistoryEntryId::AcpThread(session_id) => !exclude_threads.contains(&session_id),
                    HistoryEntryId::TextThread(path) => {
                        !exclude_paths.contains(&path.to_path_buf())
                    }
                })
                .take(RECENT_THREADS_COUNT)
                .map(|thread| RecentEntry::Thread(thread.clone())),
        );
    }

    recent
}

pub(crate) fn selection_ranges(
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

pub struct MentionLink;

impl MentionLink {
    const FILE: &str = "@file";
    const SYMBOL: &str = "@symbol";
    const SELECTION: &str = "@selection";
    const THREAD: &str = "@thread";
    const FETCH: &str = "@fetch";
    const RULE: &str = "@rule";

    const TEXT_THREAD_URL_PREFIX: &str = "text-thread://";

    pub fn for_file(file_name: &str, full_path: &str) -> String {
        format!("[@{}]({}:{})", file_name, Self::FILE, full_path)
    }

    pub fn for_symbol(symbol_name: &str, full_path: &str) -> String {
        format!(
            "[@{}]({}:{}:{})",
            symbol_name,
            Self::SYMBOL,
            full_path,
            symbol_name
        )
    }

    pub fn for_selection(file_name: &str, full_path: &str, line_range: Range<usize>) -> String {
        format!(
            "[@{} ({}-{})]({}:{}:{}-{})",
            file_name,
            line_range.start + 1,
            line_range.end + 1,
            Self::SELECTION,
            full_path,
            line_range.start,
            line_range.end
        )
    }

    pub fn for_thread(thread: &HistoryEntry) -> String {
        match thread {
            HistoryEntry::AcpThread(thread) => {
                format!("[@{}]({}:{})", thread.title, Self::THREAD, thread.id)
            }
            HistoryEntry::TextThread(thread) => {
                let filename = thread
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                let escaped_filename = urlencoding::encode(&filename);
                format!(
                    "[@{}]({}:{}{})",
                    thread.title,
                    Self::THREAD,
                    Self::TEXT_THREAD_URL_PREFIX,
                    escaped_filename
                )
            }
        }
    }

    pub fn for_fetch(url: &str) -> String {
        format!("[@{}]({}:{})", url, Self::FETCH, url)
    }

    pub fn for_rule(rule: &RulesContextEntry) -> String {
        format!("[@{}]({}:{})", rule.title, Self::RULE, rule.prompt_id.0)
    }
}
