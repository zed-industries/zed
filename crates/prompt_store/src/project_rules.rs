//! Project-specific rules parsing and execution.
//!
//! This module handles parsing `.rules` files in project directories to check if they
//! contain command rule frontmatter. If they do, the commands are executed to generate
//! dynamic content. Otherwise, the file content is used as-is.

use anyhow::{Context as _, Result};

use crate::command_executor;
use crate::file_store::{CommandConfig, RuleType};

/// A single parsed rule from a project rules file
#[derive(Debug, Clone)]
pub struct ParsedProjectRule {
    pub rule_type: RuleType,
    pub command: Option<CommandConfig>,
    pub content: String,
}

impl ParsedProjectRule {
    pub fn is_command(&self) -> bool {
        self.rule_type == RuleType::Command
    }
}

/// Parse a .rules file to extract all rules (supports multiple frontmatters)
pub fn parse_project_rules_file(content: &str) -> Vec<ParsedProjectRule> {
    // Try to parse multiple rules separated by frontmatter blocks
    match try_parse_multiple_rules(content) {
        Ok(rules) if !rules.is_empty() => rules,
        _ => {
            // No frontmatter found - treat entire file as single static rule
            vec![ParsedProjectRule {
                rule_type: RuleType::Static,
                command: None,
                content: content.to_string(),
            }]
        }
    }
}

/// Parse multiple rules from a file (each with its own frontmatter)
fn try_parse_multiple_rules(content: &str) -> Result<Vec<ParsedProjectRule>> {
    let mut rules = Vec::new();
    let mut remaining = content;

    loop {
        // Look for start of frontmatter
        if !remaining.starts_with("---\n") && !remaining.starts_with("---\r\n") {
            // No more frontmatter blocks
            if !remaining.trim().is_empty() && rules.is_empty() {
                // Content exists but no frontmatter - treat as static
                return Ok(vec![ParsedProjectRule {
                    rule_type: RuleType::Static,
                    command: None,
                    content: remaining.to_string(),
                }]);
            }
            break;
        }

        // Parse one rule
        match try_parse_single_rule(remaining)? {
            Some((rule, rest)) => {
                rules.push(rule);
                remaining = rest.trim_start();
            }
            None => break,
        }
    }

    Ok(rules)
}

/// Parse a single rule starting from the beginning of the string
fn try_parse_single_rule(content: &str) -> Result<Option<(ParsedProjectRule, &str)>> {
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return Ok(None);
    }

    // Skip opening ---
    let content_after_start = if let Some(stripped) = content.strip_prefix("---\r\n") {
        stripped
    } else if let Some(stripped) = content.strip_prefix("---\n") {
        stripped
    } else {
        return Ok(None);
    };

    // Find closing ---
    let end_marker_idx = content_after_start
        .find("\n---\n")
        .or_else(|| content_after_start.find("\n---\r\n"));

    let Some(end_marker_idx) = end_marker_idx else {
        return Ok(None);
    };

    let frontmatter_str = &content_after_start[..end_marker_idx];

    // Parse frontmatter
    let frontmatter: ProjectRuleFrontmatter = serde_yaml::from_str(frontmatter_str)
        .context("Failed to parse project rules frontmatter")?;

    // Get content after frontmatter
    let content_start = if content_after_start[end_marker_idx..].starts_with("\n---\r\n") {
        end_marker_idx + 6
    } else {
        end_marker_idx + 5
    };

    let after_frontmatter = &content_after_start[content_start..];

    // Find the next frontmatter block or use rest of file
    let (rule_content, remaining) = if let Some(next_frontmatter_pos) = after_frontmatter
        .find("\n---\n")
        .or_else(|| after_frontmatter.find("\n---\r\n"))
    {
        // There's another rule after this one
        let content = &after_frontmatter[..next_frontmatter_pos];
        let remaining = &after_frontmatter[next_frontmatter_pos + 1..]; // +1 to skip the newline
        (content.trim().to_string(), remaining)
    } else {
        // This is the last rule
        (after_frontmatter.trim().to_string(), "")
    };

    let rule = ParsedProjectRule {
        rule_type: frontmatter.rule_type,
        command: frontmatter.command,
        content: rule_content,
    };

    Ok(Some((rule, remaining)))
}

/// Frontmatter structure for project rules files
#[derive(Debug, Clone, serde::Deserialize)]
struct ProjectRuleFrontmatter {
    #[serde(default, rename = "type")]
    rule_type: RuleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<CommandConfig>,
}

/// Execute a command rule and return the combined content
pub async fn execute_project_command_rule(
    config: &CommandConfig,
    _prefix_content: String,
    title: &str,
) -> Result<String> {
    let result = command_executor::execute_command(config).await?;

    if let Some(output) = result.output_for_rule() {
        // Use only command output, ignore prefix content
        Ok(output)
    } else {
        // Command failed, log stderr
        if !result.stderr.is_empty() {
            log::warn!("Project command rule '{}' failed: {}", title, result.stderr);
        }
        anyhow::bail!("Command execution failed")
    }
}

/// Process a project rules file: parse all rules and execute command rules
pub async fn process_project_rules_file(content: &str, file_path: &str) -> Result<Vec<String>> {
    let rules = parse_project_rules_file(content);
    let mut outputs = Vec::new();

    for (index, rule) in rules.iter().enumerate() {
        let rule_name = format!("{}#{}", file_path, index + 1);

        let output = if rule.is_command() {
            if let Some(config) = &rule.command {
                log::info!("Executing command rule from project file: {}", rule_name);
                match execute_project_command_rule(config, rule.content.clone(), &rule_name).await {
                    Ok(result) => result,
                    Err(e) => {
                        log::warn!("Failed to execute command rule {}: {}", rule_name, e);
                        continue; // Skip failed command rules
                    }
                }
            } else {
                // Command rule without config - use static content
                rule.content.clone()
            }
        } else {
            // Static rule
            rule.content.clone()
        };

        outputs.push(output);
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_static_content() {
        let content = "This is just plain text rules content.";
        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_type, RuleType::Static);
        assert_eq!(result[0].content, content);
    }

    #[test]
    fn test_parse_single_static_frontmatter() {
        let content = r#"---
type: static
---

Static rule content here."#;

        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_type, RuleType::Static);
        assert_eq!(result[0].content, "Static rule content here.");
    }

    #[test]
    fn test_parse_single_command_rule() {
        let content = r#"---
type: command
command:
  cmd: git
  args: ["status", "--short"]
  timeout_seconds: 5
  max_output_bytes: 10000
  on_new_chat: true
---

This text appears before command output."#;

        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_type, RuleType::Command);
        assert!(result[0].command.is_some());

        let config = result[0].command.as_ref().unwrap();
        assert_eq!(config.cmd, "git");
        assert_eq!(config.args, vec!["status", "--short"]);
        assert_eq!(config.timeout_seconds, 5);
        assert_eq!(config.max_output_bytes, 10000);
        assert!(config.on_new_chat);
        assert_eq!(
            result[0].content,
            "This text appears before command output."
        );
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "Just regular rules without frontmatter";
        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_type, RuleType::Static);
        assert_eq!(result[0].content, content);
    }

    #[test]
    fn test_parse_incomplete_frontmatter() {
        let content = "---\nthis is not valid yaml\n";
        let result = parse_project_rules_file(content);

        // Should fall back to static content
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_type, RuleType::Static);
    }

    #[test]
    fn test_parse_multiple_rules() {
        let content = r#"---
type: static
---

First static rule content.

---
type: command
command:
  cmd: pwd
  on_new_chat: true
---

Second rule is a command.

---
type: static
---

Third static rule."#;

        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 3);

        // First rule - static
        assert_eq!(result[0].rule_type, RuleType::Static);
        assert_eq!(result[0].content, "First static rule content.");

        // Second rule - command
        assert_eq!(result[1].rule_type, RuleType::Command);
        assert!(result[1].command.is_some());
        assert_eq!(result[1].command.as_ref().unwrap().cmd, "pwd");
        assert_eq!(result[1].content, "Second rule is a command.");

        // Third rule - static
        assert_eq!(result[2].rule_type, RuleType::Static);
        assert_eq!(result[2].content, "Third static rule.");
    }

    #[test]
    fn test_parse_mixed_rules_no_content_between() {
        let content = r#"---
type: command
command:
  cmd: echo
  args: ["hello"]
---
Command rule content
---
type: static
---
Static rule content"#;

        let result = parse_project_rules_file(content);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].rule_type, RuleType::Command);
        assert_eq!(result[0].content, "Command rule content");
        assert_eq!(result[1].rule_type, RuleType::Static);
        assert_eq!(result[1].content, "Static rule content");
    }

    #[gpui::test]
    async fn test_execute_command_rule() {
        let config = CommandConfig {
            cmd: "echo".to_string(),
            args: vec!["test output".to_string()],
            timeout_seconds: 5,
            max_output_bytes: 10_000,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let prefix = "Prefix content".to_string();
        let result = execute_project_command_rule(&config, prefix, "test.rules").await;

        assert!(result.is_ok());
        let output = result.unwrap();
        // Command output replaces prefix content, so prefix should not be present
        assert!(!output.contains("Prefix content"));
        assert!(output.contains("test output"));
    }

    #[gpui::test]
    async fn test_process_static_file() {
        let content = "Static rules content";
        let result = process_project_rules_file(content, ".rules").await;

        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], content);
    }

    #[gpui::test]
    async fn test_process_multiple_rules() {
        let content = r#"---
type: static
---

Static rule 1

---
type: command
command:
  cmd: echo
  args: ["test"]
  on_new_chat: true
---

Prefix for command

---
type: static
---

Static rule 2"#;

        let result = process_project_rules_file(content, ".rules").await;

        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 3);
        assert_eq!(outputs[0], "Static rule 1");
        assert!(outputs[1].contains("test")); // Command output
        assert_eq!(outputs[2], "Static rule 2");
    }
}
