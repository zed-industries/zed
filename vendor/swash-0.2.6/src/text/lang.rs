use crate::{tag_from_bytes, Tag};
use core::fmt;

use super::lang_data::*;

/// Chinese, Japanese and Korean languages.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Cjk {
    None = 0,
    Traditional = 1,
    Simplified = 2,
    Japanese = 3,
    Korean = 4,
}

/// Representation of a language and its associated script and region.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Language {
    language: [u8; 3],
    script: [u8; 4],
    region: [u8; 2],
    lang_len: u8,
    script_len: u8,
    region_len: u8,
    cjk: Cjk,
    name_index: u16,
    tag: Option<Tag>,
}

impl Language {
    /// Parses a language tag.
    pub fn parse(tag: &str) -> Option<Self> {
        let mut lang = Self {
            language: [0; 3],
            region: [0; 2],
            script: [0; 4],
            lang_len: 0,
            region_len: 0,
            script_len: 0,
            cjk: Cjk::None,
            name_index: 0xFFFF,
            tag: None,
        };
        let mut has_region = false;
        let mut zh = false;
        let mut lang_index = 0xFFFF;
        for (i, part) in tag.split('-').enumerate() {
            let bytes = part.as_bytes();
            let len = bytes.len();
            match i {
                0 => {
                    match len {
                        2 => {
                            let a = bytes[0].to_ascii_lowercase();
                            let b = bytes[1].to_ascii_lowercase();
                            match (a, b) {
                                (b'z', b'h') => zh = true,
                                (b'j', b'a') => lang.cjk = Cjk::Japanese,
                                (b'k', b'o') => lang.cjk = Cjk::Korean,
                                _ => {}
                            };
                            lang.language[0] = a;
                            lang.language[1] = b;
                            lang.lang_len = 2;
                            let key = tag2(&[a, b]);
                            if let Ok(index) = LANG_BY_TAG2.binary_search_by(|x| x.0.cmp(&key)) {
                                lang_index = LANG_BY_TAG2.get(index)?.1;
                            }
                        }
                        3 => {
                            let a = bytes[0].to_ascii_lowercase();
                            let b = bytes[1].to_ascii_lowercase();
                            let c = bytes[2].to_ascii_lowercase();
                            zh = a == b'z' && b == b'h' && c == b'o';
                            lang.language[0] = a;
                            lang.language[1] = b;
                            lang.language[2] = c;
                            lang.lang_len = 3;
                            let key = tag3(&[a, b, c]);
                            if let Ok(index) = LANG_BY_TAG3.binary_search_by(|x| x.0.cmp(&key)) {
                                lang_index = LANG_BY_TAG3.get(index)?.1 as u16;
                            }
                        }
                        _ => return None,
                    };
                }
                1 => match len {
                    2 => {
                        let a = bytes[0].to_ascii_uppercase();
                        let b = bytes[1].to_ascii_uppercase();
                        lang.region[0] = a;
                        lang.region[1] = b;
                        lang.region_len = 2;
                        has_region = true;
                    }
                    4 => {
                        let a = bytes[0].to_ascii_uppercase();
                        let b = bytes[1].to_ascii_lowercase();
                        let c = bytes[2].to_ascii_lowercase();
                        let d = bytes[3].to_ascii_lowercase();
                        lang.script[0] = a;
                        lang.script[1] = b;
                        lang.script[2] = c;
                        lang.script[3] = d;
                        lang.script_len = 4;
                    }
                    _ => break,
                },
                2 => {
                    if has_region || len != 2 {
                        break;
                    }
                    let a = bytes[0].to_ascii_uppercase();
                    let b = bytes[1].to_ascii_uppercase();
                    lang.region[0] = a;
                    lang.region[1] = b;
                    lang.region_len = 2;
                    has_region = true;
                }
                _ => break,
            }
        }
        lang.name_index = lang_index;
        if lang_index != 0xFFFF {
            lang.tag = Some(*LANG_TAGS.get(lang_index as usize)?);
        } else if zh {
            let (tag, cjk) = match lang.script().unwrap_or("") {
                "Hant" => (tag_from_bytes(b"ZHT "), Cjk::Traditional),
                "Hans" => (tag_from_bytes(b"ZHS "), Cjk::Simplified),
                _ => (tag_from_bytes(b"ZHT "), Cjk::Traditional),
            };
            lang.tag = Some(tag);
            lang.cjk = cjk;
            lang.name_index = match LANG_TAGS.binary_search_by(|x| x.cmp(&tag)) {
                Ok(index) => index as u16,
                _ => 0xFFFF,
            };
        }
        Some(lang)
    }

    /// Returns the language associated with the specified OpenType language
    /// tag.
    pub fn from_opentype(tag: Tag) -> Option<Self> {
        if tag == tag_from_bytes(b"ZHT ") {
            return Self::parse("zh-Hant");
        } else if tag == tag_from_bytes(b"ZHS ") {
            return Self::parse("zh-Hans");
        }
        let name_index = match LANG_TAGS.binary_search_by(|x| x.cmp(&tag)) {
            Ok(index) => index,
            _ => return None,
        };
        Self::parse(LANG_ENTRIES.get(name_index)?.1)
    }

    /// Returns the language component.
    pub fn language(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.language[..self.lang_len as usize]) }
    }

    /// Returns the script component.
    pub fn script(&self) -> Option<&str> {
        Some(if self.script_len == 4 {
            unsafe { core::str::from_utf8_unchecked(&self.script) }
        } else {
            return None;
        })
    }

    /// Returns the region component.
    pub fn region(&self) -> Option<&str> {
        Some(if self.region_len == 2 {
            unsafe { core::str::from_utf8_unchecked(&self.region) }
        } else {
            return None;
        })
    }

    /// Returns the CJK language.
    pub fn cjk(&self) -> Cjk {
        self.cjk
    }

    /// Returns the name of the language.
    pub fn name(&self) -> Option<&'static str> {
        LANG_ENTRIES.get(self.name_index as usize).map(|e| e.0)
    }

    /// Returns the associated OpenType language tag.
    pub fn to_opentype(self) -> Option<Tag> {
        self.tag
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.language())?;
        if let Some(script) = self.script() {
            write!(f, "-{}", script)?;
        }
        if let Some(region) = self.region() {
            write!(f, "-{}", region)?;
        }
        if let Some(name) = self.name() {
            write!(f, " ({})", name)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.language())?;
        if let Some(script) = self.script() {
            write!(f, "-{}", script)?;
        }
        if let Some(region) = self.region() {
            write!(f, "-{}", region)?;
        }
        if let Some(tag) = self.tag {
            let tag = tag.to_be_bytes();
            if let Ok(s) = core::str::from_utf8(&tag) {
                write!(f, " ({})", s)?;
            }
        }
        if let Some(name) = self.name() {
            write!(f, " \"{}\"", name)?;
        }
        Ok(())
    }
}
