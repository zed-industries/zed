use anyhow::{Context as _, Result};
use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::paths::component_matches_ignore_ascii_case;

/// First segment of the skills directory path: `.agents`.
pub const AGENTS_DIR_NAME: &str = ".agents";

/// Second segment of the skills directory path: `skills`.
pub const SKILLS_DIR_NAME: &str = "skills";

/// Opaque identifier for the project scope a skill was loaded from.
///
/// `agent_skills` is a leaf crate and intentionally does not depend on
/// `worktree`. Callers (e.g. the `agent` crate) construct these from
/// `worktree::WorktreeId::to_usize()` and recover the original ID via
/// `worktree::WorktreeId::from_usize()` when needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillScopeId(pub usize);

/// Cap on concurrent filesystem operations during skill discovery and loading.
/// Without this bound, a `.agents/skills` directory containing thousands of
/// entries would fan out an equally large number of concurrent OS-level I/O
/// operations, potentially exhausting file descriptors or stalling the app.
const SKILL_IO_CONCURRENCY: usize = 16;

/// Maximum size for a single SKILL.md file (100KB)
pub const MAX_SKILL_FILE_SIZE: usize = 100 * 1024;

/// Maximum total size for skill descriptions in system prompt (50KB)
pub const MAX_SKILL_DESCRIPTIONS_SIZE: usize = 50 * 1024;

/// The name of the skill definition file
pub const SKILL_FILE_NAME: &str = "SKILL.md";

/// Represents a loaded skill with all its metadata and content.
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub source: SkillSource,
    /// Absolute path to the skill directory
    pub directory_path: PathBuf,
    /// Absolute path to the SKILL.md file
    pub skill_file_path: PathBuf,
    /// The full content of SKILL.md (excluding frontmatter)
    pub content: String,
    /// When `true`, this skill is hidden from the model's catalog and the
    /// `skill` tool refuses to load it. The user can still invoke it as a
    /// slash command.
    pub disable_model_invocation: bool,
}

/// Indicates where a skill was loaded from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillSource {
    /// From ~/.agents/skills/
    Global,
    /// From {project}/.agents/skills/
    ProjectLocal {
        worktree_id: SkillScopeId,
        worktree_root_name: Arc<str>,
    },
}

/// Just the frontmatter, used for parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    #[serde(default, rename = "disable-model-invocation")]
    pub disable_model_invocation: bool,
}

/// Minimal skill info for system prompt (not full content).
///
/// `Serialize` is required for handlebars rendering of the system prompt
/// template (see `ProjectContext` in `prompt_store`). `PartialEq, Eq` lets
/// the agent compare freshly-built `ProjectContext`s and skip pushing an
/// unchanged value through the project_context entity (which would
/// otherwise look like a system-prompt change to the model and invalidate
/// the API's prompt cache).
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct SkillSummary {
    pub name: String,
    pub description: String,
    /// Absolute path to the SKILL.md file, so the model can resolve
    /// references relative to the skill's directory when reading bundled
    /// resources.
    pub location: String,
}

impl From<&Skill> for SkillSummary {
    fn from(skill: &Skill) -> Self {
        Self {
            name: skill.name.clone(),
            description: skill.description.clone(),
            location: skill.skill_file_path.to_string_lossy().into_owned(),
        }
    }
}

/// Error that occurred while loading a skill
#[derive(Debug, Clone)]
pub struct SkillLoadError {
    pub path: PathBuf,
    pub message: String,
}

impl std::fmt::Display for SkillLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.message)
    }
}

impl std::error::Error for SkillLoadError {}

/// Parse a SKILL.md file into a Skill struct.
///
/// The file must have YAML frontmatter between `---` delimiters containing
/// `name` and `description` fields. The content after frontmatter becomes
/// the skill's instructions.
pub fn parse_skill(skill_file_path: &Path, content: &str, source: SkillSource) -> Result<Skill> {
    if content.len() > MAX_SKILL_FILE_SIZE {
        anyhow::bail!(
            "SKILL.md file exceeds maximum size of {}KB",
            MAX_SKILL_FILE_SIZE / 1024
        );
    }

    let (metadata, body) = extract_frontmatter(content)?;

    validate_name(&metadata.name)?;
    validate_description(&metadata.description)?;

    let directory_path = skill_file_path
        .parent()
        .context("SKILL.md file has no parent directory")?
        .to_path_buf();

    Ok(Skill {
        name: metadata.name,
        description: metadata.description,
        source,
        directory_path,
        skill_file_path: skill_file_path.to_path_buf(),
        content: body.trim().to_string(),
        disable_model_invocation: metadata.disable_model_invocation,
    })
}

fn extract_frontmatter(content: &str) -> Result<(SkillMetadata, &str)> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        anyhow::bail!("SKILL.md must start with YAML frontmatter (---)");
    }

    // Find every candidate closing `---` line: a line consisting EXACTLY of
    // `---` (followed by `\n`, `\r\n`, or EOF) at column 0, excluding the
    // opening line itself. The opener occupies bytes 0..(first line ending),
    // and our scan starts after each `\n`, so the opener is naturally skipped.
    //
    // For each candidate we record the byte position right after its line
    // ending; that's both where the YAML stream slice ends and where the body
    // begins.
    let bytes = content.as_bytes();
    let mut candidates: Vec<usize> = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if b != b'\n' {
            continue;
        }
        let line_start = i + 1;
        if line_start + 3 > bytes.len() {
            continue;
        }
        if &bytes[line_start..line_start + 3] != b"---" {
            continue;
        }
        let after_dashes = line_start + 3;
        let end = if after_dashes == bytes.len() {
            after_dashes
        } else if bytes[after_dashes] == b'\n' {
            after_dashes + 1
        } else if after_dashes + 1 < bytes.len()
            && bytes[after_dashes] == b'\r'
            && bytes[after_dashes + 1] == b'\n'
        {
            after_dashes + 2
        } else {
            // Line is something like `---trailing` or `----`; not a candidate.
            continue;
        };
        candidates.push(end);
    }

    if candidates.is_empty() {
        anyhow::bail!("SKILL.md missing closing frontmatter delimiter (---)");
    }

    // Try each candidate in order: slice content up through the candidate's
    // terminator and ask `serde_yaml_ng` to parse it as a YAML stream. If the
    // first document deserializes into `SkillMetadata`, that candidate is the
    // real closer. Otherwise an earlier candidate may have cut the YAML in the
    // middle of a scalar / quoted string; try the next one.
    let mut last_error: Option<anyhow::Error> = None;
    for end in candidates {
        let prefix = &content[..end];
        let mut docs = serde_yaml_ng::Deserializer::from_str(prefix);
        let Some(first_doc) = docs.next() else {
            continue;
        };
        match SkillMetadata::deserialize(first_doc) {
            Ok(metadata) => return Ok((metadata, &content[end..])),
            Err(e) => last_error = Some(anyhow::Error::new(e)),
        }
    }

    Err(last_error
        .unwrap_or_else(|| anyhow::anyhow!("could not parse YAML frontmatter"))
        .context("Invalid YAML frontmatter"))
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Skill name cannot be empty");
    }

    if name.len() > 64 {
        anyhow::bail!("Skill name must be at most 64 characters");
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        anyhow::bail!("Skill name must contain only lowercase letters, numbers, and hyphens");
    }

    Ok(())
}

fn validate_description(description: &str) -> Result<()> {
    if description.is_empty() {
        anyhow::bail!("Skill description cannot be empty");
    }

    if description.len() > 1024 {
        anyhow::bail!("Skill description must be at most 1024 characters");
    }

    Ok(())
}

pub async fn load_skills_from_directory(
    fs: &Arc<dyn Fs>,
    directory: &Path,
    source: SkillSource,
) -> Vec<Result<Skill, SkillLoadError>> {
    if !fs.is_dir(directory).await {
        return Vec::new();
    }

    let skill_files = find_skill_files(fs, directory).await;

    let mut results: Vec<Result<Skill, SkillLoadError>> = futures::stream::iter(skill_files)
        .map(|path| {
            let fs = fs.clone();
            let source = source.clone();
            async move { load_single_skill(fs, path, source).await }
        })
        .buffer_unordered(SKILL_IO_CONCURRENCY)
        .collect()
        .await;

    // Sort by path so name-conflict resolution in `merge_skills` is
    // deterministic — `fs.read_dir` order is filesystem-dependent.
    results.sort_by(|a, b| {
        let path_a: &Path = match a {
            Ok(skill) => &skill.skill_file_path,
            Err(error) => &error.path,
        };
        let path_b: &Path = match b {
            Ok(skill) => &skill.skill_file_path,
            Err(error) => &error.path,
        };
        path_a.cmp(path_b)
    });

    results
}

/// Find every `<skills_root>/<name>/SKILL.md` directly under `directory`.
///
/// Discovery is intentionally one level deep: a skill is the immediate
/// child directory of the skills root, and `SKILL.md` is the file that
/// names it. See `crates/agent_skills/README.md` for why we don't recurse.
async fn find_skill_files(fs: &Arc<dyn Fs>, directory: &Path) -> Vec<PathBuf> {
    let Ok(mut entries) = fs.read_dir(directory).await else {
        return Vec::new();
    };

    let mut entry_paths = Vec::new();
    while let Some(entry) = entries.next().await {
        if let Ok(entry_path) = entry {
            entry_paths.push(entry_path);
        }
    }

    futures::stream::iter(entry_paths)
        .map(|entry_path| {
            let fs = fs.clone();
            async move {
                let Ok(Some(metadata)) = fs.metadata(&entry_path).await else {
                    return None;
                };
                if !metadata.is_dir {
                    return None;
                }
                let skill_file = entry_path.join(SKILL_FILE_NAME);
                fs.is_file(&skill_file).await.then_some(skill_file)
            }
        })
        .buffer_unordered(SKILL_IO_CONCURRENCY)
        .filter_map(|x| async move { x })
        .collect()
        .await
}

async fn load_single_skill(
    fs: Arc<dyn Fs>,
    path: PathBuf,
    source: SkillSource,
) -> Result<Skill, SkillLoadError> {
    // Short-circuit on oversized files before loading their contents into
    // memory, so a stray multi-GB file named `SKILL.md` can't OOM the app.
    // We only act on a positive signal that the file is too large; if
    // metadata fails or is unavailable, we fall through to `fs.load`,
    // which will surface its own error (and `parse_skill` enforces the
    // same limit as a defense-in-depth backstop).
    if let Ok(Some(metadata)) = fs.metadata(&path).await
        && metadata.len > MAX_SKILL_FILE_SIZE as u64
    {
        return Err(SkillLoadError {
            path: path.clone(),
            message: format!(
                "SKILL.md file exceeds maximum size of {}KB",
                MAX_SKILL_FILE_SIZE / 1024
            ),
        });
    }

    let content = fs.load(&path).await.map_err(|e| SkillLoadError {
        path: path.clone(),
        message: format!("Failed to read file: {}", e),
    })?;

    parse_skill(&path, &content, source).map_err(|e| SkillLoadError {
        path: path.clone(),
        message: e.to_string(),
    })
}

/// Returns the global skills directory: `~/.agents/skills`.
///
/// Other agents (e.g. Claude Code) already write skill files into this
/// location, so a Zed installation may have skills here even before the
/// rest of Zed's skills support ships.
///
/// In test builds, `paths::home_dir()` is hardcoded to a fixed path
/// (e.g. `/Users/zed`), so all tests using this function operate on the
/// same simulated home directory. Each test should use its own `FakeFs`
/// instance to keep skill setups from leaking across tests.
pub fn global_skills_dir() -> PathBuf {
    paths::home_dir()
        .join(AGENTS_DIR_NAME)
        .join(SKILLS_DIR_NAME)
}

/// Project-local skills live at this path relative to a worktree root,
/// e.g. `<worktree>/.agents/skills/<skill>/SKILL.md`.
pub fn project_skills_relative_path() -> &'static str {
    ".agents/skills"
}

/// Returns `true` if `path` looks like it points into an agent skills
/// directory — i.e. it contains `AGENTS_DIR_NAME` immediately followed by
/// `SKILLS_DIR_NAME` as two consecutive path components, anywhere in the
/// path. Comparison is case-insensitive so it agrees with classifiers
/// that canonicalize against `~/.agents/skills` on case-insensitive
/// filesystems (macOS/Windows by default).
///
/// The path arriving here can be any of:
///
///   1. Bare relative-to-worktree-root: `.agents/skills/...`
///   2. Worktree-name prefixed:         `<worktree>/.agents/skills/...`
///   3. Absolute:                       `/path/to/worktree/.agents/skills/...`
///
/// Any-depth matching has a known cost: a `.agents/skills` directory
/// nested inside vendored sources (e.g. `vendor/x/.agents/skills/...`)
/// would also be flagged. We accept that as the safer-failing direction —
/// an extra confirmation prompt for a vendored file is annoying, while
/// silently letting the agent overwrite a `.agents/skills` tree the user
/// didn't expect to be touched is unsafe.
pub fn is_agents_skills_path(path: &Path) -> bool {
    let mut components = path.components().map(|c| c.as_os_str());
    let Some(mut prev) = components.next() else {
        return false;
    };
    for curr in components {
        if component_matches_ignore_ascii_case(prev, AGENTS_DIR_NAME)
            && component_matches_ignore_ascii_case(curr, SKILLS_DIR_NAME)
        {
            return true;
        }
        prev = curr;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;

    #[test]
    fn test_parse_valid_skill() {
        let content = r#"---
name: my-skill
description: A test skill for testing purposes
---

# My Skill

## Instructions
Do the thing.
"#;

        let result = parse_skill(
            Path::new("/skills/my-skill/SKILL.md"),
            content,
            SkillSource::Global,
        );
        let skill = result.expect("Should parse successfully");

        assert_eq!(skill.name, "my-skill");
        assert_eq!(skill.description, "A test skill for testing purposes");
        assert_eq!(skill.directory_path, Path::new("/skills/my-skill"));
        assert!(skill.content.contains("# My Skill"));
        assert!(skill.content.contains("Do the thing."));
        // Default: skill is invocable by both model and user.
        assert!(!skill.disable_model_invocation);
    }

    #[test]
    fn test_parse_disable_model_invocation_true() {
        let content = r#"---
name: deploy
description: Deploy the application to production.
disable-model-invocation: true
---

Steps to deploy.
"#;

        let skill = parse_skill(
            Path::new("/skills/deploy/SKILL.md"),
            content,
            SkillSource::Global,
        )
        .expect("should parse");
        assert!(skill.disable_model_invocation);
    }

    #[test]
    fn test_parse_disable_model_invocation_explicit_false() {
        let content = r#"---
name: helper
description: A helper skill.
disable-model-invocation: false
---

Help.
"#;

        let skill = parse_skill(
            Path::new("/skills/helper/SKILL.md"),
            content,
            SkillSource::Global,
        )
        .expect("should parse");
        assert!(!skill.disable_model_invocation);
    }

    #[test]
    fn test_parse_missing_frontmatter() {
        let content = "# My Skill\n\nNo frontmatter here.";

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must start with YAML frontmatter")
        );
    }

    #[test]
    fn test_parse_missing_closing_delimiter() {
        let content = r#"---
name: test
description: Test
# No closing delimiter
"#;

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("missing closing frontmatter delimiter")
        );
    }

    #[test]
    fn test_parse_empty_frontmatter_closing_on_next_line() {
        // An empty frontmatter (closer immediately after the opener) is a real
        // authoring case. Parsing should ultimately fail because the empty YAML
        // doc lacks `name` and `description`, but the error must be the proper
        // YAML/missing-field error rather than "missing closing frontmatter
        // delimiter" — the closer is right there.
        let content = "---\n---\nbody\n";

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_chain = format!("{:?}", err);
        assert!(
            !err_chain.contains("missing closing frontmatter delimiter"),
            "Error should NOT be the missing-closer error since the closer is present: {}",
            err_chain
        );
        assert!(
            err_chain.contains("missing field")
                || err_chain.contains("name")
                || err_chain.contains("description")
                || err_chain.contains("Invalid YAML"),
            "Error should mention missing name/description field or invalid YAML: {}",
            err_chain
        );
    }

    #[test]
    fn test_parse_missing_name() {
        let content = r#"---
description: A test skill
---

Content here.
"#;

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_chain = format!("{:?}", err);
        assert!(
            err_chain.contains("missing field")
                || err_chain.contains("name")
                || err_chain.contains("Invalid YAML"),
            "Error should mention missing name field or invalid YAML: {}",
            err_chain
        );
    }

    #[test]
    fn test_parse_missing_description() {
        let content = r#"---
name: test-skill
---

Content here.
"#;

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_chain = format!("{:?}", err);
        assert!(
            err_chain.contains("missing field")
                || err_chain.contains("description")
                || err_chain.contains("Invalid YAML"),
            "Error should mention missing description field or invalid YAML: {}",
            err_chain
        );
    }

    #[test]
    fn test_parse_name_too_long() {
        let long_name = "a".repeat(65);
        let content = format!(
            r#"---
name: {long_name}
description: Test
---

Content.
"#
        );

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            &content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("at most 64 characters")
        );
    }

    #[test]
    fn test_parse_name_invalid_chars() {
        let content = r#"---
name: My_Skill
description: Test
---

Content.
"#;

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("lowercase letters, numbers, and hyphens")
        );
    }

    #[test]
    fn test_parse_description_too_long() {
        let long_desc = "a".repeat(1025);
        let content = format!(
            r#"---
name: test
description: {long_desc}
---

Content.
"#
        );

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            &content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("at most 1024 characters")
        );
    }

    #[test]
    fn test_parse_empty_description() {
        let content = r#"---
name: test
description: ""
---

Content.
"#;

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_file_too_large() {
        let large_content = format!(
            r#"---
name: test
description: Test skill
---

{}"#,
            "x".repeat(MAX_SKILL_FILE_SIZE + 1)
        );

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            &large_content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
    }

    #[test]
    fn test_parse_empty_body_after_frontmatter() {
        let content = r#"---
name: minimal-skill
description: A skill with no body content
---
"#;

        let result = parse_skill(
            Path::new("/skills/minimal/SKILL.md"),
            content,
            SkillSource::Global,
        );

        let skill = result.expect("Empty body should be allowed");
        assert_eq!(skill.name, "minimal-skill");
        assert_eq!(skill.description, "A skill with no body content");
        assert!(skill.content.is_empty() || skill.content.trim().is_empty());
    }

    #[test]
    fn test_parse_whitespace_only_body() {
        let content = "---\nname: whitespace-skill\ndescription: Test\n---\n\n   \n\n   \n";

        let result = parse_skill(
            Path::new("/skills/ws/SKILL.md"),
            content,
            SkillSource::Global,
        );

        let skill = result.expect("Whitespace-only body should be allowed");
        assert!(skill.content.trim().is_empty());
    }

    #[test]
    fn test_parse_skill_with_crlf_line_endings() {
        let content = "---\r\nname: crlf-skill\r\ndescription: A skill with CRLF line endings\r\n---\r\n\r\n# CRLF Skill\r\n\r\nDo the thing.\r\n";

        let result = parse_skill(
            Path::new("/skills/crlf-skill/SKILL.md"),
            content,
            SkillSource::Global,
        );
        let skill = result.expect("CRLF document should parse successfully");

        assert_eq!(skill.name, "crlf-skill");
        assert_eq!(skill.description, "A skill with CRLF line endings");
        assert!(skill.content.contains("# CRLF Skill"));
        assert!(skill.content.contains("Do the thing."));
    }

    #[test]
    fn test_parse_skill_with_mixed_line_endings() {
        let content = "---\r\nname: mixed-skill\r\ndescription: Frontmatter uses CRLF, body uses LF\r\n---\r\n\n# Mixed Skill\n\nBody uses LF only.\n";

        let result = parse_skill(
            Path::new("/skills/mixed-skill/SKILL.md"),
            content,
            SkillSource::Global,
        );
        let skill = result.expect("Mixed line endings should parse successfully");

        assert_eq!(skill.name, "mixed-skill");
        assert_eq!(skill.description, "Frontmatter uses CRLF, body uses LF");
        assert!(skill.content.contains("# Mixed Skill"));
        assert!(skill.content.contains("Body uses LF only."));
    }

    #[test]
    fn test_parse_rejects_closing_delimiter_with_trailing_chars() {
        // The only `---` after the opener has trailing junk on the same line,
        // so it isn't a valid closing delimiter and parsing must error.
        let content = "---\nname: foo\ndescription: bar\n---trailing-junk\nbody content\n";

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("missing closing frontmatter delimiter")
        );
    }

    #[test]
    fn test_parse_accepts_only_truly_terminated_closing_delimiter() {
        // The first `---trailing` appears inside a quoted YAML string and is
        // NOT alone on its line, so it must not be treated as the closer.
        // The real closer comes later as `\n---\n`.
        let content = "---\nname: skill-name\ndescription: A real description\nsummary: \"---trailing\"\n---\nbody content\n";

        let skill = parse_skill(
            Path::new("/skills/skill-name/SKILL.md"),
            content,
            SkillSource::Global,
        )
        .expect("Should pick the truly-terminated closing delimiter");

        assert_eq!(skill.name, "skill-name");
        assert_eq!(skill.description, "A real description");
        assert_eq!(skill.content, "body content");
    }

    #[test]
    fn test_parse_accepts_four_dashes_as_invalid_closer() {
        // A line of four dashes is NOT a valid closing delimiter; with no
        // valid closer following, parsing must error.
        let content = "---\nname: foo\ndescription: bar\n----\nbody content\n";

        let result = parse_skill(
            Path::new("/skills/test/SKILL.md"),
            content,
            SkillSource::Global,
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("missing closing frontmatter delimiter")
        );
    }

    #[gpui::test]
    async fn test_load_skills_from_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/skills", serde_json::json!({})).await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_load_single_skill(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "my-skill": {
                    "SKILL.md": "---\nname: my-skill\ndescription: Test skill\n---\n\n# Instructions\nDo stuff."
                }
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 1);
        let skill = results[0].as_ref().expect("Should load successfully");
        assert_eq!(skill.name, "my-skill");
        assert_eq!(skill.description, "Test skill");
        assert!(skill.content.contains("# Instructions"));
    }

    #[gpui::test]
    async fn test_load_nested_skills(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "skill-one": {
                    "SKILL.md": "---\nname: skill-one\ndescription: First skill\n---\n\nContent one"
                },
                "skill-two": {
                    "SKILL.md": "---\nname: skill-two\ndescription: Second skill\n---\n\nContent two"
                }
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 2);
        let names: Vec<&str> = results
            .iter()
            .filter_map(|r| r.as_ref().ok())
            .map(|s| s.name.as_str())
            .collect();
        assert!(names.contains(&"skill-one"));
        assert!(names.contains(&"skill-two"));
    }

    #[gpui::test]
    async fn test_load_skills_returns_results_sorted_by_path(cx: &mut TestAppContext) {
        // `merge_skills` resolves name conflicts by keeping the first
        // entry in iteration order. Without a stable sort here, the
        // result depends on `fs.read_dir`, which is OS/filesystem-
        // dependent. Assert the contract: results come back sorted by
        // skill file path regardless of insertion order.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "charlie": {
                    "SKILL.md": "---\nname: charlie\ndescription: C\n---\n\nC"
                },
                "alpha": {
                    "SKILL.md": "---\nname: alpha\ndescription: A\n---\n\nA"
                },
                "bravo": {
                    "SKILL.md": "---\nname: bravo\ndescription: B\n---\n\nB"
                },
                "delta": {
                    "SKILL.md": "No frontmatter, will fail"
                },
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 4);

        let paths: Vec<PathBuf> = results
            .iter()
            .map(|r| match r {
                Ok(skill) => skill.skill_file_path.clone(),
                Err(error) => error.path.clone(),
            })
            .collect();

        let mut expected = paths.clone();
        expected.sort();
        assert_eq!(paths, expected);
    }

    #[gpui::test]
    async fn test_load_ignores_non_skill_files(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "my-skill": {
                    "SKILL.md": "---\nname: my-skill\ndescription: Test\n---\n\nContent"
                },
                "not-a-skill.txt": "This is not a skill",
                "some-dir": {
                    "other-file.md": "Not a SKILL.md"
                }
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 1);
        let skill = results[0].as_ref().expect("Should load successfully");
        assert_eq!(skill.name, "my-skill");
    }

    #[gpui::test]
    async fn test_load_returns_errors_for_invalid_skills(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "valid-skill": {
                    "SKILL.md": "---\nname: valid-skill\ndescription: Valid\n---\n\nContent"
                },
                "invalid-skill": {
                    "SKILL.md": "No frontmatter here"
                }
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 2);

        let (successes, errors): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.is_ok());

        assert_eq!(successes.len(), 1);
        assert_eq!(errors.len(), 1);

        let error = errors[0].as_ref().unwrap_err();
        assert!(error.path.to_string_lossy().contains("invalid-skill"));
    }

    #[gpui::test]
    async fn test_load_from_nonexistent_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/nonexistent"),
            SkillSource::Global,
        )
        .await;

        assert!(results.is_empty());
    }

    #[test]
    fn test_skill_summary_from_skill() {
        let skill = Skill {
            name: "test-skill".to_string(),
            description: "A test description".to_string(),
            source: SkillSource::Global,
            directory_path: PathBuf::from("/skills/test-skill"),
            skill_file_path: PathBuf::from("/skills/test-skill/SKILL.md"),
            content: "Instructions here".to_string(),
            disable_model_invocation: false,
        };

        let summary = SkillSummary::from(&skill);
        assert_eq!(summary.name, "test-skill");
        assert_eq!(summary.description, "A test description");
        assert_eq!(summary.location, "/skills/test-skill/SKILL.md");
    }

    #[gpui::test]
    async fn test_nested_skill_md_inside_skill_resources_is_not_loaded(cx: &mut TestAppContext) {
        // We only look at immediate children of the skills root, so a
        // `SKILL.md` nested inside a skill's resources directory cannot
        // accidentally be picked up as a separate skill.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "outer": {
                    "SKILL.md": "---\nname: outer\ndescription: Outer skill\n---\n\nBody",
                    "references": {
                        "SKILL.md": "---\nname: bogus-inner\ndescription: Should not load\n---\n\nBody"
                    },
                },
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        let names: Vec<&str> = results
            .iter()
            .filter_map(|r| r.as_ref().ok())
            .map(|s| s.name.as_str())
            .collect();
        assert_eq!(names, vec!["outer"]);
    }

    #[gpui::test]
    async fn test_load_oversized_skill_file_short_circuits(cx: &mut TestAppContext) {
        // A `SKILL.md` whose size exceeds `MAX_SKILL_FILE_SIZE` must be
        // rejected via metadata before we read its contents into memory.
        // Otherwise a stray multi-GB file dropped into a skill directory
        // would OOM the application before `parse_skill`'s size check fires.
        let fs = FakeFs::new(cx.executor());
        let oversized_body = "x".repeat(MAX_SKILL_FILE_SIZE + 1);
        let oversized_content = format!(
            "---\nname: huge\ndescription: Too big\n---\n\n{}",
            oversized_body
        );
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "huge": {
                    "SKILL.md": oversized_content,
                }
            }),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 1);
        let err = results[0].as_ref().expect_err("Oversized file must error");
        assert!(
            err.message.contains("exceeds maximum size"),
            "unexpected error message: {}",
            err.message
        );
    }

    #[test]
    fn is_agents_skills_path_simple_positive() {
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/skills/my-skill/SKILL.md"
        )));
    }

    #[test]
    fn is_agents_skills_path_simple_negative() {
        assert!(!is_agents_skills_path(Path::new("foo/bar/baz")));
    }

    #[test]
    fn is_agents_skills_path_double_agents() {
        // `foo/.agents/.agents/skills` contains a `.agents/skills` pair at
        // depths 2-3. Any-depth matching catches it; this is intentional, so
        // a `.agents/skills` directory the user wasn't expecting to be
        // touched still prompts for confirmation.
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/.agents/skills"
        )));
    }

    #[test]
    fn is_agents_skills_path_agents_without_skills() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents/other")));
    }

    #[test]
    fn is_agents_skills_path_at_start() {
        assert!(is_agents_skills_path(Path::new(".agents/skills")));
    }

    #[test]
    fn is_agents_skills_path_trailing_agents() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents")));
    }

    #[test]
    fn is_agents_skills_path_deep_match() {
        // Any-depth matching: nested `.agents/skills` directories — e.g.
        // inside vendored sources — are flagged too. We prefer the extra
        // prompt over silently letting the agent edit something named
        // `.agents/skills`.
        assert!(is_agents_skills_path(Path::new("a/b/.agents/skills/x.txt")));
        assert!(is_agents_skills_path(Path::new(
            "some/random/place/.agents/skills/foo"
        )));
    }

    #[test]
    fn is_agents_skills_path_absolute() {
        // Absolute paths into a project-local `.agents/skills/` are caught
        // by the same consecutive-component match.
        assert!(is_agents_skills_path(Path::new(
            "/Users/foo/project/.agents/skills/my-skill/SKILL.md"
        )));
        assert!(!is_agents_skills_path(Path::new("/etc/hosts")));
    }

    #[test]
    fn is_agents_skills_path_case_insensitive() {
        // Filesystems on macOS/Windows are case-insensitive by default; the
        // classifier must agree.
        assert!(is_agents_skills_path(Path::new(".AGENTS/skills/foo")));
        assert!(is_agents_skills_path(Path::new(".agents/SKILLS/foo")));
        assert!(is_agents_skills_path(Path::new(
            "project/.AGENTS/SKILLS/foo"
        )));
    }
}
