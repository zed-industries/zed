//! File-based storage for user rules.
//!
//! This module provides functionality to store and load user rules as markdown files
//! in the config directory, allowing them to be git-tracked and easily edited.

use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use fs::Fs;
use futures::StreamExt;
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use uuid::Uuid;

use crate::{PromptId, PromptMetadata, UserPromptId};

/// Command execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    pub cmd: String,
    pub args: Vec<String>,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_max_output_bytes")]
    pub max_output_bytes: usize,
    #[serde(default)]
    pub on_startup: bool,
    #[serde(default)]
    pub on_new_chat: bool,
    #[serde(default)]
    pub on_every_message: bool,
}

fn default_timeout() -> u64 {
    5
}

fn default_max_output_bytes() -> usize {
    40_000
}

/// Type of rule content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    #[default]
    Static,
    Command,
}

/// Frontmatter metadata for a rule file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RuleFrontmatter {
    id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(default)]
    default: bool,
    saved_at: DateTime<Utc>,
    #[serde(default, rename = "type")]
    rule_type: RuleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<CommandConfig>,
}

/// A parsed rule file with metadata and content
#[derive(Debug, Clone)]
pub struct RuleFile {
    pub id: UserPromptId,
    pub title: Option<SharedString>,
    pub default: bool,
    pub saved_at: DateTime<Utc>,
    pub rule_type: RuleType,
    pub content: String,
    pub command: Option<CommandConfig>,
}

impl RuleFile {
    /// Convert to PromptMetadata
    pub fn to_metadata(&self) -> PromptMetadata {
        PromptMetadata {
            id: PromptId::User { uuid: self.id },
            title: self.title.clone(),
            default: self.default,
            saved_at: self.saved_at,
            rule_type: self.rule_type.clone(),
            command: self.command.clone(),
        }
    }

    /// Check if this is a command rule
    pub fn is_command(&self) -> bool {
        self.rule_type == RuleType::Command
    }

    /// Check if this command should run on startup
    pub fn runs_on_startup(&self) -> bool {
        self.command.as_ref().map_or(false, |c| c.on_startup)
    }

    /// Check if this command should run on new chat
    pub fn runs_on_new_chat(&self) -> bool {
        self.command.as_ref().map_or(false, |c| c.on_new_chat)
    }

    /// Check if this command should run on every message
    pub fn runs_on_every_message(&self) -> bool {
        self.command.as_ref().map_or(false, |c| c.on_every_message)
    }
}

/// File-based rules store
pub struct FileStore {
    rules_dir: PathBuf,
    pub fs: Arc<dyn Fs>,
}

impl FileStore {
    /// Create a new FileStore
    pub fn new(rules_dir: PathBuf, fs: Arc<dyn Fs>) -> Self {
        Self { rules_dir, fs }
    }

    /// Get the rules directory path
    pub fn rules_dir(&self) -> &Path {
        &self.rules_dir
    }

    /// Initialize the rules directory, creating it if needed
    pub async fn init(&self) -> Result<()> {
        self.fs
            .create_dir(&self.rules_dir)
            .await
            .or_else(|_err| Ok::<(), anyhow::Error>(()))
            .with_context(|| format!("Failed to create rules directory: {:?}", self.rules_dir))
    }

    /// List all rule files in the directory, including subdirectories
    pub async fn list_all(&self) -> Result<Vec<PathBuf>> {
        let mut rule_files = Vec::new();
        self.list_all_recursive(&self.rules_dir, &mut rule_files)
            .await?;
        Ok(rule_files)
    }

    /// Recursively list all .md files in the directory and subdirectories
    fn list_all_recursive<'a>(
        &'a self,
        dir: &'a Path,
        rule_files: &'a mut Vec<PathBuf>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut entries = self.fs.read_dir(dir).await?;

            while let Some(entry_result) = entries.next().await {
                let path = entry_result?;

                if let Ok(Some(metadata)) = self.fs.metadata(&path).await {
                    if metadata.is_dir {
                        // Recursively scan subdirectories
                        self.list_all_recursive(&path, rule_files).await?;
                    } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        rule_files.push(path);
                    }
                }
            }

            Ok(())
        })
    }

    /// Load all rules from the directory
    pub async fn load_all(&self) -> Result<Vec<RuleFile>> {
        let paths = self.list_all().await?;
        let mut rules = Vec::new();

        for path in paths {
            match self.load_from_path(&path).await {
                Ok(rule) => rules.push(rule),
                Err(err) => {
                    log::warn!("Failed to load rule from {:?}: {}", path, err);
                }
            }
        }

        Ok(rules)
    }

    /// Load a specific rule by ID
    pub async fn load(&self, id: &UserPromptId) -> Result<RuleFile> {
        let paths = self.list_all().await?;

        for path in paths {
            if let Ok(rule) = self.load_from_path(&path).await {
                if rule.id.0 == id.0 {
                    return Ok(rule);
                }
            }
        }

        Err(anyhow!("Rule not found: {}", id.0))
    }

    /// Load a rule from a specific file path
    async fn load_from_path(&self, path: &Path) -> Result<RuleFile> {
        let content = self.fs.load(path).await?;
        Self::parse_rule_file(&content)
    }

    /// Parse a rule file with frontmatter
    fn parse_rule_file(content: &str) -> Result<RuleFile> {
        // Check for frontmatter
        if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
            return Err(anyhow!("Rule file must start with frontmatter (---)"));
        }

        // Find the end of frontmatter
        let content_after_start = if let Some(stripped) = content.strip_prefix("---\r\n") {
            stripped
        } else {
            content
                .strip_prefix("---\n")
                .ok_or_else(|| anyhow!("Rule file must start with frontmatter (---)"))?
        };

        let end_marker_idx = content_after_start
            .find("\n---\n")
            .or_else(|| content_after_start.find("\n---\r\n"))
            .ok_or_else(|| anyhow!("Rule file missing closing frontmatter marker (---)"))?;

        let frontmatter_str = &content_after_start[..end_marker_idx];
        let frontmatter: RuleFrontmatter =
            serde_yaml::from_str(frontmatter_str).context("Failed to parse frontmatter")?;

        // Get content after frontmatter
        let content_start = if content_after_start[end_marker_idx..].starts_with("\n---\r\n") {
            end_marker_idx + 6
        } else {
            end_marker_idx + 5
        };
        let rule_content = content_after_start[content_start..].trim().to_string();

        Ok(RuleFile {
            id: UserPromptId(frontmatter.id),
            title: frontmatter.title.map(SharedString::from),
            default: frontmatter.default,
            saved_at: frontmatter.saved_at,
            rule_type: frontmatter.rule_type,
            content: rule_content,
            command: frontmatter.command,
        })
    }

    /// Save a rule to a file, optionally in a subdirectory
    pub async fn save(
        &self,
        id: &UserPromptId,
        title: Option<&str>,
        content: &str,
        default: bool,
        rule_type: RuleType,
        command: Option<CommandConfig>,
    ) -> Result<PathBuf> {
        let frontmatter = RuleFrontmatter {
            id: id.0,
            title: title.map(|s| s.to_string()),
            default,
            saved_at: Utc::now(),
            rule_type,
            command,
        };

        let frontmatter_yaml =
            serde_yaml::to_string(&frontmatter).context("Failed to serialize frontmatter")?;

        let file_content = format!("---\n{}---\n\n{}\n", frontmatter_yaml, content.trim());

        // Always use UUID-based filename to prevent file proliferation when title changes
        let filename = format!("{}.md", id.0);
        let file_path = self.rules_dir.join(&filename);

        // Delete any old files with different names (e.g., from previous title-based saves)
        self.delete_old_files_for_id(id).await.ok();

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            self.fs
                .create_dir(parent)
                .await
                .or_else(|_| Ok::<(), anyhow::Error>(()))
                .ok();
        }

        self.fs
            .atomic_write(file_path.clone(), file_content)
            .await
            .context("Failed to write rule file")?;

        Ok(file_path)
    }

    /// Delete a rule file
    pub async fn delete(&self, id: &UserPromptId) -> Result<()> {
        self.delete_old_files_for_id(id).await
    }

    /// Delete all files associated with a given UUID (handles cleanup of old title-based files)
    async fn delete_old_files_for_id(&self, id: &UserPromptId) -> Result<()> {
        let paths = self.list_all().await?;
        let mut found = false;

        for path in paths {
            if let Ok(rule) = self.load_from_path(&path).await {
                if rule.id.0 == id.0 {
                    self.fs.remove_file(&path, Default::default()).await?;
                    found = true;
                }
            }
        }

        if found {
            Ok(())
        } else {
            Err(anyhow!("Rule not found: {}", id.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_file() {
        let content = r#"---
id: 550e8400-e29b-41d4-a716-446655440000
title: Test Rule
default: true
saved_at: 2024-01-01T00:00:00Z
---

This is the rule content.
"#;

        let rule = FileStore::parse_rule_file(content).unwrap();
        assert_eq!(rule.title, Some(SharedString::from("Test Rule")));
        assert_eq!(rule.default, true);
        assert_eq!(rule.content, "This is the rule content.");
    }

    #[test]
    fn test_parse_rule_file_no_title() {
        let content = r#"---
id: 550e8400-e29b-41d4-a716-446655440000
default: false
saved_at: 2024-01-01T00:00:00Z
---

Content without title.
"#;

        let rule = FileStore::parse_rule_file(content).unwrap();
        assert_eq!(rule.title, None);
        assert_eq!(rule.default, false);
        assert_eq!(rule.content, "Content without title.");
    }
}
