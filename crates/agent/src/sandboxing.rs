//! Agent-side glue for the [`sandbox`] crate.
//!
//! Centralizes the "should agent-run terminal commands be sandboxed for this
//! process?" check so the system prompt, the terminal tool, and any other
//! caller see the same answer (and so the `target_os` gate lives in one
//! place instead of scattered across the agent crate).
//!
//! The current policy is: enabled iff we're on macOS *and* the user has the
//! `sandboxing` feature flag turned on. There's deliberately no settings or
//! env-var override yet — the flag is the only switch.
//!
//! On non-macOS hosts we don't have a sandbox integration today, so this
//! returns `false` regardless of the flag.
//!
//! Naming note: this module is about agent terminal sandboxing specifically.
//! Other agent operations (e.g. file edits) are gated separately.

use feature_flags::{FeatureFlagAppExt as _, SandboxingFeatureFlag};
use gpui::App;
use std::path::PathBuf;

/// Whether agent-run terminal commands should be wrapped in an OS-level
/// sandbox for this process. See module docs for the policy.
pub(crate) fn sandboxing_enabled(cx: &App) -> bool {
    cfg!(target_os = "macos") && cx.has_flag::<SandboxingFeatureFlag>()
}

/// A request for elevated sandbox permissions for a single terminal command.
///
/// Built from the model-controlled `terminal` tool input after the user has
/// authorized the baseline command. All paths here have already been resolved
/// to absolute, canonicalized paths by the caller — never raw, model-provided
/// strings, and never the model-controlled working directory.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SandboxRequest {
    /// Allow outbound network access for this command.
    pub network: bool,
    /// Allow unrestricted filesystem writes (the broad escape hatch).
    pub allow_fs_write_all: bool,
    /// Concrete paths the command needs to write to. Each grants its whole
    /// subtree. These are never globs — write access is always a concrete path subtree
    pub write_paths: Vec<PathBuf>,
}

impl SandboxRequest {
    /// Whether this request asks for anything beyond the default sandbox
    /// scope, and therefore needs user approval.
    pub fn needs_escalation(&self) -> bool {
        self.network || self.allow_fs_write_all || !self.write_paths.is_empty()
    }
}

/// In-memory record of the sandbox permissions the user approved "for the
/// rest of the conversation".
///
/// Lives on the `Thread` and is shared (via `Rc<RefCell<…>>`) with each tool
/// call's event stream so a later command requesting an already-granted
/// permission can skip the approval prompt. This is deliberately **never**
/// persisted to settings — it dies with the conversation, unlike the global
/// `always_allow` tool-permission rules.
#[derive(Default)]
pub(crate) struct ConversationSandboxGrants {
    network: bool,
    allow_fs_write_all: bool,
    /// Canonicalized paths granted write access for the conversation. Each
    /// covers its whole subtree; redundant children are pruned on insert.
    write_paths: Vec<PathBuf>,
}

impl ConversationSandboxGrants {
    /// Whether everything `request` asks for has already been granted for the
    /// conversation, so the command can run without prompting again.
    ///
    /// Write coverage is pure subtree containment: every
    /// requested path must sit under some granted path. This is fully
    /// deterministic and never widens scope, because grants are concrete
    /// paths rather than globs.
    pub fn covers(&self, request: &SandboxRequest) -> bool {
        if request.network && !self.network {
            return false;
        }
        if request.allow_fs_write_all && !self.allow_fs_write_all {
            return false;
        }
        // A conversation-wide all-access write grant covers any concrete
        // write request.
        if self.allow_fs_write_all {
            return true;
        }
        request.write_paths.iter().all(|requested| {
            self.write_paths
                .iter()
                .any(|granted| requested.starts_with(granted))
        })
    }

    /// Record everything in `request` as granted for the rest of the
    /// conversation, pruning paths that become redundant.
    pub fn record(&mut self, request: &SandboxRequest) {
        self.network |= request.network;
        self.allow_fs_write_all |= request.allow_fs_write_all;
        for path in &request.write_paths {
            add_write_path(&mut self.write_paths, path);
        }
    }

    /// Compute the effective sandbox permissions to actually enforce for a
    /// command: the union of everything granted for the conversation and
    /// what this specific command requested.
    ///
    /// This is what makes a conversation grant "stick": every sandboxed
    /// command applies the accumulated grants, so the model can write to a
    /// previously approved path without re-requesting it. Passing the current `request` in
    /// also covers "allow once" grants, which are enforced for this command
    /// without being recorded for the conversation.
    pub fn effective(&self, request: &SandboxRequest) -> SandboxRequest {
        let mut write_paths = self.write_paths.clone();
        for path in &request.write_paths {
            add_write_path(&mut write_paths, path);
        }
        SandboxRequest {
            network: self.network || request.network,
            allow_fs_write_all: self.allow_fs_write_all || request.allow_fs_write_all,
            write_paths,
        }
    }
}

/// Insert `path` into a set of write-grant subtrees, keeping it minimal:
/// a no-op if already covered by a broader grant, otherwise added with any
/// now-subsumed child grants pruned.
fn add_write_path(write_paths: &mut Vec<PathBuf>, path: &std::path::Path) {
    if write_paths.iter().any(|granted| path.starts_with(granted)) {
        return;
    }
    write_paths.retain(|granted| !granted.starts_with(path));
    write_paths.push(path.to_path_buf());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(network: bool, all: bool, paths: &[&str]) -> SandboxRequest {
        SandboxRequest {
            network,
            allow_fs_write_all: all,
            write_paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    #[test]
    fn empty_grants_cover_nothing() {
        let grants = ConversationSandboxGrants::default();
        assert!(!grants.covers(&request(true, false, &[])));
        assert!(!grants.covers(&request(false, true, &[])));
        assert!(!grants.covers(&request(false, false, &["/tmp/build"])));
    }

    #[test]
    fn subtree_containment_covers_children() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        // Exact match and any descendant are covered.
        assert!(grants.covers(&request(false, false, &["/tmp/build"])));
        assert!(grants.covers(&request(false, false, &["/tmp/build/cache"])));
        // A sibling / parent is not.
        assert!(!grants.covers(&request(false, false, &["/tmp/other"])));
        assert!(!grants.covers(&request(false, false, &["/tmp"])));
    }

    #[test]
    fn record_prunes_redundant_children() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build/cache"]));
        grants.record(&request(false, false, &["/tmp/build"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn record_keeps_existing_broader_grant() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));
        grants.record(&request(false, false, &["/tmp/build/cache"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn all_access_covers_any_concrete_write() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, true, &[]));
        assert!(grants.covers(&request(false, false, &["/anywhere/at/all"])));
        // But not network, which wasn't granted.
        assert!(!grants.covers(&request(true, false, &[])));
    }

    #[test]
    fn network_grant_tracked_independently() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(true, false, &[]));
        assert!(grants.covers(&request(true, false, &[])));
        assert!(!grants.covers(&request(true, false, &["/tmp/build"])));
    }

    #[test]
    fn effective_applies_conversation_grants_to_empty_request() {
        // The core fix: a command that requests nothing still gets the
        // conversation's granted write paths in its enforced policy.
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        let effective = grants.effective(&request(false, false, &[]));
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn effective_unions_grants_with_once_request() {
        // An "allow once" path (passed via `request`, never recorded) is
        // enforced for this command alongside the standing grants.
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(true, false, &["/tmp/build"]));

        let effective = grants.effective(&request(false, false, &["/tmp/once"]));
        assert!(effective.network);
        assert_eq!(
            effective.write_paths,
            vec![PathBuf::from("/tmp/build"), PathBuf::from("/tmp/once")]
        );
    }

    #[test]
    fn effective_dedupes_request_already_covered_by_grant() {
        let mut grants = ConversationSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        let effective = grants.effective(&request(false, false, &["/tmp/build/cache"]));
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }
}
