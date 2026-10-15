//! The running processes of one runtime — Node.js or Python — as the Node
//! and Python panels list them: what each one is, whose it is, and stopping
//! one.
//!
//! A row used to read `node.exe` or `python.exe`. A process's command line
//! says what it really is (`next dev`, `vite`, `flask run`), the folders it
//! and the processes above it were started in say which project it belongs
//! to, and the processes above it say whether the IDE itself runs it (a
//! language server). That is all read here, off the main thread.
//!
//! [`ProcessList`] is the list itself, a view either panel puts in its
//! Processes section.

mod labels;
mod process_list;
mod redact;

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

pub use process_list::ProcessList;
pub use redact::redact;

/// A runtime whose processes are listed.
#[derive(Clone, Copy)]
pub struct Runtime {
    /// As messages name it: "Node", "Python".
    pub name: &'static str,
    /// Whether an executable's name, without its extension and lowercased,
    /// is this runtime's.
    pub is_runtime: fn(stem: &str) -> bool,
    /// What a process is, from its command line and its executable's name.
    pub label: fn(command: &[String], name: &str) -> String,
}

pub const NODE: Runtime = Runtime {
    name: "Node",
    is_runtime: labels::is_node,
    label: labels::node_label,
};

pub const PYTHON: Runtime = Runtime {
    name: "Python",
    is_runtime: labels::is_python,
    label: labels::python_label,
};

/// One process, as the list shows it.
#[derive(Clone, Debug)]
pub struct RunningProcess {
    pub pid: u32,
    /// When it started, in seconds since the epoch. With the pid, this is
    /// what tells the process apart from a later one that the OS gave the
    /// same number.
    pub start_time: u64,
    /// What it is, from its command line: `next dev`, `tsserver`,
    /// `manage.py runserver`.
    pub label: String,
    /// The whole command line, for the tooltip, with secrets hidden.
    pub command_line: String,
    /// Started by the IDE itself (a language server, a formatter), however
    /// indirectly, and not from one of its terminals: stopping it breaks an
    /// editor feature until the IDE starts it again.
    pub ide_owned: bool,
    /// Started from the workspace: in its folder, running a file from it, or
    /// below a process that was (`npm run dev` started there runs the
    /// server, wherever the server's own files are).
    pub in_workspace: bool,
    pub cpu_percent: f32,
    pub memory_mb: f64,
}

/// What is read per process on each scan. Command lines and working folders
/// don't change for a running process, so they are read once per process.
/// Working folders are read for every process, not only the runtime's: a
/// process is placed by the folders of the ones above it too.
fn refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_cpu()
        .with_memory()
        .with_cmd(UpdateKind::OnlyIfNotSet)
        .with_cwd(UpdateKind::OnlyIfNotSet)
}

/// Programs that run other programs for a person: a process below one of
/// these was started from a terminal, the Script Runner or a workflow, not
/// by the IDE for itself.
const SHELLS: &[&str] = &[
    "cmd",
    "powershell",
    "pwsh",
    "bash",
    "sh",
    "zsh",
    "fish",
    "nu",
    "dash",
    "ksh",
    "tcsh",
    "csh",
    "wsl",
];

/// An executable's name as compared here: no extension, lowercased.
fn stem(name: &str) -> String {
    let name = name.to_lowercase();
    match name.strip_suffix(".exe") {
        Some(stem) => stem.to_string(),
        None => name,
    }
}

/// Refreshes `sys` and returns `runtime`'s processes, heaviest first, placed
/// against the workspace at `root`. Walks the whole process table: call it
/// from a background thread.
pub fn scan(sys: &mut System, runtime: Runtime, root: Option<&Path>) -> Vec<RunningProcess> {
    sys.refresh_cpu_all();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind());
    let sys: &System = sys;
    let cpu_count = sys.cpus().len().max(1) as f32;
    let ide = Pid::from_u32(std::process::id());
    let root = root
        .map(|root| comparable(&root.to_string_lossy()))
        .filter(|root| !root.is_empty());

    let mut processes: Vec<RunningProcess> = sys
        .processes()
        .iter()
        .filter(|(_, process)| (runtime.is_runtime)(&stem(&process.name().to_string_lossy())))
        .map(|(pid, process)| {
            let name = process.name().to_string_lossy().into_owned();
            let command = redact(&command_of(process));
            let (lineage, under_ide) = lineage(sys, *pid, ide);
            let started_from_shell = lineage
                .iter()
                .skip(1)
                .filter_map(|pid| sys.process(*pid))
                .any(|above| SHELLS.contains(&stem(&above.name().to_string_lossy()).as_str()));
            let ide_owned = under_ide && !started_from_shell;
            let in_workspace = !ide_owned
                && root.as_deref().is_some_and(|root| {
                    lineage
                        .iter()
                        .filter_map(|pid| sys.process(*pid))
                        .any(|process| started_in(process, root))
                });
            RunningProcess {
                pid: pid.as_u32(),
                start_time: process.start_time(),
                label: (runtime.label)(&command, &name),
                command_line: command.join(" "),
                ide_owned,
                in_workspace,
                cpu_percent: process.cpu_usage() / cpu_count,
                memory_mb: process.memory() as f64 / 1_048_576.0,
            }
        })
        .collect();
    processes.sort_by(|a, b| b.memory_mb.total_cmp(&a.memory_mb));
    processes
}

fn command_of(process: &Process) -> Vec<String> {
    process
        .cmd()
        .iter()
        .map(|part| part.to_string_lossy().into_owned())
        .collect()
}

/// `pid` and the processes above it, nearest first, up to (not including)
/// the IDE; and whether the IDE is up there at all.
fn lineage(sys: &System, pid: Pid, ide: Pid) -> (Vec<Pid>, bool) {
    let mut lineage = vec![pid];
    let mut current = pid;
    while lineage.len() < 64 {
        let Some(process) = sys.process(current) else {
            break;
        };
        let Some(parent) = process.parent() else {
            break;
        };
        if parent == ide {
            return (lineage, true);
        }
        // A "parent" younger than its child is a later process that got the
        // exited parent's number: the real parent is gone.
        let Some(above) = sys.process(parent) else {
            break;
        };
        if above.start_time() > process.start_time() || lineage.contains(&parent) {
            break;
        }
        lineage.push(parent);
        current = parent;
    }
    (lineage, false)
}

/// Whether `process` was started in the folder `root` (already
/// [`comparable`]) or one inside it, or runs a file from there.
fn started_in(process: &Process, root: &str) -> bool {
    let cwd = process.cwd().map(|cwd| cwd.to_string_lossy().into_owned());
    belongs(cwd.as_deref(), &command_of(process).join(" "), root)
}

fn belongs(cwd: Option<&str>, command_line: &str, root: &str) -> bool {
    let inside = format!("{root}/");
    cwd.is_some_and(|cwd| {
        let cwd = comparable(cwd);
        cwd == root || cwd.starts_with(&inside)
    }) || comparable(command_line).contains(&inside)
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

/// How long a process is given to finish after being asked to, before it is
/// made to.
#[cfg(unix)]
const GRACE: Duration = Duration::from_secs(2);

/// Stops the process `pid` that started at `start_time`, and everything it
/// started: `npm run dev` runs the server as its child, which stopping npm
/// alone would leave running.
///
/// Nothing is stopped when `pid` now belongs to another process: the list
/// can be up to a scan old, and the OS reuses numbers.
///
/// On Linux and macOS the processes are first asked to finish (`SIGTERM`),
/// so a dev server can close its port and clean up; whatever is still
/// running after a moment is then made to. Windows has no way to ask a
/// console program to finish from outside its console, so there they are
/// ended at once.
///
/// `Ok` says what was stopped; `Err` says what wasn't, including any process
/// it started that is still running. Blocks: call it from a background
/// thread.
pub fn stop_tree(pid: u32, start_time: u64) -> Result<String, String> {
    let mut sys = System::new();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    let root = Pid::from_u32(pid);
    if !sys
        .process(root)
        .is_some_and(|process| process.start_time() == start_time)
    {
        return Err(format!("Process {pid} has already exited"));
    }

    let family = family(&sys, root);
    let pids: Vec<Pid> = family.keys().copied().collect();
    let alive = |sys: &System, pid: &Pid| {
        sys.process(*pid)
            .is_some_and(|process| Some(&process.start_time()) == family.get(pid))
    };
    let refresh = |sys: &mut System| {
        sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(&pids),
            true,
            ProcessRefreshKind::nothing(),
        );
    };

    #[cfg(unix)]
    {
        for pid in &pids {
            if let Some(process) = sys.process(*pid) {
                process.kill_with(sysinfo::Signal::Term);
            }
        }
        let deadline = std::time::Instant::now() + GRACE;
        while std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
            refresh(&mut sys);
            if !pids.iter().any(|pid| alive(&sys, pid)) {
                return Ok(format!("Stopped process {pid}"));
            }
        }
    }

    // Children first, so none is left without a parent while it is still
    // being stopped.
    for member in family_order(&sys, root, &pids).iter().rev() {
        if alive(&sys, member)
            && let Some(process) = sys.process(*member)
        {
            process.kill();
        }
    }

    // Ending a process isn't instant; give the OS a moment before checking.
    std::thread::sleep(Duration::from_millis(300));
    refresh(&mut sys);
    if alive(&sys, &root) {
        return Err(format!(
            "Couldn't stop process {pid}. It may be running as administrator or as another \
             user, which the IDE can't stop."
        ));
    }
    let left: Vec<String> = pids
        .iter()
        .filter(|&pid| alive(&sys, pid))
        .map(|pid| pid.to_string())
        .collect();
    if left.is_empty() {
        Ok(format!("Stopped process {pid}"))
    } else {
        Err(format!(
            "Stopped process {pid}, but {} it started {} still running: {}",
            if left.len() == 1 {
                "a process"
            } else {
                "processes"
            },
            if left.len() == 1 { "is" } else { "are" },
            left.join(", ")
        ))
    }
}

/// `root` and everything below it, each with its start time. A process only
/// counts as a child when it started after its parent: one that started
/// before has a parent number that was reused.
fn family(sys: &System, root: Pid) -> HashMap<Pid, u64> {
    let mut family = HashMap::new();
    if let Some(process) = sys.process(root) {
        family.insert(root, process.start_time());
    }
    let mut next = vec![root];
    while let Some(parent) = next.pop() {
        let parent_start = family[&parent];
        for (pid, process) in sys.processes() {
            if process.parent() == Some(parent)
                && process.start_time() >= parent_start
                && !family.contains_key(pid)
            {
                family.insert(*pid, process.start_time());
                next.push(*pid);
            }
        }
    }
    family
}

/// `pids` (a [`family`]) parents before children.
fn family_order(sys: &System, root: Pid, pids: &[Pid]) -> Vec<Pid> {
    let mut order = vec![root];
    let mut next = 0;
    while next < order.len() {
        let parent = order[next];
        for pid in pids {
            if !order.contains(pid) && sys.process(*pid).and_then(Process::parent) == Some(parent) {
                order.push(*pid);
            }
        }
        next += 1;
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_process_belongs_to_the_folder_it_was_started_in_or_runs_from() {
        let root = comparable("/work/app");
        assert!(belongs(Some("/work/app"), "node x", &root));
        assert!(belongs(Some("/work/app/packages/web"), "node x", &root));
        assert!(belongs(
            Some("/elsewhere"),
            "node /work/app/node_modules/vite/bin/vite.js",
            &root
        ));
        // A sibling folder whose name starts the same way is not the project.
        assert!(!belongs(Some("/work/app-old"), "node x", &root));
        assert!(!belongs(None, "node /work/other/server.js", &root));
    }

    #[test]
    fn executable_names_are_compared_without_extension_or_case() {
        assert_eq!(stem("Node.EXE"), "node");
        assert_eq!(stem("python3.12"), "python3.12");
        assert!(SHELLS.contains(&stem("pwsh.exe").as_str()));
    }

    #[test]
    fn the_running_test_is_found_with_its_start_time() {
        // This test's own process: its family is at least itself, and a
        // wrong start time makes it someone else's number.
        let mut sys = System::new();
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );
        let me = Pid::from_u32(std::process::id());
        let started = sys.process(me).unwrap().start_time();
        assert_eq!(family(&sys, me).get(&me), Some(&started));
        assert!(
            stop_tree(std::process::id(), started + 1_000_000)
                .unwrap_err()
                .contains("already exited")
        );
    }
}
