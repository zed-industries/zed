use anyhow::Result;
use cloud_llm_client::predict_edits_v3::{self, Excerpt};
use indoc::indoc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, sync::LazyLock};

use crate::{push_events, write_codeblock};

pub fn build_prompt(request: predict_edits_v3::PlanContextRetrievalRequest) -> Result<String> {
    let mut prompt = SEARCH_INSTRUCTIONS.to_string();

    if !request.events.is_empty() {
        writeln!(&mut prompt, "## User Edits\n")?;
        push_events(&mut prompt, &request.events);
    }

    writeln!(&mut prompt, "## Excerpt around the cursor\n")?;
    write_codeblock(
        &request.excerpt_path,
        &[Excerpt {
            start_line: request.excerpt_line_range.start,
            text: request.excerpt.into(),
        }],
        &[],
        request.cursor_file_max_row,
        true,
        &mut prompt,
    );

    writeln!(&mut prompt, "{TOOL_USE_REMINDER}")?;

    Ok(prompt)
}

/// Search for relevant code
///
/// For the best results, run multiple queries at once with a single invocation of this tool.
#[derive(Clone, Deserialize, Serialize, JsonSchema)]
pub struct SearchToolInput {
    /// An array of queries to run for gathering context relevant to the next prediction
    #[schemars(length(max = 5))]
    pub queries: Box<[SearchToolQuery]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchToolQuery {
    /// A glob pattern to match file paths in the codebase
    pub glob: String,
    /// A regular expression to match content within the files matched by the glob pattern
    pub regex: String,
}

pub static TOOL_SCHEMA: LazyLock<(serde_json::Value, String)> = LazyLock::new(|| {
    let schema = schemars::schema_for!(SearchToolInput);

    let description = schema
        .get("description")
        .and_then(|description| description.as_str())
        .unwrap()
        .to_string();

    (schema.into(), description)
});

pub const TOOL_NAME: &str = "search";

const SEARCH_INSTRUCTIONS: &str = indoc! {r#"
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
"#};

const TOOL_USE_REMINDER: &str = indoc! {"
    --
    Use the `search` tool now
"};
