//! Deduplication checker — ensures the curator doesn't generate skills
//! that already exist (either user-written or previously auto-generated)
//! in the skills directory.

use std::path::Path;

use crate::curator::CuratorPattern;

pub struct DedupChecker;

impl DedupChecker {
    /// Returns `true` if a skill covering this pattern already exists
    /// in `~/.agents/skills/` or was previously generated.
    ///
    /// Checks two sources:
    /// 1. The pattern's own `skill_generated` flag (fast path)
    /// 2. Existing `SKILL.md` files on disk (cross-session persistence)
    pub fn is_covered(&self, pattern: &CuratorPattern) -> bool {
        // Fast path: already generated
        if pattern.skill_generated {
            return true;
        }

        // Check on-disk skills by scanning the slug in the title
        let skills_dir = self.skills_dir();
        if let Ok(entries) = std::fs::read_dir(&skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let skill_file = path.join("SKILL.md");
                    if skill_file.exists() {
                        if let Ok(content) = std::fs::read_to_string(&skill_file) {
                            // Check if the frontmatter name or description overlaps
                            if self.contains_overlap(&content, pattern) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Heuristic overlap check: does the existing skill's name or
    /// description mention the same tools/commands as our pattern?
    fn contains_overlap(&self, skill_content: &str, pattern: &CuratorPattern) -> bool {
        let lower = skill_content.to_lowercase();
        let title_lower = pattern.title.to_lowercase().replace('-', " ");

        // Check if the title's key terms appear in the skill
        let terms: Vec<&str> = title_lower.split_whitespace().collect();
        let match_count = terms
            .iter()
            .filter(|t| t.len() > 3 && lower.contains(*t))
            .count();

        // If 2+ significant terms match, consider it overlapping
        match_count >= 2
    }

    fn skills_dir(&self) -> Box<Path> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Path::new(&home).join(".agents").join("skills").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_already_generated() {
        let pattern = CuratorPattern {
            slug: "test".into(),
            title: "run-cargo-test".into(),
            occurrences: 3,
            last_observation: crate::curator::Observation {
                command_signature: "cargo test".into(),
                edit_signature: String::new(),
                tool_signature: "terminal".into(),
                error_fragment: None,
                observed_at: 0,
            },
            skill_generated: true,
            skill_path: Some("/tmp/skill.md".into()),
        };
        assert!(DedupChecker.is_covered(&pattern));
    }

    #[test]
    fn test_overlap_detected() {
        let skill_content = "---\nname: run-cargo-test\n---\n\nRun `cargo test` after editing Rust files.";
        let pattern = CuratorPattern {
            slug: "foo".into(),
            title: "run-cargo-test".into(),
            occurrences: 2,
            last_observation: crate::curator::Observation {
                command_signature: "cargo test".into(),
                edit_signature: "rs".into(),
                tool_signature: "terminal".into(),
                error_fragment: None,
                observed_at: 0,
            },
            skill_generated: false,
            skill_path: None,
        };
        assert!(DedupChecker.contains_overlap(skill_content, &pattern));
    }

    #[test]
    fn test_no_overlap() {
        let skill_content = "---\nname: deploy-app\n---\n\nDeploy the application to staging.";
        let pattern = CuratorPattern {
            slug: "bar".into(),
            title: "run-cargo-test".into(),
            occurrences: 2,
            last_observation: crate::curator::Observation {
                command_signature: "cargo test".into(),
                edit_signature: "rs".into(),
                tool_signature: "terminal".into(),
                error_fragment: None,
                observed_at: 0,
            },
            skill_generated: false,
            skill_path: None,
        };
        assert!(!DedupChecker.contains_overlap(skill_content, &pattern));
    }
}
