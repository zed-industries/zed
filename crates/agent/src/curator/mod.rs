//! Skill curator — observes thread outcomes and auto-generates skills
//! from repeating patterns.
//!
//! After each thread completes, the curator:
//! 1. Reads the thread messages
//! 2. Extracts terminal commands, file edits, and errors
//! 3. Matches against previously observed patterns
//! 4. When a pattern hits `min_occurrences`, writes a `SKILL.md`
//!
//! Skills are written to `~/.agents/skills/<slug>/SKILL.md` so the
//! native agent discovers them on the next thread.
//!
//! ## Config
//!
//! ```json
//! {
//!   "agent": {
//!     "curator": {
//!       "enabled": true,
//!       "min_occurrences": 2,
//!       "max_skills": 50
//!     }
//!   }
//! }
//! ```

use std::sync::Arc;

use anyhow::Result;
use gpui::{App, SharedString};
use serde::{Deserialize, Serialize};

use crate::curator::classifier::PatternClassifier;
use crate::curator::dedup::DedupChecker;
use crate::curator::extractor::PatternExtractor;
use crate::curator::writer::SkillWriter;
use crate::memory::{global_store, MemoryStore};

mod classifier;
mod dedup;
mod extractor;
mod writer;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Per-profile curator config. Merged from settings; defaults shown below.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorConfig {
    /// Master switch (default: true)
    pub enabled: bool,
    /// Minimum number of occurrences before a pattern becomes a skill
    /// (default: 2 — must be seen twice across sessions)
    pub min_occurrences: usize,
    /// Maximum number of auto-generated skills before the curator
    /// starts consolidating instead of creating (default: 50)
    pub max_skills: usize,
}

impl Default for CuratorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_occurrences: 2,
            max_skills: 50,
        }
    }
}

// ---------------------------------------------------------------------------
// Pattern tracking (persisted in memory store)
// ---------------------------------------------------------------------------

/// A raw observation from one thread. Multiple observations of the same
/// signature are aggregated into a `CuratorPattern`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// Comma-joined terminal commands extracted from the thread.
    pub command_signature: String,
    /// Comma-joined file extensions edited (e.g. "rs,toml").
    pub edit_signature: String,
    /// Comma-joined tool names used.
    pub tool_signature: String,
    /// Optional error message fragment (first 120 chars).
    pub error_fragment: Option<String>,
    /// Timestamp (epoch millis) when this was observed.
    pub observed_at: u64,
}

/// An aggregated pattern the curator watches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorPattern {
    /// Unique slug (derived from the observation signature).
    pub slug: String,
    /// Human-readable title.
    pub title: String,
    /// How many times this pattern has been observed.
    pub occurrences: usize,
    /// The most recent observation data.
    pub last_observation: Observation,
    /// Whether a skill has already been generated from this pattern.
    pub skill_generated: bool,
    /// Path to the generated SKILL.md (if any).
    pub skill_path: Option<String>,
}

impl CuratorPattern {
    fn memory_key(&self) -> String {
        format!("curator_pattern_{}", self.slug)
    }
}

// ---------------------------------------------------------------------------
// Curator
// ---------------------------------------------------------------------------

pub struct Curator {
    config: CuratorConfig,
    extractor: PatternExtractor,
    classifier: PatternClassifier,
    dedup: DedupChecker,
    writer: SkillWriter,
}

impl Curator {
    pub fn new() -> Self {
        Self {
            config: CuratorConfig::default(),
            extractor: PatternExtractor,
            classifier: PatternClassifier,
            dedup: DedupChecker,
            writer: SkillWriter,
        }
    }

    pub fn with_config(config: CuratorConfig) -> Self {
        Self {
            config,
            extractor: PatternExtractor,
            classifier: PatternClassifier,
            dedup: DedupChecker,
            writer: SkillWriter,
        }
    }

    /// Called after a thread completes. Runs extraction → classification
    /// → dedup → write pipeline. All errors are logged, never propagated
    /// (the curator must never crash the agent).
    pub fn observe(&self, messages: &[Arc<crate::Message>], cx: &App) {
        if !self.config.enabled {
            return;
        }

        let observation = match self.extractor.extract(messages) {
            Some(o) => o,
            None => return, // nothing to learn
        };

        // Check if this observation matches any existing pattern
        let store = global_store();
        let existing = self.load_patterns(&store);

        if let Some(mut pattern) = existing
            .into_iter()
            .find(|p| self.patterns_match(p, &observation))
        {
            pattern.occurrences += 1;
            pattern.last_observation = observation.clone();

            if pattern.occurrences >= self.config.min_occurrences
                && !pattern.skill_generated
            {
                // Dedup check: is this already covered by an existing skill?
                if self.dedup.is_covered(&pattern) {
                    log::info!(
                        "curator: pattern '{}' is already covered by a skill, skipping",
                        pattern.title
                    );
                    pattern.skill_generated = true; // mark so we don't re-check
                } else {
                    match self.writer.write_skill(&pattern, &observation) {
                        Ok(path) => {
                            log::info!("curator: wrote skill '{}' → {}", pattern.title, path);
                            pattern.skill_generated = true;
                            pattern.skill_path = Some(path);
                        }
                        Err(e) => {
                            log::warn!("curator: failed to write skill '{}': {e}", pattern.title);
                        }
                    }
                }
            }

            self.save_pattern(&store, &pattern);
        } else {
            // New pattern — seed it with 1 occurrence
            let slug = self.compute_slug(&observation);
            let title = self.classifier.classify_title(&observation);
            let pattern = CuratorPattern {
                slug,
                title,
                occurrences: 1,
                last_observation: observation,
                skill_generated: false,
                skill_path: None,
            };
            self.save_pattern(&store, &pattern);
        }
    }

    // ── helpers ──

    fn compute_slug(&self, obs: &Observation) -> String {
        let raw = format!("{}-{}-{}", obs.command_signature, obs.edit_signature, obs.tool_signature);
        slugify(&raw)
    }

    fn patterns_match(&self, pattern: &CuratorPattern, obs: &Observation) -> bool {
        // Two observations match if they share tool signature and
        // at least one of command or edit signature.
        pattern.last_observation.tool_signature == obs.tool_signature
            && (pattern.last_observation.command_signature == obs.command_signature
                || pattern.last_observation.edit_signature == obs.edit_signature)
    }

    fn load_patterns(&self, store: &Arc<impl MemoryStore>) -> Vec<CuratorPattern> {
        store
            .search("curator_pattern_")
            .into_iter()
            .filter_map(|f| serde_json::from_str::<CuratorPattern>(&f.value).ok())
            .collect()
    }

    fn save_pattern(&self, store: &Arc<impl MemoryStore>, pattern: &CuratorPattern) {
        let key = pattern.memory_key();
        let value = serde_json::to_string(pattern).unwrap_or_default();
        store.write(key, value, Some("curator".into()), vec!["curator".into()]);
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .filter(|c| *c != '-' || true) // keep hyphens
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("cargo test"), "cargo-test");
        assert_eq!(slugify("error[E0308]"), "error-e0308");
        assert_eq!(slugify("  spaced  "), "spaced");
    }

    #[test]
    fn test_config_defaults() {
        let cfg = CuratorConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.min_occurrences, 2);
        assert_eq!(cfg.max_skills, 50);
    }
}
