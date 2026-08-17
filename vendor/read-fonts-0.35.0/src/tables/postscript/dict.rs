//! Parsing for PostScript DICTs.

use std::ops::Range;

use super::{BlendState, Error, Number, Stack, StringId};
use crate::{types::Fixed, Cursor, ReadError};

/// PostScript DICT operator.
///
/// See "Table 9 Top DICT Operator Entries" and "Table 23 Private DICT
/// Operators" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf>
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Operator {
    Version,
    Notice,
    FullName,
    FamilyName,
    Weight,
    FontBbox,
    CharstringsOffset,
    PrivateDictRange,
    VariationStoreOffset,
    Copyright,
    IsFixedPitch,
    ItalicAngle,
    UnderlinePosition,
    UnderlineThickness,
    PaintType,
    CharstringType,
    FontMatrix,
    StrokeWidth,
    FdArrayOffset,
    FdSelectOffset,
    BlueValues,
    OtherBlues,
    FamilyBlues,
    FamilyOtherBlues,
    SubrsOffset,
    VariationStoreIndex,
    BlueScale,
    BlueShift,
    BlueFuzz,
    LanguageGroup,
    ExpansionFactor,
    Encoding,
    Charset,
    UniqueId,
    Xuid,
    SyntheticBase,
    PostScript,
    BaseFontName,
    BaseFontBlend,
    Ros,
    CidFontVersion,
    CidFontRevision,
    CidFontType,
    CidCount,
    UidBase,
    FontName,
    StdHw,
    StdVw,
    DefaultWidthX,
    NominalWidthX,
    Blend,
    StemSnapH,
    StemSnapV,
    ForceBold,
    InitialRandomSeed,
}

impl Operator {
    fn from_opcode(opcode: u8) -> Option<Self> {
        use Operator::*;
        Some(match opcode {
            // Top DICT operators
            0 => Version,
            1 => Notice,
            2 => FullName,
            3 => FamilyName,
            4 => Weight,
            5 => FontBbox,
            13 => UniqueId,
            14 => Xuid,
            15 => Charset,
            16 => Encoding,
            17 => CharstringsOffset,
            18 => PrivateDictRange,
            24 => VariationStoreOffset,
            // Private DICT operators
            6 => BlueValues,
            7 => OtherBlues,
            8 => FamilyBlues,
            9 => FamilyOtherBlues,
            10 => StdHw,
            11 => StdVw,
            19 => SubrsOffset,
            20 => DefaultWidthX,
            21 => NominalWidthX,
            22 => VariationStoreIndex,
            23 => Blend,
            // Font DICT only uses PrivateDictRange
            _ => return None,
        })
    }

    fn from_extended_opcode(opcode: u8) -> Option<Self> {
        use Operator::*;
        Some(match opcode {
            // Top DICT operators
            0 => Copyright,
            1 => IsFixedPitch,
            2 => ItalicAngle,
            3 => UnderlinePosition,
            4 => UnderlineThickness,
            5 => PaintType,
            6 => CharstringType,
            7 => FontMatrix,
            8 => StrokeWidth,
            20 => SyntheticBase,
            21 => PostScript,
            22 => BaseFontName,
            23 => BaseFontBlend,
            30 => Ros,
            31 => CidFontVersion,
            32 => CidFontRevision,
            33 => CidFontType,
            34 => CidCount,
            35 => UidBase,
            36 => FdArrayOffset,
            37 => FdSelectOffset,
            38 => FontName,
            // Private DICT operators
            9 => BlueScale,
            10 => BlueShift,
            11 => BlueFuzz,
            12 => StemSnapH,
            13 => StemSnapV,
            14 => ForceBold,
            17 => LanguageGroup,
            18 => ExpansionFactor,
            19 => InitialRandomSeed,
            _ => return None,
        })
    }
}

/// Either a PostScript DICT operator or a (numeric) operand.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Token {
    /// An operator parsed from a DICT.
    Operator(Operator),
    /// A number parsed from a DICT. If the source was in
    /// binary coded decimal format, then the second field
    /// contains the parsed components.
    Operand(Number, Option<BcdComponents>),
}

impl From<Operator> for Token {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

impl<T> From<T> for Token
where
    T: Into<Number>,
{
    fn from(value: T) -> Self {
        Self::Operand(value.into(), None)
    }
}

/// Given a byte slice containing DICT data, returns an iterator yielding
/// raw operands and operators.
///
/// This does not perform any additional processing such as type conversion,
/// delta decoding or blending.
pub fn tokens(dict_data: &[u8]) -> impl Iterator<Item = Result<Token, Error>> + '_ + Clone {
    let mut cursor = crate::FontData::new(dict_data).cursor();
    std::iter::from_fn(move || {
        if cursor.remaining_bytes() == 0 {
            None
        } else {
            Some(parse_token(&mut cursor))
        }
    })
}

fn parse_token(cursor: &mut Cursor) -> Result<Token, Error> {
    // Escape opcode for accessing extensions.
    const ESCAPE: u8 = 12;
    let b0 = cursor.read::<u8>()?;
    Ok(if b0 == ESCAPE {
        let b1 = cursor.read::<u8>()?;
        Token::Operator(Operator::from_extended_opcode(b1).ok_or(Error::InvalidDictOperator(b1))?)
    } else {
        // See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-3-operand-encoding>
        match b0 {
            28 | 29 | 32..=254 => Token::Operand(parse_int(cursor, b0)?.into(), None),
            30 => {
                let components = BcdComponents::parse(cursor)?;
                Token::Operand(components.value(false).into(), Some(components))
            }
            _ => Token::Operator(Operator::from_opcode(b0).ok_or(Error::InvalidDictOperator(b0))?),
        }
    })
}

/// Parse a fixed point value with a dynamic scaling factor.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffparse.c#L580>
fn parse_fixed_dynamic(cursor: &mut Cursor) -> Result<(Fixed, i32), Error> {
    let b0 = cursor.read::<u8>()?;
    match b0 {
        30 => Ok(BcdComponents::parse(cursor)?.dynamically_scaled_value()),
        28 | 29 | 32..=254 => {
            let num = parse_int(cursor, b0)?;
            let mut int_len = 10;
            if num > BCD_INTEGER_LIMIT {
                for (i, power_ten) in BCD_POWER_TENS.iter().enumerate().skip(5) {
                    if num < *power_ten {
                        int_len = i;
                        break;
                    }
                }
                let scaling = if (num - BCD_POWER_TENS[int_len - 5]) > BCD_INTEGER_LIMIT {
                    int_len - 4
                } else {
                    int_len - 5
                };
                Ok((
                    Fixed::from_bits(num) / Fixed::from_bits(BCD_POWER_TENS[scaling]),
                    scaling as i32,
                ))
            } else {
                Ok((Fixed::from_bits(num << 16), 0))
            }
        }
        _ => Err(Error::InvalidNumber),
    }
}

/// PostScript DICT Operator with its associated operands.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Entry {
    Version(StringId),
    Notice(StringId),
    FullName(StringId),
    FamilyName(StringId),
    Weight(StringId),
    FontBbox([Fixed; 4]),
    CharstringsOffset(usize),
    PrivateDictRange(Range<usize>),
    VariationStoreOffset(usize),
    Copyright(StringId),
    IsFixedPitch(bool),
    ItalicAngle(Fixed),
    UnderlinePosition(Fixed),
    UnderlineThickness(Fixed),
    PaintType(i32),
    CharstringType(i32),
    /// Affine matrix and scaling factor.
    FontMatrix([Fixed; 6], i32),
    StrokeWidth(Fixed),
    FdArrayOffset(usize),
    FdSelectOffset(usize),
    BlueValues(Blues),
    OtherBlues(Blues),
    FamilyBlues(Blues),
    FamilyOtherBlues(Blues),
    SubrsOffset(usize),
    VariationStoreIndex(u16),
    BlueScale(Fixed),
    BlueShift(Fixed),
    BlueFuzz(Fixed),
    LanguageGroup(i32),
    ExpansionFactor(Fixed),
    Encoding(usize),
    Charset(usize),
    UniqueId(i32),
    Xuid,
    SyntheticBase(i32),
    PostScript(StringId),
    BaseFontName(StringId),
    BaseFontBlend,
    Ros {
        registry: StringId,
        ordering: StringId,
        supplement: Fixed,
    },
    CidFontVersion(Fixed),
    CidFontRevision(Fixed),
    CidFontType(i32),
    CidCount(u32),
    UidBase(i32),
    FontName(StringId),
    StdHw(Fixed),
    StdVw(Fixed),
    DefaultWidthX(Fixed),
    NominalWidthX(Fixed),
    StemSnapH(StemSnaps),
    StemSnapV(StemSnaps),
    ForceBold(bool),
    InitialRandomSeed(i32),
}

/// Given a byte slice containing DICT data, returns an iterator yielding
/// each operator with its associated operands.
///
/// This performs appropriate type conversions, decodes deltas and applies
/// blending.
///
/// If processing a Private DICT from a CFF2 table and an item variation
/// store is present, then `blend_state` must be provided.
pub fn entries<'a>(
    dict_data: &'a [u8],
    mut blend_state: Option<BlendState<'a>>,
) -> impl Iterator<Item = Result<Entry, Error>> + 'a {
    let mut stack = Stack::new();
    let mut last_bcd_components = None;
    let mut cursor = crate::FontData::new(dict_data).cursor();
    let mut cursor_pos = 0;
    std::iter::from_fn(move || loop {
        if cursor.remaining_bytes() == 0 {
            return None;
        }
        let token = match parse_token(&mut cursor) {
            Ok(token) => token,
            Err(e) => return Some(Err(e)),
        };
        match token {
            Token::Operand(number, bcd_components) => {
                last_bcd_components = bcd_components;
                match stack.push(number) {
                    Ok(_) => continue,
                    Err(e) => return Some(Err(e)),
                }
            }
            Token::Operator(op) => {
                if op == Operator::Blend || op == Operator::VariationStoreIndex {
                    let state = match blend_state.as_mut() {
                        Some(state) => state,
                        None => return Some(Err(Error::MissingBlendState)),
                    };
                    if op == Operator::VariationStoreIndex {
                        match stack
                            .get_i32(0)
                            .and_then(|ix| state.set_store_index(ix as u16))
                        {
                            Ok(_) => {}
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    if op == Operator::Blend {
                        match stack.apply_blend(state) {
                            Ok(_) => continue,
                            Err(e) => return Some(Err(e)),
                        }
                    }
                }
                if op == Operator::BlueScale {
                    // FreeType parses BlueScale using a scaling factor of
                    // 1000, presumably to capture more precision in the
                    // fractional part. We do the same.
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/master/src/cff/cfftoken.h?ref_type=heads#L87>
                    if let Some(bcd_components) = last_bcd_components.take() {
                        // If the most recent numeric value was parsed as a
                        // binary coded decimal then recompute the value using
                        // the desired scaling and replace it on the stack
                        stack.pop_fixed().ok()?;
                        stack.push(bcd_components.value(true)).ok()?;
                    }
                }
                if op == Operator::FontMatrix {
                    // FontMatrix is also parsed specially... *sigh*
                    // Redo the entire thing with special scaling factors
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffparse.c#L623>
                    // Dump the current values
                    stack.clear();
                    last_bcd_components = None;
                    // Now reparse with dynamic scaling
                    let mut cursor = crate::FontData::new(dict_data).cursor();
                    cursor.advance_by(cursor_pos);
                    if let Some((matrix, upem)) = parse_font_matrix(&mut cursor) {
                        return Some(Ok(Entry::FontMatrix(matrix, upem)));
                    }
                    continue;
                }
                last_bcd_components = None;
                let entry = parse_entry(op, &mut stack);
                stack.clear();
                cursor_pos = cursor.position().unwrap_or_default();
                return Some(entry);
            }
        }
    })
}

fn parse_entry(op: Operator, stack: &mut Stack) -> Result<Entry, Error> {
    use Operator::*;
    Ok(match op {
        Version => Entry::Version(stack.pop_i32()?.into()),
        Notice => Entry::Notice(stack.pop_i32()?.into()),
        FullName => Entry::FullName(stack.pop_i32()?.into()),
        FamilyName => Entry::FamilyName(stack.pop_i32()?.into()),
        Weight => Entry::Weight(stack.pop_i32()?.into()),
        FontBbox => Entry::FontBbox([
            stack.get_fixed(0)?,
            stack.get_fixed(1)?,
            stack.get_fixed(2)?,
            stack.get_fixed(3)?,
        ]),
        CharstringsOffset => Entry::CharstringsOffset(stack.pop_i32()? as usize),
        PrivateDictRange => {
            let len = stack.get_i32(0)? as usize;
            let start = stack.get_i32(1)? as usize;
            let end = start.checked_add(len).ok_or(ReadError::OutOfBounds)?;
            Entry::PrivateDictRange(start..end)
        }
        VariationStoreOffset => Entry::VariationStoreOffset(stack.pop_i32()? as usize),
        Copyright => Entry::Copyright(stack.pop_i32()?.into()),
        IsFixedPitch => Entry::IsFixedPitch(stack.pop_i32()? != 0),
        ItalicAngle => Entry::ItalicAngle(stack.pop_fixed()?),
        UnderlinePosition => Entry::UnderlinePosition(stack.pop_fixed()?),
        UnderlineThickness => Entry::UnderlineThickness(stack.pop_fixed()?),
        PaintType => Entry::PaintType(stack.pop_i32()?),
        CharstringType => Entry::CharstringType(stack.pop_i32()?),
        FontMatrix => unreachable!(),
        StrokeWidth => Entry::StrokeWidth(stack.pop_fixed()?),
        FdArrayOffset => Entry::FdArrayOffset(stack.pop_i32()? as usize),
        FdSelectOffset => Entry::FdSelectOffset(stack.pop_i32()? as usize),
        BlueValues => {
            stack.apply_delta_prefix_sum();
            Entry::BlueValues(Blues::new(stack.fixed_values()))
        }
        OtherBlues => {
            stack.apply_delta_prefix_sum();
            Entry::OtherBlues(Blues::new(stack.fixed_values()))
        }
        FamilyBlues => {
            stack.apply_delta_prefix_sum();
            Entry::FamilyBlues(Blues::new(stack.fixed_values()))
        }
        FamilyOtherBlues => {
            stack.apply_delta_prefix_sum();
            Entry::FamilyOtherBlues(Blues::new(stack.fixed_values()))
        }
        SubrsOffset => Entry::SubrsOffset(stack.pop_i32()? as usize),
        VariationStoreIndex => Entry::VariationStoreIndex(stack.pop_i32()? as u16),
        BlueScale => Entry::BlueScale(stack.pop_fixed()?),
        BlueShift => Entry::BlueShift(stack.pop_fixed()?),
        BlueFuzz => Entry::BlueFuzz(stack.pop_fixed()?),
        LanguageGroup => Entry::LanguageGroup(stack.pop_i32()?),
        ExpansionFactor => Entry::ExpansionFactor(stack.pop_fixed()?),
        Encoding => Entry::Encoding(stack.pop_i32()? as usize),
        Charset => Entry::Charset(stack.pop_i32()? as usize),
        UniqueId => Entry::UniqueId(stack.pop_i32()?),
        Xuid => Entry::Xuid,
        SyntheticBase => Entry::SyntheticBase(stack.pop_i32()?),
        PostScript => Entry::PostScript(stack.pop_i32()?.into()),
        BaseFontName => Entry::BaseFontName(stack.pop_i32()?.into()),
        BaseFontBlend => Entry::BaseFontBlend,
        Ros => Entry::Ros {
            registry: stack.get_i32(0)?.into(),
            ordering: stack.get_i32(1)?.into(),
            supplement: stack.get_fixed(2)?,
        },
        CidFontVersion => Entry::CidFontVersion(stack.pop_fixed()?),
        CidFontRevision => Entry::CidFontRevision(stack.pop_fixed()?),
        CidFontType => Entry::CidFontType(stack.pop_i32()?),
        CidCount => Entry::CidCount(stack.pop_i32()? as u32),
        UidBase => Entry::UidBase(stack.pop_i32()?),
        FontName => Entry::FontName(stack.pop_i32()?.into()),
        StdHw => Entry::StdHw(stack.pop_fixed()?),
        StdVw => Entry::StdVw(stack.pop_fixed()?),
        DefaultWidthX => Entry::DefaultWidthX(stack.pop_fixed()?),
        NominalWidthX => Entry::NominalWidthX(stack.pop_fixed()?),
        StemSnapH => {
            stack.apply_delta_prefix_sum();
            Entry::StemSnapH(StemSnaps::new(stack.fixed_values()))
        }
        StemSnapV => {
            stack.apply_delta_prefix_sum();
            Entry::StemSnapV(StemSnaps::new(stack.fixed_values()))
        }
        ForceBold => Entry::ForceBold(stack.pop_i32()? != 0),
        InitialRandomSeed => Entry::InitialRandomSeed(stack.pop_i32()?),
        // Blend is handled at the layer above
        Blend => unreachable!(),
    })
}

/// Parses a font matrix using dynamic scaling factors.
///
/// Returns the matrix and an adjusted upem factor.
fn parse_font_matrix(cursor: &mut Cursor) -> Option<([Fixed; 6], i32)> {
    let mut values = [Fixed::ZERO; 6];
    let mut scalings = [0i32; 6];
    let mut max_scaling = i32::MIN;
    let mut min_scaling = i32::MAX;
    for (value, scaling) in values.iter_mut().zip(&mut scalings) {
        let (v, s) = parse_fixed_dynamic(cursor).ok()?;
        if v != Fixed::ZERO {
            max_scaling = max_scaling.max(s);
            min_scaling = min_scaling.min(s);
        }
        *value = v;
        *scaling = s;
    }
    if !(-9..=0).contains(&max_scaling)
        || (max_scaling - min_scaling < 0)
        || (max_scaling - min_scaling) > 9
    {
        return None;
    }
    for (value, scaling) in values.iter_mut().zip(scalings) {
        if *value == Fixed::ZERO {
            continue;
        }
        let divisor = BCD_POWER_TENS[(max_scaling - scaling) as usize];
        let half_divisor = divisor >> 1;
        if *value < Fixed::ZERO {
            if i32::MIN + half_divisor < value.to_bits() {
                *value = Fixed::from_bits((value.to_bits() - half_divisor) / divisor);
            } else {
                *value = Fixed::from_bits(i32::MIN / divisor);
            }
        } else if i32::MAX - half_divisor > value.to_bits() {
            *value = Fixed::from_bits((value.to_bits() + half_divisor) / divisor);
        } else {
            *value = Fixed::from_bits(i32::MAX / divisor);
        }
    }
    // Check for a degenerate matrix
    if is_degenerate(&values) {
        return None;
    }
    let upem = BCD_POWER_TENS[(-max_scaling) as usize];
    Some((values, upem))
}

/// Given a font matrix and a scaled UPEM, compute a new font matrix and UPEM
/// scale factor where the Y scale of the matrix is 1.0.
pub fn normalize_font_matrix(mut matrix: [Fixed; 6], mut scaled_upem: i32) -> ([Fixed; 6], i32) {
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffobjs.c#L727>
    let factor = if matrix[3] != Fixed::ZERO {
        matrix[3].abs()
    } else {
        // Use yx if yy is zero
        matrix[1].abs()
    };
    if factor != Fixed::ONE {
        scaled_upem = (Fixed::from_bits(scaled_upem) / factor).to_bits();
        for value in &mut matrix {
            *value /= factor;
        }
    }
    // FT shifts off the fractional parts of the translation?
    for offset in matrix[4..6].iter_mut() {
        *offset = Fixed::from_bits(offset.to_bits() >> 16);
    }
    (matrix, scaled_upem)
}

/// Check for a degenerate matrix.
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/base/ftcalc.c#L725>
fn is_degenerate(matrix: &[Fixed; 6]) -> bool {
    let [mut xx, mut yx, mut xy, mut yy, ..] = matrix.map(|x| x.to_bits() as i64);
    let val = xx.abs() | yx.abs() | xy.abs() | yy.abs();
    if val == 0 || val > 0x7FFFFFFF {
        return true;
    }
    // Scale the matrix to avoid temp1 overflow
    let msb = 32 - (val as i32).leading_zeros() - 1;
    let shift = msb as i32 - 12;
    if shift > 0 {
        xx >>= shift;
        xy >>= shift;
        yx >>= shift;
        yy >>= shift;
    }
    let temp1 = 32 * (xx * yy - xy * yx).abs();
    let temp2 = (xx * xx) + (xy * xy) + (yx * yx) + (yy * yy);
    if temp1 <= temp2 {
        return true;
    }
    false
}

/// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.h#L141>
const MAX_BLUE_VALUES: usize = 7;

/// Operand for the `BlueValues`, `OtherBlues`, `FamilyBlues` and
/// `FamilyOtherBlues` operators.
///
/// These are used to generate zones when applying hints.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Blues {
    values: [(Fixed, Fixed); MAX_BLUE_VALUES],
    len: u32,
}

impl Blues {
    pub fn new(values: impl Iterator<Item = Fixed>) -> Self {
        let mut blues = Self::default();
        let mut stash = Fixed::ZERO;
        for (i, value) in values.take(MAX_BLUE_VALUES * 2).enumerate() {
            if (i & 1) == 0 {
                stash = value;
            } else {
                blues.values[i / 2] = (stash, value);
                blues.len += 1;
            }
        }
        blues
    }

    pub fn values(&self) -> &[(Fixed, Fixed)] {
        &self.values[..self.len as usize]
    }
}

/// Summary: older PostScript interpreters accept two values, but newer ones
/// accept 12. We'll assume that as maximum.
/// <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5049.StemSnap.pdf>
const MAX_STEM_SNAPS: usize = 12;

/// Operand for the `StemSnapH` and `StemSnapV` operators.
///
/// These are used for stem darkening when applying hints.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct StemSnaps {
    values: [Fixed; MAX_STEM_SNAPS],
    len: u32,
}

impl StemSnaps {
    fn new(values: impl Iterator<Item = Fixed>) -> Self {
        let mut snaps = Self::default();
        for (value, target_value) in values.take(MAX_STEM_SNAPS).zip(&mut snaps.values) {
            *target_value = value;
            snaps.len += 1;
        }
        snaps
    }

    pub fn values(&self) -> &[Fixed] {
        &self.values[..self.len as usize]
    }
}

#[inline]
pub(crate) fn parse_int(cursor: &mut Cursor, b0: u8) -> Result<i32, Error> {
    // Size   b0 range     Value range              Value calculation
    //--------------------------------------------------------------------------------
    // 1      32 to 246    -107 to +107             b0 - 139
    // 2      247 to 250   +108 to +1131            (b0 - 247) * 256 + b1 + 108
    // 2      251 to 254   -1131 to -108            -(b0 - 251) * 256 - b1 - 108
    // 3      28           -32768 to +32767         b1 << 8 | b2
    // 5      29           -(2^31) to +(2^31 - 1)   b1 << 24 | b2 << 16 | b3 << 8 | b4
    // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-3-operand-encoding>
    Ok(match b0 {
        32..=246 => b0 as i32 - 139,
        247..=250 => (b0 as i32 - 247) * 256 + cursor.read::<u8>()? as i32 + 108,
        251..=254 => -(b0 as i32 - 251) * 256 - cursor.read::<u8>()? as i32 - 108,
        28 => cursor.read::<i16>()? as i32,
        29 => cursor.read::<i32>()?,
        _ => {
            return Err(Error::InvalidNumber);
        }
    })
}

// Various unnamed constants inlined in FreeType's cff_parse_real function
// <<https://gitlab.freedesktop.org/freetype/freetype/-/blob/82090e67c24259c343c83fd9cefe6ff0be7a7eca/src/cff/cffparse.c#L183>>

// Value returned on overflow
const BCD_OVERFLOW: Fixed = Fixed::from_bits(0x7FFFFFFF);
// Value returned on underflow
const BCD_UNDERFLOW: Fixed = Fixed::ZERO;
// Limit at which we stop accumulating `number` and increase
// the exponent instead
const BCD_NUMBER_LIMIT: i32 = 0xCCCCCCC;
// Limit for the integral part of the result
const BCD_INTEGER_LIMIT: i32 = 0x7FFF;

// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/82090e67c24259c343c83fd9cefe6ff0be7a7eca/src/cff/cffparse.c#L150>
const BCD_POWER_TENS: [i32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

/// Components for computing a fixed point value for a binary coded decimal
/// number.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct BcdComponents {
    /// If overflow or underflow is detected early, then this
    /// contains the resulting value and we skip further
    /// processing.
    error: Option<Fixed>,
    number: i32,
    sign: i32,
    exponent: i32,
    exponent_add: i32,
    integer_len: i32,
    fraction_len: i32,
}

impl BcdComponents {
    /// Parse a binary coded decimal number.
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/82090e67c24259c343c83fd9cefe6ff0be7a7eca/src/cff/cffparse.c#L183>
    fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        enum Phase {
            Integer,
            Fraction,
            Exponent,
        }
        let mut phase = Phase::Integer;
        let mut sign = 1i32;
        let mut exponent_sign = 1i32;
        let mut number = 0i32;
        let mut exponent = 0i32;
        let mut exponent_add = 0i32;
        let mut integer_len = 0;
        let mut fraction_len = 0;
        // Nibble value    Represents
        //----------------------------------
        // 0 to 9          0 to 9
        // a               . (decimal point)
        // b               E
        // c               E-
        // d               <reserved>
        // e               - (minus)
        // f               end of number
        // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-5-nibble-definitions>
        'outer: loop {
            let b = cursor.read::<u8>()?;
            for nibble in [(b >> 4) & 0xF, b & 0xF] {
                match phase {
                    Phase::Integer => match nibble {
                        0x0..=0x9 => {
                            if number >= BCD_NUMBER_LIMIT {
                                exponent_add += 1;
                            } else if nibble != 0 || number != 0 {
                                number = number * 10 + nibble as i32;
                                integer_len += 1;
                            }
                        }
                        0xE => sign = -1,
                        0xA => {
                            phase = Phase::Fraction;
                        }
                        0xB => {
                            phase = Phase::Exponent;
                        }
                        0xC => {
                            phase = Phase::Exponent;
                            exponent_sign = -1;
                        }
                        _ => break 'outer,
                    },
                    Phase::Fraction => match nibble {
                        0x0..=0x9 => {
                            if nibble == 0 && number == 0 {
                                exponent_add -= 1;
                            } else if number < BCD_NUMBER_LIMIT && fraction_len < 9 {
                                number = number * 10 + nibble as i32;
                                fraction_len += 1;
                            }
                        }
                        0xB => {
                            phase = Phase::Exponent;
                        }
                        0xC => {
                            phase = Phase::Exponent;
                            exponent_sign = -1;
                        }
                        _ => break 'outer,
                    },
                    Phase::Exponent => {
                        match nibble {
                            0x0..=0x9 => {
                                // Arbitrarily limit exponent
                                if exponent > 1000 {
                                    return if exponent_sign == -1 {
                                        Ok(BCD_UNDERFLOW.into())
                                    } else {
                                        Ok(BCD_OVERFLOW.into())
                                    };
                                } else {
                                    exponent = exponent * 10 + nibble as i32;
                                }
                            }
                            _ => break 'outer,
                        }
                    }
                }
            }
        }
        exponent *= exponent_sign;
        Ok(Self {
            error: None,
            number,
            sign,
            exponent,
            exponent_add,
            integer_len,
            fraction_len,
        })
    }

    /// Returns the fixed point value for the precomputed components,
    /// optionally using an internal scale factor of 1000 to
    /// increase fractional precision.
    pub fn value(&self, scale_by_1000: bool) -> Fixed {
        if let Some(error) = self.error {
            return error;
        }
        let mut number = self.number;
        if number == 0 {
            return Fixed::ZERO;
        }
        let mut exponent = self.exponent;
        let mut integer_len = self.integer_len;
        let mut fraction_len = self.fraction_len;
        if scale_by_1000 {
            exponent += 3 + self.exponent_add;
        } else {
            exponent += self.exponent_add;
        }
        integer_len += exponent;
        fraction_len -= exponent;
        if integer_len > 5 {
            return BCD_OVERFLOW;
        }
        if integer_len < -5 {
            return BCD_UNDERFLOW;
        }
        // Remove non-significant digits
        if integer_len < 0 {
            number /= BCD_POWER_TENS[(-integer_len) as usize];
            fraction_len += integer_len;
        }
        // Can only happen if exponent was non-zero
        if fraction_len == 10 {
            number /= 10;
            fraction_len -= 1;
        }
        // Convert to fixed
        let mut result = if fraction_len > 0 {
            let b = BCD_POWER_TENS[fraction_len as usize];
            if number / b > BCD_INTEGER_LIMIT {
                0
            } else {
                (Fixed::from_bits(number) / Fixed::from_bits(b)).to_bits()
            }
        } else {
            number = number.wrapping_mul(BCD_POWER_TENS[-fraction_len as usize]);
            if number > BCD_INTEGER_LIMIT {
                return BCD_OVERFLOW;
            } else {
                number << 16
            }
        };
        if scale_by_1000 {
            // FreeType stores the scaled value and does a fixed division by
            // 1000 when the blue metrics are requested. We just do it here
            // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/psaux/psft.c#L554>
            result = (Fixed::from_bits(result) / Fixed::from_i32(1000)).to_bits();
        }
        Fixed::from_bits(result * self.sign)
    }

    /// Returns the fixed point value for the components along with a
    /// dynamically determined scale factor.
    ///
    /// Use for processing FontMatrix components.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/f1cd6dbfa0c98f352b698448f40ac27e8fb3832e/src/cff/cffparse.c#L332>
    fn dynamically_scaled_value(&self) -> (Fixed, i32) {
        if let Some(error) = self.error {
            return (error, 0);
        }
        let mut number = self.number;
        if number == 0 {
            return (Fixed::ZERO, 0);
        }
        let mut exponent = self.exponent;
        let integer_len = self.integer_len;
        let mut fraction_len = self.fraction_len;
        exponent += self.exponent_add;
        fraction_len += integer_len;
        exponent += integer_len;
        let result;
        let scaling;
        if fraction_len <= 5 {
            if number > BCD_INTEGER_LIMIT {
                result = Fixed::from_bits(number) / Fixed::from_bits(10);
                scaling = exponent - fraction_len + 1;
            } else {
                if exponent > 0 {
                    // Make scaling as small as possible
                    let new_fraction_len = exponent.min(5);
                    let shift = new_fraction_len - fraction_len;
                    if shift > 0 {
                        exponent -= new_fraction_len;
                        number *= BCD_POWER_TENS[shift as usize];
                        if number > BCD_INTEGER_LIMIT {
                            number /= 10;
                            exponent += 1;
                        }
                    } else {
                        exponent -= fraction_len;
                    }
                } else {
                    exponent -= fraction_len;
                }
                result = Fixed::from_bits(number << 16);
                scaling = exponent;
            }
        } else if (number / BCD_POWER_TENS[fraction_len as usize - 5]) > BCD_INTEGER_LIMIT {
            result = Fixed::from_bits(number)
                / Fixed::from_bits(BCD_POWER_TENS[fraction_len as usize - 4]);
            scaling = exponent - 4;
        } else {
            result = Fixed::from_bits(number)
                / Fixed::from_bits(BCD_POWER_TENS[fraction_len as usize - 5]);
            scaling = exponent - 5;
        }
        (Fixed::from_bits(result.to_bits() * self.sign), scaling)
    }
}

impl From<Fixed> for BcdComponents {
    fn from(value: Fixed) -> Self {
        Self {
            error: Some(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;
    use crate::{
        tables::variations::ItemVariationStore, types::F2Dot14, FontData, FontRead, FontRef,
        TableProvider,
    };

    #[test]
    fn int_operands() {
        // Test the boundary conditions of the ranged int operators
        let empty = FontData::new(&[]);
        let min_byte = FontData::new(&[0]);
        let max_byte = FontData::new(&[255]);
        // 32..=246 => -107..=107
        assert_eq!(parse_int(&mut empty.cursor(), 32).unwrap(), -107);
        assert_eq!(parse_int(&mut empty.cursor(), 246).unwrap(), 107);
        // 247..=250 => +108 to +1131
        assert_eq!(parse_int(&mut min_byte.cursor(), 247).unwrap(), 108);
        assert_eq!(parse_int(&mut max_byte.cursor(), 250).unwrap(), 1131);
        // 251..=254 => -1131 to -108
        assert_eq!(parse_int(&mut min_byte.cursor(), 251).unwrap(), -108);
        assert_eq!(parse_int(&mut max_byte.cursor(), 254).unwrap(), -1131);
    }

    #[test]
    fn binary_coded_decimal_operands() {
        // From <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-5-nibble-definitions>:
        //
        // "A real number is terminated by one (or two) 0xf nibbles so that it is always padded
        // to a full byte. Thus, the value -2.25 is encoded by the byte sequence (1e e2 a2 5f)
        // and the value 0.140541E-3 by the sequence (1e 0a 14 05 41 c3 ff)."
        //
        // The initial 1e byte in the examples above is the dictionary operator to trigger
        // parsing of BCD so it is dropped in the tests here.
        let bytes = FontData::new(&[0xe2, 0xa2, 0x5f]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(false),
            Fixed::from_f64(-2.25)
        );
        let bytes = FontData::new(&[0x0a, 0x14, 0x05, 0x41, 0xc3, 0xff]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(false),
            Fixed::from_f64(0.140541E-3)
        );
        // Check that we match FreeType for 375e-4.
        // Note: we used to parse 0.0375... but the new FT matching code
        // has less precision
        let bytes = FontData::new(&[0x37, 0x5c, 0x4f]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(false),
            Fixed::from_f64(0.0370025634765625)
        );
    }

    #[test]
    fn scaled_binary_coded_decimal_operands() {
        // For blue scale, we compute values with an internal factor of 1000 to match
        // FreeType, which gives us more precision for fractional bits
        let bytes = FontData::new(&[0xA, 0x06, 0x25, 0xf]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(true),
            Fixed::from_f64(0.0625)
        );
        // Just an additional check to test increased precision. Compare to
        // the test above where this value generates 0.0370...
        let bytes = FontData::new(&[0x37, 0x5c, 0x4f]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(true),
            Fixed::from_f64(0.037506103515625)
        );
    }

    #[test]
    fn example_top_dict_tokens() {
        use Operator::*;
        let top_dict_data = &font_test_data::cff2::EXAMPLE[5..12];
        let tokens: Vec<_> = tokens(top_dict_data).map(|entry| entry.unwrap()).collect();
        let expected: &[Token] = &[
            68.into(),
            FdArrayOffset.into(),
            56.into(),
            CharstringsOffset.into(),
            16.into(),
            VariationStoreOffset.into(),
        ];
        assert_eq!(&tokens, expected);
    }

    #[test]
    fn example_top_dict_entries() {
        use Entry::*;
        let top_dict_data = &font_test_data::cff2::EXAMPLE[0x5..=0xB];
        let entries: Vec<_> = entries(top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        let expected: &[Entry] = &[
            FdArrayOffset(68),
            CharstringsOffset(56),
            VariationStoreOffset(16),
        ];
        assert_eq!(&entries, expected);
    }

    #[test]
    fn example_private_dict_entries() {
        use Entry::*;
        let private_dict_data = &font_test_data::cff2::EXAMPLE[0x4f..=0xc0];
        let store =
            ItemVariationStore::read(FontData::new(&font_test_data::cff2::EXAMPLE[18..])).unwrap();
        let coords = &[F2Dot14::from_f32(0.0)];
        let blend_state = BlendState::new(store, coords, 0).unwrap();
        let entries: Vec<_> = entries(private_dict_data, Some(blend_state))
            .map(|entry| entry.unwrap())
            .collect();
        fn make_blues(values: &[f64]) -> Blues {
            Blues::new(values.iter().copied().map(Fixed::from_f64))
        }
        fn make_stem_snaps(values: &[f64]) -> StemSnaps {
            StemSnaps::new(values.iter().copied().map(Fixed::from_f64))
        }
        let expected: &[Entry] = &[
            BlueValues(make_blues(&[
                -20.0, 0.0, 472.0, 490.0, 525.0, 540.0, 645.0, 660.0, 670.0, 690.0, 730.0, 750.0,
            ])),
            OtherBlues(make_blues(&[-250.0, -240.0])),
            FamilyBlues(make_blues(&[
                -20.0, 0.0, 473.0, 491.0, 525.0, 540.0, 644.0, 659.0, 669.0, 689.0, 729.0, 749.0,
            ])),
            FamilyOtherBlues(make_blues(&[-249.0, -239.0])),
            BlueScale(Fixed::from_f64(0.037506103515625)),
            BlueFuzz(Fixed::ZERO),
            StdHw(Fixed::from_f64(55.0)),
            StdVw(Fixed::from_f64(80.0)),
            StemSnapH(make_stem_snaps(&[40.0, 55.0])),
            StemSnapV(make_stem_snaps(&[80.0, 90.0])),
            SubrsOffset(114),
        ];
        assert_eq!(&entries, expected);
    }

    #[test]
    fn noto_serif_display_top_dict_entries() {
        use Entry::*;
        let top_dict_data = FontRef::new(font_test_data::NOTO_SERIF_DISPLAY_TRIMMED)
            .unwrap()
            .cff()
            .unwrap()
            .top_dicts()
            .get(0)
            .unwrap();
        let entries: Vec<_> = entries(top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        let expected = &[
            Version(StringId::new(391)),
            Notice(StringId::new(392)),
            Copyright(StringId::new(393)),
            FullName(StringId::new(394)),
            FamilyName(StringId::new(395)),
            FontBbox([-693.0, -470.0, 2797.0, 1048.0].map(Fixed::from_f64)),
            Charset(517),
            PrivateDictRange(549..587),
            CharstringsOffset(521),
        ];
        assert_eq!(&entries, expected);
    }

    // Fuzzer caught add with overflow when constructing private DICT
    // range.
    // See <https://bugs.chromium.org/p/oss-fuzz/issues/detail?id=71746>
    // and <https://oss-fuzz.com/testcase?key=4591358306746368>
    #[test]
    fn private_dict_range_avoid_overflow() {
        // A Private DICT that tries to construct a range from -1..(-1 + -1)
        // which overflows when converted to usize
        let private_dict = BeBuffer::new()
            .push(29u8) // integer operator
            .push(-1i32) // integer value
            .push(29u8) // integer operator
            .push(-1i32) // integer value
            .push(18u8) // PrivateDICT operator
            .to_vec();
        // Just don't panic
        let _ = entries(&private_dict, None).count();
    }

    #[test]
    fn dynamically_scaled_binary_coded_decimal_operands() {
        // 0.0625
        let bytes = FontData::new(&[0xA, 0x06, 0x25, 0xf]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .dynamically_scaled_value(),
            (Fixed::from_f64(6250.0), -5)
        );
        // 0.0375
        let bytes = FontData::new(&[0x37, 0x5c, 0x4f]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .dynamically_scaled_value(),
            (Fixed::from_f64(375.0), -4)
        );
        // .001953125
        let bytes = FontData::new(&[0xa0, 0x1, 0x95, 0x31, 0x25, 0xff]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .dynamically_scaled_value(),
            (Fixed::from_bits(1280000000), -7)
        );
    }

    /// See <https://github.com/googlefonts/fontations/issues/1617>
    #[test]
    fn blue_scale_fraction_length_of_0() {
        // 0.0037
        let bytes = FontData::new(&[0x37, 0xC3, 0xFF]);
        assert_eq!(
            BcdComponents::parse(&mut bytes.cursor())
                .unwrap()
                .value(true),
            Fixed::from_f64(0.0370025634765625)
        );
    }

    #[test]
    fn read_font_matrix() {
        let dict_data = [
            30u8, 10, 0, 31, 139, 30, 10, 0, 1, 103, 255, 30, 10, 0, 31, 139, 139, 12, 7,
        ];
        let Entry::FontMatrix(matrix, _) = entries(&dict_data, None).next().unwrap().unwrap()
        else {
            panic!("This was totally a font matrix");
        };
        // From ttx: <FontMatrix value="0.001 0 0.000167 0.001 0 0"/>
        // But scaled by 1000 because that's how FreeType does it
        assert_eq!(
            matrix,
            [
                Fixed::ONE,
                Fixed::ZERO,
                Fixed::from_f64(0.167007446289062),
                Fixed::ONE,
                Fixed::ZERO,
                Fixed::ZERO,
            ]
        );
    }

    #[test]
    fn parse_degenerate_font_matrix() {
        let dict_data = [
            30u8, 0x0F, 30, 0x0F, 30, 0x0F, 30, 0x0F, 30, 0x0F, 30, 0x0F, 12, 7,
        ];
        // Don't return a degenerate matrix at all
        assert!(entries(&dict_data, None).next().is_none());
    }

    /// See <https://github.com/googlefonts/fontations/issues/1595>
    #[test]
    fn degenerate_matrix_check_doesnt_overflow() {
        // Values taken from font in the above issue
        let matrix = [
            Fixed::from_bits(639999672),
            Fixed::ZERO,
            Fixed::ZERO,
            Fixed::from_bits(639999672),
            Fixed::ZERO,
            Fixed::ZERO,
        ];
        // Just don't panic with overflow
        is_degenerate(&matrix);
        // Try again with all max values
        is_degenerate(&[Fixed::MAX; 6]);
        // And all min values
        is_degenerate(&[Fixed::MIN; 6]);
    }

    #[test]
    fn normalize_matrix() {
        // This matrix has a y scale of 0.5 so we should produce a new matrix
        // with a y scale of 1.0 and a scale factor of 2
        let matrix = [65536, 0, 0, 32768, 0, 0].map(Fixed::from_bits);
        let (normalized, scale) = normalize_font_matrix(matrix, 1);
        let expected_normalized = [131072, 0, 0, 65536, 0, 0].map(Fixed::from_bits);
        assert_eq!(normalized, expected_normalized);
        assert_eq!(scale, 2);
    }
}
