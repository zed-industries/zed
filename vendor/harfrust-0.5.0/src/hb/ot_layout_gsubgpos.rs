//! Matching of glyph patterns.

use super::buffer::GlyphInfo;
use super::buffer::{hb_buffer_t, GlyphPropsFlags};
use super::cache::hb_cache_t;
use super::hb_font_t;
use super::hb_mask_t;
use super::ot_layout::*;
use super::ot_layout_common::*;
use super::set_digest::hb_set_digest_t;
use crate::hb::ot::{ClassDefInfo, CoverageInfo};
use crate::hb::ot_layout_gsubgpos::OT::check_glyph_property;
use crate::hb::unicode::GeneralCategory;
use alloc::boxed::Box;
use read_fonts::tables::layout::SequenceLookupRecord;
use read_fonts::types::GlyphId;

pub(crate) type MatchPositions = smallvec::SmallVec<[u32; 8]>;

/// Value represents glyph id.
pub fn match_glyph(info: &mut GlyphInfo, value: u16) -> bool {
    info.glyph_id == value as u32
}

pub fn match_always(_info: &mut GlyphInfo, _value: u16) -> bool {
    true
}

pub fn match_input(
    ctx: &mut hb_ot_apply_context_t,
    input_len: u16,
    match_func: impl Fn(&mut GlyphInfo, u16) -> bool,
    end_position: &mut usize,
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

    if count == 1 {
        *end_position = ctx.buffer.idx + 1;
        ctx.match_positions_len = 1;
        ctx.match_positions[0] = ctx.buffer.idx as u32;
        if let Some(p_total_component_count) = p_total_component_count {
            *p_total_component_count = ctx.buffer.cur(0).lig_num_comps();
        }
        return true;
    }

    if count > MAX_CONTEXT_LENGTH {
        return false;
    }
    ctx.match_positions_len = count;

    let mut iter = skipping_iterator_t::with_match_fn(ctx, false, Some(match_func));
    iter.reset(iter.buffer.idx);
    iter.set_glyph_data(0);

    let first = *iter.buffer.cur(0);
    let first_lig_id = first.lig_id();
    let first_lig_comp = first.lig_comp();
    let mut total_component_count = 0;
    let mut ligbase = Ligbase::NotChecked;

    for i in 1..count {
        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            *end_position = unsafe_to;
            return false;
        }

        iter.set_match_position(i, iter.index());

        let this = iter.buffer.info[iter.index()];
        let this_lig_id = this.lig_id();
        let this_lig_comp = this.lig_comp();

        if first_lig_id != 0 && first_lig_comp != 0 {
            // If first component was attached to a previous ligature component,
            // all subsequent components should be attached to the same ligature
            // component, otherwise we shouldn't ligate them...
            if first_lig_id != this_lig_id || first_lig_comp != this_lig_comp {
                // ...unless, we are attached to a base ligature and that base
                // ligature is ignorable.
                if ligbase == Ligbase::NotChecked {
                    let out = iter.buffer.out_info();
                    let mut j = iter.buffer.out_len;
                    let mut found = false;
                    while j > 0 && out[j - 1].lig_id() == first_lig_id {
                        if out[j - 1].lig_comp() == 0 {
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

        total_component_count += this.lig_num_comps();
    }

    *end_position = iter.index() + 1;

    if let Some(p_total_component_count) = p_total_component_count {
        total_component_count += first.lig_num_comps();
        *p_total_component_count = total_component_count;
    }

    ctx.match_positions[0] = iter.buffer.idx as u32;

    true
}

pub fn match_backtrack(
    ctx: &mut hb_ot_apply_context_t,
    backtrack_len: u16,
    match_func: impl Fn(&mut GlyphInfo, u16) -> bool,
    match_start: &mut usize,
) -> bool {
    if backtrack_len == 0 {
        *match_start = ctx.buffer.backtrack_len();
        return true;
    }

    let mut iter = skipping_iterator_t::with_match_fn(ctx, true, Some(match_func));
    iter.reset_back(iter.buffer.backtrack_len());
    iter.set_glyph_data(0);

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
    match_func: impl Fn(&mut GlyphInfo, u16) -> bool,
    start_index: usize,
    end_index: &mut usize,
) -> bool {
    if lookahead_len == 0 {
        *end_index = start_index;
        return true;
    }

    // Function should always be called with a non-zero starting index
    // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
    debug_assert!(start_index >= 1);
    let mut iter = skipping_iterator_t::with_match_fn(ctx, true, Some(match_func));
    iter.reset(start_index - 1);
    iter.set_glyph_data(0);

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
pub enum may_skip_t {
    SKIP_NO,
    SKIP_YES,
    SKIP_MAYBE,
}

#[derive(Default)]
pub struct matcher_t {
    lookup_props: u32,
    mask: hb_mask_t,
    ignore_zwnj: bool,
    ignore_zwj: bool,
    ignore_hidden: bool,
    per_syllable: bool,
}

impl matcher_t {
    fn new(ctx: &hb_ot_apply_context_t, context_match: bool) -> Self {
        matcher_t {
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
            /* Per syllable matching is only for GSUB. */
            per_syllable: ctx.table_index == TableIndex::GSUB && ctx.per_syllable,
        }
    }

    fn may_match(
        &self,
        info: &mut GlyphInfo,
        glyph_data: u16,
        match_func: Option<&impl Fn(&mut GlyphInfo, u16) -> bool>,
        syllable: u8,
    ) -> may_match_t {
        if (info.mask & self.mask) == 0
            || (self.per_syllable && syllable != 0 && syllable != info.syllable())
        {
            return may_match_t::MATCH_NO;
        }

        if let Some(match_func) = match_func {
            return if match_func(info, glyph_data) {
                may_match_t::MATCH_YES
            } else {
                may_match_t::MATCH_NO
            };
        }

        may_match_t::MATCH_MAYBE
    }

    #[inline(always)]
    fn may_skip(&self, info: &GlyphInfo, face: &hb_font_t, lookup_props: u32) -> may_skip_t {
        if !check_glyph_property(face, info, lookup_props) {
            return may_skip_t::SKIP_YES;
        }

        if info.is_default_ignorable()
            && (self.ignore_zwnj || !info.is_zwnj())
            && (self.ignore_zwj || !info.is_zwj())
            && (self.ignore_hidden || !info.is_hidden())
        {
            return may_skip_t::SKIP_MAYBE;
        }

        may_skip_t::SKIP_NO
    }
}

// In harfbuzz, skipping iterator works quite differently than it works here. In harfbuzz,
// hb_ot_apply_context contains a skipping iterator that itself contains references to font
// and buffer, meaning that we multiple borrows issue. Due to ownership rules in Rust,
// we cannot copy this approach. Because of this, we basically create a new skipping iterator
// when needed, and we do not have `init` method that exist in harfbuzz. This has a performance
// cost, and makes backporting related changes very hard, but it seems unavoidable, unfortunately.
pub struct skipping_iterator_t<'f, 'c, F> {
    pub(crate) buffer: &'c mut hb_buffer_t,
    face: &'c hb_font_t<'f>,
    matcher: &'c matcher_t,
    match_positions: &'c mut MatchPositions,
    buf_len: usize,
    glyph_data: u16,
    pub(crate) buf_idx: usize,
    match_func: Option<F>,
    lookup_props: u32,
    syllable: u8,
}

impl<'f, 'c> skipping_iterator_t<'f, 'c, fn(&mut GlyphInfo, u16) -> bool> {
    pub fn new(ctx: &'c mut hb_ot_apply_context_t<'f>, context_match: bool) -> Self {
        Self::with_match_fn(ctx, context_match, None)
    }
}

pub(crate) enum MatchSource {
    Info,
    OutInfo,
}

impl<'f, 'c, F> skipping_iterator_t<'f, 'c, F>
where
    F: Fn(&mut GlyphInfo, u16) -> bool,
{
    pub fn with_match_fn(
        ctx: &'c mut hb_ot_apply_context_t<'f>,
        context_match: bool,
        match_fn: Option<F>,
    ) -> Self {
        let matcher = if context_match {
            &ctx.context_matcher
        } else {
            &ctx.matcher
        };
        let buf_len = ctx.buffer.len;
        skipping_iterator_t {
            buffer: ctx.buffer,
            face: ctx.face,
            glyph_data: 0,
            buf_len,
            buf_idx: 0,
            matcher,
            match_func: match_fn,
            match_positions: &mut ctx.match_positions,
            lookup_props: matcher.lookup_props,
            syllable: 0,
        }
    }

    pub fn set_match_position(&mut self, idx: usize, position: usize) {
        let count = idx + 1;
        if count > self.match_positions.len() {
            self.match_positions.resize(count, 0);
        }

        self.match_positions[idx] = position as u32;
    }

    pub fn set_glyph_data(&mut self, glyph_data: u16) {
        self.glyph_data = glyph_data;
    }

    fn advance_glyph_data(&mut self) {
        self.glyph_data += 1;
    }

    pub fn set_lookup_props(&mut self, lookup_props: u32) {
        self.lookup_props = lookup_props;
    }

    pub fn index(&self) -> usize {
        self.buf_idx
    }

    #[inline]
    pub fn next(&mut self, unsafe_to: Option<&mut usize>) -> bool {
        let stop = self.buf_len as i32 - 1;

        while (self.buf_idx as i32) < stop {
            self.buf_idx += 1;

            match self.match_at(self.buf_idx, MatchSource::Info) {
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

    #[inline]
    pub fn prev(&mut self, unsafe_from: Option<&mut usize>) -> bool {
        let stop: usize = 0;

        while self.buf_idx > stop {
            self.buf_idx -= 1;

            match self.match_at(self.buf_idx, MatchSource::OutInfo) {
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

    pub fn reset(&mut self, start_index: usize) {
        // For GSUB forward iterator
        self.buf_idx = start_index;
        self.buf_len = self.buffer.len;
        self.syllable = self.buffer.cur(0).syllable();
    }

    pub fn reset_back(&mut self, start_index: usize) {
        // For GSUB backward iterator
        self.buf_idx = start_index;
        self.syllable = self.buffer.cur(0).syllable();
    }

    pub fn reset_fast(&mut self, start_index: usize) {
        // Doesn't set end or syllable. Used by GPOS which doesn't care / change.
        self.buf_idx = start_index;
    }

    pub fn may_skip(&self, info: &GlyphInfo) -> may_skip_t {
        self.matcher.may_skip(info, self.face, self.lookup_props)
    }

    #[inline]
    pub fn match_at(&mut self, idx: usize, source: MatchSource) -> match_t {
        let info = match source {
            MatchSource::Info => &mut self.buffer.info[idx],
            MatchSource::OutInfo => &mut self.buffer.out_info_mut()[idx],
        };
        let skip = self.matcher.may_skip(info, self.face, self.lookup_props);

        if skip == may_skip_t::SKIP_YES {
            return match_t::SKIP;
        }

        let _match = self.matcher.may_match(
            info,
            self.glyph_data,
            self.match_func.as_ref(),
            self.syllable,
        );

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
}

pub(crate) fn apply_lookup(
    ctx: &mut hb_ot_apply_context_t,
    input_len: usize,
    match_end: usize,
    lookups: &[SequenceLookupRecord],
) {
    let mut count = input_len + 1;

    debug_assert!(count <= ctx.match_positions.len(), "");

    // All positions are distance from beginning of *output* buffer.
    // Adjust.
    let mut end: isize = {
        let backtrack_len = ctx.buffer.backtrack_len();
        let delta = backtrack_len as isize - ctx.buffer.idx as isize;

        // Convert positions to new indexing.
        for j in 0..count {
            ctx.match_positions[j] = (ctx.match_positions[j] as isize + delta) as _;
        }

        backtrack_len as isize + match_end as isize - ctx.buffer.idx as isize
    };

    for record in lookups {
        if !ctx.buffer.successful {
            break;
        }

        let idx = usize::from(record.sequence_index.get());
        if idx >= count {
            continue;
        }

        let orig_len = ctx.buffer.backtrack_len() + ctx.buffer.lookahead_len();

        // This can happen if earlier recursed lookups deleted many entries.
        if ctx.match_positions[idx] as usize >= orig_len {
            continue;
        }

        if !ctx.buffer.move_to(ctx.match_positions[idx] as usize) {
            break;
        }

        if ctx.buffer.max_ops <= 0 {
            break;
        }

        if ctx.recurse(record.lookup_list_index.get()).is_none() {
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
        if end < ctx.match_positions[idx] as isize {
            // End might end up being smaller than match_positions[idx] if the recursed
            // lookup ended up removing many items.
            // Just never rewind end beyond start of current position, since that is
            // not possible in the recursed lookup.  Also adjust delta as such.
            //
            // https://bugs.chromium.org/p/chromium/issues/detail?id=659496
            // https://github.com/harfbuzz/harfbuzz/issues/1611
            //
            delta += ctx.match_positions[idx] as isize - end;
            end = ctx.match_positions[idx] as isize;
        }

        // next now is the position after the recursed lookup.
        let mut next = idx + 1;

        if delta > 0 {
            if delta as usize + count > MAX_CONTEXT_LENGTH {
                break;
            }

            if delta as usize + count > ctx.match_positions.len() {
                ctx.match_positions.resize(delta as usize + count, 0);
            }
        } else {
            // NOTE: delta is non-positive.
            delta = delta.max(next as isize - count as isize);
            next = (next as isize - delta) as _;
        }

        // Shift!
        ctx.match_positions
            .copy_within(next..count, (next as isize + delta) as _);
        next = (next as isize + delta) as _;
        count = (count as isize + delta) as _;
        ctx.match_positions_len = count;

        // Fill in new entries.
        for j in idx + 1..next {
            ctx.match_positions[j] = ctx.match_positions[j - 1] + 1;
        }

        // And fixup the rest.
        while next < count {
            ctx.match_positions[next] = (ctx.match_positions[next] as isize + delta) as _;
            next += 1;
        }
    }

    ctx.buffer.move_to(end.try_into().unwrap());
}

/// Find out whether a lookup would be applied.
pub trait WouldApply {
    /// Whether the lookup would be applied.
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool;
}

// HB uses a cache size of 128 here; we double it to reduce collisions
// since our lookup is slower.
pub(crate) type MappingCache = hb_cache_t<
    16,  // KEY_BITS
    8,   // VALUE_BITS
    256, // CACHE_SIZE
    16,  // STORAGE_BITS
>;

pub(crate) type BinaryCache = hb_cache_t<
    15,  // KEY_BITS
    1,   // VALUE_BITS
    256, // CACHE_SIZE
    8,   // STORAGE_BITS
>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum SubtableExternalCacheMode {
    #[allow(unused)]
    None,
    Small,
    Full,
}

pub(crate) struct LigatureSubstFormat1Cache {
    pub seconds: hb_set_digest_t,
    pub coverage: MappingCache,
}

impl LigatureSubstFormat1Cache {
    pub fn new(seconds: hb_set_digest_t) -> Self {
        LigatureSubstFormat1Cache {
            coverage: MappingCache::new(),
            seconds,
        }
    }
}

pub(crate) struct LigatureSubstFormat1SmallCache {
    pub coverage: CoverageInfo,
    pub seconds: hb_set_digest_t,
}

pub(crate) struct PairPosFormat1Cache {
    pub coverage: MappingCache,
}

impl PairPosFormat1Cache {
    pub fn new() -> Self {
        PairPosFormat1Cache {
            coverage: MappingCache::new(),
        }
    }
}

pub(crate) struct PairPosFormat1SmallCache {
    pub coverage: CoverageInfo,
}

pub(crate) struct PairPosFormat2Cache {
    pub coverage: MappingCache,
    pub first: MappingCache,
    pub second: MappingCache,
}

impl PairPosFormat2Cache {
    pub fn new() -> Self {
        PairPosFormat2Cache {
            coverage: MappingCache::new(),
            first: MappingCache::new(),
            second: MappingCache::new(),
        }
    }
}

pub(crate) struct PairPosFormat2SmallCache {
    pub coverage: CoverageInfo,
    pub first: ClassDefInfo,
    pub second: ClassDefInfo,
}

pub(crate) struct ContextFormat2Cache {
    pub coverage: CoverageInfo,
    pub input: ClassDefInfo,
    pub coverage_cache: BinaryCache,
}

pub(crate) struct ChainContextFormat2Cache {
    pub coverage: CoverageInfo,
    pub backtrack: ClassDefInfo,
    pub input: ClassDefInfo,
    pub lookahead: ClassDefInfo,
    pub coverage_cache: BinaryCache,
}

pub(crate) enum SubtableExternalCache {
    None,
    LigatureSubstFormat1Cache(Box<LigatureSubstFormat1Cache>),
    LigatureSubstFormat1SmallCache(LigatureSubstFormat1SmallCache),
    PairPosFormat1Cache(Box<PairPosFormat1Cache>),
    PairPosFormat1SmallCache(PairPosFormat1SmallCache),
    PairPosFormat2Cache(Box<PairPosFormat2Cache>),
    PairPosFormat2SmallCache(PairPosFormat2SmallCache),
    ContextFormat2Cache(ContextFormat2Cache),
    ChainContextFormat2Cache(ChainContextFormat2Cache),
}

/// Apply a lookup.
pub trait Apply {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        // Default implementation just calls `apply_with_external_cache`.
        self.apply_with_external_cache(ctx, &SubtableExternalCache::None)
    }

    // The rest are relevant to subtables only

    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        _external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        // Default implementation just calls `apply`.
        self.apply(ctx)
    }

    fn apply_cached(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        // Default implementation just calls `apply_with_external_cache`.
        // This is used to apply the lookup with glyph-info caching.
        self.apply_with_external_cache(ctx, external_cache)
    }

    fn cache_cost(&self) -> u32 {
        // Default implementation returns 0, meaning no cache cost.
        // This is used to determine the cost of caching the subtable.
        0
    }

    fn external_cache_create(&self, mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        // Default implementation returns None, meaning no external cache.
        // This is used to create an external cache for the subtable.
        let _ = mode;
        SubtableExternalCache::None
    }
}

pub struct WouldApplyContext<'a> {
    pub glyphs: &'a [GlyphId],
    pub zero_context: bool,
}

pub mod OT {
    use super::*;

    fn match_properties_mark(
        face: &hb_font_t,
        info: &GlyphInfo,
        glyph_props: u16,
        match_props: u32,
    ) -> bool {
        // If using mark filtering sets, the high short of
        // match_props has the set index.
        if match_props as u16 & lookup_flags::USE_MARK_FILTERING_SET != 0 {
            let set_index = (match_props >> 16) as u16;
            return face
                .ot_tables
                .is_mark_glyph(info.as_glyph().to_u32(), set_index);
        }

        // The second byte of match_props has the meaning
        // "ignore marks of attachment type different than
        // the attachment type specified."
        if match_props as u16 & lookup_flags::MARK_ATTACHMENT_TYPE_MASK != 0 {
            return (match_props as u16 & lookup_flags::MARK_ATTACHMENT_TYPE_MASK)
                == (glyph_props & lookup_flags::MARK_ATTACHMENT_TYPE_MASK);
        }

        true
    }

    #[inline(always)]
    pub fn check_glyph_property(face: &hb_font_t, info: &GlyphInfo, match_props: u32) -> bool {
        let glyph_props = info.glyph_props();

        // Not covered, if, for example, glyph class is ligature and
        // match_props includes LookupFlags::IgnoreLigatures
        if glyph_props & match_props as u16 & lookup_flags::IGNORE_FLAGS != 0 {
            return false;
        }

        if glyph_props & GlyphPropsFlags::MARK.bits() != 0 {
            return match_properties_mark(face, info, glyph_props, match_props);
        }

        true
    }

    pub struct hb_ot_apply_context_t<'a> {
        pub table_index: TableIndex,
        pub face: &'a hb_font_t<'a>,
        pub buffer: &'a mut hb_buffer_t,
        lookup_mask: hb_mask_t,
        pub per_syllable: bool,
        pub lookup_index: u16,
        pub lookup_props: u32,
        pub nesting_level_left: usize,
        pub auto_zwnj: bool,
        pub auto_zwj: bool,
        pub random: bool,
        pub random_state: u32,
        pub new_syllables: Option<u8>,
        pub last_base: i32,
        pub last_base_until: u32,
        pub(crate) matcher: matcher_t,
        pub(crate) context_matcher: matcher_t,
        pub(crate) match_positions_len: usize,
        pub(crate) match_positions: MatchPositions,
    }

    impl<'a> hb_ot_apply_context_t<'a> {
        pub fn new(
            table_index: TableIndex,
            face: &'a hb_font_t<'a>,
            buffer: &'a mut hb_buffer_t,
        ) -> Self {
            Self {
                table_index,
                face,
                buffer,
                lookup_mask: 1,
                per_syllable: false,
                lookup_index: u16::MAX,
                lookup_props: u32::MAX,
                nesting_level_left: MAX_NESTING_LEVEL,
                auto_zwnj: true,
                auto_zwj: true,
                random: false,
                random_state: 1,
                new_syllables: None,
                last_base: -1,
                last_base_until: 0,
                matcher: matcher_t::default(),
                context_matcher: matcher_t::default(),
                match_positions_len: 0,
                match_positions: MatchPositions::from_elem(0, 1),
            }
        }

        pub fn random_number(&mut self) -> u32 {
            // http://www.cplusplus.com/reference/random/minstd_rand/
            self.random_state = self.random_state.wrapping_mul(48271) % (i32::MAX as u32);
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

        pub fn update_matchers(&mut self) {
            self.matcher = matcher_t::new(self, false);
            self.context_matcher = matcher_t::new(self, true);
        }

        pub fn recurse(&mut self, sub_lookup_index: u16) -> Option<()> {
            if self.nesting_level_left == 0 {
                self.buffer.successful = false;
                return None;
            }

            self.buffer.max_ops -= 1;
            if self.buffer.max_ops < 0 {
                self.buffer.successful = false;
                return None;
            }

            self.nesting_level_left -= 1;
            let saved_props = self.lookup_props;
            let saved_index = self.lookup_index;

            self.match_positions.resize(self.match_positions_len, 0);
            let saved_match_positions = self.match_positions.clone();
            let saved_match_positions_len = self.match_positions_len;

            self.lookup_index = sub_lookup_index;
            let applied = self
                .face
                .ot_tables
                .table_data_and_lookup(self.table_index, sub_lookup_index)
                .and_then(|(table_data, lookup)| {
                    self.lookup_props = lookup.props();
                    self.update_matchers();
                    lookup.apply(self, table_data, false)
                });
            self.lookup_props = saved_props;
            self.lookup_index = saved_index;
            self.update_matchers();
            self.match_positions = saved_match_positions;
            self.match_positions_len = saved_match_positions_len;
            self.nesting_level_left += 1;
            applied
        }

        fn set_glyph_class(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
            ligature: bool,
            component: bool,
        ) {
            self.buffer.digest.add(glyph_id.into());

            if let Some(syllable) = self.new_syllables {
                self.buffer.cur_mut(0).set_syllable(syllable);
            }

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

            let has_glyph_classes = self.face.ot_tables.has_glyph_classes();

            if has_glyph_classes {
                props &= GlyphPropsFlags::PRESERVE.bits();
                cur.set_glyph_props(props | self.face.ot_tables.glyph_props(glyph_id));
            } else if !class_guess.is_empty() {
                props &= GlyphPropsFlags::PRESERVE.bits();
                cur.set_glyph_props(props | class_guess.bits());
            } else {
                cur.set_glyph_props(props);
            }
        }

        pub fn replace_glyph(&mut self, glyph_id: GlyphId) {
            self.set_glyph_class(glyph_id, GlyphPropsFlags::empty(), false, false);
            self.buffer.replace_glyph(u32::from(glyph_id));
        }

        pub fn replace_glyph_inplace(&mut self, glyph_id: GlyphId) {
            self.set_glyph_class(glyph_id, GlyphPropsFlags::empty(), false, false);
            self.buffer.cur_mut(0).glyph_id = u32::from(glyph_id);
        }

        pub fn replace_glyph_with_ligature(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
        ) {
            self.set_glyph_class(glyph_id, class_guess, true, false);
            self.buffer.replace_glyph(u32::from(glyph_id));
        }

        pub fn output_glyph_for_component(
            &mut self,
            glyph_id: GlyphId,
            class_guess: GlyphPropsFlags,
        ) {
            self.set_glyph_class(glyph_id, class_guess, false, true);
            self.buffer.output_glyph(u32::from(glyph_id));
        }
    }
}

use OT::hb_ot_apply_context_t;

pub fn ligate_input(
    ctx: &mut hb_ot_apply_context_t,
    // Including the first glyph
    count: usize,
    // Including the first glyph
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

    let mut is_base_ligature = buffer.info[ctx.match_positions[0] as usize].is_base_glyph();
    let mut is_mark_ligature = buffer.info[ctx.match_positions[0] as usize].is_mark();
    for i in 1..count {
        if !buffer.info[ctx.match_positions[i] as usize].is_mark() {
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
    let mut last_lig_id = first.lig_id();
    let mut last_num_comps = first.lig_num_comps();
    let mut comps_so_far = last_num_comps;

    if is_ligature {
        first.set_lig_props_for_ligature(lig_id, total_component_count);
        if first.general_category() == GeneralCategory::NON_SPACING_MARK {
            first.set_general_category(GeneralCategory::OTHER_LETTER);
        }
    }

    ctx.replace_glyph_with_ligature(lig_glyph, class);
    buffer = &mut ctx.buffer;

    for i in 1..count {
        while buffer.idx < ctx.match_positions[i] as usize && buffer.successful {
            if is_ligature {
                let cur = buffer.cur_mut(0);
                let mut this_comp = cur.lig_comp();
                if this_comp == 0 {
                    this_comp = last_num_comps;
                }
                // Avoid the potential for a wrap-around bug when subtracting from an unsigned integer
                // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
                debug_assert!(comps_so_far >= last_num_comps);
                let new_lig_comp = comps_so_far - last_num_comps + this_comp.min(last_num_comps);
                cur.set_lig_props_for_mark(lig_id, new_lig_comp);
            }
            buffer.next_glyph();
        }

        let cur = buffer.cur(0);
        last_lig_id = cur.lig_id();
        last_num_comps = cur.lig_num_comps();
        comps_so_far += last_num_comps;

        // Skip the base glyph.
        buffer.idx += 1;
    }

    if !is_mark_ligature && last_lig_id != 0 {
        // Re-adjust components for any marks following.
        for i in buffer.idx..buffer.len {
            let info = &mut buffer.info[i];
            if last_lig_id != info.lig_id() {
                break;
            }

            let this_comp = info.lig_comp();
            if this_comp == 0 {
                break;
            }

            // Avoid the potential for a wrap-around bug when subtracting from an unsigned integer
            // c.f. https://github.com/harfbuzz/rustybuzz/issues/142
            debug_assert!(comps_so_far >= last_num_comps);
            let new_lig_comp = comps_so_far - last_num_comps + this_comp.min(last_num_comps);
            info.set_lig_props_for_mark(lig_id, new_lig_comp);
        }
    }
}
