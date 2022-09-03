use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
mod linux;
mod macos;
mod windows;

#[derive(Debug, Copy, Clone)]
pub enum LocalProcessStatus {
    Idle,
    Run,
    Sleep,
    Stop,
    Zombie,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    Parked,
    LockBlocked,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LocalProcessInfo {
    /// The process identifier
    pub pid: u32,
    /// The parent process identifier
    pub ppid: u32,
    /// The COMM name of the process. May not bear any relation to
    /// the executable image name. May be changed at runtime by
    /// the process.
    /// Many systems truncate this
    /// field to 15-16 characters.
    pub name: String,
    /// Path to the executable image
    pub executable: PathBuf,
    /// The argument vector.
    /// Some systems allow changing the argv block at runtime
    /// eg: setproctitle().
    pub argv: Vec<String>,
    /// The current working directory for the process, or an empty
    /// path if it was not accessible for some reason.
    pub cwd: PathBuf,
    /// The status of the process. Not all possible values are
    /// portably supported on all systems.
    pub status: LocalProcessStatus,
    /// A clock value in unspecified system dependent units that
    /// indicates the relative age of the process.
    pub start_time: u64,
    /// The console handle associated with the process, if any.
    #[cfg(windows)]
    pub console: u64,
    /// Child processes, keyed by pid
    pub children: HashMap<u32, LocalProcessInfo>,
}
#[cfg(feature = "lua")]
luahelper::impl_lua_conversion_dynamic!(LocalProcessInfo);

impl LocalProcessInfo {
    /// Walk this sub-tree of processes and return a unique set
    /// of executable base names. eg: `foo/bar` and `woot/bar`
    /// produce a set containing just `bar`.
    pub fn flatten_to_exe_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();

        fn flatten(item: &LocalProcessInfo, names: &mut HashSet<String>) {
            if let Some(exe) = item.executable.file_name() {
                names.insert(exe.to_string_lossy().into_owned());
            }
            for proc in item.children.values() {
                flatten(proc, names);
            }
        }

        flatten(self, &mut names);
        names
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    pub fn with_root_pid(_pid: u32) -> Option<Self> {
        None
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    pub fn current_working_dir(_pid: u32) -> Option<PathBuf> {
        None
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    pub fn executable_path(_pid: u32) -> Option<PathBuf> {
        None
    }
}
