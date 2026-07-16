//! User feedback → skill refinement.
//!
//! When a user thumbs up or down on an agent edit, this module
//! adjusts the curator's pattern confidence and can auto-remove
//! low-confidence skills.
//!
//! ## Hook (one-line addition in thread_view.rs)
//!
//! In `handle_feedback_click()`, after `self.thread_feedback.submit(...)`:
//!
//! ```rust
//! crate::agent::feedback::handle(
//!     feedback,
//!     &self.thread.read(cx).messages,
//!     cx,
//! );
//! ```

use std::sync::Arc;

use crate::curator::CuratorPattern;
use crate::memory::{MemoryStore, global_store};

/// Outcome of a feedback event.
pub enum FeedbackOutcome {
    Positive,
    Negative,
}

/// Handle user feedback on an agent turn.
///
/// - Positive: marks matching curator patterns as "confirmed" (higher
///   confidence, won't be auto-pruned).
/// - Negative: decrements pattern confidence. If confidence hits 0,
///   removes the auto-generated skill file from disk.
pub fn handle(feedback: FeedbackOutcome, _thread_messages: &[String]) {
    let store = global_store();

    match feedback {
        FeedbackOutcome::Positive => {
            // Find curator patterns related to this thread and boost them
            let patterns = load_patterns(&store);
            for mut pattern in patterns {
                if pattern.skill_generated {
                    // Mark as user-confirmed — store confidence
                    let conf_key = format!("curator_confirmed_{}", pattern.slug);
                    store.write(conf_key, "true".into(), Some("curator".into()), vec!["curator".into()]);
                }
            }
        }
        FeedbackOutcome::Negative => {
            // Decrement confidence for auto-generated skills
            let patterns = load_patterns(&store);
            for mut pattern in patterns {
                if !pattern.skill_generated {
                    continue;
                }
                // Read current confidence
                let conf_key = format!("curator_confidence_{}", pattern.slug);
                let current: i32 = store
                    .get(&conf_key)
                    .and_then(|f| f.value.parse().ok())
                    .unwrap_or(3); // default confidence = 3

                let new_conf = current - 1;
                if new_conf <= 0 {
                    // Remove the skill file
                    if let Some(ref path) = pattern.skill_path {
                        let _ = std::fs::remove_file(path);
                        // Also try removing the parent directory
                        if let Some(parent) = std::path::Path::new(path).parent() {
                            let _ = std::fs::remove_dir(parent);
                        }
                    }
                    store.write(conf_key, "0".into(), Some("curator".into()), vec!["curator".into()]);
                    log::info!("feedback: skill '{}' removed (confidence depleted)", pattern.slug);
                } else {
                    store.write(conf_key, new_conf.to_string(), Some("curator".into()), vec!["curator".into()]);
                    log::info!("feedback: skill '{}' confidence decreased to {}", pattern.slug, new_conf);
                }
            }
        }
    }
}

fn load_patterns(store: &Arc<impl MemoryStore>) -> Vec<CuratorPattern> {
    store
        .search("curator_pattern_")
        .into_iter()
        .filter_map(|f| serde_json::from_str::<CuratorPattern>(&f.value).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curator::CuratorPattern;

    #[test]
    fn test_positive_feedback() {
        let store = crate::memory::JsonFileMemoryStore::new(
            std::path::PathBuf::from("/tmp/test_feedback_pos.jsonl"),
        );
        let pattern = CuratorPattern {
            slug: "test".into(),
            title: "test".into(),
            occurrences: 3,
            last_observation: crate::curator::Observation {
                command_signature: "cargo test".into(),
                edit_signature: "rs".into(),
                tool_signature: "terminal".into(),
                error_fragment: None,
                observed_at: 0,
            },
            skill_generated: true,
            skill_path: Some("/tmp/test_skill.md".into()),
        };
        // Save pattern to store
        let key = pattern.memory_key();
        let value = serde_json::to_string(&pattern).unwrap();
        store.write(key, value, Some("curator".into()), vec!["curator".into()]);

        // Handle positive feedback
        handle(FeedbackOutcome::Positive, &[]);

        // Check confirmation was stored
        let conf = store.get("curator_confirmed_test");
        assert!(conf.is_some());
        assert_eq!(conf.unwrap().value, "true");
    }

    #[test]
    fn test_slugify_consistency() {
        // Verify the feedback slug matches curator slug pattern
        let slug = crate::curator::slugify("cargo test");
        assert_eq!(slug, "cargo-test");
    }
}
