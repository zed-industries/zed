use crate::{
    anthropic_client::PlainLlmClient,
    git::{ensure_repo_cloned, run_git},
    paths::{FAILED_EXAMPLES_DIR, LATEST_FAILED_EXAMPLES_DIR, SYNTHESIZE_STATE_FILE},
    progress::{InfoStyle, Progress, Step, StepProgress},
};
use anthropic::ResponseContent;
use anyhow::{Context as _, Result};
use chrono::Local;
use collections::{HashMap, HashSet};
use edit_prediction::{
    example_spec::ExampleSpec,
    udiff::{apply_diff_to_string, edits_for_diff},
};
use futures::stream::{FuturesUnordered, StreamExt};
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct SynthesizeConfig {
    pub repo_urls: Vec<String>,
    /// Number of examples to generate per repository
    pub count: usize,
    pub max_commits: usize,
    pub output_dir: PathBuf,
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

    fn take_repo_state(&mut self, repo_url: &str) -> RepoState {
        self.repositories.remove(repo_url).unwrap_or_default()
    }

    fn merge_repo_state(&mut self, repo_url: String, repo_state: RepoState) {
        self.repositories.insert(repo_url, repo_state);
    }
}

impl RepoState {
    fn is_processed(&self, commit_sha: &str) -> bool {
        self.processed_commits.contains(commit_sha)
    }

    fn mark_processed(&mut self, commit_sha: &str, examples_count: usize) {
        self.processed_commits.insert(commit_sha.to_string());
        self.examples_generated += examples_count;
    }
}

#[derive(Debug)]
struct CommitInfo {
    sha: String,
    parent_sha: String,
    message: String,
    diff: String,
    expanded_diff: String,
}

/// Claude's response parsed into structured form
#[derive(Debug)]
struct ClaudeResponse {
    name: String,
    reasoning: String,
    edit_history_hunks: Vec<String>,
    expected_patch_hunks: Vec<String>,
}

pub async fn run_synthesize(config: SynthesizeConfig) -> Result<()> {
    let mut state = if config.fresh {
        SynthesizeState::default()
    } else {
        SynthesizeState::load()
    };

    std::fs::create_dir_all(&config.output_dir)?;
    std::fs::create_dir_all(&*FAILED_EXAMPLES_DIR)?;

    // Create "latest_failed" symlink pointing to this run's failed directory
    if LATEST_FAILED_EXAMPLES_DIR.is_symlink() {
        std::fs::remove_file(&*LATEST_FAILED_EXAMPLES_DIR)?;
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(&*FAILED_EXAMPLES_DIR, &*LATEST_FAILED_EXAMPLES_DIR)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&*FAILED_EXAMPLES_DIR, &*LATEST_FAILED_EXAMPLES_DIR)?;

    let progress = Progress::global();
    let total_examples = config.count * config.repo_urls.len();
    progress.set_total_examples(total_examples);

    let client = Arc::new(PlainLlmClient::new()?);
    let config = Arc::new(config);

    let mut futures: FuturesUnordered<_> = config
        .repo_urls
        .iter()
        .map(|repo_url| {
            let client = client.clone();
            let repo_state = state.take_repo_state(repo_url);
            let config = config.clone();
            let repo_url = repo_url.clone();
            async move {
                let result = synthesize_repo(&client, repo_state, &config, &repo_url).await;
                (repo_url, result)
            }
        })
        .collect();

    let mut errors = Vec::new();
    while let Some((repo_url, result)) = futures.next().await {
        match result {
            Ok(repo_state) => {
                state.merge_repo_state(repo_url, repo_state);
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }

    state.save()?;

    progress.finalize();

    if let Some(first_error) = errors.into_iter().next() {
        return Err(first_error);
    }

    Ok(())
}

async fn synthesize_repo(
    client: &PlainLlmClient,
    mut repo_state: RepoState,
    config: &SynthesizeConfig,
    repo_url: &str,
) -> Result<RepoState> {
    let progress = Progress::global();
    let batch_size = config.max_commits;

    let clone_progress = progress.start(Step::Synthesize, &format!("clone {}", repo_url));
    let repo_path = ensure_repo_cloned(repo_url).await?;
    drop(clone_progress);

    let mut examples_generated = 0;
    let mut commits_skipped = 0;

    'outer: loop {
        let list_progress = progress.start(
            Step::Synthesize,
            &format!("{}: list-commits", repo_name_from_url(repo_url)),
        );
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

            if !config.fresh && repo_state.is_processed(&commit.sha) {
                continue;
            }

            if should_skip_commit(&commit) {
                continue;
            }

            let repo_name = repo_name_from_url(repo_url);
            let commit_label = format!(
                "{}: {} {}",
                repo_name,
                &commit.sha[..8],
                truncate_message(&commit.message, 40)
            );
            let step_progress = Arc::new(progress.start(Step::Synthesize, &commit_label));

            // Single Claude call to identify and copy hunks
            step_progress.set_substatus("analyzing...");
            let claude_response =
                match analyze_commit(client, repo_url, &commit, step_progress.clone()).await {
                    Ok(Some(response)) => response,
                    Ok(None) => {
                        step_progress.set_info("no pattern", InfoStyle::Normal);
                        repo_state.mark_processed(&commit.sha, 0);
                        continue;
                    }
                    Err(e) => {
                        step_progress.set_info(format!("error: {:?}", e), InfoStyle::Warning);
                        repo_state.mark_processed(&commit.sha, 0);
                        continue;
                    }
                };

            // Validate and build the example
            step_progress.set_substatus("validating...");
            match build_example(repo_url, &commit, &repo_path, &claude_response).await {
                Ok(spec) => {
                    let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S");
                    let filename = format!("{}--{}.md", repo_name, timestamp);
                    let path = config.output_dir.join(&filename);
                    std::fs::write(&path, spec.to_markdown())?;
                    examples_generated += 1;
                    step_progress.set_info(filename, InfoStyle::Normal);
                }
                Err(rejection_reason) => {
                    log::debug!("Example rejected: {}", rejection_reason);
                    let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S%.3f");
                    let filename = format!("{}--{}.md", repo_name, timestamp);
                    let path = FAILED_EXAMPLES_DIR.join(&filename);
                    let content = format_rejected_example(&claude_response, &rejection_reason);
                    if let Err(e) = std::fs::write(&path, content) {
                        log::warn!("Failed to write rejected example: {:?}", e);
                    }
                    step_progress.set_info(format!("rejected: {}", filename), InfoStyle::Warning);
                }
            }

            repo_state.mark_processed(&commit.sha, 1);
        }
    }

    Ok(repo_state)
}

fn repo_name_from_url(url: &str) -> String {
    url.rsplit('/')
        .next()
        .unwrap_or(url)
        .trim_end_matches(".git")
        .to_string()
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
    lines_changed < 10
        || lines_changed > 1000
        || is_non_code_commit(commit)
        || is_rename_commit(commit)
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

    diff_files
        .iter()
        .all(|f| non_code_extensions.iter().any(|ext| f.ends_with(ext)))
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

        // Get standard diff (for skip checks)
        let diff = run_git(repo_path, &["show", "--format=", &sha])
            .await
            .unwrap_or_default();

        // Get expanded diff with 30 lines of context
        let expanded_diff = run_git(repo_path, &["show", "-U30", "--format=", &sha])
            .await
            .unwrap_or_default();

        commits.push(CommitInfo {
            sha,
            parent_sha,
            message: parts[2].to_string(),
            diff,
            expanded_diff,
        });
    }

    Ok(commits)
}

fn build_prompt(repo_url: &str, commit: &CommitInfo) -> String {
    format!(
        indoc! {r#"
            You are analyzing a git commit to construct a realistic edit prediction example.

            Your goal is to tell the story of a programmer's editing session: what sequence of changes did they make, and what change logically comes next? We use these examples to train a model to predict edits, so the quality of the EDIT HISTORY is what matters most.

            An edit prediction example consists of:
            1. **Edit History**: 3-6 hunks showing what the programmer did BEFORE making the expected patch. This is the most important part - it must tell a coherent story of the changes leading up to the prediction.
            2. **Expected Patch**: One small hunk that logically follows from the edit history.

            Both single-file and multi-file patterns are acceptable.

            ## What Makes a Good Example

            The edit history should read like a story: "First the programmer changed X, then Y, then Z, and now they need to change W."

            GOOD examples (rich sequences with 3+ steps):
            - Removing a parameter: docstring update → constructor change → field removal → (predict) usage site update
            - Adding a feature: type definition → first usage → second usage → (predict) third usage
            - Bug fix pattern: fix in file A → fix in file B → fix in file C → (predict) fix in file D

            BAD examples (respond NO_PATTERN):
            - Commits where all changes are independent (no narrative thread)
            - Simple find-and-replace (renaming, version bumps)
            - Documentation-only or config-only changes
            - Changes where you can only find 1-2 hunks for the edit history

            ## Commit Information

            Repository: {repo_url}
            Commit: {sha}
            Message: {message}

            ## Diff (30 lines context)

            ```diff
            {expanded_diff}
            ```

            ## Your Task

            First, THINK through whether this commit can support a good example:

            1. What is the high-level pattern in this commit?
            2. Can you identify at least 4 related hunks (3 for edit history + 1 for expected patch)?
            3. What would be the narrative? (First... then... then... finally predict...)
            4. Which specific hunk should be the expected patch (the "punchline")?

            If you cannot construct a coherent 3+ hunk story, respond with just:
            NO_PATTERN: <brief reason>

            If you CAN construct a good example, respond in this format:

            ANALYSIS:
            Pattern: <one sentence describing the pattern>
            Steps:
            1. <file:line-range> - <what this hunk does>
            2. <file:line-range> - <what this hunk does>
            3. <file:line-range> - <what this hunk does>
            4. [EXPECTED PATCH] <file:line-range> - <what this hunk does>

            NAME: <short description, like a commit message, under 60 chars>

            EDIT_HISTORY:

            Hunk 1:
            ```diff
            --- a/src/models/user.py
            +++ b/src/models/user.py
            @@ -15,7 +15,6 @@ class User:
                 """A user in the system.

                 Attributes:
            -        email: The user's email address.
                     name: The user's display name.
                 """
            ```

            Hunk 2:
            ```diff
            --- a/src/models/user.py
            +++ b/src/models/user.py
            @@ -25,10 +24,9 @@ class User:
                 def __init__(
                     self,
                     name: str,
            -        email: str,
                     created_at: datetime,
                 ):
                     self.name = name
            -        self.email = email
                     self.created_at = created_at
            ```

            Hunk 3:
            ```diff
            --- a/src/api/handlers.py
            +++ b/src/api/handlers.py
            @@ -42,7 +42,6 @@ def create_user(request):
                 data = request.json()
                 user = User(
                     name=data["name"],
            -        email=data["email"],
                     created_at=datetime.now(),
                 )
                 return user.save()
            ```

            EXPECTED_PATCH:
            ```diff
            --- a/src/api/handlers.py
            +++ b/src/api/handlers.py
            @@ -58,7 +57,6 @@ def update_user(request, user_id):
                 user = User.get(user_id)
                 user.name = data.get("name", user.name)
            -    user.email = data.get("email", user.email)
                 user.save()
                 return user
            ```

            ## Requirements for the diffs

            Edit history:
            - MUST have 3-6 hunks (if you cannot find 3+, respond NO_PATTERN instead)
            - Each hunk needs file headers (--- a/path and +++ b/path)
            - Hunks must be valid unified diffs that apply to the parent commit
            - Order hunks as a programmer would naturally make the changes

            Expected patch:
            - Must be a SINGLE hunk from a SINGLE file
            - Must be SMALL: 1-15 changed lines (not counting context)
            - Must be clearly predictable from the edit history narrative
        "#},
        repo_url = repo_url,
        sha = commit.sha,
        message = commit.message,
        expanded_diff = commit.expanded_diff,
    )
}

async fn analyze_commit(
    client: &PlainLlmClient,
    repo_url: &str,
    commit: &CommitInfo,
    step_progress: Arc<StepProgress>,
) -> Result<Option<ClaudeResponse>> {
    use anthropic::{Message, RequestContent, Role};

    let prompt = build_prompt(repo_url, commit);
    let messages = vec![Message {
        role: Role::User,
        content: vec![RequestContent::Text {
            text: prompt,
            cache_control: None,
        }],
    }];

    let response = client
        .generate_streaming("claude-sonnet-4-5", 8192, messages, |chars, _text| {
            step_progress.set_substatus(format!("analyzing: {:.1}K", chars as f64 / 1000.0));
        })
        .await?;

    // Extract text content from response
    let response_text: String = response
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

    parse_claude_response(&response_text)
}

fn parse_claude_response(response: &str) -> Result<Option<ClaudeResponse>> {
    // Check for NO_PATTERN
    if response.contains("NO_PATTERN:") {
        return Ok(None);
    }

    // Parse NAME
    let name = response
        .lines()
        .find(|l| l.starts_with("NAME:"))
        .map(|l| l.strip_prefix("NAME:").unwrap_or("").trim().to_string())
        .unwrap_or_else(|| "unnamed example".to_string());

    // Parse ANALYSIS section (Claude's planning) - this is the primary reasoning
    let reasoning = extract_section(
        response,
        "ANALYSIS:",
        &["NAME:", "REASONING:", "EDIT_HISTORY:", "EXPECTED_PATCH:"],
    )
    .unwrap_or_default();

    // Parse EDIT_HISTORY diff block
    let edit_history_hunks = extract_diff_block(response, "EDIT_HISTORY:")?;

    // Parse EXPECTED_PATCH diff block
    let expected_patch_hunks = extract_diff_block(response, "EXPECTED_PATCH:")?;

    if edit_history_hunks.is_empty() {
        anyhow::bail!("No edit history hunks found in response");
    }
    if expected_patch_hunks.is_empty() {
        anyhow::bail!("No expected patch hunks found in response");
    }

    Ok(Some(ClaudeResponse {
        name,
        reasoning,
        edit_history_hunks,
        expected_patch_hunks,
    }))
}

fn extract_section(text: &str, start_marker: &str, end_markers: &[&str]) -> Option<String> {
    let start_idx = text.find(start_marker)?;
    let content_start = start_idx + start_marker.len();

    let end_idx = end_markers
        .iter()
        .filter_map(|marker| text[content_start..].find(marker))
        .min()
        .map(|idx| content_start + idx)
        .unwrap_or(text.len());

    Some(text[content_start..end_idx].trim().to_string())
}

fn extract_diff_block(text: &str, section_marker: &str) -> Result<Vec<String>> {
    let section_start = text
        .find(section_marker)
        .context(format!("Section {} not found", section_marker))?;

    let after_marker = &text[section_start + section_marker.len()..];

    // Find where the next major section starts (to bound our search)
    let section_end = ["EXPECTED_PATCH:", "## "]
        .iter()
        .filter(|&&m| m != section_marker)
        .filter_map(|marker| after_marker.find(marker))
        .min()
        .unwrap_or(after_marker.len());

    let section_content = &after_marker[..section_end];

    // Collect all ```diff blocks in this section
    let mut hunks = Vec::new();
    let mut search_start = 0;

    while let Some(diff_start) = section_content[search_start..].find("```diff") {
        let abs_diff_start = search_start + diff_start;
        let block_content_start = section_content[abs_diff_start..]
            .find('\n')
            .map(|i| abs_diff_start + i + 1)
            .unwrap_or(abs_diff_start);

        if let Some(block_end_rel) = section_content[block_content_start..].find("```") {
            let block_end = block_content_start + block_end_rel;
            let diff_content = section_content[block_content_start..block_end].trim();

            // Split this block into hunks (in case multiple hunks in one block)
            hunks.extend(split_into_hunks(diff_content));

            search_start = block_end + 3;
        } else {
            break;
        }
    }

    if hunks.is_empty() {
        anyhow::bail!("No diff blocks found in section {}", section_marker);
    }

    Ok(hunks)
}

/// Split a diff block into individual hunks, preserving file headers
fn split_into_hunks(diff: &str) -> Vec<String> {
    let mut hunks = Vec::new();
    let mut current_file_header: Option<String> = None;
    let mut current_hunk: Vec<String> = Vec::new();
    let mut in_hunk = false;

    for line in diff.lines() {
        if line.starts_with("--- a/") || line.starts_with("--- /") {
            // Start of file header - flush previous hunk
            if in_hunk && !current_hunk.is_empty() {
                let mut hunk_text = String::new();
                if let Some(ref header) = current_file_header {
                    hunk_text.push_str(header);
                    hunk_text.push('\n');
                }
                hunk_text.push_str(&current_hunk.join("\n"));
                hunks.push(hunk_text);
                current_hunk.clear();
            }
            current_file_header = Some(line.to_string());
            in_hunk = false;
        } else if line.starts_with("+++ b/") || line.starts_with("+++ /") {
            if let Some(ref mut header) = current_file_header {
                header.push('\n');
                header.push_str(line);
            }
        } else if line.starts_with("@@ ") {
            // New hunk - flush previous
            if in_hunk && !current_hunk.is_empty() {
                let mut hunk_text = String::new();
                if let Some(ref header) = current_file_header {
                    hunk_text.push_str(header);
                    hunk_text.push('\n');
                }
                hunk_text.push_str(&current_hunk.join("\n"));
                hunks.push(hunk_text);
                current_hunk.clear();
            }
            current_hunk.push(line.to_string());
            in_hunk = true;
        } else if in_hunk {
            current_hunk.push(line.to_string());
        }
    }

    // Flush final hunk
    if !current_hunk.is_empty() {
        let mut hunk_text = String::new();
        if let Some(ref header) = current_file_header {
            hunk_text.push_str(header);
            hunk_text.push('\n');
        }
        hunk_text.push_str(&current_hunk.join("\n"));
        hunks.push(hunk_text);
    }

    hunks
}

/// Validate Claude's output by applying diffs and build the ExampleSpec
async fn build_example(
    repo_url: &str,
    commit: &CommitInfo,
    repo_path: &Path,
    response: &ClaudeResponse,
) -> Result<ExampleSpec, String> {
    // Validate expected patch hunks
    if response.expected_patch_hunks.len() != 1 {
        return Err(format!(
            "Expected exactly 1 expected patch hunk, got {}",
            response.expected_patch_hunks.len()
        ));
    }

    // Parse the expected patch to determine cursor file
    let expected_patch = &response.expected_patch_hunks[0];
    let cursor_file = extract_file_from_hunk(expected_patch)
        .ok_or_else(|| "Could not determine file from expected patch".to_string())?;

    // Get the file content before the commit
    let before_content = run_git(
        repo_path,
        &["show", &format!("{}^:{}", commit.sha, cursor_file)],
    )
    .await
    .map_err(|e| format!("Failed to get file content for {}: {}", cursor_file, e))?;

    // Build edit history diff from Claude's hunks
    let edit_history = response.edit_history_hunks.join("\n");

    // Apply edit history to get intermediate state (validates edit history)
    let intermediate_state =
        apply_edit_history_to_content(&before_content, &edit_history, &cursor_file)?;

    // Validate expected patch applies to intermediate state
    let expected_patch_with_header = ensure_diff_header(expected_patch, &cursor_file);
    apply_diff_to_string(&expected_patch_with_header, &intermediate_state)
        .map_err(|e| format!("Expected patch failed to apply: {}", e))?;

    // Find where the expected patch edits would apply in the intermediate state
    let edits = edits_for_diff(&intermediate_state, &expected_patch_with_header)
        .map_err(|e| format!("Failed to parse expected patch: {}", e))?;
    if edits.is_empty() {
        return Err(
            "Could not locate expected patch in file (context not found or ambiguous)".to_string(),
        );
    }

    // Use the start of the first edit for cursor positioning
    let cursor_byte_offset = edits[0].0.start;

    // Extract excerpt around the edit location
    let (excerpt, cursor_offset) = extract_cursor_excerpt(&intermediate_state, cursor_byte_offset)?;

    // Build the ExampleSpec and use set_cursor_excerpt to format with comment marker
    let comment_prefix = line_comment_prefix(&cursor_file);
    let reasoning_with_source = format!(
        "Source commit: {} ({})\n\n{}",
        commit.sha,
        truncate_message(&commit.message, 60),
        response.reasoning
    );
    let mut spec = ExampleSpec {
        name: response.name.clone(),
        repository_url: repo_url.to_string(),
        revision: commit.parent_sha.clone(),
        tags: Vec::new(),
        reasoning: Some(reasoning_with_source),
        uncommitted_diff: String::new(),
        cursor_path: Arc::from(Path::new(&cursor_file)),
        cursor_position: String::new(),
        edit_history,
        expected_patches: vec![expected_patch_with_header],
        rejected_patch: None,
        captured_prompt_input: None,
        telemetry: None,
    };
    spec.set_cursor_excerpt(&excerpt, cursor_offset, comment_prefix);

    Ok(spec)
}

/// Extract file path from a hunk (looks for --- a/path or +++ b/path)
fn extract_file_from_hunk(hunk: &str) -> Option<String> {
    for line in hunk.lines() {
        if let Some(path) = line.strip_prefix("+++ b/") {
            return Some(path.to_string());
        }
        if let Some(path) = line.strip_prefix("--- a/") {
            return Some(path.to_string());
        }
    }
    None
}

/// Ensure a hunk has proper file headers
fn ensure_diff_header(hunk: &str, file_path: &str) -> String {
    if hunk.contains("--- a/") || hunk.contains("+++ b/") {
        return hunk.to_string();
    }
    format!("--- a/{}\n+++ b/{}\n{}", file_path, file_path, hunk)
}

/// Apply edit history to file content, only if hunks affect this file
fn apply_edit_history_to_content(
    content: &str,
    edit_history: &str,
    cursor_file: &str,
) -> Result<String, String> {
    // Extract just the hunks for this file from the edit history
    let file_diff = extract_file_diff_from_combined(edit_history, cursor_file);

    if file_diff.is_empty() {
        return Ok(content.to_string());
    }

    apply_diff_to_string(&file_diff, content)
        .map_err(|e| format!("Failed to apply edit history: {}", e))
}

/// Extract hunks for a specific file from a combined diff
fn extract_file_diff_from_combined(combined_diff: &str, target_file: &str) -> String {
    let mut result = String::new();
    let mut in_target_file = false;
    let mut found_header = false;

    for line in combined_diff.lines() {
        if line.starts_with("--- a/") {
            let file = line.strip_prefix("--- a/").unwrap_or("");
            in_target_file = file == target_file;
            if in_target_file {
                result.push_str(line);
                result.push('\n');
                found_header = false;
            }
        } else if line.starts_with("+++ b/") && in_target_file {
            result.push_str(line);
            result.push('\n');
            found_header = true;
        } else if in_target_file && found_header {
            if line.starts_with("--- a/") {
                break;
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Extract a cursor position excerpt from content around a byte offset.
/// Returns the excerpt and the cursor offset within the excerpt.
fn extract_cursor_excerpt(
    content: &str,
    cursor_byte_offset: usize,
) -> Result<(String, usize), String> {
    // Find the line containing the cursor
    let line_start = content[..cursor_byte_offset]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let line_end = content[cursor_byte_offset..]
        .find('\n')
        .map(|pos| cursor_byte_offset + pos)
        .unwrap_or(content.len());

    // Get context lines before
    let lines_before: Vec<&str> = content[..line_start].lines().collect();
    let context_before: Vec<&str> = lines_before.iter().rev().take(3).rev().cloned().collect();

    // Get context lines after
    let after_line_end = if line_end < content.len() {
        line_end + 1
    } else {
        line_end
    };
    let context_after: Vec<&str> = content[after_line_end..].lines().take(4).collect();

    // The line containing the cursor
    let cursor_line = &content[line_start..line_end];
    let cursor_column = cursor_byte_offset - line_start;

    // Build the excerpt
    let mut excerpt = String::new();
    for line in context_before {
        excerpt.push_str(line);
        excerpt.push('\n');
    }
    // Track where cursor will be in the excerpt
    let cursor_offset_in_excerpt = excerpt.len() + cursor_column;
    // Line containing cursor
    excerpt.push_str(cursor_line);
    excerpt.push('\n');
    for line in context_after {
        excerpt.push_str(line);
        excerpt.push('\n');
    }

    // Trim trailing newline
    if excerpt.ends_with('\n') {
        excerpt.pop();
    }

    Ok((excerpt, cursor_offset_in_excerpt))
}

/// Get the line comment prefix for a file based on its extension
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

fn format_rejected_example(response: &ClaudeResponse, rejection_reason: &str) -> String {
    let mut content = String::new();
    content.push_str("# Rejected Example\n\n");
    content.push_str(&format!("## Name\n\n{}\n\n", response.name));
    content.push_str(&format!("## Reasoning\n\n{}\n\n", response.reasoning));
    content.push_str("## Edit History Hunks\n\n```diff\n");
    for hunk in &response.edit_history_hunks {
        content.push_str(hunk);
        content.push_str("\n\n");
    }
    content.push_str("```\n\n");
    content.push_str("## Expected Patch Hunks\n\n```diff\n");
    for hunk in &response.expected_patch_hunks {
        content.push_str(hunk);
        content.push_str("\n\n");
    }
    content.push_str("```\n\n");
    content.push_str(&format!("## Rejection Reason\n\n{}\n", rejection_reason));
    content
}
