//! Which host processes belong to an execution.
//!
//! There's no pid namespace, so sandboxed processes share the host's pid
//! space. Every process of an execution descends from its helper, which is a
//! child subreaper: orphans are reparented to it rather than escaping to the
//! host's init.

use std::collections::HashMap;
use std::path::{Component, Path};

pub(crate) struct ProcessInfo {
    pub pid: i32,
    pub parent: i32,
    pub group: i32,
}

/// Parse `/proc/<pid>/stat`. The command name is in parentheses and may
/// itself contain spaces and parentheses, so fields are read after the last
/// `)`.
pub(crate) fn process_info(pid: i32) -> Option<ProcessInfo> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let (_, fields) = stat.rsplit_once(')')?;
    let mut fields = fields.split_whitespace();
    let _state = fields.next()?;
    let parent = fields.next()?.parse().ok()?;
    let group = fields.next()?.parse().ok()?;
    Some(ProcessInfo { pid, parent, group })
}

/// The controlling terminal device of `pid` (0: none).
pub(crate) fn controlling_terminal(pid: i32) -> Option<u64> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let (_, fields) = stat.rsplit_once(')')?;
    // state ppid pgrp session tty_nr
    fields
        .split_whitespace()
        .nth(4)?
        .parse::<i64>()
        .ok()
        .map(|tty| tty as u64)
}

pub(crate) fn thread_group(tid: i32) -> Option<i32> {
    read_status_head(tid)?
        .lines()
        .find_map(|line| line.strip_prefix("Tgid:"))
        .and_then(|value| value.trim().parse().ok())
}

/// The start of `/proc/<tid>/status` (`Name`, `Umask`, `State`, `Tgid`,
/// ...), in one `read`. Reading the whole file makes the kernel format
/// dozens of lines, through a growing buffer.
pub(crate) fn read_status_head(tid: i32) -> Option<String> {
    use std::io::Read as _;
    let mut file = std::fs::File::open(format!("/proc/{tid}/status")).ok()?;
    let mut buffer = [0u8; 512];
    let length = file.read(&mut buffer).ok()?;
    Some(String::from_utf8_lossy(&buffer[..length]).into_owned())
}

pub(crate) fn all_processes() -> Vec<ProcessInfo> {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
        .filter_map(process_info)
        .collect()
}

/// The pid in a `/proc/<pid>/...` path, if it is one.
pub(crate) fn proc_path_pid(path: &Path) -> Option<i32> {
    let mut components = path.components();
    if components.next() != Some(Component::RootDir)
        || components.next() != Some(Component::Normal("proc".as_ref()))
    {
        return None;
    }
    match components.next()? {
        Component::Normal(name) => name.to_str()?.parse().ok(),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ProcessTree {
    pub helper: i32,
}

impl ProcessTree {
    /// Whether `pid` (a process or thread id) strictly descends from the
    /// helper.
    pub fn contains(&self, pid: i32) -> bool {
        let mut current = pid;
        for _ in 0..4096 {
            let Some(info) = process_info(current) else {
                return false;
            };
            if info.parent == self.helper {
                return true;
            }
            if info.parent <= 1 {
                return false;
            }
            current = info.parent;
        }
        false
    }

    pub fn members(&self) -> Vec<ProcessInfo> {
        let processes = all_processes();
        let mut children: HashMap<i32, Vec<usize>> = HashMap::new();
        for (index, process) in processes.iter().enumerate() {
            children.entry(process.parent).or_default().push(index);
        }
        let mut members = Vec::new();
        let mut stack = vec![self.helper];
        while let Some(parent) = stack.pop() {
            for &index in children.get(&parent).into_iter().flatten() {
                stack.push(processes[index].pid);
                members.push(index);
            }
        }
        let mut processes: Vec<Option<ProcessInfo>> = processes.into_iter().map(Some).collect();
        members
            .into_iter()
            .filter_map(|index| processes[index].take())
            .collect()
    }
}
