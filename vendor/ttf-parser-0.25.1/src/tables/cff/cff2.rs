//! A [Compact Font Format 2 Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/cff2) implementation.

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2charstr

use core::convert::TryFrom;
use core::ops::Range;

use super::argstack::ArgumentsStack;
use super::charstring::CharStringParser;
use super::dict::DictionaryParser;
use super::index::{parse_index, Index};
use super::{calc_subroutine_bias, conv_subroutine_index, Builder, CFFError};
use crate::parser::{NumFrom, Stream, TryNumFrom};
use crate::var_store::*;
use crate::{GlyphId, NormalizedCoordinate, OutlineBuilder, Rect, RectF};

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2#7-top-dict-data
// 'Operators in DICT may be preceded by up to a maximum of 513 operands.'
const MAX_OPERANDS_LEN: usize = 513;

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2charstr#appendix-b-cff2-charstring-implementation-limits
const STACK_LIMIT: u8 = 10;
const MAX_ARGUMENTS_STACK_LEN: usize = 513;

const TWO_BYTE_OPERATOR_MARK: u8 = 12;

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2charstr#4-charstring-operators
mod operator {
    pub const HORIZONTAL_STEM: u8 = 1;
    pub const VERTICAL_STEM: u8 = 3;
    pub const VERTICAL_MOVE_TO: u8 = 4;
    pub const LINE_TO: u8 = 5;
    pub const HORIZONTAL_LINE_TO: u8 = 6;
    pub const VERTICAL_LINE_TO: u8 = 7;
    pub const CURVE_TO: u8 = 8;
    pub const CALL_LOCAL_SUBROUTINE: u8 = 10;
    pub const VS_INDEX: u8 = 15;
    pub const BLEND: u8 = 16;
    pub const HORIZONTAL_STEM_HINT_MASK: u8 = 18;
    pub const HINT_MASK: u8 = 19;
    pub const COUNTER_MASK: u8 = 20;
    pub const MOVE_TO: u8 = 21;
    pub const HORIZONTAL_MOVE_TO: u8 = 22;
    pub const VERTICAL_STEM_HINT_MASK: u8 = 23;
    pub const CURVE_LINE: u8 = 24;
    pub const LINE_CURVE: u8 = 25;
    pub const VV_CURVE_TO: u8 = 26;
    pub const HH_CURVE_TO: u8 = 27;
    pub const SHORT_INT: u8 = 28;
    pub const CALL_GLOBAL_SUBROUTINE: u8 = 29;
    pub const VH_CURVE_TO: u8 = 30;
    pub const HV_CURVE_TO: u8 = 31;
    pub const HFLEX: u8 = 34;
    pub const FLEX: u8 = 35;
    pub const HFLEX1: u8 = 36;
    pub const FLEX1: u8 = 37;
    pub const FIXED_16_16: u8 = 255;
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2#table-9-top-dict-operator-entries
mod top_dict_operator {
    pub const CHAR_STRINGS_OFFSET: u16 = 17;
    pub const VARIATION_STORE_OFFSET: u16 = 24;
    pub const FONT_DICT_INDEX_OFFSET: u16 = 1236;
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2#table-10-font-dict-operator-entries
mod font_dict_operator {
    pub const PRIVATE_DICT_SIZE_AND_OFFSET: u16 = 18;
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2#table-16-private-dict-operators
mod private_dict_operator {
    pub const LOCAL_SUBROUTINES_OFFSET: u16 = 19;
}

#[derive(Clone, Copy, Default)]
struct TopDictData {
    char_strings_offset: usize,
    font_dict_index_offset: Option<usize>,
    variation_store_offset: Option<usize>,
}

fn parse_top_dict(data: &[u8]) -> Option<TopDictData> {
    let mut dict_data = TopDictData::default();

    let mut operands_buffer = [0.0; MAX_OPERANDS_LEN];
    let mut dict_parser = DictionaryParser::new(data, &mut operands_buffer);
    while let Some(operator) = dict_parser.parse_next() {
        if operator.get() == top_dict_operator::CHAR_STRINGS_OFFSET {
            dict_data.char_strings_offset = dict_parser.parse_offset()?;
        } else if operator.get() == top_dict_operator::FONT_DICT_INDEX_OFFSET {
            dict_data.font_dict_index_offset = dict_parser.parse_offset();
        } else if operator.get() == top_dict_operator::VARIATION_STORE_OFFSET {
            dict_data.variation_store_offset = dict_parser.parse_offset();
        }
    }

    // Must be set, otherwise there are nothing to parse.
    if dict_data.char_strings_offset == 0 {
        return None;
    }

    Some(dict_data)
}

fn parse_font_dict(data: &[u8]) -> Option<Range<usize>> {
    let mut private_dict_range = None;

    let mut operands_buffer = [0.0; MAX_OPERANDS_LEN];
    let mut dict_parser = DictionaryParser::new(data, &mut operands_buffer);
    while let Some(operator) = dict_parser.parse_next() {
        if operator.get() == font_dict_operator::PRIVATE_DICT_SIZE_AND_OFFSET {
            dict_parser.parse_operands()?;
            let operands = dict_parser.operands();

            if operands.len() == 2 {
                let len = usize::try_from(operands[0] as i32).ok()?;
                let start = usize::try_from(operands[1] as i32).ok()?;
                let end = start.checked_add(len)?;
                private_dict_range = Some(start..end);
            }

            break;
        }
    }

    private_dict_range
}

fn parse_private_dict(data: &[u8]) -> Option<usize> {
    let mut subroutines_offset = None;
    let mut operands_buffer = [0.0; MAX_OPERANDS_LEN];
    let mut dict_parser = DictionaryParser::new(data, &mut operands_buffer);
    while let Some(operator) = dict_parser.parse_next() {
        if operator.get() == private_dict_operator::LOCAL_SUBROUTINES_OFFSET {
            dict_parser.parse_operands()?;
            let operands = dict_parser.operands();

            if operands.len() == 1 {
                subroutines_offset = usize::try_from(operands[0] as i32).ok();
            }

            break;
        }
    }

    subroutines_offset
}

/// CFF2 allows up to 65535 scalars, but an average font will have 3-5.
/// So 64 is more than enough.
const SCALARS_MAX: u8 = 64;

#[derive(Clone, Copy)]
pub(crate) struct Scalars {
    d: [f32; SCALARS_MAX as usize], // 256B
    len: u8,
}

impl Default for Scalars {
    fn default() -> Self {
        Scalars {
            d: [0.0; SCALARS_MAX as usize],
            len: 0,
        }
    }
}

impl Scalars {
    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn at(&self, i: u8) -> f32 {
        if i < self.len {
            self.d[usize::from(i)]
        } else {
            0.0
        }
    }

    pub fn push(&mut self, n: f32) -> Option<()> {
        if self.len < SCALARS_MAX {
            self.d[usize::from(self.len)] = n;
            self.len += 1;
            Some(())
        } else {
            None
        }
    }
}

struct CharStringParserContext<'a> {
    metadata: &'a Table<'a>,
    coordinates: &'a [NormalizedCoordinate],
    scalars: Scalars,
    had_vsindex: bool,
    had_blend: bool,
    stems_len: u32,
}

impl CharStringParserContext<'_> {
    fn update_scalars(&mut self, index: u16) -> Result<(), CFFError> {
        self.scalars.clear();

        let indices = self
            .metadata
            .item_variation_store
            .region_indices(index)
            .ok_or(CFFError::InvalidItemVariationDataIndex)?;
        for index in indices {
            let scalar = self
                .metadata
                .item_variation_store
                .regions
                .evaluate_region(index, self.coordinates);
            self.scalars
                .push(scalar)
                .ok_or(CFFError::BlendRegionsLimitReached)?;
        }

        Ok(())
    }
}

fn parse_char_string(
    data: &[u8],
    metadata: &Table,
    coordinates: &[NormalizedCoordinate],
    builder: &mut dyn OutlineBuilder,
) -> Result<Rect, CFFError> {
    let mut ctx = CharStringParserContext {
        metadata,
        coordinates,
        scalars: Scalars::default(),
        had_vsindex: false,
        had_blend: false,
        stems_len: 0,
    };

    // Load scalars at default index.
    ctx.update_scalars(0)?;

    let mut inner_builder = Builder {
        builder,
        bbox: RectF::new(),
    };

    let stack = ArgumentsStack {
        data: &mut [0.0; MAX_ARGUMENTS_STACK_LEN], // 2052B
        len: 0,
        max_len: MAX_ARGUMENTS_STACK_LEN,
    };
    let mut parser = CharStringParser {
        stack,
        builder: &mut inner_builder,
        x: 0.0,
        y: 0.0,
        has_move_to: false,
        is_first_move_to: true,
        width_only: false,
    };
    _parse_char_string(&mut ctx, data, 0, &mut parser)?;
    // let _ = _parse_char_string(&mut ctx, data, 0.0, 0.0, &mut stack, 0, &mut inner_builder)?;

    let bbox = parser.builder.bbox;

    // Check that bbox was changed.
    if bbox.is_default() {
        return Err(CFFError::ZeroBBox);
    }

    bbox.to_rect().ok_or(CFFError::BboxOverflow)
}

fn _parse_char_string(
    ctx: &mut CharStringParserContext,
    char_string: &[u8],
    depth: u8,
    p: &mut CharStringParser,
) -> Result<(), CFFError> {
    let mut s = Stream::new(char_string);
    while !s.at_end() {
        let op = s.read::<u8>().ok_or(CFFError::ReadOutOfBounds)?;
        match op {
            0 | 2 | 9 | 11 | 13 | 14 | 17 => {
                // Reserved.
                return Err(CFFError::InvalidOperator);
            }
            operator::HORIZONTAL_STEM
            | operator::VERTICAL_STEM
            | operator::HORIZONTAL_STEM_HINT_MASK
            | operator::VERTICAL_STEM_HINT_MASK => {
                // y dy {dya dyb}* hstem
                // x dx {dxa dxb}* vstem
                // y dy {dya dyb}* hstemhm
                // x dx {dxa dxb}* vstemhm

                ctx.stems_len += p.stack.len() as u32 >> 1;

                // We are ignoring the hint operators.
                p.stack.clear();
            }
            operator::VERTICAL_MOVE_TO => {
                p.parse_vertical_move_to(0)?;
            }
            operator::LINE_TO => {
                p.parse_line_to()?;
            }
            operator::HORIZONTAL_LINE_TO => {
                p.parse_horizontal_line_to()?;
            }
            operator::VERTICAL_LINE_TO => {
                p.parse_vertical_line_to()?;
            }
            operator::CURVE_TO => {
                p.parse_curve_to()?;
            }
            operator::CALL_LOCAL_SUBROUTINE => {
                if p.stack.is_empty() {
                    return Err(CFFError::InvalidArgumentsStackLength);
                }

                if depth == STACK_LIMIT {
                    return Err(CFFError::NestingLimitReached);
                }

                let subroutine_bias = calc_subroutine_bias(ctx.metadata.local_subrs.len());
                let index = conv_subroutine_index(p.stack.pop(), subroutine_bias)?;
                let char_string = ctx
                    .metadata
                    .local_subrs
                    .get(index)
                    .ok_or(CFFError::InvalidSubroutineIndex)?;
                _parse_char_string(ctx, char_string, depth + 1, p)?;
            }
            TWO_BYTE_OPERATOR_MARK => {
                // flex
                let op2 = s.read::<u8>().ok_or(CFFError::ReadOutOfBounds)?;
                match op2 {
                    operator::HFLEX => p.parse_hflex()?,
                    operator::FLEX => p.parse_flex()?,
                    operator::HFLEX1 => p.parse_hflex1()?,
                    operator::FLEX1 => p.parse_flex1()?,
                    _ => return Err(CFFError::UnsupportedOperator),
                }
            }
            operator::VS_INDEX => {
                // |- ivs vsindex (15) |-

                // `vsindex` must precede the first `blend` operator, and may occur only once.
                if ctx.had_blend || ctx.had_vsindex {
                    // TODO: maybe add a custom error
                    return Err(CFFError::InvalidOperator);
                }

                if p.stack.len() != 1 {
                    return Err(CFFError::InvalidArgumentsStackLength);
                }

                let index = u16::try_num_from(p.stack.pop())
                    .ok_or(CFFError::InvalidItemVariationDataIndex)?;
                ctx.update_scalars(index)?;

                ctx.had_vsindex = true;

                p.stack.clear();
            }
            operator::BLEND => {
                // num(0)..num(n-1), delta(0,0)..delta(k-1,0),
                // delta(0,1)..delta(k-1,1) .. delta(0,n-1)..delta(k-1,n-1)
                // n blend (16) val(0)..val(n-1)

                ctx.had_blend = true;

                let n = u16::try_num_from(p.stack.pop())
                    .ok_or(CFFError::InvalidNumberOfBlendOperands)?;
                let k = ctx.scalars.len();

                let len = usize::from(n) * (usize::from(k) + 1);
                if p.stack.len() < len {
                    return Err(CFFError::InvalidArgumentsStackLength);
                }

                let start = p.stack.len() - len;
                for i in (0..n).rev() {
                    for j in 0..k {
                        let delta = p.stack.pop();
                        p.stack.data[start + usize::from(i)] += delta * ctx.scalars.at(k - j - 1);
                    }
                }
            }
            operator::HINT_MASK | operator::COUNTER_MASK => {
                ctx.stems_len += p.stack.len() as u32 >> 1;
                s.advance(usize::num_from((ctx.stems_len + 7) >> 3));

                // We are ignoring the hint operators.
                p.stack.clear();
            }
            operator::MOVE_TO => {
                p.parse_move_to(0)?;
            }
            operator::HORIZONTAL_MOVE_TO => {
                p.parse_horizontal_move_to(0)?;
            }
            operator::CURVE_LINE => {
                p.parse_curve_line()?;
            }
            operator::LINE_CURVE => {
                p.parse_line_curve()?;
            }
            operator::VV_CURVE_TO => {
                p.parse_vv_curve_to()?;
            }
            operator::HH_CURVE_TO => {
                p.parse_hh_curve_to()?;
            }
            operator::SHORT_INT => {
                let n = s.read::<i16>().ok_or(CFFError::ReadOutOfBounds)?;
                p.stack.push(f32::from(n))?;
            }
            operator::CALL_GLOBAL_SUBROUTINE => {
                if p.stack.is_empty() {
                    return Err(CFFError::InvalidArgumentsStackLength);
                }

                if depth == STACK_LIMIT {
                    return Err(CFFError::NestingLimitReached);
                }

                let subroutine_bias = calc_subroutine_bias(ctx.metadata.global_subrs.len());
                let index = conv_subroutine_index(p.stack.pop(), subroutine_bias)?;
                let char_string = ctx
                    .metadata
                    .global_subrs
                    .get(index)
                    .ok_or(CFFError::InvalidSubroutineIndex)?;
                _parse_char_string(ctx, char_string, depth + 1, p)?;
            }
            operator::VH_CURVE_TO => {
                p.parse_vh_curve_to()?;
            }
            operator::HV_CURVE_TO => {
                p.parse_hv_curve_to()?;
            }
            32..=246 => {
                p.parse_int1(op)?;
            }
            247..=250 => {
                p.parse_int2(op, &mut s)?;
            }
            251..=254 => {
                p.parse_int3(op, &mut s)?;
            }
            operator::FIXED_16_16 => {
                p.parse_fixed(&mut s)?;
            }
        }
    }

    Ok(())
}

/// A [Compact Font Format 2 Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/cff2).
#[derive(Clone, Copy, Default)]
pub struct Table<'a> {
    global_subrs: Index<'a>,
    local_subrs: Index<'a>,
    char_strings: Index<'a>,
    item_variation_store: ItemVariationStore<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        // Parse Header.
        let major = s.read::<u8>()?;
        s.skip::<u8>(); // minor
        let header_size = s.read::<u8>()?;
        let top_dict_length = s.read::<u16>()?;

        if major != 2 {
            return None;
        }

        // Jump to Top DICT. It's not necessarily right after the header.
        if header_size > 5 {
            s.advance(usize::from(header_size) - 5);
        }

        let top_dict_data = s.read_bytes(usize::from(top_dict_length))?;
        let top_dict = parse_top_dict(top_dict_data)?;

        let mut metadata = Self::default();

        // Parse Global Subroutines INDEX.
        metadata.global_subrs = parse_index::<u32>(&mut s)?;

        metadata.char_strings = {
            let mut s = Stream::new_at(data, top_dict.char_strings_offset)?;
            parse_index::<u32>(&mut s)?
        };

        if let Some(offset) = top_dict.variation_store_offset {
            let mut s = Stream::new_at(data, offset)?;
            s.skip::<u16>(); // length
            metadata.item_variation_store = ItemVariationStore::parse(s)?;
        }

        // TODO: simplify
        if let Some(offset) = top_dict.font_dict_index_offset {
            let mut s = Stream::new_at(data, offset)?;
            'outer: for font_dict_data in parse_index::<u32>(&mut s)? {
                if let Some(private_dict_range) = parse_font_dict(font_dict_data) {
                    // 'Private DICT size and offset, from start of the CFF2 table.'
                    let private_dict_data = data.get(private_dict_range.clone())?;
                    if let Some(subroutines_offset) = parse_private_dict(private_dict_data) {
                        // 'The local subroutines offset is relative to the beginning
                        // of the Private DICT data.'
                        if let Some(start) =
                            private_dict_range.start.checked_add(subroutines_offset)
                        {
                            let data = data.get(start..data.len())?;
                            let mut s = Stream::new(data);
                            metadata.local_subrs = parse_index::<u32>(&mut s)?;
                            break 'outer;
                        }
                    }
                }
            }
        }

        Some(metadata)
    }

    /// Outlines a glyph.
    pub fn outline(
        &self,
        coordinates: &[NormalizedCoordinate],
        glyph_id: GlyphId,
        builder: &mut dyn OutlineBuilder,
    ) -> Result<Rect, CFFError> {
        let data = self
            .char_strings
            .get(u32::from(glyph_id.0))
            .ok_or(CFFError::NoGlyph)?;
        parse_char_string(data, self, coordinates, builder)
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
