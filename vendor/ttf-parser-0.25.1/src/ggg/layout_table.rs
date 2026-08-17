// Suppresses `minor_version` variable warning.
#![allow(unused_variables)]

#[cfg(feature = "variable-fonts")]
use super::FeatureVariations;
use super::LookupList;
#[cfg(feature = "variable-fonts")]
use crate::parser::Offset32;
use crate::parser::{FromData, LazyArray16, Offset, Offset16, Stream};
use crate::Tag;

/// A [Layout Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#table-organization).
#[derive(Clone, Copy, Debug)]
pub struct LayoutTable<'a> {
    /// A list of all supported scripts.
    pub scripts: ScriptList<'a>,
    /// A list of all supported features.
    pub features: FeatureList<'a>,
    /// A list of all lookups.
    pub lookups: LookupList<'a>,
    /// Used to substitute an alternate set of lookup tables
    /// to use for any given feature under specified conditions.
    #[cfg(feature = "variable-fonts")]
    pub variations: Option<FeatureVariations<'a>>,
}

impl<'a> LayoutTable<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let major_version = s.read::<u16>()?;
        let minor_version = s.read::<u16>()?;
        if major_version != 1 {
            return None;
        }

        let scripts = ScriptList::parse(s.read_at_offset16(data)?)?;
        let features = FeatureList::parse(s.read_at_offset16(data)?)?;
        let lookups = LookupList::parse(s.read_at_offset16(data)?)?;

        #[cfg(feature = "variable-fonts")]
        {
            let mut variations_offset = None;
            if minor_version >= 1 {
                variations_offset = s.read::<Option<Offset32>>()?;
            }

            let variations = match variations_offset {
                Some(offset) => data
                    .get(offset.to_usize()..)
                    .and_then(FeatureVariations::parse),
                None => None,
            };

            Some(Self {
                scripts,
                features,
                lookups,
                variations,
            })
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            Some(Self {
                scripts,
                features,
                lookups,
            })
        }
    }
}

/// An index in [`ScriptList`].
pub type ScriptIndex = u16;
/// An index in [`LanguageSystemList`].
pub type LanguageIndex = u16;
/// An index in [`FeatureList`].
pub type FeatureIndex = u16;
/// An index in [`LookupList`].
pub type LookupIndex = u16;
/// An index in [`FeatureVariations`].
pub type VariationIndex = u32;

/// A trait to parse item in [`RecordList`].
///
/// Internal use only.
pub trait RecordListItem<'a>: Sized {
    /// Parses raw data.
    fn parse(tag: Tag, data: &'a [u8]) -> Option<Self>;
}

/// A data storage used by [`ScriptList`], [`LanguageSystemList`] and [`FeatureList`] data types.
#[derive(Clone, Copy, Debug)]
pub struct RecordList<'a, T: RecordListItem<'a>> {
    data: &'a [u8],
    records: LazyArray16<'a, TagRecord>,
    data_type: core::marker::PhantomData<T>,
}

impl<'a, T: RecordListItem<'a>> RecordList<'a, T> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let records = s.read_array16(count)?;
        Some(Self {
            data,
            records,
            data_type: core::marker::PhantomData,
        })
    }

    /// Returns a number of items in the RecordList.
    pub fn len(&self) -> u16 {
        self.records.len()
    }

    /// Checks that RecordList is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns RecordList value by index.
    pub fn get(&self, index: u16) -> Option<T> {
        let record = self.records.get(index)?;
        self.data
            .get(record.offset.to_usize()..)
            .and_then(|data| T::parse(record.tag, data))
    }

    /// Returns RecordList value by [`Tag`].
    pub fn find(&self, tag: Tag) -> Option<T> {
        let record = self
            .records
            .binary_search_by(|record| record.tag.cmp(&tag))
            .map(|p| p.1)?;
        self.data
            .get(record.offset.to_usize()..)
            .and_then(|data| T::parse(record.tag, data))
    }

    /// Returns RecordList value index by [`Tag`].
    pub fn index(&self, tag: Tag) -> Option<u16> {
        self.records
            .binary_search_by(|record| record.tag.cmp(&tag))
            .map(|p| p.0)
    }
}

impl<'a, T: RecordListItem<'a>> IntoIterator for RecordList<'a, T> {
    type Item = T;
    type IntoIter = RecordListIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RecordListIter {
            list: self,
            index: 0,
        }
    }
}

/// An iterator over [`RecordList`] values.
#[allow(missing_debug_implementations)]
pub struct RecordListIter<'a, T: RecordListItem<'a>> {
    list: RecordList<'a, T>,
    index: u16,
}

impl<'a, T: RecordListItem<'a>> Iterator for RecordListIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            self.index += 1;
            self.list.get(self.index - 1)
        } else {
            None
        }
    }
}

/// A list of [`Script`] records.
pub type ScriptList<'a> = RecordList<'a, Script<'a>>;
/// A list of [`LanguageSystem`] records.
pub type LanguageSystemList<'a> = RecordList<'a, LanguageSystem<'a>>;
/// A list of [`Feature`] records.
pub type FeatureList<'a> = RecordList<'a, Feature<'a>>;

#[derive(Clone, Copy, Debug)]
struct TagRecord {
    tag: Tag,
    offset: Offset16,
}

impl FromData for TagRecord {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            tag: s.read::<Tag>()?,
            offset: s.read::<Offset16>()?,
        })
    }
}

/// A [Script Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#script-table-and-language-system-record).
#[derive(Clone, Copy, Debug)]
pub struct Script<'a> {
    /// Script tag.
    pub tag: Tag,
    /// Default language.
    pub default_language: Option<LanguageSystem<'a>>,
    /// List of supported languages, excluding the default one. Listed alphabetically.
    pub languages: LanguageSystemList<'a>,
}

impl<'a> RecordListItem<'a> for Script<'a> {
    fn parse(tag: Tag, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let mut default_language = None;
        if let Some(offset) = s.read::<Option<Offset16>>()? {
            default_language =
                LanguageSystem::parse(Tag::from_bytes(b"dflt"), data.get(offset.to_usize()..)?);
        }
        let mut languages = RecordList::parse(s.tail()?)?;
        // Offsets are relative to this table.
        languages.data = data;
        Some(Self {
            tag,
            default_language,
            languages,
        })
    }
}

/// A [Language System Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#language-system-table).
#[derive(Clone, Copy, Debug)]
pub struct LanguageSystem<'a> {
    /// Language tag.
    pub tag: Tag,
    /// Index of a feature required for this language system.
    pub required_feature: Option<FeatureIndex>,
    /// Array of indices into the FeatureList, in arbitrary order.
    pub feature_indices: LazyArray16<'a, FeatureIndex>,
}

impl<'a> RecordListItem<'a> for LanguageSystem<'a> {
    fn parse(tag: Tag, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let _lookup_order = s.read::<Offset16>()?; // Unsupported.
        let required_feature = match s.read::<FeatureIndex>()? {
            0xFFFF => None,
            v => Some(v),
        };
        let count = s.read::<u16>()?;
        let feature_indices = s.read_array16(count)?;
        Some(Self {
            tag,
            required_feature,
            feature_indices,
        })
    }
}

/// A [Feature](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#feature-table).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Feature<'a> {
    pub tag: Tag,
    pub lookup_indices: LazyArray16<'a, LookupIndex>,
}

impl<'a> RecordListItem<'a> for Feature<'a> {
    fn parse(tag: Tag, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let _params_offset = s.read::<Offset16>()?; // Unsupported.
        let count = s.read::<u16>()?;
        let lookup_indices = s.read_array16(count)?;
        Some(Self {
            tag,
            lookup_indices,
        })
    }
}
