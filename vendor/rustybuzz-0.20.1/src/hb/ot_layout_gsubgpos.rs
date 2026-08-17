//! Matching of glyph patterns.

use ttf_parser::opentype_layout::*;
use ttf_parser::{GlyphId, LazyArray16};

use super::buffer::hb_glyph_info_t;
use super::buffer::{hb_buffer_t, GlyphPropsFlags};
use super::hb_font_t;
use super::hb_mask_t;
use super::ot_layout::LayoutTable;
use super::ot_layout::*;
use super::ot_layout_common::*;
use super::unicode::hb_unicode_general_category_t;

/// Value represents glyph id.
pub fn match_glyph(glyph: GlyphId, value: u16) -> bool {
    glyph == GlyphId(value)
}

pub fn match_input(
    ctx: &mut hb_ot_apply_context_t,
    input_len: u16,
    match_func: &match_func_t,
    end_position: &mut usize,
    match_positions: &mut smallvec::SmallVec<[usize; 4]>,
    p_total_component_count: Option<&mut u8>,
) -> bool {
    // This is perhaps the trickiest part of OpenType...  Remarks:
    //
    // - If all components of the ligature were marks, we call this a mark ligature.
    //
    // - If there is no GDEF, and the ligature is NOT a mark ligature, we categorize
    //   it as a ligature glyph.
    //
    // - Ligatures cannot be formed across glyphs attached to different components
    //   of previous ligatures.  Eg. the sequence is LAM,SHADDA,LAM,FATHA,HEH, and
    //   LAM,LAM,HEH form a ligature, leaving SHADDA,FATHA next to eachother.
    //   However, it would be wrong to ligate that SHADDA,FATHA sequence.
    //   There are a couple of exceptions to this:
    //
    //   o If a ligature tries ligating with marks that belong to it itself, go ahead,
    //     assuming that the font designer knows what they are doing (otherwise it can
    //     break Indic stuff when a matra wants to ligate with a conjunct,
    //
    //   o If two marks want to ligate and they belong to different components of the
    //     same ligature glyph, and said ligature glyph is to be ignored according to
    //     mark-filtering rules, then allow.
    //     https://github.com/harfbuzz/harfbuzz/issues/545

    #[derive(PartialEq)]
    enum Ligbase {
        NotChecked,
        MayNotSkip,
        MaySkip,
    }

    let count = usize::from(input_len) + 1;
    if count > MAX_CONTEXT_LENGTH {
        return false;
    }

    if count > match_positions.len() {
        match_positions.resize(count, 0);
    }

    let mut iter = skipping_iterator_t::new(ctx, ctx.buffer.idx, false);
    iter.set_glyph_data(0);
    iter.enable_matching(match_func);

    let first = ctx.buffer.cur(0);
    let first_lig_id = _hb_glyph_info_get_lig_id(first);
    let first_lig_comp = _hb_glyph_info_get_lig_comp(first);
    let mut total_component_count = 0;
    let mut ligbase = Ligbase::NotChecked;

    for position in &mut match_positions[1..count] {
        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            *end_position = unsafe_to;
            return false;
        }

        *position = iter.index();

        let this = ctx.buffer.info[iter.index()];
        let this_lig_id = _hb_glyph_info_get_lig_id(&this);
        let this_lig_comp = _hb_glyph_info_get_lig_comp(&this);

        if first_lig_id != 0 && first_lig_comp != 0 {
            // If first component was attached to a previous ligature component,
            // all subsequent components should be attached to the same ligature
            // component, otherwise we shouldn't ligate them...
            if first_lig_id != this_lig_id || first_lig_comp != this_lig_comp {
                // ...unless, we are attached to a base ligature and that base
                // ligature is ignorable.
                if ligbase == Ligbase::NotChecked {
                    let out = ctx.buffer.out_info();
                    let mut j = ctx.buffer.out_len;
                    let mut found = false;
                    while j > 0 && _hb_glyph_info_get_lig_id(&out[j - 1]) == first_lig_id {
                        if _hb_glyph_info_get_lig_comp(&out[j - 1]) == 0 {
                            j -= 1;
                            found = true;
                            break;
                        }
                        j -= 1;
                    }

                    ligbase = if found && iter.may_skip(&out[j]) == may_skip_t::SKIP_YES {
                        Ligbase::MaySkip
                    } else {
                        Ligbase::MayNotSkip
                    };
                }

                if ligbase == Ligbase::MayNotSkip {
                    return false;
                }
            }
        } else {
            // If first component was NOT attached to a previous ligature component,
            // all subsequent components should also NOT be attached to any ligature
            // component, unless they are attached to the first component itself!
            if this_lig_id != 0 && this_lig_comp != 0 && (this_lig_id != first_lig_id) {
                return false;
            }
        }

        total_component_count += _hb_glyph_info_get_lig_num_comps(&this);
    }

    *end_position = iter.index() + 1;

    if let Some(p_total_component_count) = p_total_component_count {
        total_component_count += _hb_glyph_info_get_lig_num_comps(first);
        *p_total_component_count = total_component_count;
    }

    match_positions[0] = ctx.buffer.idx;

    true
}

pub fn match_backtrack(
    ctx: &mut hb_ot_apply_context_t,
    backtrack_len: u16,
    match_func: &match_func_t,
    match_start: &mut usize,
) -> bool {
    let mut iter = skipping_iterator_t::new(ctx, ctx.buffer.backtrack_len(), true);
    iter.set_glyph_data(0);
    iter.enable_matching(match_func);

    for _ in 0..backtrack_len {
        let mut unsafe_from = 0;
        if !iter.prev(Some(&mut unsafe_from)) {
            *match_start = unsafe_from;
            return false;
        }
    }

    *match_start = iter.index();
    true
}

pub fn match_lookahead(
    ctx: &mut hb_ot_apply_context_t,
    lookahead_len: u16,
    match_func: &match_func_t,
    start_index: usize,
    end_index: &mut usize,
) -> bool {
    // Function should always be called with a non-zero starting index
    // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
    assert!(start_index >= 1);
    let mut iter = skipping_iterator_t::new(ctx, start_index - 1, true);
    iter.set_glyph_data(0);
    iter.enable_matching(match_func);

    for _ in 0..lookahead_len {
        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            *end_index = unsafe_to;
            return false;
        }
    }

    *end_index = iter.index() + 1;
    true
}

pub type match_func_t<'a> = dyn Fn(GlyphId, u16) -> bool + 'a;

// In harfbuzz, skipping iterator works quite differently than it works here. In harfbuzz,
// hb_ot_apply_context contains a skipping iterator that itself contains another reference to
// the apply_context, meaning that we have a circular reference. Due to ownership rules in Rust,
// we cannot copy this approach. Because of this, we basically create a new skipping iterator
// when needed, and we do not have the `reset` and `init` methods that exist in harfbuzz. This makes
// backporting related changes very hard, but it seems unavoidable, unfortunately.
pub struct skipping_iterator_t<'a, 'b> {
    ctx: &'a hb_ot_apply_context_t<'a, 'b>,
    lookup_props: u32,
    ignore_zwnj: bool,
    ignore_zwj: bool,
    ignore_hidden: bool,
    mask: hb_mask_t,
    syllable: u8,
    matching: Option<&'a match_func_t<'a>>,
    buf_len: usize,
    glyph_data: u16,
    pub(crate) buf_idx: usize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum match_t {
    MATCH,
    NOT_MATCH,
    SKIP,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum may_match_t {
    MATCH_NO,
    MATCH_YES,
    MATCH_MAYBE,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum may_skip_t {
    SKIP_NO,
    SKIP_YES,
    SKIP_MAYBE,
}

impl<'a, 'b> skipping_iterator_t<'a, 'b> {
    pub fn new(
        ctx: &'a hb_ot_apply_context_t<'a, 'b>,
        start_buf_index: usize,
        context_match: bool,
    ) -> Self {
        skipping_iterator_t {
            ctx,
            lookup_props: ctx.lookup_props,
            // Ignore ZWNJ if we are matching GPOS, or matching GSUB context and asked to.
            ignore_zwnj: ctx.table_index == TableIndex::GPOS || (context_match && ctx.auto_zwnj),
            // Ignore ZWJ if we are matching context, or asked to.
            ignore_zwj: context_match || ctx.auto_zwj,
            // Ignore hidden glyphs (like CGJ) during GPOS.
            ignore_hidden: ctx.table_index == TableIndex::GPOS,
            mask: if context_match {
                u32::MAX
            } else {
                ctx.lookup_mask()
            },
            syllable: if ctx.buffer.idx == start_buf_index && ctx.per_syllable {
                ctx.buffer.cur(0).syllable()
            } else {
                0
            },
            glyph_data: 0,
            matching: None,
            buf_len: ctx.buffer.len,
            buf_idx: start_buf_index,
        }
    }

    pub fn set_glyph_data(&mut self, glyph_data: u16) {
        self.glyph_data = glyph_data
    }

    fn advance_glyph_data(&mut self) {
        self.glyph_data += 1;
    }

    pub fn set_lookup_props(&mut self, lookup_props: u32) {
        self.lookup_props = lookup_props;
    }

    pub fn enable_matching(&mut self, func: &'a match_func_t<'a>) {
        self.matching = Some(func);
    }

    pub fn index(&self) -> usize {
        self.buf_idx
    }

    pub fn next(&mut self, unsafe_to: Option<&mut usize>) -> bool {
        let stop = self.buf_len as i32 - 1;

        while (self.buf_idx as i32) < stop {
            self.buf_idx += 1;
            let info = &self.ctx.buffer.info[self.buf_idx];

            match self.match_(info) {
                match_t::MATCH => {
                    self.advance_glyph_data();
                    return true;
                }
                match_t::NOT_MATCH => {
                    if let Some(unsafe_to) = unsafe_to {
                        *unsafe_to = self.buf_idx + 1;
                    }

                    return false;
                }
                match_t::SKIP => continue,
            }
        }

        if let Some(unsafe_to) = unsafe_to {
            *unsafe_to = self.buf_idx + 1;
        }

        false
    }

    pub fn prev(&mut self, unsafe_from: Option<&mut usize>) -> bool {
        let stop: usize = 0;

        while self.buf_idx > stop {
            self.buf_idx -= 1;
            let info = &self.ctx.buffer.out_info()[self.buf_idx];

            match self.match_(info) {
                match_t::MATCH => {
                    self.advance_glyph_data();
                    return true;
                }
                match_t::NOT_MATCH => {
                    if let Some(unsafe_from) = unsafe_from {
                        *unsafe_from = self.buf_idx.max(1) - 1;
                    }

                    return false;
                }
                match_t::SKIP => {
                    continue;
                }
            }
        }

        if let Some(unsafe_from) = unsafe_from {
            *unsafe_from = 0;
        }

        false
    }

    pub fn match_(&self, info: &hb_glyph_info_t) -> match_t {
        let skip = self.may_skip(info);

        if skip == may_skip_t::SKIP_YES {
            return match_t::SKIP;
        }

        let _match = self.may_match(info);

        if _match == may_match_t::MATCH_YES
            || (_match == may_match_t::MATCH_MAYBE && skip == may_skip_t::SKIP_NO)
        {
            return match_t::MATCH;
        }

        if skip == may_skip_t::SKIP_NO {
            return match_t::NOT_MATCH;
        }

        match_t::SKIP
    }

    fn may_match(&self, info: &hb_glyph_info_t) -> may_match_t {
        if (info.mask & self.mask) == 0 || (self.syllable != 0 && self.syllable != info.syllable())
        {
            return may_match_t::MATCH_NO;
        }

        if let Some(match_func) = self.matching {
            return if match_func(info.as_glyph(), self.glyph_data) {
                may_match_t::MATCH_YES
            } else {
                may_match_t::MATCH_NO
            };
        }

        may_match_t::MATCH_MAYBE
    }

    fn may_skip(&self, info: &hb_glyph_info_t) -> may_skip_t {
        if !self.ctx.check_glyph_property(info, self.lookup_props) {
            return may_skip_t::SKIP_YES;
        }

        if _hb_glyph_info_is_default_ignorable(info)
            && (self.ignore_zwnj || !_hb_glyph_info_is_zwnj(info))
            && (self.ignore_zwj || !_hb_glyph_info_is_zwj(info))
            && (self.ignore_hidden || !_hb_glyph_info_is_hidden(info))
        {
            return may_skip_t::SKIP_MAYBE;
        }

        may_skip_t::SKIP_NO
    }
}

impl WouldApply for ContextLookup<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph = ctx.glyphs[0];
        match *self {
            Self::Format1 { coverage, sets } => coverage
                .get(glyph)
                .and_then(|index| sets.get(index))
                .map_or(false, |set| set.would_apply(ctx, &match_glyph)),
            Self::Format2 { classes, sets, .. } => {
                let class = classes.get(glyph);
                sets.get(class)
                    .map_or(false, |set| set.would_apply(ctx, &match_class(classes)))
            }
            Self::Format3 { coverages, .. } => {
                ctx.glyphs.len() == usize::from(coverages.len()) + 1
                    && coverages
                        .into_iter()
                        .enumerate()
                        .all(|(i, coverage)| coverage.get(ctx.glyphs[i + 1]).is_some())
            }
        }
    }
}

impl Apply for ContextLookup<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        match *self {
            Self::Format1 { coverage, sets } => {
                coverage.get(glyph)?;
                let set = coverage.get(glyph).and_then(|index| sets.get(index))?;
                set.apply(ctx, &match_glyph)
            }
            Self::Format2 {
                coverage,
                classes,
                sets,
            } => {
                coverage.get(glyph)?;
                let class = classes.get(glyph);
                let set = sets.get(class)?;
                set.apply(ctx, &match_class(classes))
            }
            Self::Format3 {
                coverage,
                coverages,
                lookups,
            } => {
                coverage.get(glyph)?;
                let coverages_len = coverages.len();

                let match_func = |glyph, index| {
                    let coverage = coverages.get(index).unwrap();
                    coverage.get(glyph).is_some()
                };

                let mut match_end = 0;
                let mut match_positions = smallvec::SmallVec::from_elem(0, 4);

                if match_input(
                    ctx,
                    coverages_len,
                    &match_func,
                    &mut match_end,
                    &mut match_positions,
                    None,
                ) {
                    ctx.buffer
                        .unsafe_to_break(Some(ctx.buffer.idx), Some(match_end));
                    apply_lookup(
                        ctx,
                        usize::from(coverages_len),
                        &mut match_positions,
                        match_end,
                        lookups,
                    );
                    Some(())
                } else {
                    ctx.buffer
                        .unsafe_to_concat(Some(ctx.buffer.idx), Some(match_end));
                    None
                }
            }
        }
    }
}

trait SequenceRuleSetExt {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool;
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_func: &match_func_t) -> Option<()>;
}

impl SequenceRuleSetExt for SequenceRuleSet<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool {
        self.into_iter()
            .any(|rule| rule.would_apply(ctx, match_func))
    }

    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_func: &match_func_t) -> Option<()> {
        if self
            .into_iter()
            .any(|rule| rule.apply(ctx, match_func).is_some())
        {
            Some(())
        } else {
            None
        }
    }
}

trait SequenceRuleExt {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool;
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_func: &match_func_t) -> Option<()>;
}

impl SequenceRuleExt for SequenceRule<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool {
        ctx.glyphs.len() == usize::from(self.input.len()) + 1
            && self
                .input
                .into_iter()
                .enumerate()
                .all(|(i, value)| match_func(ctx.glyphs[i + 1], value))
    }

    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_func: &match_func_t) -> Option<()> {
        apply_context(ctx, self.input, match_func, self.lookups)

        // TODO: Port optimized version from https://github.com/harfbuzz/harfbuzz/commit/645fabd10
    }
}

impl WouldApply for ChainedContextLookup<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = ctx.glyphs[0];
        match *self {
            Self::Format1 { coverage, sets } => coverage
                .get(glyph_id)
                .and_then(|index| sets.get(index))
                .map_or(false, |set| set.would_apply(ctx, &match_glyph)),
            Self::Format2 {
                input_classes,
                sets,
                ..
            } => {
                let class = input_classes.get(glyph_id);
                sets.get(class).map_or(false, |set| {
                    set.would_apply(ctx, &match_class(input_classes))
                })
            }
            Self::Format3 {
                backtrack_coverages,
                input_coverages,
                lookahead_coverages,
                ..
            } => {
                (!ctx.zero_context
                    || (backtrack_coverages.is_empty() && lookahead_coverages.is_empty()))
                    && (ctx.glyphs.len() == usize::from(input_coverages.len()) + 1
                        && input_coverages
                            .into_iter()
                            .enumerate()
                            .all(|(i, coverage)| coverage.contains(ctx.glyphs[i + 1])))
            }
        }
    }
}

impl Apply for ChainedContextLookup<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        match *self {
            Self::Format1 { coverage, sets } => {
                let index = coverage.get(glyph)?;
                let set = sets.get(index)?;
                set.apply(ctx, [&match_glyph, &match_glyph, &match_glyph])
            }
            Self::Format2 {
                coverage,
                backtrack_classes,
                input_classes,
                lookahead_classes,
                sets,
            } => {
                coverage.get(glyph)?;
                let class = input_classes.get(glyph);
                let set = sets.get(class)?;
                set.apply(
                    ctx,
                    [
                        &match_class(backtrack_classes),
                        &match_class(input_classes),
                        &match_class(lookahead_classes),
                    ],
                )
            }
            Self::Format3 {
                coverage,
                backtrack_coverages,
                input_coverages,
                lookahead_coverages,
                lookups,
            } => {
                coverage.get(glyph)?;

                let back = |glyph, index| {
                    let coverage = backtrack_coverages.get(index).unwrap();
                    coverage.contains(glyph)
                };

                let ahead = |glyph, index| {
                    let coverage = lookahead_coverages.get(index).unwrap();
                    coverage.contains(glyph)
                };

                let input = |glyph, index| {
                    let coverage = input_coverages.get(index).unwrap();
                    coverage.contains(glyph)
                };

                let mut end_index = ctx.buffer.idx;
                let mut match_end = 0;
                let mut match_positions = smallvec::SmallVec::from_elem(0, 4);

                let input_matches = match_input(
                    ctx,
                    input_coverages.len(),
                    &input,
                    &mut match_end,
                    &mut match_positions,
                    None,
                );

                if input_matches {
                    end_index = match_end;
                }

                if !(input_matches
                    && match_lookahead(
                        ctx,
                        lookahead_coverages.len(),
                        &ahead,
                        match_end,
                        &mut end_index,
                    ))
                {
                    ctx.buffer
                        .unsafe_to_concat(Some(ctx.buffer.idx), Some(end_index));
                    return None;
                }

                let mut start_index = ctx.buffer.out_len;

                if !match_backtrack(ctx, backtrack_coverages.len(), &back, &mut start_index) {
                    ctx.buffer
                        .unsafe_to_concat_from_outbuffer(Some(start_index), Some(end_index));
                    return None;
                }

                ctx.buffer
                    .unsafe_to_break_from_outbuffer(Some(start_index), Some(end_index));
                apply_lookup(
                    ctx,
                    usize::from(input_coverages.len()),
                    &mut match_positions,
                    match_end,
                    lookups,
                );

                Some(())
            }
        }
    }
}

trait ChainRuleSetExt {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool;
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_funcs: [&match_func_t; 3])
        -> Option<()>;
}

impl ChainRuleSetExt for ChainedSequenceRuleSet<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool {
        self.into_iter()
            .any(|rule| rule.would_apply(ctx, match_func))
    }

    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        match_funcs: [&match_func_t; 3],
    ) -> Option<()> {
        if self
            .into_iter()
            .any(|rule| rule.apply(ctx, match_funcs).is_some())
        {
            Some(())
        } else {
            None
        }

        // TODO: Port optimized version from https://github.com/harfbuzz/harfbuzz/commit/77080f86f
    }
}

trait ChainRuleExt {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool;
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, match_funcs: [&match_func_t; 3])
        -> Option<()>;
}

impl ChainRuleExt for ChainedSequenceRule<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &match_func_t) -> bool {
        (!ctx.zero_context || (self.backtrack.is_empty() && self.lookahead.is_empty()))
            && (ctx.glyphs.len() == usize::from(self.input.len()) + 1
                && self
                    .input
                    .into_iter()
                    .enumerate()
                    .all(|(i, value)| match_func(ctx.glyphs[i + 1], value)))
    }

    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        match_funcs: [&match_func_t; 3],
    ) -> Option<()> {
        apply_chain_context(
            ctx,
            self.backtrack,
            self.input,
            self.lookahead,
            match_funcs,
            self.lookups,
        )
    }
}

fn apply_context(
    ctx: &mut hb_ot_apply_context_t,
    input: LazyArray16<u16>,
    match_func: &match_func_t,
    lookups: LazyArray16<SequenceLookupRecord>,
) -> Option<()> {
    let match_func = |glyph, index| {
        let value = input.get(index).unwrap();
        match_func(glyph, value)
    };

    let mut match_end = 0;
    let mut match_positions = smallvec::SmallVec::from_elem(0, 4);

    if match_input(
        ctx,
        input.len(),
        &match_func,
        &mut match_end,
        &mut match_positions,
        None,
    ) {
        ctx.buffer
            .unsafe_to_break(Some(ctx.buffer.idx), Some(match_end));
        apply_lookup(
            ctx,
            usize::from(input.len()),
            &mut match_positions,
            match_end,
            lookups,
        );
        return Some(());
    }

    None
}

fn apply_chain_context(
    ctx: &mut hb_ot_apply_context_t,
    backtrack: LazyArray16<u16>,
    input: LazyArray16<u16>,
    lookahead: LazyArray16<u16>,
    match_funcs: [&match_func_t; 3],
    lookups: LazyArray16<SequenceLookupRecord>,
) -> Option<()> {
    // NOTE: Whenever something in this method changes, we also need to
    // change it in the `apply` implementation for ChainedContextLookup.
    let f1 = |glyph, index| {
        let value = backtrack.get(index).unwrap();
        match_funcs[0](glyph, value)
    };

    let f2 = |glyph, index| {
        let value = lookahead.get(index).unwrap();
        match_funcs[2](glyph, value)
    };

    let f3 = |glyph, index| {
        let value = input.get(index).unwrap();
        match_funcs[1](glyph, value)
    };

    let mut end_index = ctx.buffer.idx;
    let mut match_end = 0;
    let mut match_positions = smallvec::SmallVec::from_elem(0, 4);

    let input_matches = match_input(
        ctx,
        input.len(),
        &f3,
        &mut match_end,
        &mut match_positions,
        None,
    );

    if input_matches {
        end_index = match_end;
    }

    if !(input_matches && match_lookahead(ctx, lookahead.len(), &f2, match_end, &mut end_index)) {
        ctx.buffer
            .unsafe_to_concat(Some(ctx.buffer.idx), Some(end_index));
        return None;
    }

    let mut start_index = ctx.buffer.out_len;

    if !match_backtrack(ctx, backtrack.len(), &f1, &mut start_index) {
        ctx.buffer
            .unsafe_to_concat_from_outbuffer(Some(start_index), Some(end_index));
        return None;
    }

    ctx.buffer
        .unsafe_to_break_from_outbuffer(Some(start_index), Some(end_index));
    apply_lookup(
        ctx,
        usize::from(input.len()),
        &mut match_positions,
        match_end,
        lookups,
    );

    Some(())
}

fn apply_lookup(
    ctx: &mut hb_ot_apply_context_t,
    input_len: usize,
    match_positions: &mut smallvec::SmallVec<[usize; 4]>,
    match_end: usize,
    lookups: LazyArray16<SequenceLookupRecord>,
) {
    let mut count = input_len + 1;

    if count > match_positions.len() {
        match_positions.resize(count, 0);
    }

    // All positions are distance from beginning of *output* buffer.
    // Adjust.
    let mut end: isize = {
        let backtrack_len = ctx.buffer.backtrack_len();
        let delta = backtrack_len as isize - ctx.buffer.idx as isize;

        // Convert positions to new indexing.
        for j in 0..count {
            match_positions[j] = (match_positions[j] as isize + delta) as _;
        }

        backtrack_len as isize + match_end as isize - ctx.buffer.idx as isize
    };

    for record in lookups {
        if !ctx.buffer.successful {
            break;
        }

        let idx = usize::from(record.sequence_index);
        if idx >= count {
            continue;
        }

        let orig_len = ctx.buffer.backtrack_len() + ctx.buffer.lookahead_len();

        // This can happen if earlier recursed lookups deleted many entries.
        if match_positions[idx] >= orig_len {
            continue;
        }

        if !ctx.buffer.move_to(match_positions[idx]) {
            break;
        }

        if ctx.buffer.max_ops <= 0 {
            break;
        }

        if ctx.recurse(record.lookup_list_index).is_none() {
            continue;
        }

        let new_len = ctx.buffer.backtrack_len() + ctx.buffer.lookahead_len();
        let mut delta = new_len as isize - orig_len as isize;
        if delta == 0 {
            continue;
        }

        // Recursed lookup changed buffer len.  Adjust.
        //
        // TODO:
        //
        // Right now, if buffer length increased by n, we assume n new glyphs
        // were added right after the current position, and if buffer length
        // was decreased by n, we assume n match positions after the current
        // one where removed.  The former (buffer length increased) case is
        // fine, but the decrease case can be improved in at least two ways,
        // both of which are significant:
        //
        //   - If recursed-to lookup is MultipleSubst and buffer length
        //     decreased, then it's current match position that was deleted,
        //     NOT the one after it.
        //
        //   - If buffer length was decreased by n, it does not necessarily
        //     mean that n match positions where removed, as there recursed-to
        //     lookup might had a different LookupFlag.  Here's a constructed
        //     case of that:
        //     https://github.com/harfbuzz/harfbuzz/discussions/3538
        //
        // It should be possible to construct tests for both of these cases.

        end += delta;
        if end < match_positions[idx] as isize {
            // End might end up being smaller than match_positions[idx] if the recursed
            // lookup ended up removing many items.
            // Just never rewind end beyond start of current position, since that is
            // not possible in the recursed lookup.  Also adjust delta as such.
            //
            // https://bugs.chromium.org/p/chromium/issues/detail?id=659496
            // https://github.com/harfbuzz/harfbuzz/issues/1611
            //
            delta += match_positions[idx] as isize - end;
            end = match_positions[idx] as isize;
        }

        // next now is the position after the recursed lookup.
        let mut next = idx + 1;

        if delta > 0 {
            if delta as usize + count > MAX_CONTEXT_LENGTH {
                break;
            }

            if delta as usize + count > match_positions.len() {
                let inner_max = (core::cmp::max(4, match_positions.len()) as f32 * 1.5) as usize;
                match_positions.resize(core::cmp::max(delta as usize + count, inner_max), 0);
            }
        } else {
            // NOTE: delta is non-positive.
            delta = delta.max(next as isize - count as isize);
            next = (next as isize - delta) as _;
        }

        // Shift!
        match_positions.copy_within(next..count, (next as isize + delta) as _);
        next = (next as isize + delta) as _;
        count = (count as isize + delta) as _;

        // Fill in new entries.
        for j in idx + 1..next {
            match_positions[j] = match_positions[j - 1] + 1;
        }

        // And fixup the rest.
        while next < count {
            match_positions[next] = (match_positions[next] as isize + delta) as _;
            next += 1;
        }
    }

    ctx.buffer.move_to(end.try_into().unwrap());
}

/// Value represents glyph class.
fn match_class(class_def: ClassDefinition<'_>) -> impl Fn(GlyphId, u16) -> bool + '_ {
    move |glyph, value| class_def.get(glyph) == value
}

/// Find out whether a lookup would be applied.
pub trait WouldApply {
    /// Whether the lookup would be applied.
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool;
}

/// Apply a lookup.
pub trait Apply {
    /// Apply the lookup.
    fn apply(&self, ctx: &mut OT::hb_ot_apply_context_t) -> Option<()>;
}

pub struct WouldApplyContext<'a> {
    pub glyphs: &'a [GlyphId],
    pub zero_context: bool,
}

pub mod OT {
    use super::*;
    use crate::hb::set_digest::{hb_set_digest_ext, hb_set_digest_t};

    pub struct hb_ot_apply_context_t<'a, 'b> {
        pub table_index: TableIndex,
        pub face: &'a hb_font_t<'b>,
        pub buffer: &'a mut hb_buffer_t,
        lookup_mask: hb_mask_t,
        pub per_syllable: bool,
        pub lookup_index: LookupIndex,
        pub lookup_props: u32,
        pub nesting_level_left: usize,
        pub auto_zwnj: bool,
        pub auto_zwj: bool,
        pub random: bool,
        pub random_state: u32,
        pub last_base: i32,
        pub last_base_until: u32,
        pub digest: hb_set_digest_t,
    }

    impl<'a, 'b> hb_ot_apply_context_t<'a, 'b> {
        pub fn new(
            table_index: TableIndex,
            face: &'a hb_font_t<'b>,
            buffer: &'a mut hb_buffer_t,
        ) -> Self {
            let buffer_digest = buffer.digest();
            Self {
                table_index,
                face,
                buffer,
                lookup_mask: 1,
                per_syllable: false,
                lookup_index: u16::MAX,
                lookup_props: 0,
                nesting_level_left: MAX_NESTING_LEVEL,
                auto_zwnj: true,
                auto_zwj: true,
                random: false,
                random_state: 1,
                last_base: -1,
                last_base_until: 0,
                digest: buffer_digest,
            }
        }

        pub fn random_number(&mut self) -> u32 {
            // http://www.cplusplus.com/reference/random/minstd_rand/
            self.random_state = self.random_state.wrapping_mul(48271) % 2147483647;
            self.random_state
        }

        pub fn set_lookup_mask(&mut self, mask: hb_mask_t) {
            self.lookup_mask = mask;
            self.last_base = -1;
            self.last_base_until = 0;
        }

        pub fn lookup_mask(&self) -> hb_mask_t {
            self.lookup_mask
        }

        pub fn recurse(&mut self, sub_lookup_index: LookupIndex) -> Option<()> {
            if self.nesting_level_left == 0 {
                self.buffer.shaping_failed = true;
                return None;
            }

            self.buffer.max_ops -= 1;
            if self.buffer.max_ops < 0 {
                self.buffer.shaping_failed = true;
                return None;
            }

            self.nesting_level_left -= 1;
            let saved_props = self.lookup_props;
            let saved_index = self.lookup_index;

            self.lookup_index = sub_lookup_index;
            let applied = match self.table_index {
                TableIndex::GSUB => self
                    .face
                    .gsub
                    .as_ref()
                    .and_then(|table| table.get_lookup(sub_lookup_index))
                    .and_then(|lookup| {
                        self.lookup_props = lookup.props();
                        lookup.apply(self)
                    }),
                TableIndex::GPOS => self
                    .face
                    .gpos
                    .as_ref()
                    .and_then(|table| table.get_lookup(sub_lookup_index))
                    .and_then(|lookup| {
                        self.lookup_props = lookup.props();
                        lookup.apply(self)
                    }),
            };

            self.lookup_props = saved_props;
            self.lookup_index = saved_index;
            self.nesting_level_left += 1;
            applied
        }

        pub fn check_glyph_property(&self, info: &hb_glyph_info_t, match_props: u32) -> bool {
            let glyph_props = info.glyph_props();

            // Lookup flags are lower 16-bit of match props.
            let lookup_flags = match_props as u16;

            // Not covered, if, for example, glyph class is ligature and
            // match_props includes LookupFlags::IgnoreLigatures
            if glyph_props & lookup_flags & lookup_flags::IGNORE_FLAGS != 0 {
                return false;
            }

            if glyph_props & GlyphPropsFlags::MARK.bits() != 0 {
                // If using mark filtering sets, the high short of
                // match_props has the set index.
                if lookup_flags & lookup_flags::USE_MARK_FILTERING_SET != 0 {
                    let set_index = (match_props >> 16) as u16;
                    // TODO: harfbuzz uses a digest here to speed things up if HB_NO_GDEF_CACHE
                    // is enabled. But a bit harder to implement for us since it's taken care of by
                    // ttf-parser
                    if let Some(table) = self.face.tables().gdef {
                        return table.is_mark_glyph(info.as_glyph(), Some(set_index));
                    } else {
                        return false;
                    }
                }

                // The second byte of match_props has the meaning
                // "ignore marks of attachment type different than
                // the attachment type specified."
                if lookup_flags & lookup_flags::MARK_ATTACHMENT_TYPE_MASK != 0 {
                    return (lookup_flags & lookup_flags::MARK_ATTACHMENT_TYPE_MASK)
                        == (glyph_props & lookup_flags::MARK_ATTACHMENT_TYPE_MASK);
                }
            }

            true
        }

        fn set_glyph_class(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
            ligature: bool,
            component: bool,
        ) {
            self.digest.add(glyph_id);

            let cur = self.buffer.cur_mut(0);
            let mut props = cur.glyph_props();

            props |= GlyphPropsFlags::SUBSTITUTED.bits();

            if ligature {
                props |= GlyphPropsFlags::LIGATED.bits();
                // In the only place that the MULTIPLIED bit is used, Uniscribe
                // seems to only care about the "last" transformation between
                // Ligature and Multiple substitutions.  Ie. if you ligate, expand,
                // and ligate again, it forgives the multiplication and acts as
                // if only ligation happened.  As such, clear MULTIPLIED bit.
                props &= !GlyphPropsFlags::MULTIPLIED.bits();
            }

            if component {
                props |= GlyphPropsFlags::MULTIPLIED.bits();
            }

            let has_glyph_classes = self
                .face
                .tables()
                .gdef
                .map_or(false, |table| table.has_glyph_classes());

            if has_glyph_classes {
                props &= GlyphPropsFlags::PRESERVE.bits();
                cur.set_glyph_props(props | self.face.glyph_props(glyph_id));
            } else if !class_guess.is_empty() {
                props &= GlyphPropsFlags::PRESERVE.bits();
                cur.set_glyph_props(props | class_guess.bits());
            } else {
                cur.set_glyph_props(props);
            }
        }

        pub fn replace_glyph(&mut self, glyph_id: GlyphId) {
            self.set_glyph_class(glyph_id, GlyphPropsFlags::empty(), false, false);
            self.buffer.replace_glyph(u32::from(glyph_id.0));
        }

        pub fn replace_glyph_inplace(&mut self, glyph_id: GlyphId) {
            self.set_glyph_class(glyph_id, GlyphPropsFlags::empty(), false, false);
            self.buffer.cur_mut(0).glyph_id = u32::from(glyph_id.0);
        }

        pub fn replace_glyph_with_ligature(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
        ) {
            self.set_glyph_class(glyph_id, class_guess, true, false);
            self.buffer.replace_glyph(u32::from(glyph_id.0));
        }

        pub fn output_glyph_for_component(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
        ) {
            self.set_glyph_class(glyph_id, class_guess, false, true);
            self.buffer.output_glyph(u32::from(glyph_id.0));
        }
    }
}

use OT::hb_ot_apply_context_t;

pub fn ligate_input(
    ctx: &mut hb_ot_apply_context_t,
    // Including the first glyph
    count: usize,
    // Including the first glyph
    match_positions: &smallvec::SmallVec<[usize; 4]>,
    match_end: usize,
    total_component_count: u8,
    lig_glyph: GlyphId,
) {
    // - If a base and one or more marks ligate, consider that as a base, NOT
    //   ligature, such that all following marks can still attach to it.
    //   https://github.com/harfbuzz/harfbuzz/issues/1109
    //
    // - If all components of the ligature were marks, we call this a mark ligature.
    //   If it *is* a mark ligature, we don't allocate a new ligature id, and leave
    //   the ligature to keep its old ligature id.  This will allow it to attach to
    //   a base ligature in GPOS.  Eg. if the sequence is: LAM,LAM,SHADDA,FATHA,HEH,
    //   and LAM,LAM,HEH for a ligature, they will leave SHADDA and FATHA with a
    //   ligature id and component value of 2.  Then if SHADDA,FATHA form a ligature
    //   later, we don't want them to lose their ligature id/component, otherwise
    //   GPOS will fail to correctly position the mark ligature on top of the
    //   LAM,LAM,HEH ligature.  See:
    //     https://bugzilla.gnome.org/show_bug.cgi?id=676343
    //
    // - If a ligature is formed of components that some of which are also ligatures
    //   themselves, and those ligature components had marks attached to *their*
    //   components, we have to attach the marks to the new ligature component
    //   positions!  Now *that*'s tricky!  And these marks may be following the
    //   last component of the whole sequence, so we should loop forward looking
    //   for them and update them.
    //
    //   Eg. the sequence is LAM,LAM,SHADDA,FATHA,HEH, and the font first forms a
    //   'calt' ligature of LAM,HEH, leaving the SHADDA and FATHA with a ligature
    //   id and component == 1.  Now, during 'liga', the LAM and the LAM-HEH ligature
    //   form a LAM-LAM-HEH ligature.  We need to reassign the SHADDA and FATHA to
    //   the new ligature with a component value of 2.
    //
    //   This in fact happened to a font...  See:
    //   https://bugzilla.gnome.org/show_bug.cgi?id=437633
    //

    let mut buffer = &mut ctx.buffer;
    buffer.merge_clusters(buffer.idx, match_end);

    let mut is_base_ligature = _hb_glyph_info_is_base_glyph(&buffer.info[match_positions[0]]);
    let mut is_mark_ligature = _hb_glyph_info_is_mark(&buffer.info[match_positions[0]]);
    for i in 1..count {
        if !_hb_glyph_info_is_mark(&buffer.info[match_positions[i]]) {
            is_base_ligature = false;
            is_mark_ligature = false;
        }
    }

    let is_ligature = !is_base_ligature && !is_mark_ligature;
    let class = if is_ligature {
        GlyphPropsFlags::LIGATURE
    } else {
        GlyphPropsFlags::empty()
    };
    let lig_id = if is_ligature {
        buffer.allocate_lig_id()
    } else {
        0
    };
    let first = buffer.cur_mut(0);
    let mut last_lig_id = _hb_glyph_info_get_lig_id(first);
    let mut last_num_comps = _hb_glyph_info_get_lig_num_comps(first);
    let mut comps_so_far = last_num_comps;

    if is_ligature {
        _hb_glyph_info_set_lig_props_for_ligature(first, lig_id, total_component_count);
        if _hb_glyph_info_get_general_category(first)
            == hb_unicode_general_category_t::NonspacingMark
        {
            _hb_glyph_info_set_general_category(first, hb_unicode_general_category_t::OtherLetter);
        }
    }

    ctx.replace_glyph_with_ligature(lig_glyph, class);
    buffer = &mut ctx.buffer;

    for i in 1..count {
        while buffer.idx < match_positions[i] && buffer.successful {
            if is_ligature {
                let cur = buffer.cur_mut(0);
                let mut this_comp = _hb_glyph_info_get_lig_comp(cur);
                if this_comp == 0 {
                    this_comp = last_num_comps;
                }
                // Avoid the potential for a wrap-around bug when subtracting from an unsigned integer
                // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
                assert!(comps_so_far >= last_num_comps);
                let new_lig_comp = comps_so_far - last_num_comps + this_comp.min(last_num_comps);
                _hb_glyph_info_set_lig_props_for_mark(cur, lig_id, new_lig_comp);
            }
            buffer.next_glyph();
        }

        let cur = buffer.cur(0);
        last_lig_id = _hb_glyph_info_get_lig_id(cur);
        last_num_comps = _hb_glyph_info_get_lig_num_comps(cur);
        comps_so_far += last_num_comps;

        // Skip the base glyph.
        buffer.idx += 1;
    }

    if !is_mark_ligature && last_lig_id != 0 {
        // Re-adjust components for any marks following.
        for i in buffer.idx..buffer.len {
            let info = &mut buffer.info[i];
            if last_lig_id != _hb_glyph_info_get_lig_id(info) {
                break;
            }

            let this_comp = _hb_glyph_info_get_lig_comp(info);
            if this_comp == 0 {
                break;
            }

            // Avoid the potential for a wrap-around bug when subtracting from an unsigned integer
            // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
            assert!(comps_so_far >= last_num_comps);
            let new_lig_comp = comps_so_far - last_num_comps + this_comp.min(last_num_comps);
            _hb_glyph_info_set_lig_props_for_mark(info, lig_id, new_lig_comp)
        }
    }
}
