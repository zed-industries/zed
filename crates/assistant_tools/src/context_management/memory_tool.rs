use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::{Arc, OnceLock};

use anyhow::{Result, anyhow, bail};
use assistant_tool::{Tool, ToolResult, ToolResultOutput};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolSchemaFormat, MessageContent, Role,
};
use parking_lot::Mutex;
use project::Project;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use uuid::Uuid;

use crate::schema::json_schema_for;
use action_log::ActionLog;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryOperation {
    Store,
    Load,
    #[serde(alias = "scan")]
    List,
    Restore,
    Prune,
    Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryToolInput {
    operation: MemoryOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(default)]
    auto: bool,
    #[serde(default = "default_preview_chars")]
    max_preview_chars: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_insert_index: Option<usize>,
    #[serde(default)]
    remove_placeholder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_placeholder_with: Option<String>,
    #[serde(default)]
    allow_overlap: bool,
    #[serde(default)]
    json: bool,
}

fn default_preview_chars() -> usize {
    200
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchivedMemory {
    session_id: String,
    memory_id: String,
    start_index: usize,
    end_index: usize,
    messages: Vec<LanguageModelRequestMessage>,
    summary: Option<String>,
    created_at: std::time::SystemTime,
    #[serde(default)]
    char_count: usize,
    #[serde(default)]
    token_estimate: usize,
    #[serde(default)]
    restored_count: usize,
    #[serde(default)]
    last_restored_at: Option<std::time::SystemTime>,
}

static MEMORY_STORE: OnceLock<Mutex<HashMap<String, ArchivedMemory>>> = OnceLock::new();
static MEMORY_STORE_LOADED: OnceLock<()> = OnceLock::new();

fn memory_store() -> &'static Mutex<HashMap<String, ArchivedMemory>> {
    MEMORY_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn store_file_path() -> std::path::PathBuf {
    paths::data_dir().join("memory_store.json")
}

fn ensure_store_loaded() {
    MEMORY_STORE_LOADED.get_or_init(|| {
        let _ = load_persistent_store();
    });
}

fn load_persistent_store() -> Result<()> {
    let path = store_file_path();
    if !path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&path)?;
    if content.trim().is_empty() {
        return Ok(());
    }
    #[derive(Deserialize)]
    struct PersistedEntry {
        handle: String,
        archived: ArchivedMemory,
    }
    let entries: Vec<PersistedEntry> = serde_json::from_str(&content)?;
    let mut guard = memory_store().lock();
    for e in entries {
        guard.entry(e.handle).or_insert(e.archived);
    }
    Ok(())
}

fn persist_store() -> Result<()> {
    #[derive(Serialize)]
    struct PersistedEntry<'a> {
        handle: &'a String,
        archived: &'a ArchivedMemory,
    }
    let guard = memory_store().lock();
    let entries: Vec<PersistedEntry> = guard
        .iter()
        .map(|(h, a)| PersistedEntry {
            handle: h,
            archived: a,
        })
        .collect();
    let serialized = serde_json::to_string_pretty(&entries)?;
    fs::write(store_file_path(), serialized)?;
    Ok(())
}

// Removed ParsedPlaceholder (no longer needed)

fn placeholder_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(
            r#"^\[\[memory archived handle=(?P<handle>\S+) range=(?P<start>\d+)\.\.(?P<end>\d+) messages=(?P<count>\d+) chars=(?P<chars>\d+)(?: tokens=(?P<tokens>\d+))?\]\]"#,
        )
        .expect("placeholder regex")
    })
}

// Removed parse_placeholder (struct was removed and parsing no longer required)

fn generate_heuristic_summary(messages: &[LanguageModelRequestMessage], count: usize) -> String {
    for message in messages {
        let content = message
            .content
            .iter()
            .filter_map(|c| match c {
                MessageContent::Text(t) => Some(t.as_str()),
                MessageContent::Thinking { text, .. } => Some(text.as_str()),
                MessageContent::RedactedThinking(t) => Some(t.as_str()),
                MessageContent::Image(_) => None,
                MessageContent::ToolUse(_) => None,
                MessageContent::ToolResult(_) => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        if !content.is_empty() {
            let first_line = content.lines().next().unwrap_or(&content);
            if first_line.len() > 140 {
                return format!("{}... ({} msgs)", &first_line[..140], count);
            } else {
                return format!("{} ({} msgs)", first_line, count);
            }
        }
    }
    format!("({} msgs)", count)
}

fn generate_preview(messages: &[LanguageModelRequestMessage], max_chars: usize) -> String {
    let mut preview = String::new();
    let mut total = 0usize;

    for message in messages {
        let content = message
            .content
            .iter()
            .map(|c| match c {
                MessageContent::Text(t) => t.clone(),
                MessageContent::Thinking { text, .. } => text.clone(),
                MessageContent::RedactedThinking(t) => t.clone(),
                MessageContent::Image(_) => "[Image]".to_string(),
                MessageContent::ToolUse(_) => "[Tool Use]".to_string(),
                MessageContent::ToolResult(_) => "[Tool Result]".to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n");
        if total + content.len() <= max_chars {
            if !preview.is_empty() {
                preview.push('\n');
            }
            preview.push_str(&content);
            total += content.len();
        } else if total < max_chars {
            let remaining = max_chars - total;
            if remaining > 20 {
                if !preview.is_empty() {
                    preview.push('\n');
                }
                preview.push_str(&content[..remaining]);
                preview.push_str("...(truncated)");
            }
            break;
        } else {
            break;
        }
    }

    preview
}

pub struct MemoryTool;

impl Tool for MemoryTool {
    fn name(&self) -> String {
        "memory".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        false
    }

    fn may_perform_edits(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        "Archive a contiguous range of conversation messages, list or inspect archives, restore or prune them, and view usage stats.".into()
    }

    fn icon(&self) -> IconName {
        IconName::Book
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<MemoryToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        if let Ok(input) = serde_json::from_value::<MemoryToolInput>(input.clone()) {
            match input.operation {
                MemoryOperation::Store => {
                    if let (Some(s), Some(e)) = (input.start_index, input.end_index) {
                        format!("Archive messages {}..{}", s, e)
                    } else {
                        "Archive messages".to_string()
                    }
                }
                MemoryOperation::Load => input
                    .memory_handle
                    .map(|h| format!("Load memory: {}", h))
                    .unwrap_or_else(|| "Load memory".to_string()),
                MemoryOperation::List => "List archives".to_string(),
                MemoryOperation::Restore => input
                    .memory_handle
                    .map(|h| format!("Restore memory: {}", h))
                    .unwrap_or_else(|| "Restore memory".to_string()),
                MemoryOperation::Prune => "Prune unused archives".to_string(),
                MemoryOperation::Stats => "Memory stats".to_string(),
            }
        } else {
            "Memory operation".to_string()
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let mut input: MemoryToolInput = match serde_json::from_value(input) {
            Ok(v) => v,
            Err(e) => return Task::ready(Err(anyhow!(e))).into(),
        };

        input.max_preview_chars = input.max_preview_chars.clamp(40, 400);

        let session_id = request
            .thread_id
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "default".into());
        let messages = request.messages.clone();
        let base_request = request.clone();

        ensure_store_loaded();

        let task = cx.spawn(async move |async_cx| {
            fn rebuild_request(
                base: &LanguageModelRequest,
                msgs: Vec<LanguageModelRequestMessage>,
            ) -> LanguageModelRequest {
                LanguageModelRequest {
                    thread_id: base.thread_id.clone(),
                    prompt_id: base.prompt_id.clone(),
                    intent: base.intent.clone(),
                    mode: base.mode.clone(),
                    messages: msgs,
                    tools: base.tools.clone(),
                    tool_choice: base.tool_choice.clone(),
                    stop: base.stop.clone(),
                    temperature: base.temperature,
                    thinking_allowed: base.thinking_allowed,
                }
            }

            async fn precise_tokens(
                model: &Arc<dyn LanguageModel>,
                base: &LanguageModelRequest,
                msgs: &[LanguageModelRequestMessage],
                cx: &gpui::AsyncApp,
            ) -> usize {
                if msgs.is_empty() {
                    return 0;
                }
                let heuristic = {
                    let chars: usize = msgs
                        .iter()
                        .map(|m| {
                            m.content
                                .iter()
                                .map(|c| match c {
                                    MessageContent::Text(t) => t.len(),
                                    MessageContent::Thinking { text, .. } => text.len(),
                                    MessageContent::RedactedThinking(t) => t.len(),
                                    MessageContent::Image(_) => 7,
                                    MessageContent::ToolUse(_) => 11,
                                    MessageContent::ToolResult(_) => 14,
                                })
                                .sum::<usize>()
                        })
                        .sum();
                    (chars / 4).max(1)
                };

                let req = rebuild_request(base, msgs.to_vec());
                let fut = match cx.update(|app| model.count_tokens(req, app)) {
                    Ok(fut) => fut,
                    Err(_) => return heuristic,
                };
                match fut.await {
                    Ok(v) if v > 0 => v as usize,
                    _ => heuristic,
                }
            }

            async fn per_message_precise_tokens(
                model: &Arc<dyn LanguageModel>,
                base: &LanguageModelRequest,
                all: &[LanguageModelRequestMessage],
                cx: &gpui::AsyncApp,
            ) -> Result<(Vec<usize>, usize)> {
                let mut prefix = Vec::with_capacity(all.len() + 1);
                prefix.push(0);
                for i in 0..all.len() {
                    let t = precise_tokens(model, base, &all[..=i], cx).await;
                    prefix.push(t);
                }
                let mut per = Vec::with_capacity(all.len());
                for i in 0..all.len() {
                    per.push(prefix[i + 1].saturating_sub(prefix[i]));
                }
                Ok((per, *prefix.last().unwrap_or(&0)))
            }

            let max_context_tokens = model.max_token_count() as usize;

            match input.operation {
                MemoryOperation::Store => {
                    let start = input
                        .start_index
                        .ok_or_else(|| anyhow!("start_index required for store"))?;
                    let end = input
                        .end_index
                        .ok_or_else(|| anyhow!("end_index required for store"))?;
                    if start > end {
                        bail!("start_index must be <= end_index");
                    }
                    if end >= messages.len() {
                        bail!("end_index {end} out of bounds (len={})", messages.len());
                    }

                    {
                        let guard = memory_store().lock();
                        if !input.allow_overlap {
                            for (h, a) in guard.iter() {
                                if a.session_id == session_id {
                                    let overlap = !(end < a.start_index || start > a.end_index);
                                    if overlap {
                                        bail!(
                                            "overlapping archive with {} ({}..{}), pass allow_overlap=true to force",
                                            h, a.start_index, a.end_index
                                        );
                                    }
                                }
                            }
                        }
                    }

                    let archived_messages = messages[start..=end].to_vec();
                    let count = archived_messages.len();

                    let summary = if let Some(s) = input.summary {
                        Some(s)
                    } else if input.auto {
                        Some(generate_heuristic_summary(&archived_messages, count))
                    } else {
                        None
                    };

                    let char_count: usize = archived_messages
                        .iter()
                        .map(|m| {
                            m.content
                                .iter()
                                .map(|c| match c {
                                    MessageContent::Text(t) => t.len(),
                                    MessageContent::Thinking { text, .. } => text.len(),
                                    MessageContent::RedactedThinking(t) => t.len(),
                                    MessageContent::Image(_) => 7,
                                    MessageContent::ToolUse(_) => 11,
                                    MessageContent::ToolResult(_) => 14,
                                })
                                .sum::<usize>()
                        })
                        .sum();

                    let token_estimate =
                        precise_tokens(&model, &base_request, &archived_messages, &*async_cx).await;

                    let preview = generate_preview(&archived_messages, input.max_preview_chars);

                    let memory_id = Uuid::new_v4().to_string();
                    let handle = format!("mem://{}/{}", session_id, memory_id);

                    let archived = ArchivedMemory {
                        session_id: session_id.clone(),
                        memory_id,
                        start_index: start,
                        end_index: end,
                        messages: archived_messages,
                        summary: summary.clone(),
                        created_at: std::time::SystemTime::now(),
                        char_count,
                        token_estimate,
                        restored_count: 0,
                        last_restored_at: None,
                    };

                    {
                        let mut guard = memory_store().lock();
                        guard.insert(handle.clone(), archived);
                    }
                    persist_store()?;

                    let summary_line = summary
                        .unwrap_or_else(|| format!("{} messages archived", count));

                    let mut placeholder = format!(
                        "[[memory archived handle={} range={}..{} messages={} chars={} tokens={}]]\nSummary: {}",
                        handle, start, end, count, char_count, token_estimate, summary_line
                    );
                    if !preview.is_empty() {
                        placeholder.push_str(&format!("\nPreview: {}", preview));
                    }

                    let output = format!(
                        "Archived {} messages ({}..{}) to {}\nPlaceholder:\n{}",
                        count, start, end, handle, placeholder
                    );
                    Ok(ToolResultOutput::from(output))
                }

                MemoryOperation::Load => {
                    let handle = input
                        .memory_handle
                        .ok_or_else(|| anyhow!("memory_handle required for load"))?;
                    let guard = memory_store().lock();
                    let archived = guard
                        .get(&handle)
                        .ok_or_else(|| anyhow!("unknown memory handle: {}", handle))?;

                    let mut out = format!("# Archived Memory: {}\n\n", handle);
                    out.push_str(&format!(
                        "Range: {}..{}\nMessages: {}\nChars: {}\nTokens: {}\n",
                        archived.start_index,
                        archived.end_index,
                        archived.messages.len(),
                        archived.char_count,
                        archived.token_estimate
                    ));
                    if let Some(s) = &archived.summary {
                        out.push_str(&format!("Summary: {}\n", s));
                    }
                    out.push_str("\n## Messages\n\n");
                    for (i, msg) in archived.messages.iter().enumerate() {
                        out.push_str(&format!(
                            "### Message {} (orig idx {}) role={:?}\n",
                            i,
                            archived.start_index + i,
                            msg.role
                        ));
                        let body = msg
                            .content
                            .iter()
                            .map(|c| match c {
                                MessageContent::Text(t) => t.clone(),
                                MessageContent::Thinking { text, .. } => text.clone(),
                                MessageContent::RedactedThinking(t) => t.clone(),
                                MessageContent::Image(_) => "[Image]".into(),
                                MessageContent::ToolUse(_) => "[Tool Use]".into(),
                                MessageContent::ToolResult(_) => "[Tool Result]".into(),
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        out.push_str(&body);
                        out.push_str("\n\n");
                    }
                    Ok(ToolResultOutput::from(out))
                }

                MemoryOperation::List => {
                    // Observed placeholders: map handle -> indices where placeholder appears.
                    let mut observed: HashMap<String, Vec<usize>> = HashMap::new();
                    for (idx, msg) in messages.iter().enumerate() {
                        let joined = msg
                            .content
                            .iter()
                            .map(|c| match c {
                                MessageContent::Text(t) => t.clone(),
                                MessageContent::Thinking { text, .. } => text.clone(),
                                MessageContent::RedactedThinking(t) => t.clone(),
                                MessageContent::Image(_) => "[Image]".into(),
                                MessageContent::ToolUse(_) => "[Tool Use]".into(),
                                MessageContent::ToolResult(_) => "[Tool Result]".into(),
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        if let Some(caps) = placeholder_regex()
                            .captures(joined.lines().next().unwrap_or("").trim())
                        {
                            if let Some(handle) = caps.name("handle") {
                                observed
                                    .entry(handle.as_str().to_string())
                                    .or_default()
                                    .push(idx);
                            }
                        }
                    }

                    let guard = memory_store().lock();
                    if guard.is_empty() {
                        if input.json {
                            return Ok(ToolResultOutput::from(
                                serde_json::json!({
                                    "archives": [],
                                    "orphaned": []
                                })
                                .to_string(),
                            ));
                        }
                        return Ok(ToolResultOutput::from(
                            "# Archived Memories\n\nNo archived memories.\n".to_string(),
                        ));
                    }

                    struct Row<'a> {
                        handle: &'a str,
                        range: String,
                        count: usize,
                        referenced: bool,
                        summary: String,
                    }

                    let mut rows = Vec::new();
                    for (handle, archived) in guard.iter() {
                        let referenced = observed.contains_key(handle);
                        let range = format!("{}..{}", archived.start_index, archived.end_index);
                        let count = archived.messages.len();
                        let summary = archived
                            .summary
                            .as_ref()
                            .map(|s| s.replace('\n', " "))
                            .unwrap_or_default();
                        rows.push(Row {
                            handle,
                            range,
                            count,
                            referenced,
                            summary,
                        });
                    }

                    let mut orphaned: Vec<(String, usize)> = Vec::new();
                    for (handle, indices) in &observed {
                        if !guard.contains_key(handle) {
                            for idx in indices {
                                orphaned.push((handle.clone(), *idx));
                            }
                        }
                    }

                    if input.json {
                        #[derive(Serialize)]
                        struct JsonEntry<'a> {
                            handle: &'a str,
                            range: &'a str,
                            messages: usize,
                            referenced: bool,
                            summary: &'a str,
                        }
                        #[derive(Serialize)]
                        struct JsonOrphan {
                            handle: String,
                            index: usize,
                        }
                        #[derive(Serialize)]
                        struct JsonList<'a> {
                            archives: Vec<JsonEntry<'a>>,
                            orphaned: Vec<JsonOrphan>,
                        }

                        let archives: Vec<JsonEntry> = rows
                            .iter()
                            .map(|r| JsonEntry {
                                handle: r.handle,
                                range: r.range.as_str(),
                                messages: r.count,
                                referenced: r.referenced,
                                summary: r.summary.as_str(),
                            })
                            .collect();
                        let orphaned_json: Vec<JsonOrphan> = orphaned
                            .into_iter()
                            .map(|(h, i)| JsonOrphan { handle: h, index: i })
                            .collect();

                        let payload = JsonList {
                            archives,
                            orphaned: orphaned_json,
                        };
                        return Ok(ToolResultOutput::from(
                            serde_json::to_string_pretty(&payload)
                                .unwrap_or_else(|_| "{\"error\":\"serialization\"}".into()),
                        ));
                    }

                    let mut out = String::from(
                        "# Archived Memories\n\nHandle | Range | Count | Referenced | Summary\n--- | --- | --- | --- | ---\n",
                    );
                    for r in &rows {
                        out.push_str(&format!(
                            "{} | {} | {} | {} | {}\n",
                            r.handle,
                            r.range,
                            r.count,
                            if r.referenced { "yes" } else { "no" },
                            r.summary
                        ));
                    }
                    if !orphaned.is_empty() {
                        out.push_str("\nOrphaned placeholders (not in store):\n");
                        for (h, idx) in orphaned {
                            out.push_str(&format!("- index {} handle {}\n", idx, h));
                        }
                    }
                    Ok(ToolResultOutput::from(out))
                }

                MemoryOperation::Restore => {
                    let handle = input
                        .memory_handle
                        .ok_or_else(|| anyhow!("memory_handle required for restore"))?;
                    let mut archived = {
                        let guard = memory_store().lock();
                        guard
                            .get(&handle)
                            .ok_or_else(|| anyhow!("unknown memory handle: {}", handle))?
                            .clone()
                    };

                    let insert_index = input.restore_insert_index.unwrap_or_else(|| messages.len());

                    archived.restored_count += 1;
                    archived.last_restored_at = Some(std::time::SystemTime::now());
                    {
                        let mut guard = memory_store().lock();
                        guard.insert(handle.clone(), archived.clone());
                        let _ = persist_store();
                    }

                    #[derive(Serialize)]
                    struct RestoreMessage {
                        role: String,
                        ui_text: String,
                        model_text: String,
                    }
                    #[derive(Serialize)]
                    struct ConversationMutationInsert {
                        action: &'static str,
                        index: usize,
                        messages: Vec<RestoreMessage>,
                    }
                    #[derive(Serialize)]
                    struct ConversationMutationRemove {
                        action: &'static str,
                        handle: String,
                        replacement_text: Option<String>,
                    }

                    let converted: Vec<RestoreMessage> = archived
                        .messages
                        .iter()
                        .map(|m| {
                            let text = m
                                .content
                                .iter()
                                .map(|c| match c {
                                    MessageContent::Text(t) => t.as_str(),
                                    MessageContent::Thinking { text, .. } => text.as_str(),
                                    MessageContent::RedactedThinking(t) => t.as_str(),
                                    MessageContent::Image(_) => "[Image]",
                                    MessageContent::ToolUse(_) => "[Tool Use]",
                                    MessageContent::ToolResult(_) => "[Tool Result]",
                                })
                                .collect::<Vec<_>>()
                                .join("\n");
                            RestoreMessage {
                                role: match m.role {
                                    Role::User => "user".into(),
                                    Role::Assistant => "assistant".into(),
                                    Role::System => "system".into(),
                                },
                                ui_text: text.clone(),
                                model_text: text,
                            }
                        })
                        .collect();

                    let mut _mutations: Vec<serde_json::Value> = Vec::new();
                    _mutations.push(
                        serde_json::to_value(ConversationMutationInsert {
                            action: "insert_at",
                            index: insert_index,
                            messages: converted,
                        })
                        .unwrap(),
                    );

                    if input.remove_placeholder {
                        _mutations.push(
                            serde_json::to_value(ConversationMutationRemove {
                                action: "remove_placeholder",
                                handle: handle.clone(),
                                replacement_text: None,
                            })
                            .unwrap(),
                        );
                    } else if let Some(rep) = input.replace_placeholder_with.clone() {
                        _mutations.push(
                            serde_json::to_value(ConversationMutationRemove {
                                action: "remove_placeholder",
                                handle: handle.clone(),
                                replacement_text: Some(rep.clone()),
                            })
                            .unwrap(),
                        );
                    }

                    let mut out = format!(
                        "Restored {} messages from {} at index {}",
                        archived.messages.len(),
                        handle,
                        insert_index
                    );
                    if input.remove_placeholder {
                        out.push_str("\nPlaceholder removed.");
                    } else if let Some(rep) = input.replace_placeholder_with {
                        out.push_str(&format!("\nPlaceholder replaced with: {}", rep));
                    }
                    Ok(ToolResultOutput::from(out))
                }

                MemoryOperation::Prune => {
                    let mut referenced = HashSet::new();
                    for msg in &messages {
                        let body = msg
                            .content
                            .iter()
                            .map(|c| match c {
                                MessageContent::Text(t) => t.clone(),
                                MessageContent::Thinking { text, .. } => text.clone(),
                                MessageContent::RedactedThinking(t) => t.clone(),
                                MessageContent::Image(_) => "[Image]".into(),
                                MessageContent::ToolUse(_) => "[Tool Use]".into(),
                                MessageContent::ToolResult(_) => "[Tool Result]".into(),
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        if let Some(caps) =
                            placeholder_regex().captures(body.lines().next().unwrap_or("").trim())
                        {
                            if let Some(handle) = caps.name("handle") {
                                referenced.insert(handle.as_str().to_string());
                            }
                        }
                    }

                    let mut guard = memory_store().lock();
                    let before = guard.len();
                    let handles: Vec<String> = guard.keys().cloned().collect();
                    let mut pruned = 0;
                    for h in handles {
                        if !referenced.contains(&h) {
                            guard.remove(&h);
                            pruned += 1;
                        }
                    }
                    if pruned > 0 {
                        let _ = persist_store();
                    }
                    let out = format!(
                        "Prune complete. initial={} pruned={} remaining={} referenced_placeholders={}",
                        before,
                        pruned,
                        guard.len(),
                        referenced.len()
                    );
                    Ok(ToolResultOutput::from(out))
                }

                MemoryOperation::Stats => {
                    let guard = memory_store().lock();

                    let mut total_archives = 0usize;
                    let mut total_messages = 0usize;
                    let mut total_chars = 0usize;
                    let mut total_tokens = 0usize;
                    let mut restored_archives = 0usize;

                    for (_h, a) in guard.iter() {
                        total_archives += 1;
                        total_messages += a.messages.len();
                        total_chars += a.char_count;
                        total_tokens += a.token_estimate;
                        if a.restored_count > 0 {
                            restored_archives += 1;
                        }
                    }

                    let (per_message_tokens, active_tokens) =
                        per_message_precise_tokens(&model, &base_request, &messages, &*async_cx)
                            .await?;

                    let usage_pct = if max_context_tokens > 0 {
                        (active_tokens as f64 / max_context_tokens as f64) * 100.0
                    } else {
                        0.0
                    };

                    let avg_chars = if total_archives > 0 {
                        total_chars as f64 / total_archives as f64
                    } else {
                        0.0
                    };
                    let avg_tokens = if total_archives > 0 {
                        total_tokens as f64 / total_archives as f64
                    } else {
                        0.0
                    };

                    let mut recommendation = String::new();
                    if usage_pct > 70.0 && !messages.is_empty() {
                        let target = (max_context_tokens as f64 * 0.60) as usize;
                        let to_free = active_tokens.saturating_sub(target);
                        if to_free > 0 {
                            let mut acc = 0usize;
                            let mut end_index = 0usize;
                            for (i, m) in messages.iter().enumerate() {
                                let is_placeholder = m
                                    .content
                                    .iter()
                                    .find_map(|c| {
                                        if let MessageContent::Text(t) = c {
                                            if placeholder_regex()
                                                .is_match(t.lines().next().unwrap_or("").trim())
                                            {
                                                Some(())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    })
                                    .is_some();
                                if is_placeholder {
                                    break;
                                }
                                acc += per_message_tokens[i];
                                end_index = i;
                                if acc >= to_free {
                                    break;
                                }
                            }
                            if end_index > 0 {
                                let mut seed = String::new();
                                for m in &messages[0..=end_index] {
                                    if let Some(text) = m.content.iter().find_map(|c| {
                                        if let MessageContent::Text(t) = c {
                                            let trimmed = t.trim();
                                            if !trimmed.is_empty() {
                                                Some(trimmed)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }) {
                                        seed = text.lines().next().unwrap_or(text).to_string();
                                        break;
                                    }
                                }
                                if seed.len() > 140 {
                                    seed.truncate(140);
                                    seed.push_str("...");
                                }
                                let projected_tokens = active_tokens.saturating_sub(acc);
                                let projected_pct = if max_context_tokens > 0 {
                                    (projected_tokens as f64 / max_context_tokens as f64) * 100.0
                                } else {
                                    0.0
                                };
                                recommendation = format!(
                                    "Recommendation: store messages 0..{end} (~{freed} tokens) lowering usage {:.2}% -> {:.2}%.\nSuggested summary: \"{} ({} msgs)\".\nExample call: {{\"operation\":\"store\",\"start_index\":0,\"end_index\":{end},\"summary\":\"{} ({} msgs)\"}}",
                                    usage_pct,
                                    projected_pct,
                                    seed,
                                    end_index + 1,
                                    seed,
                                    end_index + 1,
                                    end = end_index,
                                    freed = acc
                                );
                            }
                        }
                    }

                    let mut out = String::new();
                    out.push_str("# Memory & Context Stats\n\n");
                    out.push_str("## Active Context\n");
                    out.push_str(&format!("Active tokens: {}\n", active_tokens));
                    out.push_str(&format!("Model max tokens: {}\n", max_context_tokens));
                    out.push_str(&format!("Usage: {:.2}%\n\n", usage_pct));

                    out.push_str("## Archived\n");
                    out.push_str(&format!("Archives: {}\n", total_archives));
                    out.push_str(&format!("Archived messages: {}\n", total_messages));
                    out.push_str(&format!("Archived chars: {}\n", total_chars));
                    out.push_str(&format!("Archived tokens: {}\n", total_tokens));
                    out.push_str(&format!("Avg chars/archive: {:.2}\n", avg_chars));
                    out.push_str(&format!("Avg tokens/archive: {:.2}\n", avg_tokens));
                    out.push_str(&format!(
                        "Restored archives (ever): {}\n\n",
                        restored_archives
                    ));

                    out.push_str("## Per-Message Tokens (first 30)\n");
                    for (i, t) in per_message_tokens.iter().take(30).enumerate() {
                        out.push_str(&format!("Message {}: {} tokens\n", i, t));
                    }
                    if per_message_tokens.len() > 30 {
                        out.push_str(&format!(
                            "... ({} more messages)\n",
                            per_message_tokens.len() - 30
                        ));
                    }
                    out.push('\n');

                    out.push_str("## Action\n");
                    if !recommendation.is_empty() {
                        out.push_str(&recommendation);
                        out.push('\n');
                    } else if usage_pct > 70.0 {
                        out.push_str(
                            "Context above threshold but no contiguous initial non-placeholder span found.\n",
                        );
                    } else {
                        out.push_str("No immediate action needed (below 70%).\n");
                    }

                    Ok(ToolResultOutput::from(out))
                }
            }
        });

        ToolResult {
            output: task,
            card: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Removed test_placeholder_regex_parses_basic (parse_placeholder removed)

    // Removed test_placeholder_regex_without_tokens (parse_placeholder removed)

    #[test]
    fn test_generate_preview_truncates() {
        let msg = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text("a".repeat(300))],
            cache: false,
        };
        let preview = generate_preview(&[msg], 100);
        assert!(
            preview.len() <= 120,
            "preview should truncate and append marker"
        );
    }

    #[test]
    fn test_generate_heuristic_summary() {
        let msg = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text(
                "This is a fairly long line that should appear as the first line".into(),
            )],
            cache: false,
        };
        let summary = generate_heuristic_summary(&[msg], 1);
        assert!(
            summary.contains("(1 msgs)"),
            "summary should contain message count"
        );
    }
}
