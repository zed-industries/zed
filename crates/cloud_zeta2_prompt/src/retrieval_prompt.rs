use anyhow::Result;
use cloud_llm_client::predict_edits_v3::{self, Excerpt};
use indoc::indoc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

use crate::{push_events, write_codeblock};

pub fn build_prompt(request: predict_edits_v3::PlanContextRetrievalRequest) -> Result<String> {
    let mut prompt = SEARCH_INSTRUCTIONS.to_string();

    if !request.events.is_empty() {
        writeln!(&mut prompt, "\n## User Edits\n\n")?;
        push_events(&mut prompt, &request.events);
    }

    writeln!(&mut prompt, "## Cursor context\n")?;
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
    #[schemars(length(max = 3))]
    pub queries: Box<[SearchToolQuery]>,
}

/// Search for relevant code by path, syntax hierarchy, and content.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Hash)]
pub struct SearchToolQuery {
    /// 1. A glob pattern to match file paths in the codebase to search in.
    pub glob: String,
    /// 2. Regular expressions to match syntax nodes **by their first line** and hierarchy.
    ///
    /// Subsequent regexes match nodes within the full content of the nodes matched by the previous regexes.
    ///
    /// Example: Searching for a `User` class
    ///     ["class\s+User"]
    ///
    /// Example: Searching for a `get_full_name` method under a `User` class
    ///     ["class\s+User", "def\sget_full_name"]
    ///
    /// Skip this field to match on content alone.
    #[schemars(length(max = 3))]
    #[serde(default)]
    pub syntax_node: Vec<String>,
    /// 3. An optional regular expression to match the final content that should appear in the results.
    ///
    /// - Content will be matched within all lines of the matched syntax nodes.
    /// - If syntax node regexes are provided, this field can be skipped to include as much of the node itself as possible.
    /// - If no syntax node regexes are provided, the content will be matched within the entire file.
    pub content: Option<String>,
}

pub const TOOL_NAME: &str = "search";

const SEARCH_INSTRUCTIONS: &str = indoc! {r#"
    You are part of an edit prediction system in a code editor.
    Your role is to search for code that will serve as context for predicting the next edit.

    - Analyze the user's recent edits and current cursor context
    - Use the `search` tool to find code that is relevant for predicting the next edit
    - Focus on finding:
       - Code patterns that might need similar changes based on the recent edits
       - Functions, variables, types, and constants referenced in the current cursor context
       - Related implementations, usages, or dependencies that may require consistent updates
       - How items defined in the cursor excerpt are used or altered
    - You will not be able to filter results or perform subsequent queries, so keep searches as targeted as possible
    - Use `syntax_node` parameter whenever you're looking for a particular type, class, or function
    - Avoid using wildcard globs if you already know the file path of the content you're looking for
"#};

const TOOL_USE_REMINDER: &str = indoc! {"
    --
    Analyze the user's intent in one to two sentences, then call the `search` tool.
"};
