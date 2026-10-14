//! `SpawnProfile`: the ONE builder for the `omp --mode rpc-ui` child (ADR-0020).
//!
//! Spawn and respawn both go through [`SpawnProfile::prepare`]. It produces the
//! exact argv, the scrubbed environment (ADR-0015 allow-list) and writes the
//! cedian `--config` overlay into the session dir (never the user's `.omp/`,
//! §77). Any failure fails closed: there is no bare-spawn fallback.
//!
//! Precedence (verified against pinned OMP `config/settings.ts`): global <
//! project < `--config` overlay < runtime overrides (`--approval-mode`). Scalars
//! and arrays in the overlay replace project values; `tools.approval` is a
//! deep-merged record, so every exec-tier tool cedian cares about is pinned by
//! name here — a project `tools.approval.bash: allow` must not survive.

use crate::OmpError;
use serde_json::{Map, Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

/// OMP `tools.approvalMode` values cedian may pass. `yolo` is not representable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApprovalMode {
    /// Read + in-workspace write tiers run; exec tier prompts (ADR-0023 default).
    #[default]
    Write,
    /// Only read tier runs; write and exec prompt.
    AlwaysAsk,
}

impl ApprovalMode {
    /// Parse an OMP mode string. `yolo` (and anything else) is rejected.
    pub fn parse(s: &str) -> Result<Self, OmpError> {
        match s {
            "write" => Ok(Self::Write),
            "always-ask" => Ok(Self::AlwaysAsk),
            other => Err(OmpError::InvalidSpawnProfile(format!(
                "approval mode {other:?} not allowed (want write|always-ask)"
            ))),
        }
    }

    /// OMP wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Write => "write",
            Self::AlwaysAsk => "always-ask",
        }
    }
}

/// OMP `tools.approval.<tool>` / `bash.patterns[].approval` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPolicy {
    Allow,
    Prompt,
    Deny,
}

impl ToolPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Prompt => "prompt",
            Self::Deny => "deny",
        }
    }
}

/// One ordered `bash.patterns` rule (OMP supports only `*` wildcards).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BashRule {
    pub pattern: String,
    pub approval: ToolPolicy,
}

/// Pure exec-tier OMP tools pinned in every overlay. Arg-dependent tools
/// (`gh`, `debug`, `lsp`) are left to the mode: a user policy outranks the
/// mode, so pinning them would prompt on their read-tier calls too.
pub const EXEC_TOOLS: &[&str] = &["bash", "eval", "browser", "task", "vibe_spawn", "vibe_send"];

/// Who decides OMP's approval mode and `computer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approvals {
    /// The default profile (ADR-0020): `--approval-mode`, the exec-tier
    /// prompt pins, the eval gate and `computer.enabled: false`.
    Cedian(ApprovalMode),
    /// A project opted in to the user's own OMP config (ADR-0035): cedian
    /// sets no mode, no prompt pins and no `computer` key. Only the
    /// interactive CLI may build this; reviewers and automations never do.
    Omp,
    /// A reviewer (ADR-0041): `always-ask`, every write and exec-tier tool
    /// denied except `bash`, which runs only through an allow pattern; a
    /// config `allow` cedian does not name is pinned to `deny`.
    Reviewer,
}

impl Approvals {
    fn label(self) -> &'static str {
        match self {
            Self::Cedian(_) => "cedian",
            Self::Omp => "omp",
            Self::Reviewer => "reviewer",
        }
    }

    /// The `--approval-mode` cedian passes, if it names one.
    fn mode(self) -> Option<ApprovalMode> {
        match self {
            Self::Cedian(mode) => Some(mode),
            Self::Omp => None,
            Self::Reviewer => Some(ApprovalMode::AlwaysAsk),
        }
    }
}

/// Tools a reviewer may never run: it reads and reports, nothing else.
const REVIEWER_DENY: &[&str] = &[
    "edit",
    "write",
    "ast_edit",
    "eval",
    "browser",
    "task",
    "vibe_spawn",
    "vibe_send",
    // On whatever `--tools` says: OMP always loads these.
    "manage_skill",
    "learn",
];

/// The reviewer's whole built-in tool set (ADR-0043): `--tools` turns every
/// other built-in off, and the flags after it keep a workspace from adding
/// extensions, skills or language servers. An explicit empty
/// `--append-system-prompt` replaces OMP's `APPEND_SYSTEM.md` lookup, so
/// neither the user's nor the workspace's file reaches a reviewer (ADR-0051).
const REVIEWER_TOOLS: &str = "read,grep,glob,bash";
const REVIEWER_FLAGS: &[&str] = &[
    "--no-extensions",
    "--no-skills",
    "--no-lsp",
    "--append-system-prompt",
    "",
];

/// The policy half of the profile, mapped from cedian settings by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnPolicy {
    pub approvals: Approvals,
    /// Per-tool overrides layered over the `EXEC_TOOLS` → `prompt` floor.
    /// Under [`Approvals::Omp`] only the `deny` entries are written.
    pub tool_policies: BTreeMap<String, ToolPolicy>,
    /// Under [`Approvals::Omp`] only the `deny` rules are written.
    pub bash_patterns: Vec<BashRule>,
    /// Host tools cedian serves over RPC. OMP gives them no tier (so `exec`,
    /// which `write` mode prompts for); cedian gates them itself at its own
    /// gate (ADR-0012), so the overlay allows exactly these names. A
    /// `tool_policies` entry still overrides (a `deny` stays a `deny`).
    pub host_tools: BTreeSet<String>,
    /// Tools the user's OMP config (global + project, as OMP merges them)
    /// sets to `allow`. The overlay cannot delete those keys, so the default
    /// profile pins each one it does not already name (ADR-0041 decision 2).
    pub config_allows: BTreeSet<String>,
    /// `--model` for this child; `None` leaves OMP's own routing.
    pub model: Option<String>,
    /// Run under `sandbox-exec -f` the layout's profile, with the overlay
    /// beside it and the child's temp and state roots inside its run dir.
    pub sandbox: Option<crate::sandbox::ReviewerLayout>,
    /// The app's browser endpoint (ADR-0049): OMP attaches there instead of
    /// launching its own browser or relaying the person's Chrome, under every
    /// approvals source but the reviewer's, whose tool set has no browser
    /// (ADR-0043). Must be `http://127.0.0.1:<port>`.
    pub browser_cdp_url: Option<String>,
}

impl Default for SpawnPolicy {
    fn default() -> Self {
        Self {
            approvals: Approvals::Cedian(ApprovalMode::Write),
            tool_policies: BTreeMap::new(),
            bash_patterns: crate::protected::default_protected_bash_rules(),
            host_tools: BTreeSet::new(),
            config_allows: BTreeSet::new(),
            model: None,
            sandbox: None,
            browser_cdp_url: None,
        }
    }
}

impl SpawnPolicy {
    /// The `tools.approval` record: exec floor (default profile only),
    /// host-tool allows, then caller overrides (only denies under the opt-in:
    /// a cedian Deny still wins, ADR-0012).
    fn approval_record(&self) -> Result<Map<String, Value>, OmpError> {
        let mut record = Map::new();
        match self.approvals {
            Approvals::Cedian(_) => {
                for tool in EXEC_TOOLS {
                    record.insert((*tool).to_string(), json!(ToolPolicy::Prompt.as_str()));
                }
            }
            Approvals::Reviewer => {
                for tool in REVIEWER_DENY {
                    record.insert((*tool).to_string(), json!(ToolPolicy::Deny.as_str()));
                }
                record.insert("bash".to_string(), json!(ToolPolicy::Prompt.as_str()));
            }
            Approvals::Omp => {}
        }
        for tool in &self.host_tools {
            check_tool_name(tool)?;
            if EXEC_TOOLS.contains(&tool.as_str()) {
                return Err(OmpError::InvalidSpawnProfile(format!(
                    "host tool {tool:?} shadows an OMP exec tool"
                )));
            }
            record.insert(tool.clone(), json!(ToolPolicy::Allow.as_str()));
        }
        let pin = match self.approvals {
            Approvals::Cedian(_) => Some(ToolPolicy::Prompt),
            Approvals::Reviewer => Some(ToolPolicy::Deny),
            Approvals::Omp => None,
        };
        if let Some(pin) = pin {
            // Any key OMP accepted is safe as a JSON key; MCP names may carry
            // `-` or `:`, which `check_tool_name` would refuse.
            for tool in &self.config_allows {
                record
                    .entry(tool.clone())
                    .or_insert_with(|| json!(pin.as_str()));
            }
        }
        for (tool, policy) in &self.tool_policies {
            // The eval gate: eval runs Python/JS outside `bash.patterns`, so it
            // is never auto-approved (ADR-0020 §3).
            if tool == "eval" && *policy == ToolPolicy::Allow {
                return Err(OmpError::InvalidSpawnProfile(
                    "tools.approval.eval: allow is forbidden (eval gate)".to_string(),
                ));
            }
            check_tool_name(tool)?;
            if self.approvals == Approvals::Omp && *policy != ToolPolicy::Deny {
                continue;
            }
            record.insert(tool.clone(), json!(policy.as_str()));
        }
        Ok(record)
    }

    /// The overlay document. JSON is valid YAML 1.2, which OMP's loader parses.
    pub fn overlay(&self) -> Result<Value, OmpError> {
        let patterns: Vec<Value> = self
            .bash_patterns
            .iter()
            .filter(|rule| self.approvals != Approvals::Omp || rule.approval == ToolPolicy::Deny)
            .map(|rule| {
                if rule.pattern.trim().is_empty() || rule.pattern.chars().any(char::is_control) {
                    return Err(OmpError::InvalidSpawnProfile(format!(
                        "bad bash pattern: {:?}",
                        rule.pattern
                    )));
                }
                Ok(json!({"match": rule.pattern, "approval": rule.approval.as_str()}))
            })
            .collect::<Result<_, _>>()?;
        let approval = Value::Object(self.approval_record()?);
        let bash = json!({"patterns": patterns, "allowCompoundCommands": false});
        let mut overlay = match self.approvals {
            Approvals::Cedian(_) | Approvals::Reviewer => {
                let mut overlay = json!({
                    // Until ADR-0008's atomic landing (driver + Seatbelt + bypass test).
                    "computer": {"enabled": false},
                    // Worktrees are cedian's (ADR-0009, ADR-0050): OMP's
                    // `task` never makes its own isolated copy.
                    "task": {"isolation": {"enabled": false}},
                    "tools": {
                        "approvalMode": self.approvals.mode().map(ApprovalMode::as_str),
                        "approval": approval,
                    },
                    "bash": bash,
                });
                if self.approvals == Approvals::Reviewer {
                    // A workspace's `.mcp.json` would add tools (ADR-0043).
                    overlay["mcp"] = json!({"enableProjectConfig": false});
                }
                overlay
            }
            Approvals::Omp if patterns.is_empty() => json!({"tools": {"approval": approval}}),
            Approvals::Omp => json!({"tools": {"approval": approval}, "bash": bash}),
        };
        if self.approvals == Approvals::Reviewer {
            overlay["browser"] = json!({"relay": false});
        } else if let Some(url) = &self.browser_cdp_url {
            check_browser_url(url)?;
            overlay["browser"] = json!({"cdpUrl": url, "relay": false});
        }
        Ok(overlay)
    }
}

fn check_browser_url(url: &str) -> Result<(), OmpError> {
    let loopback = url
        .strip_prefix("http://127.0.0.1:")
        .is_some_and(|port| port.parse::<u16>().is_ok_and(|p| p != 0));
    if !loopback {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "browser endpoint {url:?} is not http://127.0.0.1:<port>"
        )));
    }
    Ok(())
}

fn check_tool_name(tool: &str) -> Result<(), OmpError> {
    if tool.is_empty() || !tool.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "bad tool name in policy: {tool:?}"
        )));
    }
    Ok(())
}

/// Where OMP keeps this child's sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sessions {
    /// In `session_dir` (`--session-dir`): cedian-private sessions.
    InSessionDir,
    /// Where OMP's own CLI keeps them for this project, so a session started
    /// in either opens in the other (ADR-0040 decision 5). `session_dir`
    /// then holds only the overlay.
    OmpDefault,
}

/// Inputs for one OMP child. The dedupe key is `{binary_path, session_dir,
/// cwd, approvals}`, so a child is never reused under the other profile.
#[derive(Debug, Clone)]
pub struct SpawnProfile {
    /// Absolute path to the `omp` binary.
    pub binary_path: PathBuf,
    /// cedian's directory for this child: the overlay is written inside it,
    /// and with [`Sessions::InSessionDir`] it is `--session-dir` too.
    pub session_dir: PathBuf,
    pub sessions: Sessions,
    /// Workspace root the child runs in.
    pub cwd: PathBuf,
    pub policy: SpawnPolicy,
}

/// What [`SpawnProfile::prepare`] hands the process launcher.
#[derive(Debug, Clone)]
pub struct SpawnPlan {
    /// Exact argv, `argv[0]` = binary.
    pub argv: Vec<String>,
    /// The complete child environment (the launcher clears the rest).
    pub env: Vec<(String, String)>,
    /// Dedupe identity: `{binary_path, session_dir, cwd, approvals}`.
    pub dedupe_key: String,
    /// Where the overlay was written.
    pub overlay_path: PathBuf,
}

/// File name of the generated overlay inside the session dir.
pub const OVERLAY_FILE: &str = "cedian-overlay.yml";

const MAX_ARGS: usize = 64;
const MAX_ARG_BYTES: usize = 8 * 1024;

/// Environment variables the OMP child may inherit, by exact name. Allow-list,
/// never deny-list (ADR-0015/0020): no `SSH_AUTH_SOCK`, no cloud/forge
/// credentials. Provider auth comes from OMP's own auth store under `HOME`.
pub const ENV_ALLOW: &[&str] = &[
    "PATH",
    "HOME",
    "USER",
    "LOGNAME",
    "SHELL",
    "TMPDIR",
    "TERM",
    "TZ",
    "LANG",
    "LC_ALL",
    "LC_CTYPE",
    "XDG_CONFIG_HOME",
    "XDG_DATA_HOME",
    "XDG_CACHE_HOME",
    "XDG_STATE_HOME",
    // Which agent dir OMP reads: cedian and the user's CLI see one config
    // (ADR-0045 decision 4). Directory names, not credentials.
    "PI_CODING_AGENT_DIR",
    "OMP_PROFILE",
];

/// Filter `vars` down to [`ENV_ALLOW`], in allow-list order.
pub fn scrub_env(vars: impl IntoIterator<Item = (String, String)>) -> Vec<(String, String)> {
    let vars: BTreeMap<String, String> = vars.into_iter().collect();
    ENV_ALLOW
        .iter()
        .filter_map(|name| vars.get(*name).map(|v| ((*name).to_string(), v.clone())))
        .collect()
}

pub(crate) fn check_binary(path: &Path) -> Result<(), OmpError> {
    check_path("binary_path", path).map(|_| ())
}

fn check_path(label: &str, path: &Path) -> Result<String, OmpError> {
    if !path.is_absolute() {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "{label} must be absolute: {}",
            path.display()
        )));
    }
    let s = path.to_string_lossy();
    if s.len() > MAX_ARG_BYTES || s.chars().any(|c| c.is_control() || c == '\'') {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "{label} rejected: control chars or too long"
        )));
    }
    Ok(s.into_owned())
}

impl SpawnProfile {
    /// Validate and build the argv + env without touching disk.
    pub fn plan(
        &self,
        parent_env: impl IntoIterator<Item = (String, String)>,
    ) -> Result<SpawnPlan, OmpError> {
        let binary = check_path("binary_path", &self.binary_path)?;
        let session_dir = check_path("session_dir", &self.session_dir)?;
        let cwd = check_path("cwd", &self.cwd)?;
        // Validate the overlay before anything is spawned (fail closed).
        self.policy.overlay()?;
        let overlay_path = match &self.policy.sandbox {
            Some(layout) => layout.dir.join(OVERLAY_FILE),
            None => self.session_dir.join(OVERLAY_FILE),
        };
        let overlay = check_path("overlay", &overlay_path)?;
        let mut argv: Vec<String> = Vec::new();
        if let Some(layout) = &self.policy.sandbox {
            let sbpl = check_path("sandbox_profile", &layout.profile())?;
            argv.extend([
                crate::sandbox::SANDBOX_EXEC.to_string(),
                "-f".to_string(),
                sbpl,
            ]);
        }
        argv.extend(
            [binary.as_str(), "--mode", "rpc-ui", "--cwd", cwd.as_str()].map(str::to_string),
        );
        if self.sessions == Sessions::InSessionDir {
            argv.extend(["--session-dir".to_string(), session_dir.clone()]);
        }
        if let Some(mode) = self.policy.approvals.mode() {
            argv.extend(["--approval-mode".to_string(), mode.as_str().to_string()]);
        }
        if self.policy.approvals == Approvals::Reviewer {
            argv.extend(["--tools".to_string(), REVIEWER_TOOLS.to_string()]);
            argv.extend(REVIEWER_FLAGS.iter().map(|f| (*f).to_string()));
        }
        if let Some(model) = &self.policy.model {
            if model.is_empty()
                || model.len() > MAX_ARG_BYTES
                || model.chars().any(|c| c.is_control() || c.is_whitespace())
            {
                return Err(OmpError::InvalidSpawnProfile(format!(
                    "bad model {model:?}"
                )));
            }
            argv.extend(["--model".to_string(), model.clone()]);
        }
        argv.extend(["--config".to_string(), overlay]);
        if argv.len() > MAX_ARGS {
            return Err(OmpError::InvalidSpawnProfile(
                "argv exceeds MAX_ARGS".to_string(),
            ));
        }
        let dedupe_key = format!(
            "{binary}:{session_dir}:{cwd}:{}",
            self.policy.approvals.label()
        );
        let mut env = scrub_env(parent_env);
        if let Some(layout) = &self.policy.sandbox {
            for (name, value) in layout.env() {
                env.retain(|(n, _)| *n != name);
                env.push((name, value));
            }
        }
        Ok(SpawnPlan {
            argv,
            env,
            dedupe_key,
            overlay_path,
        })
    }

    /// [`Self::plan`] against the current process env, then write the overlay.
    /// Any error means the runtime must not start.
    pub fn prepare(&self) -> Result<SpawnPlan, OmpError> {
        let plan = self.plan(std::env::vars())?;
        let body = serde_json::to_string_pretty(&self.policy.overlay()?)
            .map_err(|e| OmpError::InvalidSpawnProfile(format!("overlay encode: {e}")))?;
        std::fs::create_dir_all(&self.session_dir)
            .and_then(|()| std::fs::write(&plan.overlay_path, body))
            .map_err(|e| {
                OmpError::InvalidSpawnProfile(format!(
                    "overlay write {}: {e}",
                    plan.overlay_path.display()
                ))
            })?;
        Ok(plan)
    }
}

/// One of OMP's effective settings for `cwd`, as `omp config get <key>
/// --json` reports it (global and project config merged by OMP itself, so
/// cedian never re-implements the merge). Read-only: no agent, no session.
/// Runs with the scrubbed env and is killed at `timeout`.
pub fn omp_config_get(
    binary: &Path,
    cwd: &Path,
    key: &str,
    timeout: std::time::Duration,
) -> Result<Value, OmpError> {
    check_path("binary_path", binary)?;
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '.') {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "bad config key {key:?}"
        )));
    }
    let out = run_captured(binary, cwd, &["config", "get", key, "--json"], timeout)?;
    let parsed: Value = serde_json::from_str(&out)
        .map_err(|e| OmpError::Spawn(format!("omp config get {key}: {e}")))?;
    parsed
        .get("value")
        .cloned()
        .ok_or_else(|| OmpError::Spawn(format!("omp config get {key}: no value")))
}

/// The version `binary --version` prints (`omp/18.6.1` → `18.6.1`).
pub fn omp_version(binary: &Path, timeout: std::time::Duration) -> Result<String, OmpError> {
    check_path("binary_path", binary)?;
    let out = run_captured(binary, &std::env::temp_dir(), &["--version"], timeout)?;
    let first = out.split_whitespace().next().unwrap_or_default();
    let version = first.rsplit('/').next().unwrap_or_default();
    if version.is_empty() {
        return Err(OmpError::Spawn(format!("omp --version printed {out:?}")));
    }
    Ok(version.to_string())
}

/// Run `binary args` in `cwd` with the scrubbed env; its stdout, or an
/// error when it fails or outlives `timeout`.
fn run_captured(
    binary: &Path,
    cwd: &Path,
    args: &[&str],
    timeout: std::time::Duration,
) -> Result<String, OmpError> {
    use std::io::Read as _;
    use std::os::unix::process::CommandExt as _;
    let what = args.join(" ");
    let mut child = std::process::Command::new(binary)
        .args(args)
        .process_group(0)
        .current_dir(cwd)
        .env_clear()
        .envs(scrub_env(std::env::vars()))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| OmpError::Spawn(format!("omp {what}: {e}")))?;
    // Drain stdout while waiting: output larger than the pipe buffer
    // would otherwise block the child until the timeout.
    let (sent, read) = std::sync::mpsc::channel();
    if let Some(mut stdout) = child.stdout.take() {
        std::thread::spawn(move || {
            let mut out = String::new();
            let _ = stdout.read_to_string(&mut out);
            let _ = sent.send(out);
        });
    }
    // The whole group: a launcher's child holding stdout would otherwise
    // keep the reader thread blocked for good.
    let kill_group = |child: &std::process::Child| {
        if let Ok(pid) = i32::try_from(child.id()) {
            // SAFETY: signals only the group this call spawned.
            unsafe { libc::killpg(pid, libc::SIGKILL) };
        }
    };
    let deadline = std::time::Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|e| OmpError::Spawn(e.to_string()))?
        {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            kill_group(&child);
            let _ = child.wait();
            let _ = read.recv();
            return Err(OmpError::Timeout {
                command: what,
                after: Some(timeout),
            });
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    };
    // The launcher exited; what is left of its group gets until the deadline.
    let left = deadline.saturating_duration_since(std::time::Instant::now());
    let out = read.recv_timeout(left).unwrap_or_else(|_| {
        kill_group(&child);
        read.recv().unwrap_or_default()
    });
    if !status.success() {
        return Err(OmpError::Spawn(format!("omp {what}: {status}")));
    }
    Ok(out)
}

/// Resolve a bare binary name against `PATH` to an absolute path (dev lane).
pub fn resolve_on_path(name: &str, path_var: Option<&str>) -> Result<PathBuf, OmpError> {
    if Path::new(name).is_absolute() {
        return Ok(PathBuf::from(name));
    }
    if name.contains('/') {
        return Err(OmpError::InvalidSpawnProfile(format!(
            "binary must be a bare name or absolute: {name}"
        )));
    }
    path_var
        .into_iter()
        .flat_map(std::env::split_paths)
        .filter(|dir| dir.is_absolute())
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| OmpError::Spawn(format!("{name} not found on PATH")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A launcher that exits while its child keeps stdout open returns
    /// within the timeout, with what it printed.
    #[test]
    fn a_grandchild_holding_stdout_does_not_outlast_the_timeout() {
        let dir = std::env::temp_dir().join(format!("cedian-grandchild-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let script = dir.join("omp");
        std::fs::write(&script, "#!/bin/sh\necho 18.6.1\nsleep 20 &\n").unwrap();
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        let started = std::time::Instant::now();
        let out = run_captured(&script, &dir, &[], std::time::Duration::from_secs(2));
        let took = started.elapsed();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(took < std::time::Duration::from_secs(5), "took {took:?}");
        assert_eq!(out.unwrap().trim(), "18.6.1");
    }

    fn profile() -> SpawnProfile {
        SpawnProfile {
            binary_path: PathBuf::from("/Applications/cedian.app/Contents/Resources/omp"),
            session_dir: PathBuf::from("/tmp/t1"),
            sessions: Sessions::InSessionDir,
            cwd: PathBuf::from("/Users/u/work"),
            policy: SpawnPolicy::default(),
        }
    }

    fn env(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn omp_default_sessions_pass_no_session_dir_but_keep_the_overlay() {
        let mut p = profile();
        p.sessions = Sessions::OmpDefault;
        let plan = p.plan(Vec::new()).unwrap();
        assert!(
            !plan.argv.iter().any(|a| a == "--session-dir"),
            "{:?}",
            plan.argv
        );
        assert_eq!(
            plan.overlay_path,
            PathBuf::from("/tmp/t1/cedian-overlay.yml")
        );
    }

    #[test]
    fn golden_argv() {
        let plan = profile().plan(Vec::new()).unwrap();
        assert_eq!(
            plan.argv,
            [
                "/Applications/cedian.app/Contents/Resources/omp",
                "--mode",
                "rpc-ui",
                "--cwd",
                "/Users/u/work",
                "--session-dir",
                "/tmp/t1",
                "--approval-mode",
                "write",
                "--config",
                "/tmp/t1/cedian-overlay.yml",
            ]
        );
        assert!(
            !plan
                .argv
                .iter()
                .any(|a| a == "yolo" || a == "--auto-approve")
        );
    }

    #[test]
    fn golden_overlay() {
        let mut policy = SpawnPolicy::default();
        policy.bash_patterns.push(BashRule {
            pattern: "cargo test*".to_string(),
            approval: ToolPolicy::Allow,
        });
        let overlay = policy.overlay().unwrap();
        assert_eq!(overlay["computer"]["enabled"], false);
        assert_eq!(overlay["task"]["isolation"]["enabled"], false);
        assert_eq!(overlay["tools"]["approvalMode"], "write");
        assert_eq!(
            overlay["tools"]["approval"],
            json!({
                "bash": "prompt", "eval": "prompt", "browser": "prompt",
                "task": "prompt", "vibe_spawn": "prompt", "vibe_send": "prompt",
            })
        );
        let patterns = overlay["bash"]["patterns"].as_array().unwrap();
        assert!(
            patterns.iter().any(|p| {
                p["approval"] == "deny" && p["match"].as_str().unwrap().contains(".git")
            }),
            "default policy denies bash writes into .git: {patterns:?}"
        );
        assert_eq!(
            patterns.last().unwrap(),
            &json!({"match": "cargo test*", "approval": "allow"})
        );
        assert_eq!(overlay["bash"]["allowCompoundCommands"], false);
    }

    #[test]
    fn env_is_exact_allow_list() {
        let scrubbed = scrub_env(env(&[
            ("PATH", "/usr/bin"),
            ("HOME", "/Users/u"),
            ("SSH_AUTH_SOCK", "/tmp/agent"),
            ("AWS_SECRET_ACCESS_KEY", "x"),
            ("GH_TOKEN", "x"),
            ("GITHUB_TOKEN", "x"),
            ("OPENAI_API_KEY", "x"),
        ]));
        assert_eq!(scrubbed, env(&[("PATH", "/usr/bin"), ("HOME", "/Users/u")]));
    }

    #[test]
    fn yolo_rejected() {
        assert!(ApprovalMode::parse("yolo").is_err());
        assert_eq!(ApprovalMode::parse("write").unwrap(), ApprovalMode::Write);
    }

    #[test]
    fn eval_allow_rejected() {
        let mut p = profile();
        p.policy
            .tool_policies
            .insert("eval".to_string(), ToolPolicy::Allow);
        assert!(p.plan(Vec::new()).is_err());
    }

    #[test]
    fn deny_overrides_exec_floor() {
        let mut policy = SpawnPolicy::default();
        policy
            .tool_policies
            .insert("eval".to_string(), ToolPolicy::Deny);
        assert_eq!(
            policy.overlay().unwrap()["tools"]["approval"]["eval"],
            "deny"
        );
    }

    #[test]
    fn host_tools_allowed_but_never_shadow_exec_tools() {
        let mut policy = SpawnPolicy::default();
        policy
            .host_tools
            .insert("cedian_worktree_request".to_string());
        assert_eq!(
            policy.overlay().unwrap()["tools"]["approval"]["cedian_worktree_request"],
            "allow"
        );
        policy.host_tools.insert("bash".to_string());
        assert!(policy.overlay().is_err());
    }

    #[test]
    fn cedian_debug_is_gated_by_cedian_not_omp() {
        let mut policy = SpawnPolicy::default();
        policy.host_tools.insert("cedian_debug".to_string());
        policy
            .host_tools
            .insert("cedian_worktree_request".to_string());
        let overlay = policy.overlay().unwrap();
        let approval = &overlay["tools"]["approval"];
        assert_eq!(
            approval["cedian_debug"], "allow",
            "cedian's own gate asks, so OMP must not ask again (ADR-0073)"
        );
        assert_eq!(approval["cedian_worktree_request"], "allow");
        assert!(
            approval.get("debug").is_none(),
            "OMP's own debug stays as OMP ships it (ADR-0048 staging)"
        );
        assert!(
            overlay.get("debug").is_none(),
            "OMP's debug is never switched off"
        );
        assert!(!EXEC_TOOLS.contains(&"cedian_debug"));
    }

    #[test]
    fn config_allow_for_an_unnamed_tool_is_pinned_to_prompt() {
        let mut policy = SpawnPolicy::default();
        policy
            .host_tools
            .insert("cedian_worktree_request".to_string());
        for tool in ["some_mcp_tool", "cedian_worktree_request", "bash"] {
            policy.config_allows.insert(tool.to_string());
        }
        let overlay = policy.overlay().unwrap();
        let approval = &overlay["tools"]["approval"];
        assert_eq!(approval["some_mcp_tool"], "prompt", "ADR-0028 gap closed");
        assert_eq!(
            approval["cedian_worktree_request"], "allow",
            "host tools keep allow"
        );
        assert_eq!(approval["bash"], "prompt", "exec floor unchanged");
    }

    #[test]
    fn cedian_tool_policy_outranks_a_config_allow_pin() {
        let mut policy = SpawnPolicy::default();
        policy.config_allows.insert("some_mcp_tool".to_string());
        policy
            .tool_policies
            .insert("some_mcp_tool".to_string(), ToolPolicy::Deny);
        assert_eq!(
            policy.overlay().unwrap()["tools"]["approval"]["some_mcp_tool"],
            "deny"
        );
    }

    #[test]
    fn opt_in_leaves_config_allows_to_omp() {
        let mut policy = SpawnPolicy {
            approvals: Approvals::Omp,
            ..SpawnPolicy::default()
        };
        policy.config_allows.insert("some_mcp_tool".to_string());
        let overlay = policy.overlay().unwrap();
        assert!(overlay["tools"]["approval"].get("some_mcp_tool").is_none());
    }

    #[test]
    fn overlay_points_omp_at_the_apps_browser_under_every_policy_but_the_reviewers() {
        let url = "http://127.0.0.1:43123";
        let reviewer = SpawnPolicy {
            approvals: Approvals::Reviewer,
            browser_cdp_url: Some(url.to_string()),
            ..SpawnPolicy::default()
        };
        assert_eq!(
            reviewer.overlay().unwrap()["browser"],
            json!({"relay": false}),
            "the reviewer gets no browser (ADR-0043) and no relay"
        );
        for approvals in [Approvals::Cedian(ApprovalMode::Write), Approvals::Omp] {
            let policy = SpawnPolicy {
                approvals,
                browser_cdp_url: Some(url.to_string()),
                ..SpawnPolicy::default()
            };
            let overlay = policy.overlay().unwrap();
            assert_eq!(
                overlay["browser"],
                json!({"cdpUrl": url, "relay": false}),
                "{approvals:?}"
            );
        }
        let none = SpawnPolicy::default().overlay().unwrap();
        assert!(none.get("browser").is_none(), "no browser, no key");
        for bad in [
            "http://example.com:9222",
            "ws://127.0.0.1:1",
            "http://127.0.0.1:1/x y",
        ] {
            let policy = SpawnPolicy {
                browser_cdp_url: Some(bad.to_string()),
                ..SpawnPolicy::default()
            };
            assert!(policy.overlay().is_err(), "{bad} refused");
        }
    }

    #[test]
    fn overlay_keeps_omp_from_making_worktrees_except_under_policy_omp() {
        for approvals in [Approvals::Cedian(ApprovalMode::Write), Approvals::Reviewer] {
            let overlay = SpawnPolicy {
                approvals,
                ..SpawnPolicy::default()
            }
            .overlay()
            .unwrap();
            assert_eq!(
                overlay["task"]["isolation"]["enabled"], false,
                "{approvals:?}: worktrees are cedian's (ADR-0009, ADR-0050)"
            );
        }
        let omp = SpawnPolicy {
            approvals: Approvals::Omp,
            ..SpawnPolicy::default()
        }
        .overlay()
        .unwrap();
        assert!(
            omp.get("task").is_none(),
            "policy = \"omp\": the user's OMP config decides"
        );
    }

    fn reviewer() -> SpawnPolicy {
        let mut policy = SpawnPolicy {
            approvals: Approvals::Reviewer,
            ..SpawnPolicy::default()
        };
        policy
            .host_tools
            .insert("cedian_review_finding".to_string());
        policy.config_allows.insert("some_mcp_tool".to_string());
        policy.bash_patterns.push(BashRule {
            pattern: "git diff*".to_string(),
            approval: ToolPolicy::Allow,
        });
        policy
    }

    #[test]
    fn reviewer_overlay_is_read_only_and_asks_for_the_rest() {
        let overlay = reviewer().overlay().unwrap();
        assert_eq!(overlay["tools"]["approvalMode"], "always-ask");
        assert_eq!(overlay["computer"]["enabled"], false);
        let approval = &overlay["tools"]["approval"];
        for tool in [
            "edit",
            "write",
            "ast_edit",
            "eval",
            "browser",
            "task",
            "vibe_spawn",
            "vibe_send",
            "manage_skill",
            "learn",
        ] {
            assert_eq!(approval[tool], "deny", "{tool} is denied to a reviewer");
        }
        assert_eq!(
            overlay["mcp"]["enableProjectConfig"], false,
            "a workspace's MCP servers add no reviewer tools"
        );
        assert_eq!(
            approval["bash"], "prompt",
            "bash runs only by an allow pattern"
        );
        assert_eq!(approval["cedian_review_finding"], "allow");
        assert_eq!(
            approval["some_mcp_tool"], "deny",
            "unnamed config allow pinned to deny"
        );
        let patterns = overlay["bash"]["patterns"].as_array().unwrap();
        assert!(
            patterns
                .iter()
                .any(|p| { p["match"] == "git diff*" && p["approval"] == "allow" }),
            "reviewer allow pattern present: {patterns:?}"
        );
        assert!(
            patterns.iter().any(|p| {
                p["approval"] == "deny" && p["match"].as_str().unwrap().contains(".git")
            }),
            "protected .git deny still applies under reviewer: {patterns:?}"
        );
    }

    #[test]
    fn reviewer_argv_runs_under_sandbox_exec_on_its_model() {
        let mut p = profile();
        p.policy = reviewer();
        p.policy.model = Some("opencode-go/glm-5.3".to_string());
        p.policy.sandbox = Some(crate::sandbox::ReviewerLayout {
            dir: PathBuf::from("/tmp/t1"),
        });
        let plan = p
            .plan([
                (
                    "TMPDIR".to_string(),
                    "/private/var/folders/x/T/".to_string(),
                ),
                ("HOME".to_string(), "/Users/u".to_string()),
            ])
            .unwrap();
        assert_eq!(
            plan.argv[..4],
            [
                "/usr/bin/sandbox-exec",
                "-f",
                "/tmp/t1/reviewer.sbpl",
                "/Applications/cedian.app/Contents/Resources/omp"
            ]
        );
        let after = |flag: &str| {
            let i = plan.argv.iter().position(|a| a == flag).unwrap();
            plan.argv[i + 1].clone()
        };
        assert_eq!(after("--approval-mode"), "always-ask");
        assert_eq!(after("--model"), "opencode-go/glm-5.3");
        assert_eq!(after("--tools"), "read,grep,glob,bash");
        for flag in ["--no-extensions", "--no-skills", "--no-lsp"] {
            assert!(plan.argv.iter().any(|a| a == flag), "{flag}");
        }
        assert_eq!(
            after("--append-system-prompt"),
            "",
            "no APPEND_SYSTEM.md reaches a reviewer"
        );
        assert!(plan.dedupe_key.ends_with(":reviewer"));
        assert_eq!(
            plan.overlay_path,
            PathBuf::from("/tmp/t1/cedian-overlay.yml"),
            "the overlay sits outside the reviewer's writable run dir"
        );
        let env: BTreeMap<_, _> = plan.env.into_iter().collect();
        assert_eq!(
            env["TMPDIR"], "/tmp/t1/run/tmp",
            "never the shared temp dir"
        );
        assert_eq!(env["XDG_STATE_HOME"], "/tmp/t1/run/state");
        assert_eq!(env["HOME"], "/Users/u");
    }

    #[test]
    fn model_with_control_chars_is_refused() {
        let mut p = profile();
        p.policy.model = Some("x\ny".to_string());
        assert!(p.plan(Vec::new()).is_err());
    }

    #[test]
    fn relative_and_control_paths_rejected() {
        let mut p = profile();
        p.binary_path = PathBuf::from("omp");
        assert!(p.plan(Vec::new()).is_err());
        let mut p = profile();
        p.session_dir = PathBuf::from("/tmp/a\nb");
        assert!(p.plan(Vec::new()).is_err());
    }

    #[test]
    fn distinct_workspaces_distinct_keys() {
        let a = profile();
        let mut b = profile();
        b.cwd = PathBuf::from("/Users/u/other");
        assert_ne!(
            a.plan(Vec::new()).unwrap().dedupe_key,
            b.plan(Vec::new()).unwrap().dedupe_key
        );
    }

    #[test]
    fn missing_overlay_dir_fails_closed() {
        // A session dir under a regular file cannot hold the overlay.
        let file = std::env::temp_dir().join(format!("cedian-p1-file-{}", std::process::id()));
        std::fs::write(&file, "x").unwrap();
        let mut p = profile();
        p.session_dir = file.join("sessions");
        assert!(p.prepare().is_err());
        std::fs::remove_file(&file).unwrap();
    }

    #[test]
    fn prepare_writes_overlay_omp_can_read() {
        let dir = std::env::temp_dir().join(format!("cedian-p1-ok-{}", std::process::id()));
        let mut p = profile();
        p.session_dir = dir.clone();
        let plan = p.prepare().unwrap();
        let raw = std::fs::read_to_string(&plan.overlay_path).unwrap();
        let parsed: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed["computer"]["enabled"], false);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_hung_version_probe_takes_its_children_down() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = std::env::temp_dir().join(format!("cedian-hung-omp-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (omp, pid_file) = (dir.join("omp"), dir.join("pid"));
        std::fs::write(
            &omp,
            format!(
                "#!/bin/sh\n/bin/sleep 30 &\necho $! > {}\nwait\n",
                pid_file.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&omp, std::fs::Permissions::from_mode(0o755)).unwrap();
        let started = std::time::Instant::now();
        let probe = omp_version(&omp, std::time::Duration::from_secs(2));
        assert!(matches!(probe, Err(OmpError::Timeout { .. })), "{probe:?}");
        assert!(started.elapsed() < std::time::Duration::from_secs(8));
        let pid = std::fs::read_to_string(&pid_file).unwrap();
        let alive = || {
            std::process::Command::new("/bin/kill")
                .args(["-0", pid.trim()])
                .status()
                .unwrap()
                .success()
        };
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while alive() && std::time::Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(
            !alive(),
            "the probe's grandchild, which holds its stdout, is killed too"
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn resolve_on_path_finds_absolute() {
        let dir = std::env::temp_dir().join(format!("cedian-p1-bin-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("omp"), "").unwrap();
        let path_var = format!("relative/bin:{}", dir.display());
        assert_eq!(
            resolve_on_path("omp", Some(&path_var)).unwrap(),
            dir.join("omp")
        );
        assert!(resolve_on_path("./omp", Some(&path_var)).is_err());
        assert!(resolve_on_path("nope-omp", Some(&path_var)).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn opted_in() -> SpawnProfile {
        let mut p = profile();
        p.policy.approvals = Approvals::Omp;
        p.policy
            .host_tools
            .insert("cedian_worktree_request".to_string());
        p.policy
            .tool_policies
            .insert("write".to_string(), ToolPolicy::Deny);
        p.policy
            .tool_policies
            .insert("lsp".to_string(), ToolPolicy::Prompt);
        p.policy.bash_patterns = vec![
            BashRule {
                pattern: "rm -rf *".to_string(),
                approval: ToolPolicy::Deny,
            },
            BashRule {
                pattern: "git push*".to_string(),
                approval: ToolPolicy::Prompt,
            },
        ];
        p
    }

    #[test]
    fn opt_in_argv_names_no_approval_mode() {
        let argv = opted_in().plan(Vec::new()).unwrap().argv;
        assert!(!argv.iter().any(|a| a == "--approval-mode"), "{argv:?}");
        assert!(argv.iter().any(|a| a == "--config"), "overlay still passed");
    }

    #[test]
    fn opt_in_overlay_keeps_only_host_allows_and_cedian_denies() {
        let overlay = opted_in().policy.overlay().unwrap();
        assert_eq!(
            overlay,
            json!({
                "tools": {"approval": {"cedian_worktree_request": "allow", "write": "deny"}},
                "bash": {
                    "patterns": [{"match": "rm -rf *", "approval": "deny"}],
                    "allowCompoundCommands": false,
                },
            })
        );
        let mut bare = opted_in();
        bare.policy.bash_patterns.clear();
        assert_eq!(
            bare.policy.overlay().unwrap(),
            json!({"tools": {"approval": {"cedian_worktree_request": "allow", "write": "deny"}}})
        );
    }

    #[test]
    fn opt_in_still_rejects_eval_allow_and_shadowing() {
        let mut p = opted_in();
        p.policy
            .tool_policies
            .insert("eval".to_string(), ToolPolicy::Allow);
        assert!(p.plan(Vec::new()).is_err());
        let mut p = opted_in();
        p.policy.host_tools.insert("bash".to_string());
        assert!(p.plan(Vec::new()).is_err());
    }

    #[test]
    fn profiles_never_share_a_dedupe_key() {
        assert_ne!(
            profile().plan(Vec::new()).unwrap().dedupe_key,
            opted_in().plan(Vec::new()).unwrap().dedupe_key
        );
    }
}
