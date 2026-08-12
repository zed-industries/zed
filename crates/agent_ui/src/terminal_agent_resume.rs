//! Pure decision logic for reviving sleeping terminal-agent sessions.
//!
//! Mirrors Orca's shared agent-session-resume module: a sleeping session (an
//! agent terminal whose process has ended) is resumed exactly once by
//! relaunching the harness with its resume locator, fenced by a claim key so
//! duplicate or older records never double-resume. These functions are pure
//! and deterministic so the fencing and staleness rules are unit-testable in
//! isolation before they are wired into the agent panel.

use std::path::Path;

use chrono::{DateTime, Duration, Utc};
use collections::HashMap;

use crate::terminal_thread_metadata_store::TerminalAgentProfile;

/// How long a non-durable sleeping record may sit before it is treated as
/// stale and cleared rather than resumed. Mirrors Orca's 18-minute window.
const SESSION_STALENESS: Duration = Duration::minutes(18);

/// Maximum accepted length of a provider session id fallback.
const SESSION_ID_MAX_LENGTH: usize = 512;

/// Rejects provider session ids that are unsafe to place on a command line:
/// empty, overlong, a leading `-` (which a CLI would parse as a flag), or
/// containing control characters.
pub fn normalize_session_id(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.len() > SESSION_ID_MAX_LENGTH
        || trimmed.starts_with('-')
        || trimmed.chars().any(char::is_control)
    {
        return None;
    }
    Some(trimmed)
}

/// Derives the fencing identity for a revivable agent session: the unique
/// combination of worktree, agent profile, and Zed-controlled resume path.
/// Exactly one resume is allowed per claim key.
pub fn session_claim_key(
    worktree_id: &str,
    profile: TerminalAgentProfile,
    resume_path: &Path,
) -> String {
    format!(
        "{worktree_id}\u{0}{}\u{0}{}",
        profile.label(),
        resume_path.to_string_lossy()
    )
}

/// Builds the argv that resumes a sleeping agent session. For OMP the
/// Zed-controlled resume path is preferred; the provider session id is the
/// fallback. Returns `None` when neither locator is available or the session
/// id fails validation.
pub fn get_agent_resume_argv(
    profile: TerminalAgentProfile,
    resume_path: Option<&Path>,
    session_id: Option<&str>,
) -> Option<Vec<String>> {
    match profile {
        TerminalAgentProfile::Omp => {
            let locator = resume_path
                .map(|path| path.to_string_lossy().into_owned())
                .or_else(|| session_id.and_then(normalize_session_id).map(str::to_owned))?;
            Some(vec!["omp".into(), "--resume".into(), locator])
        }
    }
}

/// A sleeping-session record reduced to the fields the fencing and staleness
/// rules need. Production records are mapped onto this shape for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct SleepingSessionRecord {
    pub claim_key: String,
    /// Whether the record is invalid (stale, passive evidence) and must be
    /// cleared rather than resumed.
    pub invalid: bool,
    pub updated_at: DateTime<Utc>,
}

/// A sleeping-session record is invalid (cleared, not resumed) when it lacks a
/// capture origin yet holds a durable boundary, or when it has not reached a
/// durable boundary and sat captured without an update for longer than the
/// staleness window.
pub fn is_invalid_session_record(
    has_origin: bool,
    is_durable_boundary: bool,
    captured_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
) -> bool {
    (!has_origin && is_durable_boundary)
        || (!is_durable_boundary
            && captured_at.signed_duration_since(updated_at) > SESSION_STALENESS)
}

/// A sleeping record that sat un-resumed since it was captured for longer than
/// the staleness window is stale and must be cleared rather than resumed, so
/// old dormant sessions do not spuriously resurrect on a later activation.
/// Mirrors Orca's 18-minute dormancy timeout for sleeping sessions.
pub fn is_stale_sleeping_record(created_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    now.signed_duration_since(created_at) > SESSION_STALENESS
}

/// Selects which sleeping-session records to resume: at most one per claim
/// key, preferring the newest (latest `updated_at`), and skipping invalid
/// records.
pub fn select_sessions_to_resume(records: &[SleepingSessionRecord]) -> Vec<&SleepingSessionRecord> {
    let mut newest_by_claim_key: HashMap<&str, &SleepingSessionRecord> = HashMap::default();
    for record in records {
        if record.invalid {
            continue;
        }
        let replace = match newest_by_claim_key.get(record.claim_key.as_str()) {
            Some(current) => current.updated_at < record.updated_at,
            None => true,
        };
        if replace {
            newest_by_claim_key.insert(record.claim_key.as_str(), record);
        }
    }
    newest_by_claim_key.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(key: &str, updated_at: &DateTime<Utc>, invalid: bool) -> SleepingSessionRecord {
        SleepingSessionRecord {
            claim_key: key.to_string(),
            invalid,
            updated_at: *updated_at,
        }
    }

    #[test]
    fn test_get_agent_resume_argv_prefers_resume_path() {
        let argv = get_agent_resume_argv(
            TerminalAgentProfile::Omp,
            Some(Path::new("/tmp/omp-zed/session")),
            Some("session_123"),
        );
        assert_eq!(
            argv,
            Some(vec!["omp".into(), "--resume".into(), "/tmp/omp-zed/session".into()])
        );
    }

    #[test]
    fn test_get_agent_resume_argv_falls_back_to_session_id() {
        let argv = get_agent_resume_argv(
            TerminalAgentProfile::Omp,
            None,
            Some("session_123"),
        );
        assert_eq!(
            argv,
            Some(vec!["omp".into(), "--resume".into(), "session_123".into()])
        );
    }

    #[test]
    fn test_get_agent_resume_argv_returns_none_without_a_locator() {
        assert_eq!(get_agent_resume_argv(TerminalAgentProfile::Omp, None, None), None);
    }

    #[test]
    fn test_normalize_session_id_rejects_unsafe_values() {
        assert_eq!(normalize_session_id("session_123"), Some("session_123"));
        assert_eq!(normalize_session_id("  session_123  "), Some("session_123"));
        assert_eq!(normalize_session_id(""), None);
        assert_eq!(normalize_session_id("   "), None);
        assert_eq!(normalize_session_id("-malicious"), None);
        assert_eq!(normalize_session_id(&"x".repeat(513)), None);
        assert_eq!(normalize_session_id("has\0control"), None);
    }

    #[test]
    fn test_session_claim_key_combines_worktree_profile_and_resume_path() {
        let key = session_claim_key(
            "/repo",
            TerminalAgentProfile::Omp,
            Path::new("/tmp/omp-zed/session"),
        );
        assert_eq!(key, "/repo\u{0}omp\u{0}/tmp/omp-zed/session");
    }

    #[test]
    fn test_is_invalid_session_record_rules() {
        let now = Utc::now();
        let twenty_min_ago = now - Duration::minutes(20);
        let five_min_ago = now - Duration::minutes(5);

        // No origin but a durable boundary: invalid.
        assert!(is_invalid_session_record(false, true, now, now));
        // Origin and durable boundary: valid.
        assert!(!is_invalid_session_record(true, true, now, now));
        // Non-durable and stale capture: invalid.
        assert!(is_invalid_session_record(true, false, now, twenty_min_ago));
        // Non-durable but fresh: valid.
        assert!(!is_invalid_session_record(true, false, now, five_min_ago));
    }

    #[test]
    fn test_is_stale_sleeping_record_clears_dormant_records() {
        let now = Utc::now();
        let twenty_min_ago = now - Duration::minutes(20);
        let five_min_ago = now - Duration::minutes(5);

        // A sleeping record captured more than the 18-minute staleness window
        // ago is stale (cleared, not resumed).
        assert!(is_stale_sleeping_record(twenty_min_ago, now));
        // A fresh sleeping record within the window is not stale.
        assert!(!is_stale_sleeping_record(five_min_ago, now));
        // A record captured exactly at the boundary is not stale.
        assert!(!is_stale_sleeping_record(now, now));
    }

    #[test]
    fn test_select_sessions_to_resume_newest_wins_per_claim_key() {
        let now = Utc::now();
        let older = now - Duration::minutes(5);
        let newer = now - Duration::minutes(1);
        let records = vec![
            record("key-a", &older, false),
            record("key-a", &newer, false), // newer wins for key-a
            record("key-b", &older, false),
            record("key-b-invalid", &newer, true), // skipped: invalid
        ];

        let selected = select_sessions_to_resume(&records);
        let mut keys: Vec<&str> = selected.iter().map(|r| r.claim_key.as_str()).collect();
        keys.sort_unstable();
        assert_eq!(keys, vec!["key-a", "key-b"]);

        let newest_a = selected
            .iter()
            .find(|r| r.claim_key == "key-a")
            .expect("key-a selected");
        assert_eq!(newest_a.updated_at, newer);
        assert!(!selected.iter().any(|r| r.claim_key == "key-b-invalid"));
    }
}