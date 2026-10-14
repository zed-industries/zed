//! The Node panel's process list: which Node processes are running, what
//! each one is, whose it is, and stopping one.
//!
//! Every row used to read `node.exe`. A process's command line says what it
//! really is (`next dev`, `vite`, `tsserver`, `npm run dev`), its working
//! folder says which project started it, and its parent says whether the IDE
//! itself runs it (a language server). That is all read here, off the main
//! thread, on each tick of the panel's poll.

use std::path::{Path, PathBuf};

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

/// One Node process, as the list shows it.
pub(crate) struct NodeProcess {
    pub pid: u32,
    /// What it is, from its command line: `next dev`, `tsserver`, `server.js`.
    pub label: String,
    /// The whole command line, for the tooltip.
    pub command_line: String,
    /// The folder it was started in, when the OS would say.
    pub cwd: Option<PathBuf>,
    /// Started by the IDE itself (a language server, a formatter): stopping
    /// it breaks an editor feature until the IDE starts it again.
    pub ide_owned: bool,
    pub cpu_percent: f32,
    pub memory_mb: f64,
}

impl NodeProcess {
    /// Whether this process belongs to the project at `root`: started in it,
    /// or running a file from it.
    pub fn belongs_to(&self, root: &Path) -> bool {
        let root = comparable(&root.to_string_lossy());
        if root.is_empty() {
            return false;
        }
        let in_root = |path: &str| {
            let path = comparable(path);
            path == root || path.starts_with(&format!("{root}/"))
        };
        self.cwd
            .as_ref()
            .is_some_and(|cwd| in_root(&cwd.to_string_lossy()))
            || comparable(&self.command_line).contains(&format!("{root}/"))
    }
}

/// A path as compared here: forward slashes, no trailing slash, and on
/// Windows, where paths ignore case, lowercase.
fn comparable(path: &str) -> String {
    let path = path.replace('\\', "/");
    let path = path.trim_end_matches('/');
    if cfg!(windows) {
        path.to_lowercase()
    } else {
        path.to_string()
    }
}

/// What is read per process on each tick. Command lines and working folders
/// don't change for a running process, so they are read once per process.
fn refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_cpu()
        .with_memory()
        .with_cmd(UpdateKind::OnlyIfNotSet)
        .with_cwd(UpdateKind::OnlyIfNotSet)
}

/// Refreshes `sys` and returns its Node processes, heaviest first. Walks the
/// whole process table: call it from a background thread.
pub(crate) fn scan(sys: &mut System) -> Vec<NodeProcess> {
    sys.refresh_cpu_all();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind());
    let cpu_count = sys.cpus().len().max(1) as f32;
    let ide = Pid::from_u32(std::process::id());

    let mut processes: Vec<NodeProcess> = sys
        .processes()
        .iter()
        .filter(|(_, process)| {
            process
                .name()
                .to_string_lossy()
                .to_lowercase()
                .contains("node")
        })
        .map(|(pid, process)| {
            let name = process.name().to_string_lossy().into_owned();
            let command: Vec<String> = process
                .cmd()
                .iter()
                .map(|part| part.to_string_lossy().into_owned())
                .collect();
            NodeProcess {
                pid: pid.as_u32(),
                label: label(&command, &name),
                command_line: command.join(" "),
                cwd: process.cwd().map(Path::to_path_buf),
                // A process the IDE starts directly. Anything run in one of
                // its terminals, the Script Runner or a workflow has a shell
                // between the two, so it isn't counted as the IDE's own.
                ide_owned: process.parent() == Some(ide),
                cpu_percent: process.cpu_usage() / cpu_count,
                memory_mb: process.memory() as f64 / 1_048_576.0,
            }
        })
        .collect();
    processes.sort_by(|a, b| b.memory_mb.total_cmp(&a.memory_mb));
    processes
}

/// File names that say nothing about what a script is, so the package's own
/// name is used instead.
const GENERIC_SCRIPT_NAMES: &[&str] = &["index", "cli", "main", "bin", "start", "run", "npm-cli"];

/// What a Node process is, from its command line: the tool and up to two of
/// its subcommand words (`next dev`, `npm run dev`, `gulp build`), or the
/// script's file name for a project's own script (`server.js`). `name` (the
/// executable's) when there is no script at all, as for a bare `node` REPL.
pub(crate) fn label(command: &[String], name: &str) -> String {
    let fallback = || {
        Path::new(name)
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_else(|| name.to_string())
    };
    let mut args = command.iter().skip(1).peekable();
    // Node's own options come before the script.
    while args.next_if(|arg| arg.starts_with('-')).is_some() {}
    let Some(script) = args.next() else {
        return fallback();
    };
    let words: Vec<&str> = args
        .take_while(|arg| !arg.starts_with('-'))
        .take(2)
        .map(String::as_str)
        .collect();

    let script = script.replace('\\', "/");
    let file = Path::new(&script);
    let stem = file
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();

    let tool = match package_of(&script) {
        // A package's script that names itself (`tsserver.js` in
        // `typescript`) says more than the package does.
        Some(package) if GENERIC_SCRIPT_NAMES.contains(&stem.as_str()) || stem == package => {
            package
        }
        Some(_) => stem,
        // A script of the project's own: its file name says what it is.
        None => file
            .file_name()
            .map(|file| file.to_string_lossy().into_owned())
            .unwrap_or_else(|| script.clone()),
    };

    let mut label = std::iter::once(tool.as_str())
        .chain(words)
        .collect::<Vec<_>>()
        .join(" ");
    const MAX: usize = 40;
    if label.chars().count() > MAX {
        label = label.chars().take(MAX - 1).collect::<String>() + "\u{2026}";
    }
    label
}

/// The package a script belongs to: the folder after the last
/// `node_modules/`, with its scope (`@scope/name`) when it has one.
fn package_of(script: &str) -> Option<String> {
    let (_, rest) = script.rsplit_once("node_modules/")?;
    let mut parts = rest.split('/');
    let first = parts.next()?;
    if first.starts_with('@') {
        Some(format!("{first}/{}", parts.next()?))
    } else {
        Some(first.to_string())
    }
}

/// Stops `pid` and everything it started — `npm run dev` runs the server as
/// its child, which stopping npm alone would leave running. Children go
/// first, so none is left without a parent while it is still being stopped.
pub(crate) fn stop_tree(pid: u32) -> Result<(), String> {
    let mut sys = System::new();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    let root = Pid::from_u32(pid);
    if sys.process(root).is_none() {
        return Err(format!("Process {pid} has already exited"));
    }

    let mut family = vec![root];
    let mut next = 0;
    while next < family.len() {
        let parent = family[next];
        let children: Vec<Pid> = sys
            .processes()
            .iter()
            .filter(|(child, process)| {
                process.parent() == Some(parent) && !family.contains(child)
            })
            .map(|(child, _)| *child)
            .collect();
        family.extend(children);
        next += 1;
    }

    let mut stopped_root = false;
    for member in family.iter().rev() {
        if let Some(process) = sys.process(*member) {
            let killed = process.kill();
            if *member == root {
                stopped_root = killed;
            }
        }
    }
    if stopped_root {
        Ok(())
    } else {
        Err(format!("Couldn't stop process {pid}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| part.to_string()).collect()
    }

    #[test]
    fn labels_say_what_each_process_is() {
        let cases: &[(&[&str], &str)] = &[
            (
                &[r"C:\nodejs\node.exe", r"H:\app\node_modules\next\dist\bin\next", "dev"],
                "next dev",
            ),
            (
                &["node", r"C:\nodejs\node_modules\npm\bin\npm-cli.js", "run", "dev"],
                "npm run dev",
            ),
            (
                &[
                    "node",
                    "--max-old-space-size=3072",
                    "/opt/ide/node_modules/typescript/lib/tsserver.js",
                    "--serverMode",
                    "partialSemantic",
                ],
                "tsserver",
            ),
            (&["node", "/app/node_modules/vite/bin/vite.js"], "vite"),
            (
                &["node", "/app/node_modules/gulp/bin/gulp.js", "build", "--ship"],
                "gulp build",
            ),
            (
                &["node", "/app/node_modules/@scope/tool/index.js", "watch"],
                "@scope/tool watch",
            ),
            (&["node", "src/server.js", "--port", "3000"], "server.js"),
            (&[r"C:\nodejs\node.exe"], "node"),
        ];
        for (parts, expected) in cases {
            assert_eq!(label(&command(parts), "node.exe"), *expected, "{parts:?}");
        }
    }

    #[test]
    fn a_long_label_is_shortened() {
        let long = "x".repeat(60);
        let label = label(&command(&["node", "/app/server.js", &long]), "node");
        assert_eq!(label.chars().count(), 40);
        assert!(label.ends_with('\u{2026}'));
    }

    fn process(cwd: Option<&str>, command_line: &str) -> NodeProcess {
        NodeProcess {
            pid: 1,
            label: String::new(),
            command_line: command_line.to_string(),
            cwd: cwd.map(PathBuf::from),
            ide_owned: false,
            cpu_percent: 0.0,
            memory_mb: 0.0,
        }
    }

    #[test]
    fn a_process_belongs_to_the_project_it_was_started_in_or_runs_from() {
        let root = Path::new("/work/app");
        assert!(process(Some("/work/app"), "node x").belongs_to(root));
        assert!(process(Some("/work/app/packages/web"), "node x").belongs_to(root));
        assert!(
            process(Some("/elsewhere"), "node /work/app/node_modules/vite/bin/vite.js")
                .belongs_to(root)
        );
        // A sibling folder whose name starts the same way is not the project.
        assert!(!process(Some("/work/app-old"), "node x").belongs_to(root));
        assert!(!process(None, "node /work/other/server.js").belongs_to(root));
        assert!(!process(Some("/work/app"), "node x").belongs_to(Path::new("")));
    }

    #[test]
    fn package_names_keep_their_scope() {
        assert_eq!(package_of("/a/node_modules/next/dist/bin/next").as_deref(), Some("next"));
        assert_eq!(
            package_of("/a/node_modules/x/node_modules/@b/c/i.js").as_deref(),
            Some("@b/c")
        );
        assert_eq!(package_of("/a/src/server.js"), None);
    }
}
