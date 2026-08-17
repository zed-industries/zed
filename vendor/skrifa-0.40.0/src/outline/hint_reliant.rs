//! Name detection for fonts that require hinting to be run for correct
//! contours (FreeType calls these "tricky" fonts).

use crate::{string::StringId, FontRef, MetadataProvider, Tag};

pub(super) fn require_interpreter(font: &FontRef) -> bool {
    is_hint_reliant_by_name(font) || matches_hint_reliant_id_list(FontId::from_font(font))
}

fn is_hint_reliant_by_name(font: &FontRef) -> bool {
    font.localized_strings(StringId::FAMILY_NAME)
        .english_or_first()
        .map(|name| {
            let mut buf = [0u8; MAX_HINT_RELIANT_NAME_LEN];
            let mut len = 0;
            let mut chars = name.chars();
            for ch in chars.by_ref().take(MAX_HINT_RELIANT_NAME_LEN) {
                buf[len] = ch as u8;
                len += 1;
            }
            if chars.next().is_some() {
                return false;
            }
            matches_hint_reliant_name_list(core::str::from_utf8(&buf[..len]).unwrap_or_default())
        })
        .unwrap_or_default()
}

/// Is this name on the list of fonts that require hinting?
///
/// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L174>
fn matches_hint_reliant_name_list(name: &str) -> bool {
    let name = skip_pdf_random_tag(name);
    HINT_RELIANT_NAMES
        .iter()
        // FreeType uses strstr(name, tricky_name) so we use contains() to
        // match behavior.
        .any(|tricky_name| name.contains(*tricky_name))
}

/// Fonts embedded in PDFs add random prefixes. Strip these
/// for tricky font comparison purposes.
///
/// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L153>
fn skip_pdf_random_tag(name: &str) -> &str {
    let bytes = name.as_bytes();
    // Random tag is 6 uppercase letters followed by a +
    if bytes.len() < 8 || bytes[6] != b'+' || !bytes.iter().take(6).all(|b| b.is_ascii_uppercase())
    {
        return name;
    }
    core::str::from_utf8(&bytes[7..]).unwrap_or(name)
}

/// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L180>
#[rustfmt::skip]
const HINT_RELIANT_NAMES: &[&str] = &[
    "cpop",               /* dftt-p7.ttf; version 1.00, 1992 [DLJGyShoMedium] */
    "DFGirl-W6-WIN-BF",   /* dftt-h6.ttf; version 1.00, 1993 */
    "DFGothic-EB",        /* DynaLab Inc. 1992-1995 */
    "DFGyoSho-Lt",        /* DynaLab Inc. 1992-1995 */
    "DFHei",              /* DynaLab Inc. 1992-1995 [DFHei-Bd-WIN-HK-BF] */
                          /* covers "DFHei-Md-HK-BF", maybe DynaLab Inc. */

    "DFHSGothic-W5",      /* DynaLab Inc. 1992-1995 */
    "DFHSMincho-W3",      /* DynaLab Inc. 1992-1995 */
    "DFHSMincho-W7",      /* DynaLab Inc. 1992-1995 */
    "DFKaiSho-SB",        /* dfkaisb.ttf */
    "DFKaiShu",           /* covers "DFKaiShu-Md-HK-BF", maybe DynaLab Inc. */
    "DFKai-SB",           /* kaiu.ttf; version 3.00, 1998 [DFKaiShu-SB-Estd-BF] */

    "DFMing",             /* DynaLab Inc. 1992-1995 [DFMing-Md-WIN-HK-BF] */
                          /* covers "DFMing-Bd-HK-BF", maybe DynaLab Inc. */

    "DLC",                /* dftt-m7.ttf; version 1.00, 1993 [DLCMingBold] */
                          /* dftt-f5.ttf; version 1.00, 1993 [DLCFongSung] */
                          /* covers following */
                          /* "DLCHayMedium", dftt-b5.ttf; version 1.00, 1993 */
                          /* "DLCHayBold",   dftt-b7.ttf; version 1.00, 1993 */
                          /* "DLCKaiMedium", dftt-k5.ttf; version 1.00, 1992 */
                          /* "DLCLiShu",     dftt-l5.ttf; version 1.00, 1992 */
                          /* "DLCRoundBold", dftt-r7.ttf; version 1.00, 1993 */

    "HuaTianKaiTi?",      /* htkt2.ttf */
    "HuaTianSongTi?",     /* htst3.ttf */
    "Ming(for ISO10646)", /* hkscsiic.ttf; version 0.12, 2007 [Ming] */
                          /* iicore.ttf; version 0.07, 2007 [Ming] */
    "MingLiU",            /* mingliu.ttf */
                          /* mingliu.ttc; version 3.21, 2001 */
    "MingMedium",         /* dftt-m5.ttf; version 1.00, 1993 [DLCMingMedium] */
    "PMingLiU",           /* mingliu.ttc; version 3.21, 2001 */
    "MingLi43",           /* mingli.ttf; version 1.00, 1992 */
];

const MAX_HINT_RELIANT_NAME_LEN: usize = 18;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
struct TableId {
    checksum: u32,
    len: u32,
}

impl TableId {
    fn from_font_and_tag(font: &FontRef, tag: Tag) -> Option<Self> {
        let data = font.table_data(tag)?;
        Some(Self {
            // Note: FreeType always just computes the checksum
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L281>
            checksum: raw::tables::compute_checksum(data.as_bytes()),
            len: data.len() as u32,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
struct FontId {
    cvt: TableId,
    fpgm: TableId,
    prep: TableId,
}

impl FontId {
    fn from_font(font: &FontRef) -> Self {
        Self {
            cvt: TableId::from_font_and_tag(font, Tag::new(b"cvt ")).unwrap_or_default(),
            fpgm: TableId::from_font_and_tag(font, Tag::new(b"fpgm")).unwrap_or_default(),
            prep: TableId::from_font_and_tag(font, Tag::new(b"prep")).unwrap_or_default(),
        }
    }
}

/// Checks for fonts that require hinting based on the length and checksum of
/// the cvt, fpgm and prep tables.
///
/// Roughly equivalent to <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L309>
fn matches_hint_reliant_id_list(font_id: FontId) -> bool {
    HINT_RELIANT_IDS.contains(&font_id)
}

/// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/truetype/ttobjs.c#L314>
#[rustfmt::skip]
const HINT_RELIANT_IDS: &[FontId] = &[
    // MingLiU 1995
    FontId {
        cvt: TableId { checksum: 0x05BCF058, len: 0x000002E4 },
        fpgm: TableId { checksum: 0x28233BF1, len: 0x000087C4 },
        prep: TableId { checksum: 0xA344A1EA, len: 0x000001E1 },
    },
    // MingLiU 1996-
    FontId {
        cvt: TableId { checksum: 0x05BCF058, len: 0x000002E4 },
        fpgm: TableId { checksum: 0x28233BF1, len: 0x000087C4 },
        prep: TableId { checksum: 0xA344A1EB, len: 0x000001E1 },
    },
    // DFGothic-EB
    FontId {
        cvt: TableId { checksum: 0x12C3EBB2, len: 0x00000350 },
        fpgm: TableId { checksum: 0xB680EE64, len: 0x000087A7 },
        prep: TableId { checksum: 0xCE939563, len: 0x00000758 },
    },
    // DFGyoSho-Lt
    FontId {
        cvt: TableId { checksum: 0x11E5EAD4, len: 0x00000350 },
        fpgm: TableId { checksum: 0xCE5956E9, len: 0x0000BC85 },
        prep: TableId { checksum: 0x8272F416, len: 0x00000045 },
    },
    // DFHei-Md-HK-BF
    FontId {
        cvt: TableId { checksum: 0x1257EB46, len: 0x00000350 },
        fpgm: TableId { checksum: 0xF699D160, len: 0x0000715F },
        prep: TableId { checksum: 0xD222F568, len: 0x000003BC },
    },
    // DFHSGothic-W5
    FontId {
        cvt: TableId { checksum: 0x1262EB4E, len: 0x00000350 },
        fpgm: TableId { checksum: 0xE86A5D64, len: 0x00007940 },
        prep: TableId { checksum: 0x7850F729, len: 0x000005FF },
    },
    // DFHSMincho-W3
    FontId {
        cvt: TableId { checksum: 0x122DEB0A, len: 0x00000350 },
        fpgm: TableId { checksum: 0x3D16328A, len: 0x0000859B },
        prep: TableId { checksum: 0xA93FC33B, len: 0x000002CB },
    },
    // DFHSMincho-W7
    FontId {
        cvt: TableId { checksum: 0x125FEB26, len: 0x00000350 },
        fpgm: TableId { checksum: 0xA5ACC982, len: 0x00007EE1 },
        prep: TableId { checksum: 0x90999196, len: 0x0000041F },
    },
    // DFKaiShu
    FontId {
        cvt: TableId { checksum: 0x11E5EAD4, len: 0x00000350 },
        fpgm: TableId { checksum: 0x5A30CA3B, len: 0x00009063 },
        prep: TableId { checksum: 0x13A42602, len: 0x0000007E },
    },
    // DFKaiShu, variant
    FontId {
        cvt: TableId { checksum: 0x11E5EAD4, len: 0x00000350 },
        fpgm: TableId { checksum: 0xA6E78C01, len: 0x00008998 },
        prep: TableId { checksum: 0x13A42602, len: 0x0000007E },
    },
    // DFKaiShu-Md-HK-BF
    FontId {
        cvt: TableId { checksum: 0x11E5EAD4, len: 0x00000360 },
        fpgm: TableId { checksum: 0x9DB282B2, len: 0x0000C06E },
        prep: TableId { checksum: 0x53E6D7CA, len: 0x00000082 },
    },
    // DFMing-Bd-HK-BF
    FontId {
        cvt: TableId { checksum: 0x1243EB18, len: 0x00000350 },
        fpgm: TableId { checksum: 0xBA0A8C30, len: 0x000074AD },
        prep: TableId { checksum: 0xF3D83409, len: 0x0000037B },
    },
    // DLCLiShu
    FontId {
        cvt: TableId { checksum: 0x07DCF546, len: 0x00000308 },
        fpgm: TableId { checksum: 0x40FE7C90, len: 0x00008E2A },
        prep: TableId { checksum: 0x608174B5, len: 0x0000007A },
    },
    // DLCHayBold
    FontId {
        cvt: TableId { checksum: 0xEB891238, len: 0x00000308 },
        fpgm: TableId { checksum: 0xD2E4DCD4, len: 0x0000676F },
        prep: TableId { checksum: 0x8EA5F293, len: 0x000003B8 },
    },
    // HuaTianKaiTi
    FontId {
        cvt: TableId { checksum: 0xFFFBFFFC, len: 0x00000008 },
        fpgm: TableId { checksum: 0x9C9E48B8, len: 0x0000BEA2 },
        prep: TableId { checksum: 0x70020112, len: 0x00000008 },
    },
    // HuaTianSongTi
    FontId {
        cvt: TableId { checksum: 0xFFFBFFFC, len: 0x00000008 },
        fpgm: TableId { checksum: 0x0A5A0483, len: 0x00017C39 },
        prep: TableId { checksum: 0x70020112, len: 0x00000008 },
    },
    // NEC fadpop7.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x40C92555, len: 0x000000E5 },
        prep: TableId { checksum: 0xA39B58E3, len: 0x0000117C },
    },
    // NEC fadrei5.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x33C41652, len: 0x000000E5 },
        prep: TableId { checksum: 0x26D6C52A, len: 0x00000F6A },
    },
    // NEC fangot7.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x6DB1651D, len: 0x0000019D },
        prep: TableId { checksum: 0x6C6E4B03, len: 0x00002492 },
    },
    // NEC fangyo5.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x40C92555, len: 0x000000E5 },
        prep: TableId { checksum: 0xDE51FAD0, len: 0x0000117C },
    },
    // NEC fankyo5.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x85E47664, len: 0x000000E5 },
        prep: TableId { checksum: 0xA6C62831, len: 0x00001CAA },
    },
    // NEC fanrgo5.ttf
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x2D891CFD, len: 0x0000019D },
        prep: TableId { checksum: 0xA0604633, len: 0x00001DE8 },
    },
    // NEC fangot5.ttc
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x40AA774C, len: 0x000001CB },
        prep: TableId { checksum: 0x9B5CAA96, len: 0x00001F9A },
    },
    // NEC fanmin3.ttc
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x0D3DE9CB, len: 0x00000141 },
        prep: TableId { checksum: 0xD4127766, len: 0x00002280 },
    },
    // NEC FA-Gothic, 1996
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x4A692698, len: 0x000001F0 },
        prep: TableId { checksum: 0x340D4346, len: 0x00001FCA },
    },
    // NEC FA-Minchou, 1996
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0xCD34C604, len: 0x00000166 },
        prep: TableId { checksum: 0x6CF31046, len: 0x000022B0 },
    },
    // NEC FA-RoundGothicB, 1996
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0x5DA75315, len: 0x0000019D },
        prep: TableId { checksum: 0x40745A5F, len: 0x000022E0 },
    },
    // NEC FA-RoundGothicM, 1996
    FontId {
        cvt: TableId { checksum: 0x00000000, len: 0x00000000 },
        fpgm: TableId { checksum: 0xF055FC48, len: 0x000001C2 },
        prep: TableId { checksum: 0x3900DED3, len: 0x00001E18 },
    },
    // MINGLI.TTF, 1992
    FontId {
        cvt: TableId { checksum: 0x00170003, len: 0x00000060 },
        fpgm: TableId { checksum: 0xDBB4306E, len: 0x000058AA },
        prep: TableId { checksum: 0xD643482A, len: 0x00000035 },
    },
    // DFHei-Bd-WIN-HK-BF, issue #1087
    FontId {
        cvt: TableId { checksum: 0x1269EB58, len: 0x00000350 },
        fpgm: TableId { checksum: 0x5CD5957A, len: 0x00006A4E },
        prep: TableId { checksum: 0xF758323A, len: 0x00000380 },
    },
    // DFMing-Md-WIN-HK-BF, issue #1087
    FontId {
        cvt: TableId { checksum: 0x122FEB0B, len: 0x00000350 },
        fpgm: TableId { checksum: 0x7F10919A, len: 0x000070A9 },
        prep: TableId { checksum: 0x7CD7E7B7, len: 0x0000025C },
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_max_name_len() {
        let max_len = HINT_RELIANT_NAMES
            .iter()
            .fold(0, |acc, name| acc.max(name.len()));
        assert_eq!(max_len, MAX_HINT_RELIANT_NAME_LEN);
    }

    #[test]
    fn skip_pdf_tags() {
        // length must be at least 8
        assert_eq!(skip_pdf_random_tag("ABCDEF+"), "ABCDEF+");
        // first six chars must be ascii uppercase
        assert_eq!(skip_pdf_random_tag("AbCdEF+Arial"), "AbCdEF+Arial");
        // no numbers
        assert_eq!(skip_pdf_random_tag("Ab12EF+Arial"), "Ab12EF+Arial");
        // missing +
        assert_eq!(skip_pdf_random_tag("ABCDEFArial"), "ABCDEFArial");
        // too long
        assert_eq!(skip_pdf_random_tag("ABCDEFG+Arial"), "ABCDEFG+Arial");
        // too short
        assert_eq!(skip_pdf_random_tag("ABCDE+Arial"), "ABCDE+Arial");
        // just right
        assert_eq!(skip_pdf_random_tag("ABCDEF+Arial"), "Arial");
    }

    #[test]
    fn all_hint_reliant_names() {
        for name in HINT_RELIANT_NAMES {
            assert!(matches_hint_reliant_name_list(name));
        }
    }

    #[test]
    fn non_hint_reliant_names() {
        for not_tricky in ["Roboto", "Arial", "Helvetica", "Blah", ""] {
            assert!(!matches_hint_reliant_name_list(not_tricky));
        }
    }
}
