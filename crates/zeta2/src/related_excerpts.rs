use std::{fmt::Write, ops::Range, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::{StreamExt, TryStreamExt as _};
use gpui::{App, Entity, Task};
use indoc::indoc;
use language::{Anchor, Rope, ToPoint as _};
use language_model::{
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, Role,
};
use project::{Project, WorktreeId};
use schemars::JsonSchema;
use serde::Deserialize;
use util::rel_path::RelPath;

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
    queries: Vec<SearchToolQuery>,
}

#[derive(Deserialize, JsonSchema)]
struct SearchToolQuery {
    /// A glob pattern to match file paths in the codebase
    glob: String,
    /// A regular expression to match content within the files matched by the glob pattern
    regex: String,
}

pub fn find_related_excerpts<'a>(
    buffer: Entity<language::Buffer>,
    cursor_position: Anchor,
    _project: &Entity<Project>,
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

    let prompt = PROMPT
        .replace("{edits}", &edits_string)
        .replace(
            "{current_file_path}",
            snapshot
                .file()
                .map(|f| f.path().as_unix_str())
                .unwrap_or("untitled"),
        )
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

    cx.spawn(async move |cx| {
        let mut stream = model.stream_completion(request, cx).await?;

        while let Some(event) = stream.next().await {
            match event? {
                LanguageModelCompletionEvent::ToolUse(tool_use) => {
                    if tool_use.name.as_ref() == SEARCH_TOOL_NAME {
                        // todo! handle streaming
                        let input: SearchToolInput = serde_json::from_value(tool_use.input)
                            .with_context(|| tool_use.raw_input.to_string())?;

                        println!("\n\nSearch tool invocation:");
                        for query in input.queries {
                            println!(r#"glob: "{}", regex: "{}""#, query.glob, query.regex);
                        }
                    } else {
                        log::warn!(
                            "context gathering model tried to use unknown tool: {}",
                            tool_use.name
                        );
                    }
                }
                LanguageModelCompletionEvent::Text(txt) => {
                    eprint!("{txt}");
                }
                LanguageModelCompletionEvent::Stop(reason) => {
                    eprintln!("\nStopped {reason:?}")
                }
                LanguageModelCompletionEvent::Thinking { .. } => {}
                LanguageModelCompletionEvent::StatusUpdate(_)
                | LanguageModelCompletionEvent::RedactedThinking { .. }
                | LanguageModelCompletionEvent::ToolUseJsonParseError { .. }
                | LanguageModelCompletionEvent::StartMessage { .. }
                | LanguageModelCompletionEvent::UsageUpdate(..) => {}
            }
        }
        println!();

        // todo! fail if no queries were run

        let excerpts = Vec::new();

        anyhow::Ok(excerpts)
    })
}
