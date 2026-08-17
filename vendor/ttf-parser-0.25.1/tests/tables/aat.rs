use std::num::NonZeroU16;
use ttf_parser::GlyphId;
use ttf_parser::apple_layout::Lookup;
use crate::{convert, Unit::*};

mod format0 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(0), // format
            UInt16(10), // value
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 10);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn not_enough_glyphs() {
        let data = convert(&[
            UInt16(0), // format
            UInt16(10), // value
        ]);

        assert!(Lookup::parse(NonZeroU16::new(2).unwrap(), &data).is_none());
    }

    #[test]
    fn too_many_glyphs() {
        let data = convert(&[
            UInt16(0), // format
            UInt16(10), // value
            UInt16(11), // value <-- will be ignored
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 10);
        assert!(table.value(GlyphId(1)).is_none());
    }
}

mod format2 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(2), // format

            // Binary Search Table
            UInt16(6), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(118), // last glyph
            UInt16(118), // first glyph
            UInt16(10), // value
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(118)).unwrap(), 10);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn range() {
        let data = convert(&[
            UInt16(2), // format

            // Binary Search Table
            UInt16(6), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(7), // last glyph
            UInt16(5), // first glyph
            UInt16(18), // offset
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert!(table.value(GlyphId(4)).is_none());
        assert_eq!(table.value(GlyphId(5)).unwrap(), 18);
        assert_eq!(table.value(GlyphId(6)).unwrap(), 18);
        assert_eq!(table.value(GlyphId(7)).unwrap(), 18);
        assert!(table.value(GlyphId(8)).is_none());
    }
}

mod format4 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(4), // format

            // Binary Search Table
            UInt16(6), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(118), // last glyph
            UInt16(118), // first glyph
            UInt16(18), // offset

            // Values [0]
            UInt16(10), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(118)).unwrap(), 10);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn range() {
        let data = convert(&[
            UInt16(4), // format

            // Binary Search Table
            UInt16(6), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(7), // last glyph
            UInt16(5), // first glyph
            UInt16(18), // offset

            // Values [0]
            UInt16(10), // value [0]
            UInt16(11), // value [1]
            UInt16(12), // value [2]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert!(table.value(GlyphId(4)).is_none());
        assert_eq!(table.value(GlyphId(5)).unwrap(), 10);
        assert_eq!(table.value(GlyphId(6)).unwrap(), 11);
        assert_eq!(table.value(GlyphId(7)).unwrap(), 12);
        assert!(table.value(GlyphId(8)).is_none());
    }
}

mod format6 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(4), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(0), // glyph
            UInt16(10), // value
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 10);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn multiple() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(4), // segment size
            UInt16(3), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(0), // glyph
            UInt16(10), // value
            // Segment [1]
            UInt16(5), // glyph
            UInt16(20), // value
            // Segment [2]
            UInt16(10), // glyph
            UInt16(30), // value
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 10);
        assert_eq!(table.value(GlyphId(5)).unwrap(), 20);
        assert_eq!(table.value(GlyphId(10)).unwrap(), 30);
        assert!(table.value(GlyphId(1)).is_none());
    }

    // Tests below are indirectly testing BinarySearchTable.

    #[test]
    fn no_segments() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(4), // segment size
            UInt16(0), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it
        ]);

        assert!(Lookup::parse(NonZeroU16::new(1).unwrap(), &data).is_none());
    }

    #[test]
    fn ignore_termination() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(4), // segment size
            UInt16(2), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(0), // glyph
            UInt16(10), // value
            // Segment [1]
            UInt16(0xFFFF), // glyph
            UInt16(0xFFFF), // value
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert!(table.value(GlyphId(0xFFFF)).is_none());
    }

    #[test]
    fn only_termination() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(4), // segment size
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(0xFFFF), // glyph
            UInt16(0xFFFF), // value
        ]);

        assert!(Lookup::parse(NonZeroU16::new(1).unwrap(), &data).is_none());
    }

    #[test]
    fn invalid_segment_size() {
        let data = convert(&[
            UInt16(6), // format

            // Binary Search Table
            UInt16(8), // segment size <-- must be 4
            UInt16(1), // number of segments
            UInt16(0), // search range: we don't use it
            UInt16(0), // entry selector: we don't use it
            UInt16(0), // range shift: we don't use it

            // Segment [0]
            UInt16(0), // glyph
            UInt16(10), // value
        ]);

        assert!(Lookup::parse(NonZeroU16::new(1).unwrap(), &data).is_none());
    }
}

mod format8 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(8), // format
            UInt16(0), // first glyph
            UInt16(1), // glyphs count
            UInt16(2), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 2);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn non_zero_first() {
        let data = convert(&[
            UInt16(8), // format
            UInt16(5), // first glyph
            UInt16(1), // glyphs count
            UInt16(2), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(5)).unwrap(), 2);
        assert!(table.value(GlyphId(1)).is_none());
        assert!(table.value(GlyphId(6)).is_none());
    }
}

mod format10 {
    use super::*;

    #[test]
    fn single() {
        let data = convert(&[
            UInt16(10), // format
            UInt16(1), // value size: u8
            UInt16(0), // first glyph
            UInt16(1), // glyphs count
            UInt8(2), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 2);
        assert!(table.value(GlyphId(1)).is_none());
    }

    #[test]
    fn invalid_value_size() {
        let data = convert(&[
            UInt16(10), // format
            UInt16(50), // value size <-- invalid
            UInt16(0), // first glyph
            UInt16(1), // glyphs count
            UInt8(2), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert!(table.value(GlyphId(0)).is_none());
    }

    #[test]
    fn unsupported_value_size() {
        let data = convert(&[
            UInt16(10), // format
            UInt16(8), // value size <-- we do not support u64
            UInt16(0), // first glyph
            UInt16(1), // glyphs count
            Raw(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]), // value [0]
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert!(table.value(GlyphId(0)).is_none());
    }

    #[test]
    fn u32_value_size() {
        let data = convert(&[
            UInt16(10), // format
            UInt16(4), // value size
            UInt16(0), // first glyph
            UInt16(1), // glyphs count
            UInt32(0xFFFF + 10), // value [0] <-- will be truncated
        ]);

        let table = Lookup::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
        assert_eq!(table.value(GlyphId(0)).unwrap(), 9);
    }
}
