//! Which package manager a project uses, and the commands to run with it.
//!
//! One decision, shared by the Node panel's Install button and the Package
//! Manager tab's default: running `npm install` in a pnpm or Yarn project
//! makes a second lock file and a different `node_modules`, so the project's
//! own manager is what runs.

use std::ffi::OsStr;
use std::path::Path;

/// A Node.js package manager.
#[derive(serde::Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn cli_name(self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }

    /// The program and arguments that install a project's dependencies.
    pub fn install_args(&self) -> (&'static str, Vec<&'static str>) {
        (self.cli_name(), vec!["install"])
    }

    /// The program and arguments that run the project's `script`. `run`
    /// works the same in all four, Yarn classic and Berry alike.
    pub fn run_args(&self, script: &str) -> (&'static str, Vec<String>) {
        (self.cli_name(), vec!["run".to_string(), script.to_string()])
    }

    fn from_name(name: &str) -> Option<Self> {
        match name {
            "npm" => Some(PackageManager::Npm),
            "yarn" => Some(PackageManager::Yarn),
            "pnpm" => Some(PackageManager::Pnpm),
            "bun" => Some(PackageManager::Bun),
            _ => None,
        }
    }
}

/// What a detection was decided by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// The `packageManager` field of a `package.json` (`"pnpm@9.1.0"`).
    PackageManagerField,
    /// A lock file, by its name.
    Lockfile(String),
    /// Nothing said: npm.
    Default,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Detected {
    pub manager: PackageManager,
    pub source: Source,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Detection {
    Found(Detected),
    /// Lock files of more than one manager in the same folder, and no
    /// `packageManager` field to settle it. In the order of [`LOCKFILES`].
    Ambiguous(Vec<PackageManager>),
}

/// Every lock file, and whose it is.
const LOCKFILES: &[(&str, PackageManager)] = &[
    ("pnpm-lock.yaml", PackageManager::Pnpm),
    ("yarn.lock", PackageManager::Yarn),
    ("package-lock.json", PackageManager::Npm),
    ("bun.lockb", PackageManager::Bun),
    ("bun.lock", PackageManager::Bun),
];

/// The package manager of the project in `project_dir`.
///
/// `project_dir` is searched, then each folder above it up to and including
/// `stop_at` (the workspace root), and the first folder with any signal
/// decides: pnpm and Yarn workspaces keep their lock file at the repository
/// root, with the projects in subfolders. In that folder a valid
/// `packageManager` field in `package.json` wins; otherwise a single kind
/// of lock file does, and lock files of more than one kind are
/// [`Detection::Ambiguous`]. With no signal anywhere, npm.
///
/// With `stop_at` `None`, or outside `project_dir`'s ancestors, the search
/// goes up to the root of the drive. Reads the disk: call it from a
/// background thread.
pub fn detect(project_dir: &Path, stop_at: Option<&Path>) -> Detection {
    for directory in project_dir.ancestors() {
        if let Some(detection) = detect_in(directory) {
            return detection;
        }
        if stop_at.is_some_and(|stop_at| directory == stop_at) {
            break;
        }
    }
    Detection::Found(Detected {
        manager: PackageManager::Npm,
        source: Source::Default,
    })
}

/// What `directory` alone says, if anything.
fn detect_in(directory: &Path) -> Option<Detection> {
    if let Some(manager) = package_manager_field(directory) {
        return Some(Detection::Found(Detected {
            manager,
            source: Source::PackageManagerField,
        }));
    }
    let mut kinds: Vec<(PackageManager, &str)> = Vec::new();
    for (file, manager) in LOCKFILES {
        if directory.join(file).is_file() && !kinds.iter().any(|(kind, _)| kind == manager) {
            kinds.push((*manager, file));
        }
    }
    match kinds.as_slice() {
        [] => None,
        [(manager, file)] => Some(Detection::Found(Detected {
            manager: *manager,
            source: Source::Lockfile((*file).to_string()),
        })),
        _ => Some(Detection::Ambiguous(
            kinds.iter().map(|(manager, _)| *manager).collect(),
        )),
    }
}

/// The manager `directory/package.json`'s `packageManager` field names
/// (`"pnpm@9.1.0"`, `"yarn@4.2.2+sha512…"`). `None` when there is no such
/// field, or it names another tool.
fn package_manager_field(directory: &Path) -> Option<PackageManager> {
    let raw = std::fs::read_to_string(directory.join("package.json")).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let field = json.get("packageManager")?.as_str()?;
    PackageManager::from_name(field.split('@').next()?.trim())
}

/// Whether `manager`'s command is on `path_env`, a `PATH` value: pass the one
/// the command will run with (`ide_env::path_env`). Nothing is started: the
/// folders are only looked in.
pub fn is_available(manager: PackageManager, path_env: &OsStr) -> bool {
    let name = manager.cli_name();
    std::env::split_paths(path_env).any(|directory| {
        if cfg!(windows) {
            [".cmd", ".exe", ".bat", ""]
                .iter()
                .any(|extension| directory.join(format!("{name}{extension}")).is_file())
        } else {
            is_executable(&directory.join(name))
        }
    })
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, contents).unwrap();
    }

    fn found(manager: PackageManager, source: Source) -> Detection {
        Detection::Found(Detected { manager, source })
    }

    #[test]
    fn the_package_manager_field_wins_over_a_lock_file() {
        let dir = tempfile::tempdir().unwrap();
        write(
            &dir.path().join("package.json"),
            r#"{ "packageManager": "pnpm@9.1.0" }"#,
        );
        write(&dir.path().join("yarn.lock"), "");
        assert_eq!(
            detect(dir.path(), Some(dir.path())),
            found(PackageManager::Pnpm, Source::PackageManagerField)
        );
    }

    #[test]
    fn an_unknown_package_manager_field_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        write(
            &dir.path().join("package.json"),
            r#"{ "packageManager": "deno@2.0.0" }"#,
        );
        write(&dir.path().join("yarn.lock"), "");
        assert_eq!(
            detect(dir.path(), Some(dir.path())),
            found(PackageManager::Yarn, Source::Lockfile("yarn.lock".into()))
        );
    }

    #[test]
    fn each_lock_file_alone_names_its_manager() {
        for (file, manager) in LOCKFILES {
            let dir = tempfile::tempdir().unwrap();
            write(&dir.path().join(file), "");
            assert_eq!(
                detect(dir.path(), Some(dir.path())),
                found(*manager, Source::Lockfile((*file).to_string())),
                "{file}"
            );
        }
    }

    #[test]
    fn lock_files_of_two_kinds_are_ambiguous() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("package-lock.json"), "{}");
        write(&dir.path().join("pnpm-lock.yaml"), "");
        assert_eq!(
            detect(dir.path(), Some(dir.path())),
            Detection::Ambiguous(vec![PackageManager::Pnpm, PackageManager::Npm])
        );
        // Bun's two lock files are one kind.
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("bun.lockb"), "");
        write(&dir.path().join("bun.lock"), "");
        assert_eq!(
            detect(dir.path(), Some(dir.path())),
            found(PackageManager::Bun, Source::Lockfile("bun.lockb".into()))
        );
    }

    #[test]
    fn a_monorepo_project_takes_the_root_lock_file() {
        let root = tempfile::tempdir().unwrap();
        write(&root.path().join("pnpm-lock.yaml"), "");
        let project = root.path().join("packages").join("web");
        write(&project.join("package.json"), r#"{ "name": "web" }"#);
        assert_eq!(
            detect(&project, Some(root.path())),
            found(
                PackageManager::Pnpm,
                Source::Lockfile("pnpm-lock.yaml".into())
            )
        );
    }

    #[test]
    fn the_search_stops_at_the_workspace_root() {
        let outside = tempfile::tempdir().unwrap();
        write(&outside.path().join("yarn.lock"), "");
        let root = outside.path().join("workspace");
        let project = root.join("app");
        write(&project.join("package.json"), "{}");
        assert_eq!(
            detect(&project, Some(&root)),
            found(PackageManager::Npm, Source::Default)
        );
        // Without the stop, the lock file above is found.
        assert_eq!(
            detect(&project, None),
            found(PackageManager::Yarn, Source::Lockfile("yarn.lock".into()))
        );
    }

    #[test]
    fn the_commands_for_each_manager() {
        for (manager, name) in [
            (PackageManager::Npm, "npm"),
            (PackageManager::Yarn, "yarn"),
            (PackageManager::Pnpm, "pnpm"),
            (PackageManager::Bun, "bun"),
        ] {
            assert_eq!(manager.install_args(), (name, vec!["install"]));
            assert_eq!(
                manager.run_args("dev"),
                (name, vec!["run".to_string(), "dev".to_string()])
            );
        }
    }

    #[test]
    fn availability_is_looked_up_on_the_path() {
        let bin = tempfile::tempdir().unwrap();
        let path = std::env::join_paths([bin.path()]).unwrap();
        assert!(!is_available(PackageManager::Pnpm, &path));

        #[cfg(windows)]
        write(&bin.path().join("pnpm.cmd"), "@echo off");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let pnpm = bin.path().join("pnpm");
            write(&pnpm, "#!/bin/sh");
            // Not executable yet: not available.
            assert!(!is_available(PackageManager::Pnpm, &path));
            std::fs::set_permissions(&pnpm, std::fs::Permissions::from_mode(0o755)).unwrap();
        }

        assert!(is_available(PackageManager::Pnpm, &path));
        assert!(!is_available(PackageManager::Yarn, &path));
    }
}
