use alloc::boxed::Box;

use super::algs::*;
use super::buffer::hb_buffer_t;
use super::ot_layout::*;
use super::ot_map::*;
use super::ot_shape::*;
use super::ot_shape_normalize::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::*;
use super::ot_shaper_arabic::arabic_shape_plan_t;
use super::unicode::{CharExt, GeneralCategoryExt};
use super::{hb_font_t, hb_glyph_info_t, hb_mask_t, hb_tag_t, script, Script};

pub const UNIVERSAL_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: Some(collect_features),
    override_features: None,
    create_data: Some(|plan| Box::new(UniversalShapePlan::new(plan))),
    preprocess_text: Some(preprocess_text),
    postprocess_glyphs: None,
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS_NO_SHORT_CIRCUIT,
    decompose: None,
    compose: Some(compose),
    setup_masks: Some(setup_masks),
    gpos_tag: None,
    reorder_marks: None,
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_EARLY,
    fallback_position: false,
};

pub type Category = u8;
#[allow(dead_code)]
pub mod category {
    pub const O: u8 = 0; // OTHER

    pub const B: u8 = 1; // BASE

    // pub const IND: u8     = 3;    // BASE_IND

    pub const N: u8 = 4; // BASE_NUM
    pub const GB: u8 = 5; // BASE_OTHER
    pub const CGJ: u8 = 6;

    // pub const CGJ: u8     = 6;    // CGJ
    // pub const F: u8       = 7;    // CONS_FINAL
    // pub const FM: u8 = 8;         // CONS_FINAL_MOD
    // pub const M: u8       = 9;    // CONS_MED
    // pub const CM: u8      = 10;   // CONS_MOD

    pub const SUB: u8 = 11; // CONS_SUB
    pub const H: u8 = 12; // HALANT

    pub const HN: u8 = 13; // HALANT_NUM
    pub const ZWNJ: u8 = 14; // Zero width non-joiner

    // pub const ZWJ: u8     = 15;   // Zero width joiner
    pub const WJ: u8 = 16; // Word joiner

    pub const RSV: u8 = 17; // Reserved characters
    pub const R: u8 = 18; // REPHA
    pub const S: u8 = 19; // SYM

    // pub const SM: u8      = 20;   // SYM_MOD
    // pub const VS: u8      = 21;   // VARIATION_SELECTOR
    // pub const V: u8       = 36;   // VOWEL
    // pub const VM: u8      = 40;   // VOWEL_MOD

    pub const CS: u8 = 43; // CONS_WITH_STACKER

    // https://github.com/harfbuzz/harfbuzz/issues/1102
    pub const IS: u8 = 44; // HALANT_OR_VOWEL_MODIFIER

    pub const Sk: u8 = 48; // SAKOT

    pub const FAbv: u8 = 24; // CONS_FINAL_ABOVE
    pub const FBlw: u8 = 25; // CONS_FINAL_BELOW
    pub const FPst: u8 = 26; // CONS_FINAL_POST
    pub const MAbv: u8 = 27; // CONS_MED_ABOVE
    pub const MBlw: u8 = 28; // CONS_MED_BELOW
    pub const MPst: u8 = 29; // CONS_MED_POST
    pub const MPre: u8 = 30; // CONS_MED_PRE
    pub const CMAbv: u8 = 31; // CONS_MOD_ABOVE
    pub const CMBlw: u8 = 32; // CONS_MOD_BELOW
    pub const VAbv: u8 = 33; // VOWEL_ABOVE / VOWEL_ABOVE_BELOW / VOWEL_ABOVE_BELOW_POST / VOWEL_ABOVE_POST
    pub const VBlw: u8 = 34; // VOWEL_BELOW / VOWEL_BELOW_POST
    pub const VPst: u8 = 35; // VOWEL_POST UIPC = Right
    pub const VPre: u8 = 22; // VOWEL_PRE / VOWEL_PRE_ABOVE / VOWEL_PRE_ABOVE_POST / VOWEL_PRE_POST
    pub const VMAbv: u8 = 37; // VOWEL_MOD_ABOVE
    pub const VMBlw: u8 = 38; // VOWEL_MOD_BELOW
    pub const VMPst: u8 = 39; // VOWEL_MOD_POST
    pub const VMPre: u8 = 23; // VOWEL_MOD_PRE
    pub const SMAbv: u8 = 41; // SYM_MOD_ABOVE
    pub const SMBlw: u8 = 42; // SYM_MOD_BELOW
    pub const FMAbv: u8 = 45; // CONS_FINAL_MOD UIPC = Top
    pub const FMBlw: u8 = 46; // CONS_FINAL_MOD UIPC = Bottom
    pub const FMPst: u8 = 47; // CONS_FINAL_MOD UIPC = Not_Applicable
    pub const G: u8 = 49; // HIEROGLYPH
    pub const J: u8 = 50; // HIEROGLYPH_JOINER
    pub const SB: u8 = 51; // HIEROGLYPH_SEGMENT_BEGIN
    pub const SE: u8 = 52; // HIEROGLYPH_SEGMENT_END
    pub const HVM: u8 = 53; // HIEROGLYPH_SEGMENT_END
    pub const HM: u8 = 54; // HIEROGLYPH_MOD
    pub const HR: u8 = 55; // HIEROGLYPH_MIRROR
}

// These features are applied all at once, before reordering,
// constrained to the syllable.
const BASIC_FEATURES: &[hb_tag_t] = &[
    hb_tag_t::from_bytes(b"rkrf"),
    hb_tag_t::from_bytes(b"abvf"),
    hb_tag_t::from_bytes(b"blwf"),
    hb_tag_t::from_bytes(b"half"),
    hb_tag_t::from_bytes(b"pstf"),
    hb_tag_t::from_bytes(b"vatu"),
    hb_tag_t::from_bytes(b"cjct"),
];

const TOPOGRAPHICAL_FEATURES: &[hb_tag_t] = &[
    hb_tag_t::from_bytes(b"isol"),
    hb_tag_t::from_bytes(b"init"),
    hb_tag_t::from_bytes(b"medi"),
    hb_tag_t::from_bytes(b"fina"),
];

// Same order as use_topographical_features.
#[derive(Clone, Copy, PartialEq)]
enum JoiningForm {
    Isolated = 0,
    Initial,
    Medial,
    Terminal,
}

// These features are applied all at once, after reordering and clearing syllables.
const OTHER_FEATURES: &[hb_tag_t] = &[
    hb_tag_t::from_bytes(b"abvs"),
    hb_tag_t::from_bytes(b"blws"),
    hb_tag_t::from_bytes(b"haln"),
    hb_tag_t::from_bytes(b"pres"),
    hb_tag_t::from_bytes(b"psts"),
];

impl hb_glyph_info_t {
    pub(crate) fn use_category(&self) -> Category {
        self.ot_shaper_var_u8_category()
    }

    fn set_use_category(&mut self, c: Category) {
        self.set_ot_shaper_var_u8_category(c)
    }

    fn is_halant_use(&self) -> bool {
        matches!(
            self.use_category(),
            category::H | category::HVM | category::IS
        ) && !_hb_glyph_info_ligated(self)
    }
}

struct UniversalShapePlan {
    rphf_mask: hb_mask_t,
    arabic_plan: Option<arabic_shape_plan_t>,
}

impl UniversalShapePlan {
    fn new(plan: &hb_ot_shape_plan_t) -> UniversalShapePlan {
        let mut arabic_plan = None;

        if plan.script.map_or(false, has_arabic_joining) {
            arabic_plan = Some(crate::hb::ot_shaper_arabic::data_create_arabic(plan));
        }

        UniversalShapePlan {
            rphf_mask: plan.ot_map.get_1_mask(hb_tag_t::from_bytes(b"rphf")),
            arabic_plan,
        }
    }
}

fn collect_features(planner: &mut hb_ot_shape_planner_t) {
    // Do this before any lookups have been applied.
    planner.ot_map.add_gsub_pause(Some(setup_syllables));

    // Default glyph pre-processing group
    planner
        .ot_map
        .enable_feature(hb_tag_t::from_bytes(b"locl"), F_PER_SYLLABLE, 1);
    planner
        .ot_map
        .enable_feature(hb_tag_t::from_bytes(b"ccmp"), F_PER_SYLLABLE, 1);
    planner
        .ot_map
        .enable_feature(hb_tag_t::from_bytes(b"nukt"), F_PER_SYLLABLE, 1);
    planner.ot_map.enable_feature(
        hb_tag_t::from_bytes(b"akhn"),
        F_MANUAL_ZWJ | F_PER_SYLLABLE,
        1,
    );

    // Reordering group
    planner
        .ot_map
        .add_gsub_pause(Some(crate::hb::ot_layout::_hb_clear_substitution_flags));
    planner.ot_map.add_feature(
        hb_tag_t::from_bytes(b"rphf"),
        F_MANUAL_ZWJ | F_PER_SYLLABLE,
        1,
    );
    planner.ot_map.add_gsub_pause(Some(record_rphf));
    planner
        .ot_map
        .add_gsub_pause(Some(crate::hb::ot_layout::_hb_clear_substitution_flags));
    planner.ot_map.enable_feature(
        hb_tag_t::from_bytes(b"pref"),
        F_MANUAL_ZWJ | F_PER_SYLLABLE,
        1,
    );
    planner.ot_map.add_gsub_pause(Some(record_pref));

    // Orthographic unit shaping group
    for feature in BASIC_FEATURES {
        planner
            .ot_map
            .enable_feature(*feature, F_MANUAL_ZWJ | F_PER_SYLLABLE, 1);
    }

    planner.ot_map.add_gsub_pause(Some(reorder_use));
    planner.ot_map.add_gsub_pause(Some(syllabic_clear_var)); // Don't need syllables anymore.

    // Topographical features
    for feature in TOPOGRAPHICAL_FEATURES {
        planner.ot_map.add_feature(*feature, F_NONE, 1);
    }
    planner.ot_map.add_gsub_pause(None);

    // Standard typographic presentation
    for feature in OTHER_FEATURES {
        planner.ot_map.enable_feature(*feature, F_MANUAL_ZWJ, 1);
    }
}

fn setup_syllables(plan: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    super::ot_shaper_use_machine::find_syllables(buffer);

    foreach_syllable!(buffer, start, end, {
        buffer.unsafe_to_break(Some(start), Some(end));
    });

    setup_rphf_mask(plan, buffer);
    setup_topographical_masks(plan, buffer);

    false
}

fn setup_rphf_mask(plan: &hb_ot_shape_plan_t, buffer: &mut hb_buffer_t) -> bool {
    let universal_plan = plan.data::<UniversalShapePlan>();

    let mask = universal_plan.rphf_mask;
    if mask == 0 {
        return false;
    }

    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        let limit = if buffer.info[start].use_category() == category::R {
            1
        } else {
            core::cmp::min(3, end - start)
        };

        for i in start..start + limit {
            buffer.info[i].mask |= mask;
        }

        start = end;
        end = buffer.next_syllable(start);
    }

    false
}

fn setup_topographical_masks(plan: &hb_ot_shape_plan_t, buffer: &mut hb_buffer_t) {
    use super::ot_shaper_use_machine::SyllableType;

    if plan.data::<UniversalShapePlan>().arabic_plan.is_some() {
        return;
    }

    let mut masks = [0; 4];
    let mut all_masks = 0;
    for i in 0..4 {
        masks[i] = plan.ot_map.get_1_mask(TOPOGRAPHICAL_FEATURES[i]);
        if masks[i] == plan.ot_map.get_global_mask() {
            masks[i] = 0;
        }

        all_masks |= masks[i];
    }

    if all_masks == 0 {
        return;
    }

    let other_masks = !all_masks;

    let mut last_start = 0;
    let mut last_form = None;
    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        let syllable = buffer.info[start].syllable() & 0x0F;
        if syllable == SyllableType::HieroglyphCluster as u8
            || syllable == SyllableType::NonCluster as u8
        {
            last_form = None;
        } else {
            let join = last_form == Some(JoiningForm::Terminal)
                || last_form == Some(JoiningForm::Isolated);

            if join {
                // Fixup previous syllable's form.
                let form = if last_form == Some(JoiningForm::Terminal) {
                    JoiningForm::Medial
                } else {
                    JoiningForm::Initial
                };

                for i in last_start..start {
                    buffer.info[i].mask =
                        (buffer.info[i].mask & other_masks) | masks[form as usize];
                }
            }

            // Form for this syllable.
            let form = if join {
                JoiningForm::Terminal
            } else {
                JoiningForm::Isolated
            };
            last_form = Some(form);
            for i in start..end {
                buffer.info[i].mask = (buffer.info[i].mask & other_masks) | masks[form as usize];
            }
        }

        last_start = start;
        start = end;
        end = buffer.next_syllable(start);
    }
}

fn record_rphf(plan: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    let universal_plan = plan.data::<UniversalShapePlan>();

    let mask = universal_plan.rphf_mask;
    if mask == 0 {
        return false;
    }

    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        // Mark a substituted repha as USE_R.
        for i in start..end {
            if buffer.info[i].mask & mask == 0 {
                break;
            }

            if _hb_glyph_info_substituted(&buffer.info[i]) {
                buffer.info[i].set_use_category(category::R);
                break;
            }
        }

        start = end;
        end = buffer.next_syllable(start);
    }

    false
}

fn reorder_use(_: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    use super::ot_shaper_use_machine::SyllableType;

    let mut ret = false;

    if crate::hb::ot_shaper_syllabic::insert_dotted_circles(
        face,
        buffer,
        SyllableType::BrokenCluster as u8,
        category::B,
        Some(category::R),
        None,
    ) {
        ret = true;
    }

    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        reorder_syllable_use(start, end, buffer);
        start = end;
        end = buffer.next_syllable(start);
    }

    ret
}

const fn category_flag(c: Category) -> u32 {
    rb_flag(c as u32)
}

const fn category_flag64(c: Category) -> u64 {
    rb_flag64(c as u32)
}

const POST_BASE_FLAGS: u64 = category_flag64(category::FAbv)
    | category_flag64(category::FBlw)
    | category_flag64(category::FPst)
    | category_flag64(category::FMAbv)
    | category_flag64(category::FMBlw)
    | category_flag64(category::FMPst)
    | category_flag64(category::MAbv)
    | category_flag64(category::MBlw)
    | category_flag64(category::MPst)
    | category_flag64(category::MPre)
    | category_flag64(category::VAbv)
    | category_flag64(category::VBlw)
    | category_flag64(category::VPst)
    | category_flag64(category::VPre)
    | category_flag64(category::VMAbv)
    | category_flag64(category::VMBlw)
    | category_flag64(category::VMPst)
    | category_flag64(category::VMPre);

fn reorder_syllable_use(start: usize, end: usize, buffer: &mut hb_buffer_t) {
    use super::ot_shaper_use_machine::SyllableType;

    let syllable_type = (buffer.info[start].syllable() & 0x0F) as u32;
    // Only a few syllable types need reordering.
    if (rb_flag_unsafe(syllable_type)
        & (rb_flag(SyllableType::ViramaTerminatedCluster as u32)
            | rb_flag(SyllableType::SakotTerminatedCluster as u32)
            | rb_flag(SyllableType::StandardCluster as u32)
            | rb_flag(SyllableType::BrokenCluster as u32)
            | 0))
        == 0
    {
        return;
    }

    // Move things forward.
    if buffer.info[start].use_category() == category::R && end - start > 1 {
        // Got a repha.  Reorder it towards the end, but before the first post-base glyph.
        for i in start + 1..end {
            let is_post_base_glyph =
                (rb_flag64_unsafe(buffer.info[i].use_category() as u32) & POST_BASE_FLAGS) != 0
                    || buffer.info[i].is_halant_use();

            if is_post_base_glyph || i == end - 1 {
                // If we hit a post-base glyph, move before it; otherwise move to the
                // end. Shift things in between backward.

                let mut i = i;
                if is_post_base_glyph {
                    i -= 1;
                }

                buffer.merge_clusters(start, i + 1);
                let t = buffer.info[start];
                for k in 0..i - start {
                    buffer.info[k + start] = buffer.info[k + start + 1];
                }
                buffer.info[i] = t;

                break;
            }
        }
    }

    // Move things back.
    let mut j = start;
    for i in start..end {
        let flag = rb_flag_unsafe(buffer.info[i].use_category() as u32);
        if buffer.info[i].is_halant_use() {
            // If we hit a halant, move after it; otherwise move to the beginning, and
            // shift things in between forward.
            j = i + 1;
        } else if (flag & (category_flag(category::VPre) | category_flag(category::VMPre))) != 0
            && _hb_glyph_info_get_lig_comp(&buffer.info[i]) == 0
            && j < i
        {
            // Only move the first component of a MultipleSubst.
            buffer.merge_clusters(j, i + 1);
            let t = buffer.info[i];
            for k in (0..i - j).rev() {
                buffer.info[k + j + 1] = buffer.info[k + j];
            }
            buffer.info[j] = t;
        }
    }
}

fn record_pref(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        // Mark a substituted pref as VPre, as they behave the same way.
        for i in start..end {
            if _hb_glyph_info_substituted(&buffer.info[i]) {
                buffer.info[i].set_use_category(category::VPre);
                break;
            }
        }

        start = end;
        end = buffer.next_syllable(start);
    }

    false
}

fn has_arabic_joining(script: Script) -> bool {
    // List of scripts that have data in arabic-table.
    matches!(
        script,
        script::ADLAM
            | script::ARABIC
            | script::CHORASMIAN
            | script::HANIFI_ROHINGYA
            | script::MANDAIC
            | script::MANICHAEAN
            | script::MONGOLIAN
            | script::NKO
            | script::OLD_UYGHUR
            | script::PHAGS_PA
            | script::PSALTER_PAHLAVI
            | script::SOGDIAN
            | script::SYRIAC
    )
}

fn preprocess_text(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) {
    super::ot_shaper_vowel_constraints::preprocess_text_vowel_constraints(buffer);
}

fn compose(_: &hb_ot_shape_normalize_context_t, a: char, b: char) -> Option<char> {
    // Avoid recomposing split matras.
    if a.general_category().is_mark() {
        return None;
    }

    crate::hb::unicode::compose(a, b)
}

fn setup_masks(plan: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) {
    let universal_plan = plan.data::<UniversalShapePlan>();

    // Do this before allocating use_category().
    if let Some(ref arabic_plan) = universal_plan.arabic_plan {
        crate::hb::ot_shaper_arabic::setup_masks_inner(arabic_plan, plan.script, buffer);
    }

    // We cannot setup masks here. We save information about characters
    // and setup masks later on in a pause-callback.
    for info in buffer.info_slice_mut() {
        info.set_use_category(super::ot_shaper_use_table::hb_use_get_category(
            info.glyph_id,
        ));
    }
}
