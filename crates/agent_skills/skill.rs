use anyhow::{Context as _, Result};
use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use worktree::WorktreeId;

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
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum SkillSource {
    /// From ~/.agents/skills/
    Global,
    /// From {project}/.agents/skills/
    ProjectLocal {
        worktree_id: WorktreeId,
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

/// Minimal skill info for system prompt (not full content)
#[derive(Debug, Clone, Serialize)]
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

    let after_opening = &content[3..];
    let after_opening = after_opening.trim_start_matches([' ', '\t', '\r']);
    let after_opening = after_opening
        .strip_prefix("\r\n")
        .or_else(|| after_opening.strip_prefix('\n'))
        .unwrap_or(after_opening);

    let (end_idx, delimiter_len) =
        match (after_opening.find("\r\n---"), after_opening.find("\n---")) {
            (Some(crlf_idx), Some(lf_idx)) if crlf_idx + 1 == lf_idx => (crlf_idx, 6),
            (_, Some(lf_idx)) => (lf_idx, 4),
            (Some(crlf_idx), None) => (crlf_idx, 6),
            (None, None) => {
                anyhow::bail!("SKILL.md missing closing frontmatter delimiter (---)");
            }
        };

    let frontmatter_yaml = &after_opening[..end_idx];
    let body = &after_opening[end_idx + delimiter_len..];

    let metadata: SkillMetadata =
        serde_yaml::from_str(frontmatter_yaml).context("Invalid YAML frontmatter")?;

    Ok((metadata, body))
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

    // Sort by path so that name conflict resolution in `merge_skills`
    // (in `crates/agent/src/agent.rs`) is deterministic across runs.
    // `fs.read_dir` returns entries in OS/filesystem-dependent order,
    // so without this sort, the "winner" of a name conflict can flip
    // between launches. All entries here share the same `source` (it's
    // passed in), so sorting by path alone is sufficient; the relative
    // ordering of global vs project-local skills is handled by
    // `merge_skills` itself via its iteration order.
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
                if !fs.is_dir(&entry_path).await {
                    return None;
                }
                let skill_file = entry_path.join(SKILL_FILE_NAME);
                if fs.is_file(&skill_file).await {
                    Some(skill_file)
                } else {
                    None
                }
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

pub fn global_skills_dir() -> PathBuf {
    paths::home_dir().join(".agents").join("skills")
}

pub fn project_skills_relative_path() -> &'static str {
    ".agents/skills"
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
}
