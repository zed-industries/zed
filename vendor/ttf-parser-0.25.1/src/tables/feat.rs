//! A [Feature Name Table](
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6feat.html) implementation.

use crate::parser::{FromData, LazyArray16, Offset, Offset32, Stream};

#[derive(Clone, Copy, Debug)]
struct FeatureNameRecord {
    feature: u16,
    setting_table_records_count: u16,
    // Offset from the beginning of the table.
    setting_table_offset: Offset32,
    flags: u8,
    default_setting_index: u8,
    name_index: u16,
}

impl FromData for FeatureNameRecord {
    const SIZE: usize = 12;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(FeatureNameRecord {
            feature: s.read::<u16>()?,
            setting_table_records_count: s.read::<u16>()?,
            setting_table_offset: s.read::<Offset32>()?,
            flags: s.read::<u8>()?,
            default_setting_index: s.read::<u8>()?,
            name_index: s.read::<u16>()?,
        })
    }
}

/// A setting name.
#[derive(Clone, Copy, Debug)]
pub struct SettingName {
    /// The setting.
    pub setting: u16,
    /// The `name` table index for the feature's name in a 256..32768 range.
    pub name_index: u16,
}

impl FromData for SettingName {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(SettingName {
            setting: s.read::<u16>()?,
            name_index: s.read::<u16>()?,
        })
    }
}

/// A feature names.
#[derive(Clone, Copy, Debug)]
pub struct FeatureName<'a> {
    /// The feature's ID.
    pub feature: u16,
    /// The feature's setting names.
    pub setting_names: LazyArray16<'a, SettingName>,
    /// The index of the default setting in the `setting_names`.
    pub default_setting_index: u8,
    /// The feature's exclusive settings. If set, the feature settings are mutually exclusive.
    pub exclusive: bool,
    /// The `name` table index for the feature's name in a 256..32768 range.
    pub name_index: u16,
}

/// A list of feature names.
#[derive(Clone, Copy)]
pub struct FeatureNames<'a> {
    data: &'a [u8],
    records: LazyArray16<'a, FeatureNameRecord>,
}

impl<'a> FeatureNames<'a> {
    /// Returns a feature name at an index.
    pub fn get(&self, index: u16) -> Option<FeatureName<'a>> {
        let record = self.records.get(index)?;
        let data = self.data.get(record.setting_table_offset.to_usize()..)?;
        let mut s = Stream::new(data);
        let setting_names = s.read_array16::<SettingName>(record.setting_table_records_count)?;
        Some(FeatureName {
            feature: record.feature,
            setting_names,
            default_setting_index: if record.flags & 0x40 != 0 {
                record.default_setting_index
            } else {
                0
            },
            exclusive: record.flags & 0x80 != 0,
            name_index: record.name_index,
        })
    }

    /// Finds a feature name by ID.
    pub fn find(&self, feature: u16) -> Option<FeatureName<'a>> {
        let index = self
            .records
            .binary_search_by(|name| name.feature.cmp(&feature))
            .map(|(i, _)| i)?;
        self.get(index)
    }

    /// Returns the number of feature names.
    pub fn len(&self) -> u16 {
        self.records.len()
    }

    /// Checks if there are any feature names.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl<'a> core::fmt::Debug for FeatureNames<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a> IntoIterator for FeatureNames<'a> {
    type Item = FeatureName<'a>;
    type IntoIter = FeatureNamesIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        FeatureNamesIter {
            names: self,
            index: 0,
        }
    }
}

/// An iterator over [`FeatureNames`].
#[allow(missing_debug_implementations)]
pub struct FeatureNamesIter<'a> {
    names: FeatureNames<'a>,
    index: u16,
}

impl<'a> Iterator for FeatureNamesIter<'a> {
    type Item = FeatureName<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.names.len() {
            self.index += 1;
            self.names.get(self.index - 1)
        } else {
            None
        }
    }
}

/// A [Feature Name Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6feat.html).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of feature names. Sorted by `FeatureName.feature`.
    pub names: FeatureNames<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let count = s.read::<u16>()?;
        s.advance_checked(6)?; // reserved
        let records = s.read_array16::<FeatureNameRecord>(count)?;

        Some(Table {
            names: FeatureNames { data, records },
        })
    }
}
