//! Built-in Node.js version management.
//!
//! The IDE installs, lists, selects and removes Node.js versions itself,
//! through the `fnm` library compiled into it. Nothing has to be installed
//! first: no fnm, no nvm. The versions live in a folder of the IDE's own
//! ([`store_dir`]), apart from any version manager the user may have.
//!
//! One version is *active*: the default one, when there is one. A process
//! runs under it by having that version's folder put first on its `PATH`
//! ([`path_env`]), so choosing a version here changes what the IDE launches
//! and nothing else on the machine. With no default, whatever `node` is
//! already on the `PATH` (the "system" Node.js) is used as it is.
//!
//! Like `node_backend`, this has no GPUI or app-state dependency. Every call
//! blocks on disk or network, so call them from a background thread.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use fnm::Fnm;

/// One Node.js version the built-in manager has installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVersion {
    /// The exact version, with its leading `v`: `v22.3.0`.
    pub version: String,
    /// Whether this is the active version.
    pub current: bool,
    /// The folder holding `node`, `npm` and `npx`.
    pub bin_dir: PathBuf,
}

/// What a project asks for, and what that comes to here.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectNode {
    /// The version its `.nvmrc`, `.node-version` or `package.json` asks for,
    /// as written (`v22.x.x`, `>=20`). `None` when it asks for nothing.
    pub requested: Option<String>,
    /// The installed version that satisfies it, when one does.
    pub installed: Option<String>,
}

/// Where the built-in manager keeps its Node.js versions: a `node_versions`
/// folder in the IDE's data directory.
///
/// Not the `node` folder beside it. That one belongs to `node_runtime`, the
/// IDE's own Node.js for language servers, which deletes the whole folder
/// whenever it replaces its copy — and would take these versions with it.
pub fn store_dir() -> PathBuf {
    paths::data_dir().join("node_versions")
}

fn manager(store: &Path) -> Fnm {
    Fnm::in_dir(store)
}

/// Every installed version, newest first.
pub fn list(store: &Path) -> Result<Vec<NodeVersion>, String> {
    let mut versions: Vec<NodeVersion> = manager(store)
        .installed()
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(|installed| NodeVersion {
            version: installed.version,
            current: installed.is_default,
            bin_dir: installed.bin_dir,
        })
        .collect();
    versions.reverse();
    Ok(versions)
}

/// Every version that can be downloaded, newest first. A long-term-support
/// release carries its codename: `v22.11.0 (LTS: Jod)`.
pub fn list_available(store: &Path) -> Result<Vec<String>, String> {
    let mut versions: Vec<String> = manager(store)
        .available()
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(|remote| match remote.lts {
            Some(codename) => format!("{} (LTS: {codename})", remote.version),
            None => remote.version,
        })
        .collect();
    versions.reverse();
    Ok(versions)
}

/// The bare version in a [`list_available`] entry: `v22.11.0 (LTS: Jod)`
/// gives `v22.11.0`.
pub fn version_of(entry: &str) -> &str {
    entry.split_whitespace().next().unwrap_or(entry)
}

/// Downloads and unpacks `version`. The first version installed becomes the
/// active one. `version` can be exact, partial (`22`), `lts` or `latest`.
pub fn install(store: &Path, version: &str) -> Result<String, String> {
    let installed = manager(store)
        .install(version_of(version))
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "Installed to {}",
        installed.path.to_string_lossy()
    ))
}

/// Removes `version`. Removing the active version leaves none active, which
/// falls back to the system Node.js.
pub fn uninstall(store: &Path, version: &str) -> Result<String, String> {
    let removed = manager(store)
        .uninstall(version_of(version))
        .map_err(|error| error.to_string())?;
    Ok(format!("Removed {removed}"))
}

/// Makes `version` the active one.
pub fn use_version(store: &Path, version: &str) -> Result<String, String> {
    let chosen = manager(store)
        .set_default(version_of(version))
        .map_err(|error| error.to_string())?;
    Ok(format!("Now using Node.js {}", chosen.version))
}

/// Makes no built-in version active, handing the choice back to whatever
/// `node` is on the `PATH`. The installed versions stay installed.
pub fn use_system(store: &Path) -> Result<String, String> {
    // The default is a link in the store's `aliases` folder; taking it away
    // is all "no default" means. It is a directory junction on Windows and a
    // symlink elsewhere, which are removed differently.
    let link = manager(store).base_dir().join("aliases").join("default");
    if link.symlink_metadata().is_err() {
        return Ok("Already using the system Node.js".to_string());
    }
    let removed = if cfg!(windows) {
        std::fs::remove_dir(&link)
    } else {
        std::fs::remove_file(&link)
    };
    removed.map_err(|error| format!("Couldn't switch to the system Node.js: {error}"))?;
    Ok("Now using the system Node.js".to_string())
}

/// The active version, or `None` when the system Node.js is in use.
pub fn active(store: &Path) -> Option<NodeVersion> {
    list(store).ok()?.into_iter().find(|version| version.current)
}

/// What `project` asks for and whether it is installed.
pub fn project_node(store: &Path, project: &Path) -> ProjectNode {
    let manager = manager(store);
    let Some((directory, requested)) = request_of(&manager, project) else {
        return ProjectNode::default();
    };
    let installed = manager
        .resolve_for(directory)
        .ok()
        .flatten()
        .map(|version| version.version);
    ProjectNode {
        requested: Some(requested),
        installed,
    }
}

/// Downloads and unpacks the version `project` asks for. An error when it
/// asks for nothing.
pub fn install_for_project(store: &Path, project: &Path) -> Result<String, String> {
    let manager = manager(store);
    let (directory, _) = request_of(&manager, project)
        .ok_or_else(|| "This project doesn't ask for a Node.js version".to_string())?;
    let installed = manager
        .install_for(directory)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "This project doesn't ask for a Node.js version".to_string())?;
    Ok(format!(
        "Installed {} to {}",
        installed.version,
        installed.path.to_string_lossy()
    ))
}

/// The folder whose Node.js a command run in `project` should use: the
/// version the project asks for when that is installed, otherwise the active
/// one. `None` means the system Node.js, with nothing to change.
pub fn bin_dir_for(store: &Path, project: Option<&Path>) -> Option<PathBuf> {
    let manager = manager(store);
    let for_project = project
        .and_then(|project| request_of(&manager, project))
        .and_then(|(directory, _)| manager.resolve_for(directory).ok().flatten());
    for_project
        .or_else(|| manager.resolve("default").ok())
        .map(|version| version.bin_dir)
}

/// The `PATH` to give a command run in `project`, with the right Node.js
/// first on it. `None` when the system Node.js applies and the `PATH` should
/// be left alone.
pub fn path_env(store: &Path, project: Option<&Path>) -> Option<OsString> {
    let bin_dir = bin_dir_for(store, project)?;
    let current = std::env::var_os("PATH").unwrap_or_default();
    let mut dirs = vec![bin_dir];
    dirs.extend(std::env::split_paths(&current));
    std::env::join_paths(dirs).ok()
}

/// The version `project` asks for and the folder that says so: its own, or
/// one above it, which is how every Node.js version manager reads version
/// files.
///
/// The walk upward is done here rather than with the manager's own
/// parent-directory search, because that search answers with the manager's
/// default version when it finds no file. A default isn't the project asking
/// for anything, and couldn't be told apart from a file that says the same.
fn request_of<'a>(manager: &Fnm, project: &'a Path) -> Option<(&'a Path, String)> {
    project.ancestors().find_map(|directory| {
        manager
            .requested_version(directory)
            .map(|requested| (directory, requested))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A store holding `versions`, each as the empty folder a distribution
    /// would have been unpacked into.
    fn store(versions: &[&str]) -> tempfile::TempDir {
        let store = tempfile::tempdir().unwrap();
        for version in versions {
            std::fs::create_dir_all(
                store
                    .path()
                    .join("node-versions")
                    .join(version)
                    .join("installation"),
            )
            .unwrap();
        }
        store
    }

    fn names(versions: &[NodeVersion]) -> Vec<&str> {
        versions.iter().map(|v| v.version.as_str()).collect()
    }

    #[test]
    fn lists_newest_first_and_marks_the_active_version() {
        let store = store(&["v18.20.4", "v22.3.0", "v20.11.1"]);
        let versions = list(store.path()).unwrap();
        assert_eq!(names(&versions), vec!["v22.3.0", "v20.11.1", "v18.20.4"]);
        assert!(versions.iter().all(|v| !v.current));
        assert_eq!(active(store.path()), None);

        use_version(store.path(), "v20.11.1").unwrap();
        let versions = list(store.path()).unwrap();
        assert!(versions[1].current);
        assert_eq!(active(store.path()).unwrap().version, "v20.11.1");
    }

    #[test]
    fn switching_to_the_system_node_leaves_no_version_active() {
        let store = store(&["v22.3.0"]);
        // Nothing active yet: a no-op, not an error.
        use_system(store.path()).unwrap();

        use_version(store.path(), "22").unwrap();
        assert!(active(store.path()).is_some());
        assert!(path_env(store.path(), None).is_some());

        use_system(store.path()).unwrap();
        assert_eq!(active(store.path()), None);
        assert_eq!(bin_dir_for(store.path(), None), None);
        assert_eq!(path_env(store.path(), None), None);
        // The version is still installed.
        assert_eq!(names(&list(store.path()).unwrap()), vec!["v22.3.0"]);
    }

    #[test]
    fn a_projects_own_version_wins_over_the_active_one() {
        let store = store(&["v20.11.1", "v22.3.0"]);
        use_version(store.path(), "22").unwrap();
        let active_dir = active(store.path()).unwrap().bin_dir;

        let project = tempfile::tempdir().unwrap();
        // Asks for nothing: the active version applies.
        assert_eq!(project_node(store.path(), project.path()), ProjectNode::default());
        assert_eq!(bin_dir_for(store.path(), Some(project.path())), Some(active_dir.clone()));

        // Asks for 20, which is installed: that one is used, in subfolders too.
        std::fs::write(project.path().join(".nvmrc"), "20\n").unwrap();
        let status = project_node(store.path(), project.path());
        assert_eq!(status.requested.as_deref(), Some("v20.x.x"));
        assert_eq!(status.installed.as_deref(), Some("v20.11.1"));
        let nested = project.path().join("packages").join("app");
        std::fs::create_dir_all(&nested).unwrap();
        let chosen = bin_dir_for(store.path(), Some(&nested)).unwrap();
        assert_ne!(chosen, active_dir);
        assert!(chosen.to_string_lossy().contains("v20.11.1"));
        let path = path_env(store.path(), Some(&nested)).unwrap();
        assert_eq!(std::env::split_paths(&path).next().unwrap(), chosen);

        // A `package.json` range is honoured too, when no version file is.
        std::fs::remove_file(project.path().join(".nvmrc")).unwrap();
        std::fs::write(
            project.path().join("package.json"),
            r#"{"engines": {"node": "<22"}}"#,
        )
        .unwrap();
        let status = project_node(store.path(), project.path());
        assert!(status.requested.is_some());
        assert_eq!(status.installed.as_deref(), Some("v20.11.1"));

        // Asks for 18, which isn't: reported, and the active one is used.
        std::fs::write(project.path().join(".nvmrc"), "18\n").unwrap();
        let status = project_node(store.path(), project.path());
        assert_eq!(status.requested.as_deref(), Some("v18.x.x"));
        assert_eq!(status.installed, None);
        assert_eq!(bin_dir_for(store.path(), Some(project.path())), Some(active_dir));
    }

    #[test]
    fn uninstalling_the_active_version_falls_back_to_the_system_node() {
        let store = store(&["v22.3.0"]);
        use_version(store.path(), "22").unwrap();
        uninstall(store.path(), "v22.3.0").unwrap();
        assert!(list(store.path()).unwrap().is_empty());
        assert_eq!(path_env(store.path(), None), None);
    }

    /// `node_runtime` wipes `<data dir>/node` when it replaces its own
    /// Node.js, so the store must never be that folder or inside it.
    #[test]
    fn the_store_is_not_the_ides_own_node_folder() {
        let own = paths::data_dir().join("node");
        assert!(!store_dir().starts_with(&own));
    }

    #[test]
    fn an_available_entry_gives_back_its_version() {
        assert_eq!(version_of("v22.11.0 (LTS: Jod)"), "v22.11.0");
        assert_eq!(version_of("v23.1.0"), "v23.1.0");
    }
}
