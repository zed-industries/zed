//! Node.js runtime backend — runtime detection, filesystem helpers, and
//! package.json editing support. Node.js versions themselves are managed by
//! `node_versions`.
//!
//! No GPUI, no app-state dependency: every function here takes plain
//! strings/paths and returns a plain value or `Result` — a host panel wires
//! this into its own UI/state, this crate just does the work.

use std::path::Path;

use serde::Serialize;

mod path_env;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// ── Types ─────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_symlink: bool,
}

/// A directory that looks like a Node project: it directly contains a
/// `package.json`. The scanner walks `scan_node_projects` and reports these.
#[derive(Serialize, Clone, Debug)]
pub struct DetectedNodeProject {
    pub path: String,
    pub name: String,
}

// ── Filesystem ────────────────────────────────────────────────────────

pub fn read_text_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Cannot read {path}: {e}"))
}

pub fn write_file(path: String, content: String) -> Result<(), String> {
    // Create the parent directory if it doesn't exist yet — without this,
    // writing into a not-yet-existing directory fails outright.
    if let Some(parent) = Path::new(&path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create directory for {path}: {e}"))?;
    }
    std::fs::write(&path, content).map_err(|e| format!("Cannot write {path}: {e}"))
}

/// Sends a path to the OS file manager. On Windows this opens Explorer with
/// the file selected (`explorer /select,<path>`); the `CREATE_NO_WINDOW` flag
/// prevents a console flash. On other platforms it opens the parent
/// directory.
///
/// Starts the file manager without waiting for it, but starting a process can
/// still take a moment: call it from a background thread.
#[allow(
    clippy::disallowed_methods,
    reason = "doesn't wait for the file manager; callers run it off the UI thread"
)]
pub fn reveal_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;
        let parent = Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or(path.clone());
        let dir = dunce::canonicalize(&parent).map_err(|e| format!("{e}"))?;
        let args = if Path::new(&path).is_dir() {
            vec![dir.to_string_lossy().into_owned()]
        } else {
            vec!["/select,".to_string() + &path]
        };
        Command::new("explorer")
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Cannot open Explorer: {e}"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        let parent = Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or(path);
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };
        Command::new(opener)
            .arg(parent)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Cannot open file manager: {e}"))
    }
}

pub fn read_dir(path: String) -> Result<Vec<DirEntry>, String> {
    let entries = std::fs::read_dir(&path).map_err(|e| format!("Cannot read {path}: {e}"))?;
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        // `<name>.flow.layout.json` is a hidden companion to `<name>.flow.json`,
        // so it's excluded here rather than in just the Explorer: every panel
        // that lists a directory goes through this same function and the
        // sidecar shouldn't turn up as a stray row.
        if name.ends_with(".flow.layout.json") {
            continue;
        }
        let ft = entry.file_type().map_err(|e| e.to_string())?;
        out.push(DirEntry {
            name,
            path: entry.path().to_string_lossy().into_owned(),
            is_dir: ft.is_dir(),
            is_symlink: ft.is_symlink(),
        });
    }
    out.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(out)
}

// ── Project scanning ──────────────────────────────────────────────────

/// Maximum depth the node-project scanner descends from the workspace root.
pub const MAX_SCAN_DEPTH: usize = 5;
/// Directories never descended into while scanning for node projects.
pub const SCAN_SKIP_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    ".forge",
    "dist",
    "build",
    "target",
    ".next",
    "coverage",
    ".cache",
    "out",
    ".turbo",
];

/// Walks `root` (to [`MAX_SCAN_DEPTH`], skipping [`SCAN_SKIP_DIRS`]) and
/// reports every directory that directly contains a `package.json`. Stops
/// descending once a `.git` directory is found, treating that as a project
/// boundary.
pub fn scan_node_projects(root: &str, depth: usize) -> Vec<DetectedNodeProject> {
    if depth > MAX_SCAN_DEPTH {
        return Vec::new();
    }
    let entries = match read_dir(root.to_string()) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut projects = Vec::new();

    if entries
        .iter()
        .any(|e| e.name.to_lowercase() == "package.json")
    {
        let name = std::path::Path::new(root)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "main".into());
        projects.push(DetectedNodeProject {
            path: root.to_string(),
            name,
        });
    }

    if depth > 0 && entries.iter().any(|e| e.name == ".git" && e.is_dir) {
        return projects;
    }

    for entry in &entries {
        if !entry.is_dir {
            continue;
        }
        if SCAN_SKIP_DIRS.contains(&entry.name.as_str()) {
            continue;
        }
        let sub = scan_node_projects(&entry.path, depth + 1);
        projects.extend(sub);
    }

    projects
}

// ── Runtime queries ───────────────────────────────────────────────────

/// Run a command that might be a `.cmd` shim, returning its captured stdout
/// (or stderr when stdout is empty). Falls back to `cmd /C` on Windows so
/// npm-installed `.cmd` CLIs resolve through PATHEXT.
///
/// Blocks until the command exits: only reached through `query_runtime`,
/// whose callers run it on background threads.
#[allow(
    clippy::disallowed_methods,
    reason = "blocking by design; only called from background threads"
)]
fn run_captured(exe: &str, args: &[&str]) -> Result<String, String> {
    let mut c = gpui_util::new_std_command(exe);
    c.args(args);
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(CREATE_NO_WINDOW);
        path_env::enrich_path(&mut c);
    }
    let out = match c.output() {
        Ok(out) => out,
        Err(e) => {
            log::debug!("run_captured: direct '{exe}' failed: {e}");
            #[cfg(target_os = "windows")]
            {
                let mut sh = gpui_util::new_std_command("cmd");
                sh.args(["/C", exe]);
                sh.args(args);
                use std::os::windows::process::CommandExt;
                sh.creation_flags(CREATE_NO_WINDOW);
                path_env::enrich_path(&mut sh);
                match sh.output() {
                    Ok(fb) => {
                        log::debug!(
                            "run_captured: cmd /C '{exe}' status={} stdout={} stderr={}",
                            fb.status,
                            String::from_utf8_lossy(&fb.stdout).trim(),
                            String::from_utf8_lossy(&fb.stderr).trim(),
                        );
                        fb
                    }
                    Err(e2) => {
                        log::debug!("run_captured: cmd /C '{exe}' also failed: {e2}");
                        return Err(format!("{exe} not found"));
                    }
                }
            }
            #[cfg(not(target_os = "windows"))]
            return Err(format!("{exe} not found"));
        }
    };
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
    Ok(if stdout.is_empty() { stderr } else { stdout })
}

/// Query `exe --version`.
pub fn query_runtime(exe: String) -> Result<String, String> {
    run_captured(&exe, &["--version"])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, content: &str) {
        write_file(path.to_string_lossy().into_owned(), content.to_string()).unwrap();
    }

    fn sorted_names(projects: &[DetectedNodeProject]) -> Vec<String> {
        let mut names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
        names.sort();
        names
    }

    #[test]
    fn write_file_creates_missing_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a/b/c/package.json");
        let path_string = path.to_string_lossy().into_owned();

        write_file(path_string.clone(), "{\"name\":\"x\"}".to_string()).unwrap();
        assert_eq!(read_text_file(path_string.clone()).unwrap(), "{\"name\":\"x\"}");

        // Overwrites in place.
        write_file(path_string.clone(), "{}".to_string()).unwrap();
        assert_eq!(read_text_file(path_string).unwrap(), "{}");
    }

    #[test]
    fn read_text_file_reports_the_path_on_failure() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.txt").to_string_lossy().into_owned();
        let error = read_text_file(missing.clone()).unwrap_err();
        assert!(error.contains(&missing), "{error}");
    }

    #[test]
    fn read_dir_lists_directories_first_and_hides_flow_layout_sidecars() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(&root.join("zeta.txt"), "");
        write(&root.join("Alpha.txt"), "");
        write(&root.join("build.flow.json"), "");
        write(&root.join("build.flow.layout.json"), "");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::create_dir_all(root.join("Docs")).unwrap();

        let entries = read_dir(root.to_string_lossy().into_owned()).unwrap();
        let summary: Vec<_> = entries
            .iter()
            .map(|e| (e.name.as_str(), e.is_dir))
            .collect();
        assert_eq!(
            summary,
            vec![
                ("Docs", true),
                ("src", true),
                ("Alpha.txt", false),
                ("build.flow.json", false),
                ("zeta.txt", false),
            ]
        );
        assert!(entries.iter().all(|e| Path::new(&e.path).exists()));

        assert!(read_dir(root.join("missing").to_string_lossy().into_owned()).is_err());
    }

    #[test]
    fn scan_finds_every_package_json_outside_skipped_directories() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(&root.join("package.json"), "{}");
        write(&root.join("apps/web/package.json"), "{}");
        write(&root.join("apps/api/package.json"), "{}");
        write(&root.join("apps/api/src/index.js"), "");
        write(&root.join("node_modules/react/package.json"), "{}");
        write(&root.join("apps/web/dist/package.json"), "{}");
        write(&root.join("apps/web/.next/package.json"), "{}");

        let projects = scan_node_projects(&root.to_string_lossy(), 0);
        let root_name = root.file_name().unwrap().to_string_lossy().into_owned();
        let mut expected = vec!["api".to_string(), "web".to_string(), root_name];
        expected.sort();
        assert_eq!(sorted_names(&projects), expected);

        let web = projects.iter().find(|p| p.name == "web").unwrap();
        assert!(Path::new(&web.path).join("package.json").is_file());
    }

    #[test]
    fn scan_treats_a_nested_git_directory_as_a_boundary() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        write(&root.join("package.json"), "{}");
        std::fs::create_dir_all(root.join("vendor/.git")).unwrap();
        write(&root.join("vendor/package.json"), "{}");
        write(&root.join("vendor/inner/package.json"), "{}");
        write(&root.join("mine/package.json"), "{}");

        // The root's own `.git` doesn't stop the scan; a nested repository
        // is reported itself but not descended into.
        let projects = scan_node_projects(&root.to_string_lossy(), 0);
        let root_name = root.file_name().unwrap().to_string_lossy().into_owned();
        let mut expected = vec!["mine".to_string(), "vendor".to_string(), root_name];
        expected.sort();
        assert_eq!(sorted_names(&projects), expected);
    }

    #[test]
    fn scan_stops_past_the_depth_limit_and_on_missing_directories() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("package.json"), "{}");
        let root = dir.path().to_string_lossy().into_owned();

        assert_eq!(scan_node_projects(&root, MAX_SCAN_DEPTH).len(), 1);
        assert!(scan_node_projects(&root, MAX_SCAN_DEPTH + 1).is_empty());
        assert!(scan_node_projects(&dir.path().join("missing").to_string_lossy(), 0).is_empty());
    }
}
