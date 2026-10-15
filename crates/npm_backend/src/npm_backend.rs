//! npm package-manager backend — CLI invocation, output parsing, and version
//! classification for the npm manager panel.
//!
//! No GPUI, no app-state dependency: every function here takes plain
//! strings/paths and returns a plain value or `Result` — a host panel wires
//! this into its own UI/state, this crate just does the work.
//!
//! Scope notes (mirrors the plan's §4.1/§4.3):
//! - `run_npm_cli` / `detect_package_manager` / the JSON parsers and the
//!   version classifiers are pure and live here.
//! - The Forge "security aggregation → Dashboard feed" half is intentionally
//!   dropped: this Zed host has no Dashboard consumer, so only the local
//!   per-package `npm audit` list is kept (see `parse_audit`).

use std::process::{Command, Stdio};

use semver::{Version, VersionReq};

mod engines;
mod path_env;

pub use engines::{
    EngineCompat, NodeRequirement, RequirementCheck, Verdict, engine_compat, node_requirement,
    npm_range_matches,
};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// ── Types ─────────────────────────────────────────────────────────────

/// A single installed package entry, as emitted by `npm ls --json`.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NpmInstalledPkg {
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    /// `true` when the package is a `devDependency` (npm ls marks these).
    pub is_dev: bool,
}

/// An "outdated" entry, as emitted by the flattened form of
/// `npm outdated --json` (name → current/wanted/latest/dependent/wanted).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NpmOutdatedPkg {
    pub name: String,
    pub current: String,
    pub wanted: String,
    pub latest: String,
    pub location: Option<String>,
}

/// A local `npm audit` finding (kept; the Forge Dashboard aggregator is not).
#[derive(serde::Serialize, Clone, Debug)]
pub struct NpmAuditVuln {
    pub package: String,
    pub severity: String,
    pub title: String,
    pub url: String,
    pub exploitability: Option<u8>,
    /// Version that `npm audit` reports fixes this finding (if any).
    pub fixed_in: Option<String>,
}

/// Result of classifying a version delta.
#[derive(serde::Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdateKind {
    /// Target is a patch of current.
    Patch,
    /// Target is a minor of current.
    Minor,
    /// Target is a major of current.
    Major,
    /// Target is the current
    Current,
}

/// Which package manager a project signals via its lockfile.
#[derive(serde::Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn cli_name(self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }
}

// ── Registry data model ───────────────────────────────────────────────

/// A single match from the registry `/-/v1/search` endpoint. `compat` is
/// filled in by the caller once it knows the active node version.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NpmSearchResult {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    /// Weekly downloads (present only when the registry search includes it).
    pub downloads: Option<u64>,
    /// The package's declared `engines.node` range, if any.
    pub engines_node: Option<String>,
    /// Whether the active node satisfies `engines_node`; `None` when unknown.
    pub compat: Option<bool>,
}

/// One version of a package from its registry "packument", carrying the
/// peer-dependency ranges it declares.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NpmVersionEntry {
    pub version: String,
    pub peer_deps: Vec<(String, String)>,
}

/// The parsed `registry.npmjs.org/{name}` packument for one package — the
/// data behind the search details pane.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NpmPackageDetails {
    pub name: String,
    /// Latest stable version (`dist-tags.latest`, falling back to top level).
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    /// Weekly downloads; `None` when the downloads API 404s (no data).
    pub weekly_downloads: Option<u64>,
    /// All published versions (newest first), with peer-dependency ranges.
    pub versions: Vec<NpmVersionEntry>,
    /// The package README, when the packument carries a non-empty one.
    pub readme: Option<String>,
}

// ── CLI invocation ────────────────────────────────────────────────────

/// Runs the package manager CLI in `project_root` with `args`, returning
/// captured stdout on success. Stderr is folded into the error string.
///
/// Its `PATH` is the one `npm run` would give a script: the project's
/// `node_modules/.bin` first, then the IDE's own runtimes (the built-in
/// Node.js picked in the Node panel), then the machine's (see
/// `path_env::path_with_node`). So this runs under the same Node.js as the
/// IDE's terminals and scripts. On Windows the child is spawned with
/// `CREATE_NO_WINDOW` to avoid a console flash.
pub fn run_npm_cli(
    project_root: &str,
    cli_name: &str,
    args: &[String],
) -> Result<String, String> {
    let make_command = |exe: &str, run_args: &[String]| {
        let mut command = Command::new(exe);
        command
            .current_dir(project_root)
            .args(run_args)
            .env("PATH", path_env::path_with_node(project_root))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        command
    };

    let output = match make_command(cli_name, args).output() {
        Ok(output) => output,
        // `e` is only read in the non-Windows arm below.
        #[allow(unused_variables)]
        Err(e) => {
            // Package-manager CLIs are `.cmd` shims on Windows (e.g.
            // `npm.cmd`); spawning them bare fails. Fall back to `cmd /C`
            // so PATH/PATHEXT resolution works, like `node_backend`.
            #[cfg(target_os = "windows")]
            {
                let mut wrapped = Vec::with_capacity(args.len() + 2);
                wrapped.push("/C".to_string());
                wrapped.push(cli_name.to_string());
                wrapped.extend(args.iter().cloned());
                make_command("cmd", &wrapped)
                    .output()
                    .map_err(|cmd_err| format!("failed to run `{cli_name}`: {cmd_err}"))?
            }
            #[cfg(not(target_os = "windows"))]
            {
                return Err(format!("failed to run `{cli_name}`: {e}"));
            }
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    // npm uses exit code 1 as "found something to report": `npm ls` exits 1
    // when the tree has missing/extraneous problems, `npm outdated` exits 1
    // when packages are behind, `npm audit` exits 1 when vulnerabilities
    // exist — all still printing valid JSON. Only code > 1 is a hard failure
    // (spawn error, bad project, audit error), matching the source at
    // `E:\Forge_GPUI\...\npm_manager_panel.rs` (`if code > 1`).
    let code = output.status.code().unwrap_or(-1);
    if code > 1 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut msg = format!(
            "`{cli_name} {}` exited with {code:?}",
            args.join(" "),
        );
        if !stderr.trim().is_empty() {
            msg.push_str(&format!("\n{stderr}"));
        }
        return Err(msg);
    }
    Ok(stdout)
}

/// Determines the lockfile-driven package manager for `project_root`,
/// defaulting to npm when nothing (or only `package-lock.json`) exists.
pub fn detect_package_manager(project_root: &str) -> PackageManager {
    let dir = std::path::Path::new(project_root);
    for (file, mgr) in [
        ("pnpm-lock.yaml", PackageManager::Pnpm),
        ("yarn.lock", PackageManager::Yarn),
        ("bun.lockb", PackageManager::Bun),
        ("bun.lock", PackageManager::Bun),
    ] {
        if dir.join(file).exists() {
            return mgr;
        }
    }
    PackageManager::Npm
}

/// Lists installed packages (`npm ls --json --depth 0`), returned as tagged
/// entries ready to drop into a row model.
pub fn list_installed(project_root: &str, cli_name: &str) -> Result<Vec<NpmInstalledPkg>, String> {
    let output = run_npm_cli(
        project_root,
        cli_name,
        &["ls".into(), "--json".into(), "--depth".into(), "0".into()],
    )?;
    parse_installed(&output)
}

/// `npm outdated --json` → parsed rows (an empty set means up to date).
pub fn list_outdated(project_root: &str, cli_name: &str) -> Result<Vec<NpmOutdatedPkg>, String> {
    let output = run_npm_cli(
        project_root,
        cli_name,
        &["outdated".into(), "--json".into()],
    )?;
    parse_outdated(&output)
}

/// `npm audit --json` → the local per-package vuln list.
pub fn list_audit_vulns(project_root: &str, cli_name: &str) -> Result<Vec<NpmAuditVuln>, String> {
    let output = run_npm_cli(
        project_root,
        cli_name,
        &["audit".into(), "--json".into(), "--omit=dev".into()],
    )?;
    parse_audit(&output)
}

// ── Parsers ───────────────────────────────────────────────────────────

/// Flattens `npm ls --json` into direct-install rows.
///
/// Two shapes are handled, mirroring the source at
/// `E:\Forge_GPUI\...\npm_manager_panel.rs::parse_installed`:
/// - Classic: a top-level `dependencies` object keyed by package name
///   (`npm ls --json --depth 0` on npm v7+), each node holding at least
///   `version`. The name comes from the map key, not the node.
/// - v7+ lockfile map: a top-level `packages` object keyed by path
///   (`""` for the root, `node_modules/foo`, `node_modules/@scope/foo`, …).
///   The root entry and anything with more than one `node_modules` segment
///   (transitive installs) are skipped.
pub fn parse_installed(raw: &str) -> Result<Vec<NpmInstalledPkg>, String> {
    let json: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| format!("cannot parse npm ls output: {e}"))?;

    let mut out = Vec::new();
    if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
        for (name, val) in deps {
            out.push(NpmInstalledPkg {
                name: name.clone(),
                version: val
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                path: val
                    .get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string()),
                is_dev: val.get("dev").and_then(|d| d.as_bool()).unwrap_or(false),
            });
        }
    } else if let Some(packages) = json.get("packages").and_then(|d| d.as_object()) {
        for (path, entry) in packages {
            let path = path.as_str();
            if path.is_empty()
                || path == "."
                || !path.contains("node_modules")
                || path.match_indices("node_modules").count() > 1
            {
                continue;
            }
            out.push(NpmInstalledPkg {
                name: entry
                    .get("name")
                    .and_then(|n| n.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| {
                        // Everything after `node_modules/`, so a scoped
                        // package keeps its `@scope/` prefix.
                        path.rsplit_once("node_modules/")
                            .map_or(path, |(_, name)| name)
                            .to_string()
                    }),
                version: entry
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                path: Some(path.to_string()),
                is_dev: entry.get("dev").and_then(|d| d.as_bool()).unwrap_or(false),
            });
        }
    } else {
        return Err(
            "npm ls output had neither 'dependencies' nor 'packages'".to_string(),
        );
    }

    out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(out)
}

/// `npm outdated --json` usually comes back as a per-name object, but with
/// multiple dependents it can nest `{ current, wanted, latest, ... }` under
/// `dependents`. Both shapes flatten to rows here.
pub fn parse_outdated(raw: &str) -> Result<Vec<NpmOutdatedPkg>, String> {
    let json: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| format!("cannot parse npm outdated output: {e}"))?;
    let Ok(obj) = json.as_object().ok_or_else(|| "npm outdated returned no object".to_string())
    else {
        return Ok(Vec::new());
    };

    let mut out = Vec::new();
    for (name, entry) in obj {
        let current = entry
            .get("current")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let wanted = entry
            .get("wanted")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let latest = entry
            .get("latest")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let location = entry.get("location").and_then(|v| v.as_str());
        if wanted.is_empty() || latest.is_empty() {
            continue;
        }
        out.push(NpmOutdatedPkg {
            name: name.clone(),
            current,
            wanted,
            latest,
            location: location.map(|s| s.to_string()),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// Extract the local, actionable vuln list out of `npm audit --json`. The
/// modern (v2) shape is `{ vulnerabilities: { pkg: { severity, via: [...] } } }`;
/// the legacy shape is `{ advisories: { id: { module_name, severity, title,
/// url, ... } } }`.
pub fn parse_audit(raw: &str) -> Result<Vec<NpmAuditVuln>, String> {
    let json: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| format!("cannot parse npm audit output: {e}"))?;

    let mut out = Vec::new();

    // Modern shape (npm 7+).
    if let Some(vulns) = json.get("vulnerabilities").and_then(|v| v.as_object()) {
        for (pkg, entry) in vulns {
            let severity = entry
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let (title, url, exploitability) = entry
                .get("via")
                .and_then(|v| v.as_array())
                .and_then(|via| via.last())
                .and_then(|v| v.as_object())
                .map(|v| {
                    (
                        v.get("title").and_then(|t| t.as_str()).unwrap_or(""),
                        v.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string(),
                        v.get("exploitability").and_then(|e| e.as_u64()).map(|e| e as u8),
                    )
                })
                .unwrap_or(("", String::new(), None));
            let fixed_in = entry
                .get("fixAvailable")
                .and_then(|f| f.get("version"))
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .filter(|s| !s.is_empty());
            out.push(NpmAuditVuln {
                package: pkg.clone(),
                severity: severity.to_string(),
                title: title.to_string(),
                url,
                exploitability,
                fixed_in,
            });
        }
    }

    // Legacy shape (npm 6).
    if let Some(advs) = json.get("advisories").and_then(|v| v.as_object()) {
        for entry in advs.values() {
            let Some(package) = entry.get("module_name").and_then(|v| v.as_str()) else {
                continue;
            };
            out.push(NpmAuditVuln {
                package: package.to_string(),
                severity: entry
                    .get("severity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                title: entry
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                url: entry
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                exploitability: None,
                fixed_in: entry
                    .get("fixed_in")
                    .or_else(|| entry.get("patched_versions"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty()),
            });
        }
    }

    out.sort_by(|a, b| a.package.cmp(&b.package));
    Ok(out)
}

// ── Registry parsing (npmjs.org search, packuments, downloads) ────────

/// Parses the registry `/-/v1/search` response into `(results, total)`.
/// `compat` is left `None` — the caller resolves it against the active node
/// via [`node_engine_compatible`].
pub fn parse_npm_search(json: &serde_json::Value) -> (Vec<NpmSearchResult>, usize) {
    let mut out = Vec::new();
    if let Some(objects) = json.get("objects").and_then(|v| v.as_array()) {
        for obj in objects {
            let Some(pkg) = obj.get("package") else {
                continue;
            };
            let Some(name) = pkg.get("name").and_then(|v| v.as_str()) else {
                continue;
            };
            out.push(NpmSearchResult {
                name: name.to_string(),
                version: pkg
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: pkg
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                downloads: obj.get("downloads").and_then(|v| v.as_u64()),
                engines_node: pkg
                    .get("engines")
                    .and_then(|v| v.get("node"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                compat: None,
            });
        }
    }
    let total = json
        .get("total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let count = out.len();
    (out, total.max(count))
}

/// Parses a `registry.npmjs.org/{name}` packument into package details.
///
/// `versions` are sorted newest-first; each entry's version keeps the
/// peer-dependency ranges it declares (for peer-conflict filtering on the
/// details pane). `readme` is kept only when non-empty and non-placeholder.
pub fn parse_npm_packument(json: &serde_json::Value) -> Result<NpmPackageDetails, String> {
    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "packument has no name".to_string())?
        .to_string();
    let version = json
        .get("dist-tags")
        .and_then(|v| v.get("latest"))
        .and_then(|v| v.as_str())
        .or_else(|| json.get("version").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string();

    let latest = json
        .get("versions")
        .and_then(|v| v.get(&version));
    let description = latest
        .and_then(|v| v.get("description"))
        .or_else(|| json.get("description"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let license = match latest
        .and_then(|v| v.get("license"))
        .or_else(|| json.get("license"))
    {
        Some(serde_json::Value::String(s)) => Some(s.clone()),
        Some(serde_json::Value::Object(o)) => o
            .get("url")
            .and_then(|u| u.as_str())
            .or_else(|| o.get("type").and_then(|t| t.as_str()))
            .map(str::to_string),
        _ => None,
    };
    let homepage = latest
        .and_then(|v| v.get("homepage"))
        .or_else(|| json.get("homepage"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let readme = json
        .get("readme")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty() && *s != "No README found")
        .map(str::to_string);

    let mut versions = Vec::new();
    if let Some(obj) = json.get("versions").and_then(|v| v.as_object()) {
        for (v, info) in obj {
            let peer_deps = info
                .get("peerDependencies")
                .and_then(|d| d.as_object())
                .map(|deps| {
                    deps.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            versions.push(NpmVersionEntry {
                version: v.clone(),
                peer_deps,
            });
        }
    }
    versions.sort_by(|a, b| match (Version::parse(&a.version), Version::parse(&b.version)) {
        (Ok(va), Ok(vb)) => vb.cmp(&va),
        _ => b.version.cmp(&a.version),
    });

    Ok(NpmPackageDetails {
        name,
        version,
        description,
        license,
        homepage,
        weekly_downloads: None,
        versions,
        readme,
    })
}

/// Parses the weekly download count out of `api.npmjs.org/downloads/...`.
pub fn parse_npm_downloads(json: &serde_json::Value) -> Option<u64> {
    json.get("downloads").and_then(|v| v.as_u64())
}

/// Whether the active `node_version` satisfies a package's `engines.node`
/// range, read the npm way (see [`npm_range_matches`]). `None` when either
/// side can't be read, never a guess.
pub fn node_engine_compatible(node_version: &str, range: &str) -> Option<bool> {
    npm_range_matches(node_version, range)
}

/// The peer-dependency constraints in `peers` that the *installed* packages
/// already violate (name + range, e.g. `"react@^17.0.0"`). Peers that aren't
/// installed can't be judged and are skipped (fails open).
pub fn peer_conflicts_with_installed(
    peers: &[(String, String)],
    installed: &[NpmInstalledPkg],
) -> Vec<String> {
    peers
        .iter()
        .filter(|(name, range)| {
            installed.iter().find(|p| &p.name == name).is_some_and(|p| {
                VersionReq::parse(range)
                    .map(|req| !req.matches(&Version::parse(&p.version).unwrap_or_else(|_| Version::new(0, 0, 0))))
                    .unwrap_or(false)
            })
        })
        .map(|(name, range)| format!("{name}@{range}"))
        .collect()
}

// ── Version classification ────────────────────────────────────────────

/// Classify an outdated row: how far is `latest` from `current`?
pub fn classify_update(current: &str, latest: &str) -> UpdateKind {
    let (Ok(cur), Ok(latest)) = (Version::parse(current), Version::parse(latest)) else {
        // Non-semver tags (git deps, URLs): out-of-spec → treat as major-range.
        return if current == latest { UpdateKind::Current } else { UpdateKind::Major };
    };
    if cur == latest {
        return UpdateKind::Current;
    }
    if cur.major != latest.major {
        UpdateKind::Major
    } else if cur.minor != latest.minor {
        UpdateKind::Minor
    } else {
        UpdateKind::Patch
    }
}

/// True when the candidate `version` satisfies *every* `ranges` (a set of
/// range strings sourced from package.json `dependencies`/`devDependencies`
/// for that dep).
pub fn version_matches_all(version: &str, ranges: &[String]) -> bool {
    let Ok(ver) = Version::parse(version) else {
        return false;
    };
    ranges.iter().all(|r| {
        match VersionReq::parse(r) {
            Ok(req) => req.matches(&ver),
            // Unparseable range (e.g. `file:`/`workspace:`) can't conflict.
            Err(_) => true,
        }
    })
}

/// Compatibility of a dep version chosen for install against the locked
/// peer-dependency ranges in the tree. Returns the list of peer range
/// descriptions that *would* break at `candidate`.
pub fn peer_conflicts_for(
    candidate: &str,
    peer_dep_ranges: &[(String, String)],
) -> Vec<String> {
    let candidate_ver = Version::parse(candidate).unwrap_or_else(|_| Version::new(0, 0, 0));
    peer_dep_ranges
        .iter()
        .filter(|(_, range)| {
            VersionReq::parse(range)
                .map(|req| !req.matches(&candidate_ver))
                .unwrap_or(false)
        })
        .map(|(pkg, range)| format!("{pkg}@{range}"))
        .collect()
}


/// Best-effort version of the active node/npm, parsing the `vX.Y.Z` prefix
/// out of their `--version` output.
pub fn version_from_output(output: &str) -> String {
    for token in output.split_whitespace() {
        let token = token.trim_start_matches('v');
        if Version::parse(token).is_ok() {
            return token.to_string();
        }
    }
    output.trim().to_string()
}

/// True when any of `pkg_names` shows up in `advisory_fields` (used by the
/// panel to mark installed packages that appear in the audit list).
pub fn has_vuln(pkg_names: &[String], audit: &[NpmAuditVuln]) -> bool {
    pkg_names
        .iter()
        .any(|name| audit.iter().any(|a| a.package == *name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn installed(name: &str, version: &str) -> NpmInstalledPkg {
        NpmInstalledPkg {
            name: name.to_string(),
            version: version.to_string(),
            path: None,
            is_dev: false,
        }
    }

    fn peers(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(name, range)| (name.to_string(), range.to_string()))
            .collect()
    }

    #[test]
    fn package_manager_follows_the_lockfile() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_string_lossy().into_owned();
        assert_eq!(detect_package_manager(&root), PackageManager::Npm);

        std::fs::write(dir.path().join("package-lock.json"), "{}").unwrap();
        assert_eq!(detect_package_manager(&root), PackageManager::Npm);

        std::fs::write(dir.path().join("bun.lock"), "").unwrap();
        assert_eq!(detect_package_manager(&root), PackageManager::Bun);

        std::fs::write(dir.path().join("yarn.lock"), "").unwrap();
        assert_eq!(detect_package_manager(&root), PackageManager::Yarn);

        // pnpm is checked first, so it wins when several lockfiles coexist.
        std::fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();
        assert_eq!(detect_package_manager(&root), PackageManager::Pnpm);
    }

    #[test]
    fn package_manager_cli_names() {
        assert_eq!(PackageManager::Npm.cli_name(), "npm");
        assert_eq!(PackageManager::Yarn.cli_name(), "yarn");
        assert_eq!(PackageManager::Pnpm.cli_name(), "pnpm");
        assert_eq!(PackageManager::Bun.cli_name(), "bun");
    }

    #[test]
    fn installed_classic_shape_takes_names_from_keys() {
        let raw = json!({
            "name": "app",
            "dependencies": {
                "zod": { "version": "3.23.8" },
                "@types/node": { "version": "20.11.0", "dev": true, "path": "/app/node_modules/@types/node" },
                "Express": { "version": "4.19.2" },
                "broken": {}
            }
        })
        .to_string();

        let packages = parse_installed(&raw).unwrap();
        let summary: Vec<_> = packages
            .iter()
            .map(|p| (p.name.as_str(), p.version.as_str(), p.is_dev))
            .collect();
        assert_eq!(
            summary,
            vec![
                ("@types/node", "20.11.0", true),
                ("broken", "", false),
                ("Express", "4.19.2", false),
                ("zod", "3.23.8", false),
            ]
        );
        assert_eq!(packages[0].path.as_deref(), Some("/app/node_modules/@types/node"));
        assert_eq!(packages[3].path, None);
    }

    #[test]
    fn installed_lockfile_shape_keeps_direct_installs_only() {
        let raw = json!({
            "packages": {
                "": { "name": "app", "version": "1.0.0" },
                "node_modules/react": { "version": "18.2.0" },
                "node_modules/@scope/widget": { "version": "2.0.0", "dev": true },
                "node_modules/named": { "name": "real-name", "version": "1.1.0" },
                "node_modules/react/node_modules/loose-envify": { "version": "1.4.0" },
                "packages/local": { "version": "0.0.1" }
            }
        })
        .to_string();

        let packages = parse_installed(&raw).unwrap();
        let summary: Vec<_> = packages
            .iter()
            .map(|p| (p.name.as_str(), p.version.as_str(), p.is_dev))
            .collect();
        assert_eq!(
            summary,
            vec![
                ("@scope/widget", "2.0.0", true),
                ("react", "18.2.0", false),
                ("real-name", "1.1.0", false),
            ]
        );
        assert_eq!(packages[1].path.as_deref(), Some("node_modules/react"));
    }

    #[test]
    fn installed_rejects_unrecognized_output() {
        assert!(parse_installed(r#"{"name":"app"}"#).is_err());
        assert!(parse_installed("npm ERR! something").is_err());
        assert!(parse_installed(r#"{"dependencies":{}}"#).unwrap().is_empty());
    }

    #[test]
    fn outdated_rows_are_sorted_and_incomplete_ones_dropped() {
        let raw = json!({
            "zod": { "current": "3.22.0", "wanted": "3.23.8", "latest": "3.23.8", "location": "node_modules/zod" },
            "react": { "current": "17.0.2", "wanted": "17.0.2", "latest": "18.2.0" },
            "missing-install": { "wanted": "1.0.0", "latest": "1.0.0" },
            "no-latest": { "current": "1.0.0", "wanted": "1.0.1" }
        })
        .to_string();

        let outdated = parse_outdated(&raw).unwrap();
        let names: Vec<_> = outdated.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["missing-install", "react", "zod"]);
        assert_eq!(outdated[0].current, "");
        assert_eq!(outdated[1].latest, "18.2.0");
        assert_eq!(outdated[1].location, None);
        assert_eq!(outdated[2].location.as_deref(), Some("node_modules/zod"));
    }

    #[test]
    fn outdated_non_object_output_means_up_to_date() {
        assert!(parse_outdated("{}").unwrap().is_empty());
        assert!(parse_outdated("[]").unwrap().is_empty());
        assert!(parse_outdated("").is_err());
    }

    #[test]
    fn audit_modern_shape() {
        let raw = json!({
            "vulnerabilities": {
                "lodash": {
                    "severity": "high",
                    "via": [
                        "some-dependency",
                        { "title": "Prototype Pollution", "url": "https://example.test/lodash", "exploitability": 3 }
                    ],
                    "fixAvailable": { "name": "lodash", "version": "4.17.21" }
                },
                "transitive-only": {
                    "severity": "moderate",
                    "via": ["lodash"],
                    "fixAvailable": true
                },
                "bare": {}
            }
        })
        .to_string();

        let vulns = parse_audit(&raw).unwrap();
        let packages: Vec<_> = vulns.iter().map(|v| v.package.as_str()).collect();
        assert_eq!(packages, vec!["bare", "lodash", "transitive-only"]);

        assert_eq!(vulns[0].severity, "unknown");
        assert_eq!(vulns[0].title, "");
        assert_eq!(vulns[0].fixed_in, None);

        assert_eq!(vulns[1].severity, "high");
        assert_eq!(vulns[1].title, "Prototype Pollution");
        assert_eq!(vulns[1].url, "https://example.test/lodash");
        assert_eq!(vulns[1].exploitability, Some(3));
        assert_eq!(vulns[1].fixed_in.as_deref(), Some("4.17.21"));

        // `via` holding only package names carries no advisory text, and a
        // boolean `fixAvailable` names no version.
        assert_eq!(vulns[2].title, "");
        assert_eq!(vulns[2].url, "");
        assert_eq!(vulns[2].fixed_in, None);
    }

    #[test]
    fn audit_legacy_shape() {
        let raw = json!({
            "advisories": {
                "1523": {
                    "module_name": "minimist",
                    "severity": "low",
                    "title": "Prototype Pollution",
                    "url": "https://example.test/1523",
                    "patched_versions": ">=1.2.3"
                },
                "9999": { "severity": "high" }
            }
        })
        .to_string();

        let vulns = parse_audit(&raw).unwrap();
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].package, "minimist");
        assert_eq!(vulns[0].severity, "low");
        assert_eq!(vulns[0].title, "Prototype Pollution");
        assert_eq!(vulns[0].url, "https://example.test/1523");
        assert_eq!(vulns[0].exploitability, None);
        assert_eq!(vulns[0].fixed_in.as_deref(), Some(">=1.2.3"));
    }

    #[test]
    fn audit_clean_and_invalid_output() {
        assert!(parse_audit(r#"{"vulnerabilities":{}}"#).unwrap().is_empty());
        assert!(parse_audit("{}").unwrap().is_empty());
        assert!(parse_audit("npm ERR!").is_err());
    }

    #[test]
    fn search_results_and_total() {
        let json = json!({
            "total": 250,
            "objects": [
                {
                    "downloads": 1234,
                    "package": {
                        "name": "left-pad",
                        "version": "1.3.0",
                        "description": "String left pad",
                        "engines": { "node": ">=4" }
                    }
                },
                { "package": { "version": "1.0.0" } },
                { "score": 1 },
                { "package": { "name": "bare" } }
            ]
        });

        let (results, total) = parse_npm_search(&json);
        assert_eq!(total, 250);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "left-pad");
        assert_eq!(results[0].version, "1.3.0");
        assert_eq!(results[0].description.as_deref(), Some("String left pad"));
        assert_eq!(results[0].downloads, Some(1234));
        assert_eq!(results[0].engines_node.as_deref(), Some(">=4"));
        assert_eq!(results[0].compat, None);
        assert_eq!(results[1].name, "bare");
        assert_eq!(results[1].version, "");
        assert_eq!(results[1].downloads, None);
    }

    #[test]
    fn search_total_never_undercounts_the_page() {
        let json = json!({ "objects": [{ "package": { "name": "a" } }, { "package": { "name": "b" } }] });
        let (results, total) = parse_npm_search(&json);
        assert_eq!(results.len(), 2);
        assert_eq!(total, 2);

        let (results, total) = parse_npm_search(&json!({}));
        assert!(results.is_empty());
        assert_eq!(total, 0);
    }

    #[test]
    fn packument_details() {
        let json = json!({
            "name": "widget",
            "description": "top-level description",
            "dist-tags": { "latest": "10.0.0" },
            "readme": "# Widget",
            "versions": {
                "9.0.0": { "peerDependencies": { "react": "^17.0.0" } },
                "10.0.0": {
                    "description": "latest description",
                    "license": "MIT",
                    "homepage": "https://example.test/widget",
                    "peerDependencies": { "react": "^18.0.0", "ignored": 5 }
                },
                "2.5.0": {}
            }
        });

        let details = parse_npm_packument(&json).unwrap();
        assert_eq!(details.name, "widget");
        assert_eq!(details.version, "10.0.0");
        assert_eq!(details.description.as_deref(), Some("latest description"));
        assert_eq!(details.license.as_deref(), Some("MIT"));
        assert_eq!(details.homepage.as_deref(), Some("https://example.test/widget"));
        assert_eq!(details.readme.as_deref(), Some("# Widget"));
        assert_eq!(details.weekly_downloads, None);

        // Semver order, not string order: 10.0.0 sorts above 9.0.0.
        let versions: Vec<_> = details.versions.iter().map(|v| v.version.as_str()).collect();
        assert_eq!(versions, vec!["10.0.0", "9.0.0", "2.5.0"]);
        assert_eq!(details.versions[0].peer_deps, peers(&[("react", "^18.0.0")]));
        assert_eq!(details.versions[1].peer_deps, peers(&[("react", "^17.0.0")]));
        assert!(details.versions[2].peer_deps.is_empty());
    }

    #[test]
    fn packument_fallbacks() {
        let json = json!({
            "name": "legacy",
            "version": "1.0.0",
            "description": "top-level description",
            "license": { "type": "Apache-2.0" },
            "readme": "No README found"
        });
        let details = parse_npm_packument(&json).unwrap();
        assert_eq!(details.version, "1.0.0");
        assert_eq!(details.description.as_deref(), Some("top-level description"));
        assert_eq!(details.license.as_deref(), Some("Apache-2.0"));
        assert_eq!(details.readme, None);
        assert!(details.versions.is_empty());

        let with_url = json!({ "name": "x", "license": { "type": "MIT", "url": "https://example.test/license" } });
        assert_eq!(
            parse_npm_packument(&with_url).unwrap().license.as_deref(),
            Some("https://example.test/license")
        );

        assert!(parse_npm_packument(&json!({ "version": "1.0.0" })).is_err());
    }

    #[test]
    fn weekly_downloads() {
        assert_eq!(parse_npm_downloads(&json!({ "downloads": 98765, "package": "x" })), Some(98765));
        assert_eq!(parse_npm_downloads(&json!({ "error": "not found" })), None);
    }

    #[test]
    fn node_engine_compatibility() {
        assert_eq!(node_engine_compatible("20.11.0", ""), Some(true));
        assert_eq!(node_engine_compatible("20.11.0", " * "), Some(true));
        assert_eq!(node_engine_compatible("20.11.0", ">=18"), Some(true));
        assert_eq!(node_engine_compatible("16.20.0", ">=18"), Some(false));
        // npm's own syntax: alternatives and space-separated conditions.
        assert_eq!(node_engine_compatible("20.11.0", "^18 || ^20"), Some(true));
        assert_eq!(node_engine_compatible("22.22.3", ">=18.17.1 <19.0.0"), Some(false));
        // Unknown rather than a guess when either side can't be read.
        assert_eq!(node_engine_compatible("v20", ">=18"), None);
        assert_eq!(node_engine_compatible("20.11.0", "lts/iron"), None);
    }

    #[test]
    fn peer_conflicts_against_installed_packages() {
        let tree = [installed("react", "18.2.0"), installed("weird", "not-semver")];
        let conflicts = peer_conflicts_with_installed(
            &peers(&[
                ("react", "^17.0.0"),
                ("react", "^18.0.0"),
                ("not-installed", "^1.0.0"),
                ("react", "workspace:*"),
                ("weird", "^1.0.0"),
            ]),
            &tree,
        );
        // An unparseable installed version is treated as 0.0.0.
        assert_eq!(conflicts, vec!["react@^17.0.0", "weird@^1.0.0"]);
    }

    #[test]
    fn peer_conflicts_for_a_candidate_version() {
        let ranges = peers(&[("a", "^17.0.0"), ("b", ">=18"), ("c", "file:../c")]);
        assert_eq!(peer_conflicts_for("18.2.0", &ranges), vec!["a@^17.0.0"]);
        assert_eq!(peer_conflicts_for("17.0.2", &ranges), vec!["b@>=18"]);
    }

    #[test]
    fn version_must_match_every_parseable_range() {
        let ranges = vec!["^18.0.0".to_string(), ">=18.2".to_string(), "workspace:*".to_string()];
        assert!(version_matches_all("18.2.0", &ranges));
        assert!(!version_matches_all("18.1.0", &ranges));
        assert!(!version_matches_all("latest", &ranges));
        assert!(version_matches_all("1.0.0", &[]));
    }

    #[test]
    fn classify_update_by_changed_component() {
        assert_eq!(classify_update("1.2.3", "2.0.0"), UpdateKind::Major);
        assert_eq!(classify_update("1.2.3", "1.3.0"), UpdateKind::Minor);
        assert_eq!(classify_update("1.2.3", "1.2.4"), UpdateKind::Patch);
        assert_eq!(classify_update("1.2.3", "1.2.3"), UpdateKind::Current);
    }

    #[test]
    fn classify_update_non_semver() {
        assert_eq!(classify_update("github:user/repo", "github:user/repo"), UpdateKind::Current);
        assert_eq!(classify_update("github:user/repo", "1.0.0"), UpdateKind::Major);
    }


    #[test]
    fn version_from_cli_output() {
        assert_eq!(version_from_output("v20.11.0\n"), "20.11.0");
        assert_eq!(version_from_output("10.2.4"), "10.2.4");
        assert_eq!(version_from_output("npm 10.2.4 on win32"), "10.2.4");
        assert_eq!(version_from_output("  command not found  "), "command not found");
    }

    #[test]
    fn has_vuln_matches_by_package_name() {
        let audit = parse_audit(r#"{"vulnerabilities":{"lodash":{"severity":"high"}}}"#).unwrap();
        assert!(has_vuln(&["react".to_string(), "lodash".to_string()], &audit));
        assert!(!has_vuln(&["react".to_string()], &audit));
        assert!(!has_vuln(&[], &audit));
    }
}
