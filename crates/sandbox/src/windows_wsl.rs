//! Windows sandbox integration via WSL.
//!
//! Sandboxed Windows terminal commands are routed through WSL and then executed
//! under Bubblewrap inside Linux. Projects may be opened either from native
//! Windows paths (`C:\Users\...`) or WSL UNC paths
//! (`\\wsl.localhost\Ubuntu\home\...`). Native drive-letter paths are
//! translated into the distro's filesystem view with `wslpath` (falling back
//! to the conventional `/mnt/<drive>/...` mapping if that fails) and use the
//! user's default WSL distro unless a WSL UNC path in the request pins a
//! specific distro.
//!
//! Errors fall into two classes the agent treats differently:
//!
//! * **Environment unavailable** — WSL missing or failing to start, no
//!   usable `bwrap`, or the probe/path-resolution stdout protocol breaking
//!   down. These are returned as a [`WslSandboxUnavailable`] (whose `Display`
//!   carries
//!   [`WSL_SANDBOX_UNAVAILABLE_PREFIX`](crate::WSL_SANDBOX_UNAVAILABLE_PREFIX)),
//!   so the agent recognizes them *by type* and offers the same
//!   retry / run-unsandboxed fallback it offers on Linux, rather than matching
//!   on message text.
//! * **Bad request** — a specific path that doesn't exist or can't be mapped
//!   into WSL, or a request mixing distros. These are ordinary `anyhow` errors
//!   *without* [`WslSandboxUnavailable`], and are reported back to the model,
//!   which can fix the request and retry.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use smol::process::{Command, Stdio};

use anyhow::{Context as _, Result, bail, ensure};

use crate::{SandboxPermissions, WSL_SANDBOX_UNAVAILABLE_PREFIX};

/// Exit code the environment probe script uses to signal that `bwrap` is not
/// installed, distinguishing that from WSL itself failing to start a shell.
/// Chosen to be unlikely to collide with `wsl.exe`'s own failure codes.
const BWRAP_MISSING_EXIT_CODE: i32 = 41;

/// Exit code the environment probe script uses to signal that `bwrap` is
/// installed but failed the sandbox smoke test — typically because the
/// distro restricts unprivileged user namespaces (e.g. Ubuntu 24.04's
/// default AppArmor policy), which every namespace flag we pass depends on.
const BWRAP_UNUSABLE_EXIT_CODE: i32 = 42;

/// Prefix of the probe script's single result line, so it can be picked out
/// of any stdout noise printed by the login shell's profile scripts.
const PROBE_RESULT_PREFIX: &str = "zed-wsl-probe:";

/// Marks a failure of the Windows WSL sandboxing *environment*: WSL is missing
/// or won't start, there's no usable `bwrap`, or the probe / path-resolution
/// stdout protocol broke down. Returned as the root of the `anyhow::Error` so
/// callers classify it by type ([`anyhow::Error::downcast_ref`]) instead of by
/// matching message text. Per-request failures (a missing writable path, paths
/// mixing distros) are ordinary `anyhow` errors *without* this type, so they
/// never match — the agent returns those to the model rather than offering to
/// run unsandboxed.
#[derive(Debug, Clone)]
pub struct WslSandboxUnavailable(String);

impl WslSandboxUnavailable {
    /// Build an environment-unavailable error from a human-readable reason
    /// (without the [`WSL_SANDBOX_UNAVAILABLE_PREFIX`], which `Display` adds).
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// The reason, without the leading [`WSL_SANDBOX_UNAVAILABLE_PREFIX`].
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WslSandboxUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WSL_SANDBOX_UNAVAILABLE_PREFIX}: {}", self.0)
    }
}

impl std::error::Error for WslSandboxUnavailable {}

/// Shorthand for an [`anyhow::Error`] wrapping a [`WslSandboxUnavailable`].
fn unavailable(message: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(WslSandboxUnavailable::new(message))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct WslPath {
    distro: Option<String>,
    path: String,
}

/// A path mapped for use inside WSL.
///
/// WSL UNC and WSL-absolute paths can be mapped structurally up front. Native
/// drive-letter paths depend on the distro's automount configuration
/// (`/etc/wsl.conf` can move the `/mnt` root), so they are translated with
/// `wslpath` inside the distro — but a distro can only be chosen after every
/// path has been parsed (WSL UNC paths pin one), hence this two-stage shape:
/// parse structurally first, then resolve native paths via [`resolve_paths`]
/// once the distro is known.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PathMapping {
    Wsl(WslPath),
    NativeDrive {
        /// The `\\?\`-stripped, forward-slashed form that `wslpath -u`
        /// accepts (`wslpath` is a Linux binary and doesn't understand
        /// backslash separators).
        windows_path: String,
        /// The conventional `/mnt/<drive>/...` mapping, used when `wslpath`
        /// translation fails.
        fallback: WslPath,
    },
}

impl PathMapping {
    fn distro(&self) -> Option<&str> {
        match self {
            PathMapping::Wsl(path) => path.distro.as_deref(),
            PathMapping::NativeDrive { .. } => None,
        }
    }
}

/// Wrap a Linux process invocation so it runs under Bubblewrap inside WSL.
///
/// `program` and `args` must name a Linux executable and Linux argv, not a
/// Windows executable. The caller is expected to convert the model's command
/// into a Linux shell invocation (typically `/bin/sh -c ...`) before calling
/// this function.
///
/// All writable paths and the cwd must be paths that can be mapped into WSL.
/// WSL UNC paths may specify a distro; native drive-letter paths are
/// translated with `wslpath` inside either that distro or the default distro
/// (falling back to `/mnt/<drive>/...` if translation fails).
///
/// `env` is forwarded into the sandboxed command via `bwrap --setenv` rather
/// than being set on the `wsl.exe` process. Windows environment variables
/// don't cross the WSL boundary unless they're listed in `WSLENV`, so without
/// this the command would lose `PAGER` (used to stop `git` from paging into
/// the PTY) and the rest of the project environment. Variables whose Windows
/// values are meaningless or harmful inside Linux are dropped (see
/// [`is_forwardable_env_var`]).
///
/// This function performs up to two `wsl.exe` round-trips (environment probe
/// and path resolution, each cached) plus filesystem stats of WSL UNC paths,
/// any of which can take seconds when the WSL VM is cold (and the stats can
/// stall on a slow `\\wsl.localhost` filesystem). Run it on a background
/// executor, never on the UI thread, and bound it with a timeout — a wedged
/// `wsl.exe` (a real failure mode when the WSL service is unhealthy)
/// otherwise stalls the returned future forever. This crate deliberately has
/// no timer of its own (timers come from the caller's executor so tests stay
/// deterministic); instead it guarantees that dropping the future kills any
/// in-flight `wsl.exe` child, so a caller-side timeout that drops the future
/// also reaps the process. Parameters are owned so the returned future is
/// `Send + 'static`.
pub async fn wrap_invocation<S: std::hash::BuildHasher>(
    program: String,
    args: Vec<String>,
    writable_paths: Vec<PathBuf>,
    permissions: SandboxPermissions,
    cwd: Option<PathBuf>,
    env: HashMap<String, String, S>,
) -> Result<(String, Vec<String>)> {
    // Mapping failures are bad requests (a path that doesn't exist or has a
    // shape WSL can't address), not environment problems, so no
    // `WSL_SANDBOX_UNAVAILABLE_PREFIX` here.
    let cwd_mapping =
        match &cwd {
            Some(cwd) => Some(directory_to_wsl(cwd).with_context(|| {
                format!("failed to map terminal cwd `{}` into WSL", cwd.display())
            })?),
            None => None,
        };

    let writable_mappings = writable_paths
        .iter()
        .map(|path| {
            path_to_wsl(path).with_context(|| {
                format!("failed to map writable path `{}` into WSL", path.display())
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let distro = select_distro(cwd_mapping.as_ref(), &writable_mappings)?;
    let wsl_exe = wsl_exe_path();
    if !wsl_exe.is_file() {
        return Err(unavailable(format!(
            "WSL (`wsl.exe`) was not found at `{}`",
            wsl_exe.display()
        )));
    }
    let environment = probe_environment(&wsl_exe, distro.as_deref()).await?;

    // Resolve all paths (translating native drive-letter paths with `wslpath`
    // now that the distro is known) and confirm they exist, in a single WSL
    // round-trip.
    let has_cwd = cwd_mapping.is_some();
    let mut mappings = Vec::with_capacity(writable_mappings.len() + 1);
    if let Some(mapping) = cwd_mapping {
        mappings.push((mapping, "terminal cwd"));
    }
    mappings.extend(
        writable_mappings
            .into_iter()
            .map(|mapping| (mapping, "writable path")),
    );
    let mut resolved = resolve_paths(&wsl_exe, distro.as_deref(), &mappings)
        .await?
        .into_iter();
    let cwd = if has_cwd { resolved.next() } else { None };
    let writable_paths: Vec<String> = resolved.collect();

    let mut wsl_args = Vec::new();
    if let Some(distro) = distro.as_deref() {
        wsl_args.extend(["-d".to_string(), distro.to_string()]);
    }
    if let Some(cwd) = &cwd {
        wsl_args.extend(["--cd".to_string(), cwd.clone()]);
    }
    // Use the absolute path the probe validated: `wsl --exec` searches only
    // the default WSL PATH, which may not include a profile-managed location
    // where the probe's login shell found `bwrap`.
    wsl_args.extend(["--exec".to_string(), environment.bwrap_path.clone()]);
    wsl_args.extend(build_bwrap_args(
        &writable_paths,
        permissions,
        cwd.as_deref(),
        environment.mask_interop_dir,
        &env,
    ));
    wsl_args.push("--".to_string());
    wsl_args.push(program);
    wsl_args.extend(args);

    Ok((wsl_exe.to_string_lossy().into_owned(), wsl_args))
}

fn select_distro(
    cwd: Option<&PathMapping>,
    writable_paths: &[PathMapping],
) -> Result<Option<String>> {
    let mut distro = cwd.and_then(|mapping| mapping.distro().map(str::to_string));
    for mapping in writable_paths {
        let Some(path_distro) = mapping.distro() else {
            continue;
        };
        match distro.as_deref() {
            // A bad request, not an environment problem: the model (or
            // project layout) asked for paths spanning two distros, which a
            // single bwrap invocation can't serve.
            Some(distro) => ensure!(
                distro == path_distro,
                "cannot sandbox a command whose paths mix WSL distros `{}` and `{}`",
                distro,
                path_distro
            ),
            None => distro = Some(path_distro.to_string()),
        }
    }
    Ok(distro)
}

/// What [`probe_environment`] learned about a WSL distro.
#[derive(Clone, Debug, Eq, PartialEq)]
struct EnvironmentProbe {
    /// Whether the WSL interop socket directory (`/run/WSL`) exists and so
    /// must (and can) be masked — see [`build_bwrap_args`].
    mask_interop_dir: bool,
    /// Absolute path of the `bwrap` binary the smoke test validated. The real
    /// invocation must exec this exact path: `wsl --exec` searches only the
    /// default WSL PATH, so a bare `bwrap` could miss (or differ from) the
    /// binary the probe's login shell found.
    bwrap_path: String,
}

/// Shell script run by [`probe_environment`]. Resolves `bwrap` to an absolute
/// path (exit [`BWRAP_MISSING_EXIT_CODE`] if absent), rejects setuid-root
/// binaries, then smoke-tests a real minimal sandbox (exit
/// [`BWRAP_UNUSABLE_EXIT_CODE`] on failure) using the same mount and namespace
/// flags as [`build_bwrap_args`] — presence isn't
/// enough, because unprivileged user namespaces can be disabled by the
/// distro's kernel, sysctl, or AppArmor policy (notably Ubuntu 24.04, the
/// current default WSL distro), in which case `bwrap` exists but every
/// sandboxed command would fail. The interop mask is included in the smoke
/// test when `/run/WSL` exists so the exact mount we later perform is
/// exercised too. On success, one [`PROBE_RESULT_PREFIX`]-marked result line
/// reports the interop state and the resolved `bwrap` path.
fn probe_script() -> String {
    format!(
        "bwrap_path=$(command -v bwrap) || exit {BWRAP_MISSING_EXIT_CODE}; \
         if [ -u \"$bwrap_path\" ] && [ \"$(stat -c %u \"$bwrap_path\" 2>/dev/null)\" = 0 ]; then \
         echo 'setuid-root bwrap is not supported' >&2; \
         exit {BWRAP_UNUSABLE_EXIT_CODE}; fi; \
         if [ -d /run/WSL ]; then interop=interop; mask='--tmpfs /run/WSL'; \
         else interop=no-interop; mask=''; fi; \
         \"$bwrap_path\" --ro-bind / / --tmpfs /tmp $mask --dev /dev --proc /proc \
         --unshare-net --unshare-user --unshare-ipc --unshare-uts --unshare-pid \
         --unshare-cgroup-try --die-with-parent -- true >/dev/null \
         || exit {BWRAP_UNUSABLE_EXIT_CODE}; \
         printf '{PROBE_RESULT_PREFIX} %s %s\\n' \"$interop\" \"$bwrap_path\""
    )
}

/// Probe a distro's sandbox environment in one `wsl.exe` round-trip: confirm
/// a shell starts, confirm `bwrap` is installed *and can actually set up an
/// unprivileged sandbox* (see [`probe_script`]), and report whether the
/// interop socket directory exists.
///
/// Successful results are cached per distro for the life of the process —
/// like `linux_bubblewrap::is_available`, the answers can't realistically
/// change while Zed runs. Failures are deliberately *not* cached so a user
/// who installs `bwrap` (or lifts a user-namespace restriction) after seeing
/// the error can retry the command without restarting Zed.
async fn probe_environment(wsl_exe: &Path, distro: Option<&str>) -> Result<EnvironmentProbe> {
    static CACHE: OnceLock<Mutex<HashMap<Option<String>, EnvironmentProbe>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    let key = distro.map(str::to_string);
    if let Some(probe) = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&key)
    {
        return Ok(probe.clone());
    }

    // A login shell (`-lc`) is used so a `bwrap` reachable only through a
    // profile-managed PATH is still found; the resolved absolute path is
    // reported back so the real invocation execs the same binary.
    let script = probe_script();
    let output = run_wsl_command(
        wsl_exe,
        distro,
        ["--exec", "sh", "-lc", &script],
        "probe the sandbox environment",
    )
    .await?;
    if output.status.code() == Some(BWRAP_MISSING_EXIT_CODE) {
        return Err(unavailable(format!(
            "Bubblewrap (`bwrap`) is not installed in {}",
            wsl_distro_label(distro)
        )));
    }
    if output.status.code() == Some(BWRAP_UNUSABLE_EXIT_CODE) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        return Err(unavailable(format!(
            "Bubblewrap (`bwrap`) is installed in {} but could not set up a sandbox — the \
             distro may restrict unprivileged user namespaces (as Ubuntu 24.04's default \
             AppArmor policy does){}",
            wsl_distro_label(distro),
            if stderr.is_empty() {
                String::new()
            } else {
                format!(": {stderr}")
            }
        )));
    }
    if !output.status.success() {
        return Err(unavailable(format!(
            "failed to start a shell in {}{}",
            wsl_distro_label(distro),
            command_failure_details(output.status.code(), &output.stderr)
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let probe = parse_probe_output(&stdout).map_err(|error| {
        unavailable(format!(
            "unexpected sandbox probe output from {}: {error:#}",
            wsl_distro_label(distro)
        ))
    })?;
    cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key, probe.clone());
    Ok(probe)
}

/// Parse [`probe_script`] output: the last [`PROBE_RESULT_PREFIX`]-marked
/// line wins, so stdout noise from login-shell profile scripts (which runs
/// before the script body) is ignored.
fn parse_probe_output(stdout: &str) -> Result<EnvironmentProbe> {
    let line = stdout
        .lines()
        .rev()
        .find_map(|line| line.strip_prefix(PROBE_RESULT_PREFIX))
        .with_context(|| format!("no probe result line in: {stdout:?}"))?;
    let (interop, bwrap_path) = line
        .trim_start()
        .split_once(' ')
        .with_context(|| format!("malformed probe result line: {line:?}"))?;
    let mask_interop_dir = match interop {
        "interop" => true,
        "no-interop" => false,
        _ => bail!("malformed probe result line: {line:?}"),
    };
    ensure!(
        bwrap_path.starts_with('/'),
        "`bwrap` resolved to {bwrap_path:?} rather than an absolute path; a shell \
         alias or function named `bwrap` cannot be run with `wsl --exec`"
    );
    Ok(EnvironmentProbe {
        mask_interop_dir,
        bwrap_path: bwrap_path.to_string(),
    })
}

/// Shell script that resolves and existence-checks paths in a single WSL
/// round-trip. Arguments come in triples `(kind, path, fallback)`: kind `W`
/// is a native Windows path to translate with `wslpath -u` (falling back to
/// the precomputed `/mnt/<drive>/...` mapping when translation fails), kind
/// `L` is an already-Linux path with an empty fallback. One result line is
/// printed per triple: `<ok|fallback> <ok|missing> <resolved path>`.
const PATH_RESOLUTION_SCRIPT: &str = "\
    while [ \"$#\" -ge 3 ]; do \
        kind=$1; path=$2; fallback=$3; shift 3; translate=ok; \
        if [ \"$kind\" = W ]; then \
            resolved=$(wslpath -u \"$path\" 2>/dev/null) || { resolved=$fallback; translate=fallback; }; \
        else resolved=$path; fi; \
        exists=ok; [ -e \"$resolved\" ] || exists=missing; \
        printf '%s %s %s\\n' \"$translate\" \"$exists\" \"$resolved\"; \
    done";

/// A line of [`PATH_RESOLUTION_SCRIPT`] output, parsed.
#[derive(Debug, Eq, PartialEq)]
struct ResolvedPath {
    path: String,
    used_fallback: bool,
    exists: bool,
}

/// Resolve path mappings into final WSL paths and confirm they exist.
/// Native drive-letter paths are translated with `wslpath -u` inside the
/// chosen distro so its actual automount configuration is honored, falling
/// back to the structural `/mnt/<drive>` mapping when translation fails
/// (e.g. a distro without `wslpath`); a wrong fallback is still caught by
/// the existence check.
///
/// Successful resolutions are memoized per `(distro, mapping)` for the life
/// of the process, so a steady-state command whose paths have all been seen
/// before resolves with zero `wsl.exe` round-trips; at most one round-trip
/// handles all cache misses ([`resolve_uncached_paths`]). A hit reuses the
/// translation — which only changes if the distro's automount configuration
/// is edited and the distro restarted — and also skips the WSL-side
/// existence re-check. That staleness is acceptable: native and UNC paths
/// are still stat'ed on the Windows side on every command (see
/// [`path_to_wsl`] / [`directory_to_wsl`]), and if a cached path disappears
/// mid-session bwrap fails closed on the missing bind source rather than
/// running the command unsandboxed. Failures are not cached, so a missing
/// path can be created and retried.
///
/// Each mapping is paired with a human-readable description used in errors.
/// The returned paths are in the same order as `mappings`.
async fn resolve_paths(
    wsl_exe: &Path,
    distro: Option<&str>,
    mappings: &[(PathMapping, &str)],
) -> Result<Vec<String>> {
    type ResolutionCache = HashMap<Option<String>, HashMap<PathMapping, String>>;
    static CACHE: OnceLock<Mutex<ResolutionCache>> = OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);

    let distro_key = distro.map(str::to_string);
    let mut resolved: Vec<Option<String>> = {
        let cache = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let per_distro = cache.get(&distro_key);
        mappings
            .iter()
            .map(|(mapping, _)| per_distro.and_then(|cached| cached.get(mapping)).cloned())
            .collect()
    };

    let misses: Vec<usize> = (0..mappings.len())
        .filter(|&index| resolved[index].is_none())
        .collect();
    if !misses.is_empty() {
        let miss_mappings: Vec<&(PathMapping, &str)> =
            misses.iter().map(|&index| &mappings[index]).collect();
        let miss_resolved = resolve_uncached_paths(wsl_exe, distro, &miss_mappings).await?;
        let mut cache = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let per_distro = cache.entry(distro_key).or_default();
        for (&index, path) in misses.iter().zip(miss_resolved) {
            per_distro.insert(mappings[index].0.clone(), path.clone());
            resolved[index] = Some(path);
        }
    }

    resolved
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .context("bug: a path mapping was left unresolved")
}

/// Resolve and existence-check mappings that weren't in the cache, in a
/// single `wsl.exe` round-trip. A non-login shell runs the script so profile
/// scripts can't pollute the stdout protocol.
async fn resolve_uncached_paths(
    wsl_exe: &Path,
    distro: Option<&str>,
    mappings: &[&(PathMapping, &str)],
) -> Result<Vec<String>> {
    let mut args = vec![
        "--exec".to_string(),
        "sh".to_string(),
        "-c".to_string(),
        PATH_RESOLUTION_SCRIPT.to_string(),
        // argv[0] for the script; the path triples follow as "$@".
        "zed-resolve-paths".to_string(),
    ];
    args.extend(path_resolution_args(
        mappings.iter().map(|(mapping, _)| mapping),
    ));
    let output = run_wsl_command(wsl_exe, distro, &args, "resolve sandbox paths").await?;
    if !output.status.success() {
        return Err(unavailable(format!(
            "failed to resolve sandbox paths in {}{}",
            wsl_distro_label(distro),
            command_failure_details(output.status.code(), &output.stderr)
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let resolved = parse_path_resolution_output(&stdout, mappings.len()).map_err(|error| {
        unavailable(format!(
            "failed to resolve sandbox paths in {}: {error:#}",
            wsl_distro_label(distro)
        ))
    })?;

    mappings
        .iter()
        .zip(resolved)
        .map(|((mapping, description), resolved)| {
            if resolved.used_fallback
                && let PathMapping::NativeDrive { windows_path, .. } = mapping
            {
                log::warn!(
                    "failed to translate `{windows_path}` with wslpath in {}; \
                     falling back to `{}`",
                    wsl_distro_label(distro),
                    resolved.path
                );
            }
            // A bad request (the path simply isn't there), not an
            // environment problem — the model can create it or fix the path
            // and retry, so no `WSL_SANDBOX_UNAVAILABLE_PREFIX`.
            ensure!(
                resolved.exists,
                "mapped {description} `{}` does not exist in {}",
                resolved.path,
                wsl_distro_label(distro)
            );
            Ok(resolved.path)
        })
        .collect()
}

/// Flatten path mappings into the `(kind, path, fallback)` argument triples
/// consumed by [`PATH_RESOLUTION_SCRIPT`].
fn path_resolution_args<'a>(mappings: impl Iterator<Item = &'a PathMapping>) -> Vec<String> {
    let mut args = Vec::new();
    for mapping in mappings {
        match mapping {
            PathMapping::Wsl(path) => {
                args.extend(["L".to_string(), path.path.clone(), String::new()]);
            }
            PathMapping::NativeDrive {
                windows_path,
                fallback,
            } => {
                args.extend(["W".to_string(), windows_path.clone(), fallback.path.clone()]);
            }
        }
    }
    args
}

/// Parse [`PATH_RESOLUTION_SCRIPT`] output: one strictly-formatted line per
/// input triple. Anything else (wrong line count, unknown status words, a
/// non-absolute path) means the stdout protocol was corrupted and is an error.
fn parse_path_resolution_output(stdout: &str, expected: usize) -> Result<Vec<ResolvedPath>> {
    let lines: Vec<&str> = stdout.lines().collect();
    ensure!(
        lines.len() == expected,
        "expected {expected} result lines from the path resolution script, got {}: {stdout:?}",
        lines.len()
    );
    lines
        .into_iter()
        .map(|line| {
            let mut parts = line.splitn(3, ' ');
            let (Some(translate), Some(exists), Some(path)) =
                (parts.next(), parts.next(), parts.next())
            else {
                bail!("malformed line from the path resolution script: {line:?}");
            };
            let used_fallback = match translate {
                "ok" => false,
                "fallback" => true,
                _ => bail!("malformed line from the path resolution script: {line:?}"),
            };
            let exists = match exists {
                "ok" => true,
                "missing" => false,
                _ => bail!("malformed line from the path resolution script: {line:?}"),
            };
            ensure!(
                path.starts_with('/'),
                "unexpected resolved path from the path resolution script: {path:?}"
            );
            Ok(ResolvedPath {
                path: path.to_string(),
                used_fallback,
                exists,
            })
        })
        .collect()
}

/// `CREATE_NO_WINDOW` process creation flag. `wsl.exe` is a console-subsystem
/// binary, so spawning it from a GUI process without this flag flashes a
/// console window. Defined locally because this crate doesn't depend on
/// `util` (whose command helpers normally take care of this).
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Invoke `wsl.exe` with the given args and return its raw output.
///
/// Only spawn failures become errors here; callers interpret the exit status
/// themselves. stdout, when used, is decoded as UTF-8 (lossily) — that's
/// only valid for `--exec`'d programs whose output we control, not for
/// `wsl.exe`'s own diagnostics (which are UTF-16LE).
///
/// `output()` spawns the child eagerly and the returned future owns it, so
/// with `kill_on_drop` the child can't outlive this future: a caller-side
/// timeout or cancellation that drops us also terminates a wedged `wsl.exe`
/// instead of leaking it.
async fn run_wsl_command(
    wsl_exe: &Path,
    distro: Option<&str>,
    args: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>,
    description: &str,
) -> Result<std::process::Output> {
    use smol::process::windows::CommandExt as _;

    let mut command = Command::new(wsl_exe);
    if let Some(distro) = distro {
        command.args(["-d", distro]);
    }
    command
        .args(args)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .creation_flags(CREATE_NO_WINDOW);

    command.output().await.map_err(|error| {
        unavailable(format!(
            "failed to invoke WSL while trying to {description}: {error:#}"
        ))
    })
}

fn command_failure_details(exit_code: Option<i32>, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    let exit_status = match exit_code {
        Some(code) => format!("exit code {code}"),
        None => "terminated by signal".to_string(),
    };
    if stderr.is_empty() {
        format!(" ({exit_status})")
    } else {
        format!(" ({exit_status}; stderr: {stderr})")
    }
}

fn wsl_distro_label(distro: Option<&str>) -> String {
    match distro {
        Some(distro) => format!("WSL distro `{distro}`"),
        None => "the default WSL distro".to_string(),
    }
}

fn wsl_exe_path() -> PathBuf {
    std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
        .join("System32")
        .join("wsl.exe")
}

fn build_bwrap_args<S: std::hash::BuildHasher>(
    writable_paths: &[String],
    permissions: SandboxPermissions,
    cwd: Option<&str>,
    mask_interop_dir: bool,
    env: &HashMap<String, String, S>,
) -> Vec<String> {
    let mut args = Vec::new();

    if permissions.allow_fs_write {
        push_bind(&mut args, "--bind", "/", "/");
    } else {
        push_bind(&mut args, "--ro-bind", "/", "/");
        args.extend(["--tmpfs".to_string(), "/tmp".to_string()]);
        for path in writable_paths {
            push_bind(&mut args, "--bind", path, path);
        }
    }

    // Block WSL's Windows interop, regardless of the requested permissions.
    // Without this, a sandboxed process can exec a Windows binary (e.g.
    // /mnt/c/Windows/System32/cmd.exe), which the kernel's binfmt handler
    // (`/init`) hands off to the Windows host over an AF_UNIX socket — running
    // fully outside bwrap and defeating both the filesystem and the network
    // restrictions. `/init` locates that socket via the $WSL_INTEROP
    // environment variable, so we drop it; and we mask the socket directory
    // (when it exists) so the value can't be rediscovered by listing
    // /run/WSL and re-exporting it. Both steps are required: unsetting the
    // variable alone is bypassable, and masking alone leaves the inherited
    // variable usable.
    args.extend(["--unsetenv".to_string(), "WSL_INTEROP".to_string()]);
    args.extend(["--unsetenv".to_string(), "WSLENV".to_string()]);
    if mask_interop_dir {
        args.extend(["--tmpfs".to_string(), "/run/WSL".to_string()]);
    }

    args.extend([
        "--dev".to_string(),
        "/dev".to_string(),
        "--proc".to_string(),
        "/proc".to_string(),
    ]);

    if !permissions.allow_network {
        args.push("--unshare-net".to_string());
    }

    args.extend([
        "--unshare-user".to_string(),
        "--unshare-ipc".to_string(),
        "--unshare-uts".to_string(),
        "--unshare-pid".to_string(),
        "--unshare-cgroup-try".to_string(),
        "--die-with-parent".to_string(),
    ]);

    // Forward the caller-provided environment into the command. Windows env
    // set on the `wsl.exe` process doesn't reach the Linux command, so we
    // re-apply it here on the sandbox's child instead.
    for (name, value) in env {
        if is_forwardable_env_var(name) {
            args.extend(["--setenv".to_string(), name.clone(), value.clone()]);
        }
    }

    if let Some(cwd) = cwd {
        args.extend(["--chdir".to_string(), cwd.to_string()]);
    }

    args
}

/// Whether an environment variable should be forwarded into the Linux sandbox.
///
/// `bwrap --setenv` calls `setenv(3)`, which rejects names that are empty or
/// contain `=`. Windows process environments include such entries — most
/// notably the per-drive current-directory pseudo-variables (`=C:`, `=D:`,
/// ...) Windows keeps in the environment block — so they must be skipped or
/// bwrap aborts with "setenv failed".
///
/// Beyond that, a few variables hold Windows-specific values that would be
/// meaningless or actively break the command inside WSL: `PATH` would shadow
/// WSL's own `PATH` and stop the shell from finding Linux executables, the
/// temp-dir variables point at Windows paths that don't exist in WSL (bwrap
/// provides a fresh tmpfs `/tmp` instead), and WSL interop variables would
/// undermine the explicit interop block above. Matched case-insensitively
/// because Windows environment variable names are.
fn is_forwardable_env_var(name: &str) -> bool {
    if name.is_empty() || name.contains('=') {
        return false;
    }
    const BLOCKED: [&str; 6] = ["PATH", "TMPDIR", "TMP", "TEMP", "WSL_INTEROP", "WSLENV"];
    !BLOCKED
        .iter()
        .any(|blocked| name.eq_ignore_ascii_case(blocked))
}

fn push_bind(args: &mut Vec<String>, flag: &str, source: &str, destination: &str) {
    args.extend([
        flag.to_string(),
        source.to_string(),
        destination.to_string(),
    ]);
}

fn directory_to_wsl(path: &Path) -> Result<PathMapping> {
    ensure!(
        path.is_dir(),
        "Windows sandboxing via WSL can only use an existing directory as cwd: {}",
        path.display()
    );
    map_path_to_wsl(path)
}

fn path_to_wsl(path: &Path) -> Result<PathMapping> {
    let path_string = path.to_string_lossy();
    if let Ok(path) = parse_wsl_absolute_path(&path_string) {
        return Ok(PathMapping::Wsl(path));
    }

    ensure!(
        path.is_dir() || path.is_file(),
        "Windows sandboxing via WSL can only grant existing files or directories: {}",
        path.display()
    );
    map_path_to_wsl(path)
}

fn map_path_to_wsl(path: &Path) -> Result<PathMapping> {
    let path_string = path.to_string_lossy();
    if let Ok(path) = parse_wsl_unc_path(&path_string) {
        return Ok(PathMapping::Wsl(path));
    }
    let fallback = parse_native_drive_path(&path_string)?;
    let windows_path = path_string
        .strip_prefix(r"\\?\")
        .unwrap_or(&path_string)
        .replace('\\', "/");
    Ok(PathMapping::NativeDrive {
        windows_path,
        fallback,
    })
}

fn parse_wsl_absolute_path(path: &str) -> Result<WslPath> {
    let path = path.replace('\\', "/");
    ensure!(
        path.starts_with('/') && !path.starts_with("//"),
        "path is not a WSL absolute path: {path}"
    );
    Ok(WslPath { distro: None, path })
}

fn parse_wsl_unc_path(path: &str) -> Result<WslPath> {
    let path = path.replace('/', "\\");
    let remainder = path
        .strip_prefix("\\\\wsl.localhost\\")
        .or_else(|| path.strip_prefix("\\\\wsl$\\"))
        .or_else(|| path.strip_prefix("\\\\?\\UNC\\wsl.localhost\\"))
        .or_else(|| path.strip_prefix("\\\\?\\UNC\\wsl$\\"))
        .with_context(|| format!("path is not a WSL UNC path: {path}"))?;

    let (distro, rest) = remainder
        .split_once('\\')
        .map(|(distro, rest)| (distro, Some(rest)))
        .unwrap_or((remainder, None));
    ensure!(
        !distro.is_empty(),
        "WSL UNC path is missing a distro name: {path}"
    );

    let linux_path = match rest {
        Some(rest) if !rest.is_empty() => format!("/{}", rest.replace('\\', "/")),
        _ => "/".to_string(),
    };

    Ok(WslPath {
        distro: Some(distro.to_string()),
        path: linux_path,
    })
}

fn parse_native_drive_path(path: &str) -> Result<WslPath> {
    let path = path
        .strip_prefix("\\\\?\\")
        .unwrap_or(path)
        .replace('\\', "/");
    let mut chars = path.chars();
    let Some(drive) = chars.next().filter(|drive| drive.is_ascii_alphabetic()) else {
        bail!("path is not a drive-letter Windows path: {path}");
    };
    ensure!(chars.next() == Some(':'), "path is not absolute: {path}");
    let rest = chars.as_str().trim_start_matches('/');
    let drive = drive.to_ascii_lowercase();
    let linux_path = if rest.is_empty() {
        format!("/mnt/{drive}")
    } else {
        format!("/mnt/{drive}/{rest}")
    };
    Ok(WslPath {
        distro: None,
        path: linux_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_invocation_future_is_send() {
        // Callers run `wrap_invocation` via `background_spawn`, which
        // requires a `Send` future. This fails to compile if, for example, a
        // cache `MutexGuard` is ever held across an await point.
        fn assert_send<T: Send>(_: T) {}
        assert_send(wrap_invocation(
            String::new(),
            Vec::new(),
            Vec::new(),
            SandboxPermissions::default(),
            None,
            HashMap::<String, String>::new(),
        ));
    }

    #[test]
    fn parse_wsl_localhost_path() {
        let path = parse_wsl_unc_path(r"\\wsl.localhost\Ubuntu\home\me\project").unwrap();
        assert_eq!(path.distro.as_deref(), Some("Ubuntu"));
        assert_eq!(path.path, "/home/me/project");
    }

    #[test]
    fn parse_wsl_dollar_path() {
        let path = parse_wsl_unc_path(r"\\wsl$\Debian\tmp").unwrap();
        assert_eq!(path.distro.as_deref(), Some("Debian"));
        assert_eq!(path.path, "/tmp");
    }

    #[test]
    fn parse_native_windows_path() {
        let path = parse_native_drive_path(r"C:\Users\me\project").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/mnt/c/Users/me/project");
    }

    #[test]
    fn parse_wsl_absolute_path_keeps_linux_path() {
        let path = parse_wsl_absolute_path("/home/me").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/home/me");
    }

    #[test]
    fn parse_wsl_absolute_path_rejects_unc_paths() {
        assert!(parse_wsl_absolute_path(r"\\server\share").is_err());
    }

    #[test]
    fn parse_verbatim_native_windows_path() {
        let path = parse_native_drive_path(r"\\?\D:\workspace").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/mnt/d/workspace");
    }

    #[test]
    fn rejects_unc_non_wsl_path() {
        assert!(parse_native_drive_path(r"\\server\share\project").is_err());
    }

    #[test]
    fn probe_output_reports_interop_and_bwrap_path() {
        let probe = parse_probe_output("zed-wsl-probe: interop /usr/bin/bwrap\n").unwrap();
        assert_eq!(
            probe,
            EnvironmentProbe {
                mask_interop_dir: true,
                bwrap_path: "/usr/bin/bwrap".to_string(),
            }
        );

        let probe =
            parse_probe_output("zed-wsl-probe: no-interop /home/me/.nix-profile/bin/bwrap\n")
                .unwrap();
        assert_eq!(
            probe,
            EnvironmentProbe {
                mask_interop_dir: false,
                bwrap_path: "/home/me/.nix-profile/bin/bwrap".to_string(),
            }
        );
    }

    #[test]
    fn probe_output_ignores_profile_noise_even_mentioning_interop() {
        // Login-shell profile scripts run before the probe body and may print
        // arbitrary text; only the marked result line counts.
        let probe = parse_probe_output(
            "welcome to my shell, interop fans\nzed-wsl-probe: no-interop /usr/bin/bwrap\n",
        )
        .unwrap();
        assert!(!probe.mask_interop_dir);
    }

    #[test]
    fn probe_output_rejects_missing_or_malformed_result_line() {
        assert!(parse_probe_output("").is_err());
        assert!(parse_probe_output("profile noise only\n").is_err());
        assert!(parse_probe_output("zed-wsl-probe: interop\n").is_err());
        assert!(parse_probe_output("zed-wsl-probe: maybe /usr/bin/bwrap\n").is_err());
    }

    #[test]
    fn probe_output_rejects_non_absolute_bwrap_path() {
        // `command -v` reports a bare name for shell functions and aliases,
        // which `wsl --exec` could never run.
        assert!(parse_probe_output("zed-wsl-probe: interop bwrap\n").is_err());
    }

    #[test]
    fn probe_script_smoke_tests_the_namespaces_the_real_invocation_uses() {
        // Presence isn't enough: unprivileged user namespaces can be
        // restricted (e.g. Ubuntu 24.04's AppArmor policy), so the probe must
        // actually exercise the namespace flags `build_bwrap_args` emits.
        let script = probe_script();
        for flag in [
            "--unshare-user",
            "--unshare-net",
            "--unshare-ipc",
            "--unshare-uts",
            "--unshare-pid",
            "--unshare-cgroup-try",
            "--ro-bind / /",
        ] {
            assert!(script.contains(flag), "probe script must contain {flag}");
        }
        assert!(script.contains("exit 41"));
        assert!(script.contains("exit 42"));
    }

    #[test]
    fn probe_script_rejects_setuid_root_bwrap_before_smoke_test() {
        let script = probe_script();
        let guard =
            "[ -u \"$bwrap_path\" ] && [ \"$(stat -c %u \"$bwrap_path\" 2>/dev/null)\" = 0 ]";
        let smoke_test = "\"$bwrap_path\" --ro-bind / /";
        let Some(guard_index) = script.find(guard) else {
            panic!("probe script must contain setuid-root guard: {script}");
        };
        let Some(smoke_test_index) = script.find(smoke_test) else {
            panic!("probe script must contain bwrap smoke test: {script}");
        };

        assert!(script.contains("setuid-root bwrap is not supported"));
        assert!(script.contains(&format!("exit {BWRAP_UNUSABLE_EXIT_CODE}; fi")));
        assert!(guard_index < smoke_test_index);
    }

    #[test]
    fn bwrap_denies_network_by_default() {
        let args = build_bwrap_args(
            &["/home/me/project".to_string()],
            SandboxPermissions::default(),
            Some("/home/me/project"),
            true,
            &HashMap::new(),
        );
        assert!(args.iter().any(|arg| arg == "--unshare-net"));
        assert!(
            args.windows(3)
                .any(|window| window == ["--bind", "/home/me/project", "/home/me/project"])
        );
    }

    #[test]
    fn bwrap_allows_network_when_requested() {
        let args = build_bwrap_args(
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: false,
            },
            None,
            true,
            &HashMap::new(),
        );
        assert!(!args.iter().any(|arg| arg == "--unshare-net"));
    }

    #[test]
    fn bwrap_binds_explicit_writable_file_paths() {
        let args = build_bwrap_args(
            &["/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md".to_string()],
            SandboxPermissions::default(),
            None,
            true,
            &HashMap::new(),
        );
        assert!(args.windows(3).any(|window| window
            == [
                "--bind",
                "/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md",
                "/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md"
            ]));
    }

    #[test]
    fn bwrap_blocks_wsl_interop_by_default() {
        let args = build_bwrap_args(
            &["/home/me/project".to_string()],
            SandboxPermissions::default(),
            Some("/home/me/project"),
            true,
            &HashMap::new(),
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--tmpfs", "/run/WSL"])
        );
    }

    #[test]
    fn bwrap_blocks_wsl_interop_even_with_fs_write() {
        let args = build_bwrap_args(
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: true,
            },
            None,
            true,
            &HashMap::new(),
        );
        // Interop is host code execution, not just a filesystem write, so it
        // stays blocked even when the user has granted unrestricted writes
        // and network.
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--tmpfs", "/run/WSL"])
        );
    }

    #[test]
    fn bwrap_skips_interop_dir_mask_when_absent() {
        // When the interop socket directory doesn't exist (interop disabled),
        // there's nothing to mask and a `--tmpfs /run/WSL` would abort bwrap,
        // so the mount must be omitted. Unsetting the variable is harmless and
        // stays.
        let args = build_bwrap_args(
            &[],
            SandboxPermissions::default(),
            None,
            false,
            &HashMap::new(),
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(!args.iter().any(|arg| arg == "/run/WSL"));
    }

    #[test]
    fn bwrap_forwards_env_via_setenv() {
        let env = HashMap::from([
            ("PAGER".to_string(), String::new()),
            ("CARGO_TERM_COLOR".to_string(), "always".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "PAGER", ""])
        );
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "CARGO_TERM_COLOR", "always"])
        );
    }

    #[test]
    fn bwrap_does_not_forward_wsl_interop_env() {
        let env = HashMap::from([
            (
                "WSL_INTEROP".to_string(),
                "/run/WSL/123_interop".to_string(),
            ),
            ("WsLeNv".to_string(), "WSL_INTEROP/u".to_string()),
            ("PAGER".to_string(), String::new()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);

        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSLENV"])
        );
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "PAGER", ""])
        );
        assert!(!args.windows(3).any(|window| {
            matches!(window, [flag, name, _]
                if flag.as_str() == "--setenv"
                    && name.eq_ignore_ascii_case("WSL_INTEROP"))
        }));
        assert!(!args.windows(3).any(|window| {
            matches!(window, [flag, name, _]
                if flag.as_str() == "--setenv"
                    && name.eq_ignore_ascii_case("WSLENV"))
        }));
    }

    #[test]
    fn bwrap_does_not_forward_windows_specific_env() {
        // These hold Windows paths/values that would break or be meaningless
        // inside WSL, so they must never cross the boundary. Names are matched
        // case-insensitively, as Windows env var names are.
        let env = HashMap::from([
            ("Path".to_string(), r"C:\Windows\System32".to_string()),
            (
                "TEMP".to_string(),
                r"C:\Users\me\AppData\Local\Temp".to_string(),
            ),
            (
                "Tmp".to_string(),
                r"C:\Users\me\AppData\Local\Temp".to_string(),
            ),
            ("TMPDIR".to_string(), r"C:\tmp".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(!args.iter().any(|arg| arg == "--setenv"));
    }

    #[test]
    fn bwrap_skips_env_names_setenv_would_reject() {
        // bwrap's `--setenv` calls `setenv(3)`, which rejects empty names and
        // names containing `=`. Windows environments include the per-drive
        // current-directory pseudo-variables (`=C:`, ...); forwarding them
        // would abort bwrap with "setenv failed".
        let env = HashMap::from([
            ("=C:".to_string(), r"C:\Users\me".to_string()),
            (String::new(), "value".to_string()),
            ("OK".to_string(), "value".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "OK", "value"])
        );
        assert_eq!(args.iter().filter(|arg| *arg == "--setenv").count(), 1);
    }

    #[test]
    fn select_distro_uses_wsl_distro_when_present() {
        let distro = select_distro(
            None,
            &[
                PathMapping::NativeDrive {
                    windows_path: "C:/project".to_string(),
                    fallback: WslPath {
                        distro: None,
                        path: "/mnt/c/project".to_string(),
                    },
                },
                PathMapping::Wsl(WslPath {
                    distro: Some("Ubuntu".to_string()),
                    path: "/home/me/project".to_string(),
                }),
            ],
        )
        .unwrap();
        assert_eq!(distro.as_deref(), Some("Ubuntu"));
    }

    #[test]
    fn bad_request_errors_do_not_claim_sandboxing_is_unavailable() {
        // Mixed distros and missing/unmappable paths are model-fixable bad
        // requests. They must not be typed as `WslSandboxUnavailable` (nor
        // carry its prefix), since the agent uses that type to offer the
        // run-unsandboxed fallback only for genuine environment failures.
        let mixed_distros = select_distro(
            Some(&PathMapping::Wsl(WslPath {
                distro: Some("Ubuntu".to_string()),
                path: "/home/me".to_string(),
            })),
            &[PathMapping::Wsl(WslPath {
                distro: Some("Debian".to_string()),
                path: "/home/me".to_string(),
            })],
        )
        .unwrap_err();
        assert!(
            mixed_distros
                .downcast_ref::<WslSandboxUnavailable>()
                .is_none()
        );
        assert!(!format!("{mixed_distros:#}").contains(WSL_SANDBOX_UNAVAILABLE_PREFIX));

        let missing_path =
            path_to_wsl(Path::new(r"C:\zed-test\definitely\does\not\exist-2769")).unwrap_err();
        assert!(
            missing_path
                .downcast_ref::<WslSandboxUnavailable>()
                .is_none()
        );
        assert!(!format!("{missing_path:#}").contains(WSL_SANDBOX_UNAVAILABLE_PREFIX));

        let unmappable_cwd = directory_to_wsl(Path::new(r"\\server\share\project")).unwrap_err();
        assert!(
            unmappable_cwd
                .downcast_ref::<WslSandboxUnavailable>()
                .is_none()
        );
        assert!(!format!("{unmappable_cwd:#}").contains(WSL_SANDBOX_UNAVAILABLE_PREFIX));
    }

    #[test]
    fn unavailable_errors_are_typed_and_prefixed() {
        // Environment failures are recognizable by type (so the agent doesn't
        // depend on message text) and still render with the shared prefix.
        let error = unavailable("Bubblewrap (`bwrap`) is not installed in the default WSL distro");
        let typed = error
            .downcast_ref::<WslSandboxUnavailable>()
            .expect("environment failure should downcast to WslSandboxUnavailable");
        assert_eq!(
            typed.message(),
            "Bubblewrap (`bwrap`) is not installed in the default WSL distro"
        );
        assert!(format!("{error:#}").starts_with(WSL_SANDBOX_UNAVAILABLE_PREFIX));
    }

    #[test]
    fn map_path_to_wsl_keeps_unc_paths_structural() {
        let mapping = map_path_to_wsl(Path::new(r"\\wsl.localhost\Ubuntu\home\me")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::Wsl(WslPath {
                distro: Some("Ubuntu".to_string()),
                path: "/home/me".to_string(),
            })
        );
    }

    #[test]
    fn map_path_to_wsl_defers_native_paths_to_wslpath() {
        let mapping = map_path_to_wsl(Path::new(r"C:\Users\me\project")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::NativeDrive {
                windows_path: "C:/Users/me/project".to_string(),
                fallback: WslPath {
                    distro: None,
                    path: "/mnt/c/Users/me/project".to_string(),
                },
            }
        );
    }

    #[test]
    fn map_path_to_wsl_strips_verbatim_prefix_for_wslpath() {
        let mapping = map_path_to_wsl(Path::new(r"\\?\D:\workspace")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::NativeDrive {
                windows_path: "D:/workspace".to_string(),
                fallback: WslPath {
                    distro: None,
                    path: "/mnt/d/workspace".to_string(),
                },
            }
        );
    }

    #[test]
    fn path_resolution_args_flattens_mappings_into_triples() {
        let mappings = [
            PathMapping::NativeDrive {
                windows_path: "C:/Users/me/project".to_string(),
                fallback: WslPath {
                    distro: None,
                    path: "/mnt/c/Users/me/project".to_string(),
                },
            },
            PathMapping::Wsl(WslPath {
                distro: Some("Ubuntu".to_string()),
                path: "/home/me/project".to_string(),
            }),
        ];
        assert_eq!(
            path_resolution_args(mappings.iter()),
            [
                "W",
                "C:/Users/me/project",
                "/mnt/c/Users/me/project",
                "L",
                "/home/me/project",
                "",
            ]
        );
    }

    #[test]
    fn parse_path_resolution_output_reads_one_line_per_path() {
        let resolved = parse_path_resolution_output(
            "ok ok /mnt/c/Users/me/project\nfallback missing /mnt/d/workspace\n",
            2,
        )
        .unwrap();
        assert_eq!(
            resolved,
            [
                ResolvedPath {
                    path: "/mnt/c/Users/me/project".to_string(),
                    used_fallback: false,
                    exists: true,
                },
                ResolvedPath {
                    path: "/mnt/d/workspace".to_string(),
                    used_fallback: true,
                    exists: false,
                },
            ]
        );
    }

    #[test]
    fn parse_path_resolution_output_keeps_spaces_in_paths() {
        let resolved =
            parse_path_resolution_output("ok ok /mnt/c/Users/me/My Documents/project\n", 1)
                .unwrap();
        assert_eq!(resolved[0].path, "/mnt/c/Users/me/My Documents/project");
    }

    #[test]
    fn parse_path_resolution_output_rejects_wrong_line_count() {
        assert!(parse_path_resolution_output("ok ok /a\n", 2).is_err());
        assert!(parse_path_resolution_output("ok ok /a\nok ok /b\n", 1).is_err());
    }

    #[test]
    fn parse_path_resolution_output_rejects_corrupted_lines() {
        assert!(parse_path_resolution_output("garbage\n", 1).is_err());
        assert!(parse_path_resolution_output("weird ok /a\n", 1).is_err());
        assert!(parse_path_resolution_output("ok weird /a\n", 1).is_err());
        assert!(parse_path_resolution_output("ok ok not-absolute\n", 1).is_err());
    }
}
