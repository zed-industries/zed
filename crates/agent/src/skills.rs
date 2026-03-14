//! Agent Skills discovery and formatting.
//!
//! This module discovers user-defined skills from global and worktree locations
//! and formats them for display in the agent's system prompt.

use crate::{SkillContext, SkillsPromptTemplate, Template, Templates};
use anyhow::{Result, anyhow};
use collections::HashMap;
use gpui::{App, AppContext, Context, Entity};
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

/// A minimal representation of a discovered skill for formatting.
#[derive(Clone, Debug)]
pub struct Skill {
    name: String,
    description: String,
    path: PathBuf,
}

/// Metadata extracted from a skill's YAML frontmatter.
#[derive(Deserialize, Debug)]
struct SkillMetadata {
    name: String,
    description: String,
    #[allow(dead_code)]
    license: Option<String>,
    compatibility: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    metadata: HashMap<String, String>,
    #[allow(dead_code)]
    allowed_tools: Option<String>,
}

impl SkillMetadata {
    /// Validates that the skill metadata conforms to the Agent Skills specification.
    fn validate(&self, expected_dir_name: &str) -> Result<()> {
        if self.name != expected_dir_name {
            return Err(anyhow!(
                "skill name '{}' doesn't match directory name '{}'",
                self.name,
                expected_dir_name
            ));
        }

        if self.name.is_empty() {
            return Err(anyhow!("skill name cannot be empty"));
        }

        if self.name.len() > 64 {
            return Err(anyhow!("skill name cannot exceed 64 characters"));
        }

        if !self
            .name
            .chars()
            .all(|c| c.is_lowercase() || c.is_numeric() || c == '-')
        {
            return Err(anyhow!(
                "skill name must be lowercase alphanumeric + hyphens only: {}",
                self.name
            ));
        }

        if self.name.contains("--") {
            return Err(anyhow!(
                "skill name must not contain consecutive hyphens: {}",
                self.name
            ));
        }

        if self.name.starts_with('-') || self.name.ends_with('-') {
            return Err(anyhow!(
                "skill name must not start or end with hyphen: {}",
                self.name
            ));
        }

        if self.description.is_empty() {
            return Err(anyhow!("skill description cannot be empty"));
        }

        if self.description.len() > 1024 {
            return Err(anyhow!("skill description cannot exceed 1024 characters"));
        }

        if let Some(ref compatibility) = self.compatibility {
            if compatibility.len() > 500 {
                return Err(anyhow!(
                    "skill compatibility exceeds 500 characters: {}",
                    compatibility.len()
                ));
            }
        }

        Ok(())
    }
}

/// Parses YAML frontmatter from a markdown file.
/// Returns the parsed metadata and the markdown body.
fn parse_skill_file(content: &str, expected_dir_name: &str) -> Result<(SkillMetadata, String)> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        return Err(anyhow!("SKILL.md must start with YAML frontmatter (---)"));
    }

    let end_marker = content[3..].find("\n---");
    let (yaml_part, body) = match end_marker {
        Some(end) => {
            let yaml_end = 3 + end;
            let yaml = content[3..yaml_end].trim().to_string();
            let body_start = yaml_end + 3;
            let body = content[body_start..].trim_start().to_string();
            (yaml, body)
        }
        None => return Err(anyhow!("YAML frontmatter not properly closed with ---")),
    };

    let metadata: SkillMetadata = serde_yml::from_str(&yaml_part)
        .map_err(|e| anyhow!("failed to parse YAML frontmatter: {}", e))?;

    metadata.validate(expected_dir_name)?;

    Ok((metadata, body))
}

/// Discovers all skills in the given directory.
/// Returns a map of skill name to Skill.
fn discover_skills_sync(skills_dir: &Path) -> HashMap<String, Arc<Skill>> {
    let mut skills = HashMap::default();

    if !skills_dir.exists() || !skills_dir.is_dir() {
        return skills;
    }

    let entries = match std::fs::read_dir(skills_dir) {
        Ok(entries) => entries,
        Err(e) => {
            log::warn!("failed to read skills directory: {}", e);
            return skills;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let skill_file = path.join("SKILL.md");
        if !skill_file.exists() {
            continue;
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        let content = match std::fs::read_to_string(&skill_file) {
            Ok(content) => content,
            Err(e) => {
                log::warn!("failed to read {:?}: {}", skill_file, e);
                continue;
            }
        };

        let (metadata, _body) = match parse_skill_file(&content, dir_name) {
            Ok(result) => result,
            Err(e) => {
                log::warn!("failed to parse {:?}: {}", skill_file, e);
                continue;
            }
        };

        let skill = Arc::new(Skill {
            name: metadata.name,
            description: metadata.description,
            path,
        });

        skills.insert(skill.name.clone(), skill);
    }

    skills
}

/// Returns the canonicalized global skills directory path (~/.config/zed/skills).
/// Result is cached after first call. If canonicalization fails, returns the original path.
pub fn global_skills_dir() -> PathBuf {
    paths::config_dir().join("skills")
}

/// Discovers skills from both global and worktree locations.
/// Worktree skills take precedence over global skills with the same name.
pub fn discover_all_skills_sync(worktree_roots: &[PathBuf]) -> HashMap<String, Arc<Skill>> {
    let mut all_skills = discover_skills_sync(&global_skills_dir());

    for worktree in worktree_roots {
        let worktree_skills = discover_skills_sync(&worktree.join(".agents").join("skills"));
        for (name, skill) in worktree_skills {
            all_skills.insert(name, skill);
        }
    }

    all_skills
}

/// Format skills for display in the system prompt using handlebars templating.
pub fn format_skills_for_prompt(
    skills: &HashMap<String, Arc<Skill>>,
    templates: Arc<Templates>,
) -> String {
    let mut skill_list: Vec<_> = skills.values().collect();
    skill_list.sort_by(|a, b| a.name.cmp(&b.name));

    let skill_contexts: Vec<SkillContext> = skill_list
        .into_iter()
        .map(|skill| SkillContext {
            name: skill.name.clone(),
            description: if skill.description.len() > 1024 {
                format!("{}...", &skill.description[..1021])
            } else {
                skill.description.clone()
            },
            path: skill.path.display().to_string(),
        })
        .collect();

    let template = SkillsPromptTemplate {
        has_skills: !skill_contexts.is_empty(),
        skills: skill_contexts,
    };

    template.render(&templates).unwrap_or_default()
}

/// Context entity that holds formatted skills for the system prompt.
/// Populates itself asynchronously on creation.
pub struct SkillsContext {
    formatted_skills: Option<String>,
}

impl SkillsContext {
    /// Create a new SkillsContext and spawn background task to populate it.
    pub fn new(
        worktree_roots: Vec<PathBuf>,
        templates: Arc<Templates>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            // Spawn async task that will populate the skills
            cx.spawn(async move |this, cx| {
                let formatted = cx
                    .background_spawn(async move {
                        let skills = discover_all_skills_sync(&worktree_roots);
                        format_skills_for_prompt(&skills, templates)
                    })
                    .await;

                this.update(cx, |this, _cx| {
                    this.formatted_skills = Some(formatted);
                })
                .ok();
            })
            .detach();

            Self {
                formatted_skills: None,
            }
        })
    }

    /// Create a SkillsContext with pre-populated skills (for loading from DB).
    pub fn from_formatted(formatted_skills: String, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self {
            formatted_skills: Some(formatted_skills),
        })
    }

    /// Get the formatted skills string.
    /// Returns empty string if not yet loaded.
    pub fn formatted(&self) -> &str {
        self.formatted_skills.as_deref().unwrap_or("")
    }

    /// Check if skills have been loaded.
    pub fn is_loaded(&self) -> bool {
        self.formatted_skills.is_some()
    }
}

/// Checks if a path is within a skills directory (global or worktree-specific).
///
/// Expands `~` to home directory, canonicalizes the path, and checks if it's within:
/// - The global skills directory (~/.config/zed/skills)
/// - Any worktree's .agents/skills directory
///
/// Returns Some(canonical_path) if the path is within a skills directory.
/// Returns None if the path is not within any skills directory.
/// Check if a canonicalized path is within any skills directory.
/// This is the pure logic version that operates on already-canonicalized paths.
pub fn is_skills_path_canonical(
    canonical_input: &Path,
    worktree_roots: &[PathBuf],
) -> Option<PathBuf> {
    let global_skills_root = global_skills_dir();
    if canonical_input.starts_with(&global_skills_root) {
        return Some(canonical_input.to_path_buf());
    }

    for worktree_root in worktree_roots {
        let worktree_skills_path = worktree_root.join(".agents").join("skills");
        if canonical_input.starts_with(&worktree_skills_path) {
            return Some(canonical_input.to_path_buf());
        }
    }

    None
}

/// Check if a path is within any skills directory.
/// Handles ~ expansion and canonicalization.
pub fn is_skills_path(input_path: &str, worktree_roots: &[PathBuf]) -> Option<PathBuf> {
    let path = if input_path.starts_with('~') {
        let home = paths::home_dir().to_string_lossy().into_owned();
        PathBuf::from(input_path.replacen('~', &home, 1))
    } else {
        PathBuf::from(input_path)
    };

    let canonical_input = std::fs::canonicalize(&path).ok()?;

    is_skills_path_canonical(&canonical_input, worktree_roots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_metadata() {
        let metadata = SkillMetadata {
            name: "pdf-processing".to_string(),
            description: "Extract text and tables from PDF files".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("pdf-processing").is_ok());
    }

    #[test]
    fn test_validate_name_too_long() {
        let metadata = SkillMetadata {
            name: "a".repeat(65),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("toolongname").is_err());
    }

    #[test]
    fn test_validate_name_empty() {
        let metadata = SkillMetadata {
            name: String::new(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("").is_err());
    }

    #[test]
    fn test_validate_name_invalid_chars() {
        let metadata = SkillMetadata {
            name: "invalid@name".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("invalid@name").is_err());
    }

    #[test]
    fn test_validate_name_starts_with_hyphen() {
        let metadata = SkillMetadata {
            name: "-invalid".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("-invalid").is_err());
    }

    #[test]
    fn test_validate_name_ends_with_hyphen() {
        let metadata = SkillMetadata {
            name: "invalid-".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("invalid-").is_err());
    }

    #[test]
    fn test_validate_name_consecutive_hyphens() {
        let metadata = SkillMetadata {
            name: "in--valid".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("in--valid").is_err()); // Consecutive hyphens are allowed
    }

    #[test]
    fn test_validate_description_empty() {
        let metadata = SkillMetadata {
            name: "valid-name".to_string(),
            description: String::new(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("valid-name").is_err());
    }

    #[test]
    fn test_validate_description_too_long() {
        let metadata = SkillMetadata {
            name: "valid-name".to_string(),
            description: "a".repeat(1025),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("valid-name").is_err());
    }

    #[test]
    fn test_validate_compatibility_too_long() {
        let metadata = SkillMetadata {
            name: "valid-name".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: Some("a".repeat(501)),
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("valid-name").is_err());
    }

    #[test]
    fn test_validate_name_mismatch() {
        let metadata = SkillMetadata {
            name: "bar".to_string(),
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        assert!(metadata.validate("foo").is_err());
    }

    #[test]
    fn test_parse_valid_frontmatter() {
        let content = r#"---
name: test-skill
description: A test skill
---
# Skill Content

This is the skill content."#;

        let (metadata, body) = parse_skill_file(content, "test-skill").unwrap();
        assert_eq!(metadata.name, "test-skill");
        assert_eq!(metadata.description, "A test skill");
        assert!(body.contains("Skill Content"));
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "# Just markdown content";
        assert!(parse_skill_file(content, "test").is_err());
    }

    #[test]
    fn test_parse_unclosed_frontmatter() {
        let content = "---\nname: test\n# No closing";
        assert!(parse_skill_file(content, "test").is_err());
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let content = "---\ninvalid yaml\n---\ncontent";
        assert!(parse_skill_file(content, "test").is_err());
    }

    #[test]
    fn test_format_skills_sorts_alphabetically() {
        let mut skills = HashMap::default();
        skills.insert(
            "z-skill".to_string(),
            Arc::new(Skill {
                name: "z-skill".to_string(),
                description: "Z skill desc".to_string(),
                path: PathBuf::from("/z"),
            }),
        );
        skills.insert(
            "a-skill".to_string(),
            Arc::new(Skill {
                name: "a-skill".to_string(),
                description: "A skill desc".to_string(),
                path: PathBuf::from("/a"),
            }),
        );
        skills.insert(
            "m-skill".to_string(),
            Arc::new(Skill {
                name: "m-skill".to_string(),
                description: "M skill desc".to_string(),
                path: PathBuf::from("/m"),
            }),
        );

        let result = format_skills_for_prompt(&skills, Templates::new());

        // Verify all skills are present
        assert!(result.contains("a-skill"));
        assert!(result.contains("m-skill"));
        assert!(result.contains("z-skill"));

        // Verify alphabetical order: a-skill should appear before m-skill, which should appear before z-skill
        let a_pos = result.find("a-skill").unwrap();
        let m_pos = result.find("m-skill").unwrap();
        let z_pos = result.find("z-skill").unwrap();
        assert!(a_pos < m_pos, "a-skill should appear before m-skill");
        assert!(m_pos < z_pos, "m-skill should appear before z-skill");
    }

    #[test]
    fn test_format_skills_truncates_long_description() {
        let mut skills = HashMap::default();
        let long_description = "a".repeat(1500);

        skills.insert(
            "long-desc-skill".to_string(),
            Arc::new(Skill {
                name: "long-desc-skill".to_string(),
                description: long_description.clone(),
                path: PathBuf::from("/long"),
            }),
        );

        let result = format_skills_for_prompt(&skills, Templates::new());

        // The description should be truncated with "..."
        assert!(result.contains("..."));
        // The full description should NOT be present
        assert!(!result.contains(&long_description));
        // The skill name should still be present
        assert!(result.contains("long-desc-skill"));
    }

    #[test]
    fn test_format_skills_preserves_short_description() {
        let mut skills = HashMap::default();

        skills.insert(
            "short-desc-skill".to_string(),
            Arc::new(Skill {
                name: "short-desc-skill".to_string(),
                description: "Short description".to_string(),
                path: PathBuf::from("/short"),
            }),
        );

        let result = format_skills_for_prompt(&skills, Templates::new());

        // Short descriptions should NOT be truncated (no "..." appended)
        assert!(!result.contains("Short description..."));
        assert!(result.contains("Short description"));
    }

    #[test]
    fn test_format_skills_includes_all_fields() {
        let mut skills = HashMap::default();

        skills.insert(
            "test-skill".to_string(),
            Arc::new(Skill {
                name: "test-skill".to_string(),
                description: "Test description".to_string(),
                path: PathBuf::from("/path/to/skill"),
            }),
        );

        let result = format_skills_for_prompt(&skills, Templates::new());

        // All fields should appear in the output
        assert!(result.contains("test-skill"));
        assert!(result.contains("Test description"));
        assert!(result.contains("/path/to/skill"));
    }

    #[test]
    fn test_format_skills_exactly_1024_char_description() {
        let mut skills = HashMap::default();
        // Exactly 1024 characters should NOT be truncated
        let exact_description = "b".repeat(1024);

        skills.insert(
            "exact-skill".to_string(),
            Arc::new(Skill {
                name: "exact-skill".to_string(),
                description: exact_description,
                path: PathBuf::from("/exact"),
            }),
        );

        let result = format_skills_for_prompt(&skills, Templates::new());

        // Should NOT contain "..." since it's exactly 1024 chars
        assert!(!result.contains("..."));
    }

    #[test]
    fn test_format_skills_empty() {
        let skills = HashMap::default();
        let result = format_skills_for_prompt(&skills, Templates::new());
        // With no skills, template renders to empty string
        assert!(result.is_empty());
    }

    #[test]
    fn test_is_skills_path_canonical_global_directory() {
        let worktree_roots: Vec<PathBuf> = vec![];

        let path = PathBuf::from("/home/user/.config/zed/skills/test.md");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        // This will return Some if the actual global_skills_dir() matches,
        // but since we don't know the user's home directory in tests,
        // this test may return None on systems with different paths.
        // The key assertion is that it doesn't panic and returns consistent types.
        let _ = result;
    }

    #[test]
    fn test_is_skills_path_canonical_worktree_directory() {
        let worktree_roots = vec![PathBuf::from("/home/user/projects/myproject")];

        let path = PathBuf::from("/home/user/projects/myproject/.agents/skills/test.md");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), path);
    }

    #[test]
    fn test_is_skills_path_canonical_worktree_subdirectory() {
        let worktree_roots = vec![PathBuf::from("/home/user/projects/myproject")];

        let path =
            PathBuf::from("/home/user/projects/myproject/.agents/skills/nested/deep/skill.md");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), path);
    }

    #[test]
    fn test_is_skills_path_canonical_not_in_skills() {
        let worktree_roots = vec![PathBuf::from("/home/user/project")];

        let path = PathBuf::from("/etc/passwd");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        assert!(result.is_none());
    }

    #[test]
    fn test_is_skills_path_canonical_sibling_of_skills() {
        let worktree_roots = vec![PathBuf::from("/home/user/project")];

        let path = PathBuf::from("/home/user/project/.agents/config.toml");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        assert!(result.is_none());
    }

    #[test]
    fn test_is_skills_path_canonical_different_worktree() {
        let worktree_roots = vec![PathBuf::from("/home/user/projectA")];

        let path = PathBuf::from("/home/user/projectB/.agents/skills/test.md");
        let result = is_skills_path_canonical(&path, &worktree_roots);

        assert!(result.is_none());
    }

    #[test]
    fn test_is_skills_path_canonical_multiple_worktrees() {
        let worktree_roots = vec![
            PathBuf::from("/home/user/projectA"),
            PathBuf::from("/home/user/projectB"),
        ];

        // Path in first worktree
        let path_a = PathBuf::from("/home/user/projectA/.agents/skills/skill.md");
        let result_a = is_skills_path_canonical(&path_a, &worktree_roots);
        assert!(result_a.is_some());
        assert_eq!(result_a.unwrap(), path_a);

        // Path in second worktree
        let path_b = PathBuf::from("/home/user/projectB/.agents/skills/skill.md");
        let result_b = is_skills_path_canonical(&path_b, &worktree_roots);
        assert!(result_b.is_some());
        assert_eq!(result_b.unwrap(), path_b);
    }

    #[test]
    fn test_parse_with_extra_fields() {
        let content = r#"---
name: test-skill
description: A test skill
license: MIT
compatibility: 1.0
metadata:
  author: Test
  version: 1.0
allowed_tools: bash
---
# Skill Content"#;

        let (metadata, body) = parse_skill_file(content, "test-skill").unwrap();
        assert_eq!(metadata.name, "test-skill");
        assert_eq!(metadata.license, Some("MIT".to_string()));
        assert_eq!(metadata.compatibility, Some("1.0".to_string()));
        assert!(!body.is_empty());
    }

    #[test]
    fn test_validate_unicode_name() {
        let metadata = SkillMetadata {
            name: "测试-skill".to_string(), // Chinese characters
            description: "Valid description".to_string(),
            license: None,
            compatibility: None,
            metadata: HashMap::default(),
            allowed_tools: None,
        };
        // Unicode characters outside allowed set should fail
        assert!(metadata.validate("测试-skill").is_err());
    }
}
