use alloc::string::String;
use core::ops::{Bound, RangeBounds};

use ttf_parser::Tag;

use super::text_parser::TextParser;

pub type hb_codepoint_t = char; // uint32_t in C++

pub const HB_FEATURE_GLOBAL_START: u32 = 0;
pub const HB_FEATURE_GLOBAL_END: u32 = u32::MAX;

/// Defines the direction in which text is to be read.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    /// Initial, unset direction.
    Invalid,
    /// Text is set horizontally from left to right.
    LeftToRight,
    /// Text is set horizontally from right to left.
    RightToLeft,
    /// Text is set vertically from top to bottom.
    TopToBottom,
    /// Text is set vertically from bottom to top.
    BottomToTop,
}

impl Direction {
    #[inline]
    pub(crate) fn is_horizontal(self) -> bool {
        match self {
            Direction::Invalid => false,
            Direction::LeftToRight => true,
            Direction::RightToLeft => true,
            Direction::TopToBottom => false,
            Direction::BottomToTop => false,
        }
    }

    #[inline]
    pub(crate) fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }

    #[inline]
    pub(crate) fn is_forward(self) -> bool {
        match self {
            Direction::Invalid => false,
            Direction::LeftToRight => true,
            Direction::RightToLeft => false,
            Direction::TopToBottom => true,
            Direction::BottomToTop => false,
        }
    }

    #[inline]
    pub(crate) fn is_backward(self) -> bool {
        !self.is_forward()
    }

    #[inline]
    pub(crate) fn reverse(self) -> Self {
        match self {
            Direction::Invalid => Direction::Invalid,
            Direction::LeftToRight => Direction::RightToLeft,
            Direction::RightToLeft => Direction::LeftToRight,
            Direction::TopToBottom => Direction::BottomToTop,
            Direction::BottomToTop => Direction::TopToBottom,
        }
    }

    pub(crate) fn from_script(script: Script) -> Option<Self> {
        // https://docs.google.com/spreadsheets/d/1Y90M0Ie3MUJ6UVCRDOypOtijlMDLNNyyLk36T6iMu0o

        match script {
            // Unicode-1.1 additions
            script::ARABIC |
            script::HEBREW |

            // Unicode-3.0 additions
            script::SYRIAC |
            script::THAANA |

            // Unicode-4.0 additions
            script::CYPRIOT |

            // Unicode-4.1 additions
            script::KHAROSHTHI |

            // Unicode-5.0 additions
            script::PHOENICIAN |
            script::NKO |

            // Unicode-5.1 additions
            script::LYDIAN |

            // Unicode-5.2 additions
            script::AVESTAN |
            script::IMPERIAL_ARAMAIC |
            script::INSCRIPTIONAL_PAHLAVI |
            script::INSCRIPTIONAL_PARTHIAN |
            script::OLD_SOUTH_ARABIAN |
            script::OLD_TURKIC |
            script::SAMARITAN |

            // Unicode-6.0 additions
            script::MANDAIC |

            // Unicode-6.1 additions
            script::MEROITIC_CURSIVE |
            script::MEROITIC_HIEROGLYPHS |

            // Unicode-7.0 additions
            script::MANICHAEAN |
            script::MENDE_KIKAKUI |
            script::NABATAEAN |
            script::OLD_NORTH_ARABIAN |
            script::PALMYRENE |
            script::PSALTER_PAHLAVI |

            // Unicode-8.0 additions
            script::HATRAN |

            // Unicode-9.0 additions
            script::ADLAM |

            // Unicode-11.0 additions
            script::HANIFI_ROHINGYA |
            script::OLD_SOGDIAN |
            script::SOGDIAN |

            // Unicode-12.0 additions
            script::ELYMAIC |

            // Unicode-13.0 additions
            script::CHORASMIAN |
            script::YEZIDI |

            // Unicode-14.0 additions
            script::OLD_UYGHUR => {
                Some(Direction::RightToLeft)
            }

            // https://github.com/harfbuzz/harfbuzz/issues/1000
            script::OLD_HUNGARIAN |
            script::OLD_ITALIC |
            script::RUNIC |
            script::TIFINAGH => {
                None
            }

            _ => Some(Direction::LeftToRight),
        }
    }
}

impl Default for Direction {
    #[inline]
    fn default() -> Self {
        Direction::Invalid
    }
}

impl core::str::FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("invalid direction");
        }

        // harfbuzz also matches only the first letter.
        match s.as_bytes()[0].to_ascii_lowercase() {
            b'l' => Ok(Direction::LeftToRight),
            b'r' => Ok(Direction::RightToLeft),
            b't' => Ok(Direction::TopToBottom),
            b'b' => Ok(Direction::BottomToTop),
            _ => Err("invalid direction"),
        }
    }
}

/// A script language.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Language(String);

impl Language {
    /// Returns the language as a string.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl core::str::FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(Language(s.to_ascii_lowercase()))
        } else {
            Err("invalid language")
        }
    }
}

// In harfbuzz, despite having `hb_script_t`, script can actually have any tag.
// So we're doing the same.
// The only difference is that `Script` cannot be set to `HB_SCRIPT_INVALID`.
/// A text script.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Script(pub(crate) Tag);

impl Script {
    #[inline]
    pub(crate) const fn from_bytes(bytes: &[u8; 4]) -> Self {
        Script(Tag::from_bytes(bytes))
    }

    /// Converts an ISO 15924 script tag to a corresponding `Script`.
    pub fn from_iso15924_tag(tag: Tag) -> Option<Script> {
        if tag.is_null() {
            return None;
        }

        // Be lenient, adjust case (one capital letter followed by three small letters).
        let tag = Tag((tag.as_u32() & 0xDFDFDFDF) | 0x00202020);

        match &tag.to_bytes() {
            // These graduated from the 'Q' private-area codes, but
            // the old code is still aliased by Unicode, and the Qaai
            // one in use by ICU.
            b"Qaai" => return Some(script::INHERITED),
            b"Qaac" => return Some(script::COPTIC),

            // Script variants from https://unicode.org/iso15924/
            b"Aran" => return Some(script::ARABIC),
            b"Cyrs" => return Some(script::CYRILLIC),
            b"Geok" => return Some(script::GEORGIAN),
            b"Hans" | b"Hant" => return Some(script::HAN),
            b"Jamo" => return Some(script::HANGUL),
            b"Latf" | b"Latg" => return Some(script::LATIN),
            b"Syre" | b"Syrj" | b"Syrn" => return Some(script::SYRIAC),

            _ => {}
        }

        if tag.as_u32() & 0xE0E0E0E0 == 0x40606060 {
            Some(Script(tag))
        } else {
            Some(script::UNKNOWN)
        }
    }

    /// Returns script's tag.
    #[inline]
    pub fn tag(&self) -> Tag {
        self.0
    }
}

impl core::str::FromStr for Script {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = Tag::from_bytes_lossy(s.as_bytes());
        Script::from_iso15924_tag(tag).ok_or("invalid script")
    }
}

/// Predefined scripts.
pub mod script {
    #![allow(missing_docs)]

    use crate::Script;

    // Since 1.1
    pub const COMMON: Script = Script::from_bytes(b"Zyyy");
    pub const INHERITED: Script = Script::from_bytes(b"Zinh");
    pub const ARABIC: Script = Script::from_bytes(b"Arab");
    pub const ARMENIAN: Script = Script::from_bytes(b"Armn");
    pub const BENGALI: Script = Script::from_bytes(b"Beng");
    pub const CYRILLIC: Script = Script::from_bytes(b"Cyrl");
    pub const DEVANAGARI: Script = Script::from_bytes(b"Deva");
    pub const GEORGIAN: Script = Script::from_bytes(b"Geor");
    pub const GREEK: Script = Script::from_bytes(b"Grek");
    pub const GUJARATI: Script = Script::from_bytes(b"Gujr");
    pub const GURMUKHI: Script = Script::from_bytes(b"Guru");
    pub const HANGUL: Script = Script::from_bytes(b"Hang");
    pub const HAN: Script = Script::from_bytes(b"Hani");
    pub const HEBREW: Script = Script::from_bytes(b"Hebr");
    pub const HIRAGANA: Script = Script::from_bytes(b"Hira");
    pub const KANNADA: Script = Script::from_bytes(b"Knda");
    pub const KATAKANA: Script = Script::from_bytes(b"Kana");
    pub const LAO: Script = Script::from_bytes(b"Laoo");
    pub const LATIN: Script = Script::from_bytes(b"Latn");
    pub const MALAYALAM: Script = Script::from_bytes(b"Mlym");
    pub const ORIYA: Script = Script::from_bytes(b"Orya");
    pub const TAMIL: Script = Script::from_bytes(b"Taml");
    pub const TELUGU: Script = Script::from_bytes(b"Telu");
    pub const THAI: Script = Script::from_bytes(b"Thai");
    // Since 2.0
    pub const TIBETAN: Script = Script::from_bytes(b"Tibt");
    // Since 3.0
    pub const BOPOMOFO: Script = Script::from_bytes(b"Bopo");
    pub const BRAILLE: Script = Script::from_bytes(b"Brai");
    pub const CANADIAN_SYLLABICS: Script = Script::from_bytes(b"Cans");
    pub const CHEROKEE: Script = Script::from_bytes(b"Cher");
    pub const ETHIOPIC: Script = Script::from_bytes(b"Ethi");
    pub const KHMER: Script = Script::from_bytes(b"Khmr");
    pub const MONGOLIAN: Script = Script::from_bytes(b"Mong");
    pub const MYANMAR: Script = Script::from_bytes(b"Mymr");
    pub const OGHAM: Script = Script::from_bytes(b"Ogam");
    pub const RUNIC: Script = Script::from_bytes(b"Runr");
    pub const SINHALA: Script = Script::from_bytes(b"Sinh");
    pub const SYRIAC: Script = Script::from_bytes(b"Syrc");
    pub const THAANA: Script = Script::from_bytes(b"Thaa");
    pub const YI: Script = Script::from_bytes(b"Yiii");
    // Since 3.1
    pub const DESERET: Script = Script::from_bytes(b"Dsrt");
    pub const GOTHIC: Script = Script::from_bytes(b"Goth");
    pub const OLD_ITALIC: Script = Script::from_bytes(b"Ital");
    // Since 3.2
    pub const BUHID: Script = Script::from_bytes(b"Buhd");
    pub const HANUNOO: Script = Script::from_bytes(b"Hano");
    pub const TAGALOG: Script = Script::from_bytes(b"Tglg");
    pub const TAGBANWA: Script = Script::from_bytes(b"Tagb");
    // Since 4.0
    pub const CYPRIOT: Script = Script::from_bytes(b"Cprt");
    pub const LIMBU: Script = Script::from_bytes(b"Limb");
    pub const LINEAR_B: Script = Script::from_bytes(b"Linb");
    pub const OSMANYA: Script = Script::from_bytes(b"Osma");
    pub const SHAVIAN: Script = Script::from_bytes(b"Shaw");
    pub const TAI_LE: Script = Script::from_bytes(b"Tale");
    pub const UGARITIC: Script = Script::from_bytes(b"Ugar");
    // Since 4.1
    pub const BUGINESE: Script = Script::from_bytes(b"Bugi");
    pub const COPTIC: Script = Script::from_bytes(b"Copt");
    pub const GLAGOLITIC: Script = Script::from_bytes(b"Glag");
    pub const KHAROSHTHI: Script = Script::from_bytes(b"Khar");
    pub const NEW_TAI_LUE: Script = Script::from_bytes(b"Talu");
    pub const OLD_PERSIAN: Script = Script::from_bytes(b"Xpeo");
    pub const SYLOTI_NAGRI: Script = Script::from_bytes(b"Sylo");
    pub const TIFINAGH: Script = Script::from_bytes(b"Tfng");
    // Since 5.0
    pub const UNKNOWN: Script = Script::from_bytes(b"Zzzz"); // Script can be Unknown, but not Invalid.
    pub const BALINESE: Script = Script::from_bytes(b"Bali");
    pub const CUNEIFORM: Script = Script::from_bytes(b"Xsux");
    pub const NKO: Script = Script::from_bytes(b"Nkoo");
    pub const PHAGS_PA: Script = Script::from_bytes(b"Phag");
    pub const PHOENICIAN: Script = Script::from_bytes(b"Phnx");
    // Since 5.1
    pub const CARIAN: Script = Script::from_bytes(b"Cari");
    pub const CHAM: Script = Script::from_bytes(b"Cham");
    pub const KAYAH_LI: Script = Script::from_bytes(b"Kali");
    pub const LEPCHA: Script = Script::from_bytes(b"Lepc");
    pub const LYCIAN: Script = Script::from_bytes(b"Lyci");
    pub const LYDIAN: Script = Script::from_bytes(b"Lydi");
    pub const OL_CHIKI: Script = Script::from_bytes(b"Olck");
    pub const REJANG: Script = Script::from_bytes(b"Rjng");
    pub const SAURASHTRA: Script = Script::from_bytes(b"Saur");
    pub const SUNDANESE: Script = Script::from_bytes(b"Sund");
    pub const VAI: Script = Script::from_bytes(b"Vaii");
    // Since 5.2
    pub const AVESTAN: Script = Script::from_bytes(b"Avst");
    pub const BAMUM: Script = Script::from_bytes(b"Bamu");
    pub const EGYPTIAN_HIEROGLYPHS: Script = Script::from_bytes(b"Egyp");
    pub const IMPERIAL_ARAMAIC: Script = Script::from_bytes(b"Armi");
    pub const INSCRIPTIONAL_PAHLAVI: Script = Script::from_bytes(b"Phli");
    pub const INSCRIPTIONAL_PARTHIAN: Script = Script::from_bytes(b"Prti");
    pub const JAVANESE: Script = Script::from_bytes(b"Java");
    pub const KAITHI: Script = Script::from_bytes(b"Kthi");
    pub const LISU: Script = Script::from_bytes(b"Lisu");
    pub const MEETEI_MAYEK: Script = Script::from_bytes(b"Mtei");
    pub const OLD_SOUTH_ARABIAN: Script = Script::from_bytes(b"Sarb");
    pub const OLD_TURKIC: Script = Script::from_bytes(b"Orkh");
    pub const SAMARITAN: Script = Script::from_bytes(b"Samr");
    pub const TAI_THAM: Script = Script::from_bytes(b"Lana");
    pub const TAI_VIET: Script = Script::from_bytes(b"Tavt");
    // Since 6.0
    pub const BATAK: Script = Script::from_bytes(b"Batk");
    pub const BRAHMI: Script = Script::from_bytes(b"Brah");
    pub const MANDAIC: Script = Script::from_bytes(b"Mand");
    // Since 6.1
    pub const CHAKMA: Script = Script::from_bytes(b"Cakm");
    pub const MEROITIC_CURSIVE: Script = Script::from_bytes(b"Merc");
    pub const MEROITIC_HIEROGLYPHS: Script = Script::from_bytes(b"Mero");
    pub const MIAO: Script = Script::from_bytes(b"Plrd");
    pub const SHARADA: Script = Script::from_bytes(b"Shrd");
    pub const SORA_SOMPENG: Script = Script::from_bytes(b"Sora");
    pub const TAKRI: Script = Script::from_bytes(b"Takr");
    // Since 7.0
    pub const BASSA_VAH: Script = Script::from_bytes(b"Bass");
    pub const CAUCASIAN_ALBANIAN: Script = Script::from_bytes(b"Aghb");
    pub const DUPLOYAN: Script = Script::from_bytes(b"Dupl");
    pub const ELBASAN: Script = Script::from_bytes(b"Elba");
    pub const GRANTHA: Script = Script::from_bytes(b"Gran");
    pub const KHOJKI: Script = Script::from_bytes(b"Khoj");
    pub const KHUDAWADI: Script = Script::from_bytes(b"Sind");
    pub const LINEAR_A: Script = Script::from_bytes(b"Lina");
    pub const MAHAJANI: Script = Script::from_bytes(b"Mahj");
    pub const MANICHAEAN: Script = Script::from_bytes(b"Mani");
    pub const MENDE_KIKAKUI: Script = Script::from_bytes(b"Mend");
    pub const MODI: Script = Script::from_bytes(b"Modi");
    pub const MRO: Script = Script::from_bytes(b"Mroo");
    pub const NABATAEAN: Script = Script::from_bytes(b"Nbat");
    pub const OLD_NORTH_ARABIAN: Script = Script::from_bytes(b"Narb");
    pub const OLD_PERMIC: Script = Script::from_bytes(b"Perm");
    pub const PAHAWH_HMONG: Script = Script::from_bytes(b"Hmng");
    pub const PALMYRENE: Script = Script::from_bytes(b"Palm");
    pub const PAU_CIN_HAU: Script = Script::from_bytes(b"Pauc");
    pub const PSALTER_PAHLAVI: Script = Script::from_bytes(b"Phlp");
    pub const SIDDHAM: Script = Script::from_bytes(b"Sidd");
    pub const TIRHUTA: Script = Script::from_bytes(b"Tirh");
    pub const WARANG_CITI: Script = Script::from_bytes(b"Wara");
    // Since 8.0
    pub const AHOM: Script = Script::from_bytes(b"Ahom");
    pub const ANATOLIAN_HIEROGLYPHS: Script = Script::from_bytes(b"Hluw");
    pub const HATRAN: Script = Script::from_bytes(b"Hatr");
    pub const MULTANI: Script = Script::from_bytes(b"Mult");
    pub const OLD_HUNGARIAN: Script = Script::from_bytes(b"Hung");
    pub const SIGNWRITING: Script = Script::from_bytes(b"Sgnw");
    // Since 9.0
    pub const ADLAM: Script = Script::from_bytes(b"Adlm");
    pub const BHAIKSUKI: Script = Script::from_bytes(b"Bhks");
    pub const MARCHEN: Script = Script::from_bytes(b"Marc");
    pub const OSAGE: Script = Script::from_bytes(b"Osge");
    pub const TANGUT: Script = Script::from_bytes(b"Tang");
    pub const NEWA: Script = Script::from_bytes(b"Newa");
    // Since 10.0
    pub const MASARAM_GONDI: Script = Script::from_bytes(b"Gonm");
    pub const NUSHU: Script = Script::from_bytes(b"Nshu");
    pub const SOYOMBO: Script = Script::from_bytes(b"Soyo");
    pub const ZANABAZAR_SQUARE: Script = Script::from_bytes(b"Zanb");
    // Since 11.0
    pub const DOGRA: Script = Script::from_bytes(b"Dogr");
    pub const GUNJALA_GONDI: Script = Script::from_bytes(b"Gong");
    pub const HANIFI_ROHINGYA: Script = Script::from_bytes(b"Rohg");
    pub const MAKASAR: Script = Script::from_bytes(b"Maka");
    pub const MEDEFAIDRIN: Script = Script::from_bytes(b"Medf");
    pub const OLD_SOGDIAN: Script = Script::from_bytes(b"Sogo");
    pub const SOGDIAN: Script = Script::from_bytes(b"Sogd");
    // Since 12.0
    pub const ELYMAIC: Script = Script::from_bytes(b"Elym");
    pub const NANDINAGARI: Script = Script::from_bytes(b"Nand");
    pub const NYIAKENG_PUACHUE_HMONG: Script = Script::from_bytes(b"Hmnp");
    pub const WANCHO: Script = Script::from_bytes(b"Wcho");
    // Since 13.0
    pub const CHORASMIAN: Script = Script::from_bytes(b"Chrs");
    pub const DIVES_AKURU: Script = Script::from_bytes(b"Diak");
    pub const KHITAN_SMALL_SCRIPT: Script = Script::from_bytes(b"Kits");
    pub const YEZIDI: Script = Script::from_bytes(b"Yezi");
    // Since 14.0
    pub const CYPRO_MINOAN: Script = Script::from_bytes(b"Cpmn");
    pub const OLD_UYGHUR: Script = Script::from_bytes(b"Ougr");
    pub const TANGSA: Script = Script::from_bytes(b"Tnsa");
    pub const TOTO: Script = Script::from_bytes(b"Toto");
    pub const VITHKUQI: Script = Script::from_bytes(b"Vith");
    // Since 15.0
    pub const KAWI: Script = Script::from_bytes(b"Kawi");
    pub const NAG_MUNDARI: Script = Script::from_bytes(b"Nagm");
    // Since 16.0
    pub const GARAY: Script = Script::from_bytes(b"Gara");
    pub const GURUNG_KHEMA: Script = Script::from_bytes(b"Gukh");
    pub const KIRAT_RAI: Script = Script::from_bytes(b"Krai");
    pub const OL_ONAL: Script = Script::from_bytes(b"Onao");
    pub const SUNUWAR: Script = Script::from_bytes(b"Sunu");
    pub const TODHRI: Script = Script::from_bytes(b"Todr");
    pub const TULU_TIGALARI: Script = Script::from_bytes(b"Tutg");

    pub const SCRIPT_MATH: Script = Script::from_bytes(b"Zmth");

    // https://github.com/harfbuzz/harfbuzz/issues/1162
    pub const MYANMAR_ZAWGYI: Script = Script::from_bytes(b"Qaag");
}

/// A feature tag with an accompanying range specifying on which subslice of
/// `shape`s input it should be applied.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub struct Feature {
    pub tag: Tag,
    pub value: u32,
    pub start: u32,
    pub end: u32,
}

impl Feature {
    /// Create a new `Feature` struct.
    pub fn new(tag: Tag, value: u32, range: impl RangeBounds<usize>) -> Feature {
        let max = u32::MAX as usize;
        let start = match range.start_bound() {
            Bound::Included(&included) => included.min(max) as u32,
            Bound::Excluded(&excluded) => excluded.min(max - 1) as u32 + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&included) => included.min(max) as u32,
            Bound::Excluded(&excluded) => excluded.saturating_sub(1).min(max) as u32,
            Bound::Unbounded => max as u32,
        };

        Feature {
            tag,
            value,
            start,
            end,
        }
    }

    pub(crate) fn is_global(&self) -> bool {
        self.start == 0 && self.end == u32::MAX
    }
}

impl core::str::FromStr for Feature {
    type Err = &'static str;

    /// Parses a `Feature` form a string.
    ///
    /// Possible values:
    ///
    /// - `kern` -> kern .. 1
    /// - `+kern` -> kern .. 1
    /// - `-kern` -> kern .. 0
    /// - `kern=0` -> kern .. 0
    /// - `kern=1` -> kern .. 1
    /// - `aalt=2` -> altr .. 2
    /// - `kern[]` -> kern .. 1
    /// - `kern[:]` -> kern .. 1
    /// - `kern[5:]` -> kern 5.. 1
    /// - `kern[:5]` -> kern ..=5 1
    /// - `kern[3:5]` -> kern 3..=5 1
    /// - `kern[3]` -> kern 3..=4 1
    /// - `aalt[3:5]=2` -> kern 3..=5 1
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> Option<Feature> {
            if s.is_empty() {
                return None;
            }

            let mut p = TextParser::new(s);

            // Parse prefix.
            let mut value = 1;
            match p.curr_byte()? {
                b'-' => {
                    value = 0;
                    p.advance(1);
                }
                b'+' => {
                    value = 1;
                    p.advance(1);
                }
                _ => {}
            }

            // Parse tag.
            p.skip_spaces();
            let quote = p.consume_quote();

            let tag = p.consume_tag()?;

            // Force closing quote.
            if let Some(quote) = quote {
                p.consume_byte(quote)?;
            }

            // Parse indices.
            p.skip_spaces();

            let (start, end) = if p.consume_byte(b'[').is_some() {
                let start_opt = p.consume_i32();
                let start = start_opt.unwrap_or(0) as u32; // negative value overflow is ok

                let end = if matches!(p.curr_byte(), Some(b':') | Some(b';')) {
                    p.advance(1);
                    p.consume_i32().unwrap_or(-1) as u32 // negative value overflow is ok
                } else {
                    if start_opt.is_some() && start != u32::MAX {
                        start + 1
                    } else {
                        u32::MAX
                    }
                };

                p.consume_byte(b']')?;

                (start, end)
            } else {
                (0, u32::MAX)
            };

            // Parse postfix.
            let had_equal = p.consume_byte(b'=').is_some();
            let value1 = p
                .consume_i32()
                .or_else(|| p.consume_bool().map(|b| b as i32));

            if had_equal && value1.is_none() {
                return None;
            };

            if let Some(value1) = value1 {
                value = value1 as u32; // negative value overflow is ok
            }

            p.skip_spaces();

            if !p.at_end() {
                return None;
            }

            Some(Feature {
                tag,
                value,
                start,
                end,
            })
        }

        parse(s).ok_or("invalid feature")
    }
}

#[cfg(test)]
mod tests_features {
    use super::*;
    use core::str::FromStr;

    macro_rules! test {
        ($name:ident, $text:expr, $tag:expr, $value:expr, $range:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Feature::from_str($text).unwrap(),
                    Feature::new(Tag::from_bytes($tag), $value, $range)
                );
            }
        };
    }

    test!(parse_01, "kern", b"kern", 1, ..);
    test!(parse_02, "+kern", b"kern", 1, ..);
    test!(parse_03, "-kern", b"kern", 0, ..);
    test!(parse_04, "kern=0", b"kern", 0, ..);
    test!(parse_05, "kern=1", b"kern", 1, ..);
    test!(parse_06, "kern=2", b"kern", 2, ..);
    test!(parse_07, "kern[]", b"kern", 1, ..);
    test!(parse_08, "kern[:]", b"kern", 1, ..);
    test!(parse_09, "kern[5:]", b"kern", 1, 5..);
    test!(parse_10, "kern[:5]", b"kern", 1, ..=5);
    test!(parse_11, "kern[3:5]", b"kern", 1, 3..=5);
    test!(parse_12, "kern[3]", b"kern", 1, 3..=4);
    test!(parse_13, "kern[3:5]=2", b"kern", 2, 3..=5);
    test!(parse_14, "kern[3;5]=2", b"kern", 2, 3..=5);
    test!(parse_15, "kern[:-1]", b"kern", 1, ..);
    test!(parse_16, "kern[-1]", b"kern", 1, u32::MAX as usize..);
    test!(parse_17, "kern=on", b"kern", 1, ..);
    test!(parse_18, "kern=off", b"kern", 0, ..);
    test!(parse_19, "kern=oN", b"kern", 1, ..);
    test!(parse_20, "kern=oFf", b"kern", 0, ..);
}

/// A font variation.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Variation {
    pub tag: Tag,
    pub value: f32,
}

impl core::str::FromStr for Variation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> Option<Variation> {
            if s.is_empty() {
                return None;
            }

            let mut p = TextParser::new(s);

            // Parse tag.
            p.skip_spaces();
            let quote = p.consume_quote();

            let tag = p.consume_tag()?;

            // Force closing quote.
            if let Some(quote) = quote {
                p.consume_byte(quote)?;
            }

            let _ = p.consume_byte(b'=');
            let value = p.consume_f32()?;
            p.skip_spaces();

            if !p.at_end() {
                return None;
            }

            Some(Variation { tag, value })
        }

        parse(s).ok_or("invalid variation")
    }
}

pub trait TagExt {
    fn default_script() -> Self;
    fn default_language() -> Self;
    #[cfg(test)]
    fn to_lowercase(&self) -> Self;
    fn to_uppercase(&self) -> Self;
}

impl TagExt for Tag {
    #[inline]
    fn default_script() -> Self {
        Tag::from_bytes(b"DFLT")
    }

    #[inline]
    fn default_language() -> Self {
        Tag::from_bytes(b"dflt")
    }

    /// Converts tag to lowercase.
    #[cfg(test)]
    #[inline]
    fn to_lowercase(&self) -> Self {
        let b = self.to_bytes();
        Tag::from_bytes(&[
            b[0].to_ascii_lowercase(),
            b[1].to_ascii_lowercase(),
            b[2].to_ascii_lowercase(),
            b[3].to_ascii_lowercase(),
        ])
    }

    /// Converts tag to uppercase.
    #[inline]
    fn to_uppercase(&self) -> Self {
        let b = self.to_bytes();
        Tag::from_bytes(&[
            b[0].to_ascii_uppercase(),
            b[1].to_ascii_uppercase(),
            b[2].to_ascii_uppercase(),
            b[3].to_ascii_uppercase(),
        ])
    }
}
