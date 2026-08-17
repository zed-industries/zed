use alloc::boxed::Box;
use core::any::Any;

use crate::hb::unicode::Codepoint;

use super::buffer::*;
use super::common::TagExt;
use super::ot_shape::*;
use super::ot_shape_normalize::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::{hb_font_t, hb_tag_t, script, Direction, Script};

impl GlyphInfo {
    declare_buffer_var!(
        u8,
        2,
        2,
        OT_SHAPER_VAR_U8_CATEGORY_VAR,
        ot_shaper_var_u8_category,
        set_ot_shaper_var_u8_category
    );
    declare_buffer_var!(
        u8,
        2,
        3,
        OT_SHAPER_VAR_U8_AUXILIARY_VAR,
        ot_shaper_var_u8_auxiliary,
        set_ot_shaper_var_u8_auxiliary
    );
}

pub const MAX_COMBINING_MARKS: usize = 32;

pub type hb_ot_shape_zero_width_marks_type_t = u32;
pub const HB_OT_SHAPE_ZERO_WIDTH_MARKS_NONE: u32 = 0;
pub const HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_EARLY: u32 = 1;
pub const HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_LATE: u32 = 2;

pub type DecomposeFn =
    fn(&hb_ot_shape_normalize_context_t, Codepoint) -> Option<(Codepoint, Codepoint)>;
pub type ComposeFn =
    fn(&hb_ot_shape_normalize_context_t, Codepoint, Codepoint) -> Option<Codepoint>;

pub const DEFAULT_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: None,
    override_features: None,
    create_data: None,
    preprocess_text: None,
    postprocess_glyphs: None,
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_AUTO,
    decompose: None,
    compose: None,
    setup_masks: None,
    gpos_tag: None,
    reorder_marks: None,
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_LATE,
    fallback_position: true,
};

pub struct hb_ot_shaper_t {
    /// Called during `shape_plan()`.
    /// Shapers should use plan.map to add their features and callbacks.
    pub collect_features: Option<fn(&mut hb_ot_shape_planner_t)>,

    /// Called during `shape_plan()`.
    /// Shapers should use plan.map to override features and add callbacks after
    /// common features are added.
    pub override_features: Option<fn(&mut hb_ot_shape_planner_t)>,

    /// Called at the end of `shape_plan()`.
    /// Whatever shapers return will be accessible through `plan.data()` later.
    pub create_data: Option<fn(&hb_ot_shape_plan_t) -> Box<dyn Any + Send + Sync>>,

    /// Called during `shape()`.
    /// Shapers can use to modify text before shaping starts.
    pub preprocess_text: Option<fn(&hb_ot_shape_plan_t, &hb_font_t, &mut hb_buffer_t)>,

    /// Called during `shape()`.
    /// Shapers can use to modify text before shaping starts.
    pub postprocess_glyphs: Option<fn(&hb_ot_shape_plan_t, &hb_font_t, &mut hb_buffer_t)>,

    /// How to normalize.
    pub normalization_preference: hb_ot_shape_normalization_mode_t,

    /// Called during `shape()`'s normalization.
    pub decompose: Option<DecomposeFn>,

    /// Called during `shape()`'s normalization.
    pub compose: Option<ComposeFn>,

    /// Called during `shape()`.
    /// Shapers should use map to get feature masks and set on buffer.
    /// Shapers may NOT modify characters.
    pub setup_masks: Option<fn(&hb_ot_shape_plan_t, &hb_font_t, &mut hb_buffer_t)>,

    /// If not `None`, then must match found GPOS script tag for
    /// GPOS to be applied.  Otherwise, fallback positioning will be used.
    pub gpos_tag: Option<hb_tag_t>,

    /// Called during `shape()`.
    /// Shapers can use to modify ordering of combining marks.
    pub reorder_marks: Option<fn(&hb_ot_shape_plan_t, &mut hb_buffer_t, usize, usize)>,

    /// If and when to zero-width marks.
    pub zero_width_marks: hb_ot_shape_zero_width_marks_type_t,

    /// Whether to use fallback mark positioning.
    pub fallback_position: bool,
}

// Same as default but no mark advance zeroing / fallback positioning.
// Dumbest shaper ever, basically.
pub const DUMBER_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: None,
    override_features: None,
    create_data: None,
    preprocess_text: None,
    postprocess_glyphs: None,
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_AUTO,
    decompose: None,
    compose: None,
    setup_masks: None,
    gpos_tag: None,
    reorder_marks: None,
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_NONE,
    fallback_position: false,
};

pub fn hb_ot_shape_complex_categorize(
    script: Script,
    direction: Direction,
    gsub_script: Option<hb_tag_t>,
) -> &'static hb_ot_shaper_t {
    match script {
        // Unicode-1.1 additions
        script::ARABIC

        // Unicode-3.0 additions
        | script::SYRIAC => {
            // For Arabic script, use the Arabic shaper even if no OT script tag was found.
            // This is because we do fallback shaping for Arabic script (and not others).
            // But note that Arabic shaping is applicable only to horizontal layout; for
            // vertical text, just use the generic shaper instead.
            //
            // TODO: Does this still apply? Arabic fallback shaping was removed.
            if (gsub_script != Some(hb_tag_t::default_script()) || script == script::ARABIC)
                && direction.is_horizontal()
            {
                &crate::hb::ot_shaper_arabic::ARABIC_SHAPER
            } else {
                &DEFAULT_SHAPER
            }
        }

        // Unicode-1.1 additions
        script::THAI
        | script::LAO => &crate::hb::ot_shaper_thai::THAI_SHAPER,

        // Unicode-1.1 additions
        script::HANGUL => &crate::hb::ot_shaper_hangul::HANGUL_SHAPER,

        // Unicode-1.1 additions
        script::HEBREW => &crate::hb::ot_shaper_hebrew::HEBREW_SHAPER,

        // Unicode-1.1 additions
        script::BENGALI
        | script::DEVANAGARI
        | script::GUJARATI
        | script::GURMUKHI
        | script::KANNADA
        | script::MALAYALAM
        | script::ORIYA
        | script::TAMIL
        | script::TELUGU => {
            // If the designer designed the font for the 'DFLT' script,
            // (or we ended up arbitrarily pick 'latn'), use the default shaper.
            // Otherwise, use the specific shaper.
            //
            // If it's indy3 tag, send to USE.
            if gsub_script == Some(hb_tag_t::default_script()) ||
               gsub_script == Some(hb_tag_t::new(b"latn")) {
                &DEFAULT_SHAPER
            } else if gsub_script.is_some_and(|tag| tag.to_be_bytes()[3] == b'3') {
                &crate::hb::ot_shaper_use::UNIVERSAL_SHAPER
            } else {
                &crate::hb::ot_shaper_indic::INDIC_SHAPER
            }
        }

        script::KHMER => &crate::hb::ot_shaper_khmer::KHMER_SHAPER,

        script::MYANMAR => {
            // If the designer designed the font for the 'DFLT' script,
            // (or we ended up arbitrarily pick 'latn'), use the default shaper.
            // Otherwise, use the specific shaper.
            //
            // If designer designed for 'mymr' tag, also send to default
            // shaper.  That's tag used from before Myanmar shaping spec
            // was developed.  The shaping spec uses 'mym2' tag.
            if gsub_script == Some(hb_tag_t::default_script()) ||
               gsub_script == Some(hb_tag_t::new(b"latn")) ||
               gsub_script == Some(hb_tag_t::new(b"mymr"))
            {
                &DEFAULT_SHAPER
            } else {
                &crate::hb::ot_shaper_myanmar::MYANMAR_SHAPER
            }
        }

        // https://github.com/harfbuzz/harfbuzz/issues/1162
        script::MYANMAR_ZAWGYI => &crate::hb::ot_shaper_myanmar::MYANMAR_ZAWGYI_SHAPER,

        // Unicode-2.0 additions
        script::TIBETAN

        // Unicode-3.0 additions
        | script::MONGOLIAN
        | script::SINHALA

        // Unicode-3.2 additions
        | script::BUHID
        | script::HANUNOO
        | script::TAGALOG
        | script::TAGBANWA

        // Unicode-4.0 additions
        | script::LIMBU
        | script::TAI_LE

        // Unicode-4.1 additions
        | script::BUGINESE
        | script::KHAROSHTHI
        | script::SYLOTI_NAGRI
        | script::TIFINAGH

        // Unicode-5.0 additions
        | script::BALINESE
        | script::NKO
        | script::PHAGS_PA

        // Unicode-5.1 additions
        | script::CHAM
        | script::KAYAH_LI
        | script::LEPCHA
        | script::REJANG
        | script::SAURASHTRA
        | script::SUNDANESE

        // Unicode-5.2 additions
        | script::EGYPTIAN_HIEROGLYPHS
        | script::JAVANESE
        | script::KAITHI
        | script::MEETEI_MAYEK
        | script::TAI_THAM
        | script::TAI_VIET

        // Unicode-6.0 additions
        | script::BATAK
        | script::BRAHMI
        | script::MANDAIC

        // Unicode-6.1 additions
        | script::CHAKMA
        | script::MIAO
        | script::SHARADA
        | script::TAKRI

        // Unicode-7.0 additions
        | script::DUPLOYAN
        | script::GRANTHA
        | script::KHOJKI
        | script::KHUDAWADI
        | script::MAHAJANI
        | script::MANICHAEAN
        | script::MODI
        | script::PAHAWH_HMONG
        | script::PSALTER_PAHLAVI
        | script::SIDDHAM
        | script::TIRHUTA

        // Unicode-8.0 additions
        | script::AHOM
        | script::MULTANI

        // Unicode-9.0 additions
        | script::ADLAM
        | script::BHAIKSUKI
        | script::MARCHEN
        | script::NEWA

        // Unicode-10.0 additions
        | script::MASARAM_GONDI
        | script::SOYOMBO
        | script::ZANABAZAR_SQUARE

        // Unicode-11.0 additions
        | script::DOGRA
        | script::GUNJALA_GONDI
        | script::HANIFI_ROHINGYA
        | script::MAKASAR
        | script::MEDEFAIDRIN
        | script::OLD_SOGDIAN
        | script::SOGDIAN

        // Unicode-12.0 additions
        | script::ELYMAIC
        | script::NANDINAGARI
        | script::NYIAKENG_PUACHUE_HMONG
        | script::WANCHO

        // Unicode-13.0 additions
        | script::CHORASMIAN
        | script::DIVES_AKURU
        | script::KHITAN_SMALL_SCRIPT
        | script::YEZIDI

        // Unicode-14.0 additions
        | script::CYPRO_MINOAN
        | script::OLD_UYGHUR
        | script::TANGSA
        | script::TOTO
        | script::VITHKUQI

        // Unicode-15.0 additions
        | script::KAWI
        | script::NAG_MUNDARI

        // Unicode-16.0 additions
        | script::GARAY
        | script::GURUNG_KHEMA
        | script::KIRAT_RAI
        | script::OL_ONAL
        | script::SUNUWAR
        | script::TODHRI
        | script::TULU_TIGALARI

        // Unicode-17.0 additions
        | script::BERIA_ERFE
        | script::SIDETIC
        | script::TAI_YO
        | script::TOLONG_SIKI

        => {
            // If the designer designed the font for the 'DFLT' script,
            // (or we ended up arbitrarily pick 'latn'), use the default shaper.
            // Otherwise, use the specific shaper.
            // Note that for some simple scripts, there may not be *any*
            // GSUB/GPOS needed, so there may be no scripts found!
            if gsub_script == Some(hb_tag_t::default_script()) ||
               gsub_script == Some(hb_tag_t::new(b"latn")) {
                &DEFAULT_SHAPER
            } else {
                &crate::hb::ot_shaper_use::UNIVERSAL_SHAPER
            }
        }

        _ => &DEFAULT_SHAPER
    }
}
