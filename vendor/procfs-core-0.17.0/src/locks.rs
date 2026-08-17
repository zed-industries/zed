use crate::{expect, from_str, ProcResult};
use std::io::BufRead;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// The type of a file lock
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum LockType {
    /// A BSD file lock created using `flock`
    FLock,

    /// A POSIX byte-range lock created with `fcntl`
    Posix,

    /// An Open File Description (ODF) lock created with `fnctl`
    ODF,

    /// Some other unknown lock type
    Other(String),
}

impl LockType {
    pub fn as_str(&self) -> &str {
        match self {
            LockType::FLock => "FLOCK",
            LockType::Posix => "POSIX",
            LockType::ODF => "ODF",
            LockType::Other(s) => s.as_ref(),
        }
    }
}

impl From<&str> for LockType {
    fn from(s: &str) -> LockType {
        match s {
            "FLOCK" => LockType::FLock,
            "OFDLCK" => LockType::ODF,
            "POSIX" => LockType::Posix,
            x => LockType::Other(x.to_string()),
        }
    }
}

/// The mode of a lock (advisory or mandatory)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum LockMode {
    Advisory,
    Mandatory,

    /// Some other unknown lock mode
    Other(String),
}

impl LockMode {
    pub fn as_str(&self) -> &str {
        match self {
            LockMode::Advisory => "ADVISORY",
            LockMode::Mandatory => "MANDATORY",
            LockMode::Other(s) => s.as_ref(),
        }
    }
}

impl From<&str> for LockMode {
    fn from(s: &str) -> LockMode {
        match s {
            "ADVISORY" => LockMode::Advisory,
            "MANDATORY" => LockMode::Mandatory,
            x => LockMode::Other(x.to_string()),
        }
    }
}

/// The kind of a lock (read or write)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum LockKind {
    /// A read lock (or BSD shared lock)
    Read,

    /// A write lock (or a BSD exclusive lock)
    Write,

    /// Some other unknown lock kind
    Other(String),
}

impl LockKind {
    pub fn as_str(&self) -> &str {
        match self {
            LockKind::Read => "READ",
            LockKind::Write => "WRITE",
            LockKind::Other(s) => s.as_ref(),
        }
    }
}

impl From<&str> for LockKind {
    fn from(s: &str) -> LockKind {
        match s {
            "READ" => LockKind::Read,
            "WRITE" => LockKind::Write,
            x => LockKind::Other(x.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/// Details about an individual file lock
///
/// For an example, see the [lslocks.rs](https://github.com/eminence/procfs/tree/master/examples)
/// example in the source repo.
pub struct Lock {
    /// The type of lock
    pub lock_type: LockType,
    /// The lock mode (advisory or mandatory)
    pub mode: LockMode,
    /// The kind of lock (read or write)
    pub kind: LockKind,
    /// The process that owns the lock
    ///
    /// Because OFD locks are not owned by a single process (since multiple processes
    /// may have file descriptors that refer to the same FD), this field may be `None`.
    ///
    /// Before kernel 4.14 a bug meant that the PID of of the process that initially
    /// acquired the lock was displayed instead of `None`.
    pub pid: Option<i32>,
    /// The major ID of the device containing the FS that contains this lock
    pub devmaj: u32,
    /// The minor ID of the device containing the FS that contains this lock
    pub devmin: u32,
    /// The inode of the locked file
    pub inode: u64,
    /// The offset (in bytes) of the first byte of the lock.
    ///
    /// For BSD locks, this value is always 0.
    pub offset_first: u64,
    /// The offset (in bytes) of the last byte of the lock.
    ///
    /// `None` means the lock extends to the end of the file.  For BSD locks,
    /// the value is always `None`.
    pub offset_last: Option<u64>,
}

impl Lock {
    fn from_line(line: &str) -> ProcResult<Lock> {
        let mut s = line.split_whitespace();

        let _ = expect!(s.next());
        let typ = {
            let t = expect!(s.next());
            if t == "->" {
                // some locks start a "->" which apparently means they are "blocked" (but i'm not sure what that actually means)
                From::from(expect!(s.next()))
            } else {
                From::from(t)
            }
        };
        let mode = From::from(expect!(s.next()));
        let kind = From::from(expect!(s.next()));
        let pid = expect!(s.next());
        let disk_inode = expect!(s.next());
        let offset_first = from_str!(u64, expect!(s.next()));
        let offset_last = expect!(s.next());

        let mut dis = disk_inode.split(':');
        let devmaj = from_str!(u32, expect!(dis.next()), 16);
        let devmin = from_str!(u32, expect!(dis.next()), 16);
        let inode = from_str!(u64, expect!(dis.next()));

        Ok(Lock {
            lock_type: typ,
            mode,
            kind,
            pid: if pid == "-1" { None } else { Some(from_str!(i32, pid)) },
            devmaj,
            devmin,
            inode,
            offset_first,
            offset_last: if offset_last == "EOF" {
                None
            } else {
                Some(from_str!(u64, offset_last))
            },
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/// Details about file locks
pub struct Locks(pub Vec<Lock>);

impl super::FromBufRead for Locks {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut v = Vec::new();

        for line in r.lines() {
            let line = line?;
            v.push(Lock::from_line(&line)?);
        }
        Ok(Locks(v))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_blocked() {
        let data = r#"1: POSIX  ADVISORY  WRITE 723 00:14:16845 0 EOF
        2: FLOCK  ADVISORY  WRITE 652 00:14:16763 0 EOF
        3: FLOCK  ADVISORY  WRITE 1594 fd:00:396528 0 EOF
        4: FLOCK  ADVISORY  WRITE 1594 fd:00:396527 0 EOF
        5: FLOCK  ADVISORY  WRITE 2851 fd:00:529372 0 EOF
        6: POSIX  ADVISORY  WRITE 1280 00:14:16200 0 0
        6: -> POSIX  ADVISORY  WRITE 1281 00:14:16200 0 0
        6: -> POSIX  ADVISORY  WRITE 1279 00:14:16200 0 0
        6: -> POSIX  ADVISORY  WRITE 1282 00:14:16200 0 0
        6: -> POSIX  ADVISORY  WRITE 1283 00:14:16200 0 0
        7: OFDLCK ADVISORY  READ  -1 00:06:1028 0 EOF
        8: FLOCK  ADVISORY  WRITE 6471 fd:00:529426 0 EOF
        9: FLOCK  ADVISORY  WRITE 6471 fd:00:529424 0 EOF
        10: FLOCK  ADVISORY  WRITE 6471 fd:00:529420 0 EOF
        11: FLOCK  ADVISORY  WRITE 6471 fd:00:529418 0 EOF
        12: POSIX  ADVISORY  WRITE 1279 00:14:23553 0 EOF
        13: FLOCK  ADVISORY  WRITE 6471 fd:00:393838 0 EOF
        14: POSIX  ADVISORY  WRITE 655 00:14:16146 0 EOF"#;

        for line in data.lines() {
            super::Lock::from_line(line.trim()).unwrap();
        }
    }
}
