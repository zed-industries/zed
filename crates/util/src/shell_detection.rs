//! Discovery of installed shells for the terminal profile picker (P2).
//!
//! The public entry point is [`detect_available_shells`], a cached list of
//! shells found on the local machine. It backs the P3 "+"-menu "Detected
//! shells" section and the P2 profile validator.
//!
//! The heavy lifting is done by [`detect_available_shells_inner`], a pure
//! function that takes injectable inputs (file contents, env, and a
//! `path_exists` predicate). That makes the Unix and Windows code paths
//! unit-testable from any host platform without touching the real
//! filesystem.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use collections::HashSet;

use crate::shell::{get_windows_bash, get_windows_system_shell};
use crate::get_system_shell;

/// Where a [`DetectedShell`] came from. P3 uses this to group menu entries
/// (e.g. configured vs `/etc/shells` vs PATH-resolved).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellSource {
    /// Discovered by parsing `/etc/shells`.
    EtcShells,
    /// The login shell (`$SHELL` on Unix, system shell on Windows).
    LoginShell,
    /// One of the always-probed well-known locations (fallback when
    /// `/etc/shells` is unreadable or empty).
    KnownLocation,
    /// Resolved via the `PATH` environment variable (relative name in
    /// `/etc/shells`, or a basename that matched a `PATH` entry).
    Path,
    /// A Windows Subsystem for Linux distribution.
    Wsl,
}

/// A shell discovered on the local machine. P3 renders one menu entry per
/// `DetectedShell` that isn't shadowed by a configured profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedShell {
    /// Human-readable label, e.g. `bash`, `zsh`, or `Ubuntu` (for WSL).
    /// Duplicate basenames get `" (2)"`, `" (3)"` suffixes following the
    /// VSCode convention.
    pub label: String,
    /// Absolute path to the executable (or `wsl.exe` for WSL distros).
    pub program: PathBuf,
    /// Extra args needed to launch this shell into a useful interactive
    /// state. Currently only populated for WSL (`["-d", "<distro>"]`).
    pub args: Vec<String>,
    /// Provenance of the entry — see [`ShellSource`].
    pub source: ShellSource,
}

/// Host OS the detection should run against. The inner detection function
/// is platform-agnostic so that Unix tests can exercise the Windows code
/// path (and vice versa) without requiring a real platform match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Unix,
    Windows,
}

impl Platform {
    fn host() -> Self {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Unix
        }
    }
}

static DETECTED_SHELLS: LazyLock<Vec<DetectedShell>> = LazyLock::new(detect_available_shells_blocking);

/// Cached enumeration of installed shells on the local machine.
///
/// Backed by a `LazyLock` so the filesystem is walked at most once per
/// process. Tests should call [`detect_available_shells_inner`] directly
/// with injected inputs rather than this function.
pub fn detect_available_shells() -> &'static [DetectedShell] {
    &DETECTED_SHELLS
}

#[cfg(not(target_os = "windows"))]
fn read_etc_shells() -> Option<String> {
    std::fs::read_to_string("/etc/shells").ok()
}

#[cfg(target_os = "windows")]
fn read_etc_shells() -> Option<String> {
    None
}

fn detect_available_shells_blocking() -> Vec<DetectedShell> {
    let platform = Platform::host();
    let login_shell = if platform == Platform::Unix {
        get_system_shell()
    } else {
        get_windows_system_shell()
    };
    let path_env = std::env::var_os("PATH").map(|os| os.to_string_lossy().into_owned());
    let etc_shells = if platform == Platform::Unix {
        read_etc_shells()
    } else {
        None
    };
    let windir = if platform == Platform::Windows {
        std::env::var_os("WINDIR").map(PathBuf::from)
    } else {
        None
    };
    let path_exists = |p: &Path| p.exists();

    let mut shells = detect_available_shells_inner(
        platform,
        etc_shells.as_deref(),
        &login_shell,
        path_env.as_deref(),
        windir.as_deref(),
        &path_exists,
    );

    if platform == Platform::Windows {
        let wsl = enumerate_wsl_distros();
        shells.extend(wsl);
    }

    dedup_by_program(shells)
}

/// Pure, testable detector. Walks the supplied inputs without touching the
/// real filesystem — every existence check goes through `path_exists`.
///
/// * `etc_shells_content` — contents of `/etc/shells` (Unix only); `None`
///   triggers the well-known-paths fallback.
/// * `login_shell` — `$SHELL` (Unix) or the resolved Windows system shell.
/// * `path_env` — `PATH`/`Path` value used to resolve relative names; on
///   Unix the separator is `':'`, on Windows `';'`.
/// * `windir` — value of `%WINDIR%` (Windows only); `None` skips the
///   PowerShell/cmd probes. Injected rather than read from `std::env`
///   so tests can exercise those branches in parallel without mutating
///   global process state.
/// * `path_exists` — injection point for `Path::exists()` so tests can
///   simulate any filesystem layout.
pub fn detect_available_shells_inner(
    platform: Platform,
    etc_shells_content: Option<&str>,
    login_shell: &str,
    path_env: Option<&str>,
    windir: Option<&Path>,
    path_exists: &dyn Fn(&Path) -> bool,
) -> Vec<DetectedShell> {
    match platform {
        Platform::Unix => detect_unix_inner(
            etc_shells_content,
            login_shell,
            path_env,
            path_exists,
        ),
        Platform::Windows => {
            detect_windows_inner(login_shell, path_env, windir, path_exists)
        }
    }
}

fn detect_unix_inner(
    etc_shells_content: Option<&str>,
    login_shell: &str,
    path_env: Option<&str>,
    path_exists: &dyn Fn(&Path) -> bool,
) -> Vec<DetectedShell> {
    let mut shells: Vec<DetectedShell> = Vec::new();
    let mut seen_paths: HashSet<PathBuf> = HashSet::default();

    if let Some(content) = etc_shells_content {
        for raw in content.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(shell) = resolve_etc_shells_entry(line, path_env, path_exists) {
                if seen_paths.insert(shell.program.clone()) {
                    shells.push(shell);
                }
            }
        }
    }

    // Always include the login shell ($SHELL) even if absent from /etc/shells.
    if !login_shell.is_empty() {
        let login_path = PathBuf::from(login_shell);
        if path_exists(&login_path) && seen_paths.insert(login_path.clone()) {
            shells.push(DetectedShell {
                label: basename_label(&login_path),
                program: login_path,
                args: Vec::new(),
                source: ShellSource::LoginShell,
            });
        }
    }

    // If /etc/shells is missing or empty, fall back to probing a small set
    // of well-known absolute locations. This matches VSCode's behavior on
    // systems without /etc/shells (some containers, macOS variants).
    let probed_fallback = etc_shells_content
        .map(|c| c.lines().any(|l| !l.trim().is_empty() && !l.trim().starts_with('#')))
        .unwrap_or(false);
    if !probed_fallback {
        for known in ["/bin/bash", "/usr/bin/bash", "/bin/zsh", "/usr/bin/zsh", "/bin/sh", "/usr/bin/fish"] {
            let path = PathBuf::from(known);
            if path_exists(&path) && seen_paths.insert(path.clone()) {
                shells.push(DetectedShell {
                    label: basename_label(&path),
                    program: path,
                    args: Vec::new(),
                    source: ShellSource::KnownLocation,
                });
            }
        }
    }

    dedup_by_program(shells)
}

/// Resolve a single non-comment line from `/etc/shells` into a
/// [`DetectedShell`]. Returns `None` if the entry doesn't exist as a file
/// (or, for relative basenames, cannot be found on `PATH`).
fn resolve_etc_shells_entry(
    line: &str,
    path_env: Option<&str>,
    path_exists: &dyn Fn(&Path) -> bool,
) -> Option<DetectedShell> {
    let path = Path::new(line);
    if path.is_absolute() {
        if path_exists(path) {
            return Some(DetectedShell {
                label: basename_label(path),
                program: path.to_path_buf(),
                args: Vec::new(),
                source: ShellSource::EtcShells,
            });
        }
        return None;
    }

    // Relative entry (basename only by /etc/shells convention): try each
    // PATH directory in order. This mirrors VSCode's `findExecutable`.
    let name = path.file_name()?;
    let path_env = path_env?;
    for dir in path_env.split(':') {
        if dir.is_empty() {
            continue;
        }
        let candidate = Path::new(dir).join(name);
        if path_exists(&candidate) {
            return Some(DetectedShell {
                label: basename_label(&candidate),
                program: candidate,
                args: Vec::new(),
                source: ShellSource::Path,
            });
        }
    }
    None
}

fn detect_windows_inner(
    login_shell: &str,
    _path_env: Option<&str>,
    windir: Option<&Path>,
    path_exists: &dyn Fn(&Path) -> bool,
) -> Vec<DetectedShell> {
    let mut shells: Vec<DetectedShell> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::default();

    // The login shell value on Windows is whichever of pwsh/powershell/cmd
    // the existing LazyLock-based picker found first.
    if !login_shell.is_empty() {
        let program = PathBuf::from(login_shell);
        if path_exists(&program) && seen.insert(program.clone()) {
            shells.push(DetectedShell {
                label: basename_label(&program),
                program,
                args: Vec::new(),
                source: ShellSource::LoginShell,
            });
        }
    }

    // Always probe Windows PowerShell + cmd directly — they ship with the OS
    // and the user may want them even if `get_windows_system_shell` picked
    // pwsh.
    if let Some(windir) = windir {
        let powershell = windir
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        if path_exists(&powershell) && seen.insert(powershell.clone()) {
            shells.push(DetectedShell {
                label: "PowerShell".to_string(),
                program: powershell,
                args: Vec::new(),
                source: ShellSource::KnownLocation,
            });
        }

        let cmd = windir.join("System32").join("cmd.exe");
        if path_exists(&cmd) && seen.insert(cmd.clone()) {
            shells.push(DetectedShell {
                label: "Command Prompt".to_string(),
                program: cmd,
                args: Vec::new(),
                source: ShellSource::KnownLocation,
            });
        }
    }

    // Git Bash (Scoop shim, or alongside a git-for-windows install).
    if let Some(bash) = get_windows_bash() {
        let bash_path = PathBuf::from(bash);
        if path_exists(&bash_path) && seen.insert(bash_path.clone()) {
            shells.push(DetectedShell {
                label: "Git Bash".to_string(),
                program: bash_path,
                args: Vec::new(),
                source: ShellSource::Path,
            });
        }
    }

    dedup_by_program(shells)
}

/// Enumerate WSL distros via `wsl.exe -l -q`, returning one entry per
/// non-`docker-desktop*` distro on Windows builds >= 19041.
///
/// **Currently a stub.** Implementation deferred within P2 because:
/// 1. WSL probing requires spawning `wsl.exe`, which can't be exercised
///    from unit tests on non-Windows hosts.
/// 2. The output is UTF-16LE with a BOM and embedded NULs, requiring
///    careful decoding.
/// 3. The plan explicitly marks WSL optional within P2.
///
/// The [`ShellSource::Wsl`] variant is in place so P3 can render WSL
/// entries without further schema changes once this is filled in.
#[cfg(target_os = "windows")]
fn enumerate_wsl_distros() -> Vec<DetectedShell> {
    // TODO(p2-terminal-profiles): implement WSL distro enumeration.
    //                       Use `windows::Wdk::System::SystemServices::RtlGetVersion`
    //                       for the build-number >= 19041 gate (see
    //                       `crates/platform_title_bar/src/platforms/platform_windows.rs`
    //                       for the precedent). Decode the `wsl.exe -l -q`
    //                       output as UTF-16LE, strip the BOM, filter out
    //                       distros whose name starts with `docker-desktop`.
    //                       Emit:
    //                       DetectedShell {
    //                           label: <distro>,
    //                           program: PathBuf::from("wsl.exe"),
    //                           args: vec!["-d".to_string(), <distro>],
    //                           source: ShellSource::Wsl,
    //                       }
    Vec::new()
}

#[cfg(not(target_os = "windows"))]
fn enumerate_wsl_distros() -> Vec<DetectedShell> {
    Vec::new()
}

/// Strip duplicate programs (by raw path equality) and apply VSCode-style
/// `" (n)"` suffixes to entries sharing a label.
///
/// We intentionally dedup on the raw `program` string rather than
/// canonicalizing: canonicalization requires touching the real filesystem
/// (which would break the fs-free testable inner fn) and yields
/// platform-dependent results. `/etc/shells` rarely lists the same path
/// twice, so raw-path dedup is sufficient in practice. VSCode itself
/// falls back to raw-path comparison when `realpath` fails.
fn dedup_by_program(mut shells: Vec<DetectedShell>) -> Vec<DetectedShell> {
    let mut seen: HashSet<PathBuf> = HashSet::default();
    shells.retain(|shell| seen.insert(shell.program.clone()));
    apply_duplicate_label_suffixes(shells)
}

fn apply_duplicate_label_suffixes(mut shells: Vec<DetectedShell>) -> Vec<DetectedShell> {
    let mut label_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for shell in &mut shells {
        let base = shell.label.clone();
        let count = label_counts.entry(base.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            shell.label = format!("{base} ({count})");
        }
    }
    shells
}

fn basename_label(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn detected(label: &str, program: &str, source: ShellSource) -> DetectedShell {
        DetectedShell {
            label: label.to_string(),
            program: PathBuf::from(program),
            args: Vec::new(),
            source,
        }
    }

    fn always_exists(_: &Path) -> bool {
        true
    }

    fn never_exists(_: &Path) -> bool {
        false
    }

    fn exists_set(set: &[&str]) -> impl Fn(&Path) -> bool {
        let set: HashSet<PathBuf> = set.iter().map(PathBuf::from).collect();
        move |p| set.contains(p)
    }

    /// `/etc/shells` parser: strips comments and blanks, drops nonexistent
    /// absolute entries, keeps duplicates separate (dedup happens later).
    #[test]
    fn etc_shells_strips_comments_and_blanks() {        let content = "# This is a comment\n\n/bin/bash\n  /bin/zsh  \n# trailing comment\n";
        let shells = detect_unix_inner(Some(content), "", Some("/bin"), &always_exists);
        let labels: Vec<&str> = shells.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels, vec!["bash", "zsh"]);
        assert!(shells.iter().all(|s| s.source == ShellSource::EtcShells));
    }

    #[test]
    fn etc_shells_drops_nonexistent_absolute_entries() {
        let content = "/bin/bash\n/definitely/not/real/zsh\n/bin/fish\n";
        let exists = exists_set(&["/bin/bash", "/bin/fish"]);
        let shells = detect_unix_inner(Some(content), "", Some("/bin"), &exists);
        let labels: Vec<&str> = shells.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels, vec!["bash", "fish"]);
    }

    #[test]
    fn etc_shells_dedupes_repeated_programs() {
        // /bin/sh is frequently a symlink to /bin/bash on Linux; both
        // /etc/shells entries should survive (different program strings)
        // unless canonicalized to the same path. The never_exists probe
        // means neither survives — but the parser itself doesn't dedup.
        let content = "/bin/bash\n/bin/bash\n";
        // Use always_exists so both entries would pass the file check; the
        // canonical-dedup pass collapses identical absolute paths.
        let shells = detect_unix_inner(Some(content), "", Some("/bin"), &always_exists);
        let bash_count = shells
            .iter()
            .filter(|s| s.program.as_os_str() == "/bin/bash")
            .count();
        assert_eq!(
            bash_count, 1,
            "duplicate /bin/bash entries should collapse to one after dedup"
        );
    }

    #[test]
    fn etc_shells_resolves_relative_entries_via_path() {
        let content = "bash\nzsh\n";
        let exists = exists_set(&["/usr/bin/bash", "/usr/local/bin/zsh"]);
        let shells = detect_unix_inner(Some(content), "", Some("/usr/bin:/usr/local/bin"), &exists);
        assert_eq!(shells.len(), 2);
        let bash = shells
            .iter()
            .find(|s| s.label == "bash")
            .expect("bash should be resolved via PATH");
        assert_eq!(bash.program, PathBuf::from("/usr/bin/bash"));
        assert_eq!(bash.source, ShellSource::Path);
    }

    #[test]
    fn etc_shells_drops_unresolvable_relative_entries() {
        let content = "nosuchshell\n";
        let shells = detect_unix_inner(Some(content), "", Some("/usr/bin"), &never_exists);
        assert!(
            shells.is_empty(),
            "unresolvable relative entries should produce no shells"
        );
    }

    #[test]
    fn login_shell_always_included_even_if_absent_from_etc_shells() {
        let content = "/bin/bash\n";
        let exists = exists_set(&["/bin/bash", "/bin/zsh"]);
        let shells = detect_unix_inner(Some(content), "/bin/zsh", Some("/bin"), &exists);
        let has_zsh = shells
            .iter()
            .any(|s| s.program.as_os_str() == "/bin/zsh" && s.source == ShellSource::LoginShell);
        assert!(
            has_zsh,
            "login shell should be included even when absent from /etc/shells"
        );
    }

    #[test]
    fn missing_etc_shells_falls_back_to_known_locations() {
        let exists = exists_set(&["/bin/bash", "/bin/zsh", "/bin/sh"]);
        let shells = detect_unix_inner(None, "", Some("/usr/bin"), &exists);
        // Should include whichever of /bin/bash, /bin/zsh, /bin/sh exist.
        let labels: Vec<&str> = shells.iter().map(|s| s.label.as_str()).collect();
        assert!(labels.contains(&"bash"));
        assert!(labels.contains(&"zsh"));
        assert!(labels.contains(&"sh"));
        assert!(shells
            .iter()
            .all(|s| s.source == ShellSource::KnownLocation));
    }

    #[test]
    fn empty_etc_shells_falls_back_to_known_locations() {
        let content = "# only comments\n\n";
        let exists = exists_set(&["/bin/bash"]);
        let shells = detect_unix_inner(Some(content), "", Some("/usr/bin"), &exists);
        let labels: Vec<&str> = shells.iter().map(|s| s.label.as_str()).collect();
        assert!(
            labels.contains(&"bash"),
            "empty /etc/shells should still produce known-location fallback"
        );
    }

    #[test]
    fn duplicate_labels_get_vscode_style_suffixes() {
        let shells = vec![
            detected("bash", "/bin/bash", ShellSource::EtcShells),
            detected("bash", "/usr/local/bin/bash", ShellSource::Path),
            detected("bash", "/opt/bash", ShellSource::KnownLocation),
            detected("zsh", "/bin/zsh", ShellSource::EtcShells),
        ];
        let merged = apply_duplicate_label_suffixes(shells);
        let labels: Vec<&str> = merged.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels, vec!["bash", "bash (2)", "bash (3)", "zsh"]);
    }

    #[test]
    fn dedup_by_program_uses_raw_path_equality() {
        // Distinct raw program strings survive even if they would
        // canonicalize to the same inode on a real filesystem. This
        // keeps the inner fn fs-free (canonicalize touches the real fs).
        let shells = vec![
            detected("bash", "/bin/bash", ShellSource::EtcShells),
            detected("bash (2)", "/usr/bin/bash", ShellSource::Path),
        ];
        let merged = dedup_by_program(shells);
        assert_eq!(
            merged.len(),
            2,
            "distinct raw paths should both survive dedup"
        );
    }

    #[test]
    fn dedup_by_program_drops_exact_duplicates() {
        let shells = vec![
            detected("bash", "/bin/bash", ShellSource::EtcShells),
            detected("bash", "/bin/bash", ShellSource::Path),
        ];
        let merged = dedup_by_program(shells);
        assert_eq!(merged.len(), 1, "exact raw-path duplicates collapse");
    }

    /// Windows: well-known entries (PowerShell + cmd) are emitted via the
    /// injectable `path_exists`, in a stable order.
    #[test]
    fn windows_probes_emit_login_shell_when_windir_unset() {
        // Pre-fix #3 contract still holds: when windir is None, no
        // PowerShell/cmd entries, but the login shell still appears.
        let exists = always_exists;
        let shells = detect_windows_inner(
            "C:\\Program Files\\PowerShell\\7\\pwsh.exe",
            None,
            None,
            &exists,
        );
        // Login shell always wins when WINDIR is unset.
        assert_eq!(shells.len(), 1);
        assert_eq!(shells[0].source, ShellSource::LoginShell);
    }

    #[test]
    fn windows_emits_powershell_and_cmd_when_windir_set() {
        // Fix #3 lock: with an injected windir, both Windows PowerShell and
        // cmd.exe should be emitted in stable order (login shell, then
        // PowerShell, then cmd), each with its well-known label.
        // Build expected paths via the same join semantics the production
        // code uses, so the test is portable across Unix/Windows hosts
        // (Path::join uses the host separator).
        let windir = Path::new("C:\\Windows");
        let pwsh = Path::new("C:\\Program Files\\PowerShell\\7\\pwsh.exe");
        let powershell = windir
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        let cmd = windir.join("System32").join("cmd.exe");
        let paths = [
            pwsh.to_str().expect("utf8"),
            powershell.to_str().expect("utf8"),
            cmd.to_str().expect("utf8"),
        ];
        let exists = exists_set(&paths);
        let shells = detect_windows_inner(
            pwsh.to_str().expect("utf8"),
            None,
            Some(windir),
            &exists,
        );
        assert_eq!(
            shells.len(),
            3,
            "login shell + PowerShell + cmd should all surface"
        );
        assert_eq!(shells[0].source, ShellSource::LoginShell);
        assert_eq!(shells[0].program.as_os_str(), pwsh.as_os_str());

        assert_eq!(shells[1].label, "PowerShell");
        assert_eq!(shells[1].source, ShellSource::KnownLocation);
        assert_eq!(shells[1].program.as_os_str(), powershell.as_os_str());

        assert_eq!(shells[2].label, "Command Prompt");
        assert_eq!(shells[2].source, ShellSource::KnownLocation);
        assert_eq!(shells[2].program.as_os_str(), cmd.as_os_str());
    }

    #[test]
    fn windows_skips_missing_windir_entries() {
        // windir is set but the PowerShell file doesn't exist on the
        // simulated fs — only cmd should surface alongside the login shell.
        let windir = Path::new("C:\\Windows");
        let pwsh = Path::new("C:\\pwsh.exe");
        let cmd = windir.join("System32").join("cmd.exe");
        let paths = [
            pwsh.to_str().expect("utf8"),
            cmd.to_str().expect("utf8"),
        ];
        let exists = exists_set(&paths);
        let shells = detect_windows_inner(
            pwsh.to_str().expect("utf8"),
            None,
            Some(windir),
            &exists,
        );
        assert_eq!(shells.len(), 2);
        assert_eq!(shells[0].program.as_os_str(), pwsh.as_os_str());
        assert_eq!(shells[1].label, "Command Prompt");
        assert!(
            !shells
                .iter()
                .any(|s| s.label == "PowerShell"),
            "PowerShell should be skipped when its file is missing"
        );
    }

    #[test]
    fn windows_dedupes_when_login_shell_equals_known_location() {
        // If pwsh is both the system shell AND in a well-known location,
        // only one entry should survive. Here both point at the SAME path
        // under the injected windir, so the known-location entry dedups
        // against the login-shell entry.
        let windir = Path::new("C:\\Windows");
        let powershell = windir
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        let cmd = windir.join("System32").join("cmd.exe");
        let paths = [
            powershell.to_str().expect("utf8"),
            cmd.to_str().expect("utf8"),
        ];
        let exists = exists_set(&paths);
        let shells = detect_windows_inner(
            powershell.to_str().expect("utf8"),
            None,
            Some(windir),
            &exists,
        );
        // Login shell (powershell) + cmd; powershell known-location entry
        // dedups against the login shell.
        assert_eq!(shells.len(), 2);
        assert_eq!(shells[0].program.as_os_str(), powershell.as_os_str());
        assert_eq!(shells[1].label, "Command Prompt");
    }

    #[test]
    fn windows_windir_ignored_on_unix_platform_via_inner_fn() {
        // The public testable entrypoint passes windir through to the
        // Windows branch but ignores it on Unix. Sanity check: passing a
        // windir to the Unix branch doesn't change behavior.
        let shells = detect_available_shells_inner(
            Platform::Unix,
            Some("/bin/bash\n"),
            "",
            Some("/bin"),
            Some(Path::new("C:\\should\\be\\ignored")),
            &exists_set(&["/bin/bash"]),
        );
        assert_eq!(shells.len(), 1);
        assert_eq!(shells[0].label, "bash");
    }

    #[test]
    fn platform_host_matches_cfg() {
        let host = Platform::host();
        if cfg!(target_os = "windows") {
            assert_eq!(host, Platform::Windows);
        } else {
            assert_eq!(host, Platform::Unix);
        }
    }

    #[test]
    fn inner_fn_unix_dispatch() {
        // Make sure the public testable entrypoint routes Unix correctly.
        let shells = detect_available_shells_inner(
            Platform::Unix,
            Some("/bin/bash\n"),
            "/bin/zsh",
            Some("/bin"),
            None,
            &exists_set(&["/bin/bash", "/bin/zsh"]),
        );
        assert!(shells.iter().any(|s| s.label == "bash"));
        assert!(shells.iter().any(|s| s.label == "zsh"));
    }

    #[test]
    fn inner_fn_windows_dispatch() {
        let shells = detect_available_shells_inner(
            Platform::Windows,
            None,
            "C:\\pwsh.exe",
            Some("C:\\Windows\\System32"),
            None,
            &exists_set(&["C:\\pwsh.exe"]),
        );
        assert_eq!(shells.len(), 1);
        assert_eq!(shells[0].source, ShellSource::LoginShell);
    }

    /// The public `detect_available_shells()` cache returns a non-empty
    /// list on Unix dev hosts (where /bin/sh almost always exists).
    #[test]
    fn public_detect_returns_nonempty_on_unix_host() {
        if !cfg!(unix) {
            return;
        }
        let shells = detect_available_shells();
        assert!(
            !shells.is_empty(),
            "detect_available_shells() should find at least the login shell on Unix"
        );
    }

    #[test]
    fn basename_label_uses_file_stem() {
        assert_eq!(basename_label(Path::new("/bin/bash")), "bash");
        assert_eq!(basename_label(Path::new("/usr/bin/zsh")), "zsh");
        // Windows-style paths use `\` as separator only on Windows; on Unix
        // hosts `file_stem` returns the last `/`-separated component, which
        // for a literal `"C:\\..."` string is the whole thing. Gate the
        // expectation so the test stays meaningful on both platforms.
        if cfg!(target_os = "windows") {
            assert_eq!(
                basename_label(Path::new("C:\\Program Files\\PowerShell\\7\\pwsh.exe")),
                "pwsh"
            );
        }
    }
}
