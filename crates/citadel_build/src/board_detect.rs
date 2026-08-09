/// Pure resolution and parsing logic for USB board detection.
/// No I/O, no GPUI, no avrdude invocation — only pure logic to support
/// serial port diffing, board-identity resolution, warning decisions,
/// and parsing of avrdude's intel-hex signature-read output.

use serialport::{SerialPortInfo, SerialPortType};
use std::collections::HashSet;

pub type VidPid = (u16, u16);

/// Extract VID:PID from a USB serial port, or None for non-USB ports.
pub fn vid_pid_of(port: &SerialPortInfo) -> Option<VidPid> {
    match &port.port_type {
        SerialPortType::UsbPort(usb_info) => Some((usb_info.vid, usb_info.pid)),
        _ => None,
    }
}

/// Find ports that are new (present in `current` but not in `previous`).
/// Diffs by port name (not VID:PID — two identical adapters plugged in
/// at once share a VID:PID but get distinct OS names).
pub fn newly_connected_ports<'a>(
    previous: &HashSet<String>,
    current: &'a [SerialPortInfo],
) -> Vec<&'a SerialPortInfo> {
    current
        .iter()
        .filter(|port| !previous.contains(&port.port_name))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardIdentity {
    Known(String),
    Unknown,
}

/// Resolve board identity from a stored name.
/// Some(name) -> Known(name), None -> Unknown
pub fn resolve_board_identity(stored_name: Option<String>) -> BoardIdentity {
    match stored_name {
        Some(name) => BoardIdentity::Known(name),
        None => BoardIdentity::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningDecision {
    ShowAndRecord,
    Skip,
}

/// Decide whether to show the unverified-chip warning for this device.
/// Show (and record) only when: chip not verified AND have not warned before.
/// Skip otherwise.
pub fn decide_unverified_chip_warning(
    chip_verified: bool,
    already_warned: bool,
) -> WarningDecision {
    if !chip_verified && !already_warned {
        WarningDecision::ShowAndRecord
    } else {
        WarningDecision::Skip
    }
}

/// Parse a 3-byte chip signature from avrdude's intel-hex output.
/// Intel-hex format: `:LLAAAATT[DD...][CC]`
/// where LL=length (2 chars), AAAA=address (4 chars), TT=type (2 chars),
/// DD=data bytes, CC=checksum.
/// For signature data, we expect LL=03, AAAA=0000, and 3 data bytes at offset 9.
/// Example: `:030000001E950F3B` -> [0x1E, 0x95, 0x0F]
pub fn parse_signature_from_avrdude_ihex(stdout: &[u8]) -> anyhow::Result<[u8; 3]> {
    let text = std::str::from_utf8(stdout)?;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(':') && trimmed.len() >= 15 {
            // Character offset 9 = after ':' (1) + LLAAAATT (8 chars)
            let sig_hex = &trimmed[9..15];
            let mut sig = [0u8; 3];
            for (i, chunk) in sig_hex.as_bytes().chunks(2).enumerate() {
                if i >= 3 {
                    break;
                }
                let hex_str = std::str::from_utf8(chunk)?;
                sig[i] = u8::from_str_radix(hex_str, 16)?;
            }
            return Ok(sig);
        }
    }

    anyhow::bail!("No intel-hex record found in avrdude output")
}

/// Generate a stable KVP key for board detection cache.
/// Format: `citadel_build_board_XXXX_YYYY` where XXXX and YYYY are zero-padded hex.
/// Example: (0x2341, 0x43) -> "citadel_build_board_2341_0043"
pub fn board_kvp_key(vid_pid: VidPid) -> String {
    format!(
        "citadel_build_board_{:04x}_{:04x}",
        vid_pid.0, vid_pid.1
    )
}

/// Generate a stable KVP key for the unverified-chip warning cache.
/// Format: `citadel_build_warning_XXXX_YYYY` where XXXX and YYYY are zero-padded hex.
pub fn warning_kvp_key(vid_pid: VidPid) -> String {
    format!(
        "citadel_build_warning_{:04x}_{:04x}",
        vid_pid.0, vid_pid.1
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialport::UsbPortInfo;

    #[test]
    fn test_vid_pid_of_usb_port() {
        let port = SerialPortInfo {
            port_name: "/dev/ttyUSB0".to_string(),
            port_type: SerialPortType::UsbPort(UsbPortInfo {
                vid: 0x2341,
                pid: 0x0043,
                serial_number: Some("ABC123".to_string()),
                manufacturer: Some("Arduino".to_string()),
                product: Some("Arduino Uno".to_string()),
            }),
        };
        assert_eq!(vid_pid_of(&port), Some((0x2341, 0x0043)));
    }

    #[test]
    fn test_vid_pid_of_non_usb_port() {
        let port = SerialPortInfo {
            port_name: "/dev/ttyS0".to_string(),
            port_type: SerialPortType::PciPort,
        };
        assert_eq!(vid_pid_of(&port), None);
    }

    #[test]
    fn test_newly_connected_ports_excludes_known() {
        let port1 = SerialPortInfo {
            port_name: "/dev/ttyUSB0".to_string(),
            port_type: SerialPortType::PciPort,
        };
        let port2 = SerialPortInfo {
            port_name: "/dev/ttyUSB1".to_string(),
            port_type: SerialPortType::PciPort,
        };
        let previous = ["/dev/ttyUSB0".to_string()].iter().cloned().collect();
        let current = [port1, port2];

        let new = newly_connected_ports(&previous, &current);
        assert_eq!(new.len(), 1);
        assert_eq!(new[0].port_name, "/dev/ttyUSB1");
    }

    #[test]
    fn test_newly_connected_ports_empty_previous() {
        let port1 = SerialPortInfo {
            port_name: "/dev/ttyUSB0".to_string(),
            port_type: SerialPortType::PciPort,
        };
        let port2 = SerialPortInfo {
            port_name: "/dev/ttyUSB1".to_string(),
            port_type: SerialPortType::PciPort,
        };
        let previous = HashSet::new();
        let current = [port1, port2];

        let new = newly_connected_ports(&previous, &current);
        assert_eq!(new.len(), 2);
    }

    #[test]
    fn test_newly_connected_ports_all_known() {
        let port1 = SerialPortInfo {
            port_name: "/dev/ttyUSB0".to_string(),
            port_type: SerialPortType::PciPort,
        };
        let previous = ["/dev/ttyUSB0".to_string()].iter().cloned().collect();
        let current = [port1];

        let new = newly_connected_ports(&previous, &current);
        assert_eq!(new.len(), 0);
    }

    #[test]
    fn test_resolve_board_identity_known() {
        let identity = resolve_board_identity(Some("My Arduino Uno".to_string()));
        assert_eq!(identity, BoardIdentity::Known("My Arduino Uno".to_string()));
    }

    #[test]
    fn test_resolve_board_identity_unknown() {
        let identity = resolve_board_identity(None);
        assert_eq!(identity, BoardIdentity::Unknown);
    }

    #[test]
    fn test_decide_unverified_chip_warning_show() {
        let decision = decide_unverified_chip_warning(false, false);
        assert_eq!(decision, WarningDecision::ShowAndRecord);
    }

    #[test]
    fn test_decide_unverified_chip_warning_skip_already_warned() {
        let decision = decide_unverified_chip_warning(false, true);
        assert_eq!(decision, WarningDecision::Skip);
    }

    #[test]
    fn test_decide_unverified_chip_warning_skip_verified() {
        let decision = decide_unverified_chip_warning(true, false);
        assert_eq!(decision, WarningDecision::Skip);
    }

    #[test]
    fn test_decide_unverified_chip_warning_skip_both() {
        let decision = decide_unverified_chip_warning(true, true);
        assert_eq!(decision, WarningDecision::Skip);
    }

    #[test]
    fn test_parse_signature_from_avrdude_ihex_valid() {
        let output = b":030000001E950F3B\n:00000001FF\n";
        let sig = parse_signature_from_avrdude_ihex(output).expect("should parse");
        assert_eq!(sig, [0x1E, 0x95, 0x0F]);
    }

    #[test]
    fn test_parse_signature_from_avrdude_ihex_no_record() {
        let output = b"some random output\nwith no hex record\n";
        let result = parse_signature_from_avrdude_ihex(output);
        assert!(result.is_err());
    }

    #[test]
    fn test_board_kvp_key() {
        let key = board_kvp_key((0x2341, 0x0043));
        assert_eq!(key, "citadel_build_board_2341_0043");
    }

    #[test]
    fn test_board_kvp_key_small_values() {
        let key = board_kvp_key((0x0001, 0x0002));
        assert_eq!(key, "citadel_build_board_0001_0002");
    }

    #[test]
    fn test_warning_kvp_key() {
        let key = warning_kvp_key((0x2341, 0x0043));
        assert_eq!(key, "citadel_build_warning_2341_0043");
    }

    #[test]
    fn test_warning_kvp_key_small_values() {
        let key = warning_kvp_key((0x0001, 0x0002));
        assert_eq!(key, "citadel_build_warning_0001_0002");
    }
}
