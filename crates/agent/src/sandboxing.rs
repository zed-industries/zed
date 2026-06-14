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

use agent_settings::SandboxPermissions;
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
    /// Run the command fully outside the sandbox.
    pub unsandboxed: bool,
    /// Concrete paths the command needs to write to. Each grants its whole
    /// subtree. These are never globs — write access is always a concrete path subtree
    pub write_paths: Vec<PathBuf>,
}

impl SandboxRequest {
    /// Whether this request asks for anything beyond the default sandbox
    /// scope, and therefore needs user approval.
    pub fn needs_escalation(&self) -> bool {
        self.network || self.allow_fs_write_all || self.unsandboxed || !self.write_paths.is_empty()
    }
}

/// In-memory record of the sandbox permissions the user approved "for the
/// rest of the thread".
///
/// Lives on the `Thread` and is shared (via `Rc<RefCell<…>>`) with each tool
/// call's event stream so a later command requesting an already-granted
/// permission can skip the approval prompt. Persistent "allow always" grants
/// are stored separately in [`SandboxPermissions`].
#[derive(Default)]
pub(crate) struct ThreadSandboxGrants {
    network: bool,
    allow_fs_write_all: bool,
    unsandboxed: bool,
    /// Canonicalized paths granted write access for the thread. Each covers its
    /// whole subtree; redundant children are pruned on insert.
    write_paths: Vec<PathBuf>,
}

impl ThreadSandboxGrants {
    /// Whether the union of thread grants and persistent "allow always" grants
    /// covers everything `request` asks for, so the command can run without
    /// prompting again.
    ///
    /// Write coverage is pure subtree containment: every requested path must
    /// sit under some granted path. This is fully deterministic and never
    /// widens scope, because grants are concrete paths rather than globs.
    pub fn covers_with_persistent(
        &self,
        request: &SandboxRequest,
        persistent: &SandboxPermissions,
    ) -> bool {
        if request.unsandboxed {
            return self.unsandboxed || persistent.allow_unsandboxed;
        }
        if request.network && !(self.network || persistent.allow_network) {
            return false;
        }
        if request.allow_fs_write_all && !(self.allow_fs_write_all || persistent.allow_fs_write_all)
        {
            return false;
        }
        // A full-access write grant covers any concrete write request.
        if self.allow_fs_write_all || persistent.allow_fs_write_all {
            return true;
        }
        request.write_paths.iter().all(|requested| {
            util::paths::path_within_subtree(
                requested,
                self.write_paths
                    .iter()
                    .chain(persistent.write_paths.iter())
                    .map(PathBuf::as_path),
            )
        })
    }

    /// Record everything in `request` as granted for the rest of the thread,
    /// pruning paths that become redundant.
    pub fn record(&mut self, request: &SandboxRequest) {
        self.network |= request.network;
        self.allow_fs_write_all |= request.allow_fs_write_all;
        self.unsandboxed |= request.unsandboxed;
        for path in &request.write_paths {
            util::paths::insert_subtree(&mut self.write_paths, path.clone());
        }
    }

    /// Compute the effective sandbox permissions to enforce for a command: the
    /// union of persistent "allow always" grants, thread grants, and this
    /// specific command's request.
    ///
    /// This is what makes standing grants "stick": every sandboxed command
    /// applies the accumulated grants, so the model can write to a previously
    /// approved path without re-requesting it. Passing the current `request` in
    /// also covers "allow once" grants, which are enforced for this command
    /// without being recorded for the thread.
    pub fn effective_with_persistent(
        &self,
        request: &SandboxRequest,
        persistent: &SandboxPermissions,
    ) -> SandboxRequest {
        let mut write_paths = persistent.write_paths.clone();
        for path in self.write_paths.iter().chain(request.write_paths.iter()) {
            util::paths::insert_subtree(&mut write_paths, path.clone());
        }
        SandboxRequest {
            network: persistent.allow_network || self.network || request.network,
            allow_fs_write_all: persistent.allow_fs_write_all
                || self.allow_fs_write_all
                || request.allow_fs_write_all,
            unsandboxed: request.unsandboxed,
            write_paths,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(network: bool, all: bool, paths: &[&str]) -> SandboxRequest {
        SandboxRequest {
            network,
            allow_fs_write_all: all,
            unsandboxed: false,
            write_paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    fn unsandboxed_request() -> SandboxRequest {
        SandboxRequest {
            network: false,
            allow_fs_write_all: false,
            unsandboxed: true,
            write_paths: Vec::new(),
        }
    }

    fn covers(grants: &ThreadSandboxGrants, request: &SandboxRequest) -> bool {
        grants.covers_with_persistent(request, &SandboxPermissions::default())
    }

    fn effective(grants: &ThreadSandboxGrants, request: &SandboxRequest) -> SandboxRequest {
        grants.effective_with_persistent(request, &SandboxPermissions::default())
    }

    #[test]
    fn empty_grants_cover_nothing() {
        let grants = ThreadSandboxGrants::default();
        assert!(!covers(&grants, &request(true, false, &[])));
        assert!(!covers(&grants, &request(false, true, &[])));
        assert!(!covers(&grants, &unsandboxed_request()));
        assert!(!covers(&grants, &request(false, false, &["/tmp/build"])));
    }

    #[test]
    fn subtree_containment_covers_children() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        // Exact match and any descendant are covered.
        assert!(covers(&grants, &request(false, false, &["/tmp/build"])));
        assert!(covers(
            &grants,
            &request(false, false, &["/tmp/build/cache"])
        ));
        // A sibling / parent is not.
        assert!(!covers(&grants, &request(false, false, &["/tmp/other"])));
        assert!(!covers(&grants, &request(false, false, &["/tmp"])));
    }

    #[test]
    fn record_prunes_redundant_children() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build/cache"]));
        grants.record(&request(false, false, &["/tmp/build"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn record_keeps_existing_broader_grant() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));
        grants.record(&request(false, false, &["/tmp/build/cache"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn all_access_covers_any_concrete_write() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, true, &[]));
        assert!(covers(
            &grants,
            &request(false, false, &["/anywhere/at/all"])
        ));
        // But not network, which wasn't granted.
        assert!(!covers(&grants, &request(true, false, &[])));
    }

    #[test]
    fn network_grant_tracked_independently() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(true, false, &[]));
        assert!(covers(&grants, &request(true, false, &[])));
        assert!(!covers(&grants, &request(true, false, &["/tmp/build"])));
    }

    #[test]
    fn unsandboxed_grant_tracked_independently() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&unsandboxed_request());
        assert!(covers(&grants, &unsandboxed_request()));
        assert!(!covers(&grants, &request(true, false, &[])));
        assert!(!covers(&grants, &request(false, true, &[])));
    }

    #[test]
    fn persistent_grants_combine_with_thread_grants() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(true, false, &[]));
        let persistent = SandboxPermissions {
            allow_network: false,
            allow_fs_write_all: false,
            allow_unsandboxed: false,
            write_paths: vec![PathBuf::from("/tmp/build")],
        };

        assert!(
            grants
                .covers_with_persistent(&request(true, false, &["/tmp/build/cache"]), &persistent)
        );
        assert!(
            !grants.covers_with_persistent(&request(true, false, &["/tmp/other"]), &persistent)
        );
    }

    #[test]
    fn persistent_all_access_covers_concrete_writes() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_network: false,
            allow_fs_write_all: true,
            allow_unsandboxed: false,
            write_paths: Vec::new(),
        };

        assert!(grants.covers_with_persistent(&request(false, false, &["/anywhere"]), &persistent));
        assert!(grants.covers_with_persistent(&request(false, true, &[]), &persistent));
        assert!(!grants.covers_with_persistent(&request(true, false, &[]), &persistent));
    }

    #[test]
    fn persistent_unsandboxed_covers_unsandboxed_requests_only() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_network: false,
            allow_fs_write_all: false,
            allow_unsandboxed: true,
            write_paths: Vec::new(),
        };

        assert!(grants.covers_with_persistent(&unsandboxed_request(), &persistent));
        assert!(!grants.covers_with_persistent(&request(true, false, &[]), &persistent));
        assert!(!grants.covers_with_persistent(&request(false, true, &[]), &persistent));
    }

    #[test]
    fn effective_applies_thread_grants_to_empty_request() {
        // The core fix: a command that requests nothing still gets the
        // thread's granted write paths in its enforced policy.
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        let effective = effective(&grants, &request(false, false, &[]));
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn effective_unions_grants_with_once_request() {
        // An "allow once" path (passed via `request`, never recorded) is
        // enforced for this command alongside the standing grants.
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(true, false, &["/tmp/build"]));

        let effective = effective(&grants, &request(false, false, &["/tmp/once"]));
        assert!(effective.network);
        assert_eq!(
            effective.write_paths,
            vec![PathBuf::from("/tmp/build"), PathBuf::from("/tmp/once")]
        );
    }

    #[test]
    fn effective_applies_persistent_grants_to_empty_request() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_network: true,
            allow_fs_write_all: false,
            allow_unsandboxed: false,
            write_paths: vec![PathBuf::from("/tmp/always")],
        };

        let effective = grants.effective_with_persistent(&request(false, false, &[]), &persistent);
        assert!(effective.network);
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/always")]);
    }

    #[test]
    fn effective_dedupes_request_already_covered_by_grant() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(false, false, &["/tmp/build"]));

        let effective = effective(&grants, &request(false, false, &["/tmp/build/cache"]));
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }
}
