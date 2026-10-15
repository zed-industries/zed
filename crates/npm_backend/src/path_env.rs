//! The PATH for spawned npm/yarn/pnpm/bun processes.
//!
//! A package manager run by the IDE should find the same tools a project's
//! own `npm run` would, under the same Node.js as the IDE's terminals and
//! scripts: the project's `node_modules/.bin` folders first, then the IDE's
//! own runtimes (the built-in Node.js the user picked, through `ide_env`),
//! then the machine's PATH. [`path_with_node`] builds that.
//!
//! On Windows the machine's PATH is the registry-merged one: GUI-launched
//! apps inherit a stripped-down PATH, so [`merged_path`] adds the registry
//! `Environment` values (same logic as `node_backend`).

use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

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
    if val.is_empty() { return None; }
    Some(expand_env_vars(&val))
}

#[cfg(target_os = "windows")]
fn dedupe_path(path: &str) -> String {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for part in path.split(';') {
        let part = part.trim();
        if part.is_empty() { continue; }
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
                log::debug!("npm_backend: HKCU Path missing");
            }
        }
        Err(_) => log::debug!("npm_backend: cannot open HKCU Environment"),
    }

    match LOCAL_MACHINE
        .open(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
    {
        Ok(env_key) => {
            if let Some(val) = read_env_path(&env_key) {
                parts.push(val);
            } else {
                log::debug!("npm_backend: HKLM Path missing");
            }
        }
        Err(_) => log::debug!("npm_backend: cannot open HKLM Environment"),
    }

    let merged = dedupe_path(&parts.join(";"));
    if merged.is_empty() { None } else { Some(merged) }
}

#[cfg(not(target_os = "windows"))]
pub fn merged_path() -> Option<String> {
    None
}

/// The PATH to give a package manager run in `project_root`, in the order
/// `npm run` uses: the project's `node_modules/.bin` folders, nearest first
/// (so a project's own tool wins over a global one), then the IDE's own
/// runtimes, then the machine's PATH.
///
/// The IDE's runtimes are what make the panels' npm calls run under the
/// Node.js the user picked in the Node panel, as the terminal and scripts
/// do, and work on a machine with no Node.js installed at all. This used to
/// put nvm-windows' current version first and the project's tools last.
pub fn path_with_node(project_root: &str) -> OsString {
    let root = Path::new(project_root);
    let mut dirs: Vec<PathBuf> = root
        .ancestors()
        .map(|dir| dir.join("node_modules").join(".bin"))
        .filter(|dir| dir.is_dir())
        .collect();
    dirs.extend(ide_env::bin_dirs(Some(root)));

    let base = merged_path()
        .map(OsString::from)
        .or_else(|| std::env::var_os("PATH"))
        .unwrap_or_default();
    let path = ide_env::prepend(&dirs, Some(&base)).unwrap_or(base);
    log::debug!("npm_backend: PATH = {}", path.to_string_lossy());
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_tools_come_first_nearest_first() {
        let root = tempfile::tempdir().unwrap();
        let app = root.path().join("packages").join("app");
        for dir in [root.path(), app.as_path()] {
            std::fs::create_dir_all(dir.join("node_modules").join(".bin")).unwrap();
        }

        let path = path_with_node(&app.to_string_lossy());
        let entries: Vec<PathBuf> = std::env::split_paths(&path).collect();
        assert_eq!(entries[0], app.join("node_modules").join(".bin"));
        assert_eq!(entries[1], root.path().join("node_modules").join(".bin"));
        // The machine's PATH follows them, rather than being replaced.
        assert!(entries.len() > 2);
    }

    #[test]
    fn a_project_without_tools_gets_the_machines_path() {
        let root = tempfile::tempdir().unwrap();
        // Only meaningful where nothing above the temp folder is a Node
        // project of its own.
        if root
            .path()
            .ancestors()
            .any(|dir| dir.join("node_modules").join(".bin").is_dir())
        {
            return;
        }
        let path = path_with_node(&root.path().to_string_lossy());
        let base = merged_path()
            .map(OsString::from)
            .or_else(|| std::env::var_os("PATH"))
            .unwrap_or_default();
        let entries: Vec<PathBuf> = std::env::split_paths(&path).collect();
        let expected: Vec<PathBuf> = std::env::split_paths(&base).collect();
        assert_eq!(entries, expected);
    }
}