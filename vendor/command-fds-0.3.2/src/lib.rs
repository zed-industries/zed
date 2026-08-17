// Copyright 2021, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A library for passing arbitrary file descriptors when spawning child processes.
//!
//! # Example
//!
//! ```rust
//! use command_fds::{CommandFdExt, FdMapping};
//! use std::fs::File;
//! use std::io::stdin;
//! use std::os::fd::AsFd;
//! use std::os::unix::io::AsRawFd;
//! use std::process::Command;
//!
//! // Open a file.
//! let file = File::open("Cargo.toml").unwrap();
//!
//! // Prepare to run `ls -l /proc/self/fd` with some FDs mapped.
//! let mut command = Command::new("ls");
//! command.arg("-l").arg("/proc/self/fd");
//! command
//!     .fd_mappings(vec![
//!         // Map `file` as FD 3 in the child process.
//!         FdMapping {
//!             parent_fd: file.into(),
//!             child_fd: 3,
//!         },
//!         // Map this process's stdin as FD 5 in the child process.
//!         FdMapping {
//!             parent_fd: stdin().as_fd().try_clone_to_owned().unwrap(),
//!             child_fd: 5,
//!         },
//!     ])
//!     .unwrap();
//!
//! // Spawn the child process.
//! let mut child = command.spawn().unwrap();
//! child.wait().unwrap();
//! ```

#[cfg(feature = "tokio")]
pub mod tokio;

use nix::fcntl::{FcntlArg, FdFlag, fcntl};
use nix::unistd::dup2_raw;
use std::cmp::max;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};
use std::os::unix::io::RawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;
use thiserror::Error;

/// A mapping from a file descriptor in the parent to a file descriptor in the child, to be applied
/// when spawning a child process.
///
/// This takes ownership of the `parent_fd` to ensure that it is kept open until after the child is
/// spawned.
#[derive(Debug)]
pub struct FdMapping {
    pub parent_fd: OwnedFd,
    pub child_fd: RawFd,
}

/// Error setting up FD mappings, because there were two or more mappings for the same child FD.
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
#[error("Two or more mappings for the same child FD")]
pub struct FdMappingCollision;

/// Extension to add file descriptor mappings to a [`Command`].
pub trait CommandFdExt {
    /// Adds the given set of file descriptors to the command.
    ///
    /// Warning: Calling this more than once on the same command may result in unexpected behaviour.
    /// In particular, it is not possible to check that two mappings applied separately don't use
    /// the same `child_fd`. If there is such a collision then one will apply and the other will be
    /// lost.
    ///
    /// Note that the `Command` takes ownership of the file descriptors, which means that they won't
    /// be closed in the parent process until the `Command` is dropped.
    fn fd_mappings(&mut self, mappings: Vec<FdMapping>) -> Result<&mut Self, FdMappingCollision>;

    /// Adds the given set of file descriptors to be passed on to the child process when the command
    /// is run.
    ///
    /// Note that the `Command` takes ownership of the file descriptors, which means that they won't
    /// be closed in the parent process until the `Command` is dropped.
    fn preserved_fds(&mut self, fds: Vec<OwnedFd>) -> &mut Self;
}

impl CommandFdExt for Command {
    fn fd_mappings(
        &mut self,
        mut mappings: Vec<FdMapping>,
    ) -> Result<&mut Self, FdMappingCollision> {
        let child_fds = validate_child_fds(&mappings)?;

        // Register the callback to apply the mappings after forking but before execing.
        // Safety: `map_fds` will not allocate, so it is safe to call from this hook.
        unsafe {
            // If the command is run more than once, the closure will be called multiple times but
            // in different forked processes, which will have different copies of `mappings`. So
            // their changes to it shouldn't be visible to each other.
            self.pre_exec(move || map_fds(&mut mappings, &child_fds));
        }

        Ok(self)
    }

    fn preserved_fds(&mut self, fds: Vec<OwnedFd>) -> &mut Self {
        unsafe {
            self.pre_exec(move || preserve_fds(&fds));
        }

        self
    }
}

/// Validates that there are no conflicting mappings to the same child FD.
fn validate_child_fds(mappings: &[FdMapping]) -> Result<Vec<RawFd>, FdMappingCollision> {
    let mut child_fds: Vec<RawFd> = mappings.iter().map(|mapping| mapping.child_fd).collect();
    child_fds.sort_unstable();
    child_fds.dedup();
    if child_fds.len() != mappings.len() {
        return Err(FdMappingCollision);
    }
    Ok(child_fds)
}

// This function must not do any allocation, as it is called from the pre_exec hook.
fn map_fds(mappings: &mut [FdMapping], child_fds: &[RawFd]) -> io::Result<()> {
    if mappings.is_empty() {
        // No need to do anything, and finding first_unused_fd would fail.
        return Ok(());
    }

    // Find the first FD which is higher than any parent or child FD in the mapping, so we can
    // safely use it and higher FDs as temporary FDs. There may be other files open with these FDs,
    // so we still need to ensure we don't conflict with them.
    let first_safe_fd = mappings
        .iter()
        .map(|mapping| max(mapping.parent_fd.as_raw_fd(), mapping.child_fd))
        .max()
        .unwrap()
        + 1;

    // If any parent FDs conflict with child FDs, then first duplicate them to a temporary FD which
    // is clear of either range. Mappings to the same FD are fine though, we can handle them by just
    // removing the FD_CLOEXEC flag from the existing (parent) FD.
    for mapping in mappings.iter_mut() {
        if child_fds.contains(&mapping.parent_fd.as_raw_fd())
            && mapping.parent_fd.as_raw_fd() != mapping.child_fd
        {
            let parent_fd = fcntl(&mapping.parent_fd, FcntlArg::F_DUPFD_CLOEXEC(first_safe_fd))?;
            // SAFETY: We just created `parent_fd` so we can take ownership of it.
            unsafe {
                mapping.parent_fd = OwnedFd::from_raw_fd(parent_fd);
            }
        }
    }

    // Now we can actually duplicate FDs to the desired child FDs.
    for mapping in mappings {
        if mapping.child_fd == mapping.parent_fd.as_raw_fd() {
            // Remove the FD_CLOEXEC flag, so the FD will be kept open when exec is called for the
            // child.
            fcntl(&mapping.parent_fd, FcntlArg::F_SETFD(FdFlag::empty()))?;
        } else {
            // This closes child_fd if it is already open as something else, and clears the
            // FD_CLOEXEC flag on child_fd.
            // SAFETY: `dup2_raw` returns an `OwnedFd` which takes ownership of the child FD and
            // would close it when it is dropped. We avoid this by calling `into_raw_fd` to give up
            // ownership again.
            unsafe {
                let _ = dup2_raw(&mapping.parent_fd, mapping.child_fd)?.into_raw_fd();
            }
        }
    }

    Ok(())
}

fn preserve_fds(fds: &[OwnedFd]) -> io::Result<()> {
    for fd in fds {
        // Remove the FD_CLOEXEC flag, so the FD will be kept open when exec is called for the
        // child.
        fcntl(fd, FcntlArg::F_SETFD(FdFlag::empty()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::close;
    use std::collections::HashSet;
    use std::fs::{File, read_dir};
    use std::os::unix::io::AsRawFd;
    use std::process::Output;
    use std::str;
    use std::sync::Once;

    static SETUP: Once = Once::new();

    #[test]
    fn conflicting_mappings() {
        setup();

        let mut command = Command::new("ls");

        let file1 = File::open("testdata/file1.txt").unwrap();
        let file2 = File::open("testdata/file2.txt").unwrap();

        // Mapping two different FDs to the same FD isn't allowed.
        assert!(
            command
                .fd_mappings(vec![
                    FdMapping {
                        child_fd: 4,
                        parent_fd: file1.into(),
                    },
                    FdMapping {
                        child_fd: 4,
                        parent_fd: file2.into(),
                    },
                ])
                .is_err()
        );
    }

    #[test]
    fn no_mappings() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        assert!(command.fd_mappings(vec![]).is_ok());

        let output = command.output().unwrap();
        expect_fds(&output, &[0, 1, 2, 3], 0);
    }

    #[test]
    fn none_preserved() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        command.preserved_fds(vec![]);

        let output = command.output().unwrap();
        expect_fds(&output, &[0, 1, 2, 3], 0);
    }

    #[test]
    fn one_mapping() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        let file = File::open("testdata/file1.txt").unwrap();
        // Map the file an otherwise unused FD.
        assert!(
            command
                .fd_mappings(vec![FdMapping {
                    parent_fd: file.into(),
                    child_fd: 5,
                },])
                .is_ok()
        );

        let output = command.output().unwrap();
        expect_fds(&output, &[0, 1, 2, 3, 5], 0);
    }

    #[test]
    #[ignore = "flaky on GitHub"]
    fn one_preserved() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        let file = File::open("testdata/file1.txt").unwrap();
        let file_fd: OwnedFd = file.into();
        let raw_file_fd = file_fd.as_raw_fd();
        assert!(raw_file_fd > 3);
        command.preserved_fds(vec![file_fd]);

        let output = command.output().unwrap();
        expect_fds(&output, &[0, 1, 2, 3, raw_file_fd], 0);
    }

    #[test]
    fn swap_mappings() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        let file1 = File::open("testdata/file1.txt").unwrap();
        let file2 = File::open("testdata/file2.txt").unwrap();
        let fd1: OwnedFd = file1.into();
        let fd2: OwnedFd = file2.into();
        let fd1_raw = fd1.as_raw_fd();
        let fd2_raw = fd2.as_raw_fd();
        // Map files to each other's FDs, to ensure that the temporary FD logic works.
        assert!(
            command
                .fd_mappings(vec![
                    FdMapping {
                        parent_fd: fd1,
                        child_fd: fd2_raw,
                    },
                    FdMapping {
                        parent_fd: fd2,
                        child_fd: fd1_raw,
                    },
                ])
                .is_ok(),
        );

        let output = command.output().unwrap();
        // Expect one more Fd for the /proc/self/fd directory. We can't predict what number it will
        // be assigned, because 3 might or might not be taken already by fd1 or fd2.
        expect_fds(&output, &[0, 1, 2, fd1_raw, fd2_raw], 1);
    }

    #[test]
    fn one_to_one_mapping() {
        setup();

        let mut command = Command::new("ls");
        command.arg("/proc/self/fd");

        let file1 = File::open("testdata/file1.txt").unwrap();
        let file2 = File::open("testdata/file2.txt").unwrap();
        let fd1: OwnedFd = file1.into();
        let fd1_raw = fd1.as_raw_fd();
        // Map file1 to the same FD it currently has, to ensure the special case for that works.
        assert!(
            command
                .fd_mappings(vec![FdMapping {
                    parent_fd: fd1,
                    child_fd: fd1_raw,
                }])
                .is_ok()
        );

        let output = command.output().unwrap();
        // Expect one more Fd for the /proc/self/fd directory. We can't predict what number it will
        // be assigned, because 3 might or might not be taken already by fd1 or fd2.
        expect_fds(&output, &[0, 1, 2, fd1_raw], 1);

        // Keep file2 open until the end, to ensure that it's not passed to the child.
        drop(file2);
    }

    #[test]
    fn map_stdin() {
        setup();

        let mut command = Command::new("cat");

        let file = File::open("testdata/file1.txt").unwrap();
        // Map the file to stdin.
        assert!(
            command
                .fd_mappings(vec![FdMapping {
                    parent_fd: file.into(),
                    child_fd: 0,
                },])
                .is_ok()
        );

        let output = command.output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"test 1");
    }

    /// Parse the output of ls into a set of filenames
    fn parse_ls_output(output: &[u8]) -> HashSet<String> {
        str::from_utf8(output)
            .unwrap()
            .split_terminator("\n")
            .map(str::to_owned)
            .collect()
    }

    /// Check that the output of `ls /proc/self/fd` contains the expected set of FDs, plus exactly
    /// `extra` extra FDs.
    fn expect_fds(output: &Output, expected_fds: &[RawFd], extra: usize) {
        assert!(output.status.success());
        let expected_fds: HashSet<String> = expected_fds.iter().map(RawFd::to_string).collect();
        let fds = parse_ls_output(&output.stdout);
        if extra == 0 {
            assert_eq!(fds, expected_fds);
        } else {
            assert!(expected_fds.is_subset(&fds));
            assert_eq!(fds.len(), expected_fds.len() + extra);
        }
    }

    fn setup() {
        SETUP.call_once(close_excess_fds);
    }

    /// Close all file descriptors apart from stdin, stdout and stderr.
    ///
    /// This is necessary because GitHub Actions opens a bunch of others for some reason.
    fn close_excess_fds() {
        let dir = read_dir("/proc/self/fd").unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let fd: RawFd = entry.file_name().to_str().unwrap().parse().unwrap();
            if fd > 3 {
                close(fd).unwrap();
            }
        }
    }
}
