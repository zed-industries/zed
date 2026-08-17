use std::{collections::HashMap, io::BufRead};

use super::ProcResult;
use std::str::FromStr;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A mountpoint entry under `/proc/mounts`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_snake_case)]
pub struct MountEntry {
    /// Device
    pub fs_spec: String,
    /// Mountpoint
    pub fs_file: String,
    /// FS type
    pub fs_vfstype: String,
    /// Mount options
    pub fs_mntops: HashMap<String, Option<String>>,
    /// Dump
    pub fs_freq: u8,
    /// Check
    pub fs_passno: u8,
}

impl super::FromBufRead for Vec<MountEntry> {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut vec = Vec::new();

        for line in r.lines() {
            let line = expect!(line);
            let mut s = line.split_whitespace();

            let fs_spec = unmangle_octal(expect!(s.next()));
            let fs_file = unmangle_octal(expect!(s.next()));
            let fs_vfstype = unmangle_octal(expect!(s.next()));
            let fs_mntops = unmangle_octal(expect!(s.next()));
            let fs_mntops: HashMap<String, Option<String>> = fs_mntops
                .split(',')
                .map(|s| {
                    let mut split = s.splitn(2, '=');
                    let k = split.next().unwrap().to_string(); // can not fail, splitn will always return at least 1 element
                    let v = split.next().map(|s| s.to_string());

                    (k, v)
                })
                .collect();
            let fs_freq = expect!(u8::from_str(expect!(s.next())));
            let fs_passno = expect!(u8::from_str(expect!(s.next())));

            let mount_entry = MountEntry {
                fs_spec,
                fs_file,
                fs_vfstype,
                fs_mntops,
                fs_freq,
                fs_passno,
            };

            vec.push(mount_entry);
        }

        Ok(vec)
    }
}

/// Unmangle spaces ' ', tabs '\t', line breaks '\n', backslashes '\\', and hashes '#'
///
/// See https://elixir.bootlin.com/linux/v6.2.8/source/fs/proc_namespace.c#L89
pub(crate) fn unmangle_octal(input: &str) -> String {
    let mut input = input.to_string();

    for (octal, c) in [(r"\011", "\t"), (r"\012", "\n"), (r"\134", "\\"), (r"\043", "#")] {
        input = input.replace(octal, c);
    }

    input
}

#[test]
fn test_unmangle_octal() {
    let tests = [
        (r"a\134b\011c\012d\043e", "a\\b\tc\nd#e"), // all escaped chars with abcde in between
        (r"abcd", r"abcd"),                         // do nothing
    ];

    for (input, expected) in tests {
        assert_eq!(unmangle_octal(input), expected);
    }
}

#[test]
fn test_mounts() {
    use crate::FromBufRead;
    use std::io::Cursor;

    let s = "proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
/dev/mapper/ol-root / xfs rw,relatime,attr2,inode64,logbufs=8,logbsize=32k,noquota 0 0
Downloads /media/sf_downloads vboxsf rw,nodev,relatime,iocharset=utf8,uid=0,gid=977,dmode=0770,fmode=0770,tag=VBoxAutomounter 0 0";

    let cursor = Cursor::new(s);
    let mounts = Vec::<MountEntry>::from_buf_read(cursor).unwrap();
    assert_eq!(mounts.len(), 4);
}
