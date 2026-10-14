//! Windows PATH enrichment for spawned processes.
//!
//! On Windows a packaged/GUI-launched app inherits a stripped-down PATH that
//! often misses user-installed tools (pyenv, nvm, scoop, cargo, …), since
//! those only ever get added to the registry's `Environment` PATH value
//! (read by newly-spawned shells) rather than the current process's own
//! environment. [`merged_path`] merges the current process PATH with the
//! HKCU/HKLM registry values so a child process like `python`/`pip` can
//! still be found; [`enrich_path`] injects that into a [`std::process::Command`].

#[cfg(target_os = "windows")]
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

/// Expand `%VAR%` references in a registry `Path` value (`REG_EXPAND_SZ`
/// values come back as literal `%VAR%` text, not pre-expanded) so entries
/// like `%NVM_SYMLINK%\nodejs` resolve against the environment before being
/// merged in. Reads the process environment — the same source
/// `ExpandEnvironmentStringsW` uses.
#[cfg(target_os = "windows")]
fn expand_env_vars(input: &str) -> String {
    let mut result = input.to_string();
    let mut start = 0;
    while let Some(open) = result[start..].find('%') {
        let open = start + open;
        let rest = &result[open + 1..];
        let Some(close_rel) = rest.find('%') else {
            break;
        };
        let close = open + 1 + close_rel;
        let name = &result[open + 1..close];
        if let Ok(value) = std::env::var(name) {
            result.replace_range(open..close + 1, &value);
            start = open + value.len();
        } else {
            start = close + 1;
        }
    }
    result
}

#[cfg(target_os = "windows")]
fn read_env_path(key: &windows_registry::Key) -> Option<String> {
    let val = key.get_string("Path").ok()?;
    if val.is_empty() {
        return None;
    }
    Some(expand_env_vars(&val))
}

/// De-duplicate PATH entries (case-insensitively, keeping first occurrence
/// and priority order) and drop empty/whitespace segments. Windows `cmd`
/// fails to resolve executables once `PATH` grows past its practical size
/// limit, and the naive `current;HKCU;HKLM` concatenation easily triples in
/// size from heavy overlap — so this collapses it back to a normal length.
#[cfg(target_os = "windows")]
fn dedupe_path(path: &str) -> String {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for part in path.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let key = part.to_lowercase();
        if !seen.contains(&key) {
            seen.push(key);
            out.push(part.to_string());
        }
    }
    out.join(";")
}

#[cfg(target_os = "windows")]
pub fn merged_path() -> Option<String> {
    let current = std::env::var("PATH").unwrap_or_default();
    let mut parts = vec![current];

    match CURRENT_USER.open("Environment") {
        Ok(env_key) => {
            if let Some(val) = read_env_path(&env_key) {
                parts.push(val);
            } else {
                log::debug!("merged_path: HKCU Path missing");
            }
        }
        Err(_) => log::debug!("merged_path: cannot open HKCU Environment"),
    }

    match LOCAL_MACHINE
        .open(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
    {
        Ok(env_key) => {
            if let Some(val) = read_env_path(&env_key) {
                parts.push(val);
            } else {
                log::debug!("merged_path: HKLM Path missing");
            }
        }
        Err(_) => log::debug!("merged_path: cannot open HKLM Environment"),
    }

    let merged = dedupe_path(&parts.join(";"));
    log::debug!("merged_path: deduped PATH = {merged}");
    if merged.is_empty() { None } else { Some(merged) }
}

// Only Windows needs this: a packaged app there inherits a stripped-down
// PATH. Elsewhere the inherited one is complete, and nothing calls it.
#[cfg(target_os = "windows")]
pub fn enrich_path(cmd: &mut std::process::Command) {
    if let Some(merged) = merged_path() {
        cmd.env("PATH", merged);
    }
}
