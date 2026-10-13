//! How cedian starts OMP for a person at the keyboard: the user's
//! `cedian.toml` mapped onto the spawn profile (ADR-0020, ADR-0035), and the
//! ADR-0041 pin of OMP's own `tools.approval` allows. The CLI and the app
//! both start OMP through here.

use crate::{Policy, Settings, Verdict};
use cedian_omp::{ApprovalMode, Approvals, SpawnPolicy, ToolPolicy};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Whether `cedian_worktree_request` is registered. It makes a tree and a
/// branch with no dialog of its own, so only project writes that need no
/// approval allow it (ADR-0012 strict-wins).
pub fn registers_worktree_request(settings: &Settings) -> bool {
    settings.permissions.project_write == Verdict::Allow
}

/// Map `[permissions]` onto the OMP spawn policy (ADR-0020). Under the
/// default policy it only tightens: `dangerous = allow` still leaves the exec
/// floor at `prompt` (strict-wins, ADR-0012), and yolo is unrepresentable.
/// Under `policy = "omp"` (ADR-0035) OMP's own config decides approvals;
/// cedian's denies still apply. `host_tools` is the whole host-tool set the
/// caller registers (ADR-0004), so the overlay allows exactly those names.
pub fn spawn_policy(
    settings: &Settings,
    policy_source: Policy,
    host_tools: &[&str],
) -> SpawnPolicy {
    let mut policy = SpawnPolicy::default();
    if policy_source == Policy::Omp {
        policy.approvals = Approvals::Omp;
    }
    for tool in host_tools {
        policy.host_tools.insert((*tool).to_string());
    }
    if settings.permissions.project_write != Verdict::Allow && policy.approvals != Approvals::Omp {
        policy.approvals = Approvals::Cedian(ApprovalMode::AlwaysAsk);
    }
    let mut deny = |tools: &[&str]| {
        for tool in tools {
            policy
                .tool_policies
                .insert((*tool).to_string(), ToolPolicy::Deny);
        }
    };
    if settings.permissions.project_write == Verdict::Deny {
        deny(&["edit", "write", "ast_edit"]);
    }
    if settings.permissions.dangerous == Verdict::Deny {
        deny(cedian_omp::spawn_profile::EXEC_TOOLS);
    }
    policy
}

/// The OMP version cedian is pinned to (`vendor/omp-revision.json`).
pub fn pinned_omp_version() -> String {
    let pin: serde_json::Value =
        serde_json::from_str(include_str!("../../../vendor/omp-revision.json"))
            .expect("vendor/omp-revision.json is JSON");
    pin["spikeVerified"]["ompVersion"]
        .as_str()
        .expect("vendor/omp-revision.json names spikeVerified.ompVersion")
        .to_string()
}

/// The OMP binary a launch runs, and the warning to show when it is not
/// the pinned version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmpChoice {
    pub binary: PathBuf,
    pub warning: Option<String>,
}

/// The OMP binary (ADR-0057 decision 4). A set `CEDIAN_OMP_BINARY` runs as
/// given, asked `--version` once only for the warning. Otherwise an OMP
/// bundled next to the running executable (U13: `cedian.app`'s
/// `Contents/Resources/omp`) runs as the bundle shipped it — off-pin and all,
/// because a shipped app must not need an `omp` on PATH. Else the first of
/// PATH's `omp` and `~/.local/bin/omp` whose `--version` is the pin, else the
/// first found. Off the pin, the warning names both versions. Blocking: call
/// it off the UI thread.
pub fn omp_binary() -> Result<OmpChoice, String> {
    omp_binary_for(std::env::current_exe().ok().as_deref())
}

/// The resolution, from the executable a bundle lookup starts at
/// ([`omp_binary`] passes the running one; tests drive a fake `.app` layout,
/// which `current_exe` cannot be moved to).
pub fn omp_binary_for(exe: Option<&Path>) -> Result<OmpChoice, String> {
    let pinned = pinned_omp_version();
    let probe = |binary: &Path| cedian_omp::omp_version(binary, Duration::from_secs(3));
    if let Some(binary) = std::env::var_os("CEDIAN_OMP_BINARY").map(PathBuf::from) {
        let warning = off_pin(&pinned, &binary, probe(&binary));
        return Ok(OmpChoice { binary, warning });
    }
    if let Some(binary) = exe.and_then(bundled_sidecar) {
        let warning = off_pin(&pinned, &binary, probe(&binary));
        return Ok(OmpChoice { binary, warning });
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    candidates
        .extend(cedian_omp::resolve_on_path("omp", std::env::var("PATH").ok().as_deref()).ok());
    candidates.extend(
        std::env::var_os("HOME")
            .map(|home| PathBuf::from(home).join(".local/bin/omp"))
            .filter(|path| path.is_file()),
    );
    let mut seen = BTreeSet::new();
    candidates.retain(|path| seen.insert(std::fs::canonicalize(path).unwrap_or(path.clone())));
    let mut first = None;
    for binary in candidates {
        let version = probe(&binary);
        if version.as_ref().is_ok_and(|v| *v == pinned) {
            return Ok(OmpChoice {
                binary,
                warning: None,
            });
        }
        first.get_or_insert((binary, version));
    }
    let (binary, version) =
        first.ok_or("omp not found in CEDIAN_OMP_BINARY, on PATH or in ~/.local/bin")?;
    let warning = off_pin(&pinned, &binary, version);
    Ok(OmpChoice { binary, warning })
}

/// The OMP a bundle ships: `<exe>`'s `Contents/Resources/omp` when the
/// executable sits in a bundle's `Contents/MacOS` (U13,
/// `script/cedian-build-omp`). Any other layout gives nothing, so a
/// `target/release/cedian` next to a stray `Resources/` is not a bundle.
fn bundled_sidecar(exe: &Path) -> Option<PathBuf> {
    let macos = exe.parent()?;
    if macos.file_name()? != "MacOS" {
        return None;
    }
    let contents = macos.parent()?;
    if contents.file_name()? != "Contents" {
        return None;
    }
    let sidecar = contents.join("Resources/omp");
    sidecar.is_file().then_some(sidecar)
}

fn off_pin(
    pinned: &str,
    binary: &Path,
    version: Result<String, cedian_omp::OmpError>,
) -> Option<String> {
    let found = match version {
        Ok(version) if version == pinned => return None,
        Ok(version) => format!("OMP {version}"),
        Err(e) => format!("an OMP whose version cannot be read ({e})"),
    };
    Some(format!(
        "cedian is pinned to OMP {pinned} but found {found} at {}; it runs that one",
        binary.display()
    ))
}

/// The opt-in badge (ADR-0035 decision 4): OMP's effective approval mode
/// and `computer`, and whether the workspace's own `.omp/config.yml` set them
/// (OMP's answer in the workspace differs from its answer in an empty dir).
/// The CLI prints it; the panel shows it. Blocking: runs `omp config get`.
pub fn omp_policy_badge(binary: &Path, workdir: &Path) -> String {
    let empty = std::env::temp_dir().join(format!("cedian-omp-global-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&empty);
    let get = |cwd: &Path, key: &str| {
        cedian_omp::omp_config_get(binary, cwd, key, Duration::from_secs(10)).ok()
    };
    let describe = |key: &str, show: &dyn Fn(&serde_json::Value) -> String| {
        let here = get(workdir, key);
        let from = if here.is_some() && here != get(&empty, key) {
            " from the project's .omp/config.yml"
        } else {
            ""
        };
        match here {
            Some(v) => format!("{}{from}", show(&v)),
            None => "unknown, assume on".to_string(),
        }
    };
    let mode = describe("tools.approvalMode", &|v| {
        v.as_str().unwrap_or("unknown").to_string()
    });
    let computer = describe("computer.enabled", &|v| {
        if v.as_bool() == Some(true) {
            "on"
        } else {
            "off"
        }
        .to_string()
    });
    let _ = std::fs::remove_dir(&empty);
    format!(
        "◆ OMP policy — approvals and computer from your OMP config (approvalMode: {mode}; computer: {computer})"
    )
}

/// Tools OMP's merged config sets to `allow` in `workdir` (ADR-0041
/// decision 2). Fails closed: without the record the default profile cannot
/// pin them, so the spawn is refused.
pub fn config_allows(binary: &Path, workdir: &Path) -> Result<BTreeSet<String>, String> {
    let record =
        cedian_omp::omp_config_get(binary, workdir, "tools.approval", Duration::from_secs(10))
            .map_err(|e| format!("cannot read OMP's tools.approval, so cannot pin it: {e}"))?;
    let record = record
        .as_object()
        .ok_or("OMP's tools.approval is not a record")?;
    Ok(record
        .iter()
        .filter(|(_, v)| v.as_str() == Some("allow"))
        .map(|(k, _)| k.clone())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_in_hands_approvals_to_omp_but_keeps_denies() {
        let mut settings = Settings::default();
        let default = spawn_policy(&settings, Policy::Cedian, &[]);
        assert_eq!(default.approvals, Approvals::Cedian(ApprovalMode::Write));
        assert_eq!(
            spawn_policy(&settings, Policy::Omp, &[]).approvals,
            Approvals::Omp
        );
        settings.permissions.project_write = Verdict::Deny;
        let opted = spawn_policy(&settings, Policy::Omp, &[]);
        assert_eq!(opted.approvals, Approvals::Omp);
        assert_eq!(opted.tool_policies.get("write"), Some(&ToolPolicy::Deny));
        assert_eq!(
            spawn_policy(&settings, Policy::Cedian, &[]).approvals,
            Approvals::Cedian(ApprovalMode::AlwaysAsk)
        );
    }

    #[test]
    fn host_tools_are_exactly_the_ones_named() {
        let policy = spawn_policy(
            &Settings::default(),
            Policy::Cedian,
            &["cedian_worktree_request"],
        );
        assert_eq!(
            policy
                .host_tools
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["cedian_worktree_request"]
        );
    }
}
