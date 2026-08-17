//! The [MVAR (Metrics Variation)](https://docs.microsoft.com/en-us/typography/opentype/spec/mvar) table

use super::variations::{DeltaSetIndex, ItemVariationStore};

/// Four-byte tags used to represent particular metric or other values.
pub mod tags {
    use font_types::Tag;

    /// Horizontal ascender.
    pub const HASC: Tag = Tag::new(b"hasc");
    /// Horizontal descender.
    pub const HDSC: Tag = Tag::new(b"hdsc");
    /// Horizontal line gap.
    pub const HLGP: Tag = Tag::new(b"hlgp");

    /// Horizontal clipping ascent.
    pub const HCLA: Tag = Tag::new(b"hcla");
    /// Horizontal clipping descent.
    pub const HCLD: Tag = Tag::new(b"hcld");

    /// Vertical ascender.
    pub const VASC: Tag = Tag::new(b"vasc");
    /// Vertical descender.
    pub const VDSC: Tag = Tag::new(b"vdsc");
    /// Vertical line gap.
    pub const VLGP: Tag = Tag::new(b"vlgp");

    /// Horizontal caret rise.
    pub const HCRS: Tag = Tag::new(b"hcrs");
    /// Horizontal caret run.
    pub const HCRN: Tag = Tag::new(b"hcrn");
    /// Horizontal caret offset.
    pub const HCOF: Tag = Tag::new(b"hcof");

    /// Vertical caret rise.
    pub const VCRS: Tag = Tag::new(b"vcrs");
    /// Vertical caret run.
    pub const VCRN: Tag = Tag::new(b"vcrn");
    /// Vertical caret offset.
    pub const VCOF: Tag = Tag::new(b"vcof");

    /// X-height.
    pub const XHGT: Tag = Tag::new(b"xhgt");
    /// Cap height.
    pub const CPHT: Tag = Tag::new(b"cpht");

    /// Subscript em x-offset.
    pub const SBXO: Tag = Tag::new(b"sbxo");
    /// Subscript em y-offset.
    pub const SBYO: Tag = Tag::new(b"sbyo");
    /// Subscript em x-size.
    pub const SBXS: Tag = Tag::new(b"sbxs");
    /// Subscript em y-size.
    pub const SBYS: Tag = Tag::new(b"sbys");

    /// Superscript em x-offset.
    pub const SPXO: Tag = Tag::new(b"spxo");
    /// Superscript em y-offset.
    pub const SPYO: Tag = Tag::new(b"spyo");
    /// Superscript em x-size.
    pub const SPXS: Tag = Tag::new(b"spxs");
    /// Superscript em y-size.
    pub const SPYS: Tag = Tag::new(b"spys");

    /// Strikeout size.
    pub const STRS: Tag = Tag::new(b"strs");
    /// Strikeout offset.
    pub const STRO: Tag = Tag::new(b"stro");

    /// Underline size.
    pub const UNDS: Tag = Tag::new(b"unds");
    /// Underline offset.
    pub const UNDO: Tag = Tag::new(b"undo");

    /// GaspRange\[0\]
    pub const GSP0: Tag = Tag::new(b"gsp0");
    /// GaspRange\[1\]
    pub const GSP1: Tag = Tag::new(b"gsp1");
    /// GaspRange\[2\]
    pub const GSP2: Tag = Tag::new(b"gsp2");
    /// GaspRange\[3\]
    pub const GSP3: Tag = Tag::new(b"gsp3");
    /// GaspRange\[4\]
    pub const GSP4: Tag = Tag::new(b"gsp4");
    /// GaspRange\[5\]
    pub const GSP5: Tag = Tag::new(b"gsp5");
    /// GaspRange\[6\]
    pub const GSP6: Tag = Tag::new(b"gsp6");
    /// GaspRange\[7\]
    pub const GSP7: Tag = Tag::new(b"gsp7");
    /// GaspRange\[8\]
    pub const GSP8: Tag = Tag::new(b"gsp8");
    /// GaspRange\[9\]
    pub const GSP9: Tag = Tag::new(b"gsp9");
}

include!("../../generated/generated_mvar.rs");

impl Mvar<'_> {
    /// Returns the metric delta for the specified tag and normalized
    /// variation coordinates. Possible tags are found in the [tags]
    /// module.
    pub fn metric_delta(&self, tag: Tag, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        use std::cmp::Ordering;
        let records = self.value_records();
        let mut lo = 0;
        let mut hi = records.len();
        while lo < hi {
            let i = (lo + hi) / 2;
            let record = &records[i];
            match tag.cmp(&record.value_tag()) {
                Ordering::Less => {
                    hi = i;
                }
                Ordering::Greater => {
                    lo = i + 1;
                }
                Ordering::Equal => {
                    let ivs = self.item_variation_store().ok_or(ReadError::NullOffset)??;
                    return Ok(Fixed::from_i32(ivs.compute_delta(
                        DeltaSetIndex {
                            outer: record.delta_set_outer_index(),
                            inner: record.delta_set_inner_index(),
                        },
                        coords,
                    )?));
                }
            }
        }
        Err(ReadError::MetricIsMissing(tag))
    }
}
