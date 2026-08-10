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

// --- Impure half: GPUI polling entity and global. ---
// Everything below this point does real I/O (serial port enumeration,
// avrdude subprocess, sqlite via KeyValueStore) and depends on GPUI. The
// pure functions above stay independently testable; this section wires
// them together.

use crate::board_registry::lookup_chip;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global};
use std::time::Duration;
use util::ResultExt;
use util::command::new_command;

/// A board detected on a serial port: which port, its VID:PID, the
/// user-assigned identity (if previously picked/stored via `board_picker`),
/// and — once the chip signature read completes — whether the chip is a
/// verified/known part and which `mmcu` value to pass to avrdude/avr-gcc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedBoard {
    pub port_name: String,
    pub vid_pid: VidPid,
    pub identity: BoardIdentity,
    pub chip_verified: Option<bool>,
    pub mmcu: Option<&'static str>,
}

/// Polls `serialport::available_ports()` every 2 seconds off the main
/// thread and identifies newly connected boards by reading their chip
/// signature via avrdude. Exposed as `GlobalBoardMonitor` so both the
/// status bar indicator and the build/flash action can read `detected`.
pub struct BoardMonitor {
    pub detected: Option<DetectedBoard>,
    known_ports: HashSet<String>,
}

pub struct GlobalBoardMonitor(pub Entity<BoardMonitor>);

impl Global for GlobalBoardMonitor {}

/// Emitted once per device (gated by the kvp-recorded warning flag) when a
/// connected board's chip signature doesn't match a verified part.
/// `BoardMonitor` has no `Workspace` handle to show a toast directly, so it
/// emits this event instead; the toast display lives in `citadel_build.rs`.
pub struct UnverifiedChipDetected;

impl EventEmitter<UnverifiedChipDetected> for BoardMonitor {}

/// Emitted when `begin_identify`'s chip-signature read fails (e.g. the port
/// is claimed by another process, or the board doesn't speak the expected
/// bootloader protocol). `BoardMonitor` has no `Workspace` handle to show a
/// toast directly, so it emits this event instead; the toast display lives
/// in `citadel_build.rs`. A port is only probed once per physical connect
/// event (see `apply_poll`'s `newly_connected_ports` diff), so this is the
/// only signal the user gets that the probe failed.
pub struct SignatureReadFailed {
    pub port_name: String,
}

impl EventEmitter<SignatureReadFailed> for BoardMonitor {}

pub fn init(cx: &mut App) {
    let board_monitor = cx.new(BoardMonitor::new);
    cx.set_global(GlobalBoardMonitor(board_monitor));
}

impl BoardMonitor {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| -> anyhow::Result<()> {
            loop {
                let ports = cx
                    .background_spawn(async { serialport::available_ports().unwrap_or_default() })
                    .await;
                this.update(cx, |this, cx| this.apply_poll(ports, cx))?;
                cx.background_executor().timer(Duration::from_secs(2)).await;
            }
        })
        .detach();

        Self {
            detected: None,
            known_ports: HashSet::new(),
        }
    }

    fn apply_poll(&mut self, ports: Vec<SerialPortInfo>, cx: &mut Context<Self>) {
        let current_port_names: HashSet<String> =
            ports.iter().map(|port| port.port_name.clone()).collect();

        for port in newly_connected_ports(&self.known_ports, &ports) {
            if let Some(vid_pid) = vid_pid_of(port) {
                self.begin_identify(port.port_name.clone(), vid_pid, cx);
            }
        }

        if let Some(detected) = &self.detected {
            if !current_port_names.contains(&detected.port_name) {
                self.detected = None;
            }
        }

        self.known_ports = current_port_names;
        cx.notify();
    }

    fn begin_identify(&mut self, port_name: String, vid_pid: VidPid, cx: &mut Context<Self>) {
        let stored_name = KeyValueStore::global(cx)
            .read_kvp(&board_kvp_key(vid_pid))
            .log_err()
            .flatten();
        let identity = resolve_board_identity(stored_name);

        cx.spawn(async move |this, cx| {
            let signature_port_name = port_name.clone();
            let signature_result = cx
                .background_spawn(async move { read_chip_signature(&signature_port_name).await })
                .await;
            let signature_read_failed = signature_result.is_err();
            let signature = signature_result.log_err();

            this.update(cx, move |this, cx| {
                let chip = signature.and_then(lookup_chip);
                let chip_verified = chip.map(|chip| chip.verified);
                let mmcu = chip.map(|chip| chip.mmcu);

                this.detected = Some(DetectedBoard {
                    port_name: port_name.clone(),
                    vid_pid,
                    identity,
                    chip_verified,
                    mmcu,
                });

                if let Some(verified) = chip_verified {
                    this.maybe_warn_unverified_chip(vid_pid, verified, cx);
                }

                if signature_read_failed {
                    cx.emit(SignatureReadFailed { port_name });
                }

                cx.notify();
            })
        })
        .detach();
    }

    fn maybe_warn_unverified_chip(
        &mut self,
        vid_pid: VidPid,
        verified: bool,
        cx: &mut Context<Self>,
    ) {
        let key = warning_kvp_key(vid_pid);
        let already_warned = KeyValueStore::global(cx)
            .read_kvp(&key)
            .log_err()
            .flatten()
            .is_some();

        if decide_unverified_chip_warning(verified, already_warned) == WarningDecision::ShowAndRecord
        {
            let kvp = KeyValueStore::global(cx);
            db::write_and_log(cx, move || async move {
                kvp.write_kvp(key, "1".to_string()).await
            });
            cx.emit(UnverifiedChipDetected);
        }
    }
}

/// Shells `avrdude -c arduino -p m328p -P <port> -b 115200 -F -U
/// signature:r:-:i` and parses the 3-byte chip signature from its
/// intel-hex output.
async fn read_chip_signature(port_name: &str) -> anyhow::Result<[u8; 3]> {
    let mut command = new_command("avrdude");
    command.args([
        "-c",
        "arduino",
        "-p",
        "m328p",
        "-P",
        port_name,
        "-b",
        "115200",
        "-F",
        "-U",
        "signature:r:-:i",
    ]);
    let output = command.output().await?;
    parse_signature_from_avrdude_ihex(&output.stdout)
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
