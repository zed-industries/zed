//! Additional support for working with OpenType scripts and language systems.

use super::{FeatureList, LangSys, ReadError, Script, ScriptList, Tag, TaggedElement};
use std::ops::Deref;

/// A script chosen from a set of candidate tags.
///
/// Returned by the [`ScriptList::select`] method.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SelectedScript {
    /// The actual OpenType tag of the chosen script.
    pub tag: Tag,
    /// Index of the script in the [`ScriptList`].
    pub index: u16,
    /// True if a script was chosen that wasn't in the requested list.
    pub is_fallback: bool,
}

impl<'a> ScriptList<'a> {
    /// Returns the index of the script with the given tag.
    pub fn index_for_tag(&self, tag: Tag) -> Option<u16> {
        self.script_records()
            .binary_search_by_key(&tag, |rec| rec.script_tag())
            .map(|index| index as u16)
            .ok()
    }

    /// Returns the tag and script at the given index.
    pub fn get(&self, index: u16) -> Result<TaggedElement<Script<'a>>, ReadError> {
        self.script_records()
            .get(index as usize)
            .ok_or(ReadError::OutOfBounds)
            .and_then(|rec| {
                Ok(TaggedElement::new(
                    rec.script_tag(),
                    rec.script(self.offset_data())?,
                ))
            })
    }

    /// Finds the first available script that matches one of the given tags.
    ///
    /// When none of the requested scripts are available, then `DFLT`, `dflt`
    /// and `latn` tags are tried in that order.
    ///
    /// If you're starting from a Unicode script code, use the
    /// [`ScriptTags::from_unicode`] function to generate the appropriate set
    /// of tags to pass to this method.
    ///
    /// See [`hb_ot_layout_table_select_script`](https://github.com/harfbuzz/harfbuzz/blob/2edc371e97d6d2c5ad0e085b26e9af0123501647/src/hb-ot-layout.cc#L547)
    /// for the corresponding HarfBuzz function.
    pub fn select(&self, tags: &[Tag]) -> Option<SelectedScript> {
        for &tag in tags {
            if let Some(index) = self.index_for_tag(tag) {
                return Some(SelectedScript {
                    tag,
                    index,
                    is_fallback: false,
                });
            }
        }
        for tag in [
            // Try finding 'DFLT'
            Tag::new(b"DFLT"),
            // Try with 'dflt'; MS site has had typos and many fonts use it now :(
            Tag::new(b"dflt"),
            // try with 'latn'; some old fonts put their features there even though
            // they're really trying to support Thai, for example :(
            Tag::new(b"latn"),
        ] {
            if let Some(index) = self.index_for_tag(tag) {
                return Some(SelectedScript {
                    tag,
                    index,
                    is_fallback: true,
                });
            }
        }
        None
    }
}

impl<'a> Script<'a> {
    /// If the script contains a language system with the given tag, returns
    /// the index.
    pub fn lang_sys_index_for_tag(&self, tag: Tag) -> Option<u16> {
        self.lang_sys_records()
            .binary_search_by_key(&tag, |rec| rec.lang_sys_tag())
            .map(|index| index as u16)
            .ok()
    }

    /// Returns the language system with the given index.
    pub fn lang_sys(&self, index: u16) -> Result<TaggedElement<LangSys<'a>>, ReadError> {
        self.lang_sys_records()
            .get(index as usize)
            .ok_or(ReadError::OutOfBounds)
            .and_then(|rec| {
                Ok(TaggedElement::new(
                    rec.lang_sys_tag(),
                    rec.lang_sys(self.offset_data())?,
                ))
            })
    }
}

impl LangSys<'_> {
    /// If the language system references a feature with the given tag,
    /// returns the index of that feature in the specified feature list.
    ///
    /// The feature list can be obtained from the `feature_list` method on
    /// the parent [Gsub](crate::tables::gsub::Gsub) or
    /// [Gpos](crate::tables::gpos::Gpos) tables.
    pub fn feature_index_for_tag(&self, list: &FeatureList, tag: Tag) -> Option<u16> {
        let records = list.feature_records();
        self.feature_indices()
            .iter()
            .map(|ix| ix.get())
            .find(|&feature_ix| {
                records
                    .get(feature_ix as usize)
                    .map(|rec| rec.feature_tag())
                    == Some(tag)
            })
    }
}

/// A prioritized list of OpenType script tags mapped from a Unicode script
/// tag.
///
/// This is useful as input to [`ScriptList::select`] when you have a Unicode
/// script and would like to find the appropriate OpenType script for shaping.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct ScriptTags {
    tags: [Tag; 3],
    len: usize,
}

impl ScriptTags {
    /// Given a [Unicode script code](https://unicode.org/iso15924/iso15924-codes.html),
    /// returns a prioritized list of matching
    /// [OpenType script tags](https://learn.microsoft.com/en-us/typography/opentype/spec/scripttags).
    ///
    /// See [hb_ot_all_tags_from_script](https://github.com/harfbuzz/harfbuzz/blob/63d09dbefcf7ad9f794ca96445d37b6d8c3c9124/src/hb-ot-tag.cc#L155C1-L155C27)
    /// for the equivalent HarfBuzz function.    
    pub fn from_unicode(unicode_script: Tag) -> Self {
        let mut tags = [Tag::default(); 3];
        let mut len = 0;
        if let Some(new_tag) = new_tag_from_unicode(unicode_script) {
            // Myanmar maps to mym2 but there is no mym3
            if new_tag != Tag::new(b"mym2") {
                let mut bytes = new_tag.to_be_bytes();
                bytes[3] = b'3';
                tags[len] = Tag::new(&bytes);
                len += 1;
            }
            tags[len] = new_tag;
            len += 1;
        }
        tags[len] = old_tag_from_unicode(unicode_script);
        len += 1;
        Self { tags, len }
    }

    /// Returns a slice containing the mapped script tags.
    pub fn as_slice(&self) -> &[Tag] {
        &self.tags[..self.len]
    }
}

impl Deref for ScriptTags {
    type Target = [Tag];

    fn deref(&self) -> &Self::Target {
        &self.tags[..self.len]
    }
}

impl std::fmt::Debug for ScriptTags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

// See <https://github.com/harfbuzz/harfbuzz/blob/63d09dbefcf7ad9f794ca96445d37b6d8c3c9124/src/hb-ot-tag.cc#L37>
fn old_tag_from_unicode(unicode_script: Tag) -> Tag {
    let mut bytes = unicode_script.to_be_bytes();
    let tag_bytes = match &bytes {
        b"Zmth" => b"math",
        // Katakana and Hiragana both map to 'kana'
        b"Hira" => b"kana",
        // Spaces at the end are preserved, unlike ISO 15924
        b"Laoo" => b"lao ",
        b"Yiii" => b"yi  ",
        // Unicode 5.0 additions
        b"Nkoo" => b"nko ",
        // Unicode 5.1 additions
        b"Vaii" => b"vai ",
        _ => {
            // Else, just change the first char to lowercase
            bytes[0] = bytes[0].to_ascii_lowercase();
            &bytes
        }
    };
    Tag::new(tag_bytes)
}

/// Mapping from Unicode script code to "new" OpenType script
/// tags.
#[doc(hidden)]
pub const UNICODE_TO_NEW_OPENTYPE_SCRIPT_TAGS: &[(&[u8; 4], Tag)] = &[
    (b"Beng", Tag::new(b"bng2")),
    (b"Deva", Tag::new(b"dev2")),
    (b"Gujr", Tag::new(b"gjr2")),
    (b"Guru", Tag::new(b"gur2")),
    (b"Knda", Tag::new(b"knd2")),
    (b"Mlym", Tag::new(b"mlm2")),
    (b"Mymr", Tag::new(b"mym2")),
    (b"Orya", Tag::new(b"ory2")),
    (b"Taml", Tag::new(b"tml2")),
    (b"Telu", Tag::new(b"tel2")),
];

// See <https://github.com/harfbuzz/harfbuzz/blob/63d09dbefcf7ad9f794ca96445d37b6d8c3c9124/src/hb-ot-tag.cc#L84>
fn new_tag_from_unicode(unicode_script: Tag) -> Option<Tag> {
    let ix = UNICODE_TO_NEW_OPENTYPE_SCRIPT_TAGS
        .binary_search_by_key(&unicode_script.to_be_bytes(), |entry| *entry.0)
        .ok()?;
    UNICODE_TO_NEW_OPENTYPE_SCRIPT_TAGS
        .get(ix)
        .map(|entry| entry.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, TableProvider};

    #[test]
    fn script_index_for_tag() {
        let font = FontRef::new(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS).unwrap();
        let gsub_scripts = font.gsub().unwrap().script_list().unwrap();
        let ordered_scripts = [b"DFLT", b"cyrl", b"grek", b"hebr", b"latn"];
        for (index, tag) in ordered_scripts.into_iter().enumerate() {
            let tag = Tag::new(tag);
            assert_eq!(gsub_scripts.index_for_tag(tag), Some(index as u16));
        }
    }

    #[test]
    fn simple_script_tag_from_unicode() {
        let unicode_tags = [b"Cyrl", b"Grek", b"Hebr", b"Latn"];
        for unicode_tag in unicode_tags {
            // These should all return a single tag that is simply
            // the lowercase version of the Unicode tag
            let mut bytes = *unicode_tag;
            bytes[0] = bytes[0].to_ascii_lowercase();
            let expected_tag = Tag::new(&bytes);
            let result = ScriptTags::from_unicode(Tag::new(unicode_tag));
            assert_eq!(&*result, &[expected_tag]);
        }
    }

    #[test]
    fn exception_script_tag_from_unicode() {
        let cases = [
            // (Unicode, OpenType)
            (b"Kana", b"kana"),
            // Hiragana maps to kana
            (b"Hira", b"kana"),
            // Unicode extends last char but OpenType pads with spaces
            // for tags < 4 bytes
            (b"Nkoo", b"nko "),
            (b"Yiii", b"yi  "),
            (b"Vaii", b"vai "),
        ];
        for (unicode_tag, ot_tag) in cases {
            let result = ScriptTags::from_unicode(Tag::new(unicode_tag));
            assert_eq!(&*result, &[Tag::new(ot_tag)]);
        }
    }

    #[test]
    fn multi_script_tags_from_unicode() {
        let cases = [
            // (Unicode, OpenType)
            (b"Beng", &[b"bng3", b"bng2", b"beng"][..]),
            (b"Orya", &[b"ory3", b"ory2", b"orya"]),
            (b"Mlym", &[b"mlm3", b"mlm2", b"mlym"]),
            // There's no version 3 tag for Myanmar
            (b"Mymr", &[b"mym2", b"mymr"]),
        ];
        for (unicode_tag, ot_tags) in cases {
            let result = ScriptTags::from_unicode(Tag::new(unicode_tag));
            let ot_tags = ot_tags
                .iter()
                .map(|bytes| Tag::new(bytes))
                .collect::<Vec<_>>();
            assert_eq!(&*result, &ot_tags);
        }
    }

    #[test]
    fn select_scripts_from_unicode() {
        let font = FontRef::new(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS).unwrap();
        let gsub_scripts = font.gsub().unwrap().script_list().unwrap();
        // We know Hebrew is available
        let hebr = gsub_scripts
            .select(&ScriptTags::from_unicode(Tag::new(b"Hebr")))
            .unwrap();
        assert_eq!(
            hebr,
            SelectedScript {
                tag: Tag::new(b"hebr"),
                index: 3,
                is_fallback: false,
            }
        );
        // But this font doesn't contain any Indic scripts so we'll
        // select a fallback for Bengali
        let beng = gsub_scripts
            .select(&ScriptTags::from_unicode(Tag::new(b"Beng")))
            .unwrap();
        assert_eq!(
            beng,
            SelectedScript {
                tag: Tag::new(b"DFLT"),
                index: 0,
                is_fallback: true,
            }
        );
    }

    #[test]
    fn script_list_get() {
        const LATN: Tag = Tag::new(b"latn");
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let gsub = font.gsub().unwrap();
        let script_list = gsub.script_list().unwrap();
        let latn_script_index = script_list.index_for_tag(LATN).unwrap();
        assert_eq!(latn_script_index, 1);
        let script = script_list.get(latn_script_index).unwrap();
        assert_eq!(script.tag, LATN);
    }

    #[test]
    fn script_lang_sys_helpers() {
        const TRK: Tag = Tag::new(b"TRK ");
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let gsub = font.gsub().unwrap();
        let script_list = gsub.script_list().unwrap();
        let script = script_list.get(1).unwrap();
        let lang_sys_index = script.lang_sys_index_for_tag(TRK).unwrap();
        assert_eq!(lang_sys_index, 0);
        assert_eq!(script.lang_sys(lang_sys_index).unwrap().tag, TRK);
    }

    #[test]
    fn feature_index_for_tag() {
        let font = FontRef::new(font_test_data::MATERIAL_SYMBOLS_SUBSET).unwrap();
        let gsub = font.gsub().unwrap();
        let script_list = gsub.script_list().unwrap();
        let feature_list = gsub.feature_list().unwrap();
        let lang_sys = script_list
            .get(1)
            .unwrap()
            .default_lang_sys()
            .unwrap()
            .unwrap();
        assert_eq!(
            lang_sys.feature_index_for_tag(&feature_list, Tag::new(b"rclt")),
            Some(0)
        );
        assert_eq!(
            lang_sys.feature_index_for_tag(&feature_list, Tag::new(b"rlig")),
            Some(1)
        );
        for tag in [b"locl", b"abvs", b"liga"] {
            assert_eq!(
                lang_sys.feature_index_for_tag(&feature_list, Tag::new(tag)),
                None
            );
        }
    }
}
