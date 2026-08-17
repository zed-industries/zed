//! The [meta (Metadata)](https://docs.microsoft.com/en-us/typography/opentype/spec/meta) table

include!("../../generated/generated_meta.rs");

pub const DLNG: Tag = Tag::new(b"dlng");
pub const SLNG: Tag = Tag::new(b"slng");

/// Data stored in the 'meta' table.
pub enum Metadata<'a> {
    /// Used for the 'dlng' and 'slng' metadata
    ScriptLangTags(VarLenArray<'a, ScriptLangTag<'a>>),
    /// Other metadata, which may exist in certain apple fonts
    Other(&'a [u8]),
}

impl ReadArgs for Metadata<'_> {
    type Args = (Tag, u32);
}

impl<'a> FontReadWithArgs<'a> for Metadata<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        let (tag, len) = *args;
        let data = data.slice(0..len as usize).ok_or(ReadError::OutOfBounds)?;
        if [DLNG, SLNG].contains(&tag) {
            VarLenArray::read(data).map(Metadata::ScriptLangTags)
        } else {
            Ok(Metadata::Other(data.as_bytes()))
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScriptLangTag<'a>(&'a str);

impl<'a> ScriptLangTag<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for ScriptLangTag<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[cfg(feature = "std")]
impl From<ScriptLangTag<'_>> for String {
    fn from(value: ScriptLangTag<'_>) -> Self {
        value.0.into()
    }
}

impl VarSize for ScriptLangTag<'_> {
    type Size = u32;

    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        let bytes = data.split_off(pos)?.as_bytes();
        if bytes.is_empty() {
            return None;
        }
        let end = data
            .as_bytes()
            .iter()
            .position(|b| *b == b',')
            .map(|pos| pos + 1) // include comma
            .unwrap_or(bytes.len());
        Some(end)
    }
}

impl<'a> FontRead<'a> for ScriptLangTag<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        std::str::from_utf8(data.as_bytes())
            .map_err(|_| ReadError::MalformedData("LangScriptTag must be utf8"))
            .map(|s| ScriptLangTag(s.trim_matches(|c| c == ' ' || c == ',')))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::meta as test_data;

    impl PartialEq<&str> for ScriptLangTag<'_> {
        fn eq(&self, other: &&str) -> bool {
            self.as_ref() == *other
        }
    }

    fn expect_script_lang_tags(table: Metadata, expected: &[&str]) -> bool {
        let Metadata::ScriptLangTags(langs) = table else {
            panic!("wrong metadata");
        };
        let result = langs.iter().map(|x| x.unwrap()).collect::<Vec<_>>();
        result == expected
    }

    #[test]
    fn parse_simple() {
        let table = Meta::read(test_data::SIMPLE_META_TABLE.into()).unwrap();
        let rec1 = table.data_maps()[0];
        let rec2 = table.data_maps()[1];

        assert_eq!(rec1.tag(), Tag::new(b"dlng"));
        assert_eq!(rec2.tag(), Tag::new(b"slng"));
        assert!(expect_script_lang_tags(
            rec1.data(table.offset_data()).unwrap(),
            &["en-latn", "latn"]
        ));
        assert!(expect_script_lang_tags(
            rec2.data(table.offset_data()).unwrap(),
            &["latn"]
        ));
    }
}
