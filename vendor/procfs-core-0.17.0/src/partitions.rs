use std::io::BufRead;

use super::ProcResult;
use std::str::FromStr;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A partition entry under `/proc/partitions`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_snake_case)]
pub struct PartitionEntry {
    /// Device major number
    pub major: u16,
    /// Device minor number
    pub minor: u16,
    /// Number of 1024 byte blocks
    pub blocks: u64,
    /// Device name
    pub name: String,
}

impl super::FromBufRead for Vec<PartitionEntry> {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut vec = Vec::new();

        for line in r.lines().skip(2) {
            let line = expect!(line);

            let mut s = line.split_whitespace();

            let major = expect!(u16::from_str(expect!(s.next())));
            let minor = expect!(u16::from_str(expect!(s.next())));
            let blocks = expect!(u64::from_str(expect!(s.next())));
            let name = expect!(s.next()).to_string();

            let partition_entry = PartitionEntry {
                major,
                minor,
                blocks,
                name,
            };

            vec.push(partition_entry);
        }

        Ok(vec)
    }
}

#[test]
fn test_partitions() {
    use crate::FromBufRead;
    use std::io::Cursor;

    let s = "major minor  #blocks  name

 259        0 1000204632 nvme0n1
 259        1    1048576 nvme0n1p1
 259        2    1048576 nvme0n1p2
 259        3  104857600 nvme0n1p3
 259        4  893248512 nvme0n1p4
 253        0  104841216 dm-0
 252        0    8388608 zram0
 253        1  893232128 dm-1
   8        0    3953664 sda
   8        1    2097152 sda1
   8        2    1855488 sda2
 253        2    1853440 dm-2
";

    let cursor = Cursor::new(s);
    let partitions = Vec::<PartitionEntry>::from_buf_read(cursor).unwrap();
    assert_eq!(partitions.len(), 12);

    assert_eq!(partitions[3].major, 259);
    assert_eq!(partitions[3].minor, 3);
    assert_eq!(partitions[3].blocks, 104857600);
    assert_eq!(partitions[3].name, "nvme0n1p3");

    assert_eq!(partitions[11].major, 253);
    assert_eq!(partitions[11].minor, 2);
    assert_eq!(partitions[11].blocks, 1853440);
    assert_eq!(partitions[11].name, "dm-2");
}
