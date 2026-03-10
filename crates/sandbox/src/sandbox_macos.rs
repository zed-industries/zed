//! macOS Seatbelt sandbox implementation.
//!
//! Uses `sandbox_init()` from `<sandbox.h>` to apply a Seatbelt sandbox profile
//! to the current process. Must be called after fork(), before exec().

use std::ffi::{CStr, CString};
use std::fmt::Write;
use std::io::{Error, Result};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::SandboxConfig;

unsafe extern "C" {
    fn sandbox_init(profile: *const c_char, flags: u64, errorbuf: *mut *mut c_char) -> i32;
    fn sandbox_free_error(errorbuf: *mut c_char);
    fn sandbox_check(pid: libc::pid_t, operation: *const c_char, filter_type: i32, ...) -> i32;
}

/// Filter type constant from `<sandbox.h>` for path-based checks.
const SANDBOX_FILTER_PATH: i32 = 1;

/// Check if a process is allowed to read a specific path under its sandbox profile.
/// Returns `true` if the operation is ALLOWED (sandbox_check returns 0 for allowed).
fn sandbox_check_file_read(pid: libc::pid_t, path: &Path) -> bool {
    let operation = CString::new("file-read-data").expect("static string");
    let path_cstr = match CString::new(path.to_string_lossy().as_bytes()) {
        Ok(cstr) => cstr,
        Err(_) => return false,
    };
    let result = unsafe {
        sandbox_check(
            pid,
            operation.as_ptr(),
            SANDBOX_FILTER_PATH,
            path_cstr.as_ptr(),
        )
    };
    result == 0
}

/// Per-session fingerprint for macOS process tracking via `sandbox_check()`.
///
/// Each terminal session embeds a unique fingerprint in its Seatbelt profile.
/// The fingerprint consists of a UUID-based directory pair under /tmp where
/// one path is allowed and a sibling is denied. This two-point test uniquely
/// identifies processes belonging to this session.
pub struct SessionFingerprint {
    uuid: Uuid,
    base_dir: PathBuf,
    owns_directory: bool,
}

impl SessionFingerprint {
    /// Create a new fingerprint. Creates the marker directories on disk.
    pub fn new() -> Result<Self> {
        let uuid = Uuid::new_v4();
        // Use /private/tmp (the canonical path) because Seatbelt resolves
        // symlinks — /tmp is a symlink to /private/tmp on macOS, and the
        // SBPL rules must use the canonical path to match correctly.
        let base_dir = PathBuf::from(format!("/private/tmp/.zed-sandbox-{uuid}"));
        let allow_dir = base_dir.join("allow");
        let deny_dir = base_dir.join("deny");
        std::fs::create_dir_all(&allow_dir)?;
        std::fs::create_dir_all(&deny_dir)?;
        Ok(Self {
            uuid,
            base_dir,
            owns_directory: true,
        })
    }

    /// Reconstruct a fingerprint from a UUID string (used by the child process).
    /// Does NOT create directories — assumes parent already created them.
    pub fn from_uuid_str(uuid_str: &str) -> std::result::Result<Self, String> {
        let uuid = Uuid::parse_str(uuid_str).map_err(|e| format!("invalid UUID: {e}"))?;
        let base_dir = PathBuf::from(format!("/private/tmp/.zed-sandbox-{uuid}"));
        Ok(Self {
            uuid,
            base_dir,
            owns_directory: false,
        })
    }

    /// Return the UUID as a string.
    pub fn uuid_string(&self) -> String {
        self.uuid.to_string()
    }

    /// Path that sandboxed processes CAN read (for fingerprint probing).
    pub fn allow_path(&self) -> PathBuf {
        self.base_dir.join("allow")
    }

    /// Path that sandboxed processes CANNOT read (for fingerprint probing).
    pub fn deny_path(&self) -> PathBuf {
        self.base_dir.join("deny")
    }

    /// Check if a given PID matches this session's fingerprint using `sandbox_check()`.
    ///
    /// Returns `true` if the process allows the allow-path AND denies the deny-path.
    pub fn matches_pid(&self, pid: libc::pid_t) -> bool {
        let allows = sandbox_check_file_read(pid, &self.allow_path());
        let denies = !sandbox_check_file_read(pid, &self.deny_path());
        allows && denies
    }

    /// Delete the fingerprint directory.
    pub fn cleanup(&self) {
        if let Err(err) = std::fs::remove_dir_all(&self.base_dir) {
            log::warn!(
                "Failed to clean up fingerprint directory {:?}: {err}",
                self.base_dir
            );
        }
    }
}

impl SessionFingerprint {
    /// Kill all processes belonging to this session using the convergent scan-and-kill loop.
    ///
    /// 1. killpg(pgid, SIGKILL) — best-effort kill of the process group
    /// 2. Loop: enumerate all PIDs by UID → skip zombies → filter by fingerprint → SIGKILL matches
    /// 3. Repeat until no matches found
    /// 4. Clean up the fingerprint directory
    ///
    /// This runs on a blocking thread — it's a tight loop that should complete quickly.
    pub fn kill_all_processes(&self, process_group_id: Option<libc::pid_t>) {
        // Step 1: Best-effort process group kill
        if let Some(pgid) = process_group_id {
            unsafe {
                libc::killpg(pgid, libc::SIGKILL);
            }
        }

        // Step 2: Convergent scan-and-kill loop
        loop {
            let processes = enumerate_user_processes();
            let mut found_any = false;

            for proc_info in &processes {
                if proc_info.is_zombie {
                    continue;
                }
                if self.matches_pid(proc_info.pid) {
                    found_any = true;
                    unsafe {
                        libc::kill(proc_info.pid, libc::SIGKILL);
                    }
                }
            }

            if !found_any {
                break;
            }

            // Brief sleep to let killed processes actually die before re-scanning
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // Step 3: Clean up
        self.cleanup();
    }
}

impl Drop for SessionFingerprint {
    fn drop(&mut self) {
        if self.owns_directory {
            self.cleanup();
        }
    }
}

/// Process info needed for cleanup.
struct ProcInfo {
    pid: libc::pid_t,
    is_zombie: bool,
}

/// Enumerate all processes owned by the current UID using macOS libproc APIs.
///
/// Uses `proc_listallpids` to get all PIDs, then `proc_pidinfo` with
/// `PROC_PIDTBSDINFO` to get `proc_bsdinfo` for UID filtering and zombie detection.
fn enumerate_user_processes() -> Vec<ProcInfo> {
    let uid = unsafe { libc::getuid() };

    // First call: get the count of all processes
    let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
    if count <= 0 {
        return Vec::new();
    }

    // Allocate buffer (add 20% to handle new processes appearing between calls)
    let buffer_count = (count as usize) + (count as usize) / 5;
    let mut pids: Vec<libc::pid_t> = vec![0; buffer_count];
    let buffer_size = (buffer_count * std::mem::size_of::<libc::pid_t>()) as libc::c_int;

    let actual_count =
        unsafe { libc::proc_listallpids(pids.as_mut_ptr() as *mut libc::c_void, buffer_size) };
    if actual_count <= 0 {
        return Vec::new();
    }
    pids.truncate(actual_count as usize);

    // For each PID, get BSD info to check UID and zombie status
    let mut result = Vec::new();
    for &pid in &pids {
        if pid <= 0 {
            continue;
        }
        let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
        let ret = unsafe {
            libc::proc_pidinfo(
                pid,
                libc::PROC_PIDTBSDINFO,
                0,
                &mut info as *mut _ as *mut libc::c_void,
                std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int,
            )
        };
        if ret <= 0 {
            continue;
        }
        if info.pbi_uid != uid {
            continue;
        }
        result.push(ProcInfo {
            pid,
            is_zombie: info.pbi_status == libc::SZOMB,
        });
    }

    result
}

/// Apply a compiled SBPL profile string to the current process via `sandbox_init()`.
fn apply_profile(profile: &str) -> Result<()> {
    let profile_cstr =
        CString::new(profile).map_err(|_| Error::other("sandbox profile contains null byte"))?;
    let mut errorbuf: *mut c_char = std::ptr::null_mut();

    let ret = unsafe { sandbox_init(profile_cstr.as_ptr(), 0, &mut errorbuf) };

    if ret == 0 {
        return Ok(());
    }

    let msg = if !errorbuf.is_null() {
        let s = unsafe { CStr::from_ptr(errorbuf) }
            .to_string_lossy()
            .into_owned();
        unsafe { sandbox_free_error(errorbuf) };
        s
    } else {
        "unknown sandbox error".to_string()
    };
    Err(Error::other(format!("sandbox_init failed: {msg}")))
}

/// Apply a Seatbelt sandbox profile to the current process.
/// Must be called after fork(), before exec().
///
/// # Safety
/// This function calls C FFI functions and must only be called
/// in a pre_exec context (after fork, before exec).
pub fn apply_sandbox(config: &SandboxConfig) -> Result<()> {
    apply_profile(&generate_sbpl_profile(config, None))
}

/// Apply a Seatbelt sandbox profile with an embedded session fingerprint.
pub fn apply_sandbox_with_fingerprint(
    config: &SandboxConfig,
    fingerprint: &SessionFingerprint,
) -> Result<()> {
    apply_profile(&generate_sbpl_profile(config, Some(fingerprint)))
}

/// Apply a minimal fingerprint-only Seatbelt profile (allows everything except
/// the deny-side path, enabling process identification via `sandbox_check()`).
pub fn apply_fingerprint_only(fingerprint: &SessionFingerprint) -> Result<()> {
    apply_profile(&generate_fingerprint_only_profile(fingerprint))
}

/// Generate a minimal Seatbelt profile that only contains the session fingerprint.
/// This allows everything but gives us the ability to identify the process via `sandbox_check()`.
pub(crate) fn generate_fingerprint_only_profile(fingerprint: &SessionFingerprint) -> String {
    let mut p = String::from("(version 1)\n(allow default)\n");
    write!(
        p,
        "(deny file-read* (subpath \"{}\"))\n",
        sbpl_escape(&fingerprint.deny_path())
    )
    .unwrap();
    write!(
        p,
        "(allow file-read* (subpath \"{}\"))\n",
        sbpl_escape(&fingerprint.allow_path())
    )
    .unwrap();
    p
}

/// Generate an SBPL (Sandbox Profile Language) profile from the sandbox config.
pub(crate) fn generate_sbpl_profile(
    config: &SandboxConfig,
    fingerprint: Option<&SessionFingerprint>,
) -> String {
    let mut p = String::from("(version 1)\n(deny default)\n");

    // Process lifecycle
    p.push_str("(allow process-fork)\n");
    p.push_str("(allow signal (target children))\n");

    // Mach service allowlist.
    //
    // TROUBLESHOOTING: If users report broken terminal behavior (e.g. DNS failures,
    // keychain errors, or commands hanging), a missing Mach service here is a likely
    // cause. To diagnose:
    //   1. Open Console.app and filter for "sandbox" or "deny mach-lookup" to find
    //      the denied service name.
    //   2. Or test interactively:
    //      sandbox-exec -p '(version 1)(deny default)(allow mach-lookup ...)' /bin/sh
    //   3. Add the missing service to the appropriate group below.

    // Logging: unified logging (os_log) and legacy syslog.
    p.push_str("(allow mach-lookup (global-name \"com.apple.logd\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.logd.events\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.system.logger\"))\n");

    // User/group directory lookups (getpwuid, getgrnam, id, etc.).
    p.push_str("(allow mach-lookup (global-name \"com.apple.system.opendirectoryd.libinfo\"))\n");
    p.push_str(
        "(allow mach-lookup (global-name \"com.apple.system.opendirectoryd.membership\"))\n",
    );

    // Darwin notification center, used internally by many system frameworks.
    p.push_str("(allow mach-lookup (global-name \"com.apple.system.notification_center\"))\n");

    // CFPreferences: reading user and system preferences.
    p.push_str("(allow mach-lookup (global-name \"com.apple.cfprefsd.agent\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.cfprefsd.daemon\"))\n");

    // Temp directory management (_CS_DARWIN_USER_CACHE_DIR, etc.).
    p.push_str("(allow mach-lookup (global-name \"com.apple.bsd.dirhelper\"))\n");

    // DNS and network configuration.
    p.push_str("(allow mach-lookup (global-name \"com.apple.dnssd.service\"))\n");
    p.push_str(
        "(allow mach-lookup (global-name \"com.apple.SystemConfiguration.DNSConfiguration\"))\n",
    );
    p.push_str("(allow mach-lookup (global-name \"com.apple.SystemConfiguration.configd\"))\n");
    p.push_str(
        "(allow mach-lookup (global-name \"com.apple.SystemConfiguration.NetworkInformation\"))\n",
    );
    p.push_str("(allow mach-lookup (global-name \"com.apple.SystemConfiguration.SCNetworkReachability\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.networkd\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.nehelper\"))\n");

    // Security, keychain, and TLS certificate verification.
    p.push_str("(allow mach-lookup (global-name \"com.apple.SecurityServer\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.trustd.agent\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.ocspd\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.security.authtrampoline\"))\n");

    // Launch Services: needed for the `open` command, file-type associations,
    // and anything that uses NSWorkspace or LaunchServices.
    p.push_str("(allow mach-lookup (global-name \"com.apple.coreservices.launchservicesd\"))\n");
    p.push_str("(allow mach-lookup (global-name \"com.apple.CoreServices.coreservicesd\"))\n");
    p.push_str("(allow mach-lookup (global-name-regex #\"^com\\.apple\\.lsd\\.\" ))\n");

    // Kerberos: needed in enterprise environments for authentication.
    p.push_str("(allow mach-lookup (global-name \"com.apple.GSSCred\"))\n");
    p.push_str("(allow mach-lookup (global-name \"org.h5l.kcm\"))\n");

    // Distributed notifications: some command-line tools using Foundation may need this.
    p.push_str(
        "(allow mach-lookup (global-name-regex #\"^com\\.apple\\.distributed_notifications\"))\n",
    );

    p.push_str("(allow sysctl-read)\n");

    // Root directory entry must be readable for path resolution (getcwd, realpath, etc.)
    p.push_str("(allow file-read* (literal \"/\"))\n");
    // Default shell selector symlink on macOS
    p.push_str("(allow file-read* (subpath \"/private/var/select\"))\n");

    // System executable paths (read + execute)
    for path in &config.system_paths.executable {
        write_subpath_rule(&mut p, path, "file-read* process-exec");
    }

    // System read-only paths
    for path in &config.system_paths.read_only {
        write_subpath_rule(&mut p, path, "file-read*");
    }

    // System read+write paths (devices, temp dirs, IPC)
    for path in &config.system_paths.read_write {
        write_subpath_rule(&mut p, path, "file-read* file-write*");
    }

    // Project directory: full access
    write_subpath_rule(
        &mut p,
        &config.project_dir,
        "file-read* file-write* process-exec",
    );

    // User-configured additional paths
    for path in &config.additional_executable_paths {
        write_subpath_rule(&mut p, path, "file-read* process-exec");
    }
    for path in &config.additional_read_only_paths {
        write_subpath_rule(&mut p, path, "file-read*");
    }
    for path in &config.additional_read_write_paths {
        write_subpath_rule(&mut p, path, "file-read* file-write*");
    }

    // User shell config files: read-only access to $HOME dotfiles
    if let Ok(home) = std::env::var("HOME") {
        let home = Path::new(&home);
        for dotfile in &[
            ".zshrc",
            ".zshenv",
            ".zprofile",
            ".zlogin",
            ".zlogout",
            ".bashrc",
            ".bash_profile",
            ".bash_login",
            ".profile",
            ".inputrc",
            ".terminfo",
            ".gitconfig",
        ] {
            let path = home.join(dotfile);
            if path.exists() {
                write!(
                    p,
                    "(allow file-read* (literal \"{}\"))\n",
                    sbpl_escape(&path)
                )
                .unwrap();
            }
        }
        // XDG config directory
        let config_dir = home.join(".config");
        if config_dir.exists() {
            write_subpath_rule(&mut p, &config_dir, "file-read*");
        }
    }

    // Network
    if config.allow_network {
        p.push_str("(allow network-outbound)\n");
        p.push_str("(allow network-inbound)\n");
        p.push_str("(allow system-socket)\n");
    }

    // Session fingerprint for process tracking — must come LAST so the deny
    // rule for the deny-side path takes priority over broader allow rules
    // (e.g., system read_write paths that include /private/tmp).
    if let Some(fp) = fingerprint {
        write!(
            p,
            "(deny file-read* (subpath \"{}\"))\n",
            sbpl_escape(&fp.deny_path())
        )
        .unwrap();
        write!(
            p,
            "(allow file-read* (subpath \"{}\"))\n",
            sbpl_escape(&fp.allow_path())
        )
        .unwrap();
    }

    p
}

pub(crate) fn sbpl_escape(path: &Path) -> String {
    path.display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn write_subpath_rule(p: &mut String, path: &Path, permissions: &str) {
    write!(
        p,
        "(allow {permissions} (subpath \"{}\"))\n",
        sbpl_escape(path)
    )
    .unwrap();
}
