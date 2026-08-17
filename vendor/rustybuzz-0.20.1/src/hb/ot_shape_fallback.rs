use ttf_parser::GlyphId;

use super::buffer::{hb_buffer_t, GlyphPosition};
use super::face::hb_glyph_extents_t;
use super::ot_layout::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::unicode::*;
use super::{hb_font_t, Direction};

fn recategorize_combining_class(u: u32, mut class: u8) -> u8 {
    use modified_combining_class as mcc;
    use CanonicalCombiningClass as Class;

    if class >= 200 {
        return class;
    }

    // Thai / Lao need some per-character work.
    if u & !0xFF == 0x0E00 {
        if class == 0 {
            match u {
                0x0E31 | 0x0E34 | 0x0E35 | 0x0E36 | 0x0E37 | 0x0E47 | 0x0E4C | 0x0E4D | 0x0E4E => {
                    class = Class::AboveRight as u8
                }

                0x0EB1 | 0x0EB4 | 0x0EB5 | 0x0EB6 | 0x0EB7 | 0x0EBB | 0x0ECC | 0x0ECD => {
                    class = Class::Above as u8
                }

                0x0EBC => class = Class::Below as u8,

                _ => {}
            }
        } else {
            // Thai virama is below-right
            if u == 0x0E3A {
                class = Class::BelowRight as u8;
            }
        }
    }

    match class {
        // Hebrew
        mcc::CCC10 => Class::Below as u8,         // sheva
        mcc::CCC11 => Class::Below as u8,         // hataf segol
        mcc::CCC12 => Class::Below as u8,         // hataf patah
        mcc::CCC13 => Class::Below as u8,         // hataf qamats
        mcc::CCC14 => Class::Below as u8,         // hiriq
        mcc::CCC15 => Class::Below as u8,         // tsere
        mcc::CCC16 => Class::Below as u8,         // segol
        mcc::CCC17 => Class::Below as u8,         // patah
        mcc::CCC18 => Class::Below as u8,         // qamats & qamats qatan
        mcc::CCC20 => Class::Below as u8,         // qubuts
        mcc::CCC22 => Class::Below as u8,         // meteg
        mcc::CCC23 => Class::AttachedAbove as u8, // rafe
        mcc::CCC24 => Class::AboveRight as u8,    // shin dot
        mcc::CCC25 => Class::AboveLeft as u8,     // sin dot
        mcc::CCC19 => Class::AboveLeft as u8,     // holam & holam haser for vav
        mcc::CCC26 => Class::Above as u8,         // point varika
        mcc::CCC21 => class,                      // dagesh

        // Arabic and Syriac
        mcc::CCC27 => Class::Above as u8, // fathatan
        mcc::CCC28 => Class::Above as u8, // dammatan
        mcc::CCC30 => Class::Above as u8, // fatha
        mcc::CCC31 => Class::Above as u8, // damma
        mcc::CCC33 => Class::Above as u8, // shadda
        mcc::CCC34 => Class::Above as u8, // sukun
        mcc::CCC35 => Class::Above as u8, // superscript alef
        mcc::CCC36 => Class::Above as u8, // superscript alaph
        mcc::CCC29 => Class::Below as u8, // kasratan
        mcc::CCC32 => Class::Below as u8, // kasra

        // Thai
        mcc::CCC103 => Class::BelowRight as u8, // sara u / sara uu
        mcc::CCC107 => Class::AboveRight as u8, // mai

        // Lao
        mcc::CCC118 => Class::Below as u8, // sign u / sign uu
        mcc::CCC122 => Class::Above as u8, // mai

        // Tibetian
        mcc::CCC129 => Class::Below as u8, // sign aa
        mcc::CCC130 => Class::Above as u8, // sign i
        mcc::CCC132 => Class::Below as u8, // sign u

        _ => class,
    }
}

pub fn _hb_ot_shape_fallback_mark_position_recategorize_marks(
    _: &hb_ot_shape_plan_t,
    _: &hb_font_t,
    buffer: &mut hb_buffer_t,
) {
    let len = buffer.len;
    for info in &mut buffer.info[..len] {
        if _hb_glyph_info_get_general_category(info)
            == hb_unicode_general_category_t::NonspacingMark
        {
            let mut class = _hb_glyph_info_get_modified_combining_class(info);
            class = recategorize_combining_class(info.glyph_id, class);
            _hb_glyph_info_set_modified_combining_class(info, class);
        }
    }
}

fn zero_mark_advances(
    buffer: &mut hb_buffer_t,
    start: usize,
    end: usize,
    adjust_offsets_when_zeroing: bool,
) {
    for (info, pos) in buffer.info[start..end]
        .iter()
        .zip(&mut buffer.pos[start..end])
    {
        if _hb_glyph_info_get_general_category(info)
            == hb_unicode_general_category_t::NonspacingMark
        {
            if adjust_offsets_when_zeroing {
                pos.x_offset -= pos.x_advance;
                pos.y_offset -= pos.y_advance;
            }
            pos.x_advance = 0;
            pos.y_advance = 0;
        }
    }
}

fn position_mark(
    _: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    direction: Direction,
    glyph: GlyphId,
    pos: &mut GlyphPosition,
    base_extents: &mut hb_glyph_extents_t,
    combining_class: CanonicalCombiningClass,
) {
    use CanonicalCombiningClass as Class;

    let mut mark_extents = hb_glyph_extents_t::default();
    if !face.glyph_extents(glyph, &mut mark_extents) {
        return;
    };

    let y_gap = face.units_per_em as i32 / 16;
    pos.x_offset = 0;
    pos.y_offset = 0;

    // We don't position LEFT and RIGHT marks.

    // X positioning
    match combining_class {
        Class::DoubleBelow | Class::DoubleAbove if direction.is_horizontal() => {
            pos.x_offset += base_extents.x_bearing
                + if direction.is_forward() {
                    base_extents.width
                } else {
                    0
                }
                - mark_extents.width / 2
                - mark_extents.x_bearing;
        }

        Class::AttachedBelowLeft | Class::BelowLeft | Class::AboveLeft => {
            // Left align.
            pos.x_offset += base_extents.x_bearing - mark_extents.x_bearing;
        }

        Class::AttachedAboveRight | Class::BelowRight | Class::AboveRight => {
            // Right align.
            pos.x_offset += base_extents.x_bearing + base_extents.width
                - mark_extents.width
                - mark_extents.x_bearing;
        }

        Class::AttachedBelow | Class::AttachedAbove | Class::Below | Class::Above | _ => {
            // Center align.
            pos.x_offset += base_extents.x_bearing + (base_extents.width - mark_extents.width) / 2
                - mark_extents.x_bearing;
        }
    }

    let is_attached = matches!(
        combining_class,
        Class::AttachedBelowLeft
            | Class::AttachedBelow
            | Class::AttachedAbove
            | Class::AttachedAboveRight
    );

    // Y positioning.
    match combining_class {
        Class::DoubleBelow
        | Class::BelowLeft
        | Class::Below
        | Class::BelowRight
        | Class::AttachedBelowLeft
        | Class::AttachedBelow => {
            if !is_attached {
                // Add gap.
                base_extents.height -= y_gap;
            }

            pos.y_offset = base_extents.y_bearing + base_extents.height - mark_extents.y_bearing;

            // Never shift up "below" marks.
            if (y_gap > 0) == (pos.y_offset > 0) {
                base_extents.height -= pos.y_offset;
                pos.y_offset = 0;
            }

            base_extents.height += mark_extents.height;
        }

        Class::DoubleAbove
        | Class::AboveLeft
        | Class::Above
        | Class::AboveRight
        | Class::AttachedAbove
        | Class::AttachedAboveRight => {
            if !is_attached {
                // Add gap.
                base_extents.y_bearing += y_gap;
                base_extents.height -= y_gap;
            }

            pos.y_offset = base_extents.y_bearing - (mark_extents.y_bearing + mark_extents.height);

            // Don't shift down "above" marks too much.
            if (y_gap > 0) != (pos.y_offset > 0) {
                let correction = -pos.y_offset / 2;
                base_extents.y_bearing += correction;
                base_extents.height -= correction;
                pos.y_offset += correction;
            }

            base_extents.y_bearing -= mark_extents.height;
            base_extents.height += mark_extents.height;
        }

        _ => {}
    }
}

fn position_around_base(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
    base: usize,
    end: usize,
    adjust_offsets_when_zeroing: bool,
) {
    let mut horizontal_dir = Direction::Invalid;
    buffer.unsafe_to_break(Some(base), Some(end));

    let base_info = &buffer.info[base];
    let base_pos = &buffer.pos[base];
    let base_glyph = base_info.as_glyph();

    let mut base_extents = hb_glyph_extents_t::default();
    if !face.glyph_extents(base_glyph, &mut base_extents) {
        zero_mark_advances(buffer, base + 1, end, adjust_offsets_when_zeroing);
        return;
    };

    base_extents.y_bearing += base_pos.y_offset;
    base_extents.x_bearing = 0;

    // Use horizontal advance for horizontal positioning.
    // Generally a better idea. Also works for zero-ink glyphs. See:
    // https://github.com/harfbuzz/harfbuzz/issues/1532
    base_extents.width = face.glyph_h_advance(base_glyph);

    let lig_id = _hb_glyph_info_get_lig_id(base_info) as u32;
    let num_lig_components = _hb_glyph_info_get_lig_num_comps(base_info) as i32;

    let mut x_offset = 0;
    let mut y_offset = 0;
    if buffer.direction.is_forward() {
        x_offset -= base_pos.x_advance;
        y_offset -= base_pos.y_advance;
    }

    let mut last_lig_component: i32 = -1;
    let mut last_combining_class: u8 = 255;
    let mut component_extents = base_extents;
    let mut cluster_extents = base_extents;

    for (info, pos) in buffer.info[base + 1..end]
        .iter()
        .zip(&mut buffer.pos[base + 1..end])
    {
        if _hb_glyph_info_get_modified_combining_class(info) != 0 {
            if num_lig_components > 1 {
                let this_lig_id = _hb_glyph_info_get_lig_id(info) as u32;
                let mut this_lig_component = _hb_glyph_info_get_lig_comp(info) as i32 - 1;

                // Conditions for attaching to the last component.
                if lig_id == 0 || lig_id != this_lig_id || this_lig_component >= num_lig_components
                {
                    this_lig_component = num_lig_components - 1;
                }

                if last_lig_component != this_lig_component {
                    last_lig_component = this_lig_component;
                    last_combining_class = 255;
                    component_extents = base_extents;

                    if horizontal_dir == Direction::Invalid {
                        horizontal_dir = if plan.direction.is_horizontal() {
                            plan.direction
                        } else {
                            plan.script
                                .and_then(Direction::from_script)
                                .unwrap_or(Direction::LeftToRight)
                        };
                    }

                    component_extents.x_bearing += (if horizontal_dir == Direction::LeftToRight {
                        this_lig_component
                    } else {
                        num_lig_components - 1 - this_lig_component
                    } * component_extents.width)
                        / num_lig_components;

                    component_extents.width /= num_lig_components;
                }
            }

            let this_combining_class = _hb_glyph_info_get_modified_combining_class(info);
            if last_combining_class != this_combining_class {
                last_combining_class = this_combining_class;
                cluster_extents = component_extents;
            }

            position_mark(
                plan,
                face,
                buffer.direction,
                info.as_glyph(),
                pos,
                &mut cluster_extents,
                conv_combining_class(this_combining_class),
            );

            pos.x_advance = 0;
            pos.y_advance = 0;
            pos.x_offset += x_offset;
            pos.y_offset += y_offset;
        } else {
            if buffer.direction.is_forward() {
                x_offset -= pos.x_advance;
                y_offset -= pos.y_advance;
            } else {
                x_offset += pos.x_advance;
                y_offset += pos.y_advance;
            }
        }
    }
}

fn position_cluster(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
    start: usize,
    end: usize,
    adjust_offsets_when_zeroing: bool,
) {
    if end - start < 2 {
        return;
    }

    // Find the base glyph
    let mut i = start;
    while i < end {
        if !_hb_glyph_info_is_unicode_mark(&buffer.info[i]) {
            // Find mark glyphs
            let mut j = i + 1;
            while j < end && _hb_glyph_info_is_unicode_mark(&buffer.info[j]) {
                j += 1;
            }

            position_around_base(plan, face, buffer, i, j, adjust_offsets_when_zeroing);
            i = j - 1;
        }
        i += 1;
    }
}

pub fn position_marks(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
    adjust_offsets_when_zeroing: bool,
) {
    let mut start = 0;
    let len = buffer.len;
    for i in 1..len {
        if !_hb_glyph_info_is_unicode_mark(&buffer.info[i]) {
            position_cluster(plan, face, buffer, start, i, adjust_offsets_when_zeroing);
            start = i;
        }
    }

    position_cluster(plan, face, buffer, start, len, adjust_offsets_when_zeroing);
}

pub fn _hb_ot_shape_fallback_kern(_: &hb_ot_shape_plan_t, _: &hb_font_t, _: &mut hb_buffer_t) {
    // STUB: this is deprecated in HarfBuzz
}

pub fn _hb_ot_shape_fallback_spaces(
    _: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) {
    use super::unicode::hb_unicode_funcs_t as t;

    let len = buffer.len;
    let horizontal = buffer.direction.is_horizontal();
    for (info, pos) in buffer.info[..len].iter().zip(&mut buffer.pos[..len]) {
        if _hb_glyph_info_is_unicode_space(info) && !_hb_glyph_info_ligated(info) {
            let space_type = _hb_glyph_info_get_unicode_space_fallback_type(info);
            match space_type {
                t::SPACE_EM
                | t::SPACE_EM_2
                | t::SPACE_EM_3
                | t::SPACE_EM_4
                | t::SPACE_EM_5
                | t::SPACE_EM_6
                | t::SPACE_EM_16 => {
                    let length =
                        (face.units_per_em as i32 + (space_type as i32) / 2) / space_type as i32;
                    if horizontal {
                        pos.x_advance = length;
                    } else {
                        pos.y_advance = -length;
                    }
                }

                t::SPACE_4_EM_18 => {
                    let length = ((face.units_per_em as i64) * 4 / 18) as i32;
                    if horizontal {
                        pos.x_advance = length
                    } else {
                        pos.y_advance = -length;
                    }
                }

                t::SPACE_FIGURE => {
                    for u in '0'..='9' {
                        if let Some(glyph) = face.get_nominal_glyph(u as u32) {
                            if horizontal {
                                pos.x_advance = face.glyph_h_advance(glyph);
                            } else {
                                pos.y_advance = face.glyph_v_advance(glyph);
                            }
                            break;
                        }
                    }
                }

                t::SPACE_PUNCTUATION => {
                    let punct = face
                        .get_nominal_glyph('.' as u32)
                        .or_else(|| face.get_nominal_glyph(',' as u32));

                    if let Some(glyph) = punct {
                        if horizontal {
                            pos.x_advance = face.glyph_h_advance(glyph);
                        } else {
                            pos.y_advance = face.glyph_v_advance(glyph);
                        }
                    }
                }

                t::SPACE_NARROW => {
                    // Half-space?
                    // Unicode doc https://unicode.org/charts/PDF/U2000.pdf says ~1/4 or 1/5 of EM.
                    // However, in my testing, many fonts have their regular space being about that
                    // size. To me, a percentage of the space width makes more sense. Half is as
                    // good as any.
                    if horizontal {
                        pos.x_advance /= 2;
                    } else {
                        pos.y_advance /= 2;
                    }
                }

                _ => {}
            }
        }
    }
}

// TODO: can we cast directly?
fn conv_combining_class(n: u8) -> CanonicalCombiningClass {
    use CanonicalCombiningClass as Class;
    match n {
        1 => Class::Overlay,
        6 => Class::HanReading,
        7 => Class::Nukta,
        8 => Class::KanaVoicing,
        9 => Class::Virama,
        10 => Class::CCC10,
        11 => Class::CCC11,
        12 => Class::CCC12,
        13 => Class::CCC13,
        14 => Class::CCC14,
        15 => Class::CCC15,
        16 => Class::CCC16,
        17 => Class::CCC17,
        18 => Class::CCC18,
        19 => Class::CCC19,
        20 => Class::CCC20,
        21 => Class::CCC21,
        22 => Class::CCC22,
        23 => Class::CCC23,
        24 => Class::CCC24,
        25 => Class::CCC25,
        26 => Class::CCC26,
        27 => Class::CCC27,
        28 => Class::CCC28,
        29 => Class::CCC29,
        30 => Class::CCC30,
        31 => Class::CCC31,
        32 => Class::CCC32,
        33 => Class::CCC33,
        34 => Class::CCC34,
        35 => Class::CCC35,
        36 => Class::CCC36,
        84 => Class::CCC84,
        91 => Class::CCC91,
        103 => Class::CCC103,
        107 => Class::CCC107,
        118 => Class::CCC118,
        122 => Class::CCC122,
        129 => Class::CCC129,
        130 => Class::CCC130,
        132 => Class::CCC132,
        200 => Class::AttachedBelowLeft,
        202 => Class::AttachedBelow,
        214 => Class::AttachedAbove,
        216 => Class::AttachedAboveRight,
        218 => Class::BelowLeft,
        220 => Class::Below,
        222 => Class::BelowRight,
        224 => Class::Left,
        226 => Class::Right,
        228 => Class::AboveLeft,
        230 => Class::Above,
        232 => Class::AboveRight,
        233 => Class::DoubleBelow,
        234 => Class::DoubleAbove,
        240 => Class::IotaSubscript,
        _ => Class::NotReordered,
    }
}
