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
    p.push_str("(allow process-exec)\n");
    p.push_str("(allow process-fork)\n");
    p.push_str("(allow signal)\n");

    // System services needed for basic operation
    p.push_str("(allow mach-lookup)\n");
    p.push_str("(allow sysctl-read)\n");
    p.push_str("(allow iokit-open)\n");

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
    write_subpath_rule(&mut p, &config.project_dir, "file-read* file-write*");

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
                let _ = write!(p, "(allow file-read* (literal \"{}\"))\n", path.display());
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

fn write_subpath_rule(p: &mut String, path: &Path, permissions: &str) {
    let _ = write!(
        p,
        "(allow {permissions} (subpath \"{}\"))\n",
        path.display()
    );
}
