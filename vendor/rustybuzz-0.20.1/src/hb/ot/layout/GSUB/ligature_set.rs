use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{Apply, WouldApply, WouldApplyContext};
use ttf_parser::gsub::LigatureSet;

impl WouldApply for LigatureSet<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        self.into_iter().any(|lig| lig.would_apply(ctx))
    }
}

impl Apply for LigatureSet<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        for lig in self.into_iter() {
            if lig.apply(ctx).is_some() {
                return Some(());
            }
        }
        None

        // TODO: port https://github.com/harfbuzz/harfbuzz/commit/7881eadff and
        // the following commits. Since it's behind a feature flag, we ignore it
        // for now and just use the simpler version.
    }
}
