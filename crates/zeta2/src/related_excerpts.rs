mod merge_excerpts;

use std::{cmp::Reverse, fmt::Write, ops::Range, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions, Line};
use futures::StreamExt;
use gpui::{App, AsyncApp, Entity, Task};
use indoc::indoc;
use language::{Anchor, Buffer, OffsetRangeExt, Rope, ToPoint as _};
use language_model::{
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, Role,
};
use project::{
    Project, WorktreeId, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use schemars::JsonSchema;
use serde::Deserialize;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};
use workspace::item::Settings as _;

use crate::related_excerpts::merge_excerpts::write_merged_excerpts;

pub(crate) enum RelatedExcerpt {
    Buffer {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        rope: Rope,
        range: Range<Anchor>,
    },
    File {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        text: Arc<str>,
        row_range: Range<u32>,
    },
}

const PROMPT: &str = indoc! {r#"
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
    - In the second turn, you will select the most relevant results

    ## User Edits

    {edits}

    ## Current cursor context

    `````filename={current_file_path}
    {cursor_excerpt}
    `````
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

pub fn find_related_excerpts<'a>(
    buffer: Entity<language::Buffer>,
    cursor_position: Anchor,
    project: &Entity<Project>,
    events: impl Iterator<Item = &'a crate::Event>,
    excerpt_options: &EditPredictionExcerptOptions,
    cx: &App,
) -> Task<Result<Vec<RelatedExcerpt>>> {
    let language_model_registry = LanguageModelRegistry::global(cx);
    let Some(model) = language_model_registry
        .read(cx)
        .available_models(cx)
        .find(|model| {
            model.provider_id() == language_model::ANTHROPIC_PROVIDER_ID
                && model.id() == LanguageModelId("claude-sonnet-4-5-latest".into())
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
    let Some(cursor_excerpt) = EditPredictionExcerpt::select_from_buffer(
        cursor_position.to_point(&snapshot),
        &snapshot,
        excerpt_options,
        None,
    ) else {
        return Task::ready(Ok(Vec::new()));
    };

    let current_file_path = snapshot
        .file()
        .map(|f| f.full_path(cx).display().to_string())
        .unwrap_or_else(|| "untitled".to_string());

    let prompt = PROMPT
        .replace("{edits}", &edits_string)
        .replace("{current_file_path}", &current_file_path)
        .replace("{cursor_excerpt}", &cursor_excerpt.text(&snapshot).body);
    eprintln!("\n\n{prompt}");

    let schema = schemars::schema_for!(SearchToolInput);

    let request = LanguageModelRequest {
        messages: vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            cache: false,
        }],
        tools: vec![LanguageModelRequestTool {
            name: SEARCH_TOOL_NAME.into(),
            description: schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap()
                .to_string(),
            input_schema: serde_json::to_value(schema).unwrap(),
        }],
        ..Default::default()
    };

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
        let mut stream = model.stream_completion(request, cx).await?;
        let mut queries = Vec::new();

        while let Some(event) = stream.next().await {
            match event? {
                LanguageModelCompletionEvent::ToolUse(tool_use) => {
                    if !tool_use.is_input_complete {
                        continue;
                    }

                    if tool_use.name.as_ref() == SEARCH_TOOL_NAME {
                        let input: SearchToolInput = serde_json::from_value(tool_use.input)
                            .with_context(|| tool_use.raw_input.to_string())?;

                        queries.extend(input.queries);
                    } else {
                        log::warn!(
                            "context gathering model tried to use unknown tool: {}",
                            tool_use.name
                        );
                    }
                }
                LanguageModelCompletionEvent::Text(txt) => {
                    // todo!
                    eprint!("{txt}");
                }
                LanguageModelCompletionEvent::StatusUpdate(_)
                | LanguageModelCompletionEvent::Stop(_)
                | LanguageModelCompletionEvent::Thinking { .. }
                | LanguageModelCompletionEvent::RedactedThinking { .. }
                | LanguageModelCompletionEvent::ToolUseJsonParseError { .. }
                | LanguageModelCompletionEvent::StartMessage { .. }
                | LanguageModelCompletionEvent::UsageUpdate(..) => {}
            }
        }

        if queries.is_empty() {
            return anyhow::Ok(Vec::new());
        }

        let mut excerpts_by_buffer = HashMap::default();

        // todo! parallelize?
        for query in queries {
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

        let mut merged = String::new();

        for (buffer, mut excerpts_for_buffer) in excerpts_by_buffer {
            excerpts_for_buffer.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

            buffer
                .read_with(cx, |buffer, cx| {
                    let Some(file) = buffer.file() else {
                        return;
                    };

                    writeln!(
                        &mut merged,
                        "`````filename={}",
                        file.full_path(cx).display()
                    )
                    .unwrap();

                    write_merged_excerpts(&buffer.snapshot(), excerpts_for_buffer, &mut merged);

                    merged.push_str("`````\n\n");
                })
                .ok();
        }

        anyhow::Ok(vec![])
    })
}

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

            const MIN_EXCERPT_LEN: usize = 16;
            const MAX_EXCERPT_LEN: usize = 768;
            const MAX_RESULT_BYTES_PER_QUERY: usize = MAX_EXCERPT_LEN * 5;

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
                excerpts_for_buffer.push(excerpt.line_range);
            }
        }
    }

    anyhow::Ok(())
}
