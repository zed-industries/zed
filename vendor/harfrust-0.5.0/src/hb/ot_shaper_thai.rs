use super::buffer::*;
use super::ot_layout::*;
use super::ot_shape_normalize::HB_OT_SHAPE_NORMALIZATION_MODE_AUTO;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::ot_shaper::*;
use super::unicode::GeneralCategory;
use super::{hb_font_t, script};

pub const THAI_SHAPER: hb_ot_shaper_t = hb_ot_shaper_t {
    collect_features: None,
    override_features: None,
    create_data: None,
    preprocess_text: Some(preprocess_text),
    postprocess_glyphs: None,
    normalization_preference: HB_OT_SHAPE_NORMALIZATION_MODE_AUTO,
    decompose: None,
    compose: None,
    setup_masks: None,
    gpos_tag: None,
    reorder_marks: None,
    zero_width_marks: HB_OT_SHAPE_ZERO_WIDTH_MARKS_BY_GDEF_LATE,
    fallback_position: false,
};

#[derive(Clone, Copy, PartialEq)]
enum Consonant {
    NC = 0,
    AC,
    RC,
    DC,
    NotConsonant,
}

fn get_consonant_type(u: u32) -> Consonant {
    match u {
        0x0E1B | 0x0E1D | 0x0E1F => Consonant::AC,
        0x0E0D | 0x0E10 => Consonant::RC,
        0x0E0E | 0x0E0F => Consonant::DC,
        0x0E01..=0x0E2E => Consonant::NC,
        _ => Consonant::NotConsonant,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mark {
    AV,
    BV,
    T,
    NotMark,
}

fn get_mark_type(u: u32) -> Mark {
    match u {
        0x0E31 | 0x0E34..=0x0E37 | 0x0E47 | 0x0E4D..=0x0E4E => Mark::AV,
        0x0E38..=0x0E3A => Mark::BV,
        0x0E48..=0x0E4C => Mark::T,
        _ => Mark::NotMark,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Action {
    NOP,
    /// Shift combining-mark down.
    SD,
    /// Shift combining-mark left.
    SL,
    /// Shift combining-mark down-left.
    SDL,
    /// Remove descender from base.
    RD,
}

#[derive(Clone, Copy)]
struct PuaMapping {
    u: u16,
    win_pua: u16,
    mac_pua: u16,
}

impl PuaMapping {
    const fn new(u: u16, win_pua: u16, mac_pua: u16) -> Self {
        PuaMapping {
            u,
            win_pua,
            mac_pua,
        }
    }
}

static SD_MAPPINGS: &[PuaMapping] = &[
    PuaMapping::new(0x0E48, 0xF70A, 0xF88B), // MAI EK
    PuaMapping::new(0x0E49, 0xF70B, 0xF88E), // MAI THO
    PuaMapping::new(0x0E4A, 0xF70C, 0xF891), // MAI TRI
    PuaMapping::new(0x0E4B, 0xF70D, 0xF894), // MAI CHATTAWA
    PuaMapping::new(0x0E4C, 0xF70E, 0xF897), // THANTHAKHAT
    PuaMapping::new(0x0E38, 0xF718, 0xF89B), // SARA U
    PuaMapping::new(0x0E39, 0xF719, 0xF89C), // SARA UU
    PuaMapping::new(0x0E3A, 0xF71A, 0xF89D), // PHINTHU
    PuaMapping::new(0x0000, 0x0000, 0x0000),
];

static SDL_MAPPINGS: &[PuaMapping] = &[
    PuaMapping::new(0x0E48, 0xF705, 0xF88C), // MAI EK
    PuaMapping::new(0x0E49, 0xF706, 0xF88F), // MAI THO
    PuaMapping::new(0x0E4A, 0xF707, 0xF892), // MAI TRI
    PuaMapping::new(0x0E4B, 0xF708, 0xF895), // MAI CHATTAWA
    PuaMapping::new(0x0E4C, 0xF709, 0xF898), // THANTHAKHAT
    PuaMapping::new(0x0000, 0x0000, 0x0000),
];

static SL_MAPPINGS: &[PuaMapping] = &[
    PuaMapping::new(0x0E48, 0xF713, 0xF88A), // MAI EK
    PuaMapping::new(0x0E49, 0xF714, 0xF88D), // MAI THO
    PuaMapping::new(0x0E4A, 0xF715, 0xF890), // MAI TRI
    PuaMapping::new(0x0E4B, 0xF716, 0xF893), // MAI CHATTAWA
    PuaMapping::new(0x0E4C, 0xF717, 0xF896), // THANTHAKHAT
    PuaMapping::new(0x0E31, 0xF710, 0xF884), // MAI HAN-AKAT
    PuaMapping::new(0x0E34, 0xF701, 0xF885), // SARA I
    PuaMapping::new(0x0E35, 0xF702, 0xF886), // SARA II
    PuaMapping::new(0x0E36, 0xF703, 0xF887), // SARA UE
    PuaMapping::new(0x0E37, 0xF704, 0xF888), // SARA UEE
    PuaMapping::new(0x0E47, 0xF712, 0xF889), // MAITAIKHU
    PuaMapping::new(0x0E4D, 0xF711, 0xF899), // NIKHAHIT
    PuaMapping::new(0x0000, 0x0000, 0x0000),
];

static RD_MAPPINGS: &[PuaMapping] = &[
    PuaMapping::new(0x0E0D, 0xF70F, 0xF89A), // YO YING
    PuaMapping::new(0x0E10, 0xF700, 0xF89E), // THO THAN
    PuaMapping::new(0x0000, 0x0000, 0x0000),
];

fn pua_shape(u: u32, action: Action, face: &hb_font_t) -> u32 {
    let mappings = match action {
        Action::NOP => return u,
        Action::SD => SD_MAPPINGS,
        Action::SL => SL_MAPPINGS,
        Action::SDL => SDL_MAPPINGS,
        Action::RD => RD_MAPPINGS,
    };

    for m in mappings {
        if m.u as u32 == u {
            if face.get_nominal_glyph(m.win_pua as u32).is_some() {
                return m.win_pua as u32;
            }

            if face.get_nominal_glyph(m.mac_pua as u32).is_some() {
                return m.mac_pua as u32;
            }

            break;
        }
    }

    u
}

#[derive(Clone, Copy)]
enum AboveState {
    // Cluster above looks like:
    T0, //  ⣤
    T1, //     ⣼
    T2, //        ⣾
    T3, //           ⣿
}

static ABOVE_START_STATE: &[AboveState] = &[
    AboveState::T0, // NC
    AboveState::T1, // AC
    AboveState::T0, // RC
    AboveState::T0, // DC
    AboveState::T3, // NotConsonant
];

#[derive(Clone, Copy)]
struct AboveStateMachineEdge {
    action: Action,
    next_state: AboveState,
}

impl AboveStateMachineEdge {
    const fn new(action: Action, next_state: AboveState) -> Self {
        AboveStateMachineEdge { action, next_state }
    }
}

type ASME = AboveStateMachineEdge;

static ABOVE_STATE_MACHINE: &[[ASME; 3]] = &[
    //        AV                                      BV                                      T
    /* T0 */
    [
        ASME::new(Action::NOP, AboveState::T3),
        ASME::new(Action::NOP, AboveState::T0),
        ASME::new(Action::SD, AboveState::T3),
    ],
    /* T1 */
    [
        ASME::new(Action::SL, AboveState::T2),
        ASME::new(Action::NOP, AboveState::T1),
        ASME::new(Action::SDL, AboveState::T2),
    ],
    /* T2 */
    [
        ASME::new(Action::NOP, AboveState::T3),
        ASME::new(Action::NOP, AboveState::T2),
        ASME::new(Action::SL, AboveState::T3),
    ],
    /* T3 */
    [
        ASME::new(Action::NOP, AboveState::T3),
        ASME::new(Action::NOP, AboveState::T3),
        ASME::new(Action::NOP, AboveState::T3),
    ],
];

#[derive(Clone, Copy)]
enum BelowState {
    /// No descender.
    B0,
    /// Removable descender.
    B1,
    /// Strict descender.
    B2,
}

static BELOW_START_STATE: &[BelowState] = &[
    BelowState::B0, // NC
    BelowState::B0, // AC
    BelowState::B1, // RC
    BelowState::B2, // DC
    BelowState::B2, // NotConsonant
];

#[derive(Clone, Copy)]
struct BelowStateMachineEdge {
    action: Action,
    next_state: BelowState,
}

impl BelowStateMachineEdge {
    const fn new(action: Action, next_state: BelowState) -> Self {
        BelowStateMachineEdge { action, next_state }
    }
}

type BSME = BelowStateMachineEdge;

static BELOW_STATE_MACHINE: &[[BSME; 3]] = &[
    //        AV                                      BV                                      T
    /* B0 */
    [
        BSME::new(Action::NOP, BelowState::B0),
        BSME::new(Action::NOP, BelowState::B2),
        BSME::new(Action::NOP, BelowState::B0),
    ],
    /* B1 */
    [
        BSME::new(Action::NOP, BelowState::B1),
        BSME::new(Action::RD, BelowState::B2),
        BSME::new(Action::NOP, BelowState::B1),
    ],
    /* B2 */
    [
        BSME::new(Action::NOP, BelowState::B2),
        BSME::new(Action::SD, BelowState::B2),
        BSME::new(Action::NOP, BelowState::B2),
    ],
];

fn do_pua_shaping(face: &hb_font_t, buffer: &mut hb_buffer_t) {
    let mut above_state = ABOVE_START_STATE[Consonant::NotConsonant as usize];
    let mut below_state = BELOW_START_STATE[Consonant::NotConsonant as usize];
    let mut base = 0;

    for i in 0..buffer.len {
        let mt = get_mark_type(buffer.info[i].glyph_id);

        if mt == Mark::NotMark {
            let ct = get_consonant_type(buffer.info[i].glyph_id);
            above_state = ABOVE_START_STATE[ct as usize];
            below_state = BELOW_START_STATE[ct as usize];
            base = i;
            continue;
        }

        let above_edge = ABOVE_STATE_MACHINE[above_state as usize][mt as usize];
        let below_edge = BELOW_STATE_MACHINE[below_state as usize][mt as usize];
        above_state = above_edge.next_state;
        below_state = below_edge.next_state;

        // At least one of the above/below actions is NOP.
        let action = if above_edge.action != Action::NOP {
            above_edge.action
        } else {
            below_edge.action
        };

        buffer.unsafe_to_break(Some(base), Some(i));
        if action == Action::RD {
            buffer.info[base].glyph_id = pua_shape(buffer.info[base].glyph_id, action, face);
        } else {
            buffer.info[i].glyph_id = pua_shape(buffer.info[i].glyph_id, action, face);
        }
    }
}

// TODO: more tests
fn preprocess_text(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    // This function implements the shaping logic documented here:
    //
    //   https://linux.thai.net/~thep/th-otf/shaping.html
    //
    // The first shaping rule listed there is needed even if the font has Thai
    // OpenType tables.  The rest do fallback positioning based on PUA codepoints.
    // We implement that only if there exist no Thai GSUB in the font.

    // The following is NOT specified in the MS OT Thai spec, however, it seems
    // to be what Uniscribe and other engines implement.  According to Eric Muller:
    //
    // When you have a SARA AM, decompose it in NIKHAHIT + SARA AA, *and* move the
    // NIKHAHIT backwards over any above-base marks (0E31, 0E34-0E37, 0E47-0E4E).
    //
    // <0E14, 0E4B, 0E33> -> <0E14, 0E4D, 0E4B, 0E32>
    //
    // This reordering is legit only when the NIKHAHIT comes from a SARA AM, not
    // when it's there to start with. The string <0E14, 0E4B, 0E4D> is probably
    // not what a user wanted, but the rendering is nevertheless nikhahit above
    // chattawa.
    //
    // Same for Lao.
    //
    // Note:
    //
    // Uniscribe also does some below-marks reordering.  Namely, it positions U+0E3A
    // after U+0E38 and U+0E39.  We do that by modifying the ccc for U+0E3A.
    // See unicode->modified_combining_class ().  Lao does NOT have a U+0E3A
    // equivalent.

    // Here are the characters of significance:
    //
    //              Thai    Lao
    // SARA AM:     U+0E33  U+0EB3
    // SARA AA:     U+0E32  U+0EB2
    // Nikhahit:    U+0E4D  U+0ECD
    //
    // Testing shows that Uniscribe reorder the following marks:
    // Thai:	<0E31,0E34..0E37,0E47..0E4E>
    // Lao:     <0EB1,0EB4..0EB7,0EBB,0EC8..0ECD>
    //
    // Note how the Lao versions are the same as Thai + 0x80.

    // We only get one script at a time, so a script-agnostic implementation
    // is adequate here.
    #[inline]
    fn is_sara_am(u: u32) -> bool {
        (u & !0x0080) == 0x0E33
    }
    #[inline]
    fn nikhahit_from_sara_am(u: u32) -> u32 {
        u - 0x0E33 + 0x0E4D
    }
    #[inline]
    fn sara_aa_from_sara_am(u: u32) -> u32 {
        u - 1
    }
    #[inline]
    fn is_above_base_mark(u: u32) -> bool {
        let u = u & !0x0080;
        matches!(u, 0x0E34..=0x0E37 | 0x0E47..=0x0E4E | 0x0E31..=0x0E31 | 0x0E3B..=0x0E3B)
    }

    buffer.clear_output();
    buffer.idx = 0;
    while buffer.idx < buffer.len {
        let u = buffer.cur(0).glyph_id;
        if !is_sara_am(u) {
            buffer.next_glyph();
            continue;
        }

        // Is SARA AM. Decompose and reorder.
        buffer.output_glyph(nikhahit_from_sara_am(u));
        {
            let out_idx = buffer.out_len - 1;
            let mut info = buffer.out_info_mut()[out_idx];
            info.set_continuation(&mut buffer.scratch_flags);
        }
        buffer.replace_glyph(sara_aa_from_sara_am(u));

        // Make Nikhahit be recognized as a ccc=0 mark when zeroing widths.
        let end = buffer.out_len;

        buffer.out_info_mut()[end - 2].set_general_category(GeneralCategory::NON_SPACING_MARK);

        // Ok, let's see...
        let mut start = end - 2;
        while start > 0 && is_above_base_mark(buffer.out_info()[start - 1].glyph_id) {
            start -= 1;
        }

        if start + 2 < end {
            // Move Nikhahit (end-2) to the beginning
            buffer.merge_out_clusters(start, end);
            let t = buffer.out_info()[end - 2];
            for i in 0..(end - start - 2) {
                buffer.out_info_mut()[i + start + 1] = buffer.out_info()[i + start];
            }
            buffer.out_info_mut()[start] = t;
        } else {
            // Since we decomposed, and NIKHAHIT is combining, merge clusters with the
            // previous cluster.
            if start != 0 {
                buffer.merge_out_clusters(start - 1, end);
            }
        }
    }

    buffer.sync();

    // If font has Thai GSUB, we are done.
    if plan.script == Some(script::THAI) && !plan.ot_map.found_script(TableIndex::GSUB) {
        do_pua_shaping(face, buffer);
    }
}
