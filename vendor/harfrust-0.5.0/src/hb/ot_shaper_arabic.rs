use crate::Direction;
use alloc::boxed::Box;

use super::algs::*;
use super::buffer::*;
use super::ot_map::*;
use super::ot_shape::*;
use super::ot_shape_normalize::HB_OT_SHAPE_NORMALIZATION_MODE_AUTO;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::*;
use super::unicode::*;
use super::{hb_font_t, hb_mask_t, hb_tag_t, script, GlyphInfo, Script};

const HB_BUFFER_SCRATCH_FLAG_ARABIC_HAS_STCH: hb_buffer_scratch_flags_t =
    HB_BUFFER_SCRATCH_FLAG_SHAPER0;

// See:
// https://github.com/harfbuzz/harfbuzz/commit/6e6f82b6f3dde0fc6c3c7d991d9ec6cfff57823d#commitcomment-14248516
fn is_word_category(gc: GeneralCategory) -> bool {
    (rb_flag_unsafe(gc.to_u8() as u32)
        & (rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_SPACING_MARK)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL)))
        != 0
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum hb_arabic_joining_type_t {
    U = 0,
    L = 1,
    R = 2,
    D = 3,
    // We don't have C, like harfbuzz, because Rust doesn't allow duplicated enum variants.
    GroupAlaph = 4,
    GroupDalathRish = 5,
    T = 7,
    X = 8, // means: use general-category to choose between U or T.
}

fn get_joining_type(u: Codepoint, gc: GeneralCategory) -> hb_arabic_joining_type_t {
    let j_type = super::ot_shaper_arabic_table::joining_type(u);
    if j_type != hb_arabic_joining_type_t::X {
        return j_type;
    }

    let ok = rb_flag_unsafe(gc.to_u8() as u32)
        & (rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK)
            | rb_flag(hb_gc::HB_UNICODE_GENERAL_CATEGORY_FORMAT));

    if ok != 0 {
        hb_arabic_joining_type_t::T
    } else {
        hb_arabic_joining_type_t::U
    }
}

fn feature_is_syriac(tag: hb_tag_t) -> bool {
    matches!(tag.to_be_bytes()[3], b'2' | b'3')
}

const ARABIC_FEATURES: &[hb_tag_t] = &[
    hb_tag_t::new(b"isol"),
    hb_tag_t::new(b"fina"),
    hb_tag_t::new(b"fin2"),
    hb_tag_t::new(b"fin3"),
    hb_tag_t::new(b"medi"),
    hb_tag_t::new(b"med2"),
    hb_tag_t::new(b"init"),
];

mod arabic_action_t {
    pub const ISOL: u8 = 0;
    pub const FINA: u8 = 1;
    pub const FIN2: u8 = 2;
    pub const FIN3: u8 = 3;
    pub const MEDI: u8 = 4;
    pub const MED2: u8 = 5;
    pub const INIT: u8 = 6;
    pub const NONE: u8 = 7;

    // We abuse the same byte for other things...
    pub const STRETCHING_FIXED: u8 = 8;
    pub const STRETCHING_REPEATING: u8 = 9;

    #[inline]
    pub fn is_stch(n: u8) -> bool {
        matches!(n, STRETCHING_FIXED | STRETCHING_REPEATING)
    }
}

static STATE_TABLE: &[[(u8, u8, u16); 6]] = &[
    // jt_U,          jt_L,          jt_R,
    // jt_D,          jg_ALAPH,      jg_DALATH_RISH

    // State 0: prev was U, not willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 1),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 1),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 6),
    ],
    // State 1: prev was R or action::ISOL/ALAPH, not willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 1),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::FIN2, 5),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 6),
    ],
    // State 2: prev was D/L in action::ISOL form, willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::INIT, arabic_action_t::FINA, 1),
        (arabic_action_t::INIT, arabic_action_t::FINA, 3),
        (arabic_action_t::INIT, arabic_action_t::FINA, 4),
        (arabic_action_t::INIT, arabic_action_t::FINA, 6),
    ],
    // State 3: prev was D in action::FINA form, willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::MEDI, arabic_action_t::FINA, 1),
        (arabic_action_t::MEDI, arabic_action_t::FINA, 3),
        (arabic_action_t::MEDI, arabic_action_t::FINA, 4),
        (arabic_action_t::MEDI, arabic_action_t::FINA, 6),
    ],
    // State 4: prev was action::FINA ALAPH, not willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::MED2, arabic_action_t::ISOL, 1),
        (arabic_action_t::MED2, arabic_action_t::ISOL, 2),
        (arabic_action_t::MED2, arabic_action_t::FIN2, 5),
        (arabic_action_t::MED2, arabic_action_t::ISOL, 6),
    ],
    // State 5: prev was FIN2/FIN3 ALAPH, not willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::ISOL, arabic_action_t::ISOL, 1),
        (arabic_action_t::ISOL, arabic_action_t::ISOL, 2),
        (arabic_action_t::ISOL, arabic_action_t::FIN2, 5),
        (arabic_action_t::ISOL, arabic_action_t::ISOL, 6),
    ],
    // State 6: prev was DALATH/RISH, not willing to join.
    [
        (arabic_action_t::NONE, arabic_action_t::NONE, 0),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 1),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 2),
        (arabic_action_t::NONE, arabic_action_t::FIN3, 5),
        (arabic_action_t::NONE, arabic_action_t::ISOL, 6),
    ],
];

impl GlyphInfo {
    declare_buffer_var_alias!(
        OT_SHAPER_VAR_U8_AUXILIARY_VAR,
        u8,
        ARABIC_SHAPING_ACTION_VAR,
        arabic_shaping_action,
        set_arabic_shaping_action
    );
}

fn deallocate_buffer_var(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    buffer.deallocate_var(GlyphInfo::ARABIC_SHAPING_ACTION_VAR);

    false
}

fn collect_features(planner: &mut hb_ot_shape_planner_t) {
    // We apply features according to the Arabic spec, with pauses
    // in between most.
    //
    // The pause between init/medi/... and rlig is required.  See eg:
    // https://bugzilla.mozilla.org/show_bug.cgi?id=644184
    //
    // The pauses between init/medi/... themselves are not necessarily
    // needed as only one of those features is applied to any character.
    // The only difference it makes is when fonts have contextual
    // substitutions.  We now follow the order of the spec, which makes
    // for better experience if that's what Uniscribe is doing.
    //
    // At least for Arabic, looks like Uniscribe has a pause between
    // rlig and calt.  Otherwise the IranNastaliq's ALLAH ligature won't
    // work.  However, testing shows that rlig and calt are applied
    // together for Mongolian in Uniscribe.  As such, we only add a
    // pause for Arabic, not other scripts.
    //
    // A pause after calt is required to make KFGQPC Uthmanic Script HAFS
    // work correctly.  See https://github.com/harfbuzz/harfbuzz/issues/505

    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"stch"), F_NONE, 1);
    planner.ot_map.add_gsub_pause(Some(record_stch));

    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"ccmp"), F_MANUAL_ZWJ, 1);
    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"locl"), F_MANUAL_ZWJ, 1);

    planner.ot_map.add_gsub_pause(None);

    for feature in ARABIC_FEATURES {
        let has_fallback = planner.script == Some(script::ARABIC) && !feature_is_syriac(*feature);
        let flags = if has_fallback { F_HAS_FALLBACK } else { F_NONE };
        planner
            .ot_map
            .add_feature(*feature, F_MANUAL_ZWJ | flags, 1);
        planner.ot_map.add_gsub_pause(None);
    }
    planner.ot_map.add_gsub_pause(Some(deallocate_buffer_var));

    // Normally, Unicode says a ZWNJ means "don't ligate".  In Arabic script
    // however, it says a ZWJ should also mean "don't ligate".  So we run
    // the main ligating features as MANUAL_ZWJ.

    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"rlig"), F_MANUAL_ZWJ | F_HAS_FALLBACK, 1);

    if planner.script == Some(script::ARABIC) {
        planner.ot_map.add_gsub_pause(Some(arabic_fallback_shape));
    }

    // No pause after rclt.
    // See 98460779bae19e4d64d29461ff154b3527bf8420
    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"calt"), F_MANUAL_ZWJ, 1);
    /* https://github.com/harfbuzz/harfbuzz/issues/1573 */
    if !planner.ot_map.has_feature(hb_tag_t::new(b"rclt")) {
        planner.ot_map.add_gsub_pause(None);
    }

    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"liga"), F_MANUAL_ZWJ, 1);
    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"clig"), F_MANUAL_ZWJ, 1);

    // The spec includes 'cswh'.  Earlier versions of Windows
    // used to enable this by default, but testing suggests
    // that Windows 8 and later do not enable it by default,
    // and spec now says 'Off by default'.
    // We disabled this in ae23c24c32.
    // Note that IranNastaliq uses this feature extensively
    // to fixup broken glyph sequences.  Oh well...
    // Test case: U+0643,U+0640,U+0631.

    // planner.ot_map.enable_feature(feature::CONTEXTUAL_SWASH, F_MANUAL_ZWJ, 1);
    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"mset"), F_MANUAL_ZWJ, 1);
}

pub struct arabic_shape_plan_t {
    // The "+ 1" in the next array is to accommodate for the "NONE" command,
    // which is not an OpenType feature, but this simplifies the code by not
    // having to do a "if (... < NONE) ..." and just rely on the fact that
    // mask_array[NONE] == 0.
    mask_array: [hb_mask_t; ARABIC_FEATURES.len() + 1],
    has_stch: bool,
}

pub fn data_create_arabic(plan: &hb_ot_shape_plan_t) -> arabic_shape_plan_t {
    let has_stch = plan.ot_map.get_1_mask(hb_tag_t::new(b"stch")) != 0;

    let mut mask_array = [0; ARABIC_FEATURES.len() + 1];
    for i in 0..ARABIC_FEATURES.len() {
        mask_array[i] = plan.ot_map.get_1_mask(ARABIC_FEATURES[i]);
    }

    arabic_shape_plan_t {
        mask_array,
        has_stch,
    }
}

fn arabic_joining(buffer: &mut hb_buffer_t) {
    let mut prev: Option<usize> = None;
    let mut state = 0;

    // Check pre-context.
    for i in 0..buffer.context_len[0] {
        let c = buffer.context[0][i] as Codepoint;
        let this_type = get_joining_type(c, c.general_category());
        if this_type == hb_arabic_joining_type_t::T {
            continue;
        }

        state = STATE_TABLE[state][this_type as usize].2 as usize;
        break;
    }

    for i in 0..buffer.len {
        let this_type = get_joining_type(
            buffer.info[i].as_codepoint(),
            buffer.info[i].general_category(),
        );
        if this_type == hb_arabic_joining_type_t::T {
            buffer.info[i].set_arabic_shaping_action(arabic_action_t::NONE);
            continue;
        }

        let entry = &STATE_TABLE[state][this_type as usize];
        if entry.0 != arabic_action_t::NONE && prev.is_some() {
            if let Some(prev) = prev {
                buffer.info[prev].set_arabic_shaping_action(entry.0);
                buffer.safe_to_insert_tatweel(Some(prev), Some(i + 1));
            }
        }
        // States that have a possible prev_action.
        else {
            if let Some(prev) = prev {
                if this_type >= hb_arabic_joining_type_t::R || (2 <= state && state <= 5) {
                    buffer.unsafe_to_concat(Some(prev), Some(i + 1));
                }
            } else {
                if this_type >= hb_arabic_joining_type_t::R {
                    buffer.unsafe_to_concat_from_outbuffer(Some(0), Some(i + 1));
                }
            }
        }

        buffer.info[i].set_arabic_shaping_action(entry.1);

        prev = Some(i);
        state = entry.2 as usize;
    }

    for i in 0..buffer.context_len[1] {
        let c = buffer.context[1][i] as Codepoint;
        let this_type = get_joining_type(c, c.general_category());
        if this_type == hb_arabic_joining_type_t::T {
            continue;
        }

        let entry = &STATE_TABLE[state][this_type as usize];
        if entry.0 != arabic_action_t::NONE && prev.is_some() {
            if let Some(prev) = prev {
                buffer.info[prev].set_arabic_shaping_action(entry.0);
                buffer.safe_to_insert_tatweel(Some(prev), Some(buffer.len));
            }
        }
        // States that have a possible prev_action.
        else if 2 <= state && state <= 5 {
            if let Some(prev) = prev {
                buffer.unsafe_to_concat(Some(prev), Some(buffer.len));
            }
        }

        break;
    }
}

fn mongolian_variation_selectors(buffer: &mut hb_buffer_t) {
    // Copy arabic_shaping_action() from base to Mongolian variation selectors.
    let len = buffer.len;
    let info = &mut buffer.info;
    for i in 1..len {
        if (0x180B..=0x180D).contains(&info[i].glyph_id) || info[i].glyph_id == 0x180F {
            let a = info[i - 1].arabic_shaping_action();
            info[i].set_arabic_shaping_action(a);
        }
    }
}

fn setup_masks_arabic_plan(plan: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) {
    buffer.allocate_var(GlyphInfo::ARABIC_SHAPING_ACTION_VAR);

    let arabic_plan = plan.data::<arabic_shape_plan_t>();
    setup_masks_inner(arabic_plan, plan.script, buffer);
}

pub fn setup_masks_inner(
    arabic_plan: &arabic_shape_plan_t,
    script: Option<Script>,
    buffer: &mut hb_buffer_t,
) {
    arabic_joining(buffer);
    if script == Some(script::MONGOLIAN) {
        mongolian_variation_selectors(buffer);
    }

    for info in buffer.info_slice_mut() {
        info.mask |= arabic_plan.mask_array[info.arabic_shaping_action() as usize];
    }
}

fn arabic_fallback_shape(_: &hb_ot_shape_plan_t, _: &hb_font_t, _: &mut hb_buffer_t) -> bool {
    false
}

// Stretch feature: "stch".
// See example here:
// https://docs.microsoft.com/en-us/typography/script-development/syriac
// We implement this in a generic way, such that the Arabic subtending
// marks can use it as well.
fn record_stch(plan: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    let arabic_plan = plan.data::<arabic_shape_plan_t>();
    if !arabic_plan.has_stch {
        return false;
    }

    // 'stch' feature was just applied.  Look for anything that multiplied,
    // and record it for stch treatment later.  Note that rtlm, frac, etc
    // are applied before stch, but we assume that they didn't result in
    // anything multiplying into 5 pieces, so it's safe-ish...

    let len = buffer.len;
    let info = &mut buffer.info;
    let mut has_stch = false;
    for glyph_info in &mut info[..len] {
        if glyph_info.multiplied() {
            let comp = if glyph_info.lig_comp() % 2 != 0 {
                arabic_action_t::STRETCHING_REPEATING
            } else {
                arabic_action_t::STRETCHING_FIXED
            };

            glyph_info.set_arabic_shaping_action(comp);
            has_stch = true;
        }
    }

    if has_stch {
        buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_ARABIC_HAS_STCH;
    }

    false
}

fn apply_stch(face: &hb_font_t, buffer: &mut hb_buffer_t) {
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_ARABIC_HAS_STCH == 0 {
        return;
    }

    let rtl = buffer.direction == Direction::RightToLeft;

    if !rtl {
        buffer.reverse();
    }

    // We do a two pass implementation:
    // First pass calculates the exact number of extra glyphs we need,
    // We then enlarge buffer to have that much room,
    // Second pass applies the stretch, copying things to the end of buffer.

    let mut extra_glyphs_needed: usize = 0; // Set during MEASURE, used during CUT
    const MEASURE: usize = 0;
    const CUT: usize = 1;

    for step in 0..2 {
        let new_len = buffer.len + extra_glyphs_needed; // write head during CUT
        let mut i = buffer.len;
        let mut j = new_len;
        while i != 0 {
            if !arabic_action_t::is_stch(buffer.info[i - 1].arabic_shaping_action()) {
                if step == CUT {
                    j -= 1;
                    buffer.info[j] = buffer.info[i - 1];
                    buffer.pos[j] = buffer.pos[i - 1];
                }

                i -= 1;
                continue;
            }

            // Yay, justification!

            let mut w_total = 0; // Total to be filled
            let mut w_fixed = 0; // Sum of fixed tiles
            let mut w_repeating = 0; // Sum of repeating tiles
            let mut n_repeating: i32 = 0;

            let end = i;
            while i != 0 && arabic_action_t::is_stch(buffer.info[i - 1].arabic_shaping_action()) {
                i -= 1;
                let width = face.glyph_h_advance(buffer.info[i].as_glyph());

                if buffer.info[i].arabic_shaping_action() == arabic_action_t::STRETCHING_FIXED {
                    w_fixed += width;
                } else {
                    w_repeating += width;
                    n_repeating += 1;
                }
            }

            let start = i;
            let mut context = i;
            while context != 0
                && !arabic_action_t::is_stch(buffer.info[context - 1].arabic_shaping_action())
                && (buffer.info[context - 1].is_default_ignorable()
                    || is_word_category(buffer.info[context - 1].general_category()))
            {
                context -= 1;
                w_total += buffer.pos[context].x_advance;
            }

            i += 1; // Don't touch i again.

            // Number of additional times to repeat each repeating tile.
            let mut n_copies: i32 = 0;

            let mut w_remaining = w_total - w_fixed;
            if w_remaining > w_repeating && w_repeating > 0 {
                n_copies = w_remaining / (w_repeating) - 1;
            }

            // See if we can improve the fit by adding an extra repeat and squeezing them together a bit.
            let mut extra_repeat_overlap = 0;
            let shortfall = w_remaining - w_repeating * (n_copies + 1);
            if shortfall > 0 && n_repeating > 0 {
                n_copies += 1;
                let excess = (n_copies + 1) * w_repeating - w_remaining;
                if excess > 0 {
                    extra_repeat_overlap = excess / (n_copies * n_repeating);
                    w_remaining = 0;
                }
            }

            if step == MEASURE {
                extra_glyphs_needed += (n_copies * n_repeating) as usize;
            } else {
                buffer.unsafe_to_break(Some(context), Some(end));
                let mut x_offset = w_remaining / 2;
                for k in (start + 1..=end).rev() {
                    let width = face.glyph_h_advance(buffer.info[k - 1].as_glyph());

                    let mut repeat = 1;
                    if buffer.info[k - 1].arabic_shaping_action()
                        == arabic_action_t::STRETCHING_REPEATING
                    {
                        repeat += n_copies;
                    }

                    buffer.pos[k - 1].x_advance = 0;

                    for n in 0..repeat {
                        if rtl {
                            x_offset -= width;
                            if n > 0 {
                                x_offset += extra_repeat_overlap;
                            }
                        }

                        buffer.pos[k - 1].x_offset = x_offset;

                        // Append copy.
                        j -= 1;
                        buffer.info[j] = buffer.info[k - 1];
                        buffer.pos[j] = buffer.pos[k - 1];

                        if !rtl {
                            x_offset += width;

                            if n > 0 {
                                x_offset -= extra_repeat_overlap;
                            }
                        }
                    }
                }
            }

            i -= 1;
        }

        if step == MEASURE {
            if !buffer.ensure(buffer.len + extra_glyphs_needed) {
                break;
            }
        } else {
            debug_assert_eq!(j, 0);
            buffer.len = new_len;
        }
    }

    if !rtl {
        buffer.reverse();
    }
}

fn postprocess_glyphs_arabic(_: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    apply_stch(face, buffer);
}

// http://www.unicode.org/reports/tr53/
static MODIFIER_COMBINING_MARKS: &[u32] = &[
    0x0654, // ARABIC HAMZA ABOVE
    0x0655, // ARABIC HAMZA BELOW
    0x0658, // ARABIC MARK NOON GHUNNA
    0x06DC, // ARABIC SMALL HIGH SEEN
    0x06E3, // ARABIC SMALL LOW SEEN
    0x06E7, // ARABIC SMALL HIGH YEH
    0x06E8, // ARABIC SMALL HIGH NOON
    0x08CA, // ARABIC SMALL HIGH FARSI YEH
    0x08CB, // ARABIC SMALL HIGH YEH BARREE WITH TWO DOTS BELOW
    0x08CD, // ARABIC SMALL HIGH ZAH
    0x08CE, // ARABIC LARGE ROUND DOT ABOVE
    0x08CF, // ARABIC LARGE ROUND DOT BELOW
    0x08D3, // ARABIC SMALL LOW WAW
    0x08F3, // ARABIC SMALL HIGH WAW
];

fn reorder_marks_arabic(
    _: &hb_ot_shape_plan_t,
    buffer: &mut hb_buffer_t,
    mut start: usize,
    end: usize,
) {
    let mut i = start;
    for cc in [220u8, 230] {
        while i < end && buffer.info[i].modified_combining_class() < cc {
            i += 1;
        }

        if i == end {
            break;
        }

        if buffer.info[i].modified_combining_class() > cc {
            continue;
        }

        let mut j = i;
        while j < end
            && buffer.info[j].modified_combining_class() == cc
            && MODIFIER_COMBINING_MARKS.contains(&buffer.info[j].glyph_id)
        {
            j += 1;
        }

        if i == j {
            continue;
        }

        // Shift it!
        let mut temp = [GlyphInfo::default(); MAX_COMBINING_MARKS];
        debug_assert!(j - i <= MAX_COMBINING_MARKS);
        buffer.merge_clusters(start, j);

        temp[..j - i].copy_from_slice(&buffer.info[i..j]);

        for k in (0..i - start).rev() {
            buffer.info[k + start + j - i] = buffer.info[k + start];
        }

        buffer.info[start..][..j - i].copy_from_slice(&temp[..j - i]);

        // Renumber CC such that the reordered sequence is still sorted.
        // 22 and 26 are chosen because they are smaller than all Arabic categories,
        // and are folded back to 220/230 respectively during fallback mark positioning.
        //
        // We do this because the CGJ-handling logic in the normalizer relies on
        // mark sequences having an increasing order even after this reordering.
        // https://github.com/harfbuzz/harfbuzz/issues/554
        // This, however, does break some obscure sequences, where the normalizer
        // might compose a sequence that it should not.  For example, in the seequence
        // ALEF, HAMZAH, MADDAH, we should NOT try to compose ALEF+MADDAH, but with this
        // renumbering, we will.
        let new_start = start + j - i;
        let new_cc = if cc == 220 {
            modified_combining_class::CCC22
        } else {
            modified_combining_class::CCC26
        };

        while start < new_start {
            buffer.info[start].set_modified_combining_class(new_cc);
            start += 1;
        }

        i = j;
    }
}

pub const ARABIC_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: Some(collect_features),
    override_features: None,
    create_data: Some(|plan| Box::new(data_create_arabic(plan))),
    preprocess_text: None,
    postprocess_glyphs: Some(postprocess_glyphs_arabic),
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_AUTO,
    decompose: None,
    compose: None,
    setup_masks: Some(setup_masks_arabic_plan),
    gpos_tag: None,
    reorder_marks: Some(reorder_marks_arabic),
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_LATE,
    fallback_position: true,
};
