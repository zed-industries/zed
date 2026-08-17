use std::io::BufRead;

use super::ProcResult;
use std::str::FromStr;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Device entries under `/proc/devices`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Devices {
    /// Character devices
    pub char_devices: Vec<CharDeviceEntry>,
    /// Block devices, which can be empty if the kernel doesn't support block devices (without `CONFIG_BLOCK`)
    pub block_devices: Vec<BlockDeviceEntry>,
}

/// A charcter device entry under `/proc/devices`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct CharDeviceEntry {
    /// Device major number
    pub major: u32,
    /// Device name
    pub name: String,
}

/// A block device entry under `/proc/devices`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct BlockDeviceEntry {
    /// Device major number
    pub major: i32,
    /// Device name
    pub name: String,
}

impl super::FromBufRead for Devices {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        enum State {
            Char,
            Block,
        }
        let mut state = State::Char; // Always start with char devices
        let mut devices = Devices {
            char_devices: vec![],
            block_devices: vec![],
        };

        for line in r.lines() {
            let line = expect!(line);

            if line.is_empty() {
                continue;
            } else if line.starts_with("Character devices:") {
                state = State::Char;
                continue;
            } else if line.starts_with("Block devices:") {
                state = State::Block;
                continue;
            }

            let mut s = line.split_whitespace();

            match state {
                State::Char => {
                    let major = expect!(u32::from_str(expect!(s.next())));
                    let name = expect!(s.next()).to_string();

                    let char_device_entry = CharDeviceEntry { major, name };

                    devices.char_devices.push(char_device_entry);
                }
                State::Block => {
                    let major = expect!(i32::from_str(expect!(s.next())));
                    let name = expect!(s.next()).to_string();

                    let block_device_entry = BlockDeviceEntry { major, name };

                    devices.block_devices.push(block_device_entry);
                }
            }
        }

        Ok(devices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_devices() {
        use crate::FromBufRead;
        use std::io::Cursor;

        let s = "Character devices:
  1 mem
  4 /dev/vc/0
  4 tty
  4 ttyS
  5 /dev/tty
  5 /dev/console
  5 /dev/ptmx
  7 vcs
 10 misc
 13 input
 29 fb
 90 mtd
136 pts
180 usb
188 ttyUSB
189 usb_device

Block devices:
  7 loop
  8 sd
 65 sd
 71 sd
128 sd
135 sd
254 device-mapper
259 blkext
";

        let cursor = Cursor::new(s);
        let devices = Devices::from_buf_read(cursor).unwrap();
        let (chrs, blks) = (devices.char_devices, devices.block_devices);

        assert_eq!(chrs.len(), 16);

        assert_eq!(chrs[1].major, 4);
        assert_eq!(chrs[1].name, "/dev/vc/0");

        assert_eq!(chrs[8].major, 10);
        assert_eq!(chrs[8].name, "misc");

        assert_eq!(chrs[15].major, 189);
        assert_eq!(chrs[15].name, "usb_device");

        assert_eq!(blks.len(), 8);

        assert_eq!(blks[0].major, 7);
        assert_eq!(blks[0].name, "loop");

        assert_eq!(blks[7].major, 259);
        assert_eq!(blks[7].name, "blkext");
    }

    #[test]
    fn test_devices_without_block() {
        use crate::FromBufRead;
        use std::io::Cursor;

        let s = "Character devices:
  1 mem
  4 /dev/vc/0
  4 tty
  4 ttyS
  5 /dev/tty
  5 /dev/console
  5 /dev/ptmx
  7 vcs
 10 misc
 13 input
 29 fb
 90 mtd
136 pts
180 usb
188 ttyUSB
189 usb_device
";

        let cursor = Cursor::new(s);
        let devices = Devices::from_buf_read(cursor).unwrap();
        let (chrs, blks) = (devices.char_devices, devices.block_devices);

        assert_eq!(chrs.len(), 16);

        assert_eq!(chrs[1].major, 4);
        assert_eq!(chrs[1].name, "/dev/vc/0");

        assert_eq!(chrs[8].major, 10);
        assert_eq!(chrs[8].name, "misc");

        assert_eq!(chrs[15].major, 189);
        assert_eq!(chrs[15].name, "usb_device");

        assert_eq!(blks.len(), 0);
    }
}
