//! macOS Seatbelt sandbox implementation.
//!
//! Uses `sandbox_init()` from `<sandbox.h>` to apply a Seatbelt sandbox profile
//! to the current process. Must be called after fork(), before exec().

use std::ffi::{CStr, CString};
use std::fmt::Write;
use std::io::{Error, Result};
use std::os::raw::c_char;
use std::path::Path;

use crate::terminal_settings::SandboxConfig;

unsafe extern "C" {
    fn sandbox_init(profile: *const c_char, flags: u64, errorbuf: *mut *mut c_char) -> i32;
    fn sandbox_free_error(errorbuf: *mut c_char);
}

/// Apply a Seatbelt sandbox profile to the current process.
/// Must be called after fork(), before exec().
///
/// # Safety
/// This function calls C FFI functions and must only be called
/// in a pre_exec context (after fork, before exec).
pub fn apply_sandbox(config: &SandboxConfig) -> Result<()> {
    let profile = generate_sbpl_profile(config);
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

/// Generate an SBPL (Sandbox Profile Language) profile from the sandbox config.
fn generate_sbpl_profile(config: &SandboxConfig) -> String {
    let mut p = String::from("(version 1)\n(deny default)\n");

    // Process lifecycle
    p.push_str("(allow process-fork)\n");
    p.push_str("(allow signal)\n");

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

    // No iokit-open rules: a terminal shell does not need to open IOKit user
    // clients (kernel driver interfaces). IOKit access is needed for GPU/
    // graphics (IOAccelerator, AGPMClient), audio (IOAudioEngine), USB,
    // Bluetooth, and similar hardware — none of which a shell requires. Random
    // numbers come from /dev/urandom or getentropy(), and timing uses syscalls,
    // so no IOKit involvement is needed for basic process operation. Chromium's
    // network process and Firefox's content process both operate without any
    // iokit-open rules.

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
                let _ = write!(
                    p,
                    "(allow file-read* (literal \"{}\"))\n",
                    sbpl_escape(&path)
                );
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

    p
}

fn sbpl_escape(path: &Path) -> String {
    path.display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn write_subpath_rule(p: &mut String, path: &Path, permissions: &str) {
    let _ = write!(
        p,
        "(allow {permissions} (subpath \"{}\"))\n",
        sbpl_escape(path)
    );
}
