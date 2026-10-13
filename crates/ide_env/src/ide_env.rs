//! The IDE's own runtimes, for anything that starts a process.
//!
//! The IDE can carry runtimes of its own — the Node.js versions the Node
//! panel installs, for one — that are deliberately not on the machine's
//! `PATH`. A process the IDE starts runs under them by having their folders
//! put first on *its* `PATH`, which changes nothing outside that process.
//!
//! This crate is where the two sides meet. Whatever manages a runtime
//! [`register`]s a provider at startup; whatever starts a process (a
//! terminal, the Script Runner, a workflow action) asks for [`bin_dirs`] or
//! [`path_env`] and never needs to know which runtimes exist.
//!
//! It has no dependencies, so the headless crates can use it too. A provider
//! may read the disk (a version store, a project's version file), so call
//! [`bin_dirs`] and [`path_env`] from a background thread.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::hash::BuildHasher;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Answers with the folders a process started in `directory` should have
/// first on its `PATH` for one runtime: usually one folder, or none when the
/// machine's own copy of that runtime applies.
pub type Provider = fn(directory: Option<&Path>) -> Vec<PathBuf>;

static PROVIDERS: RwLock<Vec<Provider>> = RwLock::new(Vec::new());

/// Adds a runtime. Call once per runtime, at app startup.
pub fn register(provider: Provider) {
    if let Ok(mut providers) = PROVIDERS.write() {
        providers.push(provider);
    }
}

/// The folders to put first on the `PATH` of a process started in
/// `directory`, across every registered runtime. Empty when the machine's own
/// runtimes apply and the `PATH` should be left alone.
pub fn bin_dirs(directory: Option<&Path>) -> Vec<PathBuf> {
    let providers = match PROVIDERS.read() {
        Ok(providers) => providers.clone(),
        Err(_) => return Vec::new(),
    };
    providers
        .into_iter()
        .flat_map(|provider| provider(directory))
        .collect()
}

/// The `PATH` to give a process started in `directory`: this process's own,
/// with the IDE's runtimes first. `None` when there is nothing to change.
pub fn path_env(directory: Option<&Path>) -> Option<OsString> {
    let path = std::env::var_os("PATH");
    prepend(&bin_dirs(directory), path.as_deref())
}

/// `path` with `dirs` first on it, and without any later mention of them.
/// `None` when `dirs` is empty, or can't be written as a `PATH`.
pub fn prepend(dirs: &[PathBuf], path: Option<&OsStr>) -> Option<OsString> {
    if dirs.is_empty() {
        return None;
    }
    let rest = path
        .map(|path| std::env::split_paths(path).collect::<Vec<_>>())
        .unwrap_or_default();
    let entries = dirs
        .iter()
        .cloned()
        .chain(rest.into_iter().filter(|entry| !dirs.contains(entry)));
    std::env::join_paths(entries).ok()
}

/// Puts `dirs` first on the `PATH` held in `env`, an environment that is
/// still being put together (a terminal's). When `env` has no `PATH` yet,
/// this process's own is what `dirs` go in front of, since that is what the
/// new process would otherwise inherit.
pub fn apply_to<S: BuildHasher>(env: &mut HashMap<String, String, S>, dirs: &[PathBuf]) {
    if dirs.is_empty() {
        return;
    }
    let key = env
        .keys()
        .find(|name| is_path_variable(name))
        .cloned()
        .unwrap_or_else(|| "PATH".to_string());
    let current = env
        .get(&key)
        .map(OsString::from)
        .or_else(|| std::env::var_os("PATH"));
    let Some(path) = prepend(dirs, current.as_deref()) else {
        return;
    };
    // A terminal's environment is text; a `PATH` that isn't is left as it was.
    if let Ok(path) = path.into_string() {
        env.insert(key, path);
    }
}

/// Whether `name` is the `PATH` variable. Windows spells it `Path` as often
/// as `PATH`, and treats the two as one.
pub fn is_path_variable(name: &str) -> bool {
    if cfg!(windows) {
        name.eq_ignore_ascii_case("PATH")
    } else {
        name == "PATH"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn joined(entries: &[&str]) -> OsString {
        std::env::join_paths(entries.iter().map(PathBuf::from)).unwrap()
    }

    fn entries(path: &OsStr) -> Vec<PathBuf> {
        std::env::split_paths(path).collect()
    }

    #[test]
    fn nothing_to_add_leaves_the_path_alone() {
        assert_eq!(prepend(&[], Some(&joined(&["a", "b"]))), None);

        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "untouched".to_string());
        apply_to(&mut env, &[]);
        assert_eq!(env["PATH"], "untouched");
    }

    #[test]
    fn runtimes_go_first_and_are_not_repeated() {
        let dirs = [PathBuf::from("node22"), PathBuf::from("python312")];
        let path = prepend(&dirs, Some(&joined(&["system", "node22", "tools"]))).unwrap();
        assert_eq!(
            entries(&path),
            ["node22", "python312", "system", "tools"].map(PathBuf::from)
        );

        // With no `PATH` to go in front of, they are the whole of it.
        let path = prepend(&dirs, None).unwrap();
        assert_eq!(entries(&path), dirs);
    }

    #[test]
    fn an_environment_keeps_its_own_spelling_of_path() {
        let dirs = [PathBuf::from("node22")];

        let mut env = HashMap::new();
        env.insert("PATH".to_string(), joined(&["system"]).into_string().unwrap());
        apply_to(&mut env, &dirs);
        assert_eq!(env.len(), 1);
        assert_eq!(
            entries(OsStr::new(&env["PATH"])),
            ["node22", "system"].map(PathBuf::from)
        );

        // Windows hands back `Path`; a second, differently spelled variable
        // beside it would leave it to chance which one the shell reads.
        if cfg!(windows) {
            let mut env = HashMap::new();
            env.insert("Path".to_string(), joined(&["system"]).into_string().unwrap());
            apply_to(&mut env, &dirs);
            assert_eq!(env.len(), 1);
            assert_eq!(
                entries(OsStr::new(&env["Path"])),
                ["node22", "system"].map(PathBuf::from)
            );
        }
    }

    #[test]
    fn an_environment_without_a_path_gets_one() {
        let dirs = [PathBuf::from("node22")];
        let mut env: HashMap<String, String> = HashMap::new();
        apply_to(&mut env, &dirs);
        let path = entries(OsStr::new(&env["PATH"]));
        assert_eq!(path[0], PathBuf::from("node22"));
        // In front of what would have been inherited, not instead of it.
        let inherited = std::env::var_os("PATH").unwrap_or_default();
        let inherited: Vec<PathBuf> = std::env::split_paths(&inherited)
            .filter(|entry| entry != &dirs[0])
            .collect();
        assert_eq!(&path[1..], inherited.as_slice());
    }

    #[test]
    fn registered_runtimes_are_asked_about_the_directory() {
        fn only_in_projects(directory: Option<&Path>) -> Vec<PathBuf> {
            directory
                .filter(|directory| directory.ends_with("ide_env_test_project"))
                .map(|directory| vec![directory.join("runtime")])
                .unwrap_or_default()
        }
        register(only_in_projects);

        let project = Path::new("some").join("ide_env_test_project");
        assert!(bin_dirs(Some(&project)).contains(&project.join("runtime")));
        assert!(!bin_dirs(None).contains(&project.join("runtime")));
        let path = path_env(Some(&project)).unwrap();
        assert_eq!(entries(&path)[0], project.join("runtime"));
    }
}
