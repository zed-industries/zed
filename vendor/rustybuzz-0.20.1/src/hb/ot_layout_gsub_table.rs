use ttf_parser::gsub::*;
use ttf_parser::opentype_layout::LookupIndex;

use super::buffer::hb_buffer_t;
use super::hb_font_t;
use super::ot_layout::*;
use super::ot_layout_common::{SubstLookup, SubstitutionTable};
use super::ot_layout_gsubgpos::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use OT::hb_ot_apply_context_t;

pub fn substitute(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    apply_layout_table(plan, face, buffer, face.gsub.as_ref());
}

impl<'a> LayoutTable for SubstitutionTable<'a> {
    const INDEX: TableIndex = TableIndex::GSUB;
    const IN_PLACE: bool = false;

    type Lookup = SubstLookup<'a>;

    fn get_lookup(&self, index: LookupIndex) -> Option<&Self::Lookup> {
        self.lookups.get(usize::from(index))
    }
}

impl WouldApply for SubstitutionSubtable<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        match self {
            Self::Single(t) => t.would_apply(ctx),
            Self::Multiple(t) => t.would_apply(ctx),
            Self::Alternate(t) => t.would_apply(ctx),
            Self::Ligature(t) => t.would_apply(ctx),
            Self::Context(t) => t.would_apply(ctx),
            Self::ChainContext(t) => t.would_apply(ctx),
            Self::ReverseChainSingle(t) => t.would_apply(ctx),
        }
    }
}

impl Apply for SubstitutionSubtable<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        match self {
            Self::Single(t) => t.apply(ctx),
            Self::Multiple(t) => t.apply(ctx),
            Self::Alternate(t) => t.apply(ctx),
            Self::Ligature(t) => t.apply(ctx),
            Self::Context(t) => t.apply(ctx),
            Self::ChainContext(t) => t.apply(ctx),
            Self::ReverseChainSingle(t) => t.apply(ctx),
        }
    }
}
