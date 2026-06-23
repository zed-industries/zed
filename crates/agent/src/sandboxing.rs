//! Agent-side glue for the [`sandbox`] crate.
//!
//! Centralizes the "should agent-run terminal commands be sandboxed for this
//! process?" check so the system prompt, the terminal tool, and any other
//! caller see the same answer (and so the `target_os` gate lives in one
//! place instead of scattered across the agent crate).
//!
//! The current policy is: enabled iff the user has the `sandboxing` feature
//! flag turned on, the project is local, the platform has an integration, and
//! the user has not persistently allowed unsandboxed execution (the
//! `allow_unsandboxed` sandbox setting). Setting `allow_unsandboxed`
//! persistently turns sandboxing off for the model-facing surface entirely:
//! the plain (non-sandboxed) `terminal` tool is exposed and the system prompt
//! omits the sandbox section, since every command would run without a wrap
//! anyway. The model-requested `unsandboxed: true` escape approved "once" or
//! "for this thread" does NOT change the prompt/tool set — the sandboxed tool
//! stays exposed and only the individual command runs without a wrap. See
//! `sandboxing_enabled_for_project` and `ThreadSandboxGrants`.
//!
//! macOS (Seatbelt), Linux (Bubblewrap), and Windows (Bubblewrap via WSL)
//! have real sandbox integrations; on platforms without one the per-command
//! wrap is a no-op, so commands run with the agent's ambient permissions even
//! when the flag is on.
//!
//! Naming note: this module is about agent terminal sandboxing specifically.
//! Other agent operations (e.g. file edits) are gated separately.

use agent_settings::{AgentSettings, SandboxPermissions};
use feature_flags::{FeatureFlagAppExt as _, SandboxingFeatureFlag};
use gpui::App;
use http_proxy::HostPattern;
use project::Project;
use settings::Settings;
use std::path::PathBuf;

/// Whether agent-run terminal commands should be wrapped in an OS-level
/// sandbox for this process. See module docs for the policy.
pub(crate) fn sandboxing_enabled(cx: &App) -> bool {
    cx.has_flag::<SandboxingFeatureFlag>()
}

/// Whether the sandboxed terminal can be exposed for this project.
///
/// The persistent `allow_unsandboxed` setting turns sandboxing off for the
/// model-facing surface: when it's set we expose the plain `terminal` tool and
/// omit the sandbox section from the system prompt, because every command would
/// run without a wrap regardless. This is deliberately keyed off the
/// *persistent* setting only. A model-requested `unsandboxed: true` escape that
/// the user approves "once" or "for this thread" keeps the sandboxed tool and
/// prompt in place, since the model is still operating in the sandbox model and
/// only escaping individual commands (tracked in `ThreadSandboxGrants`).
pub(crate) fn sandboxing_enabled_for_project(project: &Project, cx: &App) -> bool {
    sandboxing_enabled(cx)
        && project.is_local()
        && !AgentSettings::get_global(cx)
            .sandbox_permissions
            .allow_unsandboxed
        && cfg!(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "windows"
        ))
}

/// Network escalation requested for (or granted to) a sandboxed command.
///
/// Network access in the sandbox is allowlisted by hostname: by default
/// commands have no outbound network, and an escalation lifts that for a
/// specific set of hosts (or, as a broad escape hatch, every host). The host
/// patterns are exact hostnames (`github.com`) or leading-`*.` subdomain
/// wildcards (`*.npmjs.org`); they're validated when constructed so the
/// variants here always hold well-formed patterns.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum NetworkRequest {
    /// No network escalation — the conversation's default (blocked) applies.
    #[default]
    None,
    /// Allow connections only to these host patterns.
    Hosts(Vec<HostPattern>),
    /// Allow connections to any host ("arbitrary network access").
    AnyHost,
}

impl NetworkRequest {
    /// Whether this asks for any network access beyond the default (blocked).
    pub fn is_requested(&self) -> bool {
        !matches!(self, NetworkRequest::None)
    }

    /// The host patterns this request names, or an empty slice for the
    /// `None`/`AnyHost` variants.
    fn host_patterns(&self) -> &[HostPattern] {
        match self {
            NetworkRequest::Hosts(hosts) => hosts,
            NetworkRequest::None | NetworkRequest::AnyHost => &[],
        }
    }
}

/// A request for elevated sandbox permissions for a single terminal command.
///
/// Built from the model-controlled `terminal` tool input after the user has
/// authorized the baseline command. All paths here have already been resolved
/// to absolute, canonicalized paths by the caller — never raw, model-provided
/// strings, and never the model-controlled working directory.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SandboxRequest {
    /// Outbound network access requested for this command.
    pub network: NetworkRequest,
    /// Allow access to protected Git metadata paths.
    pub allow_git_access: bool,
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
        self.network.is_requested()
            || self.allow_git_access
            || self.allow_fs_write_all
            || self.unsandboxed
            || !self.write_paths.is_empty()
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
    /// Whether arbitrary-host network access has been granted for the thread.
    network_any_host: bool,
    /// Host patterns granted network access for the thread. Each covers its
    /// whole subdomain space; redundant entries are pruned on insert.
    network_hosts: Vec<HostPattern>,
    allow_git_access: bool,
    allow_fs_write_all: bool,
    unsandboxed: bool,
    /// Whether the user approved running commands *without* a sandbox for the
    /// rest of the thread when the OS sandbox could not be created (the
    /// fallback prompt's "Allow for this thread"). Distinct from
    /// `unsandboxed`, which records a model-requested escape; this is a
    /// user-acknowledged degradation because the sandbox is unavailable.
    sandbox_fallback: bool,
    /// Canonicalized paths granted write access for the thread. Each covers its
    /// whole subtree; redundant children are pruned on insert.
    write_paths: Vec<PathBuf>,
}

impl ThreadSandboxGrants {
    /// Whether the union of thread grants and persistent "allow always" grants
    /// covers everything `request` asks for, so the command can run without
    /// prompting again.
    ///
    /// Network coverage uses host-pattern subsumption (`*.foo.com` covers
    /// `api.foo.com`); write coverage is pure subtree containment. Both are
    /// fully deterministic and never widen scope, because grants are concrete
    /// patterns/paths rather than globs.
    pub fn covers_with_persistent(
        &self,
        request: &SandboxRequest,
        persistent: &SandboxPermissions,
    ) -> bool {
        if request.unsandboxed {
            // The persistent `allow_unsandboxed` setting is intentionally not
            // consulted here: when it's set, sandboxing is removed from the
            // model-facing surface (the plain `terminal` tool is exposed
            // instead of the sandboxed one), so the model can't issue an
            // `unsandboxed: true` request at all. Only a "for this thread"
            // grant suppresses the re-prompt while the sandboxed tool is
            // active — see `sandboxing_enabled_for_project`.
            return self.unsandboxed;
        }
        if !self.network_covered(&request.network, persistent) {
            return false;
        }
        if request.allow_git_access && !(self.allow_git_access || persistent.allow_git_access) {
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

    /// Whether the requested network escalation is already granted by the
    /// thread grants unioned with persistent "allow always" grants.
    fn network_covered(&self, request: &NetworkRequest, persistent: &SandboxPermissions) -> bool {
        let any_host_granted = self.network_any_host || persistent.allow_all_hosts;
        match request {
            NetworkRequest::None => true,
            NetworkRequest::AnyHost => any_host_granted,
            NetworkRequest::Hosts(requested) => {
                if any_host_granted {
                    return true;
                }
                let persistent_hosts = parse_persistent_hosts(&persistent.network_hosts);
                requested.iter().all(|requested| {
                    self.network_hosts
                        .iter()
                        .chain(persistent_hosts.iter())
                        .any(|granted| granted.covers(requested))
                })
            }
        }
    }

    /// Whether the user allowed running commands unsandboxed for the rest of
    /// the thread (the fallback prompt's "Allow for this thread"). Distinct
    /// from the persistent `allow_unsandboxed` setting.
    pub fn fallback_granted_for_thread(&self) -> bool {
        self.sandbox_fallback
    }

    /// Record that the user approved running commands unsandboxed for the rest
    /// of the thread when the sandbox can't be created. Only the Bubblewrap
    /// sandboxes (Linux directly, Windows via WSL) can fail to create a
    /// sandbox, so this is gated to those platforms.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn record_fallback(&mut self) {
        self.sandbox_fallback = true;
    }

    /// Record everything in `request` as granted for the rest of the thread,
    /// pruning entries that become redundant.
    pub fn record(&mut self, request: &SandboxRequest) {
        match &request.network {
            NetworkRequest::None => {}
            NetworkRequest::AnyHost => self.network_any_host = true,
            NetworkRequest::Hosts(hosts) => {
                for host in hosts {
                    insert_host_pattern(&mut self.network_hosts, host.clone());
                }
            }
        }
        self.allow_git_access |= request.allow_git_access;
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
    /// approved path (or reach a previously approved host) without
    /// re-requesting it. Passing the current `request` in also covers "allow
    /// once" grants, which are enforced for this command without being recorded
    /// for the thread.
    pub fn effective_with_persistent(
        &self,
        request: &SandboxRequest,
        persistent: &SandboxPermissions,
    ) -> SandboxRequest {
        let network = if self.network_any_host
            || persistent.allow_all_hosts
            || matches!(request.network, NetworkRequest::AnyHost)
        {
            NetworkRequest::AnyHost
        } else {
            let mut hosts = Vec::new();
            for host in self
                .network_hosts
                .iter()
                .cloned()
                .chain(parse_persistent_hosts(&persistent.network_hosts))
                .chain(request.network.host_patterns().iter().cloned())
            {
                insert_host_pattern(&mut hosts, host);
            }
            if hosts.is_empty() {
                NetworkRequest::None
            } else {
                NetworkRequest::Hosts(hosts)
            }
        };

        let mut write_paths = persistent.write_paths.clone();
        for path in self.write_paths.iter().chain(request.write_paths.iter()) {
            util::paths::insert_subtree(&mut write_paths, path.clone());
        }
        SandboxRequest {
            network,
            allow_git_access: persistent.allow_git_access
                || self.allow_git_access
                || request.allow_git_access,
            allow_fs_write_all: persistent.allow_fs_write_all
                || self.allow_fs_write_all
                || request.allow_fs_write_all,
            unsandboxed: request.unsandboxed,
            write_paths,
        }
    }
}

/// Parse persisted host strings into patterns, dropping (and logging) any
/// that fail to validate. Persisted strings are written in canonical form
/// (see `persist_sandbox_always_permission`), so this normally succeeds; the
/// filter is defensive against hand-edited settings.
fn parse_persistent_hosts(raw: &[String]) -> Vec<HostPattern> {
    raw.iter()
        .filter_map(|host| match HostPattern::parse(host) {
            Ok(pattern) => Some(pattern),
            Err(error) => {
                log::warn!(
                    "ignoring invalid network host pattern '{host}' in sandbox settings: {error}"
                );
                None
            }
        })
        .collect()
}

/// Insert `pattern` into a host-pattern set, keeping it minimal: skip it if an
/// existing entry already subsumes it, and drop existing entries it subsumes.
/// The host-pattern analogue of [`util::paths::insert_subtree`].
pub(crate) fn insert_host_pattern(set: &mut Vec<HostPattern>, pattern: HostPattern) {
    if set.iter().any(|existing| existing.covers(&pattern)) {
        return;
    }
    set.retain(|existing| !pattern.covers(existing));
    set.push(pattern);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hosts(list: &[&str]) -> NetworkRequest {
        NetworkRequest::Hosts(
            list.iter()
                .map(|h| HostPattern::parse(h).unwrap())
                .collect(),
        )
    }

    fn request(network: NetworkRequest, all: bool, paths: &[&str]) -> SandboxRequest {
        SandboxRequest {
            network,
            allow_git_access: false,
            allow_fs_write_all: all,
            unsandboxed: false,
            write_paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    fn unsandboxed_request() -> SandboxRequest {
        SandboxRequest {
            network: NetworkRequest::None,
            allow_git_access: false,
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

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[test]
    fn fallback_granted_for_thread_tracks_record_fallback() {
        let mut grants = ThreadSandboxGrants::default();
        assert!(!grants.fallback_granted_for_thread());

        // The thread-scoped fallback grant is independent of the
        // model-requested `unsandboxed` grant.
        grants.record_fallback();
        assert!(grants.fallback_granted_for_thread());
        assert!(!covers(&grants, &unsandboxed_request()));
    }

    #[test]
    fn empty_grants_cover_nothing() {
        let grants = ThreadSandboxGrants::default();
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
        assert!(!covers(
            &grants,
            &request(hosts(&["github.com"]), false, &[])
        ));
        assert!(!covers(&grants, &request(NetworkRequest::None, true, &[])));
        assert!(!covers(&grants, &unsandboxed_request()));
        assert!(!covers(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp/build"])
        ));
    }

    #[test]
    fn subtree_containment_covers_children() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build"]));

        // Exact match and any descendant are covered.
        assert!(covers(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp/build"])
        ));
        assert!(covers(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp/build/cache"])
        ));
        // A sibling / parent is not.
        assert!(!covers(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp/other"])
        ));
        assert!(!covers(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp"])
        ));
    }

    #[test]
    fn record_prunes_redundant_children() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build/cache"]));
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn record_keeps_existing_broader_grant() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build"]));
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build/cache"]));
        assert_eq!(grants.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn all_access_covers_any_concrete_write() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, true, &[]));
        assert!(covers(
            &grants,
            &request(NetworkRequest::None, false, &["/anywhere/at/all"])
        ));
        // But not network, which wasn't granted.
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
    }

    #[test]
    fn any_host_grant_covers_specific_and_any_host() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::AnyHost, false, &[]));
        assert!(covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
        assert!(covers(
            &grants,
            &request(hosts(&["github.com"]), false, &[])
        ));
        // ...but not an orthogonal write request.
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &["/tmp/build"])
        ));
    }

    #[test]
    fn host_grant_covers_subdomains_but_not_any_host() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(hosts(&["*.github.com"]), false, &[]));

        assert!(covers(
            &grants,
            &request(hosts(&["api.github.com"]), false, &[])
        ));
        assert!(covers(
            &grants,
            &request(hosts(&["*.github.com"]), false, &[])
        ));
        // The bare parent isn't a subdomain, so it isn't covered.
        assert!(!covers(
            &grants,
            &request(hosts(&["github.com"]), false, &[])
        ));
        // A different host isn't covered.
        assert!(!covers(
            &grants,
            &request(hosts(&["npmjs.org"]), false, &[])
        ));
        // A specific grant never satisfies an any-host request.
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
    }

    #[test]
    fn record_prunes_redundant_hosts() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(hosts(&["api.github.com"]), false, &[]));
        grants.record(&request(hosts(&["*.github.com"]), false, &[]));
        assert_eq!(
            grants.network_hosts,
            vec![HostPattern::parse("*.github.com").unwrap()]
        );
    }

    #[test]
    fn unsandboxed_grant_tracked_independently() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&unsandboxed_request());
        assert!(covers(&grants, &unsandboxed_request()));
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
        assert!(!covers(&grants, &request(NetworkRequest::None, true, &[])));
    }

    #[test]
    fn git_access_grant_tracked_independently() {
        let mut git_request = request(NetworkRequest::None, false, &[]);
        git_request.allow_git_access = true;

        let mut grants = ThreadSandboxGrants::default();
        assert!(!covers(&grants, &git_request));

        grants.record(&git_request);
        assert!(covers(&grants, &git_request));
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
        assert!(!covers(&grants, &request(NetworkRequest::None, true, &[])));
    }

    #[test]
    fn unrestricted_writes_do_not_cover_git_access() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, true, &[]));

        let mut git_request = request(NetworkRequest::None, false, &[]);
        git_request.allow_git_access = true;
        assert!(!covers(&grants, &git_request));
    }

    #[test]
    fn persistent_grants_combine_with_thread_grants() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(hosts(&["github.com"]), false, &[]));
        let persistent = SandboxPermissions {
            write_paths: vec![PathBuf::from("/tmp/build")],
            ..Default::default()
        };

        assert!(grants.covers_with_persistent(
            &request(hosts(&["github.com"]), false, &["/tmp/build/cache"]),
            &persistent
        ));
        assert!(!grants.covers_with_persistent(
            &request(hosts(&["github.com"]), false, &["/tmp/other"]),
            &persistent
        ));
    }

    #[test]
    fn persistent_network_hosts_are_honored() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            network_hosts: vec!["*.npmjs.org".to_string()],
            ..Default::default()
        };

        assert!(grants.covers_with_persistent(
            &request(hosts(&["registry.npmjs.org"]), false, &[]),
            &persistent
        ));
        assert!(
            !grants
                .covers_with_persistent(&request(hosts(&["github.com"]), false, &[]), &persistent)
        );
    }

    #[test]
    fn persistent_all_access_covers_concrete_writes() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_fs_write_all: true,
            ..Default::default()
        };

        assert!(grants.covers_with_persistent(
            &request(NetworkRequest::None, false, &["/anywhere"]),
            &persistent
        ));
        assert!(
            grants.covers_with_persistent(&request(NetworkRequest::None, true, &[]), &persistent)
        );
        assert!(
            !grants
                .covers_with_persistent(&request(NetworkRequest::AnyHost, false, &[]), &persistent)
        );
    }

    #[test]
    fn thread_grant_covers_unsandboxed_requests() {
        // A "for this thread" grant suppresses the re-prompt for later
        // `unsandboxed: true` requests within the same thread.
        let mut grants = ThreadSandboxGrants::default();
        assert!(!covers(&grants, &unsandboxed_request()));
        grants.record(&unsandboxed_request());
        assert!(covers(&grants, &unsandboxed_request()));

        // A thread-wide unsandboxed grant only covers unsandboxed requests; it
        // does not widen network or filesystem scope.
        assert!(!covers(
            &grants,
            &request(NetworkRequest::AnyHost, false, &[])
        ));
        assert!(!covers(&grants, &request(NetworkRequest::None, true, &[])));
    }

    #[test]
    fn persistent_allow_unsandboxed_does_not_cover_here() {
        // The persistent setting is handled by removing the sandboxed tool (see
        // `sandboxing_enabled_for_project`), not by covering requests, so on
        // its own it never makes an `unsandboxed: true` request "covered".
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_unsandboxed: true,
            ..Default::default()
        };
        assert!(!grants.covers_with_persistent(&unsandboxed_request(), &persistent));
    }

    #[test]
    fn effective_applies_thread_grants_to_empty_request() {
        // The core fix: a command that requests nothing still gets the
        // thread's granted write paths in its enforced policy.
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build"]));

        let effective = effective(&grants, &request(NetworkRequest::None, false, &[]));
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }

    #[test]
    fn effective_unions_grants_with_once_request() {
        // An "allow once" path (passed via `request`, never recorded) is
        // enforced for this command alongside the standing grants.
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(hosts(&["github.com"]), false, &["/tmp/build"]));

        let effective = effective(
            &grants,
            &request(hosts(&["npmjs.org"]), false, &["/tmp/once"]),
        );
        assert_eq!(effective.network, hosts(&["github.com", "npmjs.org"]));
        assert_eq!(
            effective.write_paths,
            vec![PathBuf::from("/tmp/build"), PathBuf::from("/tmp/once")]
        );
    }

    #[test]
    fn effective_any_host_subsumes_specific_hosts() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(hosts(&["github.com"]), false, &[]));

        let effective = effective(&grants, &request(NetworkRequest::AnyHost, false, &[]));
        assert_eq!(effective.network, NetworkRequest::AnyHost);
    }

    #[test]
    fn effective_applies_persistent_grants_to_empty_request() {
        let grants = ThreadSandboxGrants::default();
        let persistent = SandboxPermissions {
            allow_all_hosts: true,
            allow_git_access: true,
            write_paths: vec![PathBuf::from("/tmp/always")],
            ..Default::default()
        };

        let effective = grants
            .effective_with_persistent(&request(NetworkRequest::None, false, &[]), &persistent);
        assert_eq!(effective.network, NetworkRequest::AnyHost);
        assert!(effective.allow_git_access);
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/always")]);
    }

    #[test]
    fn effective_dedupes_request_already_covered_by_grant() {
        let mut grants = ThreadSandboxGrants::default();
        grants.record(&request(NetworkRequest::None, false, &["/tmp/build"]));

        let effective = effective(
            &grants,
            &request(NetworkRequest::None, false, &["/tmp/build/cache"]),
        );
        assert_eq!(effective.write_paths, vec![PathBuf::from("/tmp/build")]);
    }
}
