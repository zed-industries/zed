use alloc::boxed::Box;
use core::cmp;
use core::ops::Range;

use read_fonts::types::GlyphId;

use crate::hb::unicode::Codepoint;

use super::algs::*;
use super::buffer::*;
use super::ot_layout::*;
use super::ot_layout_gsubgpos::WouldApplyContext;
use super::ot_map::*;
use super::ot_shape::*;
use super::ot_shape_normalize::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::*;
use super::ot_shaper_syllabic::*;
use super::unicode::{hb_gc, CharExt};
use super::{hb_font_t, hb_mask_t, hb_tag_t, script, GlyphInfo, Script};

pub const INDIC_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: Some(collect_features),
    override_features: Some(override_features),
    create_data: Some(|plan| Box::new(IndicShapePlan::new(plan))),
    preprocess_text: Some(preprocess_text),
    postprocess_glyphs: None,
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_COMPOSED_DIACRITICS_NO_SHORT_CIRCUIT,
    decompose: Some(decompose),
    compose: Some(compose),
    setup_masks: Some(setup_masks),
    gpos_tag: None,
    reorder_marks: None,
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_NONE,
    fallback_position: false,
};

impl GlyphInfo {
    declare_buffer_var_alias!(
        OT_SHAPER_VAR_U8_CATEGORY_VAR,
        u8,
        INDIC_CATEGORY_VAR,
        indic_category,
        set_indic_category
    );
    declare_buffer_var_alias!(
        OT_SHAPER_VAR_U8_AUXILIARY_VAR,
        u8,
        INDIC_POSITION_VAR,
        indic_position,
        set_indic_position
    );

    fn is_one_of(&self, flags: u32) -> bool {
        // If it ligated, all bets are off.
        if self.ligated() {
            return false;
        }

        rb_flag_unsafe(self.indic_category() as u32) & flags != 0
    }

    fn is_joiner(&self) -> bool {
        self.is_one_of(JOINER_FLAGS)
    }

    pub(crate) fn is_consonant(&self) -> bool {
        self.is_one_of(CONSONANT_FLAGS_INDIC)
    }

    fn is_halant(&self) -> bool {
        self.is_one_of(rb_flag(ot_category_t::OT_H as u32))
    }

    fn set_indic_properties(&mut self) {
        let u = self.glyph_id;
        let (cat, pos) = crate::hb::ot_shaper_indic_table::get_categories(u);

        self.set_indic_category(cat);
        self.set_indic_position(pos);
    }
}

pub type Category = u8;

// This mod doesn't exist in harfbuzz anymore. Instead, the corresponding values are auto-generated
// by the various machines and stored in `hb-ot-shaper-indic-table`. This means that when updating the
// values in the machines, we also need to update them here.
#[allow(dead_code)]
pub mod ot_category_t {
    pub const OT_X: u8 = 0;
    pub const OT_C: u8 = 1;
    pub const OT_V: u8 = 2;
    pub const OT_N: u8 = 3;
    pub const OT_H: u8 = 4;
    pub const OT_ZWNJ: u8 = 5;
    pub const OT_ZWJ: u8 = 6;
    pub const OT_M: u8 = 7;
    pub const OT_SM: u8 = 8;
    pub const OT_A: u8 = 9;
    pub const OT_VD: u8 = OT_A;
    pub const OT_PLACEHOLDER: u8 = 10;
    pub const OT_GB: u8 = OT_PLACEHOLDER;
    pub const OT_DOTTEDCIRCLE: u8 = 11;
    pub const OT_RS: u8 = 12; // Register Shifter, used in Khmer OT spec.
    pub const OT_MPst: u8 = 13;
    pub const OT_Repha: u8 = 14; // Atomically-encoded logical or visual repha.
    pub const OT_Ra: u8 = 15;
    pub const OT_CM: u8 = 16; // Consonant-Medial.
    pub const OT_Symbol: u8 = 17; // Avagraha, etc that take marks (SM,A,VD).
    pub const OT_CS: u8 = 18;

    /* Khmer & Myanmar shapers. */
    pub const OT_VAbv: u8 = 20;
    pub const OT_VBlw: u8 = 21;
    pub const OT_VPre: u8 = 22;
    pub const OT_VPst: u8 = 23;

    /* Khmer. */
    pub const OT_Robatic: u8 = 25;
    pub const OT_Xgroup: u8 = 26;
    pub const OT_Ygroup: u8 = 27;

    /* Myanmar */
    pub const OT_As: u8 = 32; // Asat
    pub const OT_MH: u8 = 35; // Medial
    pub const OT_MR: u8 = 36; // Medial
    pub const OT_MW: u8 = 37; // Medial
    pub const OT_MY: u8 = 38; // Medial
    pub const OT_PT: u8 = 39; // Pwo and other tones
    pub const OT_VS: u8 = 40; // Variation selectors
    pub const OT_ML: u8 = 41; // Consonant medials

    pub const OT_SMPst: u8 = 57; // Syllable Medial Post-base

    // This one doesn't exist in ot_category_t in harfbuzz, only in
    // the Myanmar machine. However, in Rust we unfortunately can't export
    // inside the Ragel file, so we have to define it here as well. Needs to
    // be kept in sync with the value in the machine.
    pub const IV: u8 = 2;
}

pub mod ot_position_t {
    pub const POS_START: u8 = 0;

    pub const POS_RA_TO_BECOME_REPH: u8 = 1;
    pub const POS_PRE_M: u8 = 2;
    pub const POS_PRE_C: u8 = 3;

    pub const POS_BASE_C: u8 = 4;
    pub const POS_AFTER_MAIN: u8 = 5;

    pub const POS_ABOVE_C: u8 = 6;

    pub const POS_BEFORE_SUB: u8 = 7;
    pub const POS_BELOW_C: u8 = 8;
    pub const POS_AFTER_SUB: u8 = 9;

    pub const POS_BEFORE_POST: u8 = 10;
    pub const POS_POST_C: u8 = 11;
    pub const POS_AFTER_POST: u8 = 12;

    pub const POS_SMVD: u8 = 13;

    pub const POS_END: u8 = 14;
}

const INDIC_FEATURES: &[(hb_tag_t, hb_ot_map_feature_flags_t)] = &[
    // Basic features.
    // These features are applied in order, one at a time, after initial_reordering,
    // constrained to the syllable.
    (
        hb_tag_t::new(b"nukt"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"akhn"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (hb_tag_t::new(b"rphf"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (
        hb_tag_t::new(b"rkrf"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (hb_tag_t::new(b"pref"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (hb_tag_t::new(b"blwf"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (hb_tag_t::new(b"abvf"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (hb_tag_t::new(b"half"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (hb_tag_t::new(b"pstf"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (
        hb_tag_t::new(b"vatu"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"cjct"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    // Other features.
    // These features are applied all at once, after final_reordering, constrained
    // to the syllable.
    // Default Bengali font in Windows for example has intermixed
    // lookups for init,pres,abvs,blws features.
    (hb_tag_t::new(b"init"), F_MANUAL_JOINERS | F_PER_SYLLABLE),
    (
        hb_tag_t::new(b"pres"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"abvs"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"blws"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"psts"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
    (
        hb_tag_t::new(b"haln"),
        F_GLOBAL_MANUAL_JOINERS | F_PER_SYLLABLE,
    ),
];

// Must be in the same order as the INDIC_FEATURES array.
#[allow(dead_code)]
mod indic_feature {
    pub const NUKT: usize = 0;
    pub const AKHN: usize = 1;
    pub const RPHF: usize = 2;
    pub const RKRF: usize = 3;
    pub const PREF: usize = 4;
    pub const BLWF: usize = 5;
    pub const ABVF: usize = 6;
    pub const HALF: usize = 7;
    pub const PSTF: usize = 8;
    pub const VATU: usize = 9;
    pub const CJCT: usize = 10;
    pub const INIT: usize = 11;
    pub const PRES: usize = 12;
    pub const ABVS: usize = 13;
    pub const BLWS: usize = 14;
    pub const PSTS: usize = 15;
    pub const HALN: usize = 16;
}

pub(crate) const fn category_flag(c: Category) -> u32 {
    rb_flag(c as u32)
}

// Note:
//
// We treat Vowels and placeholders as if they were consonants.  This is safe because Vowels
// cannot happen in a consonant syllable.  The plus side however is, we can call the
// consonant syllable logic from the vowel syllable function and get it all right!
const CONSONANT_FLAGS_INDIC: u32 = category_flag(ot_category_t::OT_C)
    | category_flag(ot_category_t::OT_CS)
    | category_flag(ot_category_t::OT_Ra)
    | category_flag(ot_category_t::OT_CM)
    | category_flag(ot_category_t::OT_V)
    | category_flag(ot_category_t::OT_PLACEHOLDER)
    | category_flag(ot_category_t::OT_DOTTEDCIRCLE);

const JOINER_FLAGS: u32 =
    category_flag(ot_category_t::OT_ZWJ) | category_flag(ot_category_t::OT_ZWNJ);

#[derive(Clone, Copy, PartialEq)]
enum RephPosition {
    AfterMain = ot_position_t::POS_AFTER_MAIN as isize,
    BeforeSub = ot_position_t::POS_BEFORE_SUB as isize,
    AfterSub = ot_position_t::POS_AFTER_SUB as isize,
    BeforePost = ot_position_t::POS_BEFORE_POST as isize,
    AfterPost = ot_position_t::POS_AFTER_POST as isize,
}

#[derive(Clone, Copy, PartialEq)]
enum RephMode {
    /// Reph formed out of initial Ra,H sequence.
    Implicit,
    /// Reph formed out of initial Ra,H,ZWJ sequence.
    Explicit,
    /// Encoded Repha character, needs reordering.
    LogRepha,
}

#[derive(Clone, Copy, PartialEq)]
enum BlwfMode {
    /// Below-forms feature applied to pre-base and post-base.
    PreAndPost,
    /// Below-forms feature applied to post-base only.
    PostOnly,
}

#[derive(Clone, Copy)]
struct IndicConfig {
    script: Option<Script>,
    has_old_spec: bool,
    virama: u32,
    reph_pos: RephPosition,
    reph_mode: RephMode,
    blwf_mode: BlwfMode,
}

impl IndicConfig {
    const fn new(
        script: Option<Script>,
        has_old_spec: bool,
        virama: u32,
        reph_pos: RephPosition,
        reph_mode: RephMode,
        blwf_mode: BlwfMode,
    ) -> Self {
        IndicConfig {
            script,
            has_old_spec,
            virama,
            reph_pos,
            reph_mode,
            blwf_mode,
        }
    }
}

static INDIC_CONFIGS: &[IndicConfig] = &[
    IndicConfig::new(
        None,
        false,
        0,
        RephPosition::BeforePost,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::DEVANAGARI),
        true,
        0x094D,
        RephPosition::BeforePost,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::BENGALI),
        true,
        0x09CD,
        RephPosition::AfterSub,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::GURMUKHI),
        true,
        0x0A4D,
        RephPosition::BeforeSub,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::GUJARATI),
        true,
        0x0ACD,
        RephPosition::BeforePost,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::ORIYA),
        true,
        0x0B4D,
        RephPosition::AfterMain,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::TAMIL),
        true,
        0x0BCD,
        RephPosition::AfterPost,
        RephMode::Implicit,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::TELUGU),
        true,
        0x0C4D,
        RephPosition::AfterPost,
        RephMode::Explicit,
        BlwfMode::PostOnly,
    ),
    IndicConfig::new(
        Some(script::KANNADA),
        true,
        0x0CCD,
        RephPosition::AfterPost,
        RephMode::Implicit,
        BlwfMode::PostOnly,
    ),
    IndicConfig::new(
        Some(script::MALAYALAM),
        true,
        0x0D4D,
        RephPosition::AfterMain,
        RephMode::LogRepha,
        BlwfMode::PreAndPost,
    ),
    IndicConfig::new(
        Some(script::SINHALA),
        false,
        0x0DCA,
        RephPosition::AfterPost,
        RephMode::Explicit,
        BlwfMode::PreAndPost,
    ),
];

struct IndicWouldSubstituteFeature {
    lookups: Range<usize>,
    zero_context: bool,
}

impl IndicWouldSubstituteFeature {
    pub fn new(map: &hb_ot_map_t, feature_tag: hb_tag_t, zero_context: bool) -> Self {
        IndicWouldSubstituteFeature {
            lookups: match map.get_feature_stage(TableIndex::GSUB, feature_tag) {
                Some(stage) => map.stage_lookup_range(TableIndex::GSUB, stage),
                None => 0..0,
            },
            zero_context,
        }
    }

    pub fn would_substitute(
        &self,
        map: &hb_ot_map_t,
        face: &hb_font_t,
        glyphs: &[GlyphId],
    ) -> bool {
        for index in self.lookups.clone() {
            let lookup = map.lookup(TableIndex::GSUB, index);
            let ctx = WouldApplyContext {
                glyphs,
                zero_context: self.zero_context,
            };
            if face
                .ot_tables
                .gsub
                .as_ref()
                .and_then(|table| table.get_lookup(lookup.index))
                .is_some_and(|lookup| lookup.would_apply(face, &ctx) == Some(true))
            {
                return true;
            }
        }

        false
    }
}

struct IndicShapePlan {
    config: IndicConfig,
    is_old_spec: bool,
    // virama_glyph: Option<u32>,
    rphf: IndicWouldSubstituteFeature,
    pref: IndicWouldSubstituteFeature,
    blwf: IndicWouldSubstituteFeature,
    pstf: IndicWouldSubstituteFeature,
    vatu: IndicWouldSubstituteFeature,
    mask_array: [hb_mask_t; INDIC_FEATURES.len()],
}

impl IndicShapePlan {
    fn new(plan: &hb_ot_shape_plan_t) -> Self {
        let script = plan.script;
        let config = if let Some(c) = INDIC_CONFIGS.iter().skip(1).find(|c| c.script == script) {
            *c
        } else {
            INDIC_CONFIGS[0]
        };

        let is_old_spec = config.has_old_spec
            && plan
                .ot_map
                .chosen_script(TableIndex::GSUB)
                .is_none_or(|tag| tag.to_be_bytes()[3] != b'2');

        // Use zero-context would_substitute() matching for new-spec of the main
        // Indic scripts, and scripts with one spec only, but not for old-specs.
        // The new-spec for all dual-spec scripts says zero-context matching happens.
        //
        // However, testing with Malayalam shows that old and new spec both allow
        // context.  Testing with Bengali new-spec however shows that it doesn't.
        // So, the heuristic here is the way it is.  It should *only* be changed,
        // as we discover more cases of what Windows does.  DON'T TOUCH OTHERWISE.
        let zero_context = is_old_spec && script != Some(script::MALAYALAM);

        let mut mask_array = [0; INDIC_FEATURES.len()];
        for (i, feature) in INDIC_FEATURES.iter().enumerate() {
            mask_array[i] = if feature.1 & F_GLOBAL != 0 {
                0
            } else {
                plan.ot_map.get_1_mask(feature.0)
            }
        }

        // TODO: what is this?
        // let mut virama_glyph = None;
        // if config.virama != 0 {
        //     if let Some(g) = face.glyph_index(char::try_from(config.virama).unwrap()) {
        //         virama_glyph = Some(g.0 as u32);
        //     }
        // }

        IndicShapePlan {
            config,
            is_old_spec,
            // virama_glyph,
            rphf: IndicWouldSubstituteFeature::new(
                &plan.ot_map,
                hb_tag_t::new(b"rphf"),
                zero_context,
            ),
            pref: IndicWouldSubstituteFeature::new(
                &plan.ot_map,
                hb_tag_t::new(b"pref"),
                zero_context,
            ),
            blwf: IndicWouldSubstituteFeature::new(
                &plan.ot_map,
                hb_tag_t::new(b"blwf"),
                zero_context,
            ),
            pstf: IndicWouldSubstituteFeature::new(
                &plan.ot_map,
                hb_tag_t::new(b"pstf"),
                zero_context,
            ),
            vatu: IndicWouldSubstituteFeature::new(
                &plan.ot_map,
                hb_tag_t::new(b"vatu"),
                zero_context,
            ),
            mask_array,
        }
    }
}

fn collect_features(planner: &mut hb_ot_shape_planner_t) {
    // Do this before any lookups have been applied.
    planner.ot_map.add_gsub_pause(Some(setup_syllables));

    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"locl"), F_PER_SYLLABLE, 1);
    // The Indic specs do not require ccmp, but we apply it here since if
    // there is a use of it, it's typically at the beginning.
    planner
        .ot_map
        .enable_feature(hb_tag_t::new(b"ccmp"), F_PER_SYLLABLE, 1);

    planner.ot_map.add_gsub_pause(Some(initial_reordering));

    for feature in INDIC_FEATURES.iter().take(11) {
        planner.ot_map.add_feature(feature.0, feature.1, 1);
        planner.ot_map.add_gsub_pause(None);
    }

    planner.ot_map.add_gsub_pause(Some(final_reordering));

    for feature in INDIC_FEATURES.iter().skip(11) {
        planner.ot_map.add_feature(feature.0, feature.1, 1);
    }
}

fn override_features(planner: &mut hb_ot_shape_planner_t) {
    planner.ot_map.disable_feature(hb_tag_t::new(b"liga"));
    planner.ot_map.add_gsub_pause(Some(syllabic_clear_var)); // Don't need syllables anymore.
}

fn preprocess_text(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) {
    super::ot_shaper_vowel_constraints::preprocess_text_vowel_constraints(buffer);
}

fn decompose(_: &hb_ot_shape_normalize_context_t, ab: Codepoint) -> Option<(Codepoint, Codepoint)> {
    // Don't decompose these.
    match ab {
        0x0931 |               // DEVANAGARI LETTER RRA
        // https://github.com/harfbuzz/harfbuzz/issues/779
        0x09DC |               // BENGALI LETTER RRA
        0x09DD |               // BENGALI LETTER RHA
        0x0B94 => return None, // TAMIL LETTER AU
        _ => {}
    }

    crate::hb::unicode::decompose(ab)
}

fn compose(_: &hb_ot_shape_normalize_context_t, a: Codepoint, b: Codepoint) -> Option<Codepoint> {
    // Avoid recomposing split matras.
    if a.general_category().is_mark() {
        return None;
    }

    // Composition-exclusion exceptions that we want to recompose.
    if a == 0x09AF && b == 0x09BC {
        return Some(0x09DF);
    }

    crate::hb::unicode::compose(a, b)
}

fn setup_masks(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) {
    buffer.allocate_var(GlyphInfo::INDIC_CATEGORY_VAR);
    buffer.allocate_var(GlyphInfo::INDIC_POSITION_VAR);

    // We cannot setup masks here.  We save information about characters
    // and setup masks later on in a pause-callback.
    for info in buffer.info_slice_mut() {
        info.set_indic_properties();
    }
}

fn setup_syllables(_: &hb_ot_shape_plan_t, _: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    buffer.allocate_var(GlyphInfo::SYLLABLE_VAR);

    super::ot_shaper_indic_machine::find_syllables_indic(buffer);

    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        buffer.unsafe_to_break(Some(start), Some(end));
        start = end;
        end = buffer.next_syllable(start);
    }

    false
}

fn initial_reordering(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) -> bool {
    use super::ot_shaper_indic_machine::SyllableType;

    let mut ret = false;

    let indic_plan = plan.data::<IndicShapePlan>();

    update_consonant_positions(plan, indic_plan, face, buffer);
    if insert_dotted_circles(
        face,
        buffer,
        SyllableType::BrokenCluster as u8,
        ot_category_t::OT_DOTTEDCIRCLE,
        Some(ot_category_t::OT_Repha),
        Some(ot_position_t::POS_END),
    ) {
        ret = true;
    }

    let mut start = 0;
    let mut end = buffer.next_syllable(0);
    while start < buffer.len {
        initial_reordering_syllable(plan, indic_plan, face, start, end, buffer);
        start = end;
        end = buffer.next_syllable(start);
    }

    ret
}

fn update_consonant_positions(
    plan: &hb_ot_shape_plan_t,
    indic_plan: &IndicShapePlan,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) {
    let mut virama_glyph = None;
    if indic_plan.config.virama != 0 {
        virama_glyph = face.get_nominal_glyph(indic_plan.config.virama);
    }

    if let Some(virama) = virama_glyph {
        for info in buffer.info_slice_mut() {
            if info.indic_position() == ot_position_t::POS_BASE_C {
                let consonant = info.as_glyph();
                info.set_indic_position(consonant_position_from_face(
                    plan, indic_plan, face, consonant, virama,
                ));
            }
        }
    }
}

fn consonant_position_from_face(
    plan: &hb_ot_shape_plan_t,
    indic_plan: &IndicShapePlan,
    face: &hb_font_t,
    consonant: GlyphId,
    virama: GlyphId,
) -> u8 {
    // For old-spec, the order of glyphs is Consonant,Virama,
    // whereas for new-spec, it's Virama,Consonant.  However,
    // some broken fonts (like Free Sans) simply copied lookups
    // from old-spec to new-spec without modification.
    // And oddly enough, Uniscribe seems to respect those lookups.
    // Eg. in the sequence U+0924,U+094D,U+0930, Uniscribe finds
    // base at 0.  The font however, only has lookups matching
    // 930,94D in 'blwf', not the expected 94D,930 (with new-spec
    // table).  As such, we simply match both sequences.  Seems
    // to work.
    //
    // Vatu is done as well, for:
    // https://github.com/harfbuzz/harfbuzz/issues/1587

    if indic_plan
        .blwf
        .would_substitute(&plan.ot_map, face, &[virama, consonant])
        || indic_plan
            .blwf
            .would_substitute(&plan.ot_map, face, &[consonant, virama])
        || indic_plan
            .vatu
            .would_substitute(&plan.ot_map, face, &[virama, consonant])
        || indic_plan
            .vatu
            .would_substitute(&plan.ot_map, face, &[consonant, virama])
    {
        return ot_position_t::POS_BELOW_C;
    }

    if indic_plan
        .pstf
        .would_substitute(&plan.ot_map, face, &[virama, consonant])
        || indic_plan
            .pstf
            .would_substitute(&plan.ot_map, face, &[consonant, virama])
    {
        return ot_position_t::POS_POST_C;
    }

    if indic_plan
        .pref
        .would_substitute(&plan.ot_map, face, &[virama, consonant])
        || indic_plan
            .pref
            .would_substitute(&plan.ot_map, face, &[consonant, virama])
    {
        return ot_position_t::POS_POST_C;
    }

    ot_position_t::POS_BASE_C
}

fn initial_reordering_syllable(
    plan: &hb_ot_shape_plan_t,
    indic_plan: &IndicShapePlan,
    face: &hb_font_t,
    start: usize,
    end: usize,
    buffer: &mut hb_buffer_t,
) {
    use super::ot_shaper_indic_machine::SyllableType;

    let syllable_type = match buffer.info[start].syllable() & 0x0F {
        0 => SyllableType::ConsonantSyllable,
        1 => SyllableType::VowelSyllable,
        2 => SyllableType::StandaloneCluster,
        3 => SyllableType::SymbolCluster,
        4 => SyllableType::BrokenCluster,
        5 => SyllableType::NonIndicCluster,
        _ => unreachable!(),
    };

    match syllable_type {
        // We made the vowels look like consonants.  So let's call the consonant logic!
        SyllableType::VowelSyllable | SyllableType::ConsonantSyllable => {
            initial_reordering_consonant_syllable(plan, indic_plan, face, start, end, buffer);
        }
        // We already inserted dotted-circles, so just call the standalone_cluster.
        SyllableType::BrokenCluster | SyllableType::StandaloneCluster => {
            initial_reordering_standalone_cluster(plan, indic_plan, face, start, end, buffer);
        }
        SyllableType::SymbolCluster | SyllableType::NonIndicCluster => {}
    }
}

// Rules from:
// https://docs.microsqoft.com/en-us/typography/script-development/devanagari */
fn initial_reordering_consonant_syllable(
    plan: &hb_ot_shape_plan_t,
    indic_plan: &IndicShapePlan,
    face: &hb_font_t,
    start: usize,
    end: usize,
    buffer: &mut hb_buffer_t,
) {
    // https://github.com/harfbuzz/harfbuzz/issues/435#issuecomment-335560167
    // For compatibility with legacy usage in Kannada,
    // Ra+h+ZWJ must behave like Ra+ZWJ+h...
    if buffer.script == Some(script::KANNADA)
        && start + 3 <= end
        && buffer.info[start].is_one_of(category_flag(ot_category_t::OT_Ra))
        && buffer.info[start + 1].is_one_of(category_flag(ot_category_t::OT_H))
        && buffer.info[start + 2].is_one_of(category_flag(ot_category_t::OT_ZWJ))
    {
        buffer.merge_clusters(start + 1, start + 3);
        buffer.info.swap(start + 1, start + 2);
    }

    // 1. Find base consonant:
    //
    // The shaping engine finds the base consonant of the syllable, using the
    // following algorithm: starting from the end of the syllable, move backwards
    // until a consonant is found that does not have a below-base or post-base
    // form (post-base forms have to follow below-base forms), or that is not a
    // pre-base-reordering Ra, or arrive at the first consonant. The consonant
    // stopped at will be the base.
    //
    //   - If the syllable starts with Ra + Halant (in a script that has Reph)
    //     and has more than one consonant, Ra is excluded from candidates for
    //     base consonants.

    let mut base = end;
    let mut has_reph = false;

    {
        // -> If the syllable starts with Ra + Halant (in a script that has Reph)
        //    and has more than one consonant, Ra is excluded from candidates for
        //    base consonants.
        let mut limit = start;
        if indic_plan.mask_array[indic_feature::RPHF] != 0
            && start + 3 <= end
            && ((indic_plan.config.reph_mode == RephMode::Implicit
                && !buffer.info[start + 2].is_joiner())
                || (indic_plan.config.reph_mode == RephMode::Explicit
                    && buffer.info[start + 2].indic_category() == ot_category_t::OT_ZWJ))
        {
            // See if it matches the 'rphf' feature.
            let glyphs = &[
                buffer.info[start].as_glyph(),
                buffer.info[start + 1].as_glyph(),
                if indic_plan.config.reph_mode == RephMode::Explicit {
                    buffer.info[start + 2].as_glyph()
                } else {
                    GlyphId::NOTDEF
                },
            ];
            if indic_plan
                .rphf
                .would_substitute(&plan.ot_map, face, &glyphs[0..2])
                || (indic_plan.config.reph_mode == RephMode::Explicit
                    && indic_plan.rphf.would_substitute(&plan.ot_map, face, glyphs))
            {
                limit += 2;
                while limit < end && buffer.info[limit].is_joiner() {
                    limit += 1;
                }
                base = start;
                has_reph = true;
            }
        } else if indic_plan.config.reph_mode == RephMode::LogRepha
            && buffer.info[start].indic_category() == ot_category_t::OT_Repha
        {
            limit += 1;
            while limit < end && buffer.info[limit].is_joiner() {
                limit += 1;
            }
            base = start;
            has_reph = true;
        }

        {
            // -> starting from the end of the syllable, move backwards
            let mut i = end;
            let mut seen_below = false;
            loop {
                i -= 1;
                // -> until a consonant is found
                if buffer.info[i].is_consonant() {
                    // -> that does not have a below-base or post-base form
                    // (post-base forms have to follow below-base forms),
                    if buffer.info[i].indic_position() != ot_position_t::POS_BELOW_C
                        && (buffer.info[i].indic_position() != ot_position_t::POS_POST_C
                            || seen_below)
                    {
                        base = i;
                        break;
                    }
                    if buffer.info[i].indic_position() == ot_position_t::POS_BELOW_C {
                        seen_below = true;
                    }

                    // -> or that is not a pre-base-reordering Ra,
                    //
                    // IMPLEMENTATION NOTES:
                    //
                    // Our pre-base-reordering Ra's are marked position::PostC, so will be skipped
                    // by the logic above already.

                    // -> or arrive at the first consonant. The consonant stopped at will
                    // be the base.
                    base = i;
                } else {
                    // A ZWJ after a Halant stops the base search, and requests an explicit
                    // half form.
                    // A ZWJ before a Halant, requests a subjoined form instead, and hence
                    // search continues.  This is particularly important for Bengali
                    // sequence Ra,H,Ya that should form Ya-Phalaa by subjoining Ya.
                    if start < i
                        && buffer.info[i].indic_category() == ot_category_t::OT_ZWJ
                        && buffer.info[i - 1].indic_category() == ot_category_t::OT_H
                    {
                        break;
                    }
                }

                if i <= limit {
                    break;
                }
            }
        }

        // -> If the syllable starts with Ra + Halant (in a script that has Reph)
        //    and has more than one consonant, Ra is excluded from candidates for
        //    base consonants.
        //
        //  Only do this for unforced Reph. (ie. not for Ra,H,ZWJ.
        if has_reph && base == start && limit - base <= 2 {
            // Have no other consonant, so Reph is not formed and Ra becomes base.
            has_reph = false;
        }
    }

    // 2. Decompose and reorder Matras:
    //
    // Each matra and any syllable modifier sign in the syllable are moved to the
    // appropriate position relative to the consonant(s) in the syllable. The
    // shaping engine decomposes two- or three-part matras into their constituent
    // parts before any repositioning. Matra characters are classified by which
    // consonant in a conjunct they have affinity for and are reordered to the
    // following positions:
    //
    //   - Before first half form in the syllable
    //   - After subjoined consonants
    //   - After post-form consonant
    //   - After main consonant (for above marks)
    //
    // IMPLEMENTATION NOTES:
    //
    // The normalize() routine has already decomposed matras for us, so we don't
    // need to worry about that.

    // 3.  Reorder marks to canonical order:
    //
    // Adjacent nukta and halant or nukta and vedic sign are always repositioned
    // if necessary, so that the nukta is first.
    //
    // IMPLEMENTATION NOTES:
    //
    // We don't need to do this: the normalize() routine already did this for us.

    // Reorder characters

    for i in start..base {
        let pos = buffer.info[i].indic_position();
        buffer.info[i].set_indic_position(cmp::min(ot_position_t::POS_PRE_C, pos));
    }

    if base < end {
        buffer.info[base].set_indic_position(ot_position_t::POS_BASE_C);
    }

    // Handle beginning Ra
    if has_reph {
        buffer.info[start].set_indic_position(ot_position_t::POS_RA_TO_BECOME_REPH);
    }

    // For old-style Indic script tags, move the first post-base Halant after
    // last consonant.
    //
    // Reports suggest that in some scripts Uniscribe does this only if there
    // is *not* a Halant after last consonant already.  We know that is the
    // case for Kannada, while it reorders unconditionally in other scripts,
    // eg. Malayalam, Bengali, and Devanagari.  We don't currently know about
    // other scripts, so we block Kannada.
    //
    // Kannada test case:
    // U+0C9A,U+0CCD,U+0C9A,U+0CCD
    // With some versions of Lohit Kannada.
    // https://bugs.freedesktop.org/show_bug.cgi?id=59118
    //
    // Malayalam test case:
    // U+0D38,U+0D4D,U+0D31,U+0D4D,U+0D31,U+0D4D
    // With lohit-ttf-20121122/Lohit-Malayalam.ttf
    //
    // Bengali test case:
    // U+0998,U+09CD,U+09AF,U+09CD
    // With Windows XP vrinda.ttf
    // https://github.com/harfbuzz/harfbuzz/issues/1073
    //
    // Devanagari test case:
    // U+091F,U+094D,U+0930,U+094D
    // With chandas.ttf
    // https://github.com/harfbuzz/harfbuzz/issues/1071
    if indic_plan.is_old_spec {
        let disallow_double_halants = buffer.script == Some(script::KANNADA);
        for i in base + 1..end {
            if buffer.info[i].indic_category() == ot_category_t::OT_H {
                let mut j = end - 1;
                while j > i {
                    if buffer.info[j].is_consonant()
                        || (disallow_double_halants
                            && buffer.info[j].indic_category() == ot_category_t::OT_H)
                    {
                        break;
                    }

                    j -= 1;
                }

                if buffer.info[j].indic_category() != ot_category_t::OT_H && j > i {
                    // Move Halant to after last consonant.
                    let t = buffer.info[i];
                    for k in 0..j - i {
                        buffer.info[k + i] = buffer.info[k + i + 1];
                    }
                    buffer.info[j] = t;
                }

                break;
            }
        }
    }

    // Attach misc marks to previous char to move with them.
    {
        let mut last_pos = ot_position_t::POS_START;
        for i in start..end {
            let ok = rb_flag_unsafe(buffer.info[i].indic_category() as u32)
                & (category_flag(ot_category_t::OT_ZWJ)
                    | category_flag(ot_category_t::OT_ZWNJ)
                    | category_flag(ot_category_t::OT_N)
                    | category_flag(ot_category_t::OT_RS)
                    | category_flag(ot_category_t::OT_CM)
                    | category_flag(ot_category_t::OT_H))
                != 0;
            if ok {
                buffer.info[i].set_indic_position(last_pos);

                if buffer.info[i].indic_category() == ot_category_t::OT_H
                    && buffer.info[i].indic_position() == ot_position_t::POS_PRE_M
                {
                    // Uniscribe doesn't move the Halant with Left Matra.
                    // TEST: U+092B,U+093F,U+094DE
                    // We follow.
                    for j in (start + 1..=i).rev() {
                        if buffer.info[j - 1].indic_position() != ot_position_t::POS_PRE_M {
                            let pos = buffer.info[j - 1].indic_position();
                            buffer.info[i].set_indic_position(pos);
                            break;
                        }
                    }
                }
            } else if buffer.info[i].indic_position() != ot_position_t::POS_SMVD {
                if buffer.info[i].indic_category() == ot_category_t::OT_MPst
                    && i > start
                    && buffer.info[i - 1].indic_category() == ot_category_t::OT_SM
                {
                    let val = buffer.info[i].indic_position();
                    buffer.info[i - 1].set_indic_position(val);
                }

                last_pos = buffer.info[i].indic_position();
            }
        }
    }
    // For post-base consonants let them own anything before them
    // since the last consonant or matra.
    {
        let mut last = base;
        for i in base + 1..end {
            if buffer.info[i].is_consonant() {
                for j in last + 1..i {
                    if buffer.info[j].indic_position() < ot_position_t::POS_SMVD {
                        let pos = buffer.info[i].indic_position();
                        buffer.info[j].set_indic_position(pos);
                    }
                }

                last = i;
            } else if (rb_flag_unsafe(buffer.info[i].indic_category() as u32)
                & (rb_flag(ot_category_t::OT_M as u32) | rb_flag(ot_category_t::OT_MPst as u32)))
                != 0
            {
                last = i;
            }
        }
    }

    {
        // Use syllable() for sort accounting temporarily.
        let syllable = buffer.info[start].syllable();
        for i in start..end {
            // We don't care about overflow here as we won't actually use these
            // values if `end - start > 127`.
            buffer.info[i].set_syllable((i - start) as u8);
        }

        buffer.info[start..end].sort_by_key(|a| a.indic_position());

        // Find base again; also flip left-matra sequence.
        let mut first_left_mantra = end;
        let mut last_left_mantra = end;
        base = end;

        for i in start..end {
            if buffer.info[i].indic_position() == ot_position_t::POS_BASE_C {
                base = i;
                break;
            } else if buffer.info[i].indic_position() == ot_position_t::POS_PRE_M {
                if first_left_mantra == end {
                    first_left_mantra = i;
                }

                last_left_mantra = i;
            }
        }

        // https://github.com/harfbuzz/harfbuzz/issues/3863
        if first_left_mantra < last_left_mantra {
            // No need to merge clusters, handled later.
            buffer.reverse_range(first_left_mantra, last_left_mantra + 1);
            // Reverse back nuktas, etc.
            let mut i = first_left_mantra;

            for j in i..=last_left_mantra {
                if (rb_flag_unsafe(buffer.info[j].indic_category() as u32)
                    & (rb_flag(ot_category_t::OT_M as u32)
                        | rb_flag(ot_category_t::OT_MPst as u32)))
                    != 0
                {
                    buffer.reverse_range(i, j + 1);
                    i = j + 1;
                }
            }
        }

        // Things are out-of-control for post base positions, they may shuffle
        // around like crazy.  In old-spec mode, we move halants around, so in
        // that case merge all clusters after base.  Otherwise, check the sort
        // order and merge as needed.
        // For pre-base stuff, we handle cluster issues in final reordering.
        //
        // We could use buffer->sort() for this, if there was no special
        // reordering of pre-base stuff happening later...
        // We don't want to merge_clusters all of that, which buffer->sort()
        // would.  Here's a concrete example:
        //
        // Assume there's a pre-base consonant and explicit Halant before base,
        // followed by a prebase-reordering (left) Matra:
        //
        //   C,H,ZWNJ,B,M
        //
        // At this point in reordering we would have:
        //
        //   M,C,H,ZWNJ,B
        //
        // whereas in final reordering we will bring the Matra closer to Base:
        //
        //   C,H,ZWNJ,M,B
        //
        // That's why we don't want to merge-clusters anything before the Base
        // at this point.  But if something moved from after Base to before it,
        // we should merge clusters from base to them.  In final-reordering, we
        // only move things around before base, and merge-clusters up to base.
        // These two merge-clusters from the two sides of base will interlock
        // to merge things correctly.  See:
        // https://github.com/harfbuzz/harfbuzz/issues/2272
        if indic_plan.is_old_spec || end - start > 127 {
            buffer.merge_clusters(base, end);
        } else {
            // Note! syllable() is a one-byte field.
            for i in base..end {
                if buffer.info[i].syllable() != 255 {
                    let mut min = i;
                    let mut max = i;
                    let mut j = start + buffer.info[i].syllable() as usize;
                    while j != i {
                        min = cmp::min(min, j);
                        max = cmp::max(max, j);
                        let next = start + buffer.info[j].syllable() as usize;
                        buffer.info[j].set_syllable(255); // So we don't process j later again.
                        j = next;
                    }

                    buffer.merge_clusters(cmp::max(base, min), max + 1);
                }
            }
        }

        // Put syllable back in.
        for info in &mut buffer.info[start..end] {
            info.set_syllable(syllable);
        }
    }

    // Setup masks now

    {
        // Reph
        for info in &mut buffer.info[start..end] {
            if info.indic_position() != ot_position_t::POS_RA_TO_BECOME_REPH {
                break;
            }

            info.mask |= indic_plan.mask_array[indic_feature::RPHF];
        }

        // Pre-base
        let mut mask = indic_plan.mask_array[indic_feature::HALF];
        if !indic_plan.is_old_spec && indic_plan.config.blwf_mode == BlwfMode::PreAndPost {
            mask |= indic_plan.mask_array[indic_feature::BLWF];
        }

        for info in &mut buffer.info[start..base] {
            info.mask |= mask;
        }

        // Base
        mask = 0;
        if base < end {
            buffer.info[base].mask |= mask;
        }

        // Post-base
        mask = indic_plan.mask_array[indic_feature::BLWF]
            | indic_plan.mask_array[indic_feature::ABVF]
            | indic_plan.mask_array[indic_feature::PSTF];
        for i in base + 1..end {
            buffer.info[i].mask |= mask;
        }
    }

    if indic_plan.is_old_spec && buffer.script == Some(script::DEVANAGARI) {
        // Old-spec eye-lash Ra needs special handling.  From the
        // spec:
        //
        // "The feature 'below-base form' is applied to consonants
        // having below-base forms and following the base consonant.
        // The exception is vattu, which may appear below half forms
        // as well as below the base glyph. The feature 'below-base
        // form' will be applied to all such occurrences of Ra as well."
        //
        // Test case: U+0924,U+094D,U+0930,U+094d,U+0915
        // with Sanskrit 2003 font.
        //
        // However, note that Ra,Halant,ZWJ is the correct way to
        // request eyelash form of Ra, so we wouldbn't inhibit it
        // in that sequence.
        //
        // Test case: U+0924,U+094D,U+0930,U+094d,U+200D,U+0915
        for i in start..base.saturating_sub(1) {
            if buffer.info[i].indic_category() == ot_category_t::OT_Ra
                && buffer.info[i + 1].indic_category() == ot_category_t::OT_H
                && (i + 2 == base || buffer.info[i + 2].indic_category() != ot_category_t::OT_ZWJ)
            {
                buffer.info[i].mask |= indic_plan.mask_array[indic_feature::BLWF];
                buffer.info[i + 1].mask |= indic_plan.mask_array[indic_feature::BLWF];
            }
        }
    }

    let pref_len = 2;
    if indic_plan.mask_array[indic_feature::PREF] != 0 && base + pref_len < end {
        // Find a Halant,Ra sequence and mark it for pre-base-reordering processing.
        for i in base + 1..end - pref_len + 1 {
            let glyphs = &[buffer.info[i + 0].as_glyph(), buffer.info[i + 1].as_glyph()];
            if indic_plan.pref.would_substitute(&plan.ot_map, face, glyphs) {
                buffer.info[i + 0].mask |= indic_plan.mask_array[indic_feature::PREF];
                buffer.info[i + 1].mask |= indic_plan.mask_array[indic_feature::PREF];
                break;
            }
        }
    }

    // Apply ZWJ/ZWNJ effects
    for i in start + 1..end {
        if buffer.info[i].is_joiner() {
            let non_joiner = buffer.info[i].indic_category() == ot_category_t::OT_ZWNJ;
            let mut j = i;

            loop {
                j -= 1;

                // ZWJ/ZWNJ should disable CJCT.  They do that by simply
                // being there, since we don't skip them for the CJCT
                // feature (ie. F_MANUAL_ZWJ)

                // A ZWNJ disables HALF.
                if non_joiner {
                    buffer.info[j].mask &= !indic_plan.mask_array[indic_feature::HALF];
                }

                if j <= start || buffer.info[j].is_consonant() {
                    break;
                }
            }
        }
    }
}

fn initial_reordering_standalone_cluster(
    plan: &hb_ot_shape_plan_t,
    indic_plan: &IndicShapePlan,
    face: &hb_font_t,
    start: usize,
    end: usize,
    buffer: &mut hb_buffer_t,
) {
    // We treat placeholder/dotted-circle as if they are consonants, so we
    // should just chain.  Only if not in compatibility mode that is...
    initial_reordering_consonant_syllable(plan, indic_plan, face, start, end, buffer);
}

fn final_reordering(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) -> bool {
    if buffer.is_empty() {
        return false;
    }

    foreach_syllable!(buffer, start, end, {
        final_reordering_impl(plan, face, start, end, buffer);
    });

    buffer.deallocate_var(GlyphInfo::INDIC_CATEGORY_VAR);
    buffer.deallocate_var(GlyphInfo::INDIC_POSITION_VAR);

    false
}

fn final_reordering_impl(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    start: usize,
    end: usize,
    buffer: &mut hb_buffer_t,
) {
    let indic_plan = plan.data::<IndicShapePlan>();

    // This function relies heavily on halant glyphs.  Lots of ligation
    // and possibly multiple substitutions happened prior to this
    // phase, and that might have messed up our properties.  Recover
    // from a particular case of that where we're fairly sure that a
    // class of OT_H is desired but has been lost.
    //
    // We don't call load_virama_glyph(), since we know it's already loaded.
    let mut virama_glyph = None;
    if indic_plan.config.virama != 0 {
        if let Some(g) = face.get_nominal_glyph(indic_plan.config.virama) {
            virama_glyph = Some(g.to_u32());
        }
    }

    if let Some(virama_glyph) = virama_glyph {
        for info in &mut buffer.info[start..end] {
            if info.glyph_id == virama_glyph && info.ligated() && info.multiplied() {
                // This will make sure that this glyph passes is_halant() test.
                info.set_indic_category(ot_category_t::OT_H);
                info.clear_ligated_and_multiplied();
            }
        }
    }

    // 4. Final reordering:
    //
    // After the localized forms and basic shaping forms GSUB features have been
    // applied (see below), the shaping engine performs some final glyph
    // reordering before applying all the remaining font features to the entire
    // syllable.

    let mut try_pref = indic_plan.mask_array[indic_feature::PREF] != 0;

    let mut base = start;
    while base < end {
        if buffer.info[base].indic_position() as u32 >= ot_position_t::POS_BASE_C as u32 {
            if try_pref && base + 1 < end {
                for i in base + 1..end {
                    if (buffer.info[i].mask & indic_plan.mask_array[indic_feature::PREF]) != 0 {
                        if !(buffer.info[i].substituted()
                            && buffer.info[i].ligated_and_didnt_multiply())
                        {
                            // Ok, this was a 'pref' candidate but didn't form any.
                            // Base is around here...
                            base = i;
                            while base < end && buffer.info[base].is_halant() {
                                base += 1;
                            }

                            if base < end {
                                buffer.info[base].set_indic_position(ot_position_t::POS_BASE_C);
                            }

                            try_pref = false;
                        }

                        break;
                    }

                    if base == end {
                        break;
                    }
                }
            }

            // For Malayalam, skip over unformed below- (but NOT post-) forms.
            if buffer.script == Some(script::MALAYALAM) {
                let mut i = base + 1;
                while i < end {
                    while i < end && buffer.info[i].is_joiner() {
                        i += 1;
                    }

                    if i == end || !buffer.info[i].is_halant() {
                        break;
                    }

                    i += 1; // Skip halant.

                    while i < end && buffer.info[i].is_joiner() {
                        i += 1;
                    }

                    if i < end
                        && buffer.info[i].is_consonant()
                        && buffer.info[i].indic_position() == ot_position_t::POS_BELOW_C
                    {
                        base = i;
                        buffer.info[base].set_indic_position(ot_position_t::POS_BASE_C);
                    }

                    i += 1;
                }
            }

            if start < base
                && buffer.info[base].indic_position() as u32 > ot_position_t::POS_BASE_C as u32
            {
                base -= 1;
            }

            break;
        }

        base += 1;
    }

    if base == end
        && start < base
        && buffer.info[base - 1].is_one_of(rb_flag(ot_category_t::OT_ZWJ as u32))
    {
        base -= 1;
    }

    if base < end {
        while start < base
            && buffer.info[base].is_one_of(
                rb_flag(ot_category_t::OT_N as u32) | rb_flag(ot_category_t::OT_H as u32),
            )
        {
            base -= 1;
        }
    }

    // - Reorder matras:
    //
    //   If a pre-base matra character had been reordered before applying basic
    //   features, the glyph can be moved closer to the main consonant based on
    //   whether half-forms had been formed. Actual position for the matra is
    //   defined as “after last standalone halant glyph, after initial matra
    //   position and before the main consonant”. If ZWJ or ZWNJ follow this
    //   halant, position is moved after it.
    //
    // IMPLEMENTATION NOTES:
    //
    // It looks like the last sentence is wrong.  Testing, with Windows 7 Uniscribe
    // and Devanagari shows that the behavior is best described as:
    //
    // "If ZWJ follows this halant, matra is NOT repositioned after this halant.
    //  If ZWNJ follows this halant, position is moved after it."
    //
    // Test case, with Adobe Devanagari or Nirmala UI:
    //
    //   U+091F,U+094D,U+200C,U+092F,U+093F
    //   (Matra moves to the middle, after ZWNJ.)
    //
    //   U+091F,U+094D,U+200D,U+092F,U+093F
    //   (Matra does NOT move, stays to the left.)
    //
    // https://github.com/harfbuzz/harfbuzz/issues/1070

    // Otherwise there can't be any pre-base matra characters.
    if start + 1 < end && start < base {
        // If we lost track of base, alas, position before last thingy.
        let mut new_pos = if base == end { base - 2 } else { base - 1 };

        // Malayalam / Tamil do not have "half" forms or explicit virama forms.
        // The glyphs formed by 'half' are Chillus or ligated explicit viramas.
        // We want to position matra after them.
        if buffer.script != Some(script::MALAYALAM) && buffer.script != Some(script::TAMIL) {
            loop {
                while new_pos > start
                    && !buffer.info[new_pos].is_one_of(
                        rb_flag(ot_category_t::OT_M as u32)
                            | rb_flag(ot_category_t::OT_MPst as u32)
                            | rb_flag(ot_category_t::OT_H as u32),
                    )
                {
                    new_pos -= 1;
                }

                // If we found no Halant we are done.
                // Otherwise only proceed if the Halant does
                // not belong to the Matra itself!
                if buffer.info[new_pos].is_halant()
                    && buffer.info[new_pos].indic_position() != ot_position_t::POS_PRE_M
                {
                    if new_pos + 1 < end {
                        // -> If ZWJ follows this halant, matra is NOT repositioned after this halant.
                        if buffer.info[new_pos + 1].indic_category() == ot_category_t::OT_ZWJ {
                            // Keep searching.
                            if new_pos > start {
                                new_pos -= 1;
                                continue;
                            }
                        }

                        // -> If ZWNJ follows this halant, position is moved after it.
                        //
                        // IMPLEMENTATION NOTES:
                        //
                        // This is taken care of by the state-machine. A Halant,ZWNJ is a terminating
                        // sequence for a consonant syllable; any pre-base matras occurring after it
                        // will belong to the subsequent syllable.
                    }
                } else {
                    new_pos = start; // No move.
                }

                break;
            }
        }

        if start < new_pos && buffer.info[new_pos].indic_position() != ot_position_t::POS_PRE_M {
            // Now go see if there's actually any matras...
            for i in (start + 1..=new_pos).rev() {
                if buffer.info[i - 1].indic_position() == ot_position_t::POS_PRE_M {
                    let old_pos = i - 1;
                    // Shouldn't actually happen.
                    if old_pos < base && base <= new_pos {
                        base -= 1;
                    }

                    let tmp = buffer.info[old_pos];
                    for i in 0..new_pos - old_pos {
                        buffer.info[i + old_pos] = buffer.info[i + old_pos + 1];
                    }
                    buffer.info[new_pos] = tmp;

                    // Note: this merge_clusters() is intentionally *after* the reordering.
                    // Indic matra reordering is special and tricky...
                    buffer.merge_clusters(new_pos, cmp::min(end, base + 1));

                    new_pos -= 1;
                }
            }
        } else {
            for i in start..base {
                if buffer.info[i].indic_position() == ot_position_t::POS_PRE_M {
                    buffer.merge_clusters(i, cmp::min(end, base + 1));
                    break;
                }
            }
        }
    }

    // - Reorder reph:
    //
    //   Reph’s original position is always at the beginning of the syllable,
    //   (i.e. it is not reordered at the character reordering stage). However,
    //   it will be reordered according to the basic-forms shaping results.
    //   Possible positions for reph, depending on the script, are; after main,
    //   before post-base consonant forms, and after post-base consonant forms.

    // Two cases:
    //
    // - If repha is encoded as a sequence of characters (Ra,H or Ra,H,ZWJ), then
    //   we should only move it if the sequence ligated to the repha form.
    //
    // - If repha is encoded separately and in the logical position, we should only
    //   move it if it did NOT ligate.  If it ligated, it's probably the font trying
    //   to make it work without the reordering.

    if start + 1 < end
        && buffer.info[start].indic_position() == ot_position_t::POS_RA_TO_BECOME_REPH
        && (buffer.info[start].indic_category() == ot_category_t::OT_Repha)
            ^ buffer.info[start].ligated_and_didnt_multiply()
    {
        let mut new_reph_pos;
        'reph: {
            let reph_pos = indic_plan.config.reph_pos;

            // 1. If reph should be positioned after post-base consonant forms,
            //    proceed to step 5.
            if reph_pos != RephPosition::AfterPost {
                // 2. If the reph repositioning class is not after post-base: target
                //    position is after the first explicit halant glyph between the
                //    first post-reph consonant and last main consonant. If ZWJ or ZWNJ
                //    are following this halant, position is moved after it. If such
                //    position is found, this is the target position. Otherwise,
                //    proceed to the next step.
                //
                //    Note: in old-implementation fonts, where classifications were
                //    fixed in shaping engine, there was no case where reph position
                //    will be found on this step.
                {
                    new_reph_pos = start + 1;
                    while new_reph_pos < base && !buffer.info[new_reph_pos].is_halant() {
                        new_reph_pos += 1;
                    }

                    if new_reph_pos < base && buffer.info[new_reph_pos].is_halant() {
                        // ->If ZWJ or ZWNJ are following this halant, position is moved after it.
                        if new_reph_pos + 1 < base && buffer.info[new_reph_pos + 1].is_joiner() {
                            new_reph_pos += 1;
                        }

                        break 'reph;
                    }
                }

                // 3. If reph should be repositioned after the main consonant: find the
                //    first consonant not ligated with main, or find the first
                //    consonant that is not a potential pre-base-reordering Ra.
                if reph_pos == RephPosition::AfterMain {
                    new_reph_pos = base;
                    while new_reph_pos + 1 < end
                        && buffer.info[new_reph_pos + 1].indic_position()
                            <= ot_position_t::POS_AFTER_MAIN
                    {
                        new_reph_pos += 1;
                    }

                    if new_reph_pos < end {
                        break 'reph;
                    }
                }

                // 4. If reph should be positioned before post-base consonant, find
                //    first post-base classified consonant not ligated with main. If no
                //    consonant is found, the target position should be before the
                //    first matra, syllable modifier sign or vedic sign.
                //
                // This is our take on what step 4 is trying to say (and failing, BADLY).
                if reph_pos == RephPosition::AfterSub {
                    new_reph_pos = base;
                    while new_reph_pos + 1 < end
                        && (rb_flag_unsafe(buffer.info[new_reph_pos + 1].indic_position() as u32)
                            & (rb_flag(ot_position_t::POS_POST_C as u32)
                                | rb_flag(ot_position_t::POS_AFTER_POST as u32)
                                | rb_flag(ot_position_t::POS_SMVD as u32)))
                            == 0
                    {
                        new_reph_pos += 1;
                    }

                    if new_reph_pos < end {
                        break 'reph;
                    }
                }
            }

            // 5. If no consonant is found in steps 3 or 4, move reph to a position
            //    immediately before the first post-base matra, syllable modifier
            //    sign or vedic sign that has a reordering class after the intended
            //    reph position. For example, if the reordering position for reph
            //    is post-main, it will skip above-base matras that also have a
            //    post-main position.
            //
            // Copied from step 2.
            new_reph_pos = start + 1;
            while new_reph_pos < base && !buffer.info[new_reph_pos].is_halant() {
                new_reph_pos += 1;
            }

            if new_reph_pos < base && buffer.info[new_reph_pos].is_halant() {
                /* ->If ZWJ or ZWNJ are following this halant, position is moved after it. */
                if new_reph_pos + 1 < base && buffer.info[new_reph_pos + 1].is_joiner() {
                    new_reph_pos += 1;
                }

                break 'reph;
            }
            // See https://github.com/harfbuzz/harfbuzz/issues/2298#issuecomment-615318654

            // 6. Otherwise, reorder reph to the end of the syllable.
            {
                new_reph_pos = end - 1;
                while new_reph_pos > start
                    && buffer.info[new_reph_pos].indic_position() == ot_position_t::POS_SMVD
                {
                    new_reph_pos -= 1;
                }

                // If the Reph is to be ending up after a Matra,Halant sequence,
                // position it before that Halant so it can interact with the Matra.
                // However, if it's a plain Consonant,Halant we shouldn't do that.
                // Uniscribe doesn't do this.
                // TEST: U+0930,U+094D,U+0915,U+094B,U+094D
                if buffer.info[new_reph_pos].is_halant() {
                    for info in &buffer.info[base + 1..new_reph_pos] {
                        if (rb_flag_unsafe(info.indic_category() as u32)
                            & (rb_flag(ot_category_t::OT_M as u32)
                                | rb_flag(ot_category_t::OT_MPst as u32)))
                            != 0
                        {
                            // Ok, got it.
                            new_reph_pos -= 1;
                        }
                    }
                }
            }

            break 'reph;
        }

        // Move
        buffer.merge_clusters(start, new_reph_pos + 1);

        let reph = buffer.info[start];
        for i in 0..new_reph_pos - start {
            buffer.info[i + start] = buffer.info[i + start + 1];
        }
        buffer.info[new_reph_pos] = reph;

        if start < base && base <= new_reph_pos {
            base -= 1;
        }
    }

    // - Reorder pre-base-reordering consonants:
    //
    //   If a pre-base-reordering consonant is found, reorder it according to
    //   the following rules:

    // Otherwise there can't be any pre-base-reordering Ra.
    if try_pref && base + 1 < end {
        for i in base + 1..end {
            if (buffer.info[i].mask & indic_plan.mask_array[indic_feature::PREF]) != 0 {
                // 1. Only reorder a glyph produced by substitution during application
                //    of the <pref> feature. (Note that a font may shape a Ra consonant with
                //    the feature generally but block it in certain contexts.)
                //
                // Note: We just check that something got substituted.  We don't check that
                // the <pref> feature actually did it...
                //
                // Reorder pref only if it ligated.
                if buffer.info[i].ligated_and_didnt_multiply() {
                    // 2. Try to find a target position the same way as for pre-base matra.
                    //    If it is found, reorder pre-base consonant glyph.
                    //
                    // 3. If position is not found, reorder immediately before main consonant.

                    let mut new_pos = base;
                    // Malayalam / Tamil do not have "half" forms or explicit virama forms.
                    // The glyphs formed by 'half' are Chillus or ligated explicit viramas.
                    // We want to position matra after them.
                    if buffer.script != Some(script::MALAYALAM)
                        && buffer.script != Some(script::TAMIL)
                    {
                        while new_pos > start
                            && !buffer.info[new_pos - 1].is_one_of(
                                rb_flag(ot_category_t::OT_M as u32)
                                    | rb_flag(ot_category_t::OT_MPst as u32)
                                    | rb_flag(ot_category_t::OT_H as u32),
                            )
                        {
                            new_pos -= 1;
                        }
                    }

                    if new_pos > start && buffer.info[new_pos - 1].is_halant() {
                        // -> If ZWJ or ZWNJ follow this halant, position is moved after it.
                        if new_pos < end && buffer.info[new_pos].is_joiner() {
                            new_pos += 1;
                        }
                    }

                    {
                        let old_pos = i;

                        buffer.merge_clusters(new_pos, old_pos + 1);
                        let tmp = buffer.info[old_pos];
                        for i in (0..old_pos - new_pos).rev() {
                            buffer.info[i + new_pos + 1] = buffer.info[i + new_pos];
                        }
                        buffer.info[new_pos] = tmp;

                        if new_pos <= base && base < old_pos {
                            // TODO: investigate
                            #[allow(unused_assignments)]
                            {
                                base += 1;
                            }
                        }
                    }
                }

                break;
            }
        }
    }

    // Apply 'init' to the Left Matra if it's a word start.
    if buffer.info[start].indic_position() == ot_position_t::POS_PRE_M {
        if start == 0
            || (rb_flag_unsafe(buffer.info[start - 1].general_category().to_u8() as u32)
                & rb_flag_range(
                    hb_gc::HB_UNICODE_GENERAL_CATEGORY_FORMAT,
                    hb_gc::HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK,
                ))
                == 0
        {
            buffer.info[start].mask |= indic_plan.mask_array[indic_feature::INIT];
        } else {
            buffer.unsafe_to_break(Some(start - 1), Some(start + 1));
        }
    }
}
