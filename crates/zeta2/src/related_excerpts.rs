use std::{cmp::Reverse, fmt::Write, ops::Range, path::PathBuf, sync::Arc};

use crate::merge_excerpts::write_merged_excerpts;
use anyhow::{Result, anyhow};
use collections::HashMap;
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions, Line};
use futures::{StreamExt, stream::BoxStream};
use gpui::{App, AsyncApp, Entity, Task};
use indoc::indoc;
use language::{Anchor, Bias, Buffer, OffsetRangeExt, Point, TextBufferSnapshot, ToPoint as _};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelToolResult, MessageContent, Role,
};
use project::{
    Project, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use schemars::JsonSchema;
use serde::Deserialize;
use util::paths::{PathMatcher, PathStyle};
use workspace::item::Settings as _;

const SEARCH_PROMPT: &str = indoc! {r#"
    ## Task

    You are part of an edit prediction system in a code editor. Your role is to identify relevant code locations
    that will serve as context for predicting the next required edit.

    **Your task:**
    - Analyze the user's recent edits and current cursor context
    - Use the `search` tool to find code that may be relevant for predicting the next edit
    - Focus on finding:
       - Code patterns that might need similar changes based on the recent edits
       - Functions, variables, types, and constants referenced in the current cursor context
       - Related implementations, usages, or dependencies that may require consistent updates

    **Important constraints:**
    - This conversation has exactly 2 turns
    - You must make ALL search queries in your first response via the `search` tool
    - All queries will be executed in parallel and results returned together
    - In the second turn, you will select the most relevant results via the `select` tool.

    ## User Edits

    {edits}

    ## Current cursor context

    `````filename={current_file_path}
    {cursor_excerpt}
    `````

    --
    Use the `search` tool now
"#};

const SEARCH_TOOL_NAME: &str = "search";

/// Search for relevant code
///
/// For the best results, run multiple queries at once with a single invocation of this tool.
#[derive(Deserialize, JsonSchema)]
struct SearchToolInput {
    /// An array of queries to run for gathering context relevant to the next prediction
    #[schemars(length(max = 5))]
    queries: Box<[SearchToolQuery]>,
}

#[derive(Deserialize, JsonSchema)]
struct SearchToolQuery {
    /// A glob pattern to match file paths in the codebase
    glob: String,
    /// A regular expression to match content within the files matched by the glob pattern
    regex: String,
    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    #[serde(default)]
    case_sensitive: bool,
}

const RESULTS_MESSAGE: &str = indoc! {"
    Here are the results of your queries combined and grouped by file:

"};

const SELECT_TOOL_NAME: &str = "select";

const SELECT_PROMPT: &str = indoc! {"
    Use the `select` tool now to pick the most relevant line ranges according to the user state provided in the first message.
    Make sure to include enough lines of context so that the edit prediction model can suggest accurate edits.
    Include up to 200 lines in total.
"};

/// Select line ranges from search results
#[derive(Deserialize, JsonSchema)]
struct SelectToolInput {
    /// The line ranges to select from search results.
    ranges: Vec<SelectLineRange>,
}

/// A specific line range to select from a file
#[derive(Debug, Deserialize, JsonSchema)]
struct SelectLineRange {
    /// The file path containing the lines to select
    /// Exactly as it appears in the search result codeblocks.
    path: PathBuf,
    /// The starting line number (1-based)
    #[schemars(range(min = 1))]
    start_line: u32,
    /// The ending line number (1-based, inclusive)
    #[schemars(range(min = 1))]
    end_line: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlmContextOptions {
    pub excerpt: EditPredictionExcerptOptions,
}

pub fn find_related_excerpts<'a>(
    buffer: Entity<language::Buffer>,
    cursor_position: Anchor,
    project: &Entity<Project>,
    events: impl Iterator<Item = &'a crate::Event>,
    options: &LlmContextOptions,
    cx: &App,
) -> Task<Result<HashMap<Entity<Buffer>, Vec<Range<Anchor>>>>> {
    let language_model_registry = LanguageModelRegistry::global(cx);
    let Some(model) = language_model_registry
        .read(cx)
        .available_models(cx)
        .find(|model| {
            model.provider_id() == language_model::ANTHROPIC_PROVIDER_ID
                && model.id() == LanguageModelId("claude-haiku-4-5-latest".into())
        })
    else {
        return Task::ready(Err(anyhow!("could not find claude model")));
    };

    let mut edits_string = String::new();

    for event in events {
        if let Some(event) = event.to_request_event(cx) {
            writeln!(&mut edits_string, "{event}").ok();
        }
    }

    if edits_string.is_empty() {
        edits_string.push_str("(No user edits yet)");
    }

    // TODO [zeta2] include breadcrumbs?
    let snapshot = buffer.read(cx).snapshot();
    let cursor_point = cursor_position.to_point(&snapshot);
    let Some(cursor_excerpt) =
        EditPredictionExcerpt::select_from_buffer(cursor_point, &snapshot, &options.excerpt, None)
    else {
        return Task::ready(Ok(HashMap::default()));
    };

    let current_file_path = snapshot
        .file()
        .map(|f| f.full_path(cx).display().to_string())
        .unwrap_or_else(|| "untitled".to_string());

    let prompt = SEARCH_PROMPT
        .replace("{edits}", &edits_string)
        .replace("{current_file_path}", &current_file_path)
        .replace("{cursor_excerpt}", &cursor_excerpt.text(&snapshot).body);

    let path_style = project.read(cx).path_style(cx);

    let exclude_matcher = {
        let global_settings = WorktreeSettings::get_global(cx);
        let exclude_patterns = global_settings
            .file_scan_exclusions
            .sources()
            .iter()
            .chain(global_settings.private_files.sources().iter());

        match PathMatcher::new(exclude_patterns, path_style) {
            Ok(matcher) => matcher,
            Err(err) => {
                return Task::ready(Err(anyhow!(err)));
            }
        }
    };

    let project = project.clone();
    cx.spawn(async move |cx| {
        let initial_prompt_message = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            cache: false,
        };

        let mut search_stream = request_tool_call::<SearchToolInput>(
            vec![initial_prompt_message.clone()],
            SEARCH_TOOL_NAME,
            &model,
            cx,
        )
        .await?;

        let mut select_request_messages = Vec::with_capacity(5); // initial prompt, LLM response/thinking, tool use, tool result, select prompt
        select_request_messages.push(initial_prompt_message);
        let mut search_calls = Vec::new();

        while let Some(event) = search_stream.next().await {
            match event? {
                LanguageModelCompletionEvent::ToolUse(tool_use) => {
                    if !tool_use.is_input_complete {
                        continue;
                    }

                    if tool_use.name.as_ref() == SEARCH_TOOL_NAME {
                        search_calls.push((select_request_messages.len(), tool_use));
                    } else {
                        log::warn!(
                            "context gathering model tried to use unknown tool: {}",
                            tool_use.name
                        );
                    }
                }
                LanguageModelCompletionEvent::Text(txt) => {
                    if let Some(LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content,
                        ..
                    }) = select_request_messages.last_mut()
                    {
                        if let Some(MessageContent::Text(existing_text)) = content.last_mut() {
                            existing_text.push_str(&txt);
                        } else {
                            content.push(MessageContent::Text(txt));
                        }
                    } else {
                        select_request_messages.push(LanguageModelRequestMessage {
                            role: Role::Assistant,
                            content: vec![MessageContent::Text(txt)],
                            cache: false,
                        });
                    }
                }
                LanguageModelCompletionEvent::Thinking { text, signature } => {
                    if let Some(LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content,
                        ..
                    }) = select_request_messages.last_mut()
                    {
                        if let Some(MessageContent::Thinking {
                            text: existing_text,
                            signature: existing_signature,
                        }) = content.last_mut()
                        {
                            existing_text.push_str(&text);
                            *existing_signature = signature;
                        } else {
                            content.push(MessageContent::Thinking { text, signature });
                        }
                    } else {
                        select_request_messages.push(LanguageModelRequestMessage {
                            role: Role::Assistant,
                            content: vec![MessageContent::Thinking { text, signature }],
                            cache: false,
                        });
                    }
                }
                LanguageModelCompletionEvent::RedactedThinking { data } => {
                    if let Some(LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content,
                        ..
                    }) = select_request_messages.last_mut()
                    {
                        if let Some(MessageContent::RedactedThinking(existing_data)) =
                            content.last_mut()
                        {
                            existing_data.push_str(&data);
                        } else {
                            content.push(MessageContent::RedactedThinking(data));
                        }
                    } else {
                        select_request_messages.push(LanguageModelRequestMessage {
                            role: Role::Assistant,
                            content: vec![MessageContent::RedactedThinking(data)],
                            cache: false,
                        });
                    }
                }
                ev @ LanguageModelCompletionEvent::ToolUseJsonParseError { .. } => {
                    log::error!("{ev:?}");
                }
                ev => {
                    log::trace!("context search event: {ev:?}")
                }
            }
        }

        struct ResultBuffer {
            buffer: Entity<Buffer>,
            snapshot: TextBufferSnapshot,
        }

        let mut result_buffers_by_path = HashMap::default();

        for (index, tool_use) in search_calls.into_iter().rev() {
            let call = serde_json::from_value::<SearchToolInput>(tool_use.input.clone())?;

            let mut excerpts_by_buffer = HashMap::default();

            for query in call.queries {
                // TODO [zeta2] parallelize?

                run_query(
                    query,
                    &mut excerpts_by_buffer,
                    path_style,
                    exclude_matcher.clone(),
                    &project,
                    cx,
                )
                .await?;
            }

            if excerpts_by_buffer.is_empty() {
                continue;
            }

            let mut merged_result = RESULTS_MESSAGE.to_string();

            for (buffer_entity, mut excerpts_for_buffer) in excerpts_by_buffer {
                excerpts_for_buffer.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

                buffer_entity
                    .clone()
                    .read_with(cx, |buffer, cx| {
                        let Some(file) = buffer.file() else {
                            return;
                        };

                        let path = file.full_path(cx);

                        writeln!(&mut merged_result, "`````filename={}", path.display()).unwrap();

                        let snapshot = buffer.snapshot();

                        write_merged_excerpts(
                            &snapshot,
                            excerpts_for_buffer,
                            &[],
                            &mut merged_result,
                        );

                        merged_result.push_str("`````\n\n");

                        result_buffers_by_path.insert(
                            path,
                            ResultBuffer {
                                buffer: buffer_entity,
                                snapshot: snapshot.text,
                            },
                        );
                    })
                    .ok();
            }

            let tool_result = LanguageModelToolResult {
                tool_use_id: tool_use.id.clone(),
                tool_name: SEARCH_TOOL_NAME.into(),
                is_error: false,
                content: merged_result.into(),
                output: None,
            };

            // Almost always appends at the end, but in theory, the model could return some text after the tool call
            // or perform parallel tool calls, so we splice at the message index for correctness.
            select_request_messages.splice(
                index..index,
                [
                    LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content: vec![MessageContent::ToolUse(tool_use)],
                        cache: false,
                    },
                    LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![MessageContent::ToolResult(tool_result)],
                        cache: false,
                    },
                ],
            );
        }

        if result_buffers_by_path.is_empty() {
            log::trace!("context gathering queries produced no results");
            return anyhow::Ok(HashMap::default());
        }

        select_request_messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![SELECT_PROMPT.into()],
            cache: false,
        });

        let mut select_stream = request_tool_call::<SelectToolInput>(
            select_request_messages,
            SELECT_TOOL_NAME,
            &model,
            cx,
        )
        .await?;
        let mut selected_ranges = Vec::new();

        while let Some(event) = select_stream.next().await {
            match event? {
                LanguageModelCompletionEvent::ToolUse(tool_use) => {
                    if !tool_use.is_input_complete {
                        continue;
                    }

                    if tool_use.name.as_ref() == SELECT_TOOL_NAME {
                        let call =
                            serde_json::from_value::<SelectToolInput>(tool_use.input.clone())?;
                        selected_ranges.extend(call.ranges);
                    } else {
                        log::warn!(
                            "context gathering model tried to use unknown tool: {}",
                            tool_use.name
                        );
                    }
                }
                ev @ LanguageModelCompletionEvent::ToolUseJsonParseError { .. } => {
                    log::error!("{ev:?}");
                }
                ev => {
                    log::trace!("context select event: {ev:?}")
                }
            }
        }

        if selected_ranges.is_empty() {
            log::trace!("context gathering selected no ranges")
        }

        let mut related_excerpts_by_buffer: HashMap<_, Vec<_>> = HashMap::default();

        for selected_range in selected_ranges {
            if let Some(ResultBuffer { buffer, snapshot }) =
                result_buffers_by_path.get(&selected_range.path)
            {
                let start_point = Point::new(selected_range.start_line.saturating_sub(1), 0);
                let end_point =
                    snapshot.clip_point(Point::new(selected_range.end_line, 0), Bias::Left);
                let range = snapshot.anchor_after(start_point)..snapshot.anchor_before(end_point);

                related_excerpts_by_buffer
                    .entry(buffer.clone())
                    .or_default()
                    .push(range);
            } else {
                log::warn!(
                    "selected path that wasn't included in search results: {}",
                    selected_range.path.display()
                );
            }
        }

        for (buffer, ranges) in &mut related_excerpts_by_buffer {
            buffer.read_with(cx, |buffer, _cx| {
                ranges.sort_unstable_by(|a, b| {
                    a.start
                        .cmp(&b.start, buffer)
                        .then(b.end.cmp(&a.end, buffer))
                });
            })?;
        }

        anyhow::Ok(related_excerpts_by_buffer)
    })
}

async fn request_tool_call<T: JsonSchema>(
    messages: Vec<LanguageModelRequestMessage>,
    tool_name: &'static str,
    model: &Arc<dyn LanguageModel>,
    cx: &mut AsyncApp,
) -> Result<BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>>
{
    let schema = schemars::schema_for!(T);

    let request = LanguageModelRequest {
        messages,
        tools: vec![LanguageModelRequestTool {
            name: tool_name.into(),
            description: schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap()
                .to_string(),
            input_schema: serde_json::to_value(schema).unwrap(),
        }],
        ..Default::default()
    };

    Ok(model.stream_completion(request, cx).await?)
}

const MIN_EXCERPT_LEN: usize = 16;
const MAX_EXCERPT_LEN: usize = 768;
const MAX_RESULT_BYTES_PER_QUERY: usize = MAX_EXCERPT_LEN * 5;

async fn run_query(
    args: SearchToolQuery,
    excerpts_by_buffer: &mut HashMap<Entity<Buffer>, Vec<Range<Line>>>,
    path_style: PathStyle,
    exclude_matcher: PathMatcher,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let include_matcher = PathMatcher::new(vec![args.glob], path_style)?;

    let query = SearchQuery::regex(
        &args.regex,
        false,
        args.case_sensitive,
        false,
        true,
        include_matcher,
        exclude_matcher,
        true,
        None,
    )?;

    let results = project.update(cx, |project, cx| project.search(query, cx))?;
    futures::pin_mut!(results);

    let mut total_bytes = 0;

    while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
        if ranges.is_empty() {
            continue;
        }

        let excerpts_for_buffer = excerpts_by_buffer
            .entry(buffer.clone())
            .or_insert_with(|| Vec::with_capacity(ranges.len()));

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

        for range in ranges {
            let offset_range = range.to_offset(&snapshot);
            let query_point = (offset_range.start + offset_range.len() / 2).to_point(&snapshot);

            if total_bytes + MIN_EXCERPT_LEN >= MAX_RESULT_BYTES_PER_QUERY {
                break;
            }

            let excerpt = EditPredictionExcerpt::select_from_buffer(
                query_point,
                &snapshot,
                &EditPredictionExcerptOptions {
                    max_bytes: MAX_EXCERPT_LEN.min(MAX_RESULT_BYTES_PER_QUERY - total_bytes),
                    min_bytes: MIN_EXCERPT_LEN,
                    target_before_cursor_over_total_bytes: 0.5,
                },
                None,
            );

            if let Some(excerpt) = excerpt {
                total_bytes += excerpt.range.len();
                if !excerpt.line_range.is_empty() {
                    excerpts_for_buffer.push(excerpt.line_range);
                }
            }
        }

        if excerpts_for_buffer.is_empty() {
            excerpts_by_buffer.remove(&buffer);
        }
    }

    anyhow::Ok(())
}
