//! Classifies extracted observations into pattern categories and
//! generates human-readable titles for auto-generated skills.

use crate::curator::Observation;

pub enum PatternCategory {
    /// Repeating terminal command sequence (e.g. cargo test → fix → cargo test)
    Workflow,
    /// Common fix for a recurring error
    Fix,
    /// Project-specific convention
    Convention,
}

pub struct PatternClassifier;

impl PatternClassifier {
    /// Categorize an observation.
    pub fn classify(&self, obs: &Observation) -> PatternCategory {
        if obs.error_fragment.is_some() {
            return PatternCategory::Fix;
        }
        if !obs.command_signature.is_empty() && !obs.edit_signature.is_empty() {
            return PatternCategory::Workflow;
        }
        PatternCategory::Convention
    }

    /// Generate a human-readable title from an observation.
    pub fn classify_title(&self, obs: &Observation) -> String {
        if let Some(ref err) = obs.error_fragment {
            let short = err
                .split(|c: char| c == ':' || c == '(')
                .next()
                .unwrap_or(err)
                .trim();
            return format!("fix-{}", slugify_for_title(short));
        }

        let tools: Vec<&str> = obs.tool_signature.split(',').collect();

        if !obs.command_signature.is_empty() {
            // Use the first unique command as the title
            let cmd = obs.command_signature.split(',').next().unwrap_or("task");
            return format!("run-{}", slugify_for_title(cmd));
        }

        if !obs.edit_signature.is_empty() {
            let exts = obs.edit_signature.replace(',', "-");
            return format!("edit-{}-files", exts);
        }

        "general-pattern".to_string()
    }
}

fn slugify_for_title(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(commands: &str, edits: &str, tools: &str, error: Option<&str>) -> Observation {
        Observation {
            command_signature: commands.to_string(),
            edit_signature: edits.to_string(),
            tool_signature: tools.to_string(),
            error_fragment: error.map(|s| s.to_string()),
            observed_at: 0,
        }
    }

    #[test]
    fn test_classify_fix() {
        let o = obs("cargo test", "rs", "terminal,read_file", Some("error[E0308]"));
        assert!(matches!(PatternClassifier.classify(&o), PatternCategory::Fix));
    }

    #[test]
    fn test_classify_workflow() {
        let o = obs("cargo test", "rs", "terminal,edit_file", None);
        assert!(matches!(PatternClassifier.classify(&o), PatternCategory::Workflow));
    }

    #[test]
    fn test_classify_convention() {
        let o = obs("", "", "read_file", None);
        assert!(matches!(PatternClassifier.classify(&o), PatternCategory::Convention));
    }

    #[test]
    fn test_title_from_error() {
        let o = obs("cargo build", "rs", "terminal", Some("error[E0308]: mismatched types"));
        let title = PatternClassifier.classify_title(&o);
        assert!(title.contains("error"));
    }

    #[test]
    fn test_title_from_command() {
        let o = obs("cargo test", "rs", "terminal", None);
        let title = PatternClassifier.classify_title(&o);
        assert_eq!(title, "run-cargo-test");
    }

    #[test]
    fn test_title_from_edit() {
        let o = obs("", "rs,ts", "edit_file", None);
        let title = PatternClassifier.classify_title(&o);
        assert_eq!(title, "edit-rs-ts-files");
    }
}
