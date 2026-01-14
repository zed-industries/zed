use anyhow::{Context as _, Result};
use fs::Fs;
use futures::{StreamExt, future};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use worktree::WorktreeId;

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
}

/// Indicates where a skill was loaded from.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum SkillSource {
    /// From {config_dir}/skills/
    Global,
    /// From {project}/.zed/skills/
    ProjectLocal { worktree_id: WorktreeId },
}

/// Just the frontmatter, used for parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

/// Minimal skill info for system prompt (not full content)
#[derive(Debug, Clone, Serialize)]
pub struct SkillSummary {
    pub name: String,
    pub description: String,
}

impl From<&Skill> for SkillSummary {
    fn from(skill: &Skill) -> Self {
        Self {
            name: skill.name.clone(),
            description: skill.description.clone(),
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
    })
}

fn extract_frontmatter(content: &str) -> Result<(SkillMetadata, &str)> {
    let content = content.trim();

    if !content.starts_with("---") {
        anyhow::bail!("SKILL.md must start with YAML frontmatter (---)")
    }

    let after_first_delimiter = &content[3..];
    let end_delimiter_pos = after_first_delimiter
        .find("\n---")
        .or_else(|| after_first_delimiter.find("\r\n---"))
        .context("SKILL.md missing closing frontmatter delimiter (---)")?;

    let frontmatter_yaml = &after_first_delimiter[..end_delimiter_pos];
    let body_start = end_delimiter_pos + 4; // Skip past \n---
    let body = if body_start < after_first_delimiter.len() {
        after_first_delimiter[body_start..].trim_start_matches(['\n', '\r'])
    } else {
        ""
    };

    let metadata: SkillMetadata =
        serde_yaml::from_str(frontmatter_yaml).context("Failed to parse YAML frontmatter")?;

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

/// Load all skills from a directory, recursively searching for SKILL.md files.
pub async fn load_skills_from_directory(
    fs: &Arc<dyn Fs>,
    dir: &Path,
    source: SkillSource,
) -> Vec<Result<Skill, SkillLoadError>> {
    let mut results = Vec::new();

    if !fs.is_dir(dir).await {
        return results;
    }

    let skill_files = find_skill_files(fs, dir).await;

    let load_tasks: Vec<_> = skill_files
        .into_iter()
        .map(|path| {
            let fs = fs.clone();
            let source = source.clone();
            async move { load_single_skill(&fs, &path, source).await }
        })
        .collect();

    results.extend(future::join_all(load_tasks).await);
    results
}

async fn find_skill_files(fs: &Arc<dyn Fs>, dir: &Path) -> Vec<PathBuf> {
    let mut skill_files = Vec::new();
    let mut dirs_to_visit = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        let Ok(mut entries) = fs.read_dir(&current_dir).await else {
            continue;
        };

        while let Some(entry) = entries.next().await {
            let Ok(entry_path) = entry else {
                continue;
            };

            if fs.is_dir(&entry_path).await {
                dirs_to_visit.push(entry_path);
            } else if entry_path.file_name() == Some(std::ffi::OsStr::new(SKILL_FILE_NAME)) {
                skill_files.push(entry_path);
            }
        }
    }

    skill_files
}

async fn load_single_skill(
    fs: &Arc<dyn Fs>,
    path: &Path,
    source: SkillSource,
) -> Result<Skill, SkillLoadError> {
    let content = fs.load(path).await.map_err(|e| SkillLoadError {
        path: path.to_path_buf(),
        message: format!("Failed to read file: {e}"),
    })?;

    parse_skill(path, &content, source).map_err(|e| SkillLoadError {
        path: path.to_path_buf(),
        message: e.to_string(),
    })
}

/// Returns the path to the global skills directory.
pub fn global_skills_dir() -> PathBuf {
    paths::config_dir().join("skills")
}

/// Returns the relative path to the skills directory within a project.
pub fn project_skills_relative_path() -> &'static str {
    ".zed/skills"
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

        let skill = results
            .into_iter()
            .next()
            .unwrap()
            .expect("Should load successfully");
        assert_eq!(skill.name, "my-skill");
        assert_eq!(skill.description, "Test skill");
    }

    #[gpui::test]
    async fn test_load_nested_skills(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "skill-a": {
                    "SKILL.md": "---\nname: skill-a\ndescription: Skill A\n---\n\nContent A"
                },
                "nested": {
                    "skill-b": {
                        "SKILL.md": "---\nname: skill-b\ndescription: Skill B\n---\n\nContent B"
                    }
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

        let skills: Vec<_> = results
            .into_iter()
            .map(|r| r.expect("Should load"))
            .collect();
        let names: Vec<_> = skills.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"skill-a"));
        assert!(names.contains(&"skill-b"));
    }

    #[gpui::test]
    async fn test_load_ignores_non_skill_files(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/skills",
            serde_json::json!({
                "my-skill": {
                    "SKILL.md": "---\nname: my-skill\ndescription: Test\n---\n\nContent",
                    "helper.md": "Some helper file",
                    "README.md": "Readme content"
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

        let (successes, failures): (Vec<_>, Vec<_>) = results.into_iter().partition(|r| r.is_ok());
        assert_eq!(successes.len(), 1);
        assert_eq!(failures.len(), 1);
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
            description: "A test skill".to_string(),
            source: SkillSource::Global,
            directory_path: PathBuf::from("/skills/test-skill"),
            skill_file_path: PathBuf::from("/skills/test-skill/SKILL.md"),
            content: "Content here".to_string(),
        };

        let summary = SkillSummary::from(&skill);
        assert_eq!(summary.name, "test-skill");
        assert_eq!(summary.description, "A test skill");
    }
}
