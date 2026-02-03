use crate::{
    EditPredictionId, EditPredictionModelInput, cursor_excerpt, prediction::EditPredictionResult,
    udiff::DiffLine, zeta1::compute_edits,
};
use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Task, http_client};
use language::{
    Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, ToOffset, ToPoint as _,
    apply_reversed_diff_patch, language_settings::all_language_settings,
};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc, time::Instant};
use zeta_prompt::{Event, ZetaPromptInput};

const MAX_REWRITE_TOKENS: usize = 150;
const MAX_CONTEXT_TOKENS: usize = 350;

const FILE_SEPARATOR: &str = "<|file_sep|>";
const SWEEP_CONTEXT_LINES: usize = 10;

pub struct Ollama {
    api_url: String,
}

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    raw: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaGenerateOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaGenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

pub fn is_available(cx: &App) -> bool {
    let ollama_provider_id = LanguageModelProviderId::new("ollama");
    LanguageModelRegistry::read_global(cx)
        .provider(&ollama_provider_id)
        .is_some_and(|provider| provider.is_authenticated(cx))
}

/// Output from the Ollama HTTP request, containing all data needed to create the prediction result.
struct OllamaRequestOutput {
    edits: Vec<(std::ops::Range<Anchor>, Arc<str>)>,
    snapshot: BufferSnapshot,
    response_received_at: Instant,
    inputs: ZetaPromptInput,
    buffer: Entity<Buffer>,
    buffer_snapshotted_at: Instant,
}

impl Ollama {
    pub fn new() -> Self {
        Ollama {
            api_url: "http://localhost:11434".to_string(),
        }
    }

    pub fn request_prediction(
        &self,
        EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            events,
            related_files,
            ..
        }: EditPredictionModelInput,
        cx: &mut App,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let settings = &all_language_settings(None, cx).edit_predictions.ollama;
        let Some(model) = settings.model.clone() else {
            return Task::ready(Ok(None));
        };
        let max_tokens = settings.max_tokens;

        log::debug!("Ollama: Requesting completion (model: {})", model);

        let full_path: Arc<Path> = snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let http_client = cx.http_client();
        let cursor_point = position.to_point(&snapshot);
        let buffer_snapshotted_at = Instant::now();
        let api_url = self.api_url.clone();

        let is_sweep_model = is_sweep_next_edit_model(&model);

        let result = cx.background_spawn(async move {
            let (editable_range, context_range) =
                cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                    cursor_point,
                    &snapshot,
                    MAX_CONTEXT_TOKENS,
                    MAX_REWRITE_TOKENS,
                );

            let related_files = crate::filter_redundant_excerpts(
                related_files,
                full_path.as_ref(),
                context_range.start.row..context_range.end.row,
            );

            let context_offset_range = context_range.to_offset(&snapshot);
            let context_start_row = context_range.start.row;
            let editable_offset_range = editable_range.to_offset(&snapshot);

            let inputs = ZetaPromptInput {
                events: events.clone(),
                related_files: related_files.clone(),
                cursor_offset_in_excerpt: cursor_point.to_offset(&snapshot)
                    - context_offset_range.start,
                cursor_path: full_path.clone(),
                cursor_excerpt: snapshot
                    .text_for_range(context_range)
                    .collect::<String>()
                    .into(),
                editable_range_in_excerpt: (editable_offset_range.start
                    - context_offset_range.start)
                    ..(editable_offset_range.end - context_offset_range.start),
                excerpt_start_row: Some(context_start_row),
            };

            let (prompt, stop_tokens, num_predict, sweep_window_range) = if is_sweep_model {
                let output = format_sweep_next_edit_prompt(&inputs, &events, &related_files);
                let stop_tokens = get_sweep_stop_tokens();
                (
                    output.prompt,
                    stop_tokens,
                    512u32,
                    Some(output.editable_range_in_excerpt),
                )
            } else {
                let prefix = inputs.cursor_excerpt[..inputs.cursor_offset_in_excerpt].to_string();
                let suffix = inputs.cursor_excerpt[inputs.cursor_offset_in_excerpt..].to_string();
                let prompt = format_fim_prompt(&model, &prefix, &suffix);
                let stop_tokens = get_fim_stop_tokens();
                (prompt, stop_tokens, max_tokens.unwrap_or(64), None)
            };

            let request = OllamaGenerateRequest {
                model: model.clone(),
                prompt,
                raw: true,
                stream: false,
                options: Some(OllamaGenerateOptions {
                    num_predict: Some(num_predict),
                    temperature: Some(0.2),
                    stop: Some(stop_tokens),
                }),
            };

            let request_body = serde_json::to_string(&request)?;
            let http_request = http_client::Request::builder()
                .method(http_client::Method::POST)
                .uri(format!("{}/api/generate", api_url))
                .header("Content-Type", "application/json")
                .body(http_client::AsyncBody::from(request_body))?;

            let mut response = http_client.send(http_request).await?;
            let status = response.status();

            log::debug!("Ollama: Response status: {}", status);

            if !status.is_success() {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                return Err(anyhow::anyhow!("Ollama API error: {} - {}", status, body));
            }

            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            let ollama_response: OllamaGenerateResponse =
                serde_json::from_str(&body).context("Failed to parse Ollama response")?;

            let response_received_at = Instant::now();

            log::debug!(
                "Ollama: Completion received ({:.2}s)",
                (response_received_at - buffer_snapshotted_at).as_secs_f64()
            );

            let edits = if is_sweep_model {
                let editable_range =
                    sweep_window_range.expect("sweep model should have editable range");
                let buffer_editable_start = context_offset_range.start + editable_range.start;
                let buffer_editable_end = context_offset_range.start + editable_range.end;
                let old_text = snapshot
                    .text_for_range(buffer_editable_start..buffer_editable_end)
                    .collect::<String>();
                let new_text = parse_sweep_next_edit_response(&ollama_response.response, &inputs);
                std::fs::write("/tmp/new_text.txt", new_text.as_bytes()).ok();
                compute_edits(old_text, &new_text, buffer_editable_start, &snapshot)
            } else {
                let completion: Arc<str> = clean_fim_completion(&ollama_response.response).into();
                if completion.is_empty() {
                    vec![]
                } else {
                    let cursor_offset = cursor_point.to_offset(&snapshot);
                    let anchor = snapshot.anchor_after(cursor_offset);
                    vec![(anchor..anchor, completion)]
                }
            };

            anyhow::Ok(OllamaRequestOutput {
                edits,
                snapshot,
                response_received_at,
                inputs,
                buffer,
                buffer_snapshotted_at,
            })
        });

        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            let output = result.await.context("Ollama edit prediction failed")?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(String::new().into()),
                    &output.buffer,
                    &output.snapshot,
                    output.edits.into(),
                    None,
                    output.buffer_snapshotted_at,
                    output.response_received_at,
                    output.inputs,
                    cx,
                )
                .await,
            ))
        })
    }
}

fn is_sweep_next_edit_model(model: &str) -> bool {
    let model_lower = model.to_lowercase();
    model_lower.contains("sweep") || model_lower.contains("sweepai")
}

struct SweepPromptOutput {
    prompt: String,
    editable_range_in_excerpt: std::ops::Range<usize>,
}

fn format_sweep_next_edit_prompt(
    inputs: &ZetaPromptInput,
    events: &[Arc<Event>],
    related_files: &[zeta_prompt::RelatedFile],
) -> SweepPromptOutput {
    let mut prompt = String::new();

    for related_file in related_files {
        let path_str = path_to_unix_string(&related_file.path);
        for excerpt in &related_file.excerpts {
            write!(prompt, "{FILE_SEPARATOR}{path_str}\n{}\n", excerpt.text).ok();
        }
    }

    for event in events {
        match event.as_ref() {
            Event::BufferChange {
                path,
                old_path,
                diff,
                ..
            } => {
                if !diff.is_empty() {
                    let path_str = path_to_unix_string(path);
                    let old_path_str = path_to_unix_string(old_path);

                    if let Some((original, updated)) = parse_diff_to_original_updated(diff) {
                        let diff_path = if path_str == old_path_str {
                            format!("{}.diff", path_str)
                        } else {
                            format!("{}.diff", old_path_str)
                        };

                        write!(
                            prompt,
                            "{FILE_SEPARATOR}{diff_path}\noriginal:\n{original}\nupdated:\n{updated}\n"
                        )
                        .ok();
                    }
                }
            }
        }
    }

    let file_path = path_to_unix_string(&inputs.cursor_path);

    let current_cursor_line =
        get_cursor_line(&inputs.cursor_excerpt, inputs.cursor_offset_in_excerpt);

    let (full_original, original_cursor_line) =
        compute_original_content_with_cursor(inputs, current_cursor_line);
    write!(
        prompt,
        "{FILE_SEPARATOR}original/{file_path}\n{full_original}\n"
    )
    .ok();

    let current_content = extract_lines_around(&inputs.cursor_excerpt, current_cursor_line);
    write!(
        prompt,
        "{FILE_SEPARATOR}current/{file_path}\n{current_content}\n"
    )
    .ok();

    write!(prompt, "{FILE_SEPARATOR}updated/{file_path}\n").ok();

    let editable_range_in_excerpt =
        compute_line_window_byte_range(&inputs.cursor_excerpt, current_cursor_line);

    SweepPromptOutput {
        prompt,
        editable_range_in_excerpt,
    }
}

fn get_cursor_line(content: &str, cursor_offset: usize) -> usize {
    content[..cursor_offset.min(content.len())]
        .matches('\n')
        .count()
}

fn extract_lines_around(content: &str, cursor_line: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = cursor_line.saturating_sub(SWEEP_CONTEXT_LINES);
    let end = (cursor_line + SWEEP_CONTEXT_LINES + 1).min(lines.len());
    lines[start..end].join("\n")
}

fn compute_line_window_byte_range(content: &str, cursor_line: usize) -> std::ops::Range<usize> {
    let lines: Vec<&str> = content.lines().collect();
    let start_line = cursor_line.saturating_sub(SWEEP_CONTEXT_LINES);
    let end_line = (cursor_line + SWEEP_CONTEXT_LINES + 1).min(lines.len());

    let mut byte_start = 0;
    for line in lines.iter().take(start_line) {
        byte_start += line.len() + 1; // +1 for newline
    }

    let mut byte_end = byte_start;
    for line in lines.iter().skip(start_line).take(end_line - start_line) {
        byte_end += line.len() + 1; // +1 for newline
    }

    // Clamp to content length (handles missing trailing newline)
    byte_start = byte_start.min(content.len());
    byte_end = byte_end.min(content.len());

    byte_start..byte_end
}

fn compute_original_cursor_line(diff: &str, current_cursor_line: usize) -> usize {
    let mut current_line: i64 = -1;
    let mut original_line: i64 = -1;

    for line in diff.lines() {
        match DiffLine::parse(line) {
            DiffLine::HunkHeader(Some(loc)) => {
                original_line = loc.start_line_old as i64;
                current_line = loc.start_line_new as i64;
            }
            DiffLine::Context(_) => {
                current_line += 1;
                original_line += 1;
                if current_line as usize == current_cursor_line {
                    return original_line as usize;
                }
            }
            DiffLine::Addition(_) => {
                current_line += 1;
                if current_line as usize == current_cursor_line {
                    return original_line.max(0) as usize;
                }
            }
            DiffLine::Deletion(_) => {
                original_line += 1;
            }
            _ => {}
        }
    }

    // Cursor is outside all hunks - apply accumulated offset
    let offset = current_line - original_line;
    (current_cursor_line as i64 - offset).max(0) as usize
}

fn compute_original_content_with_cursor(
    inputs: &ZetaPromptInput,
    current_cursor_line: usize,
) -> (String, usize) {
    let current_content: &str = &inputs.cursor_excerpt;

    for event in inputs.events.iter().rev() {
        match event.as_ref() {
            Event::BufferChange { path, diff, .. } => {
                if path.as_ref() == inputs.cursor_path.as_ref() && !diff.is_empty() {
                    if let Some(original) = extract_original_from_diff(diff, current_content) {
                        let original_cursor_line =
                            compute_original_cursor_line(diff, current_cursor_line);
                        return (original, original_cursor_line);
                    }
                }
            }
        }
    }

    (current_content.to_string(), current_cursor_line)
}

fn extract_original_from_diff(diff: &str, current_content: &str) -> Option<String> {
    apply_reversed_diff_patch(current_content, diff).ok()
}

fn parse_diff_to_original_updated(diff: &str) -> Option<(String, String)> {
    let mut original = String::new();
    let mut updated = String::new();

    for line in diff.lines() {
        if let Some(content) = line.strip_prefix('+') {
            if !content.starts_with("++") {
                updated.push_str(content);
                updated.push('\n');
            }
        } else if let Some(content) = line.strip_prefix('-') {
            if !content.starts_with("--") {
                original.push_str(content);
                original.push('\n');
            }
        } else if let Some(content) = line.strip_prefix(' ') {
            original.push_str(content);
            original.push('\n');
            updated.push_str(content);
            updated.push('\n');
        } else if !line.starts_with("@@") && !line.starts_with("diff ") {
            original.push_str(line);
            original.push('\n');
            updated.push_str(line);
            updated.push('\n');
        }
    }

    if original.is_empty() && updated.is_empty() {
        None
    } else {
        Some((original, updated))
    }
}

fn parse_sweep_next_edit_response(response: &str, inputs: &ZetaPromptInput) -> String {
    let file_path = path_to_unix_string(&inputs.cursor_path);
    let updated_marker = format!("updated/{}", file_path);

    if let Some(pos) = response.find(&updated_marker) {
        let content_start = pos + updated_marker.len();
        let content = &response[content_start..];

        let content = content.strip_prefix('\n').unwrap_or(content);

        if let Some(end_pos) = content.find(FILE_SEPARATOR) {
            return content[..end_pos].trim_end().to_string();
        }

        return clean_sweep_response(content);
    }

    if let Some(content) = response.strip_prefix(FILE_SEPARATOR) {
        if let Some(newline_pos) = content.find('\n') {
            let after_header = &content[newline_pos + 1..];
            if let Some(end_pos) = after_header.find(FILE_SEPARATOR) {
                return after_header[..end_pos].trim_end().to_string();
            }
            return clean_sweep_response(after_header);
        }
    }

    clean_sweep_response(response)
}

fn clean_sweep_response(response: &str) -> String {
    let mut result = response.to_string();

    let end_tokens = [
        FILE_SEPARATOR,
        "<|endoftext|>",
        "<|file_separator|>",
        "<|end|>",
    ];

    for token in &end_tokens {
        if let Some(pos) = result.find(token) {
            result.truncate(pos);
        }
    }

    result.trim_end().to_string()
}

fn path_to_unix_string(path: &Path) -> String {
    let mut result = String::new();
    for (i, component) in path.components().enumerate() {
        if i > 0 {
            result.push('/');
        }
        write!(result, "{}", component.as_os_str().to_string_lossy()).ok();
    }
    result
}

fn get_sweep_stop_tokens() -> Vec<String> {
    vec![
        FILE_SEPARATOR.to_string(),
        "<|endoftext|>".to_string(),
        "<|file_separator|>".to_string(),
        "<|end|>".to_string(),
    ]
}

fn format_fim_prompt(model: &str, prefix: &str, suffix: &str) -> String {
    let model_base = model.split(':').next().unwrap_or(model);

    match model_base {
        "codellama" | "code-llama" => {
            format!("<PRE> {prefix} <SUF>{suffix} <MID>")
        }
        "starcoder" | "starcoder2" | "starcoderbase" => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
        "deepseek-coder" | "deepseek-coder-v2" => {
            format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
        }
        "qwen2.5-coder" | "qwen-coder" | "qwen" => {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        }
        "codegemma" => {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        }
        "codestral" | "mistral" => {
            format!("[SUFFIX]{suffix}[PREFIX]{prefix}")
        }
        "glm" | "glm-4" | "glm-4.5" => {
            format!("<|code_prefix|>{prefix}<|code_suffix|>{suffix}<|code_middle|>")
        }
        _ => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
    }
}

fn get_fim_stop_tokens() -> Vec<String> {
    vec![
        "<|endoftext|>".to_string(),
        "<|file_separator|>".to_string(),
        "<|fim_pad|>".to_string(),
        "<|fim_prefix|>".to_string(),
        "<|fim_middle|>".to_string(),
        "<|fim_suffix|>".to_string(),
        "<fim_prefix>".to_string(),
        "<fim_middle>".to_string(),
        "<fim_suffix>".to_string(),
        "<PRE>".to_string(),
        "<SUF>".to_string(),
        "<MID>".to_string(),
        "[PREFIX]".to_string(),
        "[SUFFIX]".to_string(),
    ]
}

fn clean_fim_completion(response: &str) -> String {
    let mut result = response.to_string();

    let end_tokens = [
        "<|endoftext|>",
        "<|file_separator|>",
        "<|fim_pad|>",
        "<|fim_prefix|>",
        "<|fim_middle|>",
        "<|fim_suffix|>",
        "<fim_prefix>",
        "<fim_middle>",
        "<fim_suffix>",
        "<PRE>",
        "<SUF>",
        "<MID>",
        "[PREFIX]",
        "[SUFFIX]",
    ];

    for token in &end_tokens {
        if let Some(pos) = result.find(token) {
            result.truncate(pos);
        }
    }

    result
}
