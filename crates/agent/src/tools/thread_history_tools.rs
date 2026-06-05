//! Tools that let the agent search and read the user's saved thread history.
//!
//! All three tools are gated behind `ThreadHistoryToolsFeatureFlag` (see
//! `Thread::enabled_tools`). Results include `zed:///agent/thread/...` links
//! which the agent panel renders as clickable links that open the thread.

use crate::db::ThreadsDatabase;
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use acp_thread::MentionUri;
use agent_client_protocol::schema as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, AppContext as _, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use regex::RegexBuilder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::sync::Arc;

const THREADS_PER_PAGE: usize = 50;
const LINES_PER_PAGE: usize = 500;
const CONTEXT_LINES: usize = 2;
const MAX_MATCHING_THREADS: usize = 10;
const MAX_SNIPPETS_PER_THREAD: usize = 3;
const MAX_LINE_LENGTH: usize = 300;

fn thread_link(title: &str, id: &acp::SessionId) -> String {
    let uri = MentionUri::Thread {
        id: id.clone(),
        name: title.to_string(),
    };
    format!("[{}]({})", title, uri.to_uri())
}

fn truncate_line(line: &str) -> &str {
    if line.len() > MAX_LINE_LENGTH {
        let mut end = MAX_LINE_LENGTH;
        while !line.is_char_boundary(end) {
            end -= 1;
        }
        &line[..end]
    } else {
        line
    }
}

/// List the user's saved agent threads (conversations), newest first.
///
/// Each result includes the thread's title, a clickable link, and when it was
/// last updated. When telling the user about a thread, always render it as the
/// provided markdown link so they can click it to open the thread in the agent
/// panel.
///
/// Prefer `search_threads` when looking for threads about a specific topic;
/// use this tool to browse recent activity or find a thread by title.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListThreadsToolInput {
    /// Optional case-insensitive substring to filter thread titles.
    #[serde(default)]
    pub title_filter: Option<String>,
    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThreadHistoryToolOutput {
    Success { content: String },
    Error { error: String },
}

impl ThreadHistoryToolOutput {
    fn error(error: impl ToString) -> Self {
        Self::Error {
            error: error.to_string(),
        }
    }
}

impl From<ThreadHistoryToolOutput> for LanguageModelToolResultContent {
    fn from(output: ThreadHistoryToolOutput) -> Self {
        match output {
            ThreadHistoryToolOutput::Success { content } => content.into(),
            ThreadHistoryToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct ListThreadsTool {
    current_session_id: acp::SessionId,
}

impl ListThreadsTool {
    pub fn new(current_session_id: acp::SessionId) -> Self {
        Self { current_session_id }
    }
}

impl AgentTool for ListThreadsTool {
    type Input = ListThreadsToolInput;
    type Output = ThreadHistoryToolOutput;

    const NAME: &'static str = "list_threads";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "List threads".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let database_future = ThreadsDatabase::connect(cx);
        let current_session_id = self.current_session_id.clone();
        cx.background_spawn(async move {
            let input = input.recv().await.map_err(ThreadHistoryToolOutput::error)?;
            let database = database_future
                .await
                .map_err(ThreadHistoryToolOutput::error)?;
            let threads = database
                .list_threads()
                .await
                .map_err(ThreadHistoryToolOutput::error)?;

            let title_filter = input
                .title_filter
                .as_deref()
                .map(|filter| filter.to_lowercase());
            let matches = threads
                .into_iter()
                .filter(|thread| thread.id != current_session_id)
                .filter(|thread| {
                    title_filter
                        .as_deref()
                        .is_none_or(|filter| thread.title.to_lowercase().contains(filter))
                })
                .collect::<Vec<_>>();

            if matches.is_empty() {
                return Ok(ThreadHistoryToolOutput::Success {
                    content: "No threads found.".to_string(),
                });
            }

            let total = matches.len();
            let page = matches
                .iter()
                .skip(input.offset)
                .take(THREADS_PER_PAGE)
                .collect::<Vec<_>>();

            let mut content = format!("Found {} threads.", total);
            if total > THREADS_PER_PAGE {
                writeln!(
                    &mut content,
                    "\nShowing results {}-{} (provide 'offset' parameter for more results):",
                    input.offset + 1,
                    input.offset + page.len()
                )
                .ok();
            } else {
                content.push('\n');
            }
            for thread in page {
                write!(
                    &mut content,
                    "\n- {} — updated {}",
                    thread_link(&thread.title, &thread.id),
                    thread.updated_at.format("%Y-%m-%d")
                )
                .ok();
                if thread.parent_session_id.is_some() {
                    content.push_str(" (subagent thread)");
                }
                if !thread.folder_paths.is_empty() {
                    let folders = thread
                        .folder_paths
                        .paths()
                        .iter()
                        .map(|path| path.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(&mut content, " — in {}", folders).ok();
                }
            }

            Ok(ThreadHistoryToolOutput::Success { content })
        })
    }
}

/// Search the contents of the user's saved agent threads (conversations) with
/// a regular expression.
///
/// - Searches the full message text of every saved thread, newest first.
/// - Returns matching snippets with surrounding context, grouped by thread.
/// - Each result includes a clickable thread link. When telling the user about
///   a thread, always render it as the provided markdown link so they can
///   click it to open the thread in the agent panel.
/// - Use `read_thread` to read more of a matching thread.
///
/// Use this when the user asks about past conversations, e.g. "where did we
/// discuss X?" or "what did I decide about Y last week?".
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchThreadsToolInput {
    /// A regex pattern to search for in thread contents. Parsed by the Rust
    /// `regex` crate; matched against individual lines of each thread's
    /// message text.
    pub regex: String,
    /// Whether the regex is case-sensitive. Defaults to false
    /// (case-insensitive).
    #[serde(default)]
    pub case_sensitive: bool,
}

pub struct SearchThreadsTool {
    current_session_id: acp::SessionId,
}

impl SearchThreadsTool {
    pub fn new(current_session_id: acp::SessionId) -> Self {
        Self { current_session_id }
    }
}

impl AgentTool for SearchThreadsTool {
    type Input = SearchThreadsToolInput;
    type Output = ThreadHistoryToolOutput;

    const NAME: &'static str = "search_threads";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        let mut title = "Search threads".to_string();
        if let Ok(input) = input {
            title.push_str(&format!(" for “`{}`”", input.regex));
        }
        title.into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let database_future = ThreadsDatabase::connect(cx);
        let current_session_id = self.current_session_id.clone();
        let search_task = cx.background_spawn(async move {
            let input = input.recv().await.map_err(ThreadHistoryToolOutput::error)?;
            let regex = RegexBuilder::new(&input.regex)
                .case_insensitive(!input.case_sensitive)
                .build()
                .map_err(|error| {
                    ThreadHistoryToolOutput::error(format!("invalid regex: {error}"))
                })?;
            let database = database_future
                .await
                .map_err(ThreadHistoryToolOutput::error)?;
            let threads = database
                .list_threads()
                .await
                .map_err(ThreadHistoryToolOutput::error)?;

            let mut content = String::new();
            let mut matching_threads = 0;
            let mut threads_skipped = false;
            for metadata in threads {
                if metadata.id == current_session_id {
                    continue;
                }
                let Some(thread) = database
                    .load_thread(metadata.id.clone())
                    .await
                    .map_err(ThreadHistoryToolOutput::error)?
                else {
                    continue;
                };
                let markdown = thread.to_markdown();
                let (snippets, snippets_truncated) =
                    extract_snippets(&markdown, &regex, MAX_SNIPPETS_PER_THREAD);
                if snippets.is_empty() {
                    continue;
                }

                if matching_threads == MAX_MATCHING_THREADS {
                    threads_skipped = true;
                    break;
                }
                matching_threads += 1;

                writeln!(
                    &mut content,
                    "\n## {} — updated {}\n",
                    thread_link(&metadata.title, &metadata.id),
                    metadata.updated_at.format("%Y-%m-%d")
                )
                .ok();
                for snippet in snippets {
                    writeln!(
                        &mut content,
                        "Lines {}-{}:",
                        snippet.start_line, snippet.end_line
                    )
                    .ok();
                    for (line_number, line) in snippet.lines {
                        writeln!(&mut content, "  L{}: {}", line_number, truncate_line(&line)).ok();
                    }
                    content.push('\n');
                }
                if snippets_truncated {
                    writeln!(
                        &mut content,
                        "(more matches in this thread; use `read_thread` to see them)"
                    )
                    .ok();
                }
            }

            if matching_threads == 0 {
                return Ok(ThreadHistoryToolOutput::Success {
                    content: "No threads matched the search.".to_string(),
                });
            }

            let mut header = format!("Found matches in {} threads.", matching_threads);
            if threads_skipped {
                header.push_str(
                    " Older matching threads were omitted; refine the search to see them.",
                );
            }
            header.push('\n');
            header.push_str(&content);
            Ok(ThreadHistoryToolOutput::Success { content: header })
        });

        cx.spawn(async move |_cx| {
            futures::select! {
                result = search_task.fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    Err(ThreadHistoryToolOutput::error("Thread search cancelled by user"))
                }
            }
        })
    }
}

struct Snippet {
    start_line: usize,
    end_line: usize,
    lines: Vec<(usize, String)>,
}

/// Extracts up to `max_snippets` snippets of matching lines (merging
/// overlapping context ranges), and returns whether any matches were omitted.
fn extract_snippets(text: &str, regex: &regex::Regex, max_snippets: usize) -> (Vec<Snippet>, bool) {
    let lines = text.lines().collect::<Vec<_>>();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut truncated = false;
    for (ix, line) in lines.iter().enumerate() {
        if !regex.is_match(line) {
            continue;
        }
        let start = ix.saturating_sub(CONTEXT_LINES);
        let end = (ix + CONTEXT_LINES).min(lines.len().saturating_sub(1));
        match ranges.last_mut() {
            Some(last) if start <= last.1 + 1 => last.1 = end,
            _ => {
                if ranges.len() == max_snippets {
                    truncated = true;
                    break;
                }
                ranges.push((start, end));
            }
        }
    }

    let snippets = ranges
        .into_iter()
        .map(|(start, end)| Snippet {
            start_line: start + 1,
            end_line: end + 1,
            lines: lines[start..=end]
                .iter()
                .enumerate()
                .map(|(offset, line)| (start + offset + 1, line.to_string()))
                .collect(),
        })
        .collect();
    (snippets, truncated)
}

/// Read the contents of a saved agent thread (conversation) as markdown.
///
/// - `thread_id` comes from `list_threads` or `search_threads` results (the
///   final path segment of the thread's `zed:///agent/thread/...` link).
/// - Output is paginated; use `offset` to read subsequent pages.
/// - When telling the user about the thread, render it as a markdown link
///   using its `zed:///agent/thread/...` URI so they can click it to open the
///   thread in the agent panel.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadThreadToolInput {
    /// The ID of the thread to read.
    pub thread_id: String,
    /// Optional 0-based line offset to start reading from. When not provided,
    /// starts from the beginning.
    #[serde(default)]
    pub offset: usize,
}

pub struct ReadThreadTool {
    current_session_id: acp::SessionId,
}

impl ReadThreadTool {
    pub fn new(current_session_id: acp::SessionId) -> Self {
        Self { current_session_id }
    }
}

impl AgentTool for ReadThreadTool {
    type Input = ReadThreadToolInput;
    type Output = ThreadHistoryToolOutput;

    const NAME: &'static str = "read_thread";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Read thread".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let database_future = ThreadsDatabase::connect(cx);
        let current_session_id = self.current_session_id.clone();
        cx.background_spawn(async move {
            let input = input.recv().await.map_err(ThreadHistoryToolOutput::error)?;
            let session_id = acp::SessionId::new(Arc::<str>::from(input.thread_id.as_str()));
            if session_id == current_session_id {
                return Err(ThreadHistoryToolOutput::error(
                    "This is the current thread; its contents are already in context.",
                ));
            }
            let database = database_future
                .await
                .map_err(ThreadHistoryToolOutput::error)?;
            let thread = database
                .load_thread(session_id.clone())
                .await
                .map_err(ThreadHistoryToolOutput::error)?
                .ok_or_else(|| {
                    ThreadHistoryToolOutput::error(format!(
                        "No thread found with ID: {}",
                        input.thread_id
                    ))
                })?;

            let markdown = thread.to_markdown();
            let lines = markdown.lines().collect::<Vec<_>>();
            let total_lines = lines.len();
            let page = lines
                .iter()
                .skip(input.offset)
                .take(LINES_PER_PAGE)
                .copied()
                .collect::<Vec<_>>()
                .join("\n");

            let mut content = format!("# {}\n\n", thread_link(&thread.title, &session_id));
            if total_lines > LINES_PER_PAGE {
                writeln!(
                    &mut content,
                    "Showing lines {}-{} of {} (provide 'offset' parameter for more):\n",
                    input.offset + 1,
                    (input.offset + LINES_PER_PAGE).min(total_lines),
                    total_lines
                )
                .ok();
            }
            content.push_str(&page);
            Ok(ThreadHistoryToolOutput::Success { content })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regex(pattern: &str) -> regex::Regex {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .unwrap()
    }

    #[test]
    fn test_extract_snippets_basic() {
        let text = "line 1\nline 2\nneedle here\nline 4\nline 5\nline 6\nline 7";
        let (snippets, truncated) = extract_snippets(text, &regex("needle"), 3);
        assert!(!truncated);
        assert_eq!(snippets.len(), 1);
        assert_eq!(snippets[0].start_line, 1);
        assert_eq!(snippets[0].end_line, 5);
        assert_eq!(snippets[0].lines.len(), 5);
        assert_eq!(snippets[0].lines[2], (3, "needle here".to_string()));
    }

    #[test]
    fn test_extract_snippets_merges_overlapping_ranges() {
        let text = "needle\nfiller\nneedle\nfiller\nfiller\nfiller\nfiller\nfiller\nneedle";
        let (snippets, truncated) = extract_snippets(text, &regex("needle"), 3);
        assert!(!truncated);
        assert_eq!(snippets.len(), 2);
        assert_eq!((snippets[0].start_line, snippets[0].end_line), (1, 5));
        assert_eq!((snippets[1].start_line, snippets[1].end_line), (7, 9));
    }

    #[test]
    fn test_extract_snippets_truncates() {
        let mut text = String::new();
        for _ in 0..5 {
            text.push_str("needle\nx\nx\nx\nx\nx\nx\n");
        }
        let (snippets, truncated) = extract_snippets(&text, &regex("needle"), 2);
        assert!(truncated);
        assert_eq!(snippets.len(), 2);
    }

    #[test]
    fn test_extract_snippets_no_matches() {
        let (snippets, truncated) = extract_snippets("a\nb\nc", &regex("needle"), 3);
        assert!(!truncated);
        assert!(snippets.is_empty());
    }

    #[test]
    fn test_truncate_line_respects_char_boundaries() {
        let line = "é".repeat(MAX_LINE_LENGTH);
        let truncated = truncate_line(&line);
        assert!(truncated.len() <= MAX_LINE_LENGTH);
        assert!(truncated.chars().all(|c| c == 'é'));
    }
}
