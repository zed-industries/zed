//! Incremental Font Transfer [Patch Map](https://w3c.github.io/IFT/Overview.html#font-format-extensions)

include!("../../generated/generated_ift.rs");

use std::str;

pub const IFT_TAG: types::Tag = Tag::new(b"IFT ");
pub const IFTX_TAG: types::Tag = Tag::new(b"IFTX");

/// Wrapper for the packed childEntryMatchModeAndCount field in IFT format 2 mapping table.
///
/// Reference: <https://w3c.github.io/IFT/Overview.html#mapping-entry-childentrymatchmodeandcount>
///
/// The MSB is a flag which indicates conjunctive (bit set) or disjunctive (bit cleared) matching.
/// The remaining 7 bits are a count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchModeAndCount(u8);

impl MatchModeAndCount {
    /// Flag indicating that copy mode is append.
    ///
    /// See: <https://w3c.github.io/IFT/Overview.html#mapping-entry-copymodeandcount>
    pub const MATCH_MODE_MASK: u8 = 0b10000000;

    /// Mask for the low 7 bits to give the copy index count.
    pub const COUNT_MASK: u8 = 0b01111111;

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// If true matching mode is conjunctive (... AND ...) otherwise disjunctive (... OR ...)
    pub fn conjunctive_match(self) -> bool {
        (self.0 & Self::MATCH_MODE_MASK) != 0
    }

    pub fn count(self) -> u8 {
        self.0 & Self::COUNT_MASK
    }
}

impl TryFrom<MatchModeAndCount> for usize {
    type Error = ReadError;

    fn try_from(value: MatchModeAndCount) -> Result<Self, Self::Error> {
        Ok(value.count() as usize)
    }
}

impl types::Scalar for MatchModeAndCount {
    type Raw = <u8 as types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.0.to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u8>::from_raw(raw);
        Self(t)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default, Ord, PartialOrd, Hash)]
pub struct CompatibilityId([u8; 16]);

impl CompatibilityId {
    pub fn new(value: [u8; 16]) -> Self {
        CompatibilityId(value)
    }

    pub fn from_u32s(values: [u32; 4]) -> Self {
        let mut data = [0u8; 16];

        for i in 0..4 {
            let be_bytes = values[i].to_be_bytes();
            for j in 0..4 {
                data[i * 4 + j] = be_bytes[j];
            }
        }

        CompatibilityId(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Scalar for CompatibilityId {
    type Raw = [u8; 16];

    fn from_raw(raw: Self::Raw) -> Self {
        CompatibilityId(raw)
    }

    fn to_raw(self) -> Self::Raw {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U8Or16(u16);

impl ReadArgs for U8Or16 {
    type Args = u16;
}

impl ComputeSize for U8Or16 {
    fn compute_size(max_entry_index: &u16) -> Result<usize, ReadError> {
        Ok(if *max_entry_index < 256 { 1 } else { 2 })
    }
}

impl FontReadWithArgs<'_> for U8Or16 {
    fn read_with_args(data: FontData<'_>, max_entry_index: &Self::Args) -> Result<Self, ReadError> {
        if *max_entry_index < 256 {
            data.read_at::<u8>(0).map(|v| Self(v as u16))
        } else {
            data.read_at::<u16>(0).map(Self)
        }
    }
}

impl U8Or16 {
    #[inline]
    pub fn get(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U16Or24(u32);

impl ReadArgs for U16Or24 {
    type Args = GlyphKeyedFlags;
}

impl ComputeSize for U16Or24 {
    fn compute_size(flags: &GlyphKeyedFlags) -> Result<usize, ReadError> {
        // See: https://w3c.github.io/IFT/Overview.html#glyph-keyed-patch-flags
        Ok(if flags.contains(GlyphKeyedFlags::WIDE_GLYPH_IDS) {
            3
        } else {
            2
        })
    }
}

impl FontReadWithArgs<'_> for U16Or24 {
    fn read_with_args(data: FontData<'_>, flags: &Self::Args) -> Result<Self, ReadError> {
        if flags.contains(GlyphKeyedFlags::WIDE_GLYPH_IDS) {
            data.read_at::<Uint24>(0).map(|v| Self(v.to_u32()))
        } else {
            data.read_at::<u16>(0).map(|v| Self(v as u32))
        }
    }
}

impl U16Or24 {
    #[inline]
    pub fn get(self) -> u32 {
        self.0
    }
}

impl<'a> PatchMapFormat1<'a> {
    pub fn gid_to_entry_iter(&'a self) -> impl Iterator<Item = (GlyphId, u16)> + 'a {
        GidToEntryIter {
            glyph_map: self.glyph_map().ok(),
            glyph_count: self.glyph_count().to_u32(),
            gid: self
                .glyph_map()
                .map(|glyph_map| glyph_map.first_mapped_glyph() as u32)
                .unwrap_or(0),
        }
        .filter(|(_, entry_index)| *entry_index > 0)
    }

    pub fn entry_count(&self) -> u32 {
        self.max_entry_index() as u32 + 1
    }

    pub fn is_entry_applied(&self, entry_index: u16) -> bool {
        let byte_index = entry_index / 8;
        let bit_mask = 1 << (entry_index % 8);
        self.applied_entries_bitmap()
            .get(byte_index as usize)
            .map(|byte| byte & bit_mask != 0)
            .unwrap_or(false)
    }
}

impl FeatureMap<'_> {
    pub fn entry_records_size(&self, max_entry_index: u16) -> Result<usize, ReadError> {
        let field_width = if max_entry_index < 256 { 1 } else { 2 };
        let mut num_bytes = 0usize;
        for record in self.feature_records().iter() {
            num_bytes += record?.entry_map_count().get() as usize * field_width * 2;
        }
        Ok(num_bytes)
    }
}

struct GidToEntryIter<'a> {
    glyph_map: Option<GlyphMap<'a>>,
    glyph_count: u32,
    gid: u32,
}

impl Iterator for GidToEntryIter<'_> {
    type Item = (GlyphId, u16);

    fn next(&mut self) -> Option<Self::Item> {
        let glyph_map = self.glyph_map.as_ref()?;

        let cur_gid = self.gid;
        self.gid += 1;

        if cur_gid >= self.glyph_count {
            return None;
        }

        let index = cur_gid as usize - glyph_map.first_mapped_glyph() as usize;
        glyph_map
            .entry_index()
            .get(index)
            .ok()
            .map(|entry_index| (cur_gid.into(), entry_index.0))
    }
}

impl<'a> GlyphPatches<'a> {
    /// Returns an iterator over the per glyph data for the table with the given index.
    pub fn glyph_data_for_table(
        &'a self,
        table_index: usize,
    ) -> impl Iterator<Item = Result<(GlyphId, &'a [u8]), ReadError>> {
        let glyph_count = self.glyph_count() as usize;
        let start_index = table_index * glyph_count;
        let start_it = self.glyph_data_offsets().iter().skip(start_index);
        let end_it = self.glyph_data_offsets().iter().skip(start_index + 1);
        let glyphs = self.glyph_ids().iter().take(glyph_count);

        let it = glyphs.zip(start_it.zip(end_it)).map(|(gid, (start, end))| {
            let start = start.get();
            let end = end.get();
            (gid, start, end)
        });
        GlyphDataIterator {
            patches: self,
            offset_iterator: it,
            previous_gid: None,
            failed: false,
        }
    }
}

/// Custom iterator for glyph keyed glyph data which allows us to terminate the iterator on the first error.
struct GlyphDataIterator<'a, T>
where
    T: Iterator<Item = (Result<U16Or24, ReadError>, Offset32, Offset32)>,
{
    patches: &'a GlyphPatches<'a>,
    offset_iterator: T,
    previous_gid: Option<GlyphId>,
    failed: bool,
}

impl<'a, T> Iterator for GlyphDataIterator<'a, T>
where
    T: Iterator<Item = (Result<U16Or24, ReadError>, Offset32, Offset32)>,
{
    type Item = Result<(GlyphId, &'a [u8]), ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        let (gid, start, end) = self.offset_iterator.next()?;
        let gid = match gid {
            Ok(gid) => GlyphId::new(gid.get()),
            Err(err) => {
                self.failed = true;
                return Some(Err(err));
            }
        };

        if let Some(previous_gid) = self.previous_gid {
            if gid <= previous_gid {
                self.failed = true;
                return Some(Err(ReadError::MalformedData(
                    "Glyph IDs are unsorted or duplicated.",
                )));
            }
        }
        self.previous_gid = Some(gid);

        let len = match end
            .to_u32()
            .checked_sub(start.to_u32())
            .ok_or(ReadError::MalformedData(
                "glyph data offsets are not ascending.",
            )) {
            Ok(len) => len as usize,
            Err(err) => {
                self.failed = true;
                return Some(Err(err));
            }
        };

        let data: Result<GlyphData<'a>, ReadError> = self.patches.resolve_offset(start);
        let data = match data {
            Ok(data) => data.data,
            Err(err) => {
                self.failed = true;
                return Some(Err(err));
            }
        };

        let Some(data) = data.as_bytes().get(..len) else {
            self.failed = true;
            return Some(Err(ReadError::OutOfBounds));
        };

        Some(Ok((gid, data)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::ift as test_data;

    // TODO(garretrieger) - more tests (as functionality is implemented):
    // - Test where entryIndex array has len 0 (eg. all glyphs map to 0)
    // - Test which appliedEntriesBitmap > 1 byte
    // - Test w/ feature map populated.
    // - Test enforced minimum entry count of > 0.
    // - Test where entryIndex is a u16.
    // - Invalid table (too short).
    // - Invalid UTF8 sequence in url template.
    // - Compat ID is to short.
    // - invalid entry map array (too short)
    // - feature map with short entry indices.

    #[test]
    fn format_1_gid_to_u8_entry_iter() {
        let data = test_data::simple_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };
        let entries: Vec<(GlyphId, u16)> = map.gid_to_entry_iter().collect();

        assert_eq!(
            entries,
            vec![(1u32.into(), 2), (2u32.into(), 1), (4u32.into(), 1)]
        );
    }

    #[test]
    fn format_1_gid_to_u16_entry_iter() {
        let data = test_data::u16_entries_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };
        let entries: Vec<(GlyphId, u16)> = map.gid_to_entry_iter().collect();

        assert_eq!(
            entries,
            vec![
                (2u32.into(), 0x50),
                (3u32.into(), 0x51),
                (4u32.into(), 0x12c),
                (5u32.into(), 0x12c),
                (6u32.into(), 0x50)
            ]
        );
    }

    #[test]
    fn format_1_feature_map() {
        let data = test_data::feature_map_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };

        let Some(feature_map_result) = map.feature_map() else {
            panic!("should have a non null feature map.");
        };

        let Ok(feature_map) = feature_map_result else {
            panic!("should have a valid feature map.");
        };

        assert_eq!(feature_map.feature_records().len(), 3);

        let fr0 = feature_map.feature_records().get(0).unwrap();
        assert_eq!(fr0.feature_tag(), Tag::new(b"dlig"));
        assert_eq!(*fr0.first_new_entry_index(), U8Or16(0x190));
        assert_eq!(*fr0.entry_map_count(), U8Or16(0x01));

        let fr1 = feature_map.feature_records().get(1).unwrap();
        assert_eq!(fr1.feature_tag(), Tag::new(b"liga"));
        assert_eq!(*fr1.first_new_entry_index(), U8Or16(0x180));
        assert_eq!(*fr1.entry_map_count(), U8Or16(0x02));
    }

    #[test]
    fn format_1_get_charstrings_offset() {
        // No offsets
        let data = test_data::simple_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };

        assert_eq!(map.cff_charstrings_offset(), None);
        assert_eq!(map.cff2_charstrings_offset(), None);

        // One offset
        let data = test_data::simple_format1_with_one_charstrings_offset();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };

        assert_eq!(map.cff_charstrings_offset(), Some(456));
        assert_eq!(map.cff2_charstrings_offset(), None);

        // Two offsets
        let data = test_data::simple_format1_with_two_charstrings_offsets();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };

        assert_eq!(map.cff_charstrings_offset(), Some(456));
        assert_eq!(map.cff2_charstrings_offset(), Some(789));
    }

    #[test]
    fn format_2_get_charstrings_offset() {
        // No offsets
        let data = test_data::codepoints_only_format2();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format2(map) = table else {
            panic!("Not format 2.");
        };
        assert_eq!(map.cff_charstrings_offset(), None);
        assert_eq!(map.cff2_charstrings_offset(), None);

        // One offset
        let data = test_data::format2_with_one_charstrings_offset();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format2(map) = table else {
            panic!("Not format 2.");
        };

        assert_eq!(map.cff_charstrings_offset(), Some(456));
        assert_eq!(map.cff2_charstrings_offset(), None);

        // Two offsets
        let data = test_data::format2_with_two_charstrings_offset();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format2(map) = table else {
            panic!("Not format 2.");
        };

        assert_eq!(map.cff_charstrings_offset(), Some(456));
        assert_eq!(map.cff2_charstrings_offset(), Some(789));
    }

    #[test]
    fn invalid_format_number() {
        // No offsets
        let mut data = test_data::codepoints_only_format2();
        data.write_at("format", 3u8);

        let Err(err) = Ift::read(FontData::new(&data)) else {
            panic!("Read should have failed due to invalid format number.");
        };

        assert_eq!(ReadError::InvalidFormat(3), err);
    }

    #[test]
    fn compatibility_id() {
        let data = test_data::simple_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };

        assert_eq!(
            map.compatibility_id(),
            CompatibilityId::from_u32s([1, 2, 3, 4])
        );
    }

    #[test]
    fn is_entry_applied() {
        let data = test_data::simple_format1();
        let table = Ift::read(FontData::new(&data)).unwrap();
        let Ift::Format1(map) = table else {
            panic!("Not format 1.");
        };
        assert!(!map.is_entry_applied(0));
        assert!(map.is_entry_applied(1));
        assert!(!map.is_entry_applied(2));
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table() {
        let data = test_data::glyf_u16_glyph_patches();
        let table = GlyphPatches::read(FontData::new(&data), GlyphKeyedFlags::NONE).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Ok((GlyphId::new(8), b"".as_slice())),
                Ok((GlyphId::new(9), b"hijkl".as_slice())),
                Ok((GlyphId::new(13), b"mn".as_slice())),
            ]
        );

        assert_eq!(table.glyph_data_for_table(1).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table_u24_ids() {
        let data = test_data::glyf_u24_glyph_patches();
        let table =
            GlyphPatches::read(FontData::new(&data), GlyphKeyedFlags::WIDE_GLYPH_IDS).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Ok((GlyphId::new(8), b"".as_slice())),
                Ok((GlyphId::new(9), b"hijkl".as_slice())),
                Ok((GlyphId::new(13), b"mn".as_slice())),
            ]
        );

        assert_eq!(table.glyph_data_for_table(1).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn glyph_keyed_glyph_data_for_multiple_tables() {
        let data = test_data::glyf_and_gvar_u16_glyph_patches();
        let table = GlyphPatches::read(FontData::new(&data), GlyphKeyedFlags::NONE).unwrap();

        assert_eq!(
            table.glyph_data_for_table(0).collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Ok((GlyphId::new(8), b"hijkl".as_slice())),
            ]
        );

        assert_eq!(
            table.glyph_data_for_table(1).collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"mn".as_slice())),
                Ok((GlyphId::new(7), b"opq".as_slice())),
                Ok((GlyphId::new(8), b"r".as_slice())),
            ]
        );

        assert_eq!(table.glyph_data_for_table(2).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn glyph_keyed_glyph_data_non_ascending_gids() {
        let mut builder = test_data::glyf_u16_glyph_patches();
        builder.write_at("gid_8", 6);
        let table =
            GlyphPatches::read(FontData::new(builder.as_slice()), GlyphKeyedFlags::NONE).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Err(ReadError::MalformedData(
                    "Glyph IDs are unsorted or duplicated."
                )),
            ]
        );
    }

    #[test]
    fn glyph_keyed_glyph_data_duplicate_gids() {
        let mut builder = test_data::glyf_u16_glyph_patches();
        builder.write_at("gid_8", 7);
        let table =
            GlyphPatches::read(FontData::new(builder.as_slice()), GlyphKeyedFlags::NONE).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Err(ReadError::MalformedData(
                    "Glyph IDs are unsorted or duplicated."
                )),
            ]
        );
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table_non_ascending_offsets() {
        let mut builder = test_data::glyf_u16_glyph_patches();
        let gid_13 = builder.offset_for("gid_13_data") as u32;
        let gid_9 = builder.offset_for("gid_8_and_9_data") as u32;
        builder.write_at("gid_13_offset", gid_9);
        builder.write_at("gid_9_offset", gid_13);

        let table =
            GlyphPatches::read(FontData::new(builder.as_slice()), GlyphKeyedFlags::NONE).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Ok((GlyphId::new(8), b"hijkl".as_slice())),
                Err(ReadError::MalformedData(
                    "glyph data offsets are not ascending."
                )),
            ]
        );

        assert_eq!(table.glyph_data_for_table(1).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table_gids_truncated() {
        let builder = test_data::glyf_u16_glyph_patches();
        let len = builder.offset_for("table_count");
        let data = &builder.as_slice()[..len];

        let Err(err) = GlyphPatches::read(FontData::new(data), GlyphKeyedFlags::NONE) else {
            panic!("Expected to fail.");
        };
        assert_eq!(ReadError::OutOfBounds, err);
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table_data_truncated() {
        let builder = test_data::glyf_u16_glyph_patches();
        let len = builder.offset_for("gid_8_and_9_data");
        let data = &builder.as_slice()[..len];

        let table = GlyphPatches::read(FontData::new(data), GlyphKeyedFlags::NONE).unwrap();

        let it = table.glyph_data_for_table(0);

        assert_eq!(
            it.collect::<Vec<_>>(),
            vec![
                Ok((GlyphId::new(2), b"abc".as_slice())),
                Ok((GlyphId::new(7), b"defg".as_slice())),
                Ok((GlyphId::new(8), b"".as_slice())),
                Err(ReadError::OutOfBounds),
            ]
        );
    }

    #[test]
    fn glyph_keyed_glyph_data_for_one_table_offset_array_truncated() {
        let builder = test_data::glyf_u16_glyph_patches();
        let len = builder.offset_for("gid_9_offset");
        let data = &builder.as_slice()[..len];

        let Err(err) = GlyphPatches::read(FontData::new(data), GlyphKeyedFlags::NONE) else {
            panic!("Expected to fail.");
        };
        assert_eq!(ReadError::OutOfBounds, err);
    }
}
