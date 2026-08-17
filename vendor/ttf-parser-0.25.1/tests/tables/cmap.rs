mod format0 {
    use ttf_parser::{cmap, GlyphId};
    use crate::{convert, Unit::*};

    #[test]
    fn maps_not_all_256_codepoints() {
        let mut data = convert(&[
            UInt16(0), // format
            UInt16(262), // subtable size
            UInt16(0), // language ID
        ]);

        // Map (only) codepoint 0x40 to 100.
        data.extend(std::iter::repeat(0).take(256));
        data[6 + 0x40] = 100;

        let subtable = cmap::Subtable0::parse(&data).unwrap();

        assert_eq!(subtable.glyph_index(0), None);
        assert_eq!(subtable.glyph_index(0x40), Some(GlyphId(100)));
        assert_eq!(subtable.glyph_index(100), None);

        let mut vec = vec![];
        subtable.codepoints(|c| vec.push(c));
        assert_eq!(vec, [0x40]);
    }
}

mod format2 {
    use ttf_parser::{cmap, GlyphId};
    use crate::{convert, Unit::*};

    const U16_SIZE: usize = std::mem::size_of::<u16>();

    #[test]
    fn collect_codepoints() {
        let mut data = convert(&[
            UInt16(2), // format
            UInt16(534), // subtable size
            UInt16(0), // language ID
        ]);

        // Make only high byte 0x28 multi-byte.
        data.extend(std::iter::repeat(0x00).take(256 * U16_SIZE));
        data[6 + 0x28 * U16_SIZE + 1] = 0x08;

        data.extend(convert(&[
            // First sub header (for single byte mapping)
            UInt16(254), // first code
            UInt16(2), // entry count
            UInt16(0), // id delta: uninteresting
            UInt16(0), // id range offset: uninteresting
            // Second sub header (for high byte 0x28)
            UInt16(16), // first code: (0x28 << 8) + 0x10 = 10256
            UInt16(3), // entry count
            UInt16(0), // id delta: uninteresting
            UInt16(0), // id range offset: uninteresting
        ]));

        // Now only glyph ID's would follow. Not interesting for codepoints.

        let subtable = cmap::Subtable2::parse(&data).unwrap();

        let mut vec = vec![];
        subtable.codepoints(|c| vec.push(c));
        assert_eq!(vec, [10256, 10257, 10258, 254, 255]);
    }

    #[test]
    fn codepoint_at_range_end() {
        let mut data = convert(&[
            UInt16(2), // format
            UInt16(532), // subtable size
            UInt16(0), // language ID
        ]);

        // Only single bytes.
        data.extend(std::iter::repeat(0x00).take(256 * U16_SIZE));
        data.extend(convert(&[
            // First sub header (for single byte mapping)
            UInt16(40), // first code
            UInt16(2), // entry count
            UInt16(0), // id delta
            UInt16(2), // id range offset
            // Glyph index
            UInt16(100), // glyph ID [0]
            UInt16(1000), // glyph ID [1]
            UInt16(10000), // glyph ID [2] (unused)
        ]));

        let subtable = cmap::Subtable2::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(39), None);
        assert_eq!(subtable.glyph_index(40), Some(GlyphId(100)));
        assert_eq!(subtable.glyph_index(41), Some(GlyphId(1000)));
        assert_eq!(subtable.glyph_index(42), None);
    }
}

mod format4 {
    use ttf_parser::{cmap, GlyphId};
    use crate::{convert, Unit::*};

    #[test]
    fn single_glyph() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(32), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x41), Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x42), None);
    }

    #[test]
    fn continuous_range() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(32), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(73), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x40), None);
        assert_eq!(subtable.glyph_index(0x41), Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x42), Some(GlyphId(2)));
        assert_eq!(subtable.glyph_index(0x43), Some(GlyphId(3)));
        assert_eq!(subtable.glyph_index(0x44), Some(GlyphId(4)));
        assert_eq!(subtable.glyph_index(0x45), Some(GlyphId(5)));
        assert_eq!(subtable.glyph_index(0x46), Some(GlyphId(6)));
        assert_eq!(subtable.glyph_index(0x47), Some(GlyphId(7)));
        assert_eq!(subtable.glyph_index(0x48), Some(GlyphId(8)));
        assert_eq!(subtable.glyph_index(0x49), Some(GlyphId(9)));
        assert_eq!(subtable.glyph_index(0x4A), None);
    }

    #[test]
    fn multiple_ranges() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(48), // subtable size
            UInt16(0), // language ID
            UInt16(8), // 2 x segCount
            UInt16(4), // search range
            UInt16(1), // entry selector
            UInt16(4), // range shift
            // End character codes
            UInt16(65), // char code [0]
            UInt16(69), // char code [1]
            UInt16(73), // char code [2]
            UInt16(65535), // char code [3]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(67), // char code [1]
            UInt16(71), // char code [2]
            UInt16(65535), // char code [3]
            // Deltas
            Int16(-64), // delta [0]
            Int16(-65), // delta [1]
            Int16(-66), // delta [2]
            Int16(1), // delta [3]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
            UInt16(0), // offset [2]
            UInt16(0), // offset [3]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x40), None);
        assert_eq!(subtable.glyph_index(0x41), Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x42), None);
        assert_eq!(subtable.glyph_index(0x43), Some(GlyphId(2)));
        assert_eq!(subtable.glyph_index(0x44), Some(GlyphId(3)));
        assert_eq!(subtable.glyph_index(0x45), Some(GlyphId(4)));
        assert_eq!(subtable.glyph_index(0x46), None);
        assert_eq!(subtable.glyph_index(0x47), Some(GlyphId(5)));
        assert_eq!(subtable.glyph_index(0x48), Some(GlyphId(6)));
        assert_eq!(subtable.glyph_index(0x49), Some(GlyphId(7)));
        assert_eq!(subtable.glyph_index(0x4A), None);
    }

    #[test]
    fn unordered_ids() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(42), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(69), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(0), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(4), // offset [0]
            UInt16(0), // offset [1]
            // Glyph index array
            UInt16(1), // glyph ID [0]
            UInt16(10), // glyph ID [1]
            UInt16(100), // glyph ID [2]
            UInt16(1000), // glyph ID [3]
            UInt16(10000), // glyph ID [4]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x40), None);
        assert_eq!(subtable.glyph_index(0x41), Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x42), Some(GlyphId(10)));
        assert_eq!(subtable.glyph_index(0x43), Some(GlyphId(100)));
        assert_eq!(subtable.glyph_index(0x44), Some(GlyphId(1000)));
        assert_eq!(subtable.glyph_index(0x45), Some(GlyphId(10000)));
        assert_eq!(subtable.glyph_index(0x46), None);
    }

    #[test]
    fn unordered_chars_and_ids() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(64), // subtable size
            UInt16(0), // language ID
            UInt16(12), // 2 x segCount
            UInt16(8), // search range
            UInt16(2), // entry selector
            UInt16(4), // range shift
            // End character codes
            UInt16(80), // char code [0]
            UInt16(256), // char code [1]
            UInt16(336), // char code [2]
            UInt16(512), // char code [3]
            UInt16(592), // char code [4]
            UInt16(65535), // char code [5]
            UInt16(0), // reserved
            // Start character codes
            UInt16(80), // char code [0]
            UInt16(256), // char code [1]
            UInt16(336), // char code [2]
            UInt16(512), // char code [3]
            UInt16(592), // char code [4]
            UInt16(65535), // char code [5]
            // Deltas
            Int16(-79), // delta [0]
            Int16(-246), // delta [1]
            Int16(-236), // delta [2]
            Int16(488), // delta [3]
            Int16(9408), // delta [4]
            Int16(1), // delta [5]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
            UInt16(0), // offset [2]
            UInt16(0), // offset [3]
            UInt16(0), // offset [4]
            UInt16(0), // offset [5]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x40),  None);
        assert_eq!(subtable.glyph_index(0x50),  Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x100), Some(GlyphId(10)));
        assert_eq!(subtable.glyph_index(0x150), Some(GlyphId(100)));
        assert_eq!(subtable.glyph_index(0x200), Some(GlyphId(1000)));
        assert_eq!(subtable.glyph_index(0x250), Some(GlyphId(10000)));
        assert_eq!(subtable.glyph_index(0x300), None);
    }

    #[test]
    fn no_end_codes() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(28), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(73), // char code [0]
            // 0xFF, 0xFF, // char code [1] <-- removed
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            // 0xFF, 0xFF, // char code [1] <-- removed
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        assert!(cmap::Subtable4::parse(&data).is_none());
    }

    #[test]
    fn invalid_segment_count() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(32), // subtable size
            UInt16(0), // language ID
            UInt16(1), // 2 x segCount <-- must be more than 1
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        assert!(cmap::Subtable4::parse(&data).is_none());
    }

    #[test]
    fn only_end_segments() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(32), // subtable size
            UInt16(0), // language ID
            UInt16(2), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        // Should not loop forever.
        assert_eq!(subtable.glyph_index(0x41), None);
    }

    #[test]
    fn invalid_length() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(16), // subtable size <-- the size should be 32, but we don't check it anyway
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x41), Some(GlyphId(1)));
        assert_eq!(subtable.glyph_index(0x42), None);
    }

    #[test]
    fn codepoint_out_of_range() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(32), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(-64), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(0), // offset [0]
            UInt16(0), // offset [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        // Format 4 support only u16 codepoints, so we have to bail immediately otherwise.
        assert_eq!(subtable.glyph_index(0x1FFFF), None);
    }

    #[test]
    fn zero() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(42), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(69), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(0), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(4), // offset [0]
            UInt16(0), // offset [1]
            // Glyph index array
            UInt16(0), // glyph ID [0] <-- indicates missing glyph
            UInt16(10), // glyph ID [1]
            UInt16(100), // glyph ID [2]
            UInt16(1000), // glyph ID [3]
            UInt16(10000), // glyph ID [4]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(0x41), None);
    }

    #[test]
    fn invalid_offset() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(42), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(69), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(65), // char code [0]
            UInt16(65535), // char code [1]
            // Deltas
            Int16(0), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(4), // offset [0]
            UInt16(65535), // offset [1]
            // Glyph index array
            UInt16(1), // glyph ID [0]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();
        assert_eq!(subtable.glyph_index(65535), None);
    }

    #[test]
    fn collect_codepoints() {
        let data = convert(&[
            UInt16(4), // format
            UInt16(24), // subtable size
            UInt16(0), // language ID
            UInt16(4), // 2 x segCount
            UInt16(2), // search range
            UInt16(0), // entry selector
            UInt16(2), // range shift
            // End character codes
            UInt16(34), // char code [0]
            UInt16(65535), // char code [1]
            UInt16(0), // reserved
            // Start character codes
            UInt16(27), // char code [0]
            UInt16(65533), // char code [1]
            // Deltas
            Int16(0), // delta [0]
            Int16(1), // delta [1]
            // Offsets into Glyph index array
            UInt16(4), // offset [0]
            UInt16(0), // offset [1]
            // Glyph index array
            UInt16(0), // glyph ID [0]
            UInt16(10), // glyph ID [1]
        ]);

        let subtable = cmap::Subtable4::parse(&data).unwrap();

        let mut vec = vec![];
        subtable.codepoints(|c| vec.push(c));
        assert_eq!(vec, [27, 28, 29, 30, 31, 32, 33, 34, 65533, 65534, 65535]);
    }
}
