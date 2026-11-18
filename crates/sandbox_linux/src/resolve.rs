//! Path resolution on behalf of a sandboxed thread.
//!
//! The supervisor never opens a path string the target gave it in one go:
//! the kernel would resolve `/proc/self` (and symlinks through it, like
//! `/dev/stdin`) as the *supervisor*, and a racing target could swap
//! components between our check and our use. Instead we walk one component
//! at a time from a root fd, holding each directory, and decide on the
//! object we end up holding.
//!
//! Most paths take a fast path: `openat2` resolves the whole path in the
//! kernel, confined to the root (`RESOLVE_IN_ROOT`, so `..` and absolute
//! symlinks behave as for the target) and refusing magic links. The result
//! is only used when it isn't on procfs, the one place where resolving as the
//! supervisor differs from resolving as the target; anything else falls back
//! to the walk.

use std::collections::VecDeque;
use std::ffi::{CString, OsStr, OsString};
use std::io;
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, OwnedFd};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Component, Path};

use crate::process::{ProcessTree, proc_path_pid};
use crate::sys::{self, errno};

const MAX_SYMLINKS: usize = 40;
const PROC_SUPER_MAGIC: i64 = 0x9fa0;

// Syscalls added after the per-arch tables were unified share one number.
const SYS_OPENAT2: libc::c_long = 437;
const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
const RESOLVE_IN_ROOT: u64 = 0x10;

pub(crate) enum Resolved {
    /// The final component, unopened: in [`Mode::Entry`], or because it
    /// doesn't exist.
    Entry { parent: OwnedFd, name: CString },
    /// The final object, opened `O_PATH`.
    Object(OwnedFd),
}

#[derive(Clone, Copy)]
pub(crate) enum Mode {
    /// Resolve the parent directory, and never look at the final component.
    Entry,
    /// Open the final component, following it if it's a symlink and `follow`.
    Object { follow: bool },
}

pub(crate) struct Resolver<'a> {
    pub root: BorrowedFd<'a>,
    /// The thread the path belongs to, for `/proc/self` and
    /// `/proc/thread-self`.
    pub tid: i32,
    /// The thread's process, looked up only if `/proc/self` is reached.
    pub tgid: &'a dyn Fn() -> io::Result<i32>,
    pub tree: ProcessTree,
}

impl Resolver<'_> {
    pub fn resolve(&self, path: &Path, mode: Mode) -> io::Result<Resolved> {
        if let Some(resolved) = self.resolve_fast(path, mode) {
            return Ok(resolved);
        }
        self.resolve_walk(path, mode)
    }

    /// `None` whenever the walk has to decide (errors included, so that
    /// error cases behave exactly as the walk makes them).
    fn resolve_fast(&self, path: &Path, mode: Mode) -> Option<Resolved> {
        match mode {
            Mode::Object { follow } => {
                let flags = if follow { 0 } else { libc::O_NOFOLLOW };
                let object = self.open_in_root(path.as_os_str(), flags).ok()?;
                (!is_procfs(object.as_fd()).ok()?).then_some(Resolved::Object(object))
            }
            Mode::Entry => {
                let name = match path.components().next_back()? {
                    Component::Normal(name) => name,
                    _ => return None,
                };
                let parent = self
                    .open_in_root(path.parent()?.as_os_str(), libc::O_DIRECTORY)
                    .ok()?;
                if is_procfs(parent.as_fd()).ok()? {
                    return None;
                }
                Some(Resolved::Entry {
                    parent,
                    name: sys::cstring(name.as_bytes()).ok()?,
                })
            }
        }
    }

    fn open_in_root(&self, path: &OsStr, flags: i32) -> io::Result<OwnedFd> {
        #[repr(C)]
        struct OpenHow {
            flags: u64,
            mode: u64,
            resolve: u64,
        }
        let path = sys::cstring(path.as_bytes())?;
        let how = OpenHow {
            flags: (libc::O_PATH | libc::O_CLOEXEC | flags) as u64,
            mode: 0,
            resolve: RESOLVE_IN_ROOT | RESOLVE_NO_MAGICLINKS,
        };
        // SAFETY: a NUL-terminated path and a correctly sized `open_how`.
        let fd = unsafe {
            libc::syscall(
                SYS_OPENAT2,
                self.root.as_raw_fd(),
                path.as_ptr(),
                &how as *const OpenHow,
                size_of::<OpenHow>(),
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: a freshly returned descriptor that nothing else owns.
        Ok(unsafe { std::os::fd::FromRawFd::from_raw_fd(fd as i32) })
    }

    fn resolve_walk(&self, path: &Path, mode: Mode) -> io::Result<Resolved> {
        let mut pending = components(path);
        let mut current = self.reopen_root()?;
        let mut symlinks = 0;
        loop {
            let Some(component) = pending.pop_front() else {
                // The path ended on a directory: `/`, `a/.`, `a/..`.
                return match mode {
                    Mode::Entry => Err(errno(libc::EBUSY)),
                    Mode::Object { .. } => Ok(Resolved::Object(current)),
                };
            };
            if component == "." {
                continue;
            }
            if component == ".." {
                if !self.is_root(current.as_fd())? {
                    current = open_path(current.as_fd(), OsStr::new(".."), libc::O_DIRECTORY)?;
                }
                continue;
            }

            let last = pending.is_empty();
            let name = sys::cstring(component.as_bytes())?;
            if last && matches!(mode, Mode::Entry) {
                return Ok(Resolved::Entry {
                    parent: current,
                    name,
                });
            }
            let next = match open_path(current.as_fd(), &component, libc::O_NOFOLLOW) {
                Ok(next) => next,
                Err(error) if last && error.raw_os_error() == Some(libc::ENOENT) => {
                    return Ok(Resolved::Entry {
                        parent: current,
                        name,
                    });
                }
                Err(error) => return Err(error),
            };
            let follow = !last || matches!(mode, Mode::Object { follow: true });
            let is_symlink = sys::fstat(next.as_fd())?.st_mode & libc::S_IFMT == libc::S_IFLNK;
            if !(is_symlink && follow) {
                if last {
                    return Ok(Resolved::Object(next));
                }
                current = next;
                continue;
            }

            symlinks += 1;
            if symlinks > MAX_SYMLINKS {
                return Err(errno(libc::ELOOP));
            }
            if is_procfs(current.as_fd())? {
                let directory = sys::fd_path(current.as_fd())?;
                if directory == Path::new("/proc") {
                    // Ordinary symlinks, some relative to `self`
                    // (`mounts -> self/mounts`). `self` itself is rewritten
                    // to the target, since the kernel would answer for us.
                    let target = match component.as_bytes() {
                        b"self" => OsString::from((self.tgid)()?.to_string()),
                        b"thread-self" => {
                            OsString::from(format!("{}/task/{}", (self.tgid)()?, self.tid))
                        }
                        _ => readlink(next.as_fd())?.into_os_string(),
                    };
                    prepend(&mut pending, Path::new(&target));
                    continue;
                }
                // Inside `/proc/<pid>`, symlinks are magic links (`cwd`,
                // `exe`, `fd/N`, ...) that jump straight to an object. Follow
                // them only for our own processes; the kernel would let us
                // (as the supervisor) reach other processes' objects too.
                if !proc_path_pid(&directory).is_some_and(|pid| self.tree.contains(pid)) {
                    return Err(errno(libc::EACCES));
                }
                let target = open_path(current.as_fd(), &component, 0)?;
                if last {
                    return Ok(Resolved::Object(target));
                }
                current = target;
                continue;
            }
            let target = readlink(next.as_fd())?;
            if target.is_absolute() {
                current = self.reopen_root()?;
            }
            prepend(&mut pending, &target);
        }
    }

    fn reopen_root(&self) -> io::Result<OwnedFd> {
        open_path(self.root, OsStr::new("."), libc::O_DIRECTORY)
    }

    fn is_root(&self, directory: BorrowedFd<'_>) -> io::Result<bool> {
        let directory = sys::fstat(directory)?;
        let root = sys::fstat(self.root)?;
        Ok((directory.st_dev, directory.st_ino) == (root.st_dev, root.st_ino))
    }
}

fn components(path: &Path) -> VecDeque<OsString> {
    let mut pending = VecDeque::new();
    prepend(&mut pending, path);
    pending
}

fn prepend(pending: &mut VecDeque<OsString>, path: &Path) {
    let names: Vec<OsString> = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(name) => Some(name.to_os_string()),
            Component::ParentDir => Some(OsString::from("..")),
            Component::RootDir | Component::CurDir | Component::Prefix(_) => None,
        })
        .collect();
    for name in names.into_iter().rev() {
        pending.push_front(name);
    }
}

fn open_path(directory: BorrowedFd<'_>, name: &OsStr, flags: i32) -> io::Result<OwnedFd> {
    sys::openat(
        directory.as_raw_fd(),
        &sys::cstring(name.as_bytes())?,
        libc::O_PATH | flags,
        0,
    )
}

/// The content of the symlink `link` (an `O_PATH | O_NOFOLLOW` fd).
fn readlink(link: BorrowedFd<'_>) -> io::Result<std::path::PathBuf> {
    let mut buffer = vec![0u8; libc::PATH_MAX as usize];
    // SAFETY: an empty path with a symlink fd reads that symlink; the buffer
    // is valid for its length.
    let length = unsafe {
        libc::readlinkat(
            link.as_raw_fd(),
            c"".as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer.len(),
        )
    };
    if length < 0 {
        return Err(io::Error::last_os_error());
    }
    buffer.truncate(length as usize);
    Ok(OsString::from(OsStr::from_bytes(&buffer)).into())
}

fn is_procfs(directory: BorrowedFd<'_>) -> io::Result<bool> {
    // SAFETY: plain old data, filled in by the kernel.
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: valid fd and pointer.
    sys::cvt(unsafe { libc::fstatfs(directory.as_raw_fd(), &mut stat) })?;
    Ok(stat.f_type as i64 == PROC_SUPER_MAGIC)
}
