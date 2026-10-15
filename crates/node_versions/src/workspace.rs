//! A Node.js choice per workspace.
//!
//! The store's `default` link is one choice for the whole IDE: switching it
//! in one window switches every window, open terminals included. Here each
//! workspace root can carry its own [`NodeChoice`], and gets a link of its
//! own in the store's `workspaces` folder that follows that choice. The
//! workspace's processes have that link on their `PATH`, so re-pointing it
//! switches the workspace's open terminals and nothing else.
//!
//! The choices are kept in a global registry, because what reads them
//! (`ide_env`'s providers) is a plain function with nowhere else to look.
//! Nothing is saved: whoever owns the choices sets them again at startup.
//!
//! Everything here that reads the store blocks on the disk, so call it from
//! a background thread.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, RwLock};

use super::{
    default_link, live_bin_dir, manager, point_link, project_node, remove_link, version_of,
};

/// Which Node.js a workspace runs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum NodeChoice {
    /// What the project's `.nvmrc`, `.node-version` or `package.json` asks
    /// for when that is installed, otherwise the IDE-wide default.
    #[default]
    Auto,
    /// The machine's own Node.js, whatever is on the `PATH`.
    System,
    /// This exact installed version, `v22.3.0` or `22.3.0`.
    Pinned { version: String },
}

/// Where a resolved version came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceSource {
    /// The workspace's own pinned version.
    Pinned,
    /// What the project's version file asks for.
    ProjectFile,
    /// The IDE-wide default (the store's `default` link), or the system
    /// Node.js when there is none.
    Default,
    /// The machine's own Node.js.
    System,
}

/// What a choice comes to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    /// The folders to put first on the `PATH`. Empty for the system Node.js.
    pub dirs: Vec<PathBuf>,
    pub source: ChoiceSource,
    /// A version that was asked for (pinned, or by the project) but isn't
    /// installed, so something else was used.
    pub missing: Option<String>,
}

/// What a choice points at.
enum Pick {
    /// An installed version: its own folder and the folder of its
    /// executables.
    Version {
        path: PathBuf,
        bin_dir: PathBuf,
    },
    /// The IDE-wide default.
    Default,
    System,
}

/// What `choice` comes to for a process started in `directory`. Reads the
/// store and the project's version files; changes nothing.
///
/// A pinned or project version comes back as that version's own folder,
/// never as the store's `default` link. A pinned version that isn't
/// installed is reported in `missing`, and the choice falls back to
/// [`NodeChoice::Auto`].
pub fn resolve(store: &Path, choice: &NodeChoice, directory: Option<&Path>) -> Resolved {
    let (pick, source, missing) = pick(store, choice, directory);
    let dirs = match pick {
        Pick::Version { bin_dir, .. } => vec![bin_dir],
        Pick::Default => vec![live_bin_dir(store)],
        Pick::System => Vec::new(),
    };
    Resolved {
        dirs,
        source,
        missing,
    }
}

fn pick(
    store: &Path,
    choice: &NodeChoice,
    directory: Option<&Path>,
) -> (Pick, ChoiceSource, Option<String>) {
    match choice {
        NodeChoice::System => (Pick::System, ChoiceSource::System, None),
        NodeChoice::Pinned { version } => match installed(store, version) {
            Some(pick) => (pick, ChoiceSource::Pinned, None),
            None => {
                let (pick, source, _) = pick_auto(store, directory);
                (pick, source, Some(version.clone()))
            }
        },
        NodeChoice::Auto => pick_auto(store, directory),
    }
}

fn pick_auto(store: &Path, directory: Option<&Path>) -> (Pick, ChoiceSource, Option<String>) {
    let mut missing = None;
    if let Some(directory) = directory {
        let project = project_node(store, directory);
        if let Some(pick) = project
            .installed
            .as_deref()
            .and_then(|version| installed(store, version))
        {
            return (pick, ChoiceSource::ProjectFile, None);
        }
        missing = project.requested;
    }
    (Pick::Default, ChoiceSource::Default, missing)
}

/// The installed version that is exactly `version`, with or without its
/// leading `v`.
fn installed(store: &Path, version: &str) -> Option<Pick> {
    let wanted = format!("v{}", version_of(version).trim_start_matches('v'));
    let found = manager(store).resolve(&wanted).ok()?;
    (found.version == wanted).then_some(Pick::Version {
        path: found.path,
        bin_dir: found.bin_dir,
    })
}

static CHOICES: LazyLock<RwLock<HashMap<PathBuf, NodeChoice>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Gives the workspace at `root` its choice, replacing any it had.
///
/// Register every open workspace, with [`NodeChoice::Auto`] when nothing
/// else was chosen: only a registered workspace gets a link of its own, and
/// only a process started with that link on its `PATH` follows a later
/// switch.
pub fn set_choice(root: &Path, choice: NodeChoice) {
    if let Ok(mut choices) = CHOICES.write() {
        choices.insert(normalize(root), choice);
    }
}

/// Forgets the workspace at `root`: it is back to [`NodeChoice::Auto`].
pub fn clear_choice(root: &Path) {
    if let Ok(mut choices) = CHOICES.write() {
        choices.remove(&normalize(root));
    }
}

/// The choice of the workspace `directory` is in: the one with the longest
/// root that `directory` is inside. [`NodeChoice::Auto`] when it is in none.
pub fn choice_for(directory: Option<&Path>) -> NodeChoice {
    workspace_for(directory)
        .map(|(_, choice)| choice)
        .unwrap_or_default()
}

/// The registered workspace `directory` is in, and its choice.
fn workspace_for(directory: Option<&Path>) -> Option<(PathBuf, NodeChoice)> {
    let directory = normalize(directory?);
    let choices = CHOICES.read().ok()?;
    choices
        .iter()
        .filter(|(root, _)| directory.starts_with(root))
        .max_by_key(|(root, _)| root.components().count())
        .map(|(root, choice)| (root.clone(), choice.clone()))
}

/// The folders to put first on the `PATH` of a process started in
/// `directory`.
///
/// Inside a registered workspace that is the workspace's own link (see
/// [`workspace_bin_dir`]), pointed at what its choice comes to first, so
/// the process follows the workspace's later switches. For the system
/// Node.js the link is removed but still returned: an entry that leads
/// nowhere is skipped, and the `node` further along the `PATH` is found.
///
/// Outside every registered workspace, or when the link can't be written,
/// it is what [`resolve`] gives for the choice.
pub fn bin_dirs_for(store: &Path, directory: Option<&Path>) -> Vec<PathBuf> {
    let Some((root, choice)) = workspace_for(directory) else {
        return resolve(store, &NodeChoice::Auto, directory).dirs;
    };
    match sync_link(store, &root, &choice) {
        Ok(()) => vec![workspace_bin_dir(store, &root)],
        Err(_) => resolve(store, &choice, directory).dirs,
    }
}

/// Points the link of the workspace at `root` at what its registered choice
/// comes to now. Call it after [`set_choice`], so the workspace's open
/// terminals follow at once rather than at the next process started.
pub fn sync_workspace(store: &Path, root: &Path) -> Result<(), String> {
    let root = normalize(root);
    let choice = CHOICES
        .read()
        .ok()
        .and_then(|choices| choices.get(&root).cloned())
        .unwrap_or_default();
    sync_link(store, &root, &choice)
        .map_err(|error| format!("Couldn't switch this workspace's Node.js: {error}"))
}

/// Serialises writes to the workspace links: two terminals opening at once
/// resolve the same workspace on two threads.
static LINK_WRITES: Mutex<()> = Mutex::new(());

/// Points `root`'s link at what `choice` comes to, when it doesn't already.
/// The version files read for [`NodeChoice::Auto`] are the root's: one link
/// serves the whole workspace.
fn sync_link(store: &Path, root: &Path, choice: &NodeChoice) -> std::io::Result<()> {
    let link = workspace_link(store, root);
    let target = match pick(store, choice, Some(root)).0 {
        Pick::Version { path, .. } => Some(path),
        // The IDE-wide default link itself, so the workspace follows it.
        // When it doesn't exist (the system Node.js) this leads nowhere,
        // which is what that means.
        Pick::Default => Some(default_link(store)),
        Pick::System => None,
    };

    let _guard = LINK_WRITES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(target) = target else {
        return remove_link(&link);
    };
    let current = std::fs::read_link(&link).ok();
    if current.is_some_and(|current| normalize(&current) == normalize(&target)) {
        return Ok(());
    }
    if let Some(folder) = link.parent() {
        std::fs::create_dir_all(folder)?;
    }
    point_link(&target, &link, store)
}

/// The folder holding the workspace links. Not `aliases`, where the manager
/// would read them as aliases of its own.
fn workspaces_dir(store: &Path) -> PathBuf {
    manager(store).base_dir().join("workspaces")
}

/// The link of the workspace at `root`, named after a hash of its folder.
pub fn workspace_link(store: &Path, root: &Path) -> PathBuf {
    workspaces_dir(store).join(format!("ws-{:016x}", fnv1a(&normalize(root))))
}

/// The folder to put on the `PATH` for the workspace at `root`: inside its
/// link, as [`live_bin_dir`] is inside the `default` one.
pub fn workspace_bin_dir(store: &Path, root: &Path) -> PathBuf {
    let link = workspace_link(store, root);
    // A Windows distribution has its executables at the top; the others
    // keep them in `bin`.
    if cfg!(windows) {
        link
    } else {
        link.join("bin")
    }
}

/// Removes the links of every workspace but those at `keep`, and returns how
/// many went. Pass every workspace open in the IDE, across all its windows:
/// a workspace left out loses its link, and its open terminals their
/// Node.js until it is synced again.
pub fn remove_stale_links(store: &Path, keep: &[&Path]) -> std::io::Result<usize> {
    let keep: Vec<PathBuf> = keep
        .iter()
        .map(|root| workspace_link(store, root))
        .collect();
    let entries = match std::fs::read_dir(workspaces_dir(store)) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error),
    };
    let _guard = LINK_WRITES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut removed = 0;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let is_link = entry.file_name().to_string_lossy().starts_with("ws-");
        if is_link && !keep.contains(&path) {
            remove_link(&path)?;
            removed += 1;
        }
    }
    Ok(removed)
}

/// `path` as roots are compared and named: no `\\?\` prefix, no trailing
/// separator, and on Windows, where paths ignore case, lowercase with
/// backslashes. Lexical only: the disk isn't read.
fn normalize(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    let text = if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{rest}")
    } else if let Some(rest) = text.strip_prefix(r"\\?\") {
        rest.to_string()
    } else {
        text.into_owned()
    };
    let text = if cfg!(windows) {
        text.replace('/', "\\").to_lowercase()
    } else {
        text
    };
    let trimmed = text.trim_end_matches(['/', '\\']);
    PathBuf::from(if trimmed.is_empty() {
        text.as_str()
    } else {
        trimmed
    })
}

/// A hash that stays the same from one build to the next, unlike the
/// standard library's, so a workspace keeps its link name.
fn fnv1a(path: &Path) -> u64 {
    path.to_string_lossy()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
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

    fn bin_dir_of(store: &Path, version: &str) -> PathBuf {
        crate::list(store)
            .unwrap()
            .into_iter()
            .find(|installed| installed.version == version)
            .unwrap()
            .bin_dir
    }

    /// A folder that exists nowhere, for the registry tests, which don't
    /// read the disk. Each test uses names of its own: the registry is
    /// shared by tests running at the same time.
    fn root(path: &str) -> PathBuf {
        if cfg!(windows) {
            PathBuf::from(format!(r"H:\{}", path.replace('/', "\\")))
        } else {
            PathBuf::from(format!("/h/{path}"))
        }
    }

    #[test]
    fn an_installed_pinned_version_is_its_own_folder() {
        let store = store(&["v20.11.1", "v22.3.0"]);
        let resolved = resolve(
            store.path(),
            &NodeChoice::Pinned {
                version: "22.3.0".into(),
            },
            None,
        );
        assert_eq!(resolved.source, ChoiceSource::Pinned);
        assert_eq!(resolved.dirs, vec![bin_dir_of(store.path(), "v22.3.0")]);
        assert_eq!(resolved.missing, None);
        assert!(!resolved.dirs.contains(&live_bin_dir(store.path())));
    }

    #[test]
    fn a_pinned_version_is_the_same_with_or_without_its_v() {
        let store = store(&["v22.3.0"]);
        let with = resolve(
            store.path(),
            &NodeChoice::Pinned {
                version: "v22.3.0".into(),
            },
            None,
        );
        let without = resolve(
            store.path(),
            &NodeChoice::Pinned {
                version: "22.3.0".into(),
            },
            None,
        );
        assert_eq!(with, without);
        assert_eq!(with.source, ChoiceSource::Pinned);
    }

    #[test]
    fn a_pinned_version_that_isnt_installed_falls_back_and_says_so() {
        let store = store(&["v22.3.0"]);
        // Not a partial match for 22.3.0 either.
        for version in ["24.0.0", "22"] {
            let resolved = resolve(
                store.path(),
                &NodeChoice::Pinned {
                    version: version.into(),
                },
                None,
            );
            assert_eq!(resolved.source, ChoiceSource::Default, "{version}");
            assert_eq!(resolved.dirs, vec![live_bin_dir(store.path())]);
            assert_eq!(resolved.missing.as_deref(), Some(version));
        }
    }

    #[test]
    fn the_system_node_puts_nothing_on_the_path() {
        let store = store(&["v22.3.0"]);
        let resolved = resolve(store.path(), &NodeChoice::System, None);
        assert!(resolved.dirs.is_empty());
        assert_eq!(resolved.source, ChoiceSource::System);
    }

    #[test]
    fn auto_follows_the_projects_version_file_when_it_is_installed() {
        let store = store(&["v20.11.1", "v22.3.0"]);
        let project = tempfile::tempdir().unwrap();
        std::fs::write(project.path().join(".nvmrc"), "22\n").unwrap();
        let resolved = resolve(store.path(), &NodeChoice::Auto, Some(project.path()));
        assert_eq!(resolved.source, ChoiceSource::ProjectFile);
        assert_eq!(resolved.dirs, vec![bin_dir_of(store.path(), "v22.3.0")]);
        assert_eq!(resolved.missing, None);
    }

    #[test]
    fn auto_falls_back_to_the_default_when_the_projects_version_isnt_installed() {
        let store = store(&["v22.3.0"]);
        let project = tempfile::tempdir().unwrap();
        std::fs::write(project.path().join(".nvmrc"), "24\n").unwrap();
        let resolved = resolve(store.path(), &NodeChoice::Auto, Some(project.path()));
        assert_eq!(resolved.source, ChoiceSource::Default);
        assert_eq!(resolved.dirs, vec![live_bin_dir(store.path())]);
        assert!(
            resolved
                .missing
                .as_deref()
                .is_some_and(|v| v.contains("24")),
            "{resolved:?}"
        );
    }

    #[test]
    fn auto_without_a_version_file_is_the_default() {
        let store = store(&["v22.3.0"]);
        let project = tempfile::tempdir().unwrap();
        let resolved = resolve(store.path(), &NodeChoice::Auto, Some(project.path()));
        assert_eq!(resolved.source, ChoiceSource::Default);
        assert_eq!(resolved.dirs, vec![live_bin_dir(store.path())]);
        assert_eq!(resolved.missing, None);
    }

    #[test]
    fn each_workspace_has_its_own_choice() {
        let pin = |version: &str| NodeChoice::Pinned {
            version: version.into(),
        };
        let a = root("registry/node-a");
        let ab = root("registry/node-ab");
        let b = root("registry/node-b");
        set_choice(&a, pin("20.11.1"));
        set_choice(&b, NodeChoice::System);

        assert_eq!(choice_for(Some(&a)), pin("20.11.1"));
        assert_eq!(choice_for(Some(&b)), NodeChoice::System);
        // A folder inside a workspace is that workspace's.
        assert_eq!(choice_for(Some(&a.join("src").join("lib"))), pin("20.11.1"));
        // A folder whose name only starts the same way is not.
        assert_eq!(choice_for(Some(&ab)), NodeChoice::Auto);
        assert_eq!(choice_for(None), NodeChoice::Auto);
        // A trailing separator is the same folder.
        assert_eq!(
            choice_for(Some(Path::new(&format!("{}/", a.display())))),
            pin("20.11.1")
        );

        // A workspace nested in another: the longer root wins.
        let inner = a.join("packages").join("web");
        set_choice(&inner, pin("22.3.0"));
        assert_eq!(choice_for(Some(&inner.join("src"))), pin("22.3.0"));
        assert_eq!(choice_for(Some(&a.join("packages"))), pin("20.11.1"));

        clear_choice(&a);
        assert_eq!(choice_for(Some(&a)), NodeChoice::Auto);
        assert_eq!(choice_for(Some(&inner)), pin("22.3.0"));
        clear_choice(&inner);
        clear_choice(&b);
    }

    #[cfg(windows)]
    #[test]
    fn windows_roots_match_whatever_their_case_or_prefix() {
        let root = PathBuf::from(r"H:\Registry\Case\Node-App");
        set_choice(&root, NodeChoice::System);
        assert_eq!(
            choice_for(Some(Path::new(r"h:\registry\case\node-app\src"))),
            NodeChoice::System
        );
        assert_eq!(
            choice_for(Some(Path::new(r"\\?\H:\REGISTRY\CASE\NODE-APP"))),
            NodeChoice::System
        );
        assert_eq!(
            choice_for(Some(Path::new("H:/Registry/Case/Node-App/src"))),
            NodeChoice::System
        );
        clear_choice(&root);
    }

    #[test]
    fn a_workspace_link_is_named_after_its_folder() {
        let store = Path::new("store");
        let a = root("names/node-a");
        assert_eq!(workspace_link(store, &a), workspace_link(store, &a));
        assert_ne!(
            workspace_link(store, &a),
            workspace_link(store, &root("names/node-b"))
        );
        assert_eq!(
            workspace_link(store, &a),
            workspace_link(store, Path::new(&format!("{}/", a.display())))
        );
        if cfg!(windows) {
            assert_eq!(
                workspace_link(store, Path::new(r"H:\Names\Node-A")),
                workspace_link(store, Path::new(r"h:\names\node-a"))
            );
        }
    }

    /// The whole point: two workspaces run two versions at once, a switch
    /// in one re-points only its own link, and the `PATH` entry a running
    /// terminal has stays the same throughout.
    #[test]
    fn each_workspace_link_follows_its_own_choice() {
        let store = store(&["v20.11.1", "v22.3.0"]);
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let pin = |version: &str| NodeChoice::Pinned {
            version: version.into(),
        };
        let points_at = |root: &Path| {
            std::fs::canonicalize(workspace_link(store.path(), root))
                .map(|target| target.to_string_lossy().into_owned())
                .ok()
        };

        set_choice(a.path(), pin("20.11.1"));
        set_choice(b.path(), pin("22.3.0"));
        let entry_a = bin_dirs_for(store.path(), Some(&a.path().join("src")));
        let entry_b = bin_dirs_for(store.path(), Some(b.path()));
        assert_eq!(entry_a, vec![workspace_bin_dir(store.path(), a.path())]);
        assert_eq!(entry_b, vec![workspace_bin_dir(store.path(), b.path())]);
        assert!(points_at(a.path()).unwrap().contains("v20.11.1"));
        assert!(points_at(b.path()).unwrap().contains("v22.3.0"));

        // A switch in A: same entry, new version; B untouched.
        set_choice(a.path(), pin("22.3.0"));
        sync_workspace(store.path(), a.path()).unwrap();
        assert_eq!(bin_dirs_for(store.path(), Some(a.path())), entry_a);
        assert!(points_at(a.path()).unwrap().contains("v22.3.0"));
        assert!(points_at(b.path()).unwrap().contains("v22.3.0"));

        // The system Node.js: A's link goes, its entry stays.
        set_choice(a.path(), NodeChoice::System);
        assert_eq!(bin_dirs_for(store.path(), Some(a.path())), entry_a);
        assert_eq!(points_at(a.path()), None);
        assert!(points_at(b.path()).is_some());

        // Auto with no version file follows the IDE-wide default, live.
        set_choice(a.path(), NodeChoice::Auto);
        assert_eq!(bin_dirs_for(store.path(), Some(a.path())), entry_a);
        assert_eq!(
            points_at(a.path()),
            None,
            "no default yet: the system Node.js"
        );
        crate::use_version(store.path(), "20").unwrap();
        assert!(points_at(a.path()).unwrap().contains("v20.11.1"));

        // Nothing of the switches' own is left in the store, and no alias
        // was added beside the default one.
        let leftovers: Vec<_> = std::fs::read_dir(store.path())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with('.'))
            .map(|entry| entry.file_name())
            .collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
        let aliases: Vec<_> = std::fs::read_dir(default_link(store.path()).parent().unwrap())
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.file_name())
            .collect();
        assert_eq!(aliases, vec![std::ffi::OsString::from("default")]);

        // Only the workspaces still open keep their links.
        assert_eq!(remove_stale_links(store.path(), &[b.path()]).unwrap(), 1);
        assert_eq!(points_at(a.path()), None);
        assert!(points_at(b.path()).is_some());

        clear_choice(a.path());
        clear_choice(b.path());
    }

    #[test]
    fn a_workspace_link_never_touches_the_ide_wide_default() {
        let store = store(&["v20.11.1", "v22.3.0"]);
        let a = tempfile::tempdir().unwrap();
        crate::use_version(store.path(), "20").unwrap();
        set_choice(
            a.path(),
            NodeChoice::Pinned {
                version: "22.3.0".into(),
            },
        );
        bin_dirs_for(store.path(), Some(a.path()));
        assert_eq!(crate::active(store.path()).unwrap().version, "v20.11.1");
        clear_choice(a.path());
    }
}
