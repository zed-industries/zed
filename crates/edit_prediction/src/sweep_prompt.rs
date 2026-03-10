use anyhow::{Result, anyhow};
use gpui::{App, AppContext as _, Task};
use language::{
    BufferSnapshot, Point, ToOffset as _, ToPoint as _, language_settings::all_language_settings,
};
use std::{fmt::Write as _, ops::Range, path::Path, sync::Arc, time::Instant};
use zeta_prompt::{RelatedFile, ZetaPromptInput, filter_redundant_excerpts};

use crate::{
    DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId, EditPredictionModelInput,
    EditPredictionResult, EditPredictionStartedDebugEvent, StoredEvent,
    cursor_excerpt::fixed_line_window_around_cursor,
    open_ai_compatible::{self, load_open_ai_compatible_api_key_if_needed},
    zeta,
};

const WINDOW_LINES_ABOVE: u32 = 10;
const WINDOW_LINES_BELOW: u32 = 10;
const MAX_RECENT_CHANGE_BLOCKS: usize = 3;
const MAX_RECENT_CHANGE_LINES: usize = 40;
const RESERVED_SWEEP_TOKENS: [&str; 2] = ["<|file_sep|>", "</s>"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SweepPromptInput {
    pub file_path: Arc<Path>,
    pub original_window: String,
    pub current_window: String,
    pub recent_changes: Vec<RecentChangeBlock>,
    pub related_files: Vec<RelatedFileBlock>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecentChangeBlock {
    pub file_path: Arc<Path>,
    pub original: String,
    pub updated: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelatedFileBlock {
    pub file_path: Arc<Path>,
    pub content: String,
}

pub fn request_prediction(
    input: EditPredictionModelInput,
    cx: &mut App,
) -> Task<Result<Option<EditPredictionResult>>> {
    let settings = &all_language_settings(None, cx).edit_predictions;
    let provider = settings.provider;
    let Some(custom_settings) = (match provider {
        settings::EditPredictionProvider::Ollama => settings.ollama.clone(),
        settings::EditPredictionProvider::OpenAiCompatibleApi => {
            settings.open_ai_compatible_api.clone()
        }
        _ => None,
    }) else {
        return Task::ready(Err(anyhow!(
            "Unsupported edit prediction provider for Sweep prompt mode"
        )));
    };

    let api_key = load_open_ai_compatible_api_key_if_needed(provider, cx);
    let http_client = cx.http_client();
    let buffer_snapshotted_at = Instant::now();

    let EditPredictionModelInput {
        buffer,
        snapshot,
        position,
        events,
        stored_events,
        related_files,
        debug_tx,
        ..
    } = input;

    let cursor_point = position.to_point(&snapshot);
    let window_range = fixed_line_window_around_cursor(
        &snapshot,
        cursor_point,
        WINDOW_LINES_ABOVE,
        WINDOW_LINES_BELOW,
    );
    let file_path = prompt_file_path(&snapshot);
    let filtered_related_files = filter_redundant_excerpts(
        related_files,
        file_path.as_ref(),
        window_range.start.row..window_range.end.row.saturating_add(1),
    );
    let prompt_input = build_prompt_input(
        &file_path,
        window_range.clone(),
        &snapshot,
        &stored_events,
        filtered_related_files.clone(),
    );
    if let Err(error) = validate_prompt_input(&prompt_input) {
        return Task::ready(Err(error));
    }
    let prompt = build_prompt(&prompt_input);

    if let Some(debug_tx) = &debug_tx {
        debug_tx
            .unbounded_send(DebugEvent::EditPredictionStarted(
                EditPredictionStartedDebugEvent {
                    buffer: buffer.downgrade(),
                    prompt: Some(prompt.clone()),
                    position,
                },
            ))
            .ok();
    }

    let window_start_offset = window_range.start.to_offset(&snapshot);
    let cursor_offset_in_window = position
        .to_offset(&snapshot)
        .saturating_sub(window_start_offset);
    let zeta_input = ZetaPromptInput {
        events,
        related_files: Some(filtered_related_files),
        cursor_path: file_path,
        cursor_excerpt: prompt_input.current_window.clone().into(),
        cursor_offset_in_excerpt: cursor_offset_in_window,
        excerpt_start_row: Some(window_range.start.row),
        excerpt_ranges: Default::default(),
        syntax_ranges: None,
        experiment: None,
        in_open_source_repo: false,
        can_collect_data: false,
        repo_url: None,
    };

    let current_window = prompt_input.current_window;
    let request_task: Task<Result<(String, String, Instant)>> = cx.background_spawn(async move {
        let (response_text, request_id) = open_ai_compatible::request_sweep_prompt_prediction(
            provider,
            &custom_settings,
            prompt,
            api_key,
            &http_client,
        )
        .await?;
        let response_received_at = Instant::now();
        Ok((
            request_id,
            clean_response_text(&response_text),
            response_received_at,
        ))
    });

    cx.spawn(async move |cx| {
        let (request_id, response_text, response_received_at) = request_task.await?;

        if let Some(debug_tx) = &debug_tx {
            debug_tx
                .unbounded_send(DebugEvent::EditPredictionFinished(
                    EditPredictionFinishedDebugEvent {
                        buffer: buffer.downgrade(),
                        position,
                        model_output: Some(response_text.clone()),
                    },
                ))
                .ok();
        }

        if response_text.is_empty() || response_text == current_window {
            return Ok(None);
        }

        let edits = zeta::compute_edits(
            current_window,
            &response_text,
            window_start_offset,
            &snapshot,
        );
        if edits.is_empty() {
            return Ok(None);
        }

        Ok(Some(
            EditPredictionResult::new(
                EditPredictionId(request_id.into()),
                &buffer,
                &snapshot,
                edits.into(),
                None,
                buffer_snapshotted_at,
                response_received_at,
                zeta_input,
                None,
                cx,
            )
            .await,
        ))
    })
}

pub fn build_prompt(input: &SweepPromptInput) -> String {
    let mut prompt = String::new();

    for related_file in &input.related_files {
        write_file_block(
            &mut prompt,
            related_file.file_path.as_ref(),
            &related_file.content,
        );
    }

    for recent_change in &input.recent_changes {
        let diff_path = format!("{}.diff", recent_change.file_path.display());
        let mut diff_body = String::new();
        writeln!(&mut diff_body, "original:").ok();
        diff_body.push_str(&recent_change.original);
        if !recent_change.original.ends_with('\n') {
            diff_body.push('\n');
        }
        writeln!(&mut diff_body, "updated:").ok();
        diff_body.push_str(&recent_change.updated);
        write_file_block(&mut prompt, Path::new(&diff_path), &diff_body);
    }

    let original_path = format!("original/{}", input.file_path.display());
    write_file_block(
        &mut prompt,
        Path::new(&original_path),
        &input.original_window,
    );

    let current_path = format!("current/{}", input.file_path.display());
    write_file_block(&mut prompt, Path::new(&current_path), &input.current_window);

    let updated_path = format!("updated/{}", input.file_path.display());
    writeln!(&mut prompt, "<|file_sep|>{updated_path}").ok();

    prompt
}

fn validate_prompt_input(input: &SweepPromptInput) -> Result<()> {
    validate_prompt_field("file path", &input.file_path.display().to_string())?;
    validate_prompt_field("original window", &input.original_window)?;
    validate_prompt_field("current window", &input.current_window)?;

    for recent_change in &input.recent_changes {
        validate_prompt_field(
            "recent change path",
            &recent_change.file_path.display().to_string(),
        )?;
        validate_prompt_field("recent change original", &recent_change.original)?;
        validate_prompt_field("recent change updated", &recent_change.updated)?;
    }

    for related_file in &input.related_files {
        validate_prompt_field(
            "related file path",
            &related_file.file_path.display().to_string(),
        )?;
        validate_prompt_field("related file content", &related_file.content)?;
    }

    Ok(())
}

fn validate_prompt_field(field_name: &str, value: &str) -> Result<()> {
    if RESERVED_SWEEP_TOKENS
        .iter()
        .any(|reserved_token| value.contains(reserved_token))
    {
        anyhow::bail!("sweep prompt {field_name} contains reserved tokens");
    }

    Ok(())
}

pub(crate) fn original_window_for_current_window(
    current_window: Range<Point>,
    latest_event: Option<&StoredEvent>,
    current_snapshot: &BufferSnapshot,
) -> Option<String> {
    let latest_event = latest_event?;
    if latest_event.old_snapshot.remote_id() != current_snapshot.remote_id() {
        return None;
    }

    let current_start = current_window.start.to_offset(current_snapshot);
    let current_end = current_window.end.to_offset(current_snapshot);
    let old_start =
        map_current_offset_to_old_snapshot(current_start, latest_event, current_snapshot)?;
    let old_end = map_current_offset_to_old_snapshot(current_end, latest_event, current_snapshot)?;
    let (old_start, old_end) = if old_start <= old_end {
        (old_start, old_end)
    } else {
        (old_end, old_start)
    };

    Some(
        latest_event
            .old_snapshot
            .text_for_range(old_start..old_end)
            .collect(),
    )
}

pub(crate) fn latest_active_buffer_event<'a>(
    stored_events: &'a [StoredEvent],
    snapshot: &BufferSnapshot,
) -> Option<&'a StoredEvent> {
    let current_remote_id = snapshot.remote_id();
    stored_events
        .iter()
        .rev()
        .find(|stored_event| stored_event.old_snapshot.remote_id() == current_remote_id)
}

fn build_prompt_input(
    file_path: &Arc<Path>,
    window_range: Range<Point>,
    snapshot: &BufferSnapshot,
    stored_events: &[StoredEvent],
    related_files: Vec<RelatedFile>,
) -> SweepPromptInput {
    let current_window = snapshot
        .text_for_range(window_range.clone())
        .collect::<String>();
    let original_window = original_window_for_current_window(
        window_range,
        latest_active_buffer_event(stored_events, snapshot),
        snapshot,
    )
    .unwrap_or_else(|| current_window.clone());

    SweepPromptInput {
        file_path: file_path.clone(),
        original_window,
        current_window,
        recent_changes: build_recent_change_blocks(stored_events),
        related_files: build_related_file_blocks(related_files),
    }
}

fn prompt_file_path(snapshot: &BufferSnapshot) -> Arc<Path> {
    snapshot
        .file()
        .map(|file| Arc::<Path>::from(file.path().as_std_path()))
        .unwrap_or_else(|| Path::new("untitled").into())
}

fn write_file_block(prompt: &mut String, path: &Path, content: &str) {
    writeln!(prompt, "<|file_sep|>{}", path.display()).ok();
    prompt.push_str(content);
    if !content.ends_with('\n') {
        prompt.push('\n');
    }
}

fn map_current_offset_to_old_snapshot(
    current_offset: usize,
    latest_event: &StoredEvent,
    current_snapshot: &BufferSnapshot,
) -> Option<usize> {
    if latest_event.old_snapshot.remote_id() != current_snapshot.remote_id() {
        return None;
    }

    let old_snapshot = &latest_event.old_snapshot;
    for edit in current_snapshot.edits_since::<usize>(old_snapshot.version()) {
        if current_offset < edit.new.start {
            break;
        }

        if current_offset < edit.new.end {
            let offset_in_edit = current_offset.saturating_sub(edit.new.start);
            return Some(
                (edit.old.start + offset_in_edit.min(edit.old.len())).min(old_snapshot.len()),
            );
        }
    }

    let old_offset = current_snapshot
        .edits_since::<usize>(old_snapshot.version())
        .take_while(|edit| current_offset >= edit.new.end)
        .fold(current_offset as isize, |old_offset, edit| {
            old_offset + edit.old.len() as isize - edit.new.len() as isize
        });

    Some(old_offset.max(0) as usize)
}

fn build_related_file_blocks(related_files: Vec<RelatedFile>) -> Vec<RelatedFileBlock> {
    related_files
        .into_iter()
        .map(|related_file| {
            let mut excerpts = related_file.excerpts;
            excerpts.sort_by_key(|excerpt| (excerpt.order, excerpt.row_range.start));
            RelatedFileBlock {
                file_path: related_file.path,
                content: excerpts
                    .into_iter()
                    .map(|excerpt| excerpt.text.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            }
        })
        .collect()
}

fn build_recent_change_blocks(stored_events: &[StoredEvent]) -> Vec<RecentChangeBlock> {
    stored_events
        .iter()
        .rev()
        .filter_map(recent_change_block_from_event)
        .take(MAX_RECENT_CHANGE_BLOCKS)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn recent_change_block_from_event(stored_event: &StoredEvent) -> Option<RecentChangeBlock> {
    let zeta_prompt::Event::BufferChange {
        old_path,
        path,
        diff,
        ..
    } = stored_event.event.as_ref();

    let (mut original_lines, mut updated_lines) = parse_unified_diff(diff);
    if original_lines.is_empty() && updated_lines.is_empty() {
        if old_path != path {
            original_lines.push(format!("path: {}", old_path.display()));
            updated_lines.push(format!("path: {}", path.display()));
        } else {
            return None;
        }
    }

    Some(RecentChangeBlock {
        file_path: path.clone(),
        original: limit_change_lines(original_lines, MAX_RECENT_CHANGE_LINES),
        updated: limit_change_lines(updated_lines, MAX_RECENT_CHANGE_LINES),
    })
}

fn parse_unified_diff(diff: &str) -> (Vec<String>, Vec<String>) {
    let mut original_lines = Vec::new();
    let mut updated_lines = Vec::new();

    for line in diff.lines() {
        if line.starts_with("@@") || line.starts_with("---") || line.starts_with("+++") {
            continue;
        }

        if let Some(rest) = line.strip_prefix('-') {
            original_lines.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix('+') {
            updated_lines.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix(' ') {
            let shared_line = rest.to_string();
            original_lines.push(shared_line.clone());
            updated_lines.push(shared_line);
        }
    }

    (original_lines, updated_lines)
}

fn limit_change_lines(lines: Vec<String>, max_lines: usize) -> String {
    if lines.len() <= max_lines {
        return lines.join("\n");
    }

    let head_count = max_lines / 2;
    let tail_count = max_lines.saturating_sub(head_count + 1);
    let mut trimmed = Vec::with_capacity(max_lines);
    trimmed.extend(lines.iter().take(head_count).cloned());
    trimmed.push("...".to_string());
    trimmed.extend(
        lines
            .iter()
            .rev()
            .take(tail_count)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev(),
    );
    trimmed.join("\n")
}

fn clean_response_text(response_text: &str) -> String {
    let mut cleaned = response_text.to_string();
    for stop_token in ["<|file_sep|>", "</s>"] {
        if let Some(position) = cleaned.find(stop_token) {
            cleaned.truncate(position);
        }
    }
    cleaned
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor_excerpt::fixed_line_window_around_cursor;
    use gpui::{App, AppContext};
    use language::Buffer;

    #[gpui::test]
    fn test_original_window_for_current_window_uses_latest_pre_edit_snapshot(cx: &mut App) {
        cx.new(|cx| {
            let mut buffer = Buffer::local("zero\none\ntwo\nthree\n", cx);
            let old_snapshot = buffer.text_snapshot();
            buffer.edit([(5..8, "ONE")], None, cx);
            let current_snapshot = buffer.snapshot();
            let window = fixed_line_window_around_cursor(&current_snapshot, Point::new(1, 1), 1, 1);

            let stored_event = StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path: Arc::from(std::path::Path::new("test.txt")),
                    path: Arc::from(std::path::Path::new("test.txt")),
                    diff: String::new(),
                    in_open_source_repo: false,
                    predicted: false,
                }),
                old_snapshot,
                edit_range: current_snapshot.anchor_before(5)..current_snapshot.anchor_before(8),
            };

            let original_window =
                original_window_for_current_window(window, Some(&stored_event), &current_snapshot)
                    .expect("expected original window");

            assert_eq!(original_window, "zero\none\ntwo");
            buffer
        });
    }

    #[gpui::test]
    fn test_original_window_for_current_window_returns_none_without_matching_history(cx: &mut App) {
        cx.new(|cx| {
            let buffer = Buffer::local("hello\nworld\n", cx);
            let current_snapshot = buffer.snapshot();
            let window = fixed_line_window_around_cursor(&current_snapshot, Point::new(0, 0), 0, 1);

            assert_eq!(
                original_window_for_current_window(window, None, &current_snapshot),
                None
            );

            buffer
        });
    }

    #[gpui::test]
    fn test_original_window_for_current_window_treats_edit_end_as_post_edit_position(cx: &mut App) {
        cx.new(|cx| {
            let mut buffer = Buffer::local("aaaaabbbbb", cx);
            let old_snapshot = buffer.text_snapshot();
            buffer.edit([(5..10, "b")], None, cx);
            let current_snapshot = buffer.snapshot();
            let window = Point::new(0, 0)..Point::new(0, current_snapshot.line_len(0));

            let stored_event = StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path: Arc::from(std::path::Path::new("test.txt")),
                    path: Arc::from(std::path::Path::new("test.txt")),
                    diff: String::new(),
                    in_open_source_repo: false,
                    predicted: false,
                }),
                old_snapshot,
                edit_range: current_snapshot.anchor_before(5)..current_snapshot.anchor_before(6),
            };

            assert_eq!(
                map_current_offset_to_old_snapshot(6, &stored_event, &current_snapshot),
                Some(10)
            );

            let original_window =
                original_window_for_current_window(window, Some(&stored_event), &current_snapshot)
                    .expect("expected original window");

            assert_eq!(original_window, "aaaaabbbbb");
            buffer
        });
    }

    #[gpui::test]
    fn test_recent_change_block_from_event_formats_original_and_updated_sections(cx: &mut App) {
        cx.new(|cx| {
            let buffer = Buffer::local("fn main() {\n    println!(\"old\");\n}\n", cx);
            let stored_event = StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path: Path::new("src/main.rs").into(),
                    path: Path::new("src/main.rs").into(),
                    diff: "@@ -1,3 +1,3 @@\n fn main() {\n-    println!(\"old\");\n+    println!(\"new\");\n }\n"
                        .to_string(),
                    in_open_source_repo: false,
                    predicted: false,
                }),
                old_snapshot: buffer.text_snapshot(),
                edit_range: language::Anchor::MIN..language::Anchor::MIN,
            };

            let block =
                recent_change_block_from_event(&stored_event).expect("expected recent change block");
            assert_eq!(block.original, "fn main() {\n    println!(\"old\");\n}");
            assert_eq!(block.updated, "fn main() {\n    println!(\"new\");\n}");

            buffer
        });
    }

    #[test]
    fn test_build_prompt_uses_run_model_ordering() {
        let prompt = build_prompt(&SweepPromptInput {
            file_path: Path::new("src/main.rs").into(),
            original_window: "old window".to_string(),
            current_window: "current window".to_string(),
            recent_changes: vec![RecentChangeBlock {
                file_path: Path::new("src/lib.rs").into(),
                original: "old".to_string(),
                updated: "new".to_string(),
            }],
            related_files: vec![RelatedFileBlock {
                file_path: Path::new("src/context.rs").into(),
                content: "context".to_string(),
            }],
        });

        assert_eq!(
            prompt,
            "\
<|file_sep|>src/context.rs\n\
context\n\
<|file_sep|>src/lib.rs.diff\n\
original:\n\
old\n\
updated:\n\
new\n\
<|file_sep|>original/src/main.rs\n\
old window\n\
<|file_sep|>current/src/main.rs\n\
current window\n\
<|file_sep|>updated/src/main.rs\n"
        );
    }
}
