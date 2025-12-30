use crate::{
    anthropic_client::PlainLlmClient,
    git::{ensure_repo_cloned, run_git},
    paths::{FAILED_EXAMPLES_DIR, SYNTHESIZE_STATE_FILE},
    progress::{InfoStyle, Progress, Step, StepProgress},
};
use anthropic::{Message, RequestContent, ResponseContent, Role, Tool, ToolChoice};
use anyhow::{Context as _, Result};
use chrono::Local;
use collections::{HashMap, HashSet};
use edit_prediction::{
    example_spec::{ExampleSpec, INLINE_CURSOR_MARKER},
    udiff::{apply_diff_to_string, extract_file_diff, strip_diff_metadata},
};
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct SynthesizeConfig {
    pub repo_url: String,
    pub count: usize,
    pub max_commits: usize,
    pub output_dir: PathBuf,
    pub require_context: bool,
    pub fresh: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SynthesizeState {
    repositories: HashMap<String, RepoState>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct RepoState {
    processed_commits: HashSet<String>,
    examples_generated: usize,
}

impl SynthesizeState {
    fn load() -> Self {
        if SYNTHESIZE_STATE_FILE.exists() {
            std::fs::read_to_string(&*SYNTHESIZE_STATE_FILE)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&*SYNTHESIZE_STATE_FILE, content)?;
        Ok(())
    }

    fn is_processed(&self, repo_url: &str, commit_sha: &str) -> bool {
        self.repositories
            .get(repo_url)
            .is_some_and(|repo| repo.processed_commits.contains(commit_sha))
    }

    fn mark_processed(&mut self, repo_url: &str, commit_sha: &str, examples_count: usize) {
        let repo = self.repositories.entry(repo_url.to_string()).or_default();
        repo.processed_commits.insert(commit_sha.to_string());
        repo.examples_generated += examples_count;
    }
}

#[derive(Debug)]
struct CommitInfo {
    sha: String,
    parent_sha: String,
    message: String,
    diff: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExampleCandidate {
    name: String,
    cursor_file: String,
    edit_history_files: Vec<String>,
    edit_history_description: String,
    expected_patch_description: String,
    cursor_location_hint: String,
    reasoning: String,
    #[serde(default)]
    requires_context: bool,
}

pub async fn run_synthesize(config: SynthesizeConfig) -> Result<()> {
    let mut state = if config.fresh {
        SynthesizeState::default()
    } else {
        SynthesizeState::load()
    };

    std::fs::create_dir_all(&config.output_dir)?;

    let progress = Progress::global();
    progress.set_total_examples(config.count);

    let clone_progress = progress.start(Step::Synthesize, "clone");
    let repo_path = ensure_repo_cloned(&config.repo_url).await?;
    drop(clone_progress);

    let client = PlainLlmClient::new()?;
    let mut examples_generated = 0;
    let mut commits_skipped = 0;
    let batch_size = config.max_commits;

    'outer: loop {
        let list_progress = progress.start(Step::Synthesize, "list-commits");
        let commits = list_commits(&repo_path, batch_size, commits_skipped).await?;
        drop(list_progress);

        if commits.is_empty() {
            break;
        }

        commits_skipped += commits.len();

        for commit in commits {
            if examples_generated >= config.count {
                break 'outer;
            }

            if !config.fresh && state.is_processed(&config.repo_url, &commit.sha) {
                continue;
            }

            if should_skip_commit(&commit) {
                continue;
            }

            let commit_label = format!(
                "{} {}",
                &commit.sha[..8],
                truncate_message(&commit.message, 40)
            );
            let step_progress = Arc::new(progress.start(Step::Synthesize, &commit_label));

            // Turn 1: Identify edit predication example candidates
            step_progress.set_substatus("identifying candidates...");
            let example_candidates = match identify_example_candidates(
                &client,
                &config,
                &commit,
                config.count - examples_generated,
                step_progress.clone(),
            )
            .await
            {
                Ok(candidates) => candidates,
                Err(e) => {
                    step_progress.set_info(format!("error: {:?}", e), InfoStyle::Warning);
                    state.mark_processed(&config.repo_url, &commit.sha, 0);
                    state.save()?;
                    continue;
                }
            };

            if example_candidates.is_empty() {
                step_progress.set_info("no candidates", InfoStyle::Normal);
                state.mark_processed(&config.repo_url, &commit.sha, 0);
                state.save()?;
                continue;
            }

            // Turn 2: Formulate each candidate into a precise example
            let mut valid_examples = Vec::new();
            for (i, pattern) in example_candidates.iter().enumerate() {
                if examples_generated + valid_examples.len() >= config.count {
                    break;
                }

                step_progress.set_substatus(format!(
                    "formulating {}/{}...",
                    i + 1,
                    example_candidates.len()
                ));

                // Collect all unique files (edit history files + cursor file)
                let mut all_files: Vec<String> = pattern.edit_history_files.clone();
                if !all_files.contains(&pattern.cursor_file) {
                    all_files.push(pattern.cursor_file.clone());
                }

                // Fetch file contents for all files uniformly
                let mut file_contexts = Vec::new();
                let mut skip_pattern = false;
                for file_path in &all_files {
                    match get_file_context(&repo_path, &commit.sha, file_path).await {
                        Ok(ctx) => {
                            let file_diff = extract_file_diff(&commit.diff, file_path).ok();
                            file_contexts.push((file_path.clone(), ctx, file_diff));
                        }
                        Err(e) => {
                            log::warn!("Failed to get file context for {}: {:?}", file_path, e);
                            skip_pattern = true;
                            break;
                        }
                    }
                }
                if skip_pattern {
                    continue;
                }

                let spec = match formulate_example(
                    &client,
                    pattern,
                    &file_contexts,
                    step_progress.clone(),
                )
                .await
                {
                    Ok(Some(spec)) => spec,
                    Ok(None) => {
                        log::debug!("Cannot formulate example");
                        continue;
                    }
                    Err(e) => {
                        log::warn!("Failed to formulate example: {:?}", e);
                        continue;
                    }
                };

                match validate_example(&spec, &file_contexts) {
                    Ok(()) => {
                        valid_examples.push(spec);
                    }
                    Err(rejection_reason) => {
                        log::debug!("Example rejected: {}", rejection_reason);
                        // Write rejected example to failed examples directory
                        let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S%.3f");
                        let filename = format!("{}.md", timestamp);
                        let path = FAILED_EXAMPLES_DIR.join(&filename);
                        let content = format_rejected_example(&spec, &rejection_reason);
                        if let Err(e) = std::fs::write(&path, content) {
                            log::warn!("Failed to write rejected example: {:?}", e);
                        } else {
                            step_progress
                                .set_info(format!("rejected: {}", filename), InfoStyle::Warning);
                        }
                    }
                }
            }

            if valid_examples.is_empty() {
                step_progress.set_info("0 valid", InfoStyle::Normal);
                state.mark_processed(&config.repo_url, &commit.sha, 0);
                state.save()?;
                continue;
            }

            let count = valid_examples.len();
            step_progress.set_info(format!("{} valid", count), InfoStyle::Normal);

            for mut spec in valid_examples.into_iter() {
                if examples_generated >= config.count {
                    break;
                }

                spec.repository_url = config.repo_url.clone();
                spec.revision = commit.parent_sha.clone();

                let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S");
                let path = config.output_dir.join(format!("{}.md", timestamp));
                std::fs::write(&path, spec.to_markdown())?;
                examples_generated += 1;
            }

            state.mark_processed(&config.repo_url, &commit.sha, count);
            state.save()?;
        }
    }

    progress.finalize();
    Ok(())
}

fn truncate_message(msg: &str, max_len: usize) -> String {
    let first_line = msg.lines().next().unwrap_or("");
    if first_line.len() <= max_len {
        first_line.to_string()
    } else {
        format!("{}...", &first_line[..max_len - 3])
    }
}

fn should_skip_commit(commit: &CommitInfo) -> bool {
    let lines_changed = commit
        .diff
        .lines()
        .filter(|l| l.starts_with('+') || l.starts_with('-'))
        .count();
    return lines_changed < 10
        || lines_changed > 1000
        || is_non_code_commit(commit)
        || is_rename_commit(commit);
}

fn is_non_code_commit(commit: &CommitInfo) -> bool {
    let non_code_extensions = [
        ".md", ".txt", ".json", ".yaml", ".yml", ".toml", ".lock", ".svg", ".png", ".jpg", ".gif",
        ".ico", ".woff", ".ttf", ".eot",
    ];

    let diff_files: Vec<&str> = commit
        .diff
        .lines()
        .filter(|l| l.starts_with("+++ b/") || l.starts_with("--- a/"))
        .filter_map(|l| {
            l.strip_prefix("+++ b/")
                .or_else(|| l.strip_prefix("--- a/"))
        })
        .collect();

    if diff_files.is_empty() {
        return false;
    }

    let all_non_code = diff_files
        .iter()
        .all(|f| non_code_extensions.iter().any(|ext| f.ends_with(ext)));

    all_non_code
}

fn is_rename_commit(commit: &CommitInfo) -> bool {
    commit.diff.contains("similarity index")
        || commit.diff.contains("rename from")
        || commit.diff.contains("rename to")
}

async fn list_commits(
    repo_path: &Path,
    max_commits: usize,
    skip: usize,
) -> Result<Vec<CommitInfo>> {
    let output = run_git(
        repo_path,
        &[
            "log",
            "--no-merges",
            &format!("--skip={}", skip),
            &format!("-{}", max_commits),
            "--format=%H|%P|%s",
        ],
    )
    .await?;

    let mut commits = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() < 3 {
            continue;
        }
        let sha = parts[0].to_string();
        let parent_sha = parts[1].split_whitespace().next().unwrap_or("").to_string();
        if parent_sha.is_empty() {
            continue;
        }
        let diff = run_git(repo_path, &["show", "--format=", &sha])
            .await
            .unwrap_or_default();
        commits.push(CommitInfo {
            sha,
            parent_sha,
            message: parts[2].to_string(),
            diff,
        });
    }

    Ok(commits)
}

#[derive(Debug)]
struct FileContext {
    before_content: String,
    after_content: String,
}

async fn get_file_context(
    repo_path: &Path,
    commit_sha: &str,
    file_path: &str,
) -> Result<FileContext> {
    let after_content = run_git(
        repo_path,
        &["show", &format!("{}:{}", commit_sha, file_path)],
    )
    .await
    .unwrap_or_default();

    let before_content = run_git(
        repo_path,
        &["show", &format!("{}^:{}", commit_sha, file_path)],
    )
    .await
    .unwrap_or_default();

    Ok(FileContext {
        before_content,
        after_content,
    })
}

fn build_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "formulate_example".to_string(),
            description: "Request to formulate a precise edit prediction example. Call this when you've identified a predictable pattern in the commit.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "A short, commit-message-like description of the edit pattern (e.g., 'add new field to struct and update constructor', 'rename method and update call sites'). Should be lowercase, concise (under 60 chars), and describe the pattern."
                    },
                    "cursor_file": {
                        "type": "string",
                        "description": "Path to the file where the cursor will be positioned and where the expected patch will be applied"
                    },
                    "edit_history_files": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of file paths whose changes form the edit history (the pattern to follow). Can include the cursor file and/or other files. Changes from these files establish the pattern that informs the prediction."
                    },
                    "edit_history_description": {
                        "type": "string",
                        "description": "Description of which changes in the diff establish the pattern (the changes that come BEFORE the predicted edit). Reference specific files, hunks, line numbers, or code snippets."
                    },
                    "expected_patch_description": {
                        "type": "string",
                        "description": "Description of which change should be predicted (the change that logically follows from the edit history). This should be a small change (1-10 lines) in the cursor_file."
                    },
                    "cursor_location_hint": {
                        "type": "string",
                        "description": "Description of where the cursor should be positioned in cursor_file - this is the location where the expected patch will be applied, described relative to surrounding code."
                    },
                    "reasoning": {
                        "type": "string",
                        "description": "2-4 sentences explaining why this is a predictable pattern. What establishes the pattern? Why would the expected patch logically follow?"
                    },
                    "requires_context": {
                        "type": "boolean",
                        "description": "Whether this prediction requires information from outside the immediate edit history (e.g., type definitions, function signatures from elsewhere in the codebase)"
                    }
                },
                "required": ["name", "cursor_file", "edit_history_files", "edit_history_description", "expected_patch_description", "cursor_location_hint", "reasoning"]
            }),
        },
        Tool {
            name: "no_predictable_pattern".to_string(),
            description: "Indicate that no good predictable edit pattern was found in this commit.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "reason": {
                        "type": "string",
                        "description": "Brief explanation of why no good predictable pattern exists in this commit"
                    }
                },
                "required": ["reason"]
            }),
        },
    ]
}

fn build_identification_prompt(
    config: &SynthesizeConfig,
    commit: &CommitInfo,
    max_patterns: usize,
) -> String {
    let context_guidance = if config.require_context {
        indoc! {r#"
            IMPORTANT: Only identify patterns where a correct prediction REQUIRES information from outside the immediate edit history.

            Look specifically for patterns where:
            - A type/struct definition was changed, and usages need updating based on the NEW field names or types
            - A function signature was modified, and call sites need updating based on the NEW parameters
            - A new enum variant, struct field, or method was added, and other code needs to reference it
            - An API pattern changed, and the prediction needs to know the new pattern from a definition

            Do NOT identify patterns where:
            - The pattern is purely mechanical (find/replace style)
            - The edit history alone contains enough information to infer the change
            - The prediction could be made just by looking at the immediate cursor context
        "#}
    } else {
        ""
    };

    format!(
        indoc! {r#"
            You are analyzing a git commit to find "predictable edit" patterns for an edit prediction evaluation dataset.

            A predictable edit is one where:
            - A developer makes code changes in one or more locations
            - Similar or related code changes need to be made elsewhere
            - The pattern is clear enough that a model could predict the remaining changes after seeing the initial ones
            - The expected patch is small

            GOOD examples of predictable edits:
            - Adding a new parameter to a function, then updating call sites to pass the new argument
            - Adding a feature flag check in one place, then adding it to similar places
            - Changing an API pattern (e.g., sync to async), then updating other usages
            - Adding error handling to one function call, then adding similar handling to related calls
            - Modifying a struct/type definition in one file, then updating usages in another file
            - Adding a new method to a trait/interface, then implementing it in another file

            BAD examples (DO NOT identify these):
            - File renames and updating import paths
            - Simple find-and-replace style changes
            - Changes that only involve string literals, comments, or configuration

            {context_guidance}

            ## Commit Information

            Repository: {repo_url}
            Commit: {sha}
            Message: {message}

            ## Diff

            ```diff
            {diff}
            ```

            ## Your Task

            Analyze this commit and identify up to {max_patterns} predictable edit pattern(s).

            For each pattern you find, call the `formulate_example` tool with details about the pattern.
            If no good patterns exist in this commit, call `no_predictable_pattern` with a brief reason.

            Remember:
            - Focus on code changes, not paths/comments/strings
            - The edit history establishes a pattern; the expected patch follows that pattern
            - Edit history can span multiple files (e.g., a type definition change in one file informs updates in another)
            - The cursor_file is where the prediction happens; edit_history_files are where the pattern is established
        "#},
        context_guidance = context_guidance,
        repo_url = config.repo_url,
        sha = commit.sha,
        message = commit.message,
        diff = commit.diff,
        max_patterns = max_patterns,
    )
}

async fn identify_example_candidates(
    client: &PlainLlmClient,
    config: &SynthesizeConfig,
    commit: &CommitInfo,
    max_examples: usize,
    step_progress: Arc<StepProgress>,
) -> Result<Vec<ExampleCandidate>> {
    let prompt = build_identification_prompt(config, commit, max_examples);
    let tools = build_tools();
    let messages = vec![Message {
        role: Role::User,
        content: vec![RequestContent::Text {
            text: prompt,
            cache_control: None,
        }],
    }];

    let response = client
        .generate_with_tools(
            "claude-sonnet-4-20250514",
            4096,
            messages,
            tools,
            Some(ToolChoice::Any),
            |bytes, _text| {
                step_progress.set_substatus(format!("identifying: {:.1}kb", bytes as f64 / 1000.0));
            },
        )
        .await?;

    let mut patterns = Vec::new();
    for content in &response.content {
        if let ResponseContent::ToolUse { name, input, .. } = content {
            if name == "formulate_example" {
                if let Ok(candidate) = serde_json::from_value::<ExampleCandidate>(input.clone()) {
                    patterns.push(candidate);
                }
            } else if name == "no_predictable_pattern" {
                return Ok(Vec::new());
            }
        }
    }

    Ok(patterns)
}

fn build_formulation_prompt(
    pattern: &ExampleCandidate,
    file_contexts: &[(String, FileContext, Option<String>)],
) -> String {
    // Build file contexts section
    let mut file_contexts_section = String::new();
    for (file_path, context, diff) in file_contexts {
        let is_cursor_file = file_path == &pattern.cursor_file;
        let is_edit_history_file = pattern.edit_history_files.contains(file_path);

        let mut annotations = Vec::new();
        if is_cursor_file {
            annotations.push("CURSOR FILE");
        }
        if is_edit_history_file {
            annotations.push("EDIT HISTORY");
        }
        let annotation_str = if annotations.is_empty() {
            String::new()
        } else {
            format!(" ({})", annotations.join(", "))
        };

        file_contexts_section.push_str(&format!(
            "### {}{}\n\n**Before:**\n```\n{}\n```\n\n**After:**\n```\n{}\n```\n\n",
            file_path, annotation_str, context.before_content, context.after_content
        ));
        if let Some(d) = diff {
            file_contexts_section.push_str(&format!("**Diff:**\n```diff\n{}\n```\n\n", d));
        }
    }

    // Get first edit history file for the output format example
    let first_edit_file: &str = pattern
        .edit_history_files
        .first()
        .map(|s| s.as_str())
        .unwrap_or(&pattern.cursor_file);

    format!(
        indoc! {r#"
        You are formulating a precise edit prediction example based on a pattern that was identified in a commit.

        ## Pattern Description

        **Cursor File (where prediction happens):** {cursor_file}

        **Edit History Files:** {edit_history_files:?}

        **Edit History (changes that establish the pattern):**
        {edit_history_description}

        **Expected Patch (change to be predicted):**
        {expected_patch_description}

        **Cursor Location:**
        {cursor_location_hint}

        **Reasoning:**
        {reasoning}

        **Requires Context:** {requires_context}

        ## File Contents

        Files are annotated with their role:
        - **CURSOR FILE**: Where the cursor is positioned and where the expected patch will be applied
        - **EDIT HISTORY**: Files whose changes establish the pattern

        {file_contexts_section}

        ## Your Task

        Formulate the precise example in the following format. Be exact with the diff syntax and cursor positioning.

        CRITICAL RULES:
        1. The edit_history can include diffs from multiple files. Include ALL relevant changes that establish the pattern.
        2. The CURSOR_POSITION excerpt must show the cursor file state AFTER the edit_history changes have been applied, but BEFORE the expected_patch is applied.
        3. The cursor marker should point to code that WILL BE CHANGED by the expected_patch.
        4. The expected_patch must be a valid unified diff that applies to the cursor file only.
        5. Place the cursor marker <|user_cursor|> INLINE at the exact position where the cursor should be. For example: `func(<|user_cursor|>)` or `let x = <|user_cursor|>value`.

        Output your response in this EXACT format:

        EDIT_HISTORY:
        ```diff
        --- a/{first_edit_file}
        +++ b/{first_edit_file}
        @@ -<line>,<count> +<line>,<count> @@
         <context line>
        -<removed line>
        +<added line>
         <context line>
        ```
        (Include multiple file diffs if the edit history spans multiple files)

        CURSOR_POSITION:
        ```
        <5-15 lines of code from {cursor_file} showing the state AFTER edit_history is applied, with the cursor marker>
        ```

        EXPECTED_PATCH:
        ```diff
        --- a/{cursor_file}
        +++ b/{cursor_file}
        @@ -<line>,<count> +<line>,<count> @@
         <context line>
        -<removed line>
        +<added line>
         <context line>
        ```

        If you cannot formulate a valid example (e.g., the pattern doesn't work as described), respond with:
        CANNOT_FORMULATE: <reason>
    "#},
        cursor_file = pattern.cursor_file,
        edit_history_files = pattern.edit_history_files,
        edit_history_description = pattern.edit_history_description,
        expected_patch_description = pattern.expected_patch_description,
        cursor_location_hint = pattern.cursor_location_hint,
        reasoning = pattern.reasoning,
        requires_context = pattern.requires_context,
        file_contexts_section = file_contexts_section,
        first_edit_file = first_edit_file,
    )
}

fn format_rejected_example(spec: &ExampleSpec, rejection_reason: &str) -> String {
    let mut content = spec.to_markdown();
    content.push_str("\n## Rejection Reason\n\n");
    content.push_str(rejection_reason);
    content.push_str("\n");
    content
}

fn validate_example(
    spec: &ExampleSpec,
    file_contexts: &[(String, FileContext, Option<String>)],
) -> Result<(), String> {
    // Check for cursor marker
    let (cursor_excerpt, _cursor_offset) = spec.cursor_excerpt().map_err(|e| {
        format!(
            "Cursor position missing cursor marker (<|user_cursor|> or [CURSOR_POSITION]): {}",
            e
        )
    })?;

    let cursor_path_str = spec.cursor_path.to_string_lossy();

    let mut cursor_file_intermediate_state: Option<String> = None;

    for (file_path, file_context, _) in file_contexts {
        let file_edit_history = extract_file_diff(&spec.edit_history, file_path).ok();

        // Compute intermediate state for this file
        let intermediate_state = match &file_edit_history {
            None => file_context.before_content.clone(),
            Some(diff) => match apply_diff_to_string(&file_context.before_content, diff) {
                Ok(state) => state,
                Err(e) => {
                    return Err(format!(
                        "Edit history failed to apply to {}: {}",
                        file_path, e
                    ));
                }
            },
        };

        // Save cursor file's intermediate state for later validation
        if file_path == cursor_path_str.as_ref() {
            cursor_file_intermediate_state = Some(intermediate_state);
        }
    }

    let cursor_intermediate_state = cursor_file_intermediate_state
        .ok_or_else(|| format!("Cursor file {} not found in file contexts", cursor_path_str))?;

    // Validate that cursor position text actually exists in the intermediate state
    if !cursor_intermediate_state.contains(&cursor_excerpt) {
        return Err(format!(
            "Cursor position text not found in intermediate state (after applying edit_history). Looking for:\n{}\n",
            cursor_excerpt
        ));
    }

    // Validate that expected_patch applies to the intermediate state
    for (i, patch) in spec.expected_patches.iter().enumerate() {
        if let Err(e) = apply_diff_to_string(&cursor_intermediate_state, patch) {
            return Err(format!(
                "Expected patch {} failed to apply to intermediate state: {}",
                i + 1,
                e
            ));
        }
    }

    Ok(())
}

async fn formulate_example(
    client: &PlainLlmClient,
    pattern: &ExampleCandidate,
    file_contexts: &[(String, FileContext, Option<String>)],
    step_progress: Arc<crate::progress::StepProgress>,
) -> Result<Option<ExampleSpec>> {
    let prompt = build_formulation_prompt(pattern, file_contexts);

    let messages = vec![Message {
        role: Role::User,
        content: vec![RequestContent::Text {
            text: prompt,
            cache_control: None,
        }],
    }];

    let response = client
        .generate_streaming(
            "claude-sonnet-4-20250514",
            8192,
            messages,
            |chars, _text| {
                step_progress.set_substatus(format!("formulating: {:.1}K", chars as f64 / 1000.0));
            },
        )
        .await?;

    let response_text = response
        .content
        .iter()
        .filter_map(|block| {
            if let ResponseContent::Text { text } = block {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if response_text.contains("CANNOT_FORMULATE:") {
        return Ok(None);
    }

    parse_formulated_example(&response_text, pattern)
}

fn parse_formulated_example(
    response: &str,
    pattern: &ExampleCandidate,
) -> Result<Option<ExampleSpec>> {
    let mut edit_history = String::new();
    let mut cursor_excerpt = String::new();
    let mut expected_patch = String::new();

    #[derive(PartialEq)]
    enum Section {
        None,
        EditHistory,
        CursorPosition,
        ExpectedPatch,
    }

    let mut current_section = Section::None;
    let mut in_code_block = false;
    let mut current_block = String::new();

    for line in response.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("EDIT_HISTORY:") {
            current_section = Section::EditHistory;
            continue;
        } else if trimmed.starts_with("CURSOR_POSITION:") {
            current_section = Section::CursorPosition;
            continue;
        } else if trimmed.starts_with("EXPECTED_PATCH:") {
            current_section = Section::ExpectedPatch;
            continue;
        }

        if trimmed.starts_with("```") {
            if in_code_block {
                in_code_block = false;
                match current_section {
                    Section::EditHistory => {
                        edit_history = current_block.trim().to_string();
                    }
                    Section::CursorPosition => {
                        cursor_excerpt = current_block.trim().to_string();
                    }
                    Section::ExpectedPatch => {
                        expected_patch = current_block.trim().to_string();
                    }
                    Section::None => {}
                }
                current_block.clear();
            } else {
                in_code_block = true;
                current_block.clear();
            }
            continue;
        }

        if in_code_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }

    if cursor_excerpt.is_empty() || expected_patch.is_empty() {
        return Ok(None);
    }

    let edit_history = strip_diff_metadata(&edit_history);
    let expected_patch = strip_diff_metadata(&expected_patch);

    // Extract cursor offset from inline marker and format with set_cursor_excerpt
    let cursor_offset = cursor_excerpt
        .find(INLINE_CURSOR_MARKER)
        .context("missing cursor marker")?;
    cursor_excerpt.replace_range(
        cursor_offset..cursor_offset + INLINE_CURSOR_MARKER.len(),
        "",
    );

    let mut tags = Vec::new();
    if pattern.requires_context {
        tags.push("requires-context".to_string());
    }

    let comment_prefix = line_comment_prefix(&pattern.cursor_file);

    let mut spec = ExampleSpec {
        name: pattern.name.clone(),
        repository_url: String::new(),
        revision: String::new(),
        tags,
        reasoning: Some(pattern.reasoning.clone()),
        uncommitted_diff: String::new(),
        cursor_path: Arc::from(Path::new(&pattern.cursor_file)),
        cursor_position: String::new(),
        edit_history,
        expected_patches: vec![expected_patch],
    };
    spec.set_cursor_excerpt(&cursor_excerpt, cursor_offset, comment_prefix);
    Ok(Some(spec))
}

fn line_comment_prefix(file_path: &str) -> &'static str {
    let extension = file_path.rsplit('.').next().unwrap_or("");
    match extension {
        "rs" | "c" | "cpp" | "cc" | "h" | "hpp" | "js" | "ts" | "tsx" | "jsx" | "go" | "java"
        | "swift" | "kt" | "kts" | "scala" | "cs" | "m" | "mm" | "zig" | "v" | "d" => "//",
        "py" | "rb" | "sh" | "bash" | "zsh" | "pl" | "pm" | "r" | "jl" | "yaml" | "yml"
        | "toml" | "coffee" | "cr" | "ex" | "exs" | "elixir" => "#",
        "lua" | "hs" | "sql" => "--",
        "lisp" | "clj" | "cljs" | "scm" | "rkt" | "el" => ";",
        "erl" | "hrl" => "%",
        _ => "//",
    }
}
