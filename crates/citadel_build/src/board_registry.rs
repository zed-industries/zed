/// AVR chip signature and board-family registry.
/// Pure, static lookup tables with no I/O and no GPUI dependency.

pub struct ChipInfo {
    pub signature: [u8; 3],
    pub mmcu: &'static str,
    pub display_name: &'static str,
    pub verified: bool,
}

pub static KNOWN_CHIPS: &[ChipInfo] = &[
    ChipInfo {
        signature: [0x1E, 0x95, 0x0F],
        mmcu: "atmega328p",
        display_name: "ATmega328P",
        verified: true,
    },
    ChipInfo {
        signature: [0x1E, 0x95, 0x16],
        mmcu: "atmega328pb",
        display_name: "ATmega328PB",
        verified: false,
    },
    ChipInfo {
        signature: [0x1E, 0x95, 0x14],
        mmcu: "atmega328",
        display_name: "ATmega328",
        verified: false,
    },
    ChipInfo {
        signature: [0x1E, 0x98, 0x01],
        mmcu: "atmega2560",
        display_name: "ATmega2560",
        verified: false,
    },
    ChipInfo {
        signature: [0x1E, 0x95, 0x87],
        mmcu: "atmega32u4",
        display_name: "ATmega32U4",
        verified: false,
    },
];

pub fn lookup_chip(signature: [u8; 3]) -> Option<&'static ChipInfo> {
    KNOWN_CHIPS.iter().find(|chip| chip.signature == signature)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardKind {
    Uno,
    Nano,
    ProMini,
    Other,
}

pub fn board_kind_display_name(kind: BoardKind) -> &'static str {
    match kind {
        BoardKind::Uno => "Arduino Uno",
        BoardKind::Nano => "Arduino Nano",
        BoardKind::ProMini => "Arduino Pro Mini",
        BoardKind::Other => "Other",
    }
}

/// Reverse of `board_kind_display_name`: resolve a `BoardKind` from the
/// display name stored via `board_picker`'s kvp write. Returns `None` for
/// any name that isn't one of the exact strings `board_kind_display_name`
/// produces.
pub fn board_kind_from_display_name(name: &str) -> Option<BoardKind> {
    [
        BoardKind::Uno,
        BoardKind::Nano,
        BoardKind::ProMini,
        BoardKind::Other,
    ]
    .into_iter()
    .find(|kind| board_kind_display_name(*kind) == name)
}

pub fn avrdude_defaults(kind: BoardKind) -> (&'static str, u32) {
    let programmer = "arduino";
    // ponytail: baud rates are heuristic defaults (Nano/ProMini=57600, Uno/Other=115200 --
    // bootloaders commonly differ this way); upgrade path if a picked family's default is
    // wrong for a specific board is a per-VID:PID baud override, not a bigger abstraction here.
    let baud = match kind {
        BoardKind::Uno => 115200,
        BoardKind::Nano => 57600,
        BoardKind::ProMini => 57600,
        BoardKind::Other => 115200,
    };
    (programmer, baud)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_chip_atmega328p() {
        let result = lookup_chip([0x1E, 0x95, 0x0F]);
        assert!(result.is_some());
        let chip = result.unwrap();
        assert_eq!(chip.mmcu, "atmega328p");
        assert_eq!(chip.verified, true);
    }

    #[test]
    fn test_lookup_chip_atmega328pb() {
        let result = lookup_chip([0x1E, 0x95, 0x16]);
        assert!(result.is_some());
        let chip = result.unwrap();
        assert_eq!(chip.mmcu, "atmega328pb");
        assert_eq!(chip.verified, false);
    }

    #[test]
    fn test_lookup_chip_unrecognized() {
        let result = lookup_chip([0xFF, 0xFF, 0xFF]);
        assert!(result.is_none());
    }

    #[test]
    fn test_board_kind_display_name() {
        assert_eq!(board_kind_display_name(BoardKind::Uno), "Arduino Uno");
        assert_eq!(board_kind_display_name(BoardKind::Nano), "Arduino Nano");
        assert_eq!(
            board_kind_display_name(BoardKind::ProMini),
            "Arduino Pro Mini"
        );
        assert_eq!(board_kind_display_name(BoardKind::Other), "Other");
    }

    #[test]
    fn test_avrdude_defaults_programmer() {
        assert_eq!(avrdude_defaults(BoardKind::Uno).0, "arduino");
        assert_eq!(avrdude_defaults(BoardKind::Nano).0, "arduino");
        assert_eq!(avrdude_defaults(BoardKind::ProMini).0, "arduino");
        assert_eq!(avrdude_defaults(BoardKind::Other).0, "arduino");
    }

    #[test]
    fn test_avrdude_defaults_uno_baud() {
        assert_eq!(avrdude_defaults(BoardKind::Uno).1, 115200);
    }

    #[test]
    fn test_avrdude_defaults_nano_promini_baud() {
        assert_eq!(avrdude_defaults(BoardKind::Nano).1, 57600);
        assert_eq!(avrdude_defaults(BoardKind::ProMini).1, 57600);
    }

    #[test]
    fn test_board_kind_display_name_round_trip() {
        for kind in [
            BoardKind::Uno,
            BoardKind::Nano,
            BoardKind::ProMini,
            BoardKind::Other,
        ] {
            let name = board_kind_display_name(kind);
            assert_eq!(board_kind_from_display_name(name), Some(kind));
        }
    }

    #[test]
    fn test_board_kind_from_display_name_unrecognized() {
        assert_eq!(board_kind_from_display_name("Raspberry Pi Pico"), None);
    }
}
