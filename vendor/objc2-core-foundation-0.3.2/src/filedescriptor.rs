#![cfg(unix)] // std::os::fd only available on unix platforms.
use std::os::fd::{AsRawFd, RawFd};

use crate::CFFileDescriptor;

impl AsRawFd for CFFileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        self.native_descriptor()
    }
}

// NOTE: We cannot implement `AsFd`, since if `CFFileDescriptor` was created
// with `closeOnInvalidate`, the user could close the file descriptor while
// the `BorrowedFd` was alive, thus breaking its invariant.
