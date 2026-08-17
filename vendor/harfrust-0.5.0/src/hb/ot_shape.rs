use super::aat::map::*;
use super::buffer::*;
use super::ot_layout::*;
use super::ot_layout_gpos_table::GPOS;
use super::ot_map::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::*;
use super::unicode::CharExt;
use super::*;
use super::{hb_font_t, hb_tag_t};
use crate::hb::aat;
use crate::hb::algs::{rb_flag, rb_flag_unsafe};
use crate::hb::buffer::glyph_flag::{SAFE_TO_INSERT_TATWEEL, UNSAFE_TO_BREAK, UNSAFE_TO_CONCAT};
use crate::hb::unicode::hb_gc::{
    HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER, HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER,
    HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR, HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER,
    HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER,
};
use crate::hb::unicode::GeneralCategory;
use crate::BufferClusterLevel;
use crate::BufferFlags;
use crate::{Direction, Feature, Language, Script};
use core::ptr;
use read_fonts::TableProvider;

pub struct hb_ot_shape_planner_t<'a> {
    pub face: &'a hb_font_t<'a>,
    pub direction: Direction,
    pub script: Option<Script>,
    pub language: Option<Language>,
    pub ot_map: hb_ot_map_builder_t<'a>,
    pub aat_map: AatMapBuilder,
    pub apply_morx: bool,
    pub script_zero_marks: bool,
    pub script_fallback_position: bool,
    pub shaper: &'static hb_ot_shaper_t,
}

impl<'a> hb_ot_shape_planner_t<'a> {
    pub fn new(
        face: &'a hb_font_t<'a>,
        direction: Direction,
        script: Option<Script>,
        language: Option<&Language>,
    ) -> Self {
        let ot_map = hb_ot_map_builder_t::new(face, script, language);
        let aat_map = AatMapBuilder::default();

        let mut shaper = match script {
            Some(script) => hb_ot_shape_complex_categorize(
                script,
                direction,
                ot_map.chosen_script(TableIndex::GSUB),
            ),
            None => &DEFAULT_SHAPER,
        };

        let script_zero_marks = shaper.zero_width_marks != HB_OT_SHAPE_ZERO_WIDTH_MARKS_NONE;
        let script_fallback_position = shaper.fallback_position;

        // https://github.com/harfbuzz/harfbuzz/issues/2124
        let apply_morx = face.aat_tables.morx.is_some()
            && (direction.is_horizontal() || face.ot_tables.gsub.is_none());

        // https://github.com/harfbuzz/harfbuzz/issues/1528
        if apply_morx && !ptr::eq(ptr::from_ref(shaper), ptr::from_ref(&DEFAULT_SHAPER)) {
            shaper = &DUMBER_SHAPER;
        }

        hb_ot_shape_planner_t {
            face,
            direction,
            script,
            language: language.cloned(),
            ot_map,
            aat_map,
            apply_morx,
            script_zero_marks,
            script_fallback_position,
            shaper,
        }
    }

    pub fn collect_features(&mut self, user_features: &[Feature]) {
        static COMMON_FEATURES: &[(hb_tag_t, hb_ot_map_feature_flags_t)] = &[
            (hb_tag_t::new(b"abvm"), F_GLOBAL),
            (hb_tag_t::new(b"blwm"), F_GLOBAL),
            (hb_tag_t::new(b"ccmp"), F_GLOBAL),
            (hb_tag_t::new(b"locl"), F_GLOBAL),
            (hb_tag_t::new(b"mark"), F_GLOBAL_MANUAL_JOINERS),
            (hb_tag_t::new(b"mkmk"), F_GLOBAL_MANUAL_JOINERS),
            (hb_tag_t::new(b"rlig"), F_GLOBAL),
        ];

        static HORIZONTAL_FEATURES: &[(hb_tag_t, hb_ot_map_feature_flags_t)] = &[
            (hb_tag_t::new(b"calt"), F_GLOBAL),
            (hb_tag_t::new(b"clig"), F_GLOBAL),
            (hb_tag_t::new(b"curs"), F_GLOBAL),
            (hb_tag_t::new(b"dist"), F_GLOBAL),
            (hb_tag_t::new(b"kern"), F_GLOBAL_HAS_FALLBACK),
            (hb_tag_t::new(b"liga"), F_GLOBAL),
            (hb_tag_t::new(b"rclt"), F_GLOBAL),
        ];

        let empty = F_NONE;

        self.ot_map.is_simple = true;

        self.ot_map.enable_feature(hb_tag_t::new(b"rvrn"), empty, 1);
        self.ot_map.add_gsub_pause(None);

        match self.direction {
            Direction::LeftToRight => {
                self.ot_map.enable_feature(hb_tag_t::new(b"ltra"), empty, 1);
                self.ot_map.enable_feature(hb_tag_t::new(b"ltrm"), empty, 1);
            }
            Direction::RightToLeft => {
                self.ot_map.enable_feature(hb_tag_t::new(b"rtla"), empty, 1);
                self.ot_map.add_feature(hb_tag_t::new(b"rtlm"), empty, 1);
            }
            _ => {}
        }

        // Automatic fractions.
        self.ot_map.add_feature(hb_tag_t::new(b"frac"), empty, 1);
        self.ot_map.add_feature(hb_tag_t::new(b"numr"), empty, 1);
        self.ot_map.add_feature(hb_tag_t::new(b"dnom"), empty, 1);

        // Random!
        self.ot_map
            .enable_feature(hb_tag_t::new(b"rand"), F_RANDOM, hb_ot_map_t::MAX_VALUE);

        // Tracking.  We enable dummy feature here just to allow disabling
        // AAT 'trak' table using features.
        // https://github.com/harfbuzz/harfbuzz/issues/1303
        self.ot_map
            .enable_feature(hb_tag_t::new(b"trak"), F_HAS_FALLBACK, 1);

        self.ot_map.enable_feature(hb_tag_t::new(b"Harf"), empty, 1); // Considered required.
        self.ot_map.enable_feature(hb_tag_t::new(b"HARF"), empty, 1); // Considered discretionary.

        if let Some(func) = self.shaper.collect_features {
            self.ot_map.is_simple = false;
            func(self);
        }

        self.ot_map.enable_feature(hb_tag_t::new(b"Buzz"), empty, 1); // Considered required.
        self.ot_map.enable_feature(hb_tag_t::new(b"BUZZ"), empty, 1); // Considered discretionary.

        for &(tag, flags) in COMMON_FEATURES {
            self.ot_map.add_feature(tag, flags, 1);
        }

        if self.direction.is_horizontal() {
            for &(tag, flags) in HORIZONTAL_FEATURES {
                self.ot_map.add_feature(tag, flags, 1);
            }
        } else {
            // We only apply `vert` feature. See:
            // https://github.com/harfbuzz/harfbuzz/commit/d71c0df2d17f4590d5611239577a6cb532c26528
            // https://lists.freedesktop.org/archives/harfbuzz/2013-August/003490.html

            // We really want to find a 'vert' feature if there's any in the font, no
            // matter which script/langsys it is listed (or not) under.
            // See various bugs referenced from:
            // https://github.com/harfbuzz/harfbuzz/issues/63
            self.ot_map
                .enable_feature(hb_tag_t::new(b"vert"), F_GLOBAL_SEARCH, 1);
        }

        if !user_features.is_empty() {
            self.ot_map.is_simple = false;
        }

        for feature in user_features {
            let flags = if feature.is_global() { F_GLOBAL } else { empty };
            self.ot_map.add_feature(feature.tag, flags, feature.value);
        }

        if let Some(func) = self.shaper.override_features {
            func(self);
        }
    }

    pub fn compile(mut self, features: &[Feature]) -> hb_ot_shape_plan_t {
        let ot_map = self.ot_map.compile();
        let mut aat_map = AatMap::default();
        if self.apply_morx {
            self.aat_map.compile(self.face, &mut aat_map);
        }

        let frac_mask = ot_map.get_1_mask(hb_tag_t::new(b"frac"));
        let numr_mask = ot_map.get_1_mask(hb_tag_t::new(b"numr"));
        let dnom_mask = ot_map.get_1_mask(hb_tag_t::new(b"dnom"));
        let has_frac = frac_mask != 0 || (numr_mask != 0 && dnom_mask != 0);

        let rtlm_mask = ot_map.get_1_mask(hb_tag_t::new(b"rtlm"));
        let has_vert = ot_map.get_1_mask(hb_tag_t::new(b"vert")) != 0;

        let horizontal = self.direction.is_horizontal();
        let kern_tag = if horizontal {
            hb_tag_t::new(b"kern")
        } else {
            hb_tag_t::new(b"vkrn")
        };
        let kern_mask = ot_map.get_mask(kern_tag).0;
        let requested_kerning = kern_mask != 0;

        let has_gpos_kern = ot_map
            .get_feature_index(TableIndex::GPOS, kern_tag)
            .is_some();
        let disable_gpos = self.shaper.gpos_tag.is_some()
            && self.shaper.gpos_tag != ot_map.chosen_script(TableIndex::GPOS);

        // Decide who provides glyph classes. GDEF or Unicode.
        let fallback_glyph_classes = !hb_ot_layout_has_glyph_classes(self.face);

        // Decide who does substitutions. GSUB, morx, or fallback.
        let apply_morx = self.apply_morx;

        let mut apply_gpos = false;
        let mut apply_kerx = false;
        let mut apply_kern = false;

        // Decide who does positioning. GPOS, kerx, kern, or fallback.
        let has_kerx = self.face.aat_tables.kerx.is_some();
        let has_gsub = !apply_morx && self.face.ot_tables.gsub.is_some();
        let has_gpos = !disable_gpos && self.face.ot_tables.gpos.is_some();

        // Prefer GPOS over kerx if GSUB is present;
        // https://github.com/harfbuzz/harfbuzz/issues/3008
        if has_kerx && !(has_gsub && has_gpos) {
            apply_kerx = true;
        } else if has_gpos {
            apply_gpos = true;
        }

        if !apply_kerx && (!has_gpos_kern || !apply_gpos) {
            if has_kerx {
                apply_kerx = true;
            } else if hb_ot_layout_has_kerning(self.face) {
                apply_kern = self.script_fallback_position;
            }
        }

        let apply_fallback_kern = !(apply_gpos || apply_kerx || apply_kern);
        let zero_marks = self.script_zero_marks
            && !apply_kerx
            && (!apply_kern || !hb_ot_layout_has_machine_kerning(self.face));

        let has_gpos_mark = ot_map.get_1_mask(hb_tag_t::new(b"mark")) != 0;

        let mut adjust_mark_positioning_when_zeroing = !apply_gpos
            && !apply_kerx
            && (!apply_kern || !hb_ot_layout_has_cross_kerning(self.face));

        let fallback_mark_positioning =
            adjust_mark_positioning_when_zeroing && self.script_fallback_position;

        // If we're using morx shaping, we cancel mark position adjustment because
        // Apple Color Emoji assumes this will NOT be done when forming emoji sequences;
        // https://github.com/harfbuzz/harfbuzz/issues/2967.
        if apply_morx {
            adjust_mark_positioning_when_zeroing = false;
        }

        // According to Ned, trak is applied by default for "modern fonts", as detected by presence of STAT table.
        // https://github.com/googlefonts/fontations/issues/1492
        let apply_trak = self.face.font.trak().is_ok()
            && self
                .face
                .font
                .table_directory
                .table_records()
                .iter()
                .any(|table| table.tag() == "STAT");

        let mut plan = hb_ot_shape_plan_t {
            direction: self.direction,
            script: self.script,
            language: self.language,
            shaper: self.shaper,
            ot_map,
            aat_map,
            data: None,
            frac_mask,
            numr_mask,
            dnom_mask,
            rtlm_mask,
            kern_mask,
            requested_kerning,
            has_frac,
            has_vert,
            has_gpos_mark,
            zero_marks,
            fallback_glyph_classes,
            fallback_mark_positioning,
            adjust_mark_positioning_when_zeroing,
            apply_gpos,
            apply_kern,
            apply_fallback_kern,
            apply_kerx,
            apply_morx,
            apply_trak,
            user_features: features.into(),
        };

        if let Some(func) = self.shaper.create_data {
            plan.data = Some(func(&plan));
        }

        plan
    }
}

pub struct hb_ot_shape_context_t<'a> {
    pub plan: &'a hb_ot_shape_plan_t,
    pub face: &'a hb_font_t<'a>,
    pub buffer: &'a mut hb_buffer_t,
    // Transient stuff
    pub target_direction: Direction,
    pub features: &'a [Feature],
}

// Pull it all together!
pub fn shape_internal(ctx: &mut hb_ot_shape_context_t) {
    ctx.buffer.allocate_unicode_vars();

    initialize_masks(ctx);
    set_unicode_props(ctx.buffer);
    insert_dotted_circle(ctx.buffer, ctx.face);

    form_clusters(ctx.buffer);

    ensure_native_direction(ctx.buffer);

    if let Some(func) = ctx.plan.shaper.preprocess_text {
        func(ctx.plan, ctx.face, ctx.buffer);
    }

    substitute_pre(ctx);
    position(ctx);
    substitute_post(ctx);

    propagate_flags(ctx.buffer);

    ctx.buffer.deallocate_unicode_vars();

    ctx.buffer.direction = ctx.target_direction;
}

fn substitute_pre(ctx: &mut hb_ot_shape_context_t) {
    hb_ot_substitute_default(ctx);

    ctx.buffer.allocate_gsubgpos_vars();

    hb_ot_substitute_plan(ctx);

    if ctx.plan.apply_morx && ctx.plan.apply_gpos {
        aat::layout::remove_deleted_glyphs(ctx.buffer);
    }
}

fn substitute_post(ctx: &mut hb_ot_shape_context_t) {
    if ctx.plan.apply_morx && !ctx.plan.apply_gpos {
        aat::layout::remove_deleted_glyphs(ctx.buffer);
    }

    deal_with_variation_selectors(ctx.buffer);
    hide_default_ignorables(ctx.buffer, ctx.face);

    if let Some(func) = ctx.plan.shaper.postprocess_glyphs {
        func(ctx.plan, ctx.face, ctx.buffer);
    }
}

fn hb_ot_substitute_default(ctx: &mut hb_ot_shape_context_t) {
    rotate_chars(ctx);

    ctx.buffer
        .allocate_var(GlyphInfo::NORMALIZER_GLYPH_INDEX_VAR);

    ot_shape_normalize::_hb_ot_shape_normalize(ctx.plan, ctx.buffer, ctx.face);

    setup_masks(ctx);

    // This is unfortunate to go here, but necessary...
    if ctx.plan.fallback_mark_positioning {
        ot_shape_fallback::_hb_ot_shape_fallback_mark_position_recategorize_marks(
            ctx.plan, ctx.face, ctx.buffer,
        );
    }

    map_glyphs_fast(ctx.buffer);

    ctx.buffer
        .deallocate_var(GlyphInfo::NORMALIZER_GLYPH_INDEX_VAR);
}

fn hb_ot_substitute_plan(ctx: &mut hb_ot_shape_context_t) {
    hb_ot_layout_substitute_start(ctx.face, ctx.buffer);

    if ctx.plan.fallback_glyph_classes {
        hb_synthesize_glyph_classes(ctx.buffer);
    }

    if ctx.plan.apply_morx {
        aat::layout::substitute(ctx.plan, ctx.face, ctx.buffer, ctx.features);
        ctx.buffer.update_digest();
    } else {
        ctx.buffer.update_digest();
        ot_layout_gsub_table::substitute(ctx.plan, ctx.face, ctx.buffer);
    }
}

fn position(ctx: &mut hb_ot_shape_context_t) {
    ctx.buffer.clear_positions();

    position_default(ctx);

    position_complex(ctx);

    if ctx.buffer.direction.is_backward() {
        ctx.buffer.reverse();
    }

    ctx.buffer.deallocate_gsubgpos_vars();
}

fn position_default(ctx: &mut hb_ot_shape_context_t) {
    let len = ctx.buffer.len;

    if ctx.buffer.direction.is_horizontal() {
        ctx.face.glyph_h_advances(ctx.buffer);
    } else {
        for (info, pos) in ctx.buffer.info[..len]
            .iter()
            .zip(&mut ctx.buffer.pos[..len])
        {
            let glyph = info.as_glyph();
            pos.y_advance = ctx.face.glyph_v_advance(glyph);
            pos.x_offset -= ctx.face.glyph_h_origin(glyph);
            pos.y_offset -= ctx.face.glyph_v_origin(glyph);
        }
    }

    if ctx.buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_SPACE_FALLBACK != 0 {
        ot_shape_fallback::_hb_ot_shape_fallback_spaces(ctx.plan, ctx.face, ctx.buffer);
    }
}

fn position_complex(ctx: &mut hb_ot_shape_context_t) {
    // If the font has no GPOS and direction is forward, then when
    // zeroing mark widths, we shift the mark with it, such that the
    // mark is positioned hanging over the previous glyph.  When
    // direction is backward we don't shift and it will end up
    // hanging over the next glyph after the final reordering.
    //
    // Note: If fallback positioning happens, we don't care about
    // this as it will be overridden.
    let adjust_offsets_when_zeroing =
        ctx.plan.adjust_mark_positioning_when_zeroing && ctx.buffer.direction.is_forward();

    // We change glyph origin to what GPOS expects (horizontal), apply GPOS, change it back.

    GPOS::position_start(ctx.face, ctx.buffer);

    if ctx.plan.zero_marks
        && ctx.plan.shaper.zero_width_marks == HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_EARLY
    {
        zero_mark_widths_by_gdef(ctx.buffer, adjust_offsets_when_zeroing);
    }

    position_by_plan(ctx.plan, ctx.face, ctx.buffer);

    if ctx.plan.zero_marks
        && ctx.plan.shaper.zero_width_marks == HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_LATE
    {
        zero_mark_widths_by_gdef(ctx.buffer, adjust_offsets_when_zeroing);
    }

    // Finish off.  Has to follow a certain order.
    GPOS::position_finish_advances(ctx.face, ctx.buffer);
    zero_width_default_ignorables(ctx.buffer);
    GPOS::position_finish_offsets(ctx.face, ctx.buffer);

    if ctx.plan.fallback_mark_positioning {
        ot_shape_fallback::position_marks(
            ctx.plan,
            ctx.face,
            ctx.buffer,
            adjust_offsets_when_zeroing,
        );
    }
}

fn position_by_plan(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    if plan.apply_gpos {
        ot_layout_gpos_table::position(plan, face, buffer);
    } else if plan.apply_kerx {
        aat::layout::position(plan, face, buffer);
    }
    if plan.apply_kern {
        kerning::hb_ot_layout_kern(plan, face, buffer);
    } else if plan.apply_fallback_kern {
        ot_shape_fallback::_hb_ot_shape_fallback_kern(plan, face, buffer);
    }

    if plan.apply_trak {
        aat::layout::track(plan, face, buffer);
    }
}

fn initialize_masks(ctx: &mut hb_ot_shape_context_t) {
    let global_mask = ctx.plan.ot_map.get_global_mask();
    ctx.buffer.reset_masks(global_mask);
}

fn setup_masks(ctx: &mut hb_ot_shape_context_t) {
    setup_masks_fraction(ctx);

    if let Some(func) = ctx.plan.shaper.setup_masks {
        func(ctx.plan, ctx.face, ctx.buffer);
    }

    for feature in ctx.features {
        if !feature.is_global() {
            let (mask, shift) = ctx.plan.ot_map.get_mask(feature.tag);
            ctx.buffer
                .set_masks(feature.value << shift, mask, feature.start, feature.end);
        }
    }
}

fn setup_masks_fraction(ctx: &mut hb_ot_shape_context_t) {
    let buffer = &mut ctx.buffer;
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_FRACTION_SLASH == 0 || !ctx.plan.has_frac {
        return;
    }

    let (pre_mask, post_mask) = if buffer.direction.is_forward() {
        (
            ctx.plan.numr_mask | ctx.plan.frac_mask,
            ctx.plan.frac_mask | ctx.plan.dnom_mask,
        )
    } else {
        (
            ctx.plan.frac_mask | ctx.plan.dnom_mask,
            ctx.plan.numr_mask | ctx.plan.frac_mask,
        )
    };

    let len = buffer.len;
    let mut i = 0;
    while i < len {
        // FRACTION SLASH
        if buffer.info[i].glyph_id == 0x2044 {
            let mut start = i;
            while start > 0
                && buffer.info[start - 1].general_category() == GeneralCategory::DECIMAL_NUMBER
            {
                start -= 1;
            }

            let mut end = i + 1;
            while end < len
                && buffer.info[end].general_category() == GeneralCategory::DECIMAL_NUMBER
            {
                end += 1;
            }

            if start == i || end == i + 1 {
                if start == i {
                    buffer.unsafe_to_concat(Some(start), Some(start + 1));
                }

                if end == i + 1 {
                    buffer.unsafe_to_concat(Some(end - 1), Some(end));
                }

                i += 1;
                continue;
            }

            buffer.unsafe_to_break(Some(start), Some(end));

            for info in &mut buffer.info[start..i] {
                info.mask |= pre_mask;
            }

            buffer.info[i].mask |= ctx.plan.frac_mask;

            for info in &mut buffer.info[i + 1..end] {
                info.mask |= post_mask;
            }

            i = end;
        } else {
            i += 1;
        }
    }
}

fn set_unicode_props(buffer: &mut hb_buffer_t) {
    // Implement enough of Unicode Graphemes here that shaping
    // in reverse-direction wouldn't break graphemes.  Namely,
    // we mark all marks and ZWJ and ZWJ,Extended_Pictographic
    // sequences as continuations.  The foreach_grapheme()
    // macro uses this bit.
    //
    // https://www.unicode.org/reports/tr29/#Regex_Definitions

    let len = buffer.len;

    let mut i = 0;
    while i < len {
        let info = &mut buffer.info[i];
        info.init_unicode_props(&mut buffer.scratch_flags);

        if info.glyph_id < 0x80 {
            i += 1;
            continue;
        }

        let gen_cat = info.general_category();

        if (rb_flag_unsafe(gen_cat.to_u8() as u32)
            & (rb_flag(HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER)
                | rb_flag(HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER)
                | rb_flag(HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER)
                | rb_flag(HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER)
                | rb_flag(HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR)))
            != 0
        {
            i += 1;
            continue;
        }

        // Mutably borrow buffer.info[i] and immutably borrow
        // buffer.info[i - 1] (if present) in a way that the borrow
        // checker can understand.
        let (prior, later) = buffer.info.split_at_mut(i);
        let info = &mut later[0];

        // Marks are already set as continuation by the above line.
        // Handle Emoji_Modifier and ZWJ-continuation.
        if gen_cat == GeneralCategory::MODIFIER_SYMBOL && matches!(info.glyph_id, 0x1F3FB..=0x1F3FF)
        {
            info.set_continuation(&mut buffer.scratch_flags);
        } else if i != 0 && matches!(info.glyph_id, 0x1F1E6..=0x1F1FF) {
            // Should never fail because we checked for i > 0.
            // TODO: use let chains when they become stable
            let prev = prior.last().unwrap();
            if matches!(prev.glyph_id, 0x1F1E6..=0x1F1FF) && !prev.is_continuation() {
                info.set_continuation(&mut buffer.scratch_flags);
            }
        } else if info.is_zwj() {
            info.set_continuation(&mut buffer.scratch_flags);
            if let Some(next) = buffer.info[..len].get_mut(i + 1) {
                if next.as_codepoint().is_emoji_extended_pictographic() {
                    next.init_unicode_props(&mut buffer.scratch_flags);
                    next.set_continuation(&mut buffer.scratch_flags);
                    i += 1;
                }
            }
        } else if matches!(info.glyph_id, 0xFF9E..=0xFF9F | 0xE0020..=0xE007F) {
            // Or part of the Other_Grapheme_Extend that is not marks.
            // As of Unicode 15 that is just:
            //
            // 200C          ; Other_Grapheme_Extend # Cf       ZERO WIDTH NON-JOINER
            // FF9E..FF9F    ; Other_Grapheme_Extend # Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA
            // SEMI-VOICED SOUND MARK E0020..E007F  ; Other_Grapheme_Extend # Cf  [96] TAG SPACE..CANCEL TAG
            //
            // ZWNJ is special, we don't want to merge it as there's no need, and keeping
            // it separate results in more granular clusters.
            // Tags are used for Emoji sub-region flag sequences:
            // https://github.com/harfbuzz/harfbuzz/issues/1556
            // Katakana ones were requested:
            // https://github.com/harfbuzz/harfbuzz/issues/3844
            info.set_continuation(&mut buffer.scratch_flags);
        } else if info.glyph_id == 0x2044
        /* FRACTION SLASH */
        {
            buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_FRACTION_SLASH;
        }

        i += 1;
    }
}

fn insert_dotted_circle(buffer: &mut hb_buffer_t, face: &hb_font_t) {
    if !buffer
        .flags
        .contains(BufferFlags::DO_NOT_INSERT_DOTTED_CIRCLE)
        && buffer.flags.contains(BufferFlags::BEGINNING_OF_TEXT)
        && buffer.context_len[0] == 0
        && buffer.info[0].is_unicode_mark()
        && face.has_glyph(0x25CC)
    {
        let mut info = GlyphInfo {
            glyph_id: 0x25CC,
            mask: buffer.cur(0).mask,
            cluster: buffer.cur(0).cluster,
            ..GlyphInfo::default()
        };

        info.init_unicode_props(&mut buffer.scratch_flags);
        buffer.clear_output();
        buffer.output_info(info);
        buffer.sync();
    }
}

fn form_clusters(buffer: &mut hb_buffer_t) {
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_CONTINUATIONS != 0 {
        if BufferClusterLevel::new(buffer.cluster_level).is_graphemes() {
            foreach_grapheme!(buffer, start, end, { buffer.merge_clusters(start, end) });
        } else {
            foreach_grapheme!(buffer, start, end, {
                buffer.unsafe_to_break(Some(start), Some(end));
            });
        }
    }
}

fn ensure_native_direction(buffer: &mut hb_buffer_t) {
    let dir = buffer.direction;
    let mut hor = buffer
        .script
        .and_then(Direction::from_script)
        .unwrap_or_default();

    // Numeric runs in natively-RTL scripts are actually native-LTR, so we reset
    // the horiz_dir if the run contains at least one decimal-number char, and no
    // letter chars (ideally we should be checking for chars with strong
    // directionality but hb-unicode currently lacks bidi categories).
    //
    // This allows digit sequences in Arabic etc to be shaped in "native"
    // direction, so that features like ligatures will work as intended.
    //
    // https://github.com/harfbuzz/harfbuzz/issues/501
    //
    // Similar thing about Regional_Indicators; They are bidi=L, but Script=Common.
    // If they are present in a run of natively-RTL text, they get assigned a script
    // with natively RTL direction, which would result in wrong shaping if we
    // assign such native RTL direction to them then. Detect that as well.
    //
    // https://github.com/harfbuzz/harfbuzz/issues/3314

    if hor == Direction::RightToLeft && dir == Direction::LeftToRight {
        let mut found_number = false;
        let mut found_letter = false;
        let mut found_ri = false;
        for info in &buffer.info {
            let gc = info.general_category();
            if gc == GeneralCategory::DECIMAL_NUMBER {
                found_number = true;
            } else if gc.is_letter() {
                found_letter = true;
                break;
            } else if matches!(info.glyph_id, 0x1F1E6..=0x1F1FF) {
                found_ri = true;
            }
        }
        if (found_number || found_ri) && !found_letter {
            hor = Direction::LeftToRight;
        }
    }

    // TODO vertical:
    // The only BTT vertical script is Ogham, but it's not clear to me whether OpenType
    // Ogham fonts are supposed to be implemented BTT or not.  Need to research that
    // first.
    if (dir.is_horizontal() && dir != hor && hor != Direction::Invalid)
        || (dir.is_vertical() && dir != Direction::TopToBottom)
    {
        _hb_ot_layout_reverse_graphemes(buffer);
        buffer.direction = buffer.direction.reverse();
    }
}

fn rotate_chars(ctx: &mut hb_ot_shape_context_t) {
    let len = ctx.buffer.len;

    if ctx.target_direction.is_backward() {
        let rtlm_mask = ctx.plan.rtlm_mask;

        for info in &mut ctx.buffer.info[..len] {
            if let Some(c) = info.as_codepoint().mirrored() {
                if ctx.face.has_glyph(c) {
                    info.glyph_id = c;
                    continue;
                }
            }
            info.mask |= rtlm_mask;
        }
    }

    if ctx.target_direction.is_vertical() && !ctx.plan.has_vert {
        for info in &mut ctx.buffer.info[..len] {
            if let Some(c) = info.as_codepoint().vertical() {
                if ctx.face.has_glyph(c) {
                    info.glyph_id = c;
                }
            }
        }
    }
}

fn map_glyphs_fast(buffer: &mut hb_buffer_t) {
    // Normalization process sets up normalizer_glyph_index(), we just copy it.
    let len = buffer.len;
    for info in &mut buffer.info[..len] {
        info.glyph_id = info.normalizer_glyph_index();
    }

    for info in &mut buffer.out_info_mut()[..len] {
        info.glyph_id = info.normalizer_glyph_index();
    }
}

fn hb_synthesize_glyph_classes(buffer: &mut hb_buffer_t) {
    let len = buffer.len;
    for info in &mut buffer.info[..len] {
        // Never mark default-ignorables as marks.
        // They won't get in the way of lookups anyway,
        // but having them as mark will cause them to be skipped
        // over if the lookup-flag says so, but at least for the
        // Mongolian variation selectors, looks like Uniscribe
        // marks them as non-mark.  Some Mongolian fonts without
        // GDEF rely on this.  Another notable character that
        // this applies to is COMBINING GRAPHEME JOINER.
        let class = if info.general_category() != GeneralCategory::NON_SPACING_MARK
            || info.is_default_ignorable()
        {
            GlyphPropsFlags::BASE_GLYPH
        } else {
            GlyphPropsFlags::MARK
        };

        info.set_glyph_props(class.bits());
    }
}

fn zero_width_default_ignorables(buffer: &mut hb_buffer_t) {
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_DEFAULT_IGNORABLES != 0
        && !buffer
            .flags
            .contains(BufferFlags::PRESERVE_DEFAULT_IGNORABLES)
        && !buffer
            .flags
            .contains(BufferFlags::REMOVE_DEFAULT_IGNORABLES)
    {
        let len = buffer.len;
        for (info, pos) in buffer.info[..len].iter().zip(&mut buffer.pos[..len]) {
            if info.is_default_ignorable() {
                pos.x_advance = 0;
                pos.y_advance = 0;
                if buffer.direction.is_horizontal() {
                    pos.x_offset = 0;
                } else {
                    pos.y_offset = 0;
                }
            }
        }
    }
}

fn deal_with_variation_selectors(buffer: &mut hb_buffer_t) {
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_VARIATION_SELECTOR_FALLBACK == 0 {
        return;
    }

    // Note: In harfbuzz, this is part of the condition above (with OR), so it needs to stay
    // in sync.
    let Some(nf) = buffer.not_found_variation_selector else {
        return;
    };

    let count = buffer.len;
    let info = &mut buffer.info;
    let pos = &mut buffer.pos;

    for i in 0..count {
        if info[i].is_variation_selector() {
            info[i].glyph_id = nf;
            pos[i].x_advance = 0;
            pos[i].y_advance = 0;
            pos[i].x_offset = 0;
            pos[i].y_offset = 0;
            info[0].set_variation_selector(false);
        }
    }
}

fn zero_mark_widths_by_gdef(buffer: &mut hb_buffer_t, adjust_offsets: bool) {
    let len = buffer.len;
    for (info, pos) in buffer.info[..len].iter().zip(&mut buffer.pos[..len]) {
        if info.is_mark() {
            if adjust_offsets {
                pos.x_offset -= pos.x_advance;
                pos.y_offset -= pos.y_advance;
            }

            pos.x_advance = 0;
            pos.y_advance = 0;
        }
    }
}

fn hide_default_ignorables(buffer: &mut hb_buffer_t, face: &hb_font_t) {
    if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_DEFAULT_IGNORABLES != 0
        && !buffer
            .flags
            .contains(BufferFlags::PRESERVE_DEFAULT_IGNORABLES)
    {
        if !buffer
            .flags
            .contains(BufferFlags::REMOVE_DEFAULT_IGNORABLES)
        {
            if let Some(invisible) = buffer
                .invisible
                .or_else(|| face.get_nominal_glyph(u32::from(' ')))
            {
                let len = buffer.len;
                for info in &mut buffer.info[..len] {
                    if info.is_default_ignorable() {
                        info.glyph_id = invisible.to_u32();
                    }
                }
                return;
            }
        }

        buffer.delete_glyphs_inplace(GlyphInfo::is_default_ignorable);
    }
}

fn propagate_flags(buffer: &mut hb_buffer_t) {
    // Propagate cluster-level glyph flags to be the same on all cluster glyphs.
    // Simplifies using them.

    let mut and_mask = glyph_flag::DEFINED;
    if !buffer.flags.contains(BufferFlags::PRODUCE_UNSAFE_TO_CONCAT) {
        and_mask &= !UNSAFE_TO_CONCAT;
    }

    if !buffer
        .flags
        .contains(BufferFlags::PRODUCE_SAFE_TO_INSERT_TATWEEL)
    {
        foreach_cluster!(buffer, start, end, {
            if end - start == 1 {
                buffer.info[start].mask &= and_mask;
            } else {
                let mut mask = 0;
                for info in &buffer.info[start..end] {
                    mask |= info.mask;
                }

                mask &= and_mask;

                for info in &mut buffer.info[start..end] {
                    info.mask = mask;
                }
            }
        });
        return;
    }

    /* If we are producing SAFE_TO_INSERT_TATWEEL, then do two things:
     *
     * - If the places that the Arabic shaper marked as SAFE_TO_INSERT_TATWEEL,
     *   are UNSAFE_TO_BREAK, then clear the SAFE_TO_INSERT_TATWEEL,
     * - Any place that is SAFE_TO_INSERT_TATWEEL, is also now UNSAFE_TO_BREAK.
     *
     * We couldn't make this interaction earlier. It has to be done here.
     */
    foreach_cluster!(buffer, start, end, {
        // We cannot use `continue` in our `for_each_cluster!` macro.
        if end - start != 1 {
            let mut mask = 0;
            for info in &buffer.info[start..end] {
                mask |= info.mask;
            }

            mask &= glyph_flag::DEFINED;

            if mask & UNSAFE_TO_BREAK != 0 {
                mask &= !SAFE_TO_INSERT_TATWEEL;
            }

            if mask & SAFE_TO_INSERT_TATWEEL != 0 {
                mask |= UNSAFE_TO_BREAK | UNSAFE_TO_CONCAT;
            }

            mask &= and_mask;

            for info in &mut buffer.info[start..end] {
                info.mask = mask;
            }
        }
    });
}
