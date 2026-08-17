use crate::hb::ot_layout::LayoutLookup;
use crate::hb::ot_layout_common::PositioningLookup;
use crate::hb::ot_layout_gsubgpos::Apply;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::set_digest::{hb_set_digest_ext, hb_set_digest_t};

impl LayoutLookup for PositioningLookup<'_> {
    fn props(&self) -> u32 {
        self.props
    }

    fn is_reverse(&self) -> bool {
        false
    }

    fn digest(&self) -> &hb_set_digest_t {
        &self.set_digest
    }
}

impl Apply for PositioningLookup<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        if self.digest().may_have_glyph(ctx.buffer.cur(0).as_glyph()) {
            for subtable in &self.subtables {
                if subtable.apply(ctx).is_some() {
                    return Some(());
                }
            }
        }

        None
    }
}
