use std::{fmt::Write, ops::Range, sync::Arc};

use anyhow::{Result, anyhow};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::TryStreamExt as _;
use gpui::{App, Entity, Task};
use indoc::indoc;
use language::{Anchor, Rope, ToPoint as _};
use language_model::{
    LanguageModelId, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use project::{Project, WorktreeId};
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
    You are a part of an edit prediction system in a code editor.

    Given a sequence of edits by the user, the system predicts the next edits that the user will make.

    The first step of this process is to find other locations in a codebase that need to
    be edited or read, in order to compute edits.

    Your task is to determine which queries should be run on the user's machine to
    find those locations in the codebase.

    You must output a JSON array matching the following schema:
    <schema>
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "glob": {
                    "type": "string",
                    "description": "A glob pattern to match file paths in the codebase"
                },
                "regex": {
                    "type": "string",
                    "description": "A regular expression to match content within the files matched by the glob pattern"
                }
            },
            "required": ["glob", "regex"],
            "additionalProperties": false
        }
    }
    </schema>

    Include up to 5 queries. The results of all of these queries will be returned together
    so that you can pick the most relevant for the edit prediction.

    ## User Edits

    {edits}

    ## Context around the cursor

    `````
    {cursor_excerpt}
    `````
"#};

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
    let Some(cursor_excerpt_string) = EditPredictionExcerpt::select_from_buffer(
        cursor_position.to_point(&snapshot),
        &snapshot,
        excerpt_options,
        None,
    ) else {
        return Task::ready(Ok(Vec::new()));
    };

    let prompt = PROMPT.replace("{edits}", &edits_string).replace(
        "{cursor_excerpt}",
        &cursor_excerpt_string.text(&snapshot).body,
    );
    eprintln!("\n\n{prompt}");

    let request = LanguageModelRequest {
        messages: vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            cache: false,
        }],
        ..Default::default()
    };

    cx.spawn(async move |cx| {
        let stream = model.stream_completion_text(request, cx).await?;
        let text: String = stream.stream.try_collect().await?;
        eprintln!("query JSON:\n{text}");
        let excerpts = Vec::new();

        anyhow::Ok(excerpts)
    })
}
