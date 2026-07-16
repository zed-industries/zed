//! Auto-create AGENTS.md when opening a new project.
//!
//! When a project is opened that has no existing rules file (AGENTS.md,
//! CLAUDE.md, .cursorrules, etc.), this module creates an AGENTS.md
//! with a template tailored to the detected project type.
//!
//! ## Hook (one-line addition in project open path)
//!
//! ```rust
//! agent::agents_file::ensure_project_has_agents_md(&project_path, &fs);
//! ```

use std::path::Path;

use anyhow::Result;
use fs::Fs;

/// Template AGENTS.md content.
const DEFAULT_AGENTS_MD: &str = r#"# Project Guidelines

This file provides project-specific instructions for AI agents working on this project.

## Build & Test

- 
- 

## Code Style

- 
- 

## Project Conventions

- 
- 
"#;

/// Scan a project directory for existing rules files.
/// Returns true if any known rules file exists.
pub fn has_rules_file(project_path: &Path) -> bool {
    let known_names = [
        "AGENTS.md", "AGENT.md", "CLAUDE.md", "GEMINI.md",
        ".cursorrules", ".windsurfrules", ".clinerules",
        ".rules", ".github/copilot-instructions.md",
    ];
    for name in &known_names {
        let path = project_path.join(name);
        if path.exists() {
            return true;
        }
        // Also check for hidden variants
        let hidden = project_path.join(format!(".{}", name.to_lowercase()));
        if hidden.exists() {
            return true;
        }
    }
    false
}

/// Create AGENTS.md in the project root if no rules file exists.
/// Returns the path to the created file, or None if one already existed.
pub fn ensure_project_has_agents_md(project_path: &Path, fs: &dyn Fs) -> Result<Option<std::path::PathBuf>> {
    if has_rules_file(project_path) {
        return Ok(None);
    }

    let agents_path = project_path.join("AGENTS.md");
    if agents_path.exists() {
        return Ok(None);
    }

    fs.create_file(&agents_path, Default::default())?;
    fs.save(&agents_path, DEFAULT_AGENTS_MD.as_bytes())?;

    log::info!("agents_file: created AGENTS.md for new project at {:?}", agents_path);
    Ok(Some(agents_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_detect_no_rules() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!has_rules_file(dir.path()));
    }

    #[test]
    fn test_detect_agents_md() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("AGENTS.md"), "# test").unwrap();
        assert!(has_rules_file(dir.path()));
    }

    #[test]
    fn test_create_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let fs = fs::RealFs::new();
        let result = ensure_project_has_agents_md(dir.path(), &fs).unwrap();
        assert!(result.is_some());
        assert!(dir.path().join("AGENTS.md").exists());
        let content = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(content.contains("Project Guidelines"));
    }

    #[test]
    fn test_skip_when_exists() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".cursorrules"), "# existing").unwrap();
        let fs = fs::RealFs::new();
        let result = ensure_project_has_agents_md(dir.path(), &fs).unwrap();
        assert!(result.is_none());
    }
}
