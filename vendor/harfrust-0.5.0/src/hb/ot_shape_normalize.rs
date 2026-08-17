use crate::hb::unicode::Codepoint;

use super::buffer::*;
use super::hb_font_t;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::{ComposeFn, DecomposeFn, MAX_COMBINING_MARKS};
use super::unicode::{hb_unicode_funcs_t, CharExt};

impl GlyphInfo {
    declare_buffer_var!(
        u32,
        1,
        0,
        NORMALIZER_GLYPH_INDEX_VAR,
        normalizer_glyph_index,
        set_normalizer_glyph_index
    );
}

pub struct hb_ot_shape_normalize_context_t<'a> {
    pub plan: &'a hb_ot_shape_plan_t,
    pub buffer: &'a mut hb_buffer_t,
    pub face: &'a hb_font_t<'a>,
    pub decompose: DecomposeFn,
    pub compose: ComposeFn,
}

impl hb_ot_shape_normalize_context_t<'_> {
    pub(crate) fn override_decompose_and_compose(
        &mut self,
        decompose: Option<DecomposeFn>,
        compose: Option<ComposeFn>,
    ) {
        if let Some(decompose) = decompose {
            self.decompose = decompose;
        }

        if let Some(compose) = compose {
            self.compose = compose;
        }
    }
}

pub type hb_ot_shape_normalization_mode_t = i32;
pub const HB_OT_SHAPE_NORMALIZATION_MODE_NONE: i32 = 0;
pub const HB_OT_SHAPE_NORMALIZATION_MODE_DECOMPOSED: i32 = 1;
pub const HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS: i32 = 2; /* Never composes base-to-base */
pub const HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS_NO_SHORT_CIRCUIT: i32 = 3; /* Always fully decomposes and then recompose back */
pub const HB_OT_SHAPE_NORMALIZATION_MODE_AUTO: i32 = 4; /* See hb-ot-shape-normalize.cc for logic. */
#[allow(dead_code)]
pub const HB_OT_SHAPE_NORMALIZATION_MODE_DEFAULT: i32 = HB_OT_SHAPE_NORMALIZATION_MODE_AUTO;

// HIGHLEVEL DESIGN:
//
// This file exports one main function: normalize().
//
// This function closely reflects the Unicode Normalization Algorithm,
// yet it's different.
//
// Each shaper specifies whether it prefers decomposed (NFD) or composed (NFC).
// The logic however tries to use whatever the font can support.
//
// In general what happens is that: each grapheme is decomposed in a chain
// of 1:2 decompositions, marks reordered, and then recomposed if desired,
// so far it's like Unicode Normalization.  However, the decomposition and
// recomposition only happens if the font supports the resulting characters.
//
// The goals are:
//
//   - Try to render all canonically equivalent strings similarly.  To really
//     achieve this we have to always do the full decomposition and then
//     selectively recompose from there.  It's kinda too expensive though, so
//     we skip some cases.  For example, if composed is desired, we simply
//     don't touch 1-character clusters that are supported by the font, even
//     though their NFC may be different.
//
//   - When a font has a precomposed character for a sequence but the 'ccmp'
//     feature in the font is not adequate, use the precomposed character
//     which typically has better mark positioning.
//
//   - When a font does not support a combining mark, but supports it precomposed
//     with previous base, use that.  This needs the itemizer to have this
//     knowledge too.  We need to provide assistance to the itemizer.
//
//   - When a font does not support a character but supports its canonical
//     decomposition, well, use the decomposition.
//
//   - The shapers can customize the compose and decompose functions to
//     offload some of their requirements to the normalizer.  For example, the
//     Indic shaper may want to disallow recomposing of two matras.

fn decompose_unicode(
    _: &hb_ot_shape_normalize_context_t,
    ab: Codepoint,
) -> Option<(Codepoint, Codepoint)> {
    super::unicode::decompose(ab)
}

fn compose_unicode(
    _: &hb_ot_shape_normalize_context_t,
    a: Codepoint,
    b: Codepoint,
) -> Option<Codepoint> {
    super::unicode::compose(a, b)
}

fn set_glyph(info: &mut GlyphInfo, font: &hb_font_t) {
    if let Some(glyph_id) = font.get_nominal_glyph(info.glyph_id) {
        info.set_normalizer_glyph_index(u32::from(glyph_id));
    }
}

fn output_char(buffer: &mut hb_buffer_t, unichar: u32, glyph: u32) {
    // This is very confusing indeed.
    buffer.cur_mut(0).set_normalizer_glyph_index(glyph);
    buffer.output_glyph(unichar);
    // TODO: should be _hb_glyph_info_set_unicode_props (&buffer->prev(), buffer);
    let mut flags = buffer.scratch_flags;
    buffer.prev_mut().init_unicode_props(&mut flags);
    buffer.scratch_flags = flags;
}

fn next_char(buffer: &mut hb_buffer_t, glyph: u32) {
    buffer.cur_mut(0).set_normalizer_glyph_index(glyph);
    buffer.next_glyph();
}

fn skip_char(buffer: &mut hb_buffer_t) {
    buffer.skip_glyph();
}

/// Returns 0 if didn't decompose, number of resulting characters otherwise.
fn decompose(ctx: &mut hb_ot_shape_normalize_context_t, shortest: bool, ab: Codepoint) -> u32 {
    let Some((a, b)) = (ctx.decompose)(ctx, ab) else {
        return 0;
    };

    let a_glyph = ctx.face.get_nominal_glyph(a);
    let b_glyph = if b != 0 {
        match ctx.face.get_nominal_glyph(b) {
            Some(glyph_id) => Some(glyph_id),
            None => return 0,
        }
    } else {
        None
    };

    if !shortest || a_glyph.is_none() {
        let ret = decompose(ctx, shortest, a);
        if ret != 0 {
            if let Some(b_glyph) = b_glyph {
                output_char(ctx.buffer, b, u32::from(b_glyph));
                return ret + 1;
            }
            return ret;
        }
    }

    if let Some(a_glyph) = a_glyph {
        // Output a and b.
        output_char(ctx.buffer, a, u32::from(a_glyph));
        if let Some(b_glyph) = b_glyph {
            output_char(ctx.buffer, b, u32::from(b_glyph));
            return 2;
        }
        return 1;
    }

    0
}

fn decompose_current_character(ctx: &mut hb_ot_shape_normalize_context_t, shortest: bool) {
    let u = ctx.buffer.cur(0).as_codepoint();
    let glyph = ctx.face.get_nominal_glyph(u);

    // TODO: different to harfbuzz, sync
    if !shortest || glyph.is_none() {
        if decompose(ctx, shortest, u) > 0 {
            skip_char(ctx.buffer);
            return;
        }
    }

    // TODO: different to harfbuzz, sync
    if let Some(glyph) = glyph {
        next_char(ctx.buffer, u32::from(glyph));
        return;
    }

    if ctx.buffer.cur(0).is_unicode_space() {
        let space_type = u.space_fallback();
        if space_type != hb_unicode_funcs_t::NOT_SPACE {
            let space_glyph = ctx.face.get_nominal_glyph(0x0020).or(ctx.buffer.invisible);

            if let Some(space_glyph) = space_glyph {
                ctx.buffer
                    .cur_mut(0)
                    .set_unicode_space_fallback_type(space_type);
                next_char(ctx.buffer, u32::from(space_glyph));
                ctx.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_SPACE_FALLBACK;
                return;
            }
        }
    }

    // U+2011 is the only sensible character that is a no-break version of another character
    // and not a space.  The space ones are handled already.  Handle this lone one.
    if u == 0x2011 {
        if let Some(other_glyph) = ctx.face.get_nominal_glyph(0x2010) {
            next_char(ctx.buffer, u32::from(other_glyph));
            return;
        }
    }

    // Insert a .notdef glyph if decomposition failed.
    next_char(ctx.buffer, 0);
}

fn handle_variation_selector_cluster(
    ctx: &mut hb_ot_shape_normalize_context_t,
    end: usize,
    _: bool,
) {
    let face = ctx.face;

    // Currently if there's a variation-selector we give-up on normalization, it's just too hard.
    let buffer = &mut ctx.buffer;
    while buffer.idx < end - 1 && buffer.successful {
        if buffer.cur(1).as_codepoint().is_variation_selector() {
            if let Some(glyph_id) = face.get_nominal_variant_glyph(
                buffer.cur(0).as_codepoint(),
                buffer.cur(1).as_codepoint(),
            ) {
                buffer
                    .cur_mut(0)
                    .set_normalizer_glyph_index(u32::from(glyph_id));
                let unicode = buffer.cur(0).glyph_id;
                buffer.replace_glyphs(2, 1, &[unicode]);
            } else {
                // Just pass on the two characters separately, let GSUB do its magic.
                set_glyph(buffer.cur_mut(0), face);
                buffer.next_glyph();

                buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_VARIATION_SELECTOR_FALLBACK;

                buffer.cur_mut(0).set_variation_selector(true);

                if buffer.not_found_variation_selector.is_some() {
                    buffer.cur_mut(0).clear_default_ignorable();
                }

                set_glyph(buffer.cur_mut(0), face);
                buffer.next_glyph();
            }

            // Skip any further variation selectors.
            while buffer.idx < end && buffer.cur(0).as_codepoint().is_variation_selector() {
                set_glyph(buffer.cur_mut(0), face);
                buffer.next_glyph();
            }
        } else {
            set_glyph(buffer.cur_mut(0), face);
            buffer.next_glyph();
        }
    }

    if ctx.buffer.idx < end {
        set_glyph(ctx.buffer.cur_mut(0), face);
        ctx.buffer.next_glyph();
    }
}

fn decompose_multi_char_cluster(
    ctx: &mut hb_ot_shape_normalize_context_t,
    end: usize,
    short_circuit: bool,
) {
    let mut i = ctx.buffer.idx;
    while i < end && ctx.buffer.successful {
        if ctx.buffer.info[i].as_codepoint().is_variation_selector() {
            handle_variation_selector_cluster(ctx, end, short_circuit);
            return;
        }
        i += 1;
    }

    while ctx.buffer.idx < end && ctx.buffer.successful {
        decompose_current_character(ctx, short_circuit);
    }
}

fn compare_combining_class(pa: &GlyphInfo, pb: &GlyphInfo) -> bool {
    let a = pa.modified_combining_class();
    let b = pb.modified_combining_class();
    a > b
}

pub fn _hb_ot_shape_normalize(
    plan: &hb_ot_shape_plan_t,
    buffer: &mut hb_buffer_t,
    face: &hb_font_t,
) {
    if buffer.is_empty() {
        return;
    }

    buffer.assert_unicode_vars();

    let mut mode = plan.shaper.normalization_preference;
    if mode == HB_OT_SHAPE_NORMALIZATION_MODE_AUTO {
        if plan.has_gpos_mark {
            // https://github.com/harfbuzz/harfbuzz/issues/653#issuecomment-423905920
            // mode = Some(HB_OT_SHAPE_NORMALIZATION_MODE_DECOMPOSED);
            mode = HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS;
        } else {
            mode = HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS;
        }
    }

    let mut ctx = hb_ot_shape_normalize_context_t {
        plan,
        buffer,
        face,
        decompose: decompose_unicode,
        compose: compose_unicode,
    };
    ctx.override_decompose_and_compose(plan.shaper.decompose, plan.shaper.compose);

    let mut buffer = &mut ctx.buffer;

    let always_short_circuit = mode == HB_OT_SHAPE_NORMALIZATION_MODE_NONE;
    let might_short_circuit = always_short_circuit
        || (mode != HB_OT_SHAPE_NORMALIZATION_MODE_DECOMPOSED
            && mode != HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS_NO_SHORT_CIRCUIT);

    // We do a fairly straightforward yet custom normalization process in three
    // separate rounds: decompose, reorder, recompose (if desired).  Currently
    // this makes two buffer swaps.  We can make it faster by moving the last
    // two rounds into the inner loop for the first round, but it's more readable
    // this way.

    // First round, decompose
    let mut all_simple = true;
    {
        buffer.clear_output();
        let count = buffer.len;
        buffer.idx = 0;
        loop {
            let mut end = buffer.idx + 1;
            while end < count && !buffer.info[end].is_unicode_mark() {
                end += 1;
            }

            if end < count {
                // Leave one base for the marks to cluster with.
                end -= 1;
            }

            // From idx to end are simple clusters.
            if might_short_circuit {
                let len = end - buffer.idx;
                let mut done = 0;
                while done < len {
                    let cur = buffer.cur_mut(done);
                    cur.set_normalizer_glyph_index(match face.get_nominal_glyph(cur.glyph_id) {
                        Some(glyph_id) => u32::from(glyph_id),
                        None => break,
                    });
                    done += 1;
                }
                buffer.next_glyphs(done);
            }

            while buffer.idx < end && buffer.successful {
                decompose_current_character(&mut ctx, might_short_circuit);
                buffer = &mut ctx.buffer;
            }

            if buffer.idx == count || !buffer.successful {
                break;
            }

            all_simple = false;

            // Find all the marks now.
            end = buffer.idx + 1;
            while end < count && buffer.info[end].is_unicode_mark() {
                end += 1;
            }

            // idx to end is one non-simple cluster.
            decompose_multi_char_cluster(&mut ctx, end, always_short_circuit);
            buffer = &mut ctx.buffer;

            if buffer.idx >= count || !buffer.successful {
                break;
            }
        }

        buffer.sync();
    }

    // Second round, reorder (inplace)
    if !all_simple {
        let count = buffer.len;
        let mut i = 0;
        while i < count {
            if buffer.info[i].modified_combining_class() == 0 {
                i += 1;
                continue;
            }

            let mut end = i + 1;
            while end < count && buffer.info[end].modified_combining_class() != 0 {
                end += 1;
            }

            // We are going to do a O(n^2).  Only do this if the sequence is short.
            if end - i <= MAX_COMBINING_MARKS {
                buffer.sort(i, end, compare_combining_class);

                if let Some(reorder_marks) = ctx.plan.shaper.reorder_marks {
                    reorder_marks(ctx.plan, buffer, i, end);
                }
            }

            i = end + 1;
        }
    }
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_CGJ != 0 {
        // For all CGJ, check if it prevented any reordering at all.
        // If it did NOT, then make it skippable.
        // https://github.com/harfbuzz/harfbuzz/issues/554
        for i in 1..buffer.len.saturating_sub(1) {
            if buffer.info[i].glyph_id == 0x034F
            /* CGJ */
            {
                let last = buffer.info[i - 1].modified_combining_class();
                let next = buffer.info[i + 1].modified_combining_class();
                if next == 0 || last <= next {
                    buffer.info[i].unhide();
                }
            }
        }
    }

    // Third round, recompose
    if !all_simple
        && buffer.successful
        && (mode == HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS
            || mode == HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS_NO_SHORT_CIRCUIT)
    {
        // As noted in the comment earlier, we don't try to combine
        // ccc=0 chars with their previous Starter.

        let count = buffer.len;
        let mut starter = 0;
        buffer.clear_output();
        buffer.next_glyph();
        while buffer.idx < count && buffer.successful {
            // We don't try to compose a non-mark character with it's preceding starter.
            // This is both an optimization to avoid trying to compose every two neighboring
            // glyphs in most scripts AND a desired feature for Hangul.  Apparently Hangul
            // fonts are not designed to mix-and-match pre-composed syllables and Jamo.
            let cur = buffer.cur(0);
            if cur.is_unicode_mark() &&
                // If there's anything between the starter and this char, they should have CCC
                // smaller than this character's.
                (starter == buffer.out_len - 1
                    || buffer.prev().modified_combining_class() < cur.modified_combining_class())
            {
                let a = buffer.out_info()[starter].as_codepoint();
                let b = cur.as_codepoint();
                if let Some(composed) = (ctx.compose)(&ctx, a, b) {
                    if let Some(glyph_id) = face.get_nominal_glyph(composed) {
                        // Copy to out-buffer.
                        buffer = &mut ctx.buffer;
                        buffer.next_glyph();
                        if !buffer.successful {
                            return;
                        }

                        // Merge and remove the second composable.
                        buffer.merge_out_clusters(starter, buffer.out_len);
                        buffer.out_len -= 1;

                        // Modify starter and carry on.
                        let mut flags = buffer.scratch_flags;
                        let info = &mut buffer.out_info_mut()[starter];
                        info.glyph_id = composed;
                        info.set_normalizer_glyph_index(u32::from(glyph_id));
                        info.init_unicode_props(&mut flags);
                        buffer.scratch_flags = flags;

                        continue;
                    }
                }
            }

            // Blocked, or doesn't compose.
            buffer = &mut ctx.buffer;
            buffer.next_glyph();

            if buffer.prev().modified_combining_class() == 0 {
                starter = buffer.out_len - 1;
            }
        }

        buffer.sync();
    }
}
